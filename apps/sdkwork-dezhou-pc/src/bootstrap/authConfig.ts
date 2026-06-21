import type { SdkworkAuthRuntimeConfig } from '@sdkwork/auth-pc-react';

export interface SdkworkDezhouPcAuthAppearanceConfig {
  asidePanelClassName?: string;
  bodyClassName?: string;
  contentContainerClassName?: string;
  pageClassName?: string;
  qrFrameClassName?: string;
  shellClassName?: string;
  slotProps?: {
    background?: { className?: string };
    page?: { className?: string };
    shell?: { className?: string };
  };
  theme?: Record<string, string>;
}

export type SdkworkDezhouPcAuthRuntimeConfig = SdkworkAuthRuntimeConfig;

const DEZHOU_VERIFICATION_POLICY = {
  emailCodeLoginEnabled: true,
  emailRegistrationVerificationRequired: false,
  phoneCodeLoginEnabled: true,
  phoneRegistrationVerificationRequired: false,
};

export function resolveSdkworkDezhouPcAuthRuntimeConfig(): SdkworkDezhouPcAuthRuntimeConfig {
  return {
    leftRailMode: 'qr-only',
    loginMethods: ['password', 'emailCode', 'phoneCode'],
    oauthLoginEnabled: false,
    oauthProviders: [],
    qrLoginEnabled: true,
    recoveryMethods: ['email', 'phone'],
    registerMethods: ['email', 'phone'],
    verificationPolicy: DEZHOU_VERIFICATION_POLICY,
  };
}

export function resolveSdkworkDezhouPcAuthAppearance(): SdkworkDezhouPcAuthAppearanceConfig {
  return {
    asidePanelClassName: 'sdkwork-dezhou-pc-auth-aside-panel',
    bodyClassName: 'sdkwork-dezhou-pc-auth-body',
    contentContainerClassName: 'sdkwork-dezhou-pc-auth-content',
    pageClassName: 'sdkwork-dezhou-pc-auth-page',
    qrFrameClassName: 'sdkwork-dezhou-pc-auth-qr-frame',
    shellClassName: 'sdkwork-dezhou-pc-auth-card-shell',
    slotProps: {
      background: {
        className: 'sdkwork-dezhou-pc-auth-background',
      },
      page: {
        className: 'sdkwork-dezhou-pc-auth-page',
      },
      shell: {
        className: 'sdkwork-dezhou-pc-auth-card-shell',
      },
    },
  };
}

export function resolveSdkworkDezhouPcAuthLocale(defaultLocale: string): string {
  return defaultLocale;
}
