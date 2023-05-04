import {useState} from 'react';
import {Popover} from 'react-tiny-popover';
import {Menu} from './Menu';
import {MenuItem} from './MenuItem';

interface MenuProps {
  menuItems: MenuItem[];
  canOpen?(): boolean;
  children?: React.ReactNode;
}

export const PopupMenu = ({menuItems, canOpen, children}: MenuProps) => {
  const [isOpen, setOpen] = useState(false);

  const canOpenMenu = () => {
    if (!isOpen) {
      return false;
    }

    if (canOpen && !canOpen()) {
      return false;
    }

    return true;
  };

  const items = menuItems.map((item) => {
    return {
      ...item,
      onClick: () => {
        setOpen(false);
        setTimeout(item.onClick, 500);
      },
    };
  });

  return (
    <Popover
      isOpen={canOpenMenu()}
      positions={['bottom']}
      content={<Menu menuItems={items} />}
      onClickOutside={() => setOpen(false)}
    >
      <div onClick={() => setOpen(true)}>{children}</div>
    </Popover>
  );
};
