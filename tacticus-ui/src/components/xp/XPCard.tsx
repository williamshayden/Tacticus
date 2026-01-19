import React from 'react';

interface XPCardProps {
  title: string;
  icon?: string;
  children: React.ReactNode;
  onClick?: () => void;
  className?: string;
}

export const XPCard: React.FC<XPCardProps> = ({
  title,
  icon,
  children,
  onClick,
  className = '',
}) => {
  return (
    <div 
      className={`xp-card ${onClick ? 'cursor-pointer' : ''} ${className}`}
      onClick={onClick}
      style={{ cursor: onClick ? 'pointer' : 'default' }}
    >
      <div className="xp-card-header">
        {icon && <span className="xp-card-icon">{icon}</span>}
        <span className="xp-card-title">{title}</span>
      </div>
      {children}
    </div>
  );
};
