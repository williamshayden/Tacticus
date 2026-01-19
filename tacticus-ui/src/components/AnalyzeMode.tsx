import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { XPWindow } from './xp/XPWindow';
import { XPButton } from './xp/XPButton';
import { XPPanel } from './xp/XPPanel';
import { XPInput } from './xp/XPInput';
import { ChessBoard } from './board/ChessBoard';
import { useUserStore } from '../stores/userStore';
import './AnalyzeMode.css';

interface CoachResponse {
  message: {
    content: string;
  };
}

interface AnalyzeModeProps {
  onBack: () => void;
}

export const AnalyzeMode: React.FC<AnalyzeModeProps> = ({ onBack }) => {
  const [fen, setFen] = useState('rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1');
  const [evaluation, setEvaluation] = useState<number | null>(null);
  const [arrows, setArrows] = useState<{ from: string; to: string; color?: string }[]>([]);
  const [highlights] = useState<{ square: string; color?: string }[]>([]);
  const [analysisText, setAnalysisText] = useState<string>('');
  const [loading, setLoading] = useState(false);
  const { apiKey } = useUserStore();

  const handleEvaluate = async () => {
    setLoading(true);
    try {
      const eval_score = await invoke<number>('evaluate_position', { fen });
      setEvaluation(eval_score);
      setAnalysisText(`Position evaluation: ${eval_score >= 0 ? '+' : ''}${eval_score.toFixed(2)}\n\n${
        eval_score > 1 ? 'White has a significant advantage.' :
        eval_score < -1 ? 'Black has a significant advantage.' :
        'The position is roughly equal.'
      }`);
    } catch (err) {
      setAnalysisText(`Error: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleBestMove = async () => {
    setLoading(true);
    try {
      const move = await invoke<{ uci: string; evaluation: number }>('get_engine_move', { 
        fen, 
        engineElo: 2000 
      });
      
      setArrows([{
        from: move.uci.slice(0, 2),
        to: move.uci.slice(2, 4),
        color: 'rgba(0, 200, 0, 0.7)'
      }]);
      
      setAnalysisText(`Best move: ${move.uci}\nEvaluation: ${move.evaluation >= 0 ? '+' : ''}${move.evaluation.toFixed(2)}`);
    } catch (err) {
      setAnalysisText(`Error: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleGurgrehAnalysis = async () => {
    if (!apiKey) {
      setAnalysisText('API key required. Please configure your OpenRouter API key in Settings.');
      return;
    }
    
    setLoading(true);
    setAnalysisText('Gurgeh is analyzing the position...');
    
    try {
      const response = await invoke<CoachResponse>('analyze_position_with_coach', { 
        fen,
        apiKey 
      });
      setAnalysisText(response.message.content);
    } catch (err) {
      setAnalysisText(`Error: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleFenChange = async (newFen: string) => {
    setFen(newFen);
    setArrows([]);
    setEvaluation(null);
    setAnalysisText('');
  };

  const handleReset = () => {
    setFen('rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1');
    setArrows([]);
    setEvaluation(null);
    setAnalysisText('');
  };

  return (
    <div className="analyze-mode-container">
      <div className="analyze-layout">
        {/* Chess Board */}
        <XPWindow title="Analysis Board" icon="[A]" width={520} height="auto">
          <ChessBoard
            fen={fen}
            interactive={false}
            arrows={arrows}
            highlights={highlights}
          />
        </XPWindow>

        {/* Analysis Sidebar */}
        <div className="analyze-sidebar">
          {/* Position */}
          <XPWindow title="Position" icon="[F]" width={350} height="auto">
            <div className="position-section">
              <label>FEN String</label>
              <XPInput
                value={fen}
                onChange={handleFenChange}
                className="fen-input"
              />
              <div className="position-actions">
                <XPButton onClick={handleReset}>Reset</XPButton>
                <XPButton onClick={() => navigator.clipboard.writeText(fen)}>
                  Copy FEN
                </XPButton>
              </div>
            </div>
          </XPWindow>

          {/* Analysis Tools */}
          <XPWindow title="Analysis" icon="[#]" width={350} height="auto">
            <div className="analysis-tools">
              <div className="analysis-buttons">
                <XPButton onClick={handleEvaluate} disabled={loading}>
                  Evaluate Position
                </XPButton>
                <XPButton onClick={handleBestMove} disabled={loading}>
                  Find Best Move
                </XPButton>
                <XPButton onClick={handleGurgrehAnalysis} disabled={loading || !apiKey} primary>
                  Ask Gurgeh
                </XPButton>
              </div>

              {evaluation !== null && (
                <div className="eval-display">
                  <div className="eval-bar">
                    <div 
                      className="eval-white"
                      style={{ width: `${50 + Math.max(-50, Math.min(50, evaluation * 10))}%` }}
                    />
                  </div>
                  <span className="eval-value">
                    {evaluation >= 0 ? '+' : ''}{evaluation.toFixed(2)}
                  </span>
                </div>
              )}

              {analysisText && (
                <XPPanel className="analysis-output">
                  {analysisText}
                </XPPanel>
              )}
              
              {!apiKey && (
                <div className="api-key-notice">
                  [!] Configure API key in Settings to enable AI analysis
                </div>
              )}
            </div>
          </XPWindow>

          <div className="analyze-back">
            <XPButton onClick={onBack}>Back to Hub</XPButton>
          </div>
        </div>
      </div>
    </div>
  );
};
