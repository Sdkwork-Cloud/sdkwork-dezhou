import { configureDezhouTableService, createDezhouTableService } from 'sdkwork-dezhou-pc-core';

import type { SdkworkDezhouPcRuntimeConfig } from './environment';
import type { SdkworkDezhouPcIamRuntime } from './iamRuntime';
import type { SdkworkDezhouPcSdkClientInventory } from './sdkClients';

export interface SdkworkDezhouPcProviders {
  tableService: ReturnType<typeof createDezhouTableService>;
}

export function configureSdkworkDezhouPcProviders(input: {
  config: SdkworkDezhouPcRuntimeConfig;
  iamRuntime: SdkworkDezhouPcIamRuntime;
  sdkClients: SdkworkDezhouPcSdkClientInventory;
}): SdkworkDezhouPcProviders {
  void input.config;
  void input.iamRuntime;
  const tableService = createDezhouTableService(input.sdkClients.dezhouAppClient);
  configureDezhouTableService(tableService);
  return { tableService };
}
