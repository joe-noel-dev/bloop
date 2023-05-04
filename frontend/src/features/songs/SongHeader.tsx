import React from 'react';
import styles from './SongHeader.module.css';

interface SongHeaderProps {
  name: string;
  selected: boolean;
}
export const SongHeader = (props: SongHeaderProps) => {
  return (
    <div
      className={`${styles.container} ${
        props.selected && styles['container-selected']
      }`}
    >
      <h2>{props.name}</h2>
    </div>
  );
};
