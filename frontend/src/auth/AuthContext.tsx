import { createContext, useCallback, useContext, useEffect, useMemo, useState } from 'react'
import type { ReactNode } from 'react'

import { auth as authApi } from '../api/endpoints'
import { clearTokens, getAccess } from '../api/client'
import type { Me } from '../api/types'

interface AuthValue {
  me: Me | null
  loading: boolean
  isAdmin: boolean
  login: (username: string, password: string) => Promise<void>
  logout: () => void
  refreshMe: () => Promise<void>
}

const Ctx = createContext<AuthValue | null>(null)

export function AuthProvider({ children }: { children: ReactNode }) {
  const [me, setMe] = useState<Me | null>(null)
  const [loading, setLoading] = useState(true)

  const refreshMe = useCallback(async () => {
    if (!getAccess()) {
      setMe(null)
      setLoading(false)
      return
    }
    try {
      setMe(await authApi.me())
    } catch {
      setMe(null)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    void refreshMe()
  }, [refreshMe])

  const login = useCallback(
    async (username: string, password: string) => {
      await authApi.login(username, password)
      setMe(await authApi.me())
    },
    [],
  )

  const logout = useCallback(() => {
    clearTokens()
    setMe(null)
  }, [])

  const value = useMemo<AuthValue>(
    () => ({ me, loading, isAdmin: me?.role === 'admin', login, logout, refreshMe }),
    [me, loading, login, logout, refreshMe],
  )

  return <Ctx.Provider value={value}>{children}</Ctx.Provider>
}

export function useAuth(): AuthValue {
  const v = useContext(Ctx)
  if (!v) throw new Error('useAuth outside AuthProvider')
  return v
}
