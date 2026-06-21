export interface DezhouTableItem {
  id: string;
  tableCode: string;
  title: string;
  summary?: string;
  maxSeats?: number;
  currentSeats?: number;
  status: string;
}
