// 10 themes shared with the native app (CDC §7). Each maps to CSS variables.
export interface Theme {
  bg: string
  surface: string
  sidebar: string
  text: string
  textDim: string
  accent: string
  border: string
}

export const THEME_NAMES = [
  'amoled',
  'dark',
  'dark-blue',
  'dark-purple',
  'dark-green',
  'light',
  'light-warm',
  'light-blue',
  'light-purple',
  'light-green',
] as const

export type ThemeName = (typeof THEME_NAMES)[number]

export const THEMES: Record<ThemeName, Theme> = {
  amoled: { bg: '#000000', surface: '#0b0b0b', sidebar: '#050505', text: '#f2f2f2', textDim: '#8a8a8a', accent: '#4c8dff', border: '#1c1c1c' },
  dark: { bg: '#121212', surface: '#1e1e1e', sidebar: '#181818', text: '#eaeaea', textDim: '#9a9a9a', accent: '#4c8dff', border: '#2c2c2c' },
  'dark-blue': { bg: '#0d1b2a', surface: '#1b263b', sidebar: '#12203a', text: '#e0e6f0', textDim: '#8fa0b8', accent: '#4da3ff', border: '#25344a' },
  'dark-purple': { bg: '#160f1f', surface: '#241832', sidebar: '#1c1229', text: '#ece4f5', textDim: '#a493b8', accent: '#a86bff', border: '#33244a' },
  'dark-green': { bg: '#0d1f16', surface: '#152e22', sidebar: '#10251b', text: '#e2f0e8', textDim: '#8fb3a1', accent: '#3ddc84', border: '#1f3d2e' },
  light: { bg: '#f7f7f7', surface: '#ffffff', sidebar: '#efefef', text: '#1a1a1a', textDim: '#666666', accent: '#2f6fed', border: '#dcdcdc' },
  'light-warm': { bg: '#faf6f0', surface: '#fffdfa', sidebar: '#f2e9dd', text: '#2a2018', textDim: '#7a6a58', accent: '#d2691e', border: '#e6dccd' },
  'light-blue': { bg: '#eef4fb', surface: '#ffffff', sidebar: '#e0ecf8', text: '#152436', textDim: '#5f7488', accent: '#2f6fed', border: '#cfe0f2' },
  'light-purple': { bg: '#f4eefb', surface: '#ffffff', sidebar: '#e9ddf6', text: '#241832', textDim: '#6f5f84', accent: '#7a3ff2', border: '#e0d0f2' },
  'light-green': { bg: '#eef8f1', surface: '#ffffff', sidebar: '#dff0e5', text: '#14261b', textDim: '#5f7a68', accent: '#1faa55', border: '#cfe8d7' },
}

export function isThemeName(v: string): v is ThemeName {
  return (THEME_NAMES as readonly string[]).includes(v)
}

export function applyTheme(name: ThemeName): void {
  const t = THEMES[name]
  const root = document.documentElement
  root.style.setProperty('--bg', t.bg)
  root.style.setProperty('--surface', t.surface)
  root.style.setProperty('--sidebar', t.sidebar)
  root.style.setProperty('--text', t.text)
  root.style.setProperty('--text-dim', t.textDim)
  root.style.setProperty('--accent', t.accent)
  root.style.setProperty('--border', t.border)
}
