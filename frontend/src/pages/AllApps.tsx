import { useMemo, useState } from 'react'

import { useUi } from '../theme/UiContext'
import { useApps, useCategories } from '../hooks'
import { AppGrid } from '../components/AppGrid'

export function AllApps() {
  const { mode } = useUi()
  const { apps, loading } = useApps(mode)
  const categories = useCategories(mode)
  const [categoryId, setCategoryId] = useState<number | 0>(0)
  const [q, setQ] = useState('')

  const filtered = useMemo(() => {
    const needle = q.toLowerCase()
    return apps.filter(
      (a) =>
        (categoryId === 0 || a.category_id === categoryId) &&
        (needle === '' || a.name.toLowerCase().includes(needle)),
    )
  }, [apps, categoryId, q])

  return (
    <div className="page-enter">
      <div className="row" style={{ justifyContent: 'space-between' }}>
        <h2>Toutes les apps — {mode}</h2>
        <input
          placeholder="Filtrer…"
          value={q}
          onChange={(e) => setQ(e.target.value)}
          style={{ maxWidth: 240 }}
        />
      </div>

      <div className="chips">
        <button
          className={`chip${categoryId === 0 ? ' active' : ''}`}
          onClick={() => setCategoryId(0)}
        >
          Tout
        </button>
        {categories.map((c) => (
          <button
            key={c.id}
            className={`chip${categoryId === c.id ? ' active' : ''}`}
            onClick={() => setCategoryId(c.id)}
          >
            {c.name}
          </button>
        ))}
      </div>

      <AppGrid apps={filtered} empty={loading ? 'Chargement…' : 'Aucune application.'} />
    </div>
  )
}
