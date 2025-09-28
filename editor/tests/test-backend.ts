import PocketBase from 'pocketbase';

export interface DbUser {
  email: string;
  name: string;
}

export interface DbProject {
  collectionId: string;
  collectionName: string;
  created: Date;
  id: string;
  name: string;
  project: string;
  samples: string[];
  userId: string;
}

export interface Backend {
  signIn: (username: string, password: string) => Promise<any>;
  signOut: () => Promise<void>;
  getUser: () => DbUser | null;
  fetchProjects: () => Promise<DbProject[]>;
  loadProject: (projectId: string) => Promise<[any, DbProject]>;
  createProject: () => Promise<[any, DbProject]>;
  removeProject: (projectId: string) => Promise<void>;
  renameProject: (projectId: string, newName: string) => Promise<DbProject>;
  updateProject: (projectId: string, project: any) => Promise<void>;
  addSample: (projectId: string, sampleId: any, sample: File) => Promise<void>;
  removeSample: (projectId: string, sampleId: any) => Promise<void>;
  fetchSample: (project: DbProject, sampleId: any) => Promise<Blob | null>;
}

export interface TestUser {
  id: string;
  email: string;
  password: string;
  name: string;
}

export interface TestBackendConfig {
  useLocalPocketbase?: boolean;
  pocketbaseUrl?: string;
}

/**
 * Create a backend for testing purposes
 * This can use either a local PocketBase instance or a mock
 */
export function createTestBackend(config: TestBackendConfig = {}): Backend {
  if (config.useLocalPocketbase) {
    // Use a local PocketBase instance if available
    const pocketbaseUrl = config.pocketbaseUrl || 'http://127.0.0.1:8090';
    return createBackendWithUrl(pocketbaseUrl);
  } else {
    // Use a default backend (which points to production)
    // In a real test environment, you might want to mock this
    return createBackendWithUrl('https://joe-noel-dev-bloop.fly.dev');
  }
}

/**
 * Create a backend with a custom PocketBase URL
 */
