import React, { useState, useRef, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { XPWindow } from '../xp/XPWindow';
import { XPButton } from '../xp/XPButton';
import { useUserStore } from '../../stores/userStore';
import './GurgrehChat.css';

interface CoachMessage {
  role: string;
  content: string;
  timestamp: number;
  actions: CoachAction[];
}

interface CoachAction {
  action_type: string;
  label: string;
  data: string;
}

interface CoachResponse {
  message: CoachMessage;
  board_fen: string | null;
  highlights: string[];
  arrows: [string, string][];
}

interface GurgrehChatProps {
  onClose?: () => void;
  onAction?: (action: CoachAction) => void;
  initialGreeting?: boolean;
  position?: { x: number; y: number };
}

export const GurgrehChat: React.FC<GurgrehChatProps> = ({
  onClose,
  onAction,
  initialGreeting = true,
  position,
}) => {
  const [messages, setMessages] = useState<CoachMessage[]>([]);
  const [input, setInput] = useState('');
  const [isTyping, setIsTyping] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const { apiKey } = useUserStore();

  useEffect(() => {
    if (initialGreeting) {
      loadGreeting();
    }
  }, [initialGreeting]);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const loadGreeting = async () => {
    try {
      const response = await invoke<CoachResponse>('get_coach_greeting', {
        userName: 'Player',
        currentElo: 800,
        exercisesCompleted: 0,
      });
      setMessages([response.message]);
    } catch (err) {
      console.error('Failed to load greeting:', err);
    }
  };

  const sendMessage = async () => {
    if (!input.trim()) return;

    const userMessage: CoachMessage = {
      role: 'user',
      content: input,
      timestamp: Date.now(),
      actions: [],
    };

    setMessages(prev => [...prev, userMessage]);
    setInput('');
    setIsTyping(true);

    try {
      const response = await invoke<CoachResponse>('chat_with_coach', {
        message: input,
        context: null,
        apiKey: apiKey,
      });

      setMessages(prev => [...prev, response.message]);
    } catch (err) {
      console.error('Failed to send message:', err);
      const errorMessage: CoachMessage = {
        role: 'gurgeh',
        content: `I encountered an error: ${err}. Please check your API key in Settings.`,
        timestamp: Date.now(),
        actions: [
          {
            action_type: 'open_settings',
            label: 'Open Settings',
            data: '',
          },
        ],
      };
      setMessages(prev => [...prev, errorMessage]);
    } finally {
      setIsTyping(false);
    }
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
        width={360}
        height={480}
      >
        <div className="gurgeh-messages">
          {messages.map((msg, i) => (
            <div key={i} className={`gurgeh-message ${msg.role}`}>
              <div className="gurgeh-message-avatar">
                {msg.role === 'gurgeh' ? '[G]' : '[U]'}
              </div>
              <div className="gurgeh-message-content">
                <div className="gurgeh-message-text">{msg.content}</div>
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
          {isTyping && (
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

        <div className="gurgeh-input-area">
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyPress}
            placeholder="Ask Gurgeh anything..."
            className="xp-input gurgeh-input"
          />
          <XPButton onClick={sendMessage} disabled={!input.trim() || isTyping}>
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
