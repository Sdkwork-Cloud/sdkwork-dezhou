import { dezhouMessages } from 'sdkwork-dezhou-pc-i18n';

interface AuthProps {
  setCurrentView: (view: string) => void;
}

export function Auth({ setCurrentView }: AuthProps) {
  const messages = dezhouMessages['zh-CN'];

  return (
    <div className="h-full flex items-center justify-center">
      <div className="w-full max-w-md p-8 rounded-2xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 shadow-sm">
        <h2 className="text-2xl font-semibold mb-2">{messages.appTitle}</h2>
        <p className="text-sm text-zinc-500 mb-6">SDKWork 德州扑克平台</p>
        <button
          type="button"
          className="w-full py-2.5 rounded-lg bg-emerald-600 text-white font-medium hover:bg-emerald-700"
          onClick={() => setCurrentView('lobby')}
        >
          {messages.login}
        </button>
      </div>
    </div>
  );
}
