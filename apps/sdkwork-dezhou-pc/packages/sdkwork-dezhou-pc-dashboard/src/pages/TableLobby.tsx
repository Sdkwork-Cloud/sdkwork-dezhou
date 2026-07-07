import { useEffect, useState } from 'react';
import { getDezhouTableService, useDezhouUserStore } from 'sdkwork-dezhou-pc-core';
import type { DezhouTableItem } from 'sdkwork-dezhou-pc-core';
import { dezhouMessages } from 'sdkwork-dezhou-pc-i18n';
export function TableLobby() {
  const messages = dezhouMessages['zh-CN'];
  const displayName = useDezhouUserStore((s) => s.displayName);
  const chipBalance = useDezhouUserStore((s) => s.chipBalance);
  const [tables, setTables] = useState<DezhouTableItem[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let active = true;
    getDezhouTableService()
      .listTables()
      .then((page) => {
        if (!active) return;
        setTables(page?.items ?? []);
      })
      .catch(() => {
        if (active) setTables([]);
      })
      .finally(() => {
        if (active) setLoading(false);
      });
    return () => {
      active = false;
    };
  }, []);

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-semibold">{messages.lobby}</h2>
        <p className="text-sm text-zinc-500 mt-1">
          欢迎，{displayName} · 筹码 {chipBalance.toLocaleString()}
        </p>
      </div>
      {loading ? (
        <p className="text-sm text-zinc-500">加载牌桌中…</p>
      ) : tables.length === 0 ? (
        <p className="text-sm text-zinc-500">暂无开放牌桌</p>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
          {tables.map((table) => (
            <article
              key={table.id}
              className="p-4 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900"
            >
              <h3 className="font-medium">{table.title}</h3>
              <p className="text-sm text-zinc-500 mt-1">
                座位 {table.currentSeats ?? 0}/{table.maxSeats ?? 9}
              </p>
              <span className="inline-block mt-3 text-xs px-2 py-1 rounded-full bg-emerald-100 text-emerald-800 dark:bg-emerald-900/40 dark:text-emerald-200">
                {table.status}
              </span>
            </article>
          ))}
        </div>
      )}
    </div>
  );
}

interface ProfileProps {
  setCurrentView: (view: string) => void;
}

export function Profile({ setCurrentView }: ProfileProps) {
  const displayName = useDezhouUserStore((s) => s.displayName);
  const setDisplayName = useDezhouUserStore((s) => s.setDisplayName);

  return (
    <div className="max-w-md space-y-4">
      <h2 className="text-2xl font-semibold">个人中心</h2>
      <label className="block text-sm">
        昵称
        <input
          className="mt-1 w-full px-3 py-2 rounded-lg border border-zinc-200 dark:border-zinc-700 bg-transparent"
          value={displayName}
          onChange={(e) => setDisplayName(e.target.value)}
        />
      </label>
      <button
        type="button"
        className="text-sm text-emerald-600"
        onClick={() => setCurrentView('lobby')}
      >
        返回大厅
      </button>
    </div>
  );
}
