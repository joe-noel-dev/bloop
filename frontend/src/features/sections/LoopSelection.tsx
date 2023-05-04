import styles from './LoopSelection.module.css';

interface Props {
  loop: boolean;
  onChange(loop: boolean): void;
}

export const LoopSelection = (props: Props) => {
  interface SegmentOptions {
    name: string;
    action: () => void;
    selected: boolean;
  }

  const segment = (options: SegmentOptions) => {
    return (
      <div
        onClick={(event) => {
          options.action();
          event.stopPropagation();
        }}
        className={`${styles['radio-segment']} ${
          options.selected && styles['radio-segment-selected']
        }`}
      >
        <p>{options.name}</p>
      </div>
    );
  };

  return (
    <div className={styles['container']}>
      <div className={styles['radio-box']}>
        {segment({
          name: 'Off',
          action: () => props.onChange(false),
          selected: !props.loop,
        })}

        {segment({
          name: 'On',
          action: () => props.onChange(true),
          selected: props.loop,
        })}
      </div>
    </div>
  );
};
