import styles from './ToggleSwitch.module.css';

interface Props {
  isOn: boolean;
  onChange(isOn: boolean): void;
}

export const ToggleSwitch = ({isOn, onChange}: Props) => {
  return (
    <button
      className={`${styles.button} ${isOn && styles['button-enabled']}`}
      onClick={() => onChange(!isOn)}
    >
      <div className={`${styles.toggle} ${isOn && styles['toggle-enabled']}`} />
    </button>
  );
};
