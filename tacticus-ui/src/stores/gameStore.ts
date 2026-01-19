import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface GameState {
  fen: string;
  turn: string;
  is_checkmate: boolean;
  is_stalemate: boolean;
  is_check: boolean;
  legal_moves: string[];
  last_move: string | null;
  evaluation: number;
}

interface MoveResult {
  success: boolean;
  new_state: GameState | null;
  error: string | null;
}

interface EngineMove {
  uci: string;
  san: string;
  evaluation: number;
}

interface GameStore {
  gameState: GameState | null;
  selectedSquare: string | null;
  legalMovesForSelected: string[];
  isThinking: boolean;
  gameHistory: string[];
  playerColor: 'white' | 'black';
  engineElo: number;

  // Actions
  startNewGame: (playerColor?: 'white' | 'black') => Promise<void>;
  selectSquare: (square: string) => void;
  makeMove: (from: string, to: string, promotion?: string) => Promise<boolean>;
  makeEngineMove: () => Promise<void>;
  loadPosition: (fen: string) => Promise<void>;
  setEngineElo: (elo: number) => void;
  resetSelection: () => void;
}

export const useGameStore = create<GameStore>((set, get) => ({
  gameState: null,
  selectedSquare: null,
  legalMovesForSelected: [],
  isThinking: false,
  gameHistory: [],
  playerColor: 'white',
  engineElo: 800,

  startNewGame: async (playerColor = 'white') => {
    try {
      const gameState = await invoke<GameState>('get_initial_position');
      set({ 
        gameState, 
        selectedSquare: null, 
        legalMovesForSelected: [],
        gameHistory: [],
        playerColor 
      });
      
      // If player is black, let engine move first
      if (playerColor === 'black') {
        get().makeEngineMove();
      }
    } catch (err) {
      console.error('Failed to start new game:', err);
    }
  },

  selectSquare: (square: string) => {
    const { gameState, selectedSquare, playerColor } = get();
    if (!gameState) return;

    // Check if it's player's turn
    if (gameState.turn !== playerColor) return;

    // If clicking on a piece of own color, select it
    const legalFromSquare = gameState.legal_moves
      .filter(move => move.startsWith(square))
      .map(move => move.slice(2, 4));

    if (legalFromSquare.length > 0) {
      set({ selectedSquare: square, legalMovesForSelected: legalFromSquare });
    } else if (selectedSquare) {
      // Try to make a move
      get().makeMove(selectedSquare, square);
    }
  },

  makeMove: async (from: string, to: string, promotion?: string) => {
    const { gameState } = get();
    if (!gameState) return false;

    const uciMove = from + to + (promotion || '');
    
    try {
      const result = await invoke<MoveResult>('make_move', { 
        fen: gameState.fen, 
        uciMove 
      });

      if (result.success && result.new_state) {
        set({ 
          gameState: result.new_state, 
          selectedSquare: null, 
          legalMovesForSelected: [],
          gameHistory: [...get().gameHistory, uciMove]
        });

        // If game not over and it's engine's turn, make engine move
        if (!result.new_state.is_checkmate && !result.new_state.is_stalemate) {
          setTimeout(() => get().makeEngineMove(), 500);
        }
        return true;
      }
      return false;
    } catch (err) {
      console.error('Move failed:', err);
      return false;
    }
  },

  makeEngineMove: async () => {
    const { gameState } = get();
    const engineElo = get().engineElo;
    if (!gameState || gameState.is_checkmate || gameState.is_stalemate) return;

    set({ isThinking: true });

    try {
      const engineMove = await invoke<EngineMove>('get_engine_move', { 
        fen: gameState.fen, 
        engineElo 
      });

      const result = await invoke<MoveResult>('make_move', { 
        fen: gameState.fen, 
        uciMove: engineMove.uci 
      });

      if (result.success && result.new_state) {
        set({ 
          gameState: result.new_state,
          gameHistory: [...get().gameHistory, engineMove.uci],
          isThinking: false
        });
      }
    } catch (err) {
      console.error('Engine move failed:', err);
      set({ isThinking: false });
    }
  },

  loadPosition: async (fen: string) => {
    try {
      const gameState = await invoke<GameState>('get_position_from_fen', { fen });
      set({ gameState, selectedSquare: null, legalMovesForSelected: [] });
    } catch (err) {
      console.error('Failed to load position:', err);
    }
  },

  setEngineElo: (elo: number) => {
    set({ engineElo: elo });
  },

  resetSelection: () => {
    set({ selectedSquare: null, legalMovesForSelected: [] });
  },
}));
