import React, { useState } from 'react';
import { XPWindow } from './xp/XPWindow';
import { XPButton } from './xp/XPButton';
import { ChessBoard } from './board/ChessBoard';
import { useGameStore } from '../stores/gameStore';
import { useUserStore } from '../stores/userStore';
import './PlayMode.css';

interface PlayModeProps {
  onBack: () => void;
  isCalibration?: boolean;
}

export const PlayMode: React.FC<PlayModeProps> = ({ onBack, isCalibration = false }) => {
  const [showConfig, setShowConfig] = useState(true);
  const [selectedColor, setSelectedColor] = useState<'white' | 'black' | 'random'>('random');
  const [timeControl, setTimeControl] = useState('10+0');
  
  const { 
    gameState, 
    isThinking, 
    startNewGame, 
    makeMove,
    playerColor,
    engineElo,
  } = useGameStore();
  
  const { stats } = useUserStore();

  const handleStartGame = async () => {
    const color = selectedColor === 'random' 
      ? (Math.random() > 0.5 ? 'white' : 'black') 
      : selectedColor;
    
    await startNewGame(color);
    setShowConfig(false);
  };

  const handleMove = (from: string, to: string): boolean => {
    if (!gameState || isThinking) return false;
    if (gameState.turn !== playerColor) return false;
    
    makeMove(from, to);
    return true;
  };

  const handleResign = () => {
    setShowConfig(true);
  };

  const handleNewGame = () => {
    setShowConfig(true);
  };

  const lastMove = gameState?.last_move ? {
    from: gameState.last_move.slice(0, 2),
    to: gameState.last_move.slice(2, 4),
  } : null;

  const gameStatus = gameState?.is_checkmate 
    ? (gameState.turn === playerColor ? 'You lost by checkmate' : 'You won by checkmate!')
    : gameState?.is_stalemate 
      ? 'Draw by stalemate'
      : gameState?.is_check 
        ? 'Check!'
        : isThinking 
          ? 'Engine thinking...'
          : gameState?.turn === playerColor 
            ? 'Your move'
            : 'Waiting for engine...';

  if (showConfig) {
    return (
      <div className="play-mode-container">
        <XPWindow
          title={isCalibration ? "Calibration Game" : "New Game"}
          icon="[P]"
          width={450}
          height="auto"
          onClose={onBack}
        >
          <div className="game-config">
            {isCalibration && (
              <div className="calibration-notice">
                <span>[!]</span>
                <p>This calibration game will help adjust your rating based on your performance against the engine.</p>
              </div>
            )}

            <div className="config-section">
              <label>Opponent</label>
              <div className="opponent-info">
                <span className="engine-name">Tacticus Engine</span>
                <span className="engine-elo">ELO: {engineElo} (+/-25)</span>
              </div>
            </div>

            <div className="config-section">
              <label>Your Rating</label>
              <div className="user-rating">
                <span>{stats?.current_elo || 800}</span>
              </div>
            </div>

            <div className="config-section">
              <label>Time Control</label>
              <div className="time-options">
                {['1+0', '3+0', '5+0', '10+0', '15+10', '30+0'].map((tc) => (
                  <button
                    key={tc}
                    className={`time-option ${timeControl === tc ? 'selected' : ''}`}
                    onClick={() => setTimeControl(tc)}
                  >
                    {tc}
                  </button>
                ))}
              </div>
            </div>

            <div className="config-section">
              <label>Play As</label>
              <div className="color-options">
                <button
                  className={`color-option ${selectedColor === 'white' ? 'selected' : ''}`}
                  onClick={() => setSelectedColor('white')}
                >
                  <span className="color-piece">[W]</span>
                  <span>White</span>
                </button>
                <button
                  className={`color-option ${selectedColor === 'random' ? 'selected' : ''}`}
                  onClick={() => setSelectedColor('random')}
                >
                  <span className="color-piece">[?]</span>
                  <span>Random</span>
                </button>
                <button
                  className={`color-option ${selectedColor === 'black' ? 'selected' : ''}`}
                  onClick={() => setSelectedColor('black')}
                >
                  <span className="color-piece">[B]</span>
                  <span>Black</span>
                </button>
              </div>
            </div>

            <div className="config-actions">
              <XPButton onClick={onBack}>Cancel</XPButton>
              <XPButton primary onClick={handleStartGame}>
                Start Game
              </XPButton>
            </div>
          </div>
        </XPWindow>
      </div>
    );
  }

  return (
    <div className="play-mode-container">
      <div className="game-layout">
        <XPWindow
          title={`Game vs Engine (${engineElo} ELO)`}
          icon="[P]"
          width={520}
          height="auto"
        >
          <ChessBoard
            fen={gameState?.fen}
            orientation={playerColor}
            onMove={handleMove}
            lastMove={lastMove}
            interactive={!gameState?.is_checkmate && !gameState?.is_stalemate && !isThinking}
          />
        </XPWindow>

        <div className="game-sidebar">
          <XPWindow title="Game Status" icon="[i]" width={280} height="auto">
            <div className="status-section">
              <div className="player-info opponent">
                <span className="player-icon">[E]</span>
                <div className="player-details">
                  <span className="player-name">Engine</span>
                  <span className="player-elo">{engineElo} ELO</span>
                </div>
              </div>

              <div className="game-status-message">
                {gameStatus}
              </div>

              <div className="player-info you">
                <span className="player-icon">[U]</span>
                <div className="player-details">
                  <span className="player-name">You</span>
                  <span className="player-elo">{stats?.current_elo || 800} ELO</span>
                </div>
              </div>
            </div>

            <div className="xp-divider" />

            <div className="eval-section">
              <label>Evaluation</label>
              <div className="eval-bar">
                <div 
                  className="eval-white" 
                  style={{ 
                    width: `${50 + Math.max(-50, Math.min(50, (gameState?.evaluation || 0) * 10))}%` 
                  }}
                />
              </div>
              <span className="eval-value">
                {gameState ? (gameState.evaluation >= 0 ? '+' : '') + gameState.evaluation.toFixed(1) : '0.0'}
              </span>
            </div>

            <div className="xp-divider" />

            <div className="game-actions">
              <XPButton onClick={handleResign}>Resign</XPButton>
              <XPButton onClick={handleNewGame}>New Game</XPButton>
            </div>
          </XPWindow>

          <XPWindow title="Move Log" icon="[#]" width={280} height={200}>
            <div className="xp-panel move-log">
              {gameState?.legal_moves.length === 0 && !gameState?.is_checkmate && !gameState?.is_stalemate ? (
                <span className="no-moves">No moves yet</span>
              ) : (
                <span className="no-moves">Game in progress...</span>
              )}
            </div>
          </XPWindow>
        </div>
      </div>
    </div>
  );
};