function createBackendWithUrl(url: string): Backend {
  const pocketbase = new PocketBase(url);

  pocketbase.authStore.onChange(() => {
    console.log('Auth store changed:', pocketbase.authStore.record);
  });

  return {
    signIn: async (username: string, password: string) =>
      await signIn(pocketbase, username, password),

    signOut: async () => pocketbase.authStore.clear(),

    getUser: () => {
      if (!pocketbase.authStore.isValid || !pocketbase.authStore.record) {
        return null;
      }

      return {
        email: pocketbase.authStore.record.email,
        name: pocketbase.authStore.record.name,
      };
    },

    fetchProjects: async () => {
      const records = await pocketbase.collection('projects').getFullList({
        sort: '-created',
      });
      return records.map(record => ({
        collectionId: record.collectionId,
        collectionName: record.collectionName,
        created: new Date(record.created),
        id: record.id,
        name: record.name,
        project: record.project,
        samples: record.samples || [],
        userId: record.userId,
      }));
    },

    loadProject: async (projectId: string) => {
      const record = await pocketbase.collection('projects').getOne(projectId);
      const projectInfo = {
        collectionId: record.collectionId,
        collectionName: record.collectionName,
        created: new Date(record.created),
        id: record.id,
        name: record.name,
        project: record.project,
        samples: record.samples || [],
        userId: record.userId,
      };

      const projectUrl = `${pocketbase.baseURL}/api/files/${record.collectionId}/${record.id}/${record.project}`;
      const response = await fetch(projectUrl);
      const projectData = await response.arrayBuffer();
      
      // This would need the actual Project decoder from the protobuf
      // For now, return a mock project
      const project = { 
        songs: [],
        samples: []
      };

      return [project, projectInfo];
    },

    createProject: async () => {
      if (!pocketbase.authStore.isValid || !pocketbase.authStore.record) {
        throw new Error('User is not authenticated');
      }

      // Create a mock project for testing
      const mockProject = { songs: [], samples: [] };
      const projectData = new Uint8Array(0); // Mock project data

      const record = await pocketbase.collection('projects').create({
        name: 'Test Project',
        userId: pocketbase.authStore.record.id,
        project: new File([projectData], 'project.bin'),
      });

      const dbProject = {
        collectionId: record.collectionId,
        collectionName: record.collectionName,
        created: new Date(record.created),
        id: record.id,
        name: record.name,
        project: record.project,
        samples: record.samples || [],
        userId: record.userId,
      };

      return [mockProject, dbProject];
    },

    removeProject: async (projectId: string) => {
      if (!pocketbase.authStore.isValid || !pocketbase.authStore.record) {
        throw new Error('User is not authenticated');
      }

      await pocketbase.collection('projects').delete(projectId);
    },

    renameProject: async (projectId: string, newName: string) => {
      if (!pocketbase.authStore.isValid || !pocketbase.authStore.record) {
        throw new Error('User is not authenticated');
      }

      const projectInfo = await pocketbase
        .collection('projects')
        .update(projectId, { name: newName });

      return {
        collectionId: projectInfo.collectionId,
        collectionName: projectInfo.collectionName,
        created: new Date(projectInfo.created),
        id: projectInfo.id,
        name: projectInfo.name,
        project: projectInfo.project,
        samples: projectInfo.samples || [],
        userId: projectInfo.userId,
      };
    },

    updateProject: async (projectId: string, project: any) => {
      if (!projectId) {
        console.warn('Project not updated: no project ID provided');
        return;
      }

      // Mock project update
      const projectData = new Uint8Array(0);
      const projectFile = new File([projectData], 'project.bin');

      await pocketbase
        .collection('projects')
        .update(projectId, { project: projectFile });
    },

    addSample: async (projectId: string, sampleId: any, sample: File) => {
      if (!pocketbase.authStore.isValid || !pocketbase.authStore.record) {
        throw new Error('User is not authenticated');
      }

      const renamedSample = new File([sample], `${sampleId}.wav`, {
        type: sample.type,
      });

      await pocketbase.collection('projects').update(projectId, {
        'samples+': [renamedSample],
      });
    },

    removeSample: async (projectId: string, sampleId: any) => {
      if (!pocketbase.authStore.isValid || !pocketbase.authStore.record) {
        throw new Error('User is not authenticated');
      }

      const project = await pocketbase.collection('projects').getOne(projectId);
      const samples = project.samples || [];
      const samplesToRemove = samples.filter((s: string) =>
        s.includes(sampleId.toString())
      );

      if (samplesToRemove.length > 0) {
        await pocketbase.collection('projects').update(projectId, {
          'samples-': samplesToRemove,
        });
      }
    },

    fetchSample: async (project: any, sampleId: any) => {
      if (!pocketbase.authStore.isValid || !pocketbase.authStore.record) {
        throw new Error('User is not authenticated');
      }

      const samples = project.samples || [];
      const sampleFile = samples.find((s: string) =>
        s.includes(sampleId.toString())
      );

      if (!sampleFile) {
        return null;
      }

      const sampleUrl = `${pocketbase.baseURL}/api/files/${project.collectionId}/${project.id}/${sampleFile}`;
      const response = await fetch(sampleUrl);

      if (!response.ok) {
        throw new Error(`Failed to fetch sample: ${response.statusText}`);
      }

      return await response.blob();
    },
  };
}

const signIn = async (
  pocketbase: PocketBase,
  username: string,
  password: string
) => {
  const authData = await pocketbase
    .collection('users')
    .authWithPassword(username, password);

  return {
    ...authData.record,
  };
};

/**
 * Add a test user to a PocketBase instance
 */
export async function addTestUser(pocketbaseUrl: string, user: TestUser): Promise<void> {
  const payload = {
    email: user.email,
    password: user.password,
    passwordConfirm: user.password,
    name: user.name,
    emailVisibility: true,
    verified: true
  };

  const response = await fetch(`${pocketbaseUrl}/api/collections/users/records`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(payload)
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to create user: ${errorText}`);
  }
}

/**
 * Create a test user object
 */
export function createTestUser(id: string, email: string, password: string, name: string): TestUser {
  return { id, email, password, name };
}