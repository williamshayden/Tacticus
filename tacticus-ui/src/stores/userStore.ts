import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface UserProfile {
  id: number;
  name: string;
  initial_level: string;
  current_elo: number;
  peak_elo: number;
  games_played: number;
  exercises_completed: number;
  streak: number;
  style: string;
  weaknesses: string[];
  strengths: string[];
  created_at: string;
}

interface UserStats {
  current_elo: number;
  peak_elo: number;
  games_played: number;
  exercises_completed: number;
  streak: number;
  style: string;
  exercises_until_calibration: number;
}

interface UserStore {
  profile: UserProfile | null;
  stats: UserStats | null;
  hasOnboarded: boolean;
  apiKey: string | null;
  loading: boolean;
  error: string | null;

  // Actions
  checkOnboarding: () => Promise<boolean>;
  createProfile: (name: string, level: string) => Promise<void>;
  loadProfile: () => Promise<void>;
  loadStats: () => Promise<void>;
  updateElo: (newElo: number, result: string) => Promise<void>;
  setApiKey: (key: string) => Promise<void>;
  loadApiKey: () => Promise<void>;
}

export const useUserStore = create<UserStore>((set, get) => ({
  profile: null,
  stats: null,
  hasOnboarded: false,
  apiKey: null,
  loading: false,
  error: null,

  checkOnboarding: async () => {
    try {
      const hasOnboarded = await invoke<boolean>('has_completed_onboarding');
      set({ hasOnboarded });
      return hasOnboarded;
    } catch (err) {
      set({ error: String(err) });
      return false;
    }
  },

  createProfile: async (name: string, level: string) => {
    set({ loading: true, error: null });
    try {
      const profile = await invoke<UserProfile>('create_user_profile', { name, initialLevel: level });
      set({ profile, hasOnboarded: true, loading: false });
    } catch (err) {
      set({ error: String(err), loading: false });
    }
  },

  loadProfile: async () => {
    set({ loading: true, error: null });
    try {
      const profile = await invoke<UserProfile | null>('get_user_profile');
      set({ profile, loading: false });
    } catch (err) {
      set({ error: String(err), loading: false });
    }
  },

  loadStats: async () => {
    try {
      const stats = await invoke<UserStats | null>('get_user_stats');
      set({ stats });
    } catch (err) {
      set({ error: String(err) });
    }
  },

  updateElo: async (newElo: number, result: string) => {
    try {
      const profile = await invoke<UserProfile>('update_user_elo', { newElo, gameResult: result });
      set({ profile });
      get().loadStats();
    } catch (err) {
      set({ error: String(err) });
    }
  },

  setApiKey: async (key: string) => {
    try {
      await invoke('save_api_key', { apiKey: key });
      set({ apiKey: key });
    } catch (err) {
      set({ error: String(err) });
    }
  },

  loadApiKey: async () => {
    try {
      const apiKey = await invoke<string | null>('get_api_key');
      set({ apiKey });
    } catch (err) {
      set({ error: String(err) });
    }
  },
}));
