import { useState } from 'react';
import { Sidebar, Topbar } from 'sdkwork-dezhou-pc-commons';
import { Profile, TableLobby } from 'sdkwork-dezhou-pc-dashboard';

export interface DezhouAppShellProps {
  onLogout: () => void;
}

export default function DezhouAppShell({ onLogout }: DezhouAppShellProps) {
  const [currentView, setCurrentView] = useState('lobby');

  const content =
    currentView === 'profile' ? (
      <Profile setCurrentView={setCurrentView} />
    ) : (
      <TableLobby />
    );

  return (
    <div className="flex h-screen bg-zinc-50 dark:bg-zinc-950 text-zinc-900 dark:text-zinc-50">
      <Sidebar currentView={currentView} setCurrentView={setCurrentView} />
      <div className="flex-1 flex flex-col">
        <Topbar onLogout={onLogout} />
        <main className="flex-1 overflow-y-auto p-6">{content}</main>
      </div>
    </div>
  );
}
