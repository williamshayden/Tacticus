import React from 'react';

interface XPWindowProps {
  title: string;
  icon?: string;
  children: React.ReactNode;
  width?: string | number;
  height?: string | number;
  onClose?: () => void;
  onMinimize?: () => void;
  onMaximize?: () => void;
  className?: string;
}

export const XPWindow: React.FC<XPWindowProps> = ({
  title,
  icon = '📁',
  children,
  width = 'auto',
  height = 'auto',
  onClose,
  onMinimize,
  onMaximize,
  className = '',
}) => {
  return (
    <div 
      className={`xp-window ${className}`}
      style={{ width, height }}
    >
      <div className="xp-window-titlebar">
        <span className="xp-window-icon">{icon}</span>
        <span className="xp-window-title">{title}</span>
        <div className="xp-window-controls">
          {onMinimize && (
            <button className="xp-window-btn xp-window-btn-minimize" onClick={onMinimize}>
              −
            </button>
          )}
          {onMaximize && (
            <button className="xp-window-btn xp-window-btn-maximize" onClick={onMaximize}>
              □
            </button>
          )}
          {onClose && (
            <button className="xp-window-btn xp-window-btn-close" onClick={onClose}>
              ×
            </button>
          )}
        </div>
      </div>
      <div className="xp-window-content">
        {children}
      </div>
    </div>
  );
};
