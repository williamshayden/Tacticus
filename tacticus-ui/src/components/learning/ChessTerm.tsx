import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './ChessTerm.css';

interface ChessTermProps {
  term: string;
  children?: React.ReactNode;
}

export const ChessTerm: React.FC<ChessTermProps> = ({ term, children }) => {
  const [showTooltip, setShowTooltip] = useState(false);
  const [definition, setDefinition] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handleMouseEnter = async () => {
    setShowTooltip(true);
    if (!definition && !loading) {
      setLoading(true);
      try {
        const def = await invoke<string | null>('define_term', { term: term.toLowerCase() });
        setDefinition(def);
      } catch (err) {
        console.error('Failed to fetch definition:', err);
      } finally {
        setLoading(false);
      }
    }
  };

  const handleMouseLeave = () => {
    setShowTooltip(false);
  };

  return (
    <span 
      className="chess-term"
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
    >
      {children || term}
      {showTooltip && (
        <div className="chess-term-tooltip">
          <div className="tooltip-header">{term}</div>
          <div className="tooltip-content">
            {loading ? 'Loading...' : definition || 'Definition not found'}
          </div>
        </div>
      )}
    </span>
  );
};
