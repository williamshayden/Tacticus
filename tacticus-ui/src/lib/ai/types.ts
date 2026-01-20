// Types for the AI agent

export interface Game {
  id: number;
  profile_id: number;
  initial_fen: string;
  final_fen: string;
  moves: string[];
  result: string;
  player_color: string;
  opponent_type: string;
  opponent_elo: number | null;
  analysis: string | null;
  mistakes: number;
  blunders: number;
  opening_name: string | null;
  created_at: string;
  finished_at: string | null;
}

export interface PlayerStats {
  current_elo: number;
  peak_elo: number;
  games_played: number;
  wins: number;
  losses: number;
  draws: number;
  win_rate: number;
  exercises_completed: number;
  exercises_solved: number;
  exercise_success_rate: number;
  streak: number;
  style: string;
  weaknesses: string[];
  strengths: string[];
}

export interface TrainingProgress {
  total_attempted: number;
  total_solved: number;
  success_rate: number;
  avg_time_seconds: number;
  avg_hints_used: number;
}

export interface ImprovementTrend {
  elo_change: number;
  games_in_period: number;
  win_rate_in_period: number;
  exercises_in_period: number;
  exercise_success_rate_in_period: number;
}

export interface WeaknessEntry {
  exercise_type: string;
  total_attempts: number;
  success_rate: number;
  recent_trend: string;
}

export interface Conversation {
  id: number;
  profile_id: number;
  title: string | null;
  context: string | null;
  created_at: string;
  updated_at: string;
}

export interface Message {
  id: number;
  conversation_id: number;
  role: string;
  content: string;
  tool_calls: string | null;
  tool_results: string | null;
  created_at: string;
}

export interface CoachAction {
  action_type: string;
  label: string;
  data: string;
}

export interface ChatMessage {
  role: 'user' | 'assistant';
  content: string;
  actions?: CoachAction[];
}
