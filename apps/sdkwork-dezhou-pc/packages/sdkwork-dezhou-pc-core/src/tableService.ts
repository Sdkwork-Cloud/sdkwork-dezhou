import type { DezhouTableItem } from '@sdkwork-internal/dezhou-app-sdk-generated';
import type { SdkworkDezhouAppClient } from '@sdkwork-internal/dezhou-app-sdk-generated';

export type { DezhouTableItem };

export interface DezhouTableListParams {
  page?: number;
  pageSize?: number;
  status?: string;
}

export interface DezhouTablePage {
  items?: DezhouTableItem[];
}

export interface DezhouTableService {
  listTables(params?: DezhouTableListParams): Promise<DezhouTablePage>;
}

let configuredTableService: DezhouTableService | null = null;

export function configureDezhouTableService(service: DezhouTableService): void {
  configuredTableService = service;
}

export function getDezhouTableService(): DezhouTableService {
  if (!configuredTableService) {
    throw new Error(
      'Dezhou table service is not configured. Bootstrap sdkwork-dezhou-pc runtime first.',
    );
  }
  return configuredTableService;
}

export function createDezhouTableService(client: SdkworkDezhouAppClient): DezhouTableService {
  return {
    async listTables(params) {
      const result = await client.dezhou.table.list(params);
      return result.data as DezhouTablePage;
    },
  };
}
