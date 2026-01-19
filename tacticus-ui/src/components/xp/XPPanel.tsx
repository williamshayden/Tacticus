import React from 'react';

interface XPPanelProps {
  label?: string;
  children: React.ReactNode;
  className?: string;
}

export const XPPanel: React.FC<XPPanelProps> = ({
  label,
  children,
  className = '',
}) => {
  return (
    <div className={`xp-panel ${className}`}>
      {label && <div className="xp-panel-label">{label}</div>}
      {children}
    </div>
  );
};
