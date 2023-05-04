import React from 'react';
import {MenuItem} from './MenuItem';
import styles from './Menu.module.css';

interface MenuProps {
  menuItems: MenuItem[];
}

export const Menu = (props: MenuProps) => {
  return (
    <div className={styles.container}>
      {props.menuItems.map((item) => (
        <div
          className={styles.row}
          onClick={(event) => {
            event.preventDefault();
            item.onClick();
          }}
          key={item.title}
        >
          <p>{item.title}</p>
        </div>
      ))}
    </div>
  );
};
