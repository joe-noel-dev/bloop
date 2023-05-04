import {useCore} from '../core/use-core';
import {FaPlus} from 'react-icons/fa';
import {addSectionRequest} from '../../api/request';
import styles from './AddSection.module.css';

interface AddSectionProps {
  songId: string;
}

export const AddSection = (props: AddSectionProps) => {
  const core = useCore();

  const addSection = () => core?.sendRequest(addSectionRequest(props.songId));

  return (
    <button className={styles['add-section']} onClick={addSection}>
      <FaPlus />
    </button>
  );
};
