import { appApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { DezhouHealthResponse } from '../types';


export class HealthDezhouReadyApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async check(): Promise<DezhouHealthResponse> {
    return this.client.get<DezhouHealthResponse>(appApiPath(`/system/ready`));
  }
}

export class HealthDezhouHealthApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async check(): Promise<DezhouHealthResponse> {
    return this.client.get<DezhouHealthResponse>(appApiPath(`/system/health`));
  }
}

export class HealthDezhouApi {
  private client: HttpClient;
  public readonly health: HealthDezhouHealthApi;
  public readonly ready: HealthDezhouReadyApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.health = new HealthDezhouHealthApi(client);
    this.ready = new HealthDezhouReadyApi(client);
  }

}

export class HealthApi {
  private client: HttpClient;
  public readonly dezhou: HealthDezhouApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.dezhou = new HealthDezhouApi(client);
  }

}

export function createHealthApi(client: HttpClient): HealthApi {
  return new HealthApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}
