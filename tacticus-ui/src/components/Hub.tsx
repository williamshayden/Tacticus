import React from 'react';
import { XPWindow } from './xp/XPWindow';
import { XPCard } from './xp/XPCard';
import { XPButton } from './xp/XPButton';
import { XPProgress } from './xp/XPProgress';
import { useUserStore } from '../stores/userStore';
import './Hub.css';

type View = 'hub' | 'train' | 'play' | 'analyze' | 'learn';

interface HubProps {
  onNavigate: (view: View) => void;
  onChatClick: () => void;
}

export const Hub: React.FC<HubProps> = ({ onNavigate, onChatClick }) => {
  const { profile, stats } = useUserStore();

  const exercisesUntilCalibration = stats?.exercises_until_calibration ?? 10;
  const calibrationProgress = ((10 - exercisesUntilCalibration) / 10) * 100;

  return (
    <div className="hub-container">
      <XPWindow
        title="Tacticus - Gurgeh's Training Ground"
        icon="[K]"
        width={900}
        height={600}
      >
        {/* Stats Bar */}
        <div className="hub-stats-bar">
          <div className="hub-stat">
            <span className="hub-stat-label">Player</span>
            <span className="hub-stat-value">{profile?.name || 'Player'}</span>
          </div>
          <div className="hub-stat-divider" />
          <div className="hub-stat">
            <span className="hub-stat-label">Rating</span>
            <span className="hub-stat-value rating">
              {stats?.current_elo || 800}
              {stats && stats.current_elo > stats.peak_elo - 50 && (
                <span className="rating-change">+{stats.current_elo - (profile?.current_elo || 800)}</span>
              )}
            </span>
          </div>
          <div className="hub-stat-divider" />
          <div className="hub-stat">
            <span className="hub-stat-label">Streak</span>
            <span className="hub-stat-value">{stats?.streak || 0}</span>
          </div>
          <div className="hub-stat-divider" />
          <div className="hub-stat">
            <span className="hub-stat-label">Style</span>
            <span className="hub-stat-value">{stats?.style || 'Unknown'}</span>
          </div>
          <div className="hub-stat-divider" />
          <div className="hub-stat">
            <span className="hub-stat-label">Next Calibration</span>
            <span className="hub-stat-value">{exercisesUntilCalibration} exercises</span>
          </div>
        </div>

        {/* Main Cards */}
        <div className="hub-cards">
          <XPCard
            title="TRAIN"
            icon="[T]"
            onClick={() => onNavigate('train')}
            className="hub-card"
          >
            <p className="card-description">
              Personalized exercises tailored to your weaknesses
            </p>
            <div className="card-stats">
              <span>10 exercises ready</span>
              <span className="focus-tag">Focus: Tactics</span>
            </div>
            <div className="card-progress">
              <span>Session Progress</span>
              <XPProgress value={calibrationProgress} />
            </div>
            <XPButton primary onClick={() => onNavigate('train')}>
              Continue Training
            </XPButton>
          </XPCard>

          <XPCard
            title="PLAY"
            icon="[P]"
            onClick={() => onNavigate('play')}
            className="hub-card"
          >
            <p className="card-description">
              Challenge an engine matched to your rating
            </p>
            <div className="card-stats">
              <span>Engine ELO: {(stats?.current_elo || 800) + 25}</span>
              <span className="format-tag">Rapid 10+0</span>
            </div>
            <div className="card-last-game">
              <span>Last Game:</span>
              <span className="game-result win">Win vs 832</span>
            </div>
            <XPButton primary onClick={() => onNavigate('play')}>
              Play Now
            </XPButton>
          </XPCard>

          <XPCard
            title="ANALYZE"
            icon="[A]"
            onClick={() => onNavigate('analyze')}
            className="hub-card"
          >
            <p className="card-description">
              Review games with Gurgeh's guidance
            </p>
            <div className="card-stats">
              <span>Games: {stats?.games_played || 0}</span>
              <span>Reviewed: 0</span>
            </div>
            <div className="card-features">
              <span>[+] Move-by-move analysis</span>
              <span>[+] Blunder detection</span>
              <span>[+] Custom exercises</span>
            </div>
            <XPButton onClick={() => onNavigate('analyze')}>
              Open Analysis
            </XPButton>
          </XPCard>

          <XPCard
            title="LEARN"
            icon="[L]"
            onClick={() => onNavigate('learn')}
            className="hub-card"
          >
            <p className="card-description">
              Chess concepts & terminology from basics
            </p>
            <div className="card-stats">
              <span>Concepts: 15+</span>
              <span>Mastered: 0</span>
            </div>
            <div className="card-features">
              <span>[+] Forks, pins, skewers</span>
              <span>[+] Opening principles</span>
              <span>[+] Endgame basics</span>
            </div>
            <XPButton onClick={() => onNavigate('learn')}>
              Browse Concepts
            </XPButton>
          </XPCard>
        </div>

        {/* Gurgeh Message */}
        <div className="hub-gurgeh-message">
          <span className="gurgeh-icon">[G]</span>
          <div className="gurgeh-text">
            <strong>Gurgeh:</strong> "Your knight play has improved 15% this week. 
            But I've noticed you're still missing pins in complex positions. 
            Today's exercises will sharpen that."
          </div>
          <XPButton onClick={onChatClick}>Chat with Gurgeh</XPButton>
        </div>
      </XPWindow>
    </div>
  );
};
