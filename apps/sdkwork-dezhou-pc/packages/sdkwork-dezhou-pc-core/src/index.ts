export type SdkworkDezhouPcSdkSurface = 'app';

export type SdkworkDezhouPcCredentialScope = 'global-session';

export type SdkworkDezhouPcRouteSurface = 'app';

export interface SdkworkDezhouPcRouteContribution {
  readonly auth: 'public' | 'required';
  readonly capability: string;
  readonly domain: 'game';
  readonly id: string;
  readonly packageName: string;
  readonly path: string;
  readonly screen: string;
  readonly surface: SdkworkDezhouPcRouteSurface;
  readonly title: string;
  readonly titleKey: string;
}

export interface SdkworkDezhouPcSdkFamilyInventoryItem {
  readonly authority: string;
  readonly family: string;
  readonly generationInputSpec: string;
  readonly generatedPackageName?: string;
  readonly surface: SdkworkDezhouPcSdkSurface;
  readonly tokenManagerScope: SdkworkDezhouPcCredentialScope;
}

export const SdkworkDezhouPcRuntimeIdentity = {
  appKey: 'sdkwork-dezhou-pc',
  architecture: 'pc-react',
  domain: 'game',
  runtimeFamily: 'web',
} as const;

export const SdkworkDezhouPcAppSdkFamilies = [
  {
    authority: 'sdkwork-dezhou-app-api',
    family: 'sdkwork-dezhou-app-sdk',
    generationInputSpec: 'apis/app-api/game/dezhou-app-api.openapi.json',
    generatedPackageName: '@sdkwork-internal/dezhou-app-sdk-generated',
    surface: 'app',
    tokenManagerScope: 'global-session',
  },
  {
    authority: 'sdkwork-iam-app-api',
    family: 'sdkwork-iam-app-sdk',
    generationInputSpec:
      '../sdkwork-iam/sdks/sdkwork-iam-app-sdk/openapi/sdkwork-iam-app-api.openapi.yaml',
    generatedPackageName: '@sdkwork/iam-app-sdk',
    surface: 'app',
    tokenManagerScope: 'global-session',
  },
] as const satisfies readonly SdkworkDezhouPcSdkFamilyInventoryItem[];

export function listSdkworkDezhouPcAppSdkFamilies(): readonly SdkworkDezhouPcSdkFamilyInventoryItem[] {
  return SdkworkDezhouPcAppSdkFamilies;
}

export function createSdkworkDezhouPcRouteRegistry(
  ...routeGroups: readonly (readonly SdkworkDezhouPcRouteContribution[])[]
): readonly SdkworkDezhouPcRouteContribution[] {
  return routeGroups.flat();
}

export * from './store/configStore';
export * from './store/useUserStore';
export * from './tableService';
