import React from 'react';

interface XPButtonProps {
  children: React.ReactNode;
  onClick?: () => void;
  disabled?: boolean;
  primary?: boolean;
  className?: string;
  type?: 'button' | 'submit' | 'reset';
}

export const XPButton: React.FC<XPButtonProps> = ({
  children,
  onClick,
  disabled = false,
  primary = false,
  className = '',
  type = 'button',
}) => {
  return (
    <button
      type={type}
      className={`xp-button ${primary ? 'xp-button-primary' : ''} ${className}`}
      onClick={onClick}
      disabled={disabled}
    >
      {children}
    </button>
  );
};
