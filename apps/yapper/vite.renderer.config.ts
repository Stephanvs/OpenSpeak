import path from 'node:path'

import tailwindcss from '@tailwindcss/vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { defineConfig } from 'vite'

export default defineConfig({
  base: './',
  build: {
    outDir: '../../.vite/renderer/main_window'
  },
  plugins: [tailwindcss(), svelte()],
  resolve: {
    alias: {
      $lib: path.resolve(__dirname, 'src/renderer/src/lib')
    }
  },
  root: 'src/renderer'
})
