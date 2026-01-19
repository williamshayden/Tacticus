import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface ExerciseData {
  id: number;
  title: string;
  description: string;
  difficulty: string;
  exercise_type: string;
  fen: string;
  hints: string[];
  solution_moves: string[];
}

interface TrainingSession {
  exercises: ExerciseData[];
  focus_areas: string[];
  total_exercises: number;
}

interface ExerciseResult {
  correct: boolean;
  explanation: string;
  correct_move: string | null;
}

interface TrainingStore {
  session: TrainingSession | null;
  currentExerciseIndex: number;
  currentExercise: ExerciseData | null;
  exerciseResult: ExerciseResult | null;
  selectedSquare: string | null;
  score: number;
  streak: number;
  hintsUsed: number;
  loading: boolean;

  // Actions
  startTrainingSession: (count?: number) => Promise<void>;
  checkSolution: (move: string) => Promise<boolean>;
  nextExercise: () => void;
  getHint: () => Promise<string | null>;
  selectSquare: (square: string) => void;
  resetExercise: () => void;
  endSession: () => void;
}

export const useTrainingStore = create<TrainingStore>((set, get) => ({
  session: null,
  currentExerciseIndex: 0,
  currentExercise: null,
  exerciseResult: null,
  selectedSquare: null,
  score: 0,
  streak: 0,
  hintsUsed: 0,
  loading: false,

  startTrainingSession: async (count = 10) => {
    set({ loading: true });
    try {
      const session = await invoke<TrainingSession>('get_training_exercises', {
        count,
        userElo: 800, // TODO: Get from user profile
        weaknesses: [],
      });
      
      set({ 
        session, 
        currentExerciseIndex: 0,
        currentExercise: session.exercises[0] || null,
        exerciseResult: null,
        score: 0,
        streak: 0,
        hintsUsed: 0,
        loading: false
      });
    } catch (err) {
      console.error('Failed to start training session:', err);
      set({ loading: false });
    }
  },

  checkSolution: async (move: string) => {
    const { currentExercise, streak, score } = get();
    if (!currentExercise) return false;

    try {
      const result = await invoke<ExerciseResult>('check_exercise_solution', {
        exerciseId: currentExercise.id,
        userMove: move,
      });

      if (result.correct) {
        const bonus = streak >= 3 ? 50 : 0;
        set({ 
          exerciseResult: result,
          streak: streak + 1,
          score: score + 100 + bonus
        });
      } else {
        set({ 
          exerciseResult: result,
          streak: 0
        });
      }

      return result.correct;
    } catch (err) {
      console.error('Failed to check solution:', err);
      return false;
    }
  },

  nextExercise: () => {
    const { session, currentExerciseIndex } = get();
    if (!session) return;

    const nextIndex = currentExerciseIndex + 1;
    if (nextIndex < session.exercises.length) {
      set({
        currentExerciseIndex: nextIndex,
        currentExercise: session.exercises[nextIndex],
        exerciseResult: null,
        selectedSquare: null,
        hintsUsed: 0,
      });
    }
  },

  getHint: async () => {
    const { currentExercise, hintsUsed } = get();
    if (!currentExercise) return null;

    try {
      const hint = await invoke<string | null>('get_exercise_hint', {
        exerciseId: currentExercise.id,
        hintIndex: hintsUsed,
      });

      if (hint) {
        set({ hintsUsed: hintsUsed + 1 });
      }

      return hint;
    } catch (err) {
      console.error('Failed to get hint:', err);
      return null;
    }
  },

  selectSquare: (square: string) => {
    set({ selectedSquare: square });
  },

  resetExercise: () => {
    set({ 
      exerciseResult: null, 
      selectedSquare: null,
      hintsUsed: 0
    });
  },

  endSession: () => {
    set({
      session: null,
      currentExerciseIndex: 0,
      currentExercise: null,
      exerciseResult: null,
      selectedSquare: null,
    });
  },
}));
