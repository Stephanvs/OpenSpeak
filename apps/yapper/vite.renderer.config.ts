import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'

export default defineConfig({
  base: './',
  build: {
    outDir: '../../.vite/renderer/main_window'
  },
  plugins: [svelte()],
  root: 'src/renderer'
})
