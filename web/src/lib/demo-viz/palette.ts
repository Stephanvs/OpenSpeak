/** Resolve site CSS variables into Three.js-friendly hex colors. */

function cssColor(name: string, fallback: string): string {
  if (typeof document === 'undefined') return fallback
  const value = getComputedStyle(document.documentElement).getPropertyValue(name).trim()
  return value || fallback
}

function parseToHex(input: string, fallback: number): number {
  const c = input.trim()
  if (c.startsWith('#')) {
    const hex = c.slice(1)
    if (hex.length === 3) {
      return parseInt(hex.split('').map((ch) => ch + ch).join(''), 16)
    }
    if (hex.length === 6) return parseInt(hex, 16)
  }

  // rgb / rgba
  const rgb = c.match(/rgba?\(\s*([\d.]+)\s*,\s*([\d.]+)\s*,\s*([\d.]+)/i)
  if (rgb) {
    const r = Math.round(Number(rgb[1]))
    const g = Math.round(Number(rgb[2]))
    const b = Math.round(Number(rgb[3]))
    return (r << 16) | (g << 8) | b
  }

  // oklch — approximate via canvas
  if (c.startsWith('oklch') || c.startsWith('hsl') || c.startsWith('lab')) {
    try {
      const canvas = document.createElement('canvas')
      canvas.width = canvas.height = 1
      const ctx = canvas.getContext('2d')
      if (ctx) {
        ctx.fillStyle = '#000'
        ctx.fillStyle = c
        const normalized = String(ctx.fillStyle)
        if (normalized.startsWith('#')) {
          return parseInt(normalized.slice(1), 16)
        }
        const m = normalized.match(/rgba?\(\s*([\d.]+)\s*,\s*([\d.]+)\s*,\s*([\d.]+)/i)
        if (m) {
          return (Math.round(+m[1]) << 16) | (Math.round(+m[2]) << 8) | Math.round(+m[3])
        }
      }
    } catch {
      /* fall through */
    }
  }

  return fallback
}

export type DemoPalette = {
  ink: number
  inkSoft: number
  inkDim: number
  orange: number
  orangeDark: number
  surface: number
  line: number
  live: number
  isDark: boolean
}

export function readPalette(): DemoPalette {
  const scheme =
    typeof window !== 'undefined' && window.matchMedia('(prefers-color-scheme: dark)').matches

  return {
    ink: parseToHex(cssColor('--ink-strong', scheme ? '#f0f0f5' : '#202020'), scheme ? 0xf0f0f5 : 0x202020),
    inkSoft: parseToHex(cssColor('--ink-soft', scheme ? '#9a9aad' : '#888888'), scheme ? 0x9a9aad : 0x888888),
    inkDim: parseToHex(cssColor('--ink-dim', scheme ? '#7a7a90' : '#999999'), scheme ? 0x7a7a90 : 0x999999),
    orange: parseToHex(cssColor('--orange', '#f6821f'), 0xf6821f),
    orangeDark: parseToHex(cssColor('--orange-dark', scheme ? '#ff8a5c' : '#da5b16'), scheme ? 0xff8a5c : 0xda5b16),
    surface: parseToHex(cssColor('--surface-recessed', scheme ? '#121218' : '#f2f2f2'), scheme ? 0x121218 : 0xf2f2f2),
    line: parseToHex(cssColor('--line', scheme ? '#3a3a48' : '#dedede'), scheme ? 0x3a3a48 : 0xdedede),
    live: parseToHex(cssColor('--status-live', scheme ? '#5fd98a' : '#35a95a'), scheme ? 0x5fd98a : 0x35a95a),
    isDark: scheme,
  }
}
