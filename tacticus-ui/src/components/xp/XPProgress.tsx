import React from 'react';

interface XPProgressProps {
  value: number; // 0-100
  showLabel?: boolean;
  className?: string;
}

export const XPProgress: React.FC<XPProgressProps> = ({
  value,
  showLabel = true,
  className = '',
}) => {
  const clampedValue = Math.max(0, Math.min(100, value));
  
  return (
    <div className={`xp-progress ${className}`}>
      <div 
        className="xp-progress-bar" 
        style={{ width: `${clampedValue}%` }}
      />
      {showLabel && (
        <span style={{ 
          position: 'absolute', 
          left: '50%', 
          transform: 'translateX(-50%)',
          fontSize: '10px',
          fontWeight: 'bold',
          color: clampedValue > 50 ? 'white' : 'black',
          textShadow: clampedValue > 50 ? '0 0 2px black' : 'none'
        }}>
          {Math.round(clampedValue)}%
        </span>
      )}
    </div>
  );
};
