import React, {useRef, useState} from 'react';
import {FiEdit2} from 'react-icons/fi';
import styles from './NameEditor.module.css';

interface NameEditorProps {
  onSave(name: string): void;
  name: string;
  editable: boolean;
  inputType?: string;
  textClassName?: string;
}

export const NameEditor = (props: NameEditorProps) => {
  const [editing, setEditing] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);
  const [value, setValue] = useState(props.name);

  const startEditing = () => {
    if (!editing && props.editable) {
      setEditing(true);
      inputRef.current?.focus();
    }
  };

  const onKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (!props.editable) return;

    if (event.key === 'Enter') {
      if (inputRef.current && inputRef.current.value) {
        props.onSave(inputRef.current.value);
      }

      setEditing(false);
    }

    if (event.key === 'Escape') {
      setValue(props.name);
      setEditing(false);
    }
  };

  const onClickOutside = () => {
    if (!props.editable) return;

    if (inputRef.current && inputRef.current.value) {
      props.onSave(inputRef.current.value);
    }

    if (editing) {
      setEditing(false);
    }
  };

  return (
    <div
      onClick={(event) => {
        startEditing();
        event.stopPropagation();
      }}
    >
      {!editing && (
        <h3 className={`${styles.name} ${props.textClassName}`}>
          {props.name}
          {props.editable && (
            <button className={styles['edit-button']}>
              <FiEdit2 />
            </button>
          )}
        </h3>
      )}
      {editing && (
        <input
          className={`${styles.input} ${props.textClassName}`}
          autoFocus
          onKeyDown={onKeyDown}
          onBlur={onClickOutside}
          ref={inputRef}
          type={props.inputType || 'text'}
          value={value}
          onChange={(event) => setValue(event.target.value)}
        />
      )}
    </div>
  );
};
