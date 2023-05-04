import {useRef, useState} from 'react';
import {FiMinus, FiPlus} from 'react-icons/fi';
import styles from './ValueEditor.module.css';

interface Props {
  value: number;
  onSubmit(value: number): void;
  validate?(value: number): boolean;
  sanitise?(value: number): number;
  disabled?: boolean;
}

export const ValueEditor = (props: Props) => {
  const [editing, setEditing] = useState(false);
  const [value, setValue] = useState(`${props.value}`);
  const inputRef = useRef<HTMLInputElement>(null);

  const submit = (value: string) => {
    let numericValue = parseFloat(value);

    if (isNaN(numericValue)) {
      return;
    }

    if (props.validate && !props.validate(numericValue)) {
      return;
    }

    if (props.sanitise) {
      numericValue = props.sanitise(numericValue);
    }

    props.onSubmit(numericValue);
  };

  const onEditChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setValue(event.target.value);
  };

  const onClickOutside = () => {
    submit(value);
    setEditing(false);
  };

  const onKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Enter') {
      submit(value);
      setEditing(false);
    }

    if (event.key === 'Escape') {
      setValue(`${props.value}`);
      setEditing(false);
    }
  };

  return (
    <div className={styles.container}>
      <div style={{display: 'flex'}}>
        <button
          className={styles['increment-button']}
          onClick={(event) => {
            submit(`${props.value - 1}`);
            event.preventDefault();
          }}
        >
          <FiMinus />
        </button>
        <div
          className={styles['value-container']}
          onClick={() => {
            if (!editing && !props.disabled) setEditing(true);
          }}
        >
          {editing ? (
            <input
              autoFocus
              disabled={props.disabled}
              ref={inputRef}
              type="number"
              value={`${value}`}
              onKeyDown={onKeyDown}
              onChange={onEditChange}
              onBlur={onClickOutside}
            />
          ) : (
            <p>{props.value}</p>
          )}
        </div>

        <button
          className={styles['increment-button']}
          onClick={(event) => {
            submit(`${props.value + 1}`);
            event.preventDefault();
          }}
        >
          <FiPlus />
        </button>
      </div>
    </div>
  );
};
