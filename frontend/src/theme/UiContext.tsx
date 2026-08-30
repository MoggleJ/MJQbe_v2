import { createContext, useCallback, useContext, useEffect, useMemo, useState } from 'react'
import type { ReactNode } from 'react'

import { user as userApi } from '../api/endpoints'
import { getAccess } from '../api/client'
import { applyTheme, isThemeName, type ThemeName } from './themes'
import type { Mode } from '../api/types'

interface UiValue {
  theme: ThemeName
  mode: Mode
  layout: 'grid' | 'list'
  iconSize: 'small' | 'medium' | 'large'
  setTheme: (t: ThemeName) => void
  setMode: (m: Mode) => void
  toggleMode: () => void
  setLayout: (l: 'grid' | 'list') => void
  setIconSize: (s: 'small' | 'medium' | 'large') => void
}

const Ctx = createContext<UiValue | null>(null)
const LS = 'mjqbe.ui'

function readLocal() {
  try {
    return JSON.parse(localStorage.getItem(LS) || '{}')
  } catch {
    return {}
  }
}

export function UiProvider({ children }: { children: ReactNode }) {
  const initial = readLocal()
  const [theme, setThemeState] = useState<ThemeName>(
    isThemeName(initial.theme) ? initial.theme : 'amoled',
  )
  const [mode, setMode] = useState<Mode>(initial.mode === 'desktop' ? 'desktop' : 'tv')
  const [layout, setLayout] = useState<'grid' | 'list'>(
    initial.layout === 'list' ? 'list' : 'grid',
  )
  const [iconSize, setIconSize] = useState<'small' | 'medium' | 'large'>(
    ['small', 'large'].includes(initial.iconSize) ? initial.iconSize : 'medium',
  )

  useEffect(() => {
    applyTheme(theme)
  }, [theme])

  useEffect(() => {
    localStorage.setItem(LS, JSON.stringify({ theme, mode, layout, iconSize }))
  }, [theme, mode, layout, iconSize])

  // Pull server-side settings once logged in.
  useEffect(() => {
    if (!getAccess()) return
    userApi
      .settings()
      .then((s) => {
        if (isThemeName(s.theme)) setThemeState(s.theme)
        setLayout(s.layout)
        setIconSize(s.icon_size)
        setMode(s.default_mode)
      })
      .catch(() => {})
  }, [])

  const persist = useCallback((patch: Record<string, unknown>) => {
    if (getAccess()) userApi.updateSettings(patch).catch(() => {})
  }, [])

  const setTheme = useCallback(
    (t: ThemeName) => {
      setThemeState(t)
      persist({ theme: t })
    },
    [persist],
  )

  const value = useMemo<UiValue>(
    () => ({
      theme,
      mode,
      layout,
      iconSize,
      setTheme,
      setMode: (m) => {
        setMode(m)
        persist({ default_mode: m })
      },
      toggleMode: () =>
        setMode((prev) => {
          const next = prev === 'tv' ? 'desktop' : 'tv'
          persist({ default_mode: next })
          return next
        }),
      setLayout: (l) => {
        setLayout(l)
        persist({ layout: l })
      },
      setIconSize: (s) => {
        setIconSize(s)
        persist({ icon_size: s })
      },
    }),
    [theme, mode, layout, iconSize, setTheme, persist],
  )

  return <Ctx.Provider value={value}>{children}</Ctx.Provider>
}

export function useUi(): UiValue {
  const v = useContext(Ctx)
  if (!v) throw new Error('useUi outside UiProvider')
  return v
}
