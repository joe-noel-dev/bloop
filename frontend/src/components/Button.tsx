import styles from './Button.module.css';

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  children?: React.ReactNode;
}

export const Button = ({children, ...props}: ButtonProps) => (
  <button {...props} className={`${styles.button} ${props.className}`}>
    {children}
  </button>
);

export const PrimaryButton = ({children, ...props}: ButtonProps) => (
  <Button className={`${styles.primary} ${props.className}`} {...props}>
    {children}
  </Button>
);

export const SecondaryButton = ({children, ...props}: ButtonProps) => (
  <Button className={styles.secondary} {...props}>
    {children}
  </Button>
);

export const SecondaryDarkButton = ({children, ...props}: ButtonProps) => (
  <Button
    className={`${styles['seconary-dark']} ${props.className}`}
    {...props}
  >
    {children}
  </Button>
);

export const WarningButton = ({children, ...props}: ButtonProps) => (
  <SecondaryDarkButton className={styles['warning']} {...props}>
    {children}
  </SecondaryDarkButton>
);
