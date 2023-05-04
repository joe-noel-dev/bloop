import {useRef, useState} from 'react';
import {FiCheck, FiX} from 'react-icons/fi';
import {SecondaryButton, SecondaryDarkButton} from '../../components/Button';
import styles from './RenameProject.module.css';
import {Spacer} from '../../components/Spacer';

interface Props {
  name?: string;
  onSave(name: string): void;
  onCancel(): void;
  confirmButtonText?: string;
  title?: string;
}

export const RenameProject = (props: Props) => {
  const inputRef = useRef<HTMLInputElement>(null);
  const [value, setValue] = useState(props.name || '');

  const onKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') {
      if (inputRef.current && inputRef.current.value) {
        props.onSave(inputRef.current.value);
      }
    }

    if (event.key === 'Escape') {
      props.onCancel();
    }
  };

  return (
    <div className={styles['container']}>
      <h2>{props.title || 'Project Name'}</h2>

      <input
        className={styles['name-box']}
        autoFocus
        onKeyDown={onKeyDown}
        ref={inputRef}
        type="text"
        value={value}
        onChange={(event) => setValue(event.target.value)}
      />

      <div className={styles['button-container']}>
        <SecondaryDarkButton onClick={props.onCancel}>
          <FiX size={16} />
          <p>Cancel</p>
        </SecondaryDarkButton>
        <Spacer />
        <SecondaryButton onClick={() => props.onSave(value)}>
          <FiCheck size={16} />
          <label>{props.confirmButtonText || 'Create'}</label>
        </SecondaryButton>
      </div>
    </div>
  );
};
