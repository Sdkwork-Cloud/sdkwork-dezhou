import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import { defineConfig, loadEnv } from 'vite';

const dezhouAppSdkRoot = path.resolve(
  __dirname,
  '../../sdks/sdkwork-dezhou-app-sdk/sdkwork-dezhou-app-sdk-typescript/generated/server-openapi',
);

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, '.', '');
  return {
    plugins: [react(), tailwindcss()],
    define: {
      'process.env.SDKWORK_ACCESS_TOKEN': JSON.stringify(env.SDKWORK_ACCESS_TOKEN ?? ''),
    },
    resolve: {
      alias: {
        '@': path.resolve(__dirname, '.'),
        '@sdkwork-internal/dezhou-app-sdk-generated': path.resolve(dezhouAppSdkRoot, 'src/index.ts'),
      },
    },
    server: {
      port: 3001,
      host: '0.0.0.0',
      proxy: {
        '/app/v3/api': {
          target: env.VITE_DEZHOU_API_BASE_URL ?? 'http://127.0.0.1:8096',
          changeOrigin: true,
        },
      },
    },
  };
});
