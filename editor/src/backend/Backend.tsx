import {createContext, useContext} from 'react';
import PocketBase from 'pocketbase';
import {EventEmitter} from 'events';
import {Project} from '../api/bloop';

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

export const useBackend = () => {
  const context = useContext(BackendContext);
  if (!context) {
    throw new Error('useBackend should be called from within a CoreProvider');
  }
  return context;
};

export const createBackend = () => {
  const pocketbase = new PocketBase('https://joe-noel-dev-bloop.fly.dev');
  const events = new EventEmitter();

  pocketbase.authStore.onChange(() => {
    events.emit('user', {...pocketbase.authStore.record});
  });
  return {
    signIn: async (username: string, password: string) => {
      const user = await signIn(pocketbase, username, password);
      events.emit('user', user);
      return user;
    },

    signOut: async () => {
      pocketbase.authStore.clear();
      events.emit('user', null);
    },

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

    fetchProjects: async () => {
      const projects = await fetchProjects(pocketbase);
      events.emit('projects', projects);
    },

    loadProject: async (projectId: string) => {
      const [project, projectInfo] = await loadProject(pocketbase, projectId);
      events.emit('project-info', projectInfo);
      events.emit('project', project);
    },

    events,
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
  return records.map((record) => ({
    collectionId: record.collectionId,
    collectionName: record.collectionName,
    created: new Date(record.created),
    id: record.id,
    name: record.name,
    project: record.project,
    samples: record.samples,
    userId: record.userId,
  }));
};

const loadProject = async (pocketbase: PocketBase, projectId: string) => {
  const record = await pocketbase.collection('projects').getOne(projectId);

  const projectInfo = {
    collectionId: record.collectionId,
    collectionName: record.collectionName,
    created: new Date(record.created),
    id: record.id,
    name: record.name,
    project: record.project,
    samples: record.samples,
    userId: record.userId,
  };

  const projectUrl = pocketbase.files.getURL(
    {project: projectInfo.id},
    record.project
  );

  const response = await fetch(projectUrl);
  const projectData = await response.arrayBuffer();
  const project = Project.decode(new Uint8Array(projectData));

  return [project, projectInfo];
};

export type Backend = ReturnType<typeof createBackend>;
