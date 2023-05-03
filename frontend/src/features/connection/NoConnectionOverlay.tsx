import React from 'react';
import styled from 'styled-components';
import {IndeterminateSpinner} from '../../components/IndeterminateSpinner';
import {LargeMain} from '../../typography/Typography';
import styles from './NoConnectionOverlay.module.css';

export const NoConnectionOverlay = () => {
  return (
    <div className={styles['container']}>
      <h1 className={styles['message']}>Waiting for connection...</h1>
      <IndeterminateSpinner />
    </div>
  );
};
