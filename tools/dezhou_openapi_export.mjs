#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { isBlank } from '@sdkwork/utils';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const outputDir = path.join(root, 'generated/openapi');

const OWNER = 'sdkwork-dezhou';
const DOMAIN = 'game';

const sharedComponents = {
  securitySchemes: {
    AuthToken: {
      type: 'http',
      scheme: 'bearer',
      bearerFormat: 'SDKWork-Auth-Token',
      description: 'SDKWork dual-token auth principal (Authorization bearer).',
    },
    AccessToken: {
      type: 'apiKey',
      in: 'header',
      name: 'Access-Token',
      description: 'SDKWork dual-token access credential header.',
    },
  },
  parameters: {
    TableIdPath: {
      name: 'tableId',
      in: 'path',
      required: true,
      schema: { type: 'string' },
    },
    PageQuery: {
      name: 'page',
      in: 'query',
      required: false,
      schema: { type: 'integer', minimum: 1, default: 1 },
    },
    PageSizeQuery: {
      name: 'page_size',
      in: 'query',
      required: false,
      schema: { type: 'integer', minimum: 1, maximum: 200, default: 20 },
    },
    StatusQuery: {
      name: 'status',
      in: 'query',
      required: false,
      schema: { type: 'string' },
    },
  },
  responses: {
    ProblemDetailResponse: {
      description: 'RFC 9457 problem details.',
      content: {
        'application/problem+json': {
          schema: { $ref: '#/components/schemas/ProblemDetail' },
        },
      },
    },
  },
  schemas: {
    ProblemDetail: {
      type: 'object',
      additionalProperties: true,
      required: ['type', 'title', 'status', 'code', 'traceId'],
      properties: {
        type: { type: 'string', format: 'uri' },
        title: { type: 'string' },
        status: { type: 'integer' },
        detail: { type: 'string' },
        instance: { type: 'string' },
        code: { type: 'integer', format: 'int32', minimum: 40001, maximum: 79999 },
        traceId: { type: 'string' },
      },
    },
    SdkWorkApiResponse: {
      type: 'object',
      additionalProperties: false,
      required: ['code', 'data', 'traceId'],
      properties: {
        code: { type: 'integer', format: 'int32', enum: [0], default: 0 },
        data: {},
        traceId: { type: 'string' },
      },
    },
    DezhouTableItem: {
      type: 'object',
      additionalProperties: false,
      required: ['id', 'tableCode', 'title', 'status'],
      properties: {
        id: { type: 'string' },
        tableCode: { type: 'string' },
        title: { type: 'string' },
        summary: { type: 'string' },
        maxSeats: { type: 'integer' },
        currentSeats: { type: 'integer' },
        status: { type: 'string' },
      },
    },
    PageInfo: {
      type: 'object',
      additionalProperties: false,
      required: ['mode', 'page', 'pageSize', 'totalItems', 'totalPages'],
      properties: {
        mode: { type: 'string', enum: ['offset'] },
        page: { type: 'integer', minimum: 1 },
        pageSize: { type: 'integer', minimum: 1 },
        totalItems: { type: 'string', pattern: '^[0-9]+$' },
        totalPages: { type: 'integer', minimum: 0 },
        hasMore: { type: 'boolean' },
      },
    },
    DezhouTableResponse: responseSchema({
      type: 'object',
      additionalProperties: false,
      required: ['item'],
      properties: { item: { $ref: '#/components/schemas/DezhouTableItem' } },
    }),
    DezhouTableListResponse: responseSchema({
      type: 'object',
      additionalProperties: false,
      required: ['items', 'pageInfo'],
      properties: {
        items: { type: 'array', items: { $ref: '#/components/schemas/DezhouTableItem' } },
        pageInfo: { $ref: '#/components/schemas/PageInfo' },
      },
    }),
  },
};

const dualTokenSecurity = [{ AuthToken: [], AccessToken: [] }];

function responseSchema(dataSchema) {
  return {
    allOf: [
      { $ref: '#/components/schemas/SdkWorkApiResponse' },
      { type: 'object', required: ['data'], properties: { data: dataSchema } },
    ],
  };
}

function operation(operationId, tags, apiSurface, options = {}) {
  const { parameters = [], responseSchema: successSchema = 'DezhouTableListResponse' } = options;
  return {
    operationId,
    tags,
    'x-sdkwork-request-context': 'WebRequestContext',
    'x-sdkwork-api-surface': apiSurface,
    'x-sdkwork-owner': OWNER,
    'x-sdkwork-domain': DOMAIN,
    security: dualTokenSecurity,
    ...(parameters.length > 0 ? { parameters } : {}),
    responses: {
      200: {
        description: 'OK',
        content: {
          'application/json': {
            schema: { $ref: `#/components/schemas/${successSchema}` },
          },
        },
      },
      default: { $ref: '#/components/responses/ProblemDetailResponse' },
    },
  };
}

function buildOpenApi(title, serverUrl, operations) {
  return {
    openapi: '3.1.2',
    jsonSchemaDialect: 'https://json-schema.org/draft/2020-12/schema',
    info: {
      title,
      version: '0.1.0',
      description: 'SDKWork dezhou Texas Hold\'em HTTP contract.',
      'x-sdkwork-owner': OWNER,
      'x-sdkwork-domain': DOMAIN,
    },
    servers: [{ url: serverUrl, description: 'SDKWork API root' }],
    paths: operations,
    components: sharedComponents,
  };
}

const appOperations = {
  '/app/v3/api/tables': {
    get: operation('dezhou.table.list', ['dezhou'], 'app-api', {
      parameters: [
        { $ref: '#/components/parameters/PageQuery' },
        { $ref: '#/components/parameters/PageSizeQuery' },
        { $ref: '#/components/parameters/StatusQuery' },
      ],
    }),
  },
  '/app/v3/api/tables/{tableId}': {
    get: operation('dezhou.table.retrieve', ['dezhou'], 'app-api', {
      parameters: [{ $ref: '#/components/parameters/TableIdPath' }],
      responseSchema: 'DezhouTableResponse',
    }),
  },
};

const backendOperations = {
  '/backend/v3/api/tables': {
    get: operation('backend.dezhou.table.list', ['dezhou'], 'backend-api', {
      parameters: [
        { $ref: '#/components/parameters/PageQuery' },
        { $ref: '#/components/parameters/PageSizeQuery' },
        { $ref: '#/components/parameters/StatusQuery' },
      ],
    }),
  },
};

if (isBlank(OWNER)) {
  throw new Error('owner must be non-empty');
}

fs.mkdirSync(path.join(root, 'apis/app-api/game'), { recursive: true });
fs.mkdirSync(path.join(root, 'apis/backend-api/game'), { recursive: true });
fs.mkdirSync(outputDir, { recursive: true });

const appDoc = buildOpenApi('SDKWork Dezhou App API', '/app/v3/api', appOperations);
const backendDoc = buildOpenApi('SDKWork Dezhou Backend API', '/backend/v3/api', backendOperations);

const targets = [
  ['apis/app-api/game/dezhou-app-api.openapi.json', appDoc],
  ['apis/backend-api/game/dezhou-backend-api.openapi.json', backendDoc],
  ['generated/openapi/dezhou-app-api.openapi.json', appDoc],
  ['generated/openapi/dezhou-backend-api.openapi.json', backendDoc],
];

for (const [relativePath, doc] of targets) {
  fs.writeFileSync(path.join(root, relativePath), `${JSON.stringify(doc, null, 2)}\n`);
}

process.stdout.write('[dezhou-openapi] exported L2 app and backend OpenAPI documents\n');
