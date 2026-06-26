import { BrowserRouter, Route, Routes } from 'react-router-dom';
import { SdkworkSessionAuthBrowserRoot } from '@sdkwork/auth-pc-react';

import { AppRoutes } from './AppRoutes';
import { AuthGate } from './AuthGate';
import { createSdkworkDezhouPcRuntime } from './bootstrap/runtime';

const runtime = createSdkworkDezhouPcRuntime();

export function App() {
  return (
    <BrowserRouter>
      <SdkworkSessionAuthBrowserRoot>
      <Routes>
        <Route
          element={
            <AuthGate runtime={runtime}>
              <AppRoutes runtime={runtime} />
            </AuthGate>
          }
          path="/*"
        />
      </Routes>
          </SdkworkSessionAuthBrowserRoot>
    </BrowserRouter>
  );
}

export default App;
