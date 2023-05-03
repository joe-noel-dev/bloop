import React, {useState} from 'react';
import {Popover} from 'react-tiny-popover';
import {Menu} from './Menu';
import {MenuItem} from './MenuItem';
import styled from 'styled-components';

interface MenuProps {
  menuItems: MenuItem[];
  canOpen?(): boolean;
}

export const PopupMenu: React.FunctionComponent<MenuProps> = (props) => {
  const [isOpen, setOpen] = useState(false);

  const canOpen = () => {
    if (!isOpen) return false;
    if (props.canOpen && !props.canOpen()) return false;
    return true;
  };

  const menuItems = props.menuItems.map((item) => {
    return {
      ...item,
      onClick: () => {
        setOpen(false);
        setTimeout(item.onClick, 500);
      },
    };
  });

  return (
    <Popover
      isOpen={canOpen()}
      positions={['bottom']}
      content={<Menu menuItems={menuItems} />}
      onClickOutside={() => setOpen(false)}
    >
      <Content onClick={() => setOpen(true)}>{props.children}</Content>
    </Popover>
  );
};

const Content = styled.div``;
