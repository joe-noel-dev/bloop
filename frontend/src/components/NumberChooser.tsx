import {FiMinus, FiPlus} from 'react-icons/fi';
import styles from './NumberChooser.module.css';
import {EditText} from 'react-edit-text';

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
      <EditText
        className={styles.text}
        inputClassName={styles['edit-text']}
        defaultValue={`${value}`}
        type="number"
        onSave={({value}) => {
          const newValue = parseInt(value);
          if (!isNaN(newValue)) {
            onValueChange(newValue);
          }
        }}
      />
      <button onClick={() => onValueChange(value + 1)}>
        <FiPlus />
      </button>
    </div>
  );
};
