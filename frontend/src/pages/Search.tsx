import { useMemo, useState } from 'react'

import { useUi } from '../theme/UiContext'
import { useApps } from '../hooks'
import { AppGrid } from '../components/AppGrid'

export function Search() {
  const { mode } = useUi()
  const { apps } = useApps(mode)
  const [q, setQ] = useState('')

  const results = useMemo(() => {
    const needle = q.trim().toLowerCase()
    return needle === '' ? [] : apps.filter((a) => a.name.toLowerCase().includes(needle))
  }, [apps, q])

  return (
    <div className="page-enter">
      <h2>Recherche</h2>
      <input
        autoFocus
        placeholder="Rechercher une application…"
        value={q}
        onChange={(e) => setQ(e.target.value)}
        style={{ width: '100%', marginBottom: 18 }}
      />
      <AppGrid
        apps={results}
        empty={q === '' ? 'Tapez pour rechercher.' : 'Aucun résultat.'}
      />
    </div>
  )
}
