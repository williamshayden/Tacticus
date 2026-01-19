import { useEffect, useState } from 'react';
import { Onboarding } from './components/Onboarding';
import { Hub } from './components/Hub';
import { PlayMode } from './components/PlayMode';
import { TrainMode } from './components/TrainMode';
import { LearnMode } from './components/LearnMode';
import { AnalyzeMode } from './components/AnalyzeMode';
import { GurgrehChat } from './components/gurgeh/GurgrehChat';
import { XPTaskbar } from './components/xp/XPTaskbar';
import { Settings } from './components/Settings';
import { useUserStore } from './stores/userStore';
import './styles/xp-theme.css';
import './App.css';

type View = 'hub' | 'train' | 'play' | 'analyze' | 'learn';

function App() {
  const [currentView, setCurrentView] = useState<View>('hub');
  const [showChat, setShowChat] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [isCalibration, setIsCalibration] = useState(false);
  const { hasOnboarded, checkOnboarding, loadProfile, loadStats, loadApiKey, apiKey } = useUserStore();
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const init = async () => {
      const onboarded = await checkOnboarding();
      if (onboarded) {
        await Promise.all([loadProfile(), loadStats(), loadApiKey()]);
      }
      setLoading(false);
    };
    init();
  }, []);

  const handleOnboardingComplete = async () => {
    await loadProfile();
    await loadStats();
    // Prompt for API key after onboarding
    setShowSettings(true);
  };

  const handleNavigate = (view: View) => {
    setIsCalibration(false);
    setCurrentView(view);
  };

  const handleCalibrationNeeded = () => {
    setIsCalibration(true);
    setCurrentView('play');
  };

  const handleChatAction = (action: { action_type: string }) => {
    if (action.action_type === 'start_training') {
      setCurrentView('train');
      setShowChat(false);
    } else if (action.action_type === 'play_game') {
      setCurrentView('play');
      setShowChat(false);
    } else if (action.action_type === 'open_settings') {
      setShowSettings(true);
    }
  };

  const taskbarItems = [
    { id: 'tacticus', icon: '[K]', title: 'Tacticus', active: true },
  ];

  if (showChat) {
    taskbarItems.push({ id: 'gurgeh', icon: '[G]', title: 'Gurgeh', active: true });
  }

  if (loading) {
    return (
      <div className="xp-desktop">
        <div className="loading-container">
          <div className="loading-text">Loading Tacticus...</div>
        </div>
      </div>
    );
  }

  return (
    <div className="xp-desktop">
      <div className="app-content">
        {!hasOnboarded ? (
          <Onboarding onComplete={handleOnboardingComplete} />
        ) : (
          <>
            {currentView === 'hub' && (
              <Hub 
                onNavigate={handleNavigate} 
                onChatClick={() => setShowChat(true)}
              />
            )}
            {currentView === 'train' && (
              <TrainMode 
                onBack={() => setCurrentView('hub')}
                onCalibrationNeeded={handleCalibrationNeeded}
              />
            )}
            {currentView === 'play' && (
              <PlayMode 
                onBack={() => setCurrentView('hub')}
                isCalibration={isCalibration}
              />
            )}
            {currentView === 'analyze' && (
              <AnalyzeMode onBack={() => setCurrentView('hub')} />
            )}
            {currentView === 'learn' && (
              <LearnMode onBack={() => setCurrentView('hub')} />
            )}
          </>
        )}

        {showChat && hasOnboarded && (
          <GurgrehChat
            onClose={() => setShowChat(false)}
            onAction={handleChatAction}
          />
        )}

        {showSettings && (
          <Settings 
            onClose={() => setShowSettings(false)}
            onSave={() => loadApiKey()}
          />
        )}
      </div>

      <XPTaskbar
        items={taskbarItems}
        onItemClick={(id) => {
          if (id === 'gurgeh') {
            setShowChat(true);
          }
        }}
        onStartClick={() => setShowChat(!showChat)}
        onSettingsClick={() => setShowSettings(true)}
      />

      {/* Show API key prompt banner if not configured */}
      {hasOnboarded && !apiKey && !showSettings && (
        <div className="api-key-banner" onClick={() => setShowSettings(true)}>
          [!] OpenRouter API key required for AI features. Click here to configure.
        </div>
      )}
    </div>
  );
}

export default App;
