import React from 'react';
import styles from './IndeterminateSpinner.module.css';

export const IndeterminateSpinner = () => {
  return (
    <div className={styles.spinner}>
      <div className={styles['left-half']}>
        <div className={styles.left}>
          <div className={styles.circle} />
        </div>
      </div>
      <div className={`${styles['right-half']}`}>
        <div className={styles.right}>
          <div className={styles.circle} />
        </div>
      </div>
    </div>
  );
};
