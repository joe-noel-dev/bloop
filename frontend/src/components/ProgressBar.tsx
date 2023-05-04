import styles from './ProgressBar.module.css';

interface Props {
  progress: number;
  colour: string;
}

export const ProgressBar = ({progress, colour}: Props) => {
  return (
    <div
      className={styles.container}
      style={
        {
          '--progress': `${progress * 100.0}%`,
          '--colour': colour,
        } as React.CSSProperties
      }
    />
  );
};
