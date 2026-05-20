import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// En Docker dev : l'API est sur le service "api" (réseau Docker interne)
// En local hors Docker : fallback sur localhost:4848
const API_TARGET = process.env.VITE_API_TARGET || 'http://api:4848'

export default defineConfig({
  plugins: [react()],
  server: {
    host: '0.0.0.0',
    port: 5173,
    proxy: {
      '/api': {
        target: API_TARGET,
        rewrite: (path) => path.replace(/^\/api/, ''),
        changeOrigin: true,
      },
    },
  },
})
