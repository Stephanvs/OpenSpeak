import { ElectronAPI } from '@electron-toolkit/preload'

export interface YapperApi {
  hideLauncher: () => Promise<void>
  revealOutput: (outputPath: string) => Promise<void>
  synthesizeText: (text: string) => Promise<string>
}

declare global {
  interface Window {
    electron: ElectronAPI
    api: YapperApi
  }
}
