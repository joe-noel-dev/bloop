import React, {useContext} from 'react';
import styled from 'styled-components';
import {LargeMain, SmallMain, XLargeMain} from '../../typography/Typography';
import moment from 'moment';
import {useCore} from '../core/use-core';
import {FiTrash} from 'react-icons/fi';
import {loadProjectRequest, removeProjectRequest} from '../../api/request';
import {CoreDataContext} from '../core/CoreData';

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
    <Container>
      <ProjectsTitleText>Projects</ProjectsTitleText>
      <ProjectsContainer>
        {projects?.map((project) => {
          return (
            <ProjectButton
              key={project.id}
              onClick={() => loadProject(project.id)}
              tabIndex={0}
              onKeyDown={(event) => onKeyDown(event, project.id)}
            >
              <TextContainer>
                <ProjectNameText>{project.name}</ProjectNameText>
                <LastSavedText>
                  Last saved {formatTimeAgo(project.lastSaved)}
                </LastSavedText>
              </TextContainer>
              <Spacer />
              <DeleteButton
                onClick={(event) => {
                  deleteProject(project.id);
                  event.stopPropagation();
                }}
              >
                <FiTrash size={16} />
              </DeleteButton>
            </ProjectButton>
          );
        })}
      </ProjectsContainer>
    </Container>
  );
};

const ProjectsTitleText = styled.title`
  ${XLargeMain};
`;

const ProjectNameText = styled.h2`
  ${LargeMain}
`;

const LastSavedText = styled.p`
  ${SmallMain}
`;

const Container = styled.div`
  backgroud: white;
  padding: ${(props) => props.theme.units(2)};

  width: 80vw;
`;

const ProjectButton = styled.div`
  display: block;
  margin: ${(props) => props.theme.units(2)} 0;
  width: 100%;
  background: none;
  border: 1px solid ${(props) => props.theme.colours.cardLayer};
  border-radius: ${(props) => props.theme.borderRadius}
  text-align: left;
  padding: ${(props) => props.theme.units(2)};

  height: ${(props) => props.theme.units(12)};

  display: flex;
`;

const TextContainer = styled.div`
  margin: auto 0;
`;

const Spacer = styled.div`
  flex: 1;
`;

const DeleteButton = styled.button`
  background: none;
  min-width: ${(props) => props.theme.units(6)};
`;

const ProjectsContainer = styled.div`
  max-height: ${(props) => props.theme.units(60)};
  overflow-y: scroll;
`;
