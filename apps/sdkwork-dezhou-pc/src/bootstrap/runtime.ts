import { configureSdkworkDezhouPcProviders } from './dezhouProviders';
import {
  resolveSdkworkDezhouPcRuntimeConfig,
  type SdkworkDezhouPcRuntimeConfig,
} from './environment';
import {
  createSdkworkDezhouPcIamRuntime,
  createSdkworkDezhouPcSdkClientsWithTokenManager,
  type SdkworkDezhouPcIamRuntime,
} from './iamRuntime';
import { SdkworkDezhouPcRoutes } from './routes';
import {
  createSdkworkDezhouPcSessionStore,
  type SdkworkDezhouPcSessionStore,
} from './sessionStore';
import { createSdkworkDezhouPcSessionTokenManager } from './sessionTokenManager';
import type { SdkworkDezhouPcSdkClientInventory } from './sdkClients';
import type { SdkworkDezhouPcProviders } from './dezhouProviders';

export interface SdkworkDezhouPcRuntime {
  config: SdkworkDezhouPcRuntimeConfig;
  iamRuntime: SdkworkDezhouPcIamRuntime;
  routes: typeof SdkworkDezhouPcRoutes;
  sdkClients: SdkworkDezhouPcSdkClientInventory;
  session: SdkworkDezhouPcSessionStore;
  tableService: SdkworkDezhouPcProviders['tableService'];
}

export function createSdkworkDezhouPcRuntime(): SdkworkDezhouPcRuntime {
  const config = resolveSdkworkDezhouPcRuntimeConfig();
  const session = createSdkworkDezhouPcSessionStore(
    typeof window === 'undefined' ? undefined : window.sessionStorage,
  );
  const tokenManager = createSdkworkDezhouPcSessionTokenManager(session);
  const sdkClients = createSdkworkDezhouPcSdkClientsWithTokenManager(config, tokenManager);
  const iamRuntime = createSdkworkDezhouPcIamRuntime({
    config,
    sdkClients,
    session,
  });
  const { tableService } = configureSdkworkDezhouPcProviders({
    config,
    iamRuntime,
    sdkClients,
  });

  return {
    config,
    iamRuntime,
    routes: SdkworkDezhouPcRoutes,
    sdkClients,
    session,
    tableService,
  };
}
