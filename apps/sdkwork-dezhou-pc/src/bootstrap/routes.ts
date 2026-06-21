import { createSdkworkDezhouPcRouteRegistry } from 'sdkwork-dezhou-pc-core';
import type { SdkworkDezhouPcRouteContribution } from 'sdkwork-dezhou-pc-core';

export const SdkworkDezhouPcDashboardRoutes = [
  {
    auth: 'required',
    capability: 'table',
    domain: 'game',
    id: 'app.dezhou.table.home',
    packageName: 'sdkwork-dezhou-pc-shell',
    path: '/app/dezhou/*',
    screen: 'DezhouAppShell',
    surface: 'app',
    title: 'Dezhou',
    titleKey: 'dezhou.table.title',
  },
] as const satisfies readonly SdkworkDezhouPcRouteContribution[];

export const SdkworkDezhouPcRoutes = createSdkworkDezhouPcRouteRegistry(SdkworkDezhouPcDashboardRoutes);

export type { SdkworkDezhouPcRouteContribution };
