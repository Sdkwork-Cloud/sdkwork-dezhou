import { Navigate, Route, Routes, useLocation, useNavigate } from 'react-router-dom';
import { DezhouAppShell } from 'sdkwork-dezhou-pc-shell';

import {
  buildSdkworkDezhouPcAuthLoginRedirect,
  hasSdkworkDezhouPcAuthenticatedSession,
} from './authGateLogic';
import type { SdkworkDezhouPcRuntime } from './bootstrap/runtime';

export function AppRoutes({ runtime }: { runtime: SdkworkDezhouPcRuntime }) {
  const navigate = useNavigate();

  const handleLogout = () => {
    runtime.session.clearSession();
    navigate(buildSdkworkDezhouPcAuthLoginRedirect({ pathname: '/', search: '', hash: '' }), {
      replace: true,
    });
  };

  return (
    <Routes>
      <Route
        element={
          <ProtectedRoute runtime={runtime}>
            <DezhouAppShell onLogout={handleLogout} />
          </ProtectedRoute>
        }
        path="/app/dezhou/*"
      />
      <Route element={<Navigate replace to="/app/dezhou" />} path="/" />
      <Route element={<Navigate replace to="/app/dezhou" />} path="*" />
    </Routes>
  );
}

function ProtectedRoute({
  children,
  runtime,
}: {
  children: React.ReactNode;
  runtime: SdkworkDezhouPcRuntime;
}) {
  const location = useLocation();
  const snapshot = runtime.session.getSnapshot();
  if (!hasSdkworkDezhouPcAuthenticatedSession(snapshot)) {
    return <Navigate replace to={buildSdkworkDezhouPcAuthLoginRedirect(location)} />;
  }
  return <>{children}</>;
}
