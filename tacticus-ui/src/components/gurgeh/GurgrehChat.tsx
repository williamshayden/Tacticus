import React, { useState, useRef, useEffect, useCallback } from 'react';
import { XPWindow } from '../xp/XPWindow';
import { XPButton } from '../xp/XPButton';
import { useUserStore } from '../../stores/userStore';
import { streamCoachResponse, getPersonalizedGreeting } from '../../lib/ai/agent';
import { GREETING_PROMPT } from '../../lib/ai/prompts';
import type { ChatMessage, CoachAction } from '../../lib/ai/types';
import './GurgrehChat.css';

interface GurgrehChatProps {
  onClose?: () => void;
  onAction?: (action: CoachAction) => void;
  initialGreeting?: boolean;
  position?: { x: number; y: number };
}

interface DisplayMessage {
  role: 'user' | 'assistant';
  content: string;
  actions: CoachAction[];
  isStreaming?: boolean;
}

export const GurgrehChat: React.FC<GurgrehChatProps> = ({
  onClose,
  onAction,
  initialGreeting = true,
  position,
}) => {
  const [messages, setMessages] = useState<DisplayMessage[]>([]);
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [toolActivity, setToolActivity] = useState<string | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const { apiKey, profile, stats } = useUserStore();

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const loadGreeting = useCallback(async () => {
    // If no API key, show static greeting
    if (!apiKey) {
      const greeting = GREETING_PROMPT(
        profile?.name || 'Player',
        stats?.current_elo || 800,
        stats?.exercises_completed || 0
      );
      setMessages([{
        role: 'assistant',
        content: greeting,
        actions: [
          { action_type: 'start_training', label: 'Start Training', data: '' },
          { action_type: 'play_game', label: 'Play a Game', data: '' },
        ],
      }]);
      return;
    }

    setIsLoading(true);
    try {
      const greeting = await getPersonalizedGreeting(
        apiKey,
        profile?.name || 'Player'
      );
      setMessages([{
        role: 'assistant',
        content: greeting,
        actions: [
          { action_type: 'start_training', label: 'Start Training', data: '' },
          { action_type: 'play_game', label: 'Play a Game', data: '' },
        ],
      }]);
    } catch (error) {
      console.error('Failed to load greeting:', error);
      // Fallback to static greeting
      const greeting = GREETING_PROMPT(
        profile?.name || 'Player',
        stats?.current_elo || 800,
        stats?.exercises_completed || 0
      );
      setMessages([{
        role: 'assistant',
        content: greeting,
        actions: [
          { action_type: 'start_training', label: 'Start Training', data: '' },
          { action_type: 'play_game', label: 'Play a Game', data: '' },
        ],
      }]);
    } finally {
      setIsLoading(false);
    }
  }, [apiKey, profile?.name, stats?.current_elo, stats?.exercises_completed]);

  useEffect(() => {
    if (initialGreeting && messages.length === 0) {
      loadGreeting();
    }
  }, [initialGreeting, messages.length, loadGreeting]);

  const sendMessage = async () => {
    if (!input.trim() || !apiKey) return;

    const userMessage: DisplayMessage = {
      role: 'user',
      content: input,
      actions: [],
    };

    // Add user message and prepare for assistant response
    setMessages(prev => [...prev, userMessage]);
    setInput('');
    setIsLoading(true);

    // Create streaming assistant message
    const streamingMessage: DisplayMessage = {
      role: 'assistant',
      content: '',
      actions: [],
      isStreaming: true,
    };
    setMessages(prev => [...prev, streamingMessage]);

    // Build chat history for context
    const chatHistory: ChatMessage[] = messages
      .filter(m => !m.isStreaming)
      .map(m => ({
        role: m.role,
        content: m.content,
      }));
    chatHistory.push({ role: 'user', content: input });

    try {
      await streamCoachResponse(
        apiKey,
        chatHistory,
        {
          onChunk: (chunk) => {
            setMessages(prev => {
              const updated = [...prev];
              const lastIdx = updated.length - 1;
              if (updated[lastIdx].isStreaming) {
                updated[lastIdx] = {
                  ...updated[lastIdx],
                  content: updated[lastIdx].content + chunk,
                };
              }
              return updated;
            });
          },
          onComplete: (fullText) => {
            setMessages(prev => {
              const updated = [...prev];
              const lastIdx = updated.length - 1;
              if (updated[lastIdx].isStreaming) {
                updated[lastIdx] = {
                  role: 'assistant',
                  content: fullText,
                  actions: [],
                  isStreaming: false,
                };
              }
              return updated;
            });
            setIsLoading(false);
            setToolActivity(null);
          },
          onError: (error) => {
            console.error('Chat error:', error);
            setMessages(prev => {
              const updated = [...prev];
              const lastIdx = updated.length - 1;
              if (updated[lastIdx].isStreaming) {
                updated[lastIdx] = {
                  role: 'assistant',
                  content: `I encountered an error: ${error.message}. Please check your API key in Settings.`,
                  actions: [
                    { action_type: 'open_settings', label: 'Open Settings', data: '' },
                  ],
                  isStreaming: false,
                };
              }
              return updated;
            });
            setIsLoading(false);
            setToolActivity(null);
          },
          onToolCall: (toolName) => {
            setToolActivity(`Querying ${formatToolName(toolName)}...`);
          },
          onToolResult: () => {
            setToolActivity(null);
          },
        }
      );
    } catch (error) {
      console.error('Failed to send message:', error);
      setMessages(prev => {
        const updated = [...prev];
        const lastIdx = updated.length - 1;
        if (updated[lastIdx].isStreaming) {
          updated[lastIdx] = {
            role: 'assistant',
            content: `I encountered an error: ${error}. Please check your API key in Settings.`,
            actions: [
              { action_type: 'open_settings', label: 'Open Settings', data: '' },
            ],
            isStreaming: false,
          };
        }
        return updated;
      });
      setIsLoading(false);
    }
  };

  const formatToolName = (name: string): string => {
    const names: Record<string, string> = {
      getRecentGames: 'your recent games',
      getPlayerStats: 'your statistics',
      getWeaknessHistory: 'your weakness history',
      searchGamesByOpening: 'games by opening',
      getGamesWithMistakes: 'games with mistakes',
      getTrainingProgress: 'training progress',
      getImprovementTrend: 'improvement trend',
    };
    return names[name] || name;
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  };

  const handleAction = (action: CoachAction) => {
    onAction?.(action);
  };

  return (
    <div className="gurgeh-chat-container" style={position ? { left: position.x, top: position.y } : {}}>
      <XPWindow
        title="Gurgeh - Chess Coach"
        icon="[G]"
        onClose={onClose}
        width={400}
        height={520}
      >
        <div className="gurgeh-messages">
          {messages.map((msg, i) => (
            <div key={i} className={`gurgeh-message ${msg.role === 'assistant' ? 'gurgeh' : 'user'}`}>
              <div className="gurgeh-message-avatar">
                {msg.role === 'assistant' ? '[G]' : '[U]'}
              </div>
              <div className="gurgeh-message-content">
                <div className="gurgeh-message-text">
                  {msg.content}
                  {msg.isStreaming && <span className="cursor">|</span>}
                </div>
                {msg.actions.length > 0 && (
                  <div className="gurgeh-message-actions">
                    {msg.actions.map((action, j) => (
                      <XPButton
                        key={j}
                        onClick={() => handleAction(action)}
                      >
                        {action.label}
                      </XPButton>
                    ))}
                  </div>
                )}
              </div>
            </div>
          ))}
          {isLoading && !messages.some(m => m.isStreaming) && (
            <div className="gurgeh-message gurgeh">
              <div className="gurgeh-message-avatar">[G]</div>
              <div className="gurgeh-typing">
                <span></span>
                <span></span>
                <span></span>
              </div>
            </div>
          )}
          <div ref={messagesEndRef} />
        </div>

        {toolActivity && (
          <div className="gurgeh-status">
            <div className="gurgeh-status-dot"></div>
            {toolActivity}
          </div>
        )}

        <div className="gurgeh-input-area">
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyPress}
            placeholder="Ask Gurgeh anything..."
            className="xp-input gurgeh-input"
            disabled={isLoading}
          />
          <XPButton onClick={sendMessage} disabled={!input.trim() || isLoading || !apiKey}>
            Send
          </XPButton>
        </div>

        {!apiKey && (
          <div className="gurgeh-api-warning">
            [!] No API key configured. Add one in Settings to enable AI coaching.
          </div>
        )}
      </XPWindow>
    </div>
  );
};
