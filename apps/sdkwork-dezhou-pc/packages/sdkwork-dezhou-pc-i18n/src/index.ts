export const dezhouMessages = {
  'zh-CN': {
    appTitle: '德州扑克',
    lobby: '牌桌大厅',
    login: '进入牌桌',
  },
  'en-US': {
    appTitle: "Texas Hold'em",
    lobby: 'Table Lobby',
    login: 'Enter Lobby',
  },
} as const;

export type DezhouLocale = keyof typeof dezhouMessages;
