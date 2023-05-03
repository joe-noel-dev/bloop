import styled from 'styled-components';
import {horizontalGap} from '../components/Gap';

export const Button = styled.button`
  height: ${(props) => props.theme.units(6)};
  border-radius: ${(props) => props.theme.borderRadius};
  border: none;

  padding: 0 ${(props) => props.theme.units(2)};

  ${(props) => horizontalGap(props.theme.units(1))};

  display: flex;
  align-items: center;
  justify-content: center;
`;

export const PrimaryButton = styled(Button)`
  background-color: ${(props) => props.theme.colours.primary};
  color: ${(props) => props.theme.textColours.primary};

  :active {
    background: ${(props) => props.theme.colours.primaryLight};
  }
`;

export const SecondaryButton = styled(Button)`
  background-color: ${(props) => props.theme.colours.secondary};
  color: ${(props) => props.theme.textColours.secondary};

  :active {
    background: ${(props) => props.theme.colours.secondaryLight};
  }
`;

export const SecondaryDarkButton = styled(Button)`
  background-color: ${(props) => props.theme.colours.secondaryDark};
  color: ${(props) => props.theme.textColours.secondaryDark};

  :active {
    background: ${(props) => props.theme.colours.secondary};
  }
`;

export const WarningButton = styled(SecondaryButton)`
  background-color: #6e1404;
`;
