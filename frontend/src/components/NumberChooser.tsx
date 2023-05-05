import {FiMinus, FiPlus} from 'react-icons/fi';
import styles from './NumberChooser.module.css';

interface Props {
  value: number;
  onValueChange(newValue: number): void;
}

export const NumberChooser = ({value, onValueChange}: Props) => {
  return (
    <div className={styles.container}>
      <button onClick={() => onValueChange(value - 1)}>
        <FiMinus />
      </button>
      <label>{value}</label>
      <button onClick={() => onValueChange(value + 1)}>
        <FiPlus />
      </button>
    </div>
  );
};
