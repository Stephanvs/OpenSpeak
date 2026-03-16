import { contextBridge, ipcRenderer } from 'electron'
import { electronAPI } from '@electron-toolkit/preload'

const api = {
  hideLauncher: (): Promise<void> => ipcRenderer.invoke('yapper:hide-launcher'),
  revealOutput: (outputPath: string): Promise<void> =>
    ipcRenderer.invoke('yapper:reveal-output', outputPath),
  synthesizeText: (text: string): Promise<string> => ipcRenderer.invoke('yapper:synthesize-text', text)
}

if (process.contextIsolated) {
  try {
    contextBridge.exposeInMainWorld('electron', electronAPI)
    contextBridge.exposeInMainWorld('api', api)
  } catch (error) {
    console.error(error)
  }
} else {
  // @ts-ignore (define in dts)
  window.electron = electronAPI
  // @ts-ignore (define in dts)
  window.api = api
}
