import { access, mkdir } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { execFile } from 'node:child_process'
import { promisify } from 'node:util'
import {
  app,
  BrowserWindow,
  globalShortcut,
  ipcMain,
  Menu,
  nativeImage,
  shell,
  Tray
} from 'electron'
import electronSquirrelStartup from 'electron-squirrel-startup'
import { electronApp, optimizer } from '@electron-toolkit/utils'

const execFileAsync = promisify(execFile)

if (electronSquirrelStartup) {
  app.quit()
}

const GLOBAL_SHORTCUT = 'CommandOrControl+Shift+Y'
const MAIN_WINDOW_WIDTH = 560
const MAIN_WINDOW_HEIGHT = 420
const MAIN_WINDOW_MIN_WIDTH = 420
const MAIN_WINDOW_MIN_HEIGHT = 320

let mainWindow: BrowserWindow | null = null
let tray: Tray | null = null
let isQuitting = false

function getExecutableName(baseName: string): string {
  return process.platform === 'win32' ? `${baseName}.exe` : baseName
}

function getIconPath(): string {
  return join(app.getAppPath(), 'resources', 'icon.png')
}

async function pathExists(filePath: string): Promise<boolean> {
  try {
    await access(filePath)
    return true
  } catch {
    return false
  }
}

function createMainWindow(): BrowserWindow {
  if (mainWindow) {
    return mainWindow
  }

  mainWindow = new BrowserWindow({
    title: 'Yapper',
    width: MAIN_WINDOW_WIDTH,
    height: MAIN_WINDOW_HEIGHT,
    minWidth: MAIN_WINDOW_MIN_WIDTH,
    minHeight: MAIN_WINDOW_MIN_HEIGHT,
    show: false,
    alwaysOnTop: true,
    autoHideMenuBar: true,
    backgroundColor: '#e8f0f7',
    ...(process.platform === 'linux' ? { icon: getIconPath() } : {}),
    ...(process.platform === 'darwin' ? { titleBarStyle: 'hiddenInset' as const } : {}),
    webPreferences: {
      preload: join(__dirname, 'preload.js'),
      sandbox: false
    }
  })

  mainWindow.on('close', (event) => {
    if (isQuitting) {
      return
    }

    event.preventDefault()
    mainWindow?.hide()
  })

  mainWindow.on('closed', () => {
    mainWindow = null
  })

  mainWindow.webContents.setWindowOpenHandler((details) => {
    void shell.openExternal(details.url)
    return { action: 'deny' }
  })

  if (MAIN_WINDOW_VITE_DEV_SERVER_URL) {
    void mainWindow.loadURL(MAIN_WINDOW_VITE_DEV_SERVER_URL)
  } else {
    void mainWindow.loadFile(join(__dirname, `../renderer/${MAIN_WINDOW_VITE_NAME}/index.html`))
  }

  return mainWindow
}

function showMainWindow(): void {
  const window = createMainWindow()

  if (window.isMinimized()) {
    window.restore()
  }

  window.show()
  window.focus()
}

function toggleMainWindow(): void {
  const window = createMainWindow()

  if (window.isVisible()) {
    window.hide()
    return
  }

  showMainWindow()
}

function createTray(): void {
  if (tray) {
    return
  }

  const trayIcon = nativeImage.createFromPath(getIconPath())
  const menu = Menu.buildFromTemplate([
    {
      label: 'Toggle UI',
      click: () => toggleMainWindow()
    },
    {
      label: 'Quit',
      click: () => {
        isQuitting = true
        app.quit()
      }
    }
  ])

  tray = new Tray(
    process.platform === 'darwin' ? trayIcon.resize({ width: 18, height: 18 }) : trayIcon
  )
  tray.setToolTip('Yapper')
  tray.setContextMenu(menu)
  tray.on('click', () => toggleMainWindow())
}

type YapInvocation = {
  command: string
  baseArgs: string[]
  cwd?: string
}

async function resolveYapInvocation(): Promise<YapInvocation> {
  const executableName = getExecutableName('yap')

  if (app.isPackaged) {
    const packagedBinary = join(process.resourcesPath, 'yap', executableName)

    if (!(await pathExists(packagedBinary))) {
      throw new Error(`Bundled yap sidecar was not found at ${packagedBinary}`)
    }

    return {
      command: packagedBinary,
      baseArgs: []
    }
  }

  const appPath = app.getAppPath()
  const devBinary = join(appPath, 'resources', 'yap', executableName)

  if (await pathExists(devBinary)) {
    return {
      command: devBinary,
      baseArgs: []
    }
  }

  const serviceProjectDir = join(appPath, '..', '..', 'services', 'yap')

  return {
    command: 'dotnet',
    baseArgs: ['run', '--project', join(serviceProjectDir, 'yap.csproj'), '--'],
    cwd: serviceProjectDir
  }
}

function parseOutputPath(stdout: string, fallbackPath: string): string {
  const outputPath = stdout
    .split(/\r?\n/)
    .reverse()
    .find((line) => line.includes(' to '))
    ?.split(' to ')
    .at(-1)
    ?.trim()

  return outputPath || fallbackPath
}

async function synthesizeText(rawText: string): Promise<string> {
  const text = rawText.trim()

  if (!text) {
    throw new Error('Enter text before generating audio.')
  }

  const outputDir = join(tmpdir(), 'yapper-audio')
  await mkdir(outputDir, { recursive: true })

  const yap = await resolveYapInvocation()

  try {
    const { stdout, stderr } = await execFileAsync(
      yap.command,
      [...yap.baseArgs, 'synthesize', text, '--output', outputDir],
      {
        cwd: yap.cwd,
        windowsHide: true,
        maxBuffer: 10 * 1024 * 1024
      }
    )

    if (stderr.trim()) {
      console.warn(stderr.trim())
    }

    return parseOutputPath(stdout, outputDir)
  } catch (error) {
    const stdout = typeof error === 'object' && error && 'stdout' in error ? String(error.stdout ?? '') : ''
    const stderr = typeof error === 'object' && error && 'stderr' in error ? String(error.stderr ?? '') : ''
    const message = error instanceof Error ? error.message : 'Unknown sidecar execution error.'

    throw new Error(
      stdout || stderr
        ? `Audio generation failed.\nstdout:\n${stdout.trim()}\nstderr:\n${stderr.trim()}`
        : `Failed to execute yap sidecar: ${message}`
    )
  }
}

app.whenReady().then(() => {
  electronApp.setAppUserModelId('io.hayman.Yapper')

  app.on('browser-window-created', (_, window) => {
    optimizer.watchWindowShortcuts(window)
  })

  ipcMain.handle('yapper:hide-launcher', () => {
    mainWindow?.hide()
  })

  ipcMain.handle('yapper:reveal-output', (_event, outputPath: string) => {
    if (!outputPath) {
      return
    }

    shell.showItemInFolder(outputPath)
  })

  ipcMain.handle('yapper:synthesize-text', async (_event, text: string) => {
    return synthesizeText(text)
  })

  createMainWindow()
  createTray()

  const shortcutRegistered = globalShortcut.register(GLOBAL_SHORTCUT, () => {
    toggleMainWindow()
  })

  if (!shortcutRegistered) {
    console.error(`Failed to register global shortcut: ${GLOBAL_SHORTCUT}`)
  }

  app.on('activate', function () {
    if (BrowserWindow.getAllWindows().length === 0) {
      createMainWindow()
    }
  })
})

app.on('before-quit', () => {
  isQuitting = true
})

app.on('will-quit', () => {
  globalShortcut.unregisterAll()
})

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})
