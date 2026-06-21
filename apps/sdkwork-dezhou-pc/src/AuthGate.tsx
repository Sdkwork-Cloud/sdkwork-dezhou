import { type ReactNode, useEffect, useMemo, useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { SdkworkIamAuthRoutes } from '@sdkwork/auth-pc-react';

import {
  resolveSdkworkDezhouPcAuthAppearance,
  resolveSdkworkDezhouPcAuthLocale,
  resolveSdkworkDezhouPcAuthRuntimeConfig,
} from './bootstrap/authConfig';
import type { SdkworkDezhouPcRuntime } from './bootstrap/runtime';
import {
  hasSdkworkDezhouPcAuthenticatedSession,
  resolveSdkworkDezhouPcAuthGateDecision,
} from './authGateLogic';

export interface AuthGateProps {
  children: ReactNode;
  runtime: SdkworkDezhouPcRuntime;
}

export function AuthGate({ children, runtime }: AuthGateProps) {
  const location = useLocation();
  const navigate = useNavigate();
  const [snapshot, setSnapshot] = useState(() => runtime.session.getSnapshot());

  useEffect(() => runtime.session.subscribe(setSnapshot), [runtime.session]);

  const decision = useMemo(
    () =>
      resolveSdkworkDezhouPcAuthGateDecision({
        hasSession: hasSdkworkDezhouPcAuthenticatedSession(snapshot),
        homePath: '/app/dezhou',
        location,
      }),
    [location, snapshot],
  );

  useEffect(() => {
    if (decision.kind !== 'redirect') {
      return;
    }
    navigate(decision.to, { replace: true });
  }, [decision, navigate]);

  if (decision.kind === 'redirect') {
    return null;
  }

  if (decision.kind === 'auth-route') {
    const authProps = {
      appearance: resolveSdkworkDezhouPcAuthAppearance(),
      basePath: '/auth',
      getRuntime: () => runtime.iamRuntime,
      homePath: '/app/dezhou',
      locale: resolveSdkworkDezhouPcAuthLocale(runtime.config.i18n.defaultLocale),
      runtimeConfig: resolveSdkworkDezhouPcAuthRuntimeConfig(),
      viewportMode: 'flow' as const,
    };

    return (
      <SdkworkIamAuthRoutes
        {...(authProps as unknown as Parameters<typeof SdkworkIamAuthRoutes>[0])}
      />
    );
  }

  return <>{children}</>;
}
