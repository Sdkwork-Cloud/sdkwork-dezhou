import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

interface SidebarProps {
  currentView: string;
  setCurrentView: (view: string) => void;
}

export function Sidebar({ currentView, setCurrentView }: SidebarProps) {
  const items = [
    { id: 'lobby', label: '牌桌大厅' },
    { id: 'profile', label: '个人中心' },
  ];

  return (
    <aside className="w-56 border-r border-zinc-200 dark:border-zinc-800 p-4 space-y-2">
      <h1 className="text-lg font-semibold mb-4">德州扑克</h1>
      {items.map((item) => (
        <button
          key={item.id}
          type="button"
          onClick={() => setCurrentView(item.id)}
          className={cn(
            'w-full text-left px-3 py-2 rounded-lg text-sm',
            currentView === item.id
              ? 'bg-emerald-600 text-white'
              : 'hover:bg-zinc-100 dark:hover:bg-zinc-900',
          )}
        >
          {item.label}
        </button>
      ))}
    </aside>
  );
}

interface TopbarProps {
  onLogout: () => void;
}

export function Topbar({ onLogout }: TopbarProps) {
  return (
    <header className="h-14 border-b border-zinc-200 dark:border-zinc-800 flex items-center justify-between px-6">
      <span className="text-sm text-zinc-500">SDKWork Dezhou</span>
      <button
        type="button"
        className="text-sm text-zinc-500 hover:text-zinc-900 dark:hover:text-zinc-100"
        onClick={onLogout}
      >
        退出
      </button>
    </header>
  );
}
