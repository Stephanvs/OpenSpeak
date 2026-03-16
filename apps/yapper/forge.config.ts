import type { ForgeConfig } from '@electron-forge/shared-types'

const isDraftRelease = process.env.YAPPER_RELEASE_DRAFT !== 'false'
const isPrereleaseRelease = process.env.YAPPER_RELEASE_PRERELEASE === 'true'

const config: ForgeConfig = {
  packagerConfig: {
    appBundleId: 'io.hayman.Yapper',
    appCategoryType: 'public.app-category.utilities',
    asar: true,
    executableName: 'yapper',
    extraResource: ['./resources/yap'],
    icon: './build/icon',
    name: 'Yapper'
  },
  makers: [
    {
      name: '@electron-forge/maker-squirrel',
      platforms: ['win32'],
      config: {
        name: 'yapper',
        setupIcon: './build/icon.ico'
      }
    },
    {
      name: '@electron-forge/maker-zip',
      platforms: ['darwin'],
      config: {}
    },
    {
      name: '@electron-forge/maker-dmg',
      platforms: ['darwin'],
      config: {
        icon: './build/icon.icns',
        name: 'Yapper',
        overwrite: true
      }
    },
    {
      name: '@electron-forge/maker-deb',
      platforms: ['linux'],
      config: {
        options: {
          categories: ['Utility'],
          icon: './build/icon.png',
          maintainer: 'Hayman'
        }
      }
    },
    {
      name: '@electron-forge/maker-rpm',
      platforms: ['linux'],
      config: {}
    }
  ],
  publishers: [
    {
      name: '@electron-forge/publisher-github',
      config: {
        draft: isDraftRelease,
        generateReleaseNotes: true,
        prerelease: isPrereleaseRelease,
        repository: {
          owner: 'stephanvs',
          name: 'yapper'
        },
        tagPrefix: 'v'
      }
    }
  ],
  plugins: [
    {
      name: '@electron-forge/plugin-vite',
      config: {
        build: [
          {
            entry: 'src/main/index.ts',
            config: 'vite.main.config.ts',
            target: 'main'
          },
          {
            entry: 'src/preload/index.ts',
            config: 'vite.preload.config.ts',
            target: 'preload'
          }
        ],
        renderer: [
          {
            config: 'vite.renderer.config.ts',
            name: 'main_window'
          }
        ]
      }
    }
  ]
}

export default config
