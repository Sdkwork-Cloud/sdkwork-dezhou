import { listSdkworkDezhouPcAppSdkFamilies } from 'sdkwork-dezhou-pc-core/composition';
import type { SdkworkDezhouAppClient } from '@sdkwork-internal/dezhou-app-sdk-generated';

import type { SdkworkDezhouPcRuntimeConfig } from './environment';

export interface SdkworkDezhouPcSdkClientInventory {
  appApiBaseUrl: string;
  backendApiBaseUrl?: string;
  dezhouAppClient: SdkworkDezhouAppClient;
  sdkFamilies: {
    app: string[];
  };
}

export function listSdkworkDezhouPcRegisteredSdkFamilies(
  config: SdkworkDezhouPcRuntimeConfig,
): SdkworkDezhouPcSdkClientInventory['sdkFamilies'] {
  void config;
  return {
    app: listSdkworkDezhouPcAppSdkFamilies()
      .filter((sdkFamily) => sdkFamily.surface === 'app')
      .map((sdkFamily) => sdkFamily.family),
  };
}
