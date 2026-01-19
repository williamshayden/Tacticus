import React, { useEffect, useState } from 'react';
import { XPWindow } from './xp/XPWindow';
import { XPButton } from './xp/XPButton';
import { XPProgress } from './xp/XPProgress';
import { XPPanel } from './xp/XPPanel';
import { ChessBoard } from './board/ChessBoard';
import { useTrainingStore } from '../stores/trainingStore';
import './TrainMode.css';

interface TrainModeProps {
  onBack: () => void;
  onCalibrationNeeded: () => void;
}

export const TrainMode: React.FC<TrainModeProps> = ({ onBack, onCalibrationNeeded }) => {
  const [hintText, setHintText] = useState<string | null>(null);

  const {
    session,
    currentExercise,
    currentExerciseIndex,
    exerciseResult,
    score,
    streak,
    hintsUsed,
    loading,
    startTrainingSession,
    checkSolution,
    nextExercise,
    getHint,
    resetExercise,
    endSession,
  } = useTrainingStore();

  useEffect(() => {
    if (!session) {
      startTrainingSession(10);
    }
  }, []);

  const handleMove = (from: string, to: string): boolean => {
    if (!currentExercise || exerciseResult) return false;
    
    const move = from + to;
    checkSolution(move);
    return true;
  };

  const handleHint = async () => {
    const hint = await getHint();
    if (hint) {
      setHintText(hint);
    }
  };

  const handleNext = () => {
    setHintText(null);
    
    if (session && currentExerciseIndex >= session.exercises.length - 1) {
      onCalibrationNeeded();
    } else {
      nextExercise();
    }
  };

  const handleRetry = () => {
    setHintText(null);
    resetExercise();
  };

  if (loading || !session) {
    return (
      <div className="train-mode-container">
        <div className="loading-message">Loading exercises...</div>
      </div>
    );
  }

  const progress = ((currentExerciseIndex + (exerciseResult ? 1 : 0)) / session.exercises.length) * 100;

  return (
    <div className="train-mode-container">
      <div className="train-layout">
        {/* Chess Board */}
        <XPWindow
          title={`Exercise ${currentExerciseIndex + 1}/${session.exercises.length}`}
          icon="[T]"
          width={520}
          height="auto"
        >
          {currentExercise && (
            <ChessBoard
              fen={currentExercise.fen}
              onMove={handleMove}
              interactive={!exerciseResult}
            />
          )}
        </XPWindow>

        {/* Sidebar */}
        <div className="train-sidebar">
          {/* Exercise Info */}
          <XPWindow title="Exercise Info" icon="[i]" width={320} height="auto">
            {currentExercise && (
              <div className="exercise-info">
                <h3 className="exercise-title">{currentExercise.title}</h3>
                <div className="exercise-meta">
                  <span className={`difficulty ${currentExercise.difficulty.toLowerCase()}`}>
                    {'*'.repeat(
                      currentExercise.difficulty === 'Beginner' ? 1 :
                      currentExercise.difficulty === 'Intermediate' ? 2 :
                      currentExercise.difficulty === 'Advanced' ? 3 : 4
                    )} {currentExercise.difficulty}
                  </span>
                  <span className="exercise-type">{currentExercise.exercise_type}</span>
                </div>
                
                <XPPanel className="exercise-description">
                  {currentExercise.description}
                </XPPanel>

                {hintText && (
                  <div className="hint-box">
                    <span>[?]</span>
                    <span>{hintText}</span>
                  </div>
                )}

                {exerciseResult && (
                  <div className={`result-box ${exerciseResult.correct ? 'correct' : 'incorrect'}`}>
                    <span>{exerciseResult.correct ? '[OK]' : '[X]'}</span>
                    <div>
                      <p>{exerciseResult.explanation}</p>
                      {exerciseResult.correct_move && (
                        <p className="correct-move">Correct move: {exerciseResult.correct_move}</p>
                      )}
                    </div>
                  </div>
                )}

                <div className="exercise-actions">
                  {!exerciseResult && (
                    <XPButton onClick={handleHint} disabled={hintsUsed >= 2}>
                      Hint ({2 - hintsUsed})
                    </XPButton>
                  )}
                  {exerciseResult && !exerciseResult.correct && (
                    <XPButton onClick={handleRetry}>
                      Retry
                    </XPButton>
                  )}
                  {exerciseResult && (
                    <XPButton primary onClick={handleNext}>
                      {currentExerciseIndex >= session.exercises.length - 1 ? 'Finish' : 'Next'}
                    </XPButton>
                  )}
                </div>
              </div>
            )}
          </XPWindow>

          {/* Progress */}
          <XPWindow title="Session Progress" icon="[#]" width={320} height="auto">
            <div className="session-progress">
              <div className="progress-stats">
                <div className="stat">
                  <span className="stat-value">{score}</span>
                  <span className="stat-label">Score</span>
                </div>
                <div className="stat">
                  <span className="stat-value">{streak}</span>
                  <span className="stat-label">Streak</span>
                </div>
                <div className="stat">
                  <span className="stat-value">{currentExerciseIndex + 1}/{session.exercises.length}</span>
                  <span className="stat-label">Progress</span>
                </div>
              </div>

              <XPProgress value={progress} />

              <div className="focus-areas">
                <label>Focus Areas:</label>
                <div className="focus-tags">
                  {session.focus_areas.map((area, i) => (
                    <span key={i} className="focus-tag">{area}</span>
                  ))}
                </div>
              </div>

              <div className="session-actions">
                <XPButton onClick={() => { endSession(); onBack(); }}>
                  End Session
                </XPButton>
              </div>
            </div>
          </XPWindow>
        </div>
      </div>
    </div>
  );
};
