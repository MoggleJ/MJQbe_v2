import { useCallback, useEffect, useState } from 'react'

import { catalog, user } from './api/endpoints'
import { getAccess } from './api/client'
import type { App, Category } from './api/types'

export function useApps(mode: string) {
  const [apps, setApps] = useState<App[]>([])
  const [loading, setLoading] = useState(true)
  useEffect(() => {
    let alive = true
    setLoading(true)
    catalog
      .apps(mode)
      .then((a) => alive && setApps(a))
      .catch(() => alive && setApps([]))
      .finally(() => alive && setLoading(false))
    return () => {
      alive = false
    }
  }, [mode])
  return { apps, loading }
}

export function useCategories(mode: string) {
  const [categories, setCategories] = useState<Category[]>([])
  useEffect(() => {
    let alive = true
    catalog
      .categories(mode)
      .then((c) => alive && setCategories(c))
      .catch(() => alive && setCategories([]))
    return () => {
      alive = false
    }
  }, [mode])
  return categories
}

export function useFavorites() {
  const [ids, setIds] = useState<number[]>([])
  const loggedIn = !!getAccess()

  const reload = useCallback(() => {
    if (!loggedIn) return
    user.favorites().then((f) => setIds(f.app_ids)).catch(() => {})
  }, [loggedIn])

  useEffect(reload, [reload])

  const toggle = useCallback(
    async (appId: number) => {
      if (!loggedIn) return
      const has = ids.includes(appId)
      const res = has ? await user.removeFavorite(appId) : await user.addFavorite(appId)
      setIds(res.app_ids)
    },
    [ids, loggedIn],
  )

  return { ids, toggle, reload, enabled: loggedIn }
}

export function openApp(app: App): void {
  if (app.url) window.open(app.url, '_blank', 'noopener')
}
