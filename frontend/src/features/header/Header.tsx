import React, {useContext, useState} from 'react';
import styled from 'styled-components';
import {LargeMain} from '../../typography/Typography';
import {FiMenu} from 'react-icons/fi';
import {PopupMenu} from '../menu/PopupMenu';
import {useCore} from '../core/use-core';
import ReactModal from 'react-modal';
import {Projects} from './Projects';
import {RenameProject} from './RenameProject';
import {
  addProjectRequest,
  loadProjectsRequest,
  renameProjectRequest,
  saveRequest,
} from '../../api/request';
import {CoreDataContext} from '../core/CoreData';

export const Header: React.FunctionComponent = () => {
  const {project} = useContext(CoreDataContext);
  const core = useCore();
  const [projectModalOpen, setProjectModalOpen] = useState(false);
  const [renameModalOpen, setRenameModalOpen] = useState(false);
  const [newProjectModalOpen, setNewProjectModalOpen] = useState(false);

  const menuItems = [
    {
      title: 'New Project',
      onClick: () => {
        setNewProjectModalOpen(true);
      },
    },
    {
      title: 'Save Project',
      onClick: () => {
        core?.sendRequest(saveRequest());
      },
    },
    {
      title: 'Projects',
      onClick: () => {
        setProjectModalOpen(true);
        core?.sendRequest(loadProjectsRequest());
      },
    },
    {
      title: 'Rename Project',
      onClick: () => {
        setRenameModalOpen(true);
      },
    },
  ];

  return (
    <Container>
      <ProjectInfoTitle>{project?.info.name}</ProjectInfoTitle>
      <Spacer />

      <MenuButton>
        <PopupMenu menuItems={menuItems}>
          <FiMenu size={24} />
        </PopupMenu>
      </MenuButton>

      <ReactModal
        isOpen={renameModalOpen}
        onRequestClose={() => setRenameModalOpen(false)}
        style={modalStyle}
      >
        <RenameProject
          onSave={(name) => {
            core?.sendRequest(renameProjectRequest(name));
            setRenameModalOpen(false);
          }}
          onCancel={() => setRenameModalOpen(false)}
          name={project?.info.name}
          confirmButtonText="Rename"
          title="Rename Project"
        />
      </ReactModal>

      <ReactModal
        isOpen={newProjectModalOpen}
        onRequestClose={() => setNewProjectModalOpen(false)}
        style={modalStyle}
      >
        <RenameProject
          onSave={(name) => {
            core?.sendRequest(addProjectRequest());
            core?.sendRequest(renameProjectRequest(name));
            setNewProjectModalOpen(false);
          }}
          onCancel={() => setNewProjectModalOpen(false)}
          name="My Project"
          confirmButtonText="Create"
          title="Add New Project"
        />
      </ReactModal>

      <ReactModal
        isOpen={projectModalOpen}
        onRequestClose={() => setProjectModalOpen(false)}
        style={modalStyle}
      >
        <Projects onDismiss={() => setProjectModalOpen(false)} />
      </ReactModal>
    </Container>
  );
};

const ProjectInfoTitle = styled.h1`
  ${LargeMain}
`;

const modalStyle = {
  content: {
    top: '50%',
    left: '50%',
    bottom: 'auto',
    right: 'auto',
    marginRight: '-50%',
    transform: 'translate(-50%, -50%)',
    padding: 'none',
  },
};

const Container = styled.div`
  height: ${(props) => props.theme.units(8)};

  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;

  background: ${(props) => props.theme.colours.primaryLight};
  color: ${(props) => props.theme.textColours.primaryLight};

  box-shadow: ${(props) => props.theme.dropShadow};

  padding: ${(props) => props.theme.units(2)};

  position: relative;
`;

const Spacer = styled.div`
  flex: 1;
`;

const MenuButton = styled.button`
  background: none;
  border: none;
`;
