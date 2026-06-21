import type { DezhouTableItem } from './dezhou-table-item';

export interface DezhouTablePage {
  items: DezhouTableItem[];
  total: number;
  page: number;
  pageSize: number;
}
