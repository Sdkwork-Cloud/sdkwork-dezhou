import { createTokenManager, type AuthTokenManager } from '@sdkwork/sdk-common';

import type { SdkworkDezhouPcSessionStore } from './sessionStore';

export function createSdkworkDezhouPcSessionTokenManager(
  session: SdkworkDezhouPcSessionStore,
): AuthTokenManager {
  const tokenManager = createTokenManager();

  const hydrate = () => {
    const snapshot = session.getSnapshot();
    tokenManager.setTokens({
      accessToken: snapshot.accessToken,
      authToken: snapshot.authToken,
      refreshToken: snapshot.refreshToken,
    });
  };

  hydrate();
  session.subscribe(() => {
    hydrate();
  });

  return tokenManager;
}
