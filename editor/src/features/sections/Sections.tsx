import {Section as ModelSection} from '../../model/section';
import {Section} from './Section';
import styles from './Sections.module.css';

interface SectionsProps {
  sections: ModelSection[];
}

export const Sections = ({sections}: SectionsProps) => {
  return (
    <div className={styles.container}>
      {sections.map((section) => (
        <Section key={section.id} section={section} />
      ))}
    </div>
  );
};
