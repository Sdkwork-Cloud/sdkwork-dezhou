import { create } from 'zustand';
import { persist } from 'zustand/middleware';

export type DezhouLanguage = 'zh' | 'en';

interface DezhouConfigState {
  language: DezhouLanguage;
  setLanguage: (language: DezhouLanguage) => void;
}

export const useDezhouConfigStore = create<DezhouConfigState>()(
  persist(
    (set) => ({
      language: 'zh',
      setLanguage: (language) => set({ language }),
    }),
    {
      name: 'sdkwork-dezhou-pc-config',
    },
  ),
);
