import React from 'react';
import {FiPlus} from 'react-icons/fi';
import {addSongRequest} from '../../api/request';
import {SecondaryButton} from '../../components/Button';
import {useCore} from '../core/use-core';
import styles from './AddSong.module.css';

export const AddSong = () => {
  const core = useCore();

  const addSong = () => {
    core?.sendRequest(addSongRequest());
  };

  return (
    <SecondaryButton className={styles.button} onClick={addSong}>
      <FiPlus />
      <label>Add Song</label>
    </SecondaryButton>
  );
};
