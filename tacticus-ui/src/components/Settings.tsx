import React, { useState, useEffect } from 'react';
import { XPWindow } from './xp/XPWindow';
import { XPButton } from './xp/XPButton';
import { XPInput } from './xp/XPInput';
import { XPPanel } from './xp/XPPanel';
import { useUserStore } from '../stores/userStore';
import './Settings.css';

interface SettingsProps {
  onClose: () => void;
  onSave?: () => void;
}

export const Settings: React.FC<SettingsProps> = ({ onClose, onSave }) => {
  const { apiKey, setApiKey, loadApiKey } = useUserStore();
  const [keyInput, setKeyInput] = useState('');
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    loadApiKey();
  }, []);

  useEffect(() => {
    if (apiKey) {
      // Mask the key for display
      setKeyInput(apiKey.slice(0, 10) + '...' + apiKey.slice(-4));
    }
  }, [apiKey]);

  const handleSave = async () => {
    if (!keyInput.trim() || keyInput.includes('...')) {
      return;
    }
    setSaving(true);
    try {
      await setApiKey(keyInput.trim());
      setSaved(true);
      setTimeout(() => {
        setSaved(false);
        onSave?.();
      }, 1500);
    } catch (err) {
      console.error('Failed to save API key:', err);
    } finally {
      setSaving(false);
    }
  };

  const handleInputChange = (value: string) => {
    setKeyInput(value);
    setSaved(false);
  };

  return (
    <div className="settings-overlay">
      <XPWindow
        title="Settings - API Configuration"
        icon="[=]"
        width={500}
        height="auto"
        onClose={onClose}
      >
        <div className="settings-content">
          <XPPanel label="OpenRouter API Key" className="api-key-section">
            <p className="settings-description">
              Tacticus uses OpenRouter to power Gurgeh, your AI chess coach. 
              You need an API key to enable AI features.
            </p>
            
            <div className="api-key-input-row">
              <XPInput
                value={keyInput}
                onChange={handleInputChange}
                placeholder="sk-or-v1-..."
                type="text"
                className="api-key-input"
              />
              <XPButton 
                onClick={handleSave} 
                disabled={saving || !keyInput.trim() || keyInput.includes('...')}
                primary
              >
                {saving ? 'Saving...' : saved ? 'Saved!' : 'Save'}
              </XPButton>
            </div>

            <div className="api-key-help">
              <p><strong>How to get an API key:</strong></p>
              <ol>
                <li>Visit <a href="https://openrouter.ai" target="_blank" rel="noopener noreferrer">openrouter.ai</a></li>
                <li>Create an account or sign in</li>
                <li>Go to Keys section and create a new key</li>
                <li>Copy and paste the key above</li>
              </ol>
              <p className="note">
                Your key is stored locally and never shared. OpenRouter provides access 
                to various AI models including Claude, GPT-4, and others.
              </p>
            </div>
          </XPPanel>

          <div className="settings-status">
            {apiKey ? (
              <span className="status-ok">[OK] API key configured - Gurgeh is ready</span>
            ) : (
              <span className="status-warning">[!] No API key - AI features disabled</span>
            )}
          </div>

          <div className="settings-actions">
            <XPButton onClick={onClose}>Close</XPButton>
          </div>
        </div>
      </XPWindow>
    </div>
  );
};
