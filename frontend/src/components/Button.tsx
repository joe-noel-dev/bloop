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
  <Button {...props} className={`${styles.primary} ${props.className}`}>
    {children}
  </Button>
);

export const SecondaryButton = ({children, ...props}: ButtonProps) => (
  <Button {...props} className={`${styles.secondary} ${props.className}`}>
    {children}
  </Button>
);

export const SecondaryDarkButton = ({children, ...props}: ButtonProps) => (
  <Button
    {...props}
    className={`${styles['seconary-dark']} ${props.className}`}
  >
    {children}
  </Button>
);

export const WarningButton = ({children, ...props}: ButtonProps) => (
  <SecondaryDarkButton {...props} className={styles['warning']}>
    {children}
  </SecondaryDarkButton>
);
