import {useEffect, useRef, useState} from 'react';
import {useOnClickOutside} from '../hooks/ClickOutside';
import styles from './Editable.module.css';

interface EditableProps {
  onSubmit(): void;
  onCancel(): void;
  onEdit?(): void;
  displayComponent: React.ReactNode;
  editComponent: React.ReactNode;
}

export const Editable = (props: EditableProps) => {
  const [isEditing, setIsEditing] = useState(false);

  const wrapperRef = useRef(null);

  useOnClickOutside(wrapperRef, () => {
    if (!isEditing) return;
    props.onSubmit();
    setIsEditing(false);
  });

  const handleKeyPress = (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (!isEditing) {
      return;
    }

    if (e.key === 'Enter') {
      props.onSubmit();
      setIsEditing(false);
    }

    if (e.key === 'Escape') {
      props.onCancel();
      setIsEditing(false);
    }
  };

  useEffect(() => {
    if (isEditing && props.onEdit) props.onEdit();
  }, [isEditing, props]);

  return (
    <div
      className={styles.container}
      onKeyDown={handleKeyPress}
      ref={wrapperRef}
      onClick={() => setIsEditing(true)}
    >
      {!isEditing && props.displayComponent}
      {isEditing && props.editComponent}
    </div>
  );
};
