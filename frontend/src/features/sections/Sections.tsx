import {Section} from './Section';
import styles from './Sections.module.css';

interface SectionsProps {
  songId: string;
  sectionIds: string[];
}

export const Sections = (props: SectionsProps) => {
  return (
    <div className={styles.container}>
      {props.sectionIds.map((sectionId) => (
        <Section key={sectionId} songId={props.songId} sectionId={sectionId} />
      ))}
    </div>
  );
};
