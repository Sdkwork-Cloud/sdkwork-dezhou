import { create } from 'zustand';

export interface DezhouUserState {
  displayName: string;
  chipBalance: number;
  setDisplayName: (name: string) => void;
}

export const useDezhouUserStore = create<DezhouUserState>((set) => ({
  displayName: 'Player',
  chipBalance: 10000,
  setDisplayName: (displayName) => set({ displayName }),
}));

export const API_BASE_URL = 'http://localhost:8096';
