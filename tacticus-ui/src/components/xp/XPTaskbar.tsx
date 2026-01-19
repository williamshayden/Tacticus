import React, { useState, useEffect } from 'react';

interface TaskbarItem {
  id: string;
  icon: string;
  title: string;
  active?: boolean;
}

interface XPTaskbarProps {
  items?: TaskbarItem[];
  onItemClick?: (id: string) => void;
  onStartClick?: () => void;
  onSettingsClick?: () => void;
}

export const XPTaskbar: React.FC<XPTaskbarProps> = ({
  items = [],
  onItemClick,
  onStartClick,
  onSettingsClick,
}) => {
  const [time, setTime] = useState(new Date());

  useEffect(() => {
    const timer = setInterval(() => setTime(new Date()), 1000);
    return () => clearInterval(timer);
  }, []);

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString('en-US', { 
      hour: '2-digit', 
      minute: '2-digit',
      hour12: false 
    });
  };

  return (
    <div className="xp-taskbar">
      <button className="xp-start-button" onClick={onStartClick}>
        <div className="xp-start-icon">[*]</div>
        <span>start</span>
      </button>

      <div className="xp-taskbar-items">
        {items.map((item) => (
          <button
            key={item.id}
            className={`xp-taskbar-item ${item.active ? 'active' : ''}`}
            onClick={() => onItemClick?.(item.id)}
          >
            <span>{item.icon}</span>
            <span>{item.title}</span>
          </button>
        ))}
      </div>

      <div className="xp-system-tray">
        <button 
          className="xp-tray-button"
          onClick={onSettingsClick}
          title="Settings"
        >
          [=]
        </button>
        <span className="xp-tray-time">{formatTime(time)}</span>
      </div>
    </div>
  );
};
