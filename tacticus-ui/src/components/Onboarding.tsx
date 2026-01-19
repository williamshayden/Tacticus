import React, { useState } from 'react';
import { XPWindow } from './xp/XPWindow';
import { XPButton } from './xp/XPButton';
import { XPInput } from './xp/XPInput';
import { useUserStore } from '../stores/userStore';
import './Onboarding.css';

interface OnboardingProps {
  onComplete: () => void;
}

export const Onboarding: React.FC<OnboardingProps> = ({ onComplete }) => {
  const [step, setStep] = useState(0);
  const [name, setName] = useState('');
  const [level, setLevel] = useState<string | null>(null);
  const [customElo, setCustomElo] = useState('');
  const { createProfile, loading } = useUserStore();

  const handleLevelSelect = (selectedLevel: string) => {
    setLevel(selectedLevel);
  };

  const handleComplete = async () => {
    const finalLevel = level === 'custom' ? customElo : level;
    if (name && finalLevel) {
      await createProfile(name, finalLevel);
      onComplete();
    }
  };

  return (
    <div className="onboarding-container">
      <XPWindow
        title="Welcome to Tacticus"
        icon="[K]"
        width={500}
        height="auto"
      >
        {step === 0 && (
          <div className="onboarding-step">
            <div className="onboarding-gurgeh">
              <div className="gurgeh-avatar">[G]</div>
              <div className="gurgeh-speech">
                <p>"Welcome to Tacticus. I'm <strong>Gurgeh</strong>, your chess coach.</p>
                <p>Named after the legendary game player from the Culture, I'll guide you 
                   from wherever you are to wherever you want to be in chess.</p>
                <p>Together, we'll master this ancient game."</p>
              </div>
            </div>
            
            <div className="onboarding-form">
              <label>What should I call you?</label>
              <XPInput
                value={name}
                onChange={setName}
                placeholder="Enter your name"
                className="onboarding-input"
              />
            </div>

            <div className="onboarding-actions">
              <XPButton
                primary
                onClick={() => setStep(1)}
                disabled={!name.trim()}
              >
                Continue
              </XPButton>
            </div>
          </div>
        )}

        {step === 1 && (
          <div className="onboarding-step">
            <div className="onboarding-gurgeh">
              <div className="gurgeh-avatar">[G]</div>
              <div className="gurgeh-speech">
                <p>"Nice to meet you, <strong>{name}</strong>.</p>
                <p>What's your current chess experience? This helps me tailor 
                   exercises and opponents to your level."</p>
              </div>
            </div>

            <div className="level-grid">
              <button
                className={`level-card ${level === 'beginner' ? 'selected' : ''}`}
                onClick={() => handleLevelSelect('beginner')}
              >
                <span className="level-icon">[B]</span>
                <span className="level-name">Beginner</span>
                <span className="level-elo">&lt; 800 ELO</span>
                <span className="level-desc">I know how pieces move</span>
              </button>

              <button
                className={`level-card ${level === 'intermediate' ? 'selected' : ''}`}
                onClick={() => handleLevelSelect('intermediate')}
              >
                <span className="level-icon">[I]</span>
                <span className="level-name">Intermediate</span>
                <span className="level-elo">800-1400 ELO</span>
                <span className="level-desc">I know basic tactics</span>
              </button>

              <button
                className={`level-card ${level === 'advanced' ? 'selected' : ''}`}
                onClick={() => handleLevelSelect('advanced')}
              >
                <span className="level-icon">[A]</span>
                <span className="level-name">Advanced</span>
                <span className="level-elo">1400-2000 ELO</span>
                <span className="level-desc">I study openings & endgames</span>
              </button>

              <button
                className={`level-card ${level === 'custom' ? 'selected' : ''}`}
                onClick={() => handleLevelSelect('custom')}
              >
                <span className="level-icon">[C]</span>
                <span className="level-name">Custom</span>
                <span className="level-elo">Enter your ELO</span>
                {level === 'custom' && (
                  <XPInput
                    value={customElo}
                    onChange={setCustomElo}
                    placeholder="e.g. 1200"
                    type="number"
                    className="custom-elo-input"
                  />
                )}
              </button>
            </div>

            <div className="onboarding-note">
              <span>[i]</span>
              <span>Don't worry - after this we'll play a calibration game to fine-tune your rating.</span>
            </div>

            <div className="onboarding-actions">
              <XPButton onClick={() => setStep(0)}>
                Back
              </XPButton>
              <XPButton
                primary
                onClick={handleComplete}
                disabled={!level || (level === 'custom' && !customElo) || loading}
              >
                {loading ? 'Starting...' : 'Begin Training'}
              </XPButton>
            </div>
          </div>
        )}
      </XPWindow>
    </div>
  );
};
