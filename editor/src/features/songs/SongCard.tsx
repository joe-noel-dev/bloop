import styles from './SongCard.module.css';

interface Props {
  isSelected: boolean;
  onSelectSong?(): void;
  children?: React.ReactNode;
}

export const SongCard = (props: Props) => {
  return (
    <div className={styles.container} onClick={props.onSelectSong}>
      {props.children}
    </div>
  );
};
