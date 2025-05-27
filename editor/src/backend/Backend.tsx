import {createContext} from 'react';
import PocketBase, {RecordModel} from 'pocketbase';
import {Project} from '../api/bloop';
import {ID} from '../api/helpers';
import {emptyProject} from '../api/project-helpers';

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

export const BackendContext = createContext<Backend | null>(null);

// export const useBackend = () => {
//   const context = useContext(BackendContext);
//   if (!context) {
//     throw new Error('useBackend should be called from within a CoreProvider');
//   }
//   return context;
// };

export const createBackend = () => {
  const pocketbase = new PocketBase('https://joe-noel-dev-bloop.fly.dev');

  pocketbase.authStore.onChange(() => {
    console.log('Auth store changed:', pocketbase.authStore.record);
  });

  return {
    signIn: async (username: string, password: string) =>
      await signIn(pocketbase, username, password),

    signOut: async () => pocketbase.authStore.clear(),

    getUser: (): DbUser | null => {
      if (!pocketbase.authStore.isValid || !pocketbase.authStore.record) {
        return null;
      }

      return {
        ...pocketbase.authStore.record,
        email: pocketbase.authStore.record.email,
        name: pocketbase.authStore.record.name,
      };
    },

    fetchProjects: async () => await fetchProjects(pocketbase),

    loadProject: async (projectId: string) =>
      await loadProject(pocketbase, projectId),

    createProject: async () => await createProject(pocketbase),

    removeProject: async (projectId: string) =>
      await removeProject(pocketbase, projectId),

    renameProject: async (projectId: string, newName: string) =>
      await renameProject(pocketbase, projectId, newName),

    updateProject: async (projectId: string, project: Project) => {
      if (!projectId) {
        console.warn('Project not updated: no project ID provided');
        return;
      }

      await updateProject(pocketbase, projectId, project);
      console.debug('Updated project:', projectId, project);
    },

    addSample: async (projectId: string, sampleId: ID, sample: File) =>
      await addSample(pocketbase, projectId, sampleId, sample),

    removeSample: async (projectId: string, sampleId: ID) =>
      await removeSample(pocketbase, projectId, sampleId),
  };
};

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

const fetchProjects = async (pocketbase: PocketBase) => {
  const records = await pocketbase.collection('projects').getFullList({
    sort: '-created',
  });
  return records.map(toDbProject);
};

const toDbProject = (record: RecordModel): DbProject => {
  return {
    collectionId: record.collectionId,
    collectionName: record.collectionName,
    created: new Date(record.created),
    id: record.id,
    name: record.name,
    project: record.project,
    samples: record.samples || [],
    userId: record.userId,
  };
};

const loadProject = async (
  pocketbase: PocketBase,
  projectId: string
): Promise<[Project, DbProject]> => {
  const record = await pocketbase.collection('projects').getOne(projectId);
  const projectInfo = toDbProject(record);

  console.log('Loaded project:', projectInfo);

  const projectUrl = `${pocketbase.baseURL}/api/files/${record.collectionId}/${record.id}/${record.project}`;
  const response = await fetch(projectUrl);
  const projectData = await response.arrayBuffer();
  const project = Project.decode(new Uint8Array(projectData));

  return [project, projectInfo];
};

const updateProject = async (
  pocketbase: PocketBase,
  projectId: string,
  project: Project
) => {
  await pocketbase.collection('projects').update(projectId, {project: []});

  const projectData = Project.encode(project).finish();
  const projectFile = new File([projectData], 'project.bin');

  await pocketbase
    .collection('projects')
    .update(projectId, {project: projectFile});
};

const renameProject = async (
  pocketbase: PocketBase,
  projectId: string,
  newName: string
): Promise<DbProject> => {
  if (!pocketbase.authStore.isValid || !pocketbase.authStore.record) {
    throw new Error('User is not authenticated');
  }

  const projectInfo = await pocketbase
    .collection('projects')
    .update(projectId, {name: newName});

  console.log(`Renamed project with ID: ${projectId} to ${newName}`);

  return toDbProject(projectInfo);
};

const createProject = async (
  pocketbase: PocketBase
): Promise<[Project, DbProject]> => {
  if (!pocketbase.authStore.isValid || !pocketbase.authStore.record) {
    throw new Error('User is not authenticated');
  }

  const project = emptyProject();
  const projectData = Project.encode(project).finish();

  const record = await pocketbase.collection('projects').create({
    name: 'Project',
    userId: pocketbase.authStore.record.id,
    project: new File([projectData], 'project.bin'),
  });

  const dbProject = toDbProject(record);

  console.log('Created project:', dbProject);

  return [project, dbProject];
};

const removeProject = async (pocketbase: PocketBase, projectId: string) => {
  if (!pocketbase.authStore.isValid || !pocketbase.authStore.record) {
    throw new Error('User is not authenticated');
  }

  await pocketbase.collection('projects').delete(projectId);

  console.log(`Removed project with ID: ${projectId}`);
};

const removeSample = async (
  pocketbase: PocketBase,
  projectId: string,
  sampleId: ID
) => {
  if (!pocketbase.authStore.isValid || !pocketbase.authStore.record) {
    throw new Error('User is not authenticated');
  }

  if (!projectId) {
    throw new Error('Project ID is required to remove a sample');
  }

  const project = await pocketbase.collection('projects').getOne(projectId);

  const samples = project.samples || [];

  const samplesToRemove = samples.filter((s: string) =>
    s.includes(sampleId.toString())
  );

  if (samplesToRemove.length === 0) {
    return;
  }

  await pocketbase.collection('projects').update(projectId, {
    'samples-': samplesToRemove,
  });

  console.log(`Removed sample with ID: ${sampleId} from project ${projectId}`);
};

const addSample = async (
  pocketbase: PocketBase,
  projectId: string,
  sampleId: ID,
  sample: File
) => {
  if (!pocketbase.authStore.isValid || !pocketbase.authStore.record) {
    throw new Error('User is not authenticated');
  }

  await removeSample(pocketbase, projectId, sampleId);

  const renamedSample = new File([sample], `${sampleId}.wav`, {
    type: sample.type,
  });

  await pocketbase.collection('projects').update(projectId, {
    'samples+': [renamedSample],
  });

  console.log(`Added sample with ID: ${sampleId} to project ${projectId}`);
};

export type Backend = ReturnType<typeof createBackend>;
