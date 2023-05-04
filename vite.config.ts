import {defineConfig} from 'vite';
import react from '@vitejs/plugin-react';
import viteTsconfigPaths from 'vite-tsconfig-paths';

export default defineConfig({
  root: 'frontend',
  plugins: [react(), viteTsconfigPaths()],
  build: {
    target: 'esnext',
  },
});
