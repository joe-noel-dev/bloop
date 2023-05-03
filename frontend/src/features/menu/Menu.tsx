import React from 'react';
import styled from 'styled-components';
import {MediumMain} from '../../typography/Typography';
import {MenuItem} from './MenuItem';

interface MenuProps {
  menuItems: MenuItem[];
}

export const Menu = (props: MenuProps) => {
  return (
    <Container>
      {props.menuItems.map((item) => (
        <MenuRow
          onClick={(event) => {
            event.preventDefault();
            item.onClick();
          }}
          key={item.title}
        >
          <ItemTitle>{item.title}</ItemTitle>
        </MenuRow>
      ))}
    </Container>
  );
};

const ItemTitle = styled.p`
  ${MediumMain};
`;

const Container = styled.div`
  padding: ${(props) => props.theme.units(2)};
  background-color: ${(props) => props.theme.colours.background};
  color: ${(props) => props.theme.textColours.background};

  border-radius: ${(props) => props.theme.borderRadius};
  box-shadow: ${(props) => props.theme.dropShadow};

  color: ${(props) => props.theme.textColours.card};
`;

const MenuRow = styled.div`
  padding: ${(props) => props.theme.units(1)};
`;
