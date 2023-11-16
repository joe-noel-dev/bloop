import {useContext} from 'react';
import moment from 'moment';
import {useCore} from '../core/use-core';
import {FiTrash} from 'react-icons/fi';
import {loadProjectRequest, removeProjectRequest} from '../../api/request';
import {CoreDataContext} from '../core/CoreData';
import styles from './Projects.module.css';
import {Spacer} from '../../components/Spacer';

const formatTimeAgo = (lastSaved: number) => {
  const date = new Date(0);
  date.setUTCMilliseconds(lastSaved);
  return moment(date).fromNow();
};

interface Props {
  onDismiss(): void;
}

export const Projects = (props: Props) => {
  const core = useCore();
  let {projects} = useContext(CoreDataContext);

  projects = projects?.slice().sort((a, b) => b.lastSaved - a.lastSaved);

  const loadProject = (projectId: string) => {
    core?.sendRequest(loadProjectRequest(projectId));
    props.onDismiss();
  };

  const deleteProject = (projectId: string) => {
    core?.sendRequest(removeProjectRequest(projectId));
  };

  const onKeyDown = (
    event: React.KeyboardEvent<HTMLDivElement>,
    projectId: string
  ) => {
    if (event.key === 'Enter') {
      loadProject(projectId);
    }
  };

  return (
    <div className={styles['container']}>
      <title>Projects</title>
      <div className={styles['projects-list']}>
        {projects?.map((project) => {
          return (
            <div
              className={styles['project-tab']}
              key={project.id}
              onClick={() => loadProject(project.id)}
              tabIndex={0}
              onKeyDown={(event) => onKeyDown(event, project.id)}
            >
              <div className={styles['text-container']}>
                <h2>{project.name}</h2>
                <p>Last saved {formatTimeAgo(project.lastSaved)}</p>
              </div>

              <Spacer />

              <button
                className={styles['delete-button']}
                onClick={(event) => {
                  deleteProject(project.id);
                  event.stopPropagation();
                }}
              >
                <FiTrash />
              </button>
            </div>
          );
        })}
      </div>
    </div>
  );
};
