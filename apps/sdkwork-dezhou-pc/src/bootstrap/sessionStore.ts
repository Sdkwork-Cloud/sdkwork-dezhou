export interface SdkworkDezhouPcSessionSnapshot {
  accessToken?: string;
  authToken?: string;
  refreshToken?: string;
  sessionId?: string;
  context?: {
    tenantId?: string;
    userId?: string;
    organizationId?: string;
    sessionId?: string;
    appId?: string;
    environment?: string;
    deploymentMode?: string;
  };
  updatedAt?: string;
}

export interface SdkworkDezhouPcSessionStorageLike {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
}

export interface SdkworkDezhouPcSessionStore {
  clearSession(): void;
  getSnapshot(): SdkworkDezhouPcSessionSnapshot;
  refreshSession(): SdkworkDezhouPcSessionSnapshot;
  setSession(nextSession: SdkworkDezhouPcSessionSnapshot): void;
  subscribe(listener: (snapshot: SdkworkDezhouPcSessionSnapshot) => void): () => void;
}

export const SDKWORK_DEZHOU_PC_SESSION_STORAGE_KEY = 'sdkwork-dezhou-pc-session';

function readInitialSession(
  storage: SdkworkDezhouPcSessionStorageLike | undefined,
  storageKey: string,
): SdkworkDezhouPcSessionSnapshot {
  if (!storage) {
    return {};
  }

  try {
    const raw = storage.getItem(storageKey);
    return raw ? (JSON.parse(raw) as SdkworkDezhouPcSessionSnapshot) : {};
  } catch {
    return {};
  }
}

export function createSdkworkDezhouPcSessionStore(
  storage?: SdkworkDezhouPcSessionStorageLike,
  storageKey = SDKWORK_DEZHOU_PC_SESSION_STORAGE_KEY,
): SdkworkDezhouPcSessionStore {
  let snapshot = readInitialSession(storage, storageKey);
  const listeners = new Set<(nextSnapshot: SdkworkDezhouPcSessionSnapshot) => void>();

  const emit = () => {
    for (const listener of listeners) {
      listener(snapshot);
    }
  };

  const persist = () => {
    if (!storage) {
      return;
    }

    if (!snapshot.authToken && !snapshot.accessToken && !snapshot.refreshToken) {
      storage.removeItem(storageKey);
      return;
    }

    storage.setItem(storageKey, JSON.stringify(snapshot));
  };

  return {
    clearSession() {
      snapshot = {};
      persist();
      emit();
    },
    getSnapshot() {
      return snapshot;
    },
    refreshSession() {
      snapshot = readInitialSession(storage, storageKey);
      emit();
      return snapshot;
    },
    setSession(nextSession) {
      snapshot = {
        ...nextSession,
        updatedAt: new Date().toISOString(),
      };
      persist();
      emit();
    },
    subscribe(listener) {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
  };
}

export function hasSdkworkDezhouPcIamSession(snapshot: SdkworkDezhouPcSessionSnapshot): boolean {
  return Boolean(snapshot.authToken && snapshot.accessToken && snapshot.context?.tenantId);
}
