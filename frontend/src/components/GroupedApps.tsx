import type { App, Category } from '../api/types'
import { AppGrid } from './AppGrid'

// Desktop mode: apps stacked in one section per category (CDC §3.2).
export function GroupedApps({
  apps,
  categories,
  empty,
}: {
  apps: App[]
  categories: Category[]
  empty?: string
}) {
  if (apps.length === 0) return <p className="muted">{empty ?? 'Aucune application.'}</p>

  const byId = new Map(categories.map((c) => [c.id, c.name]))
  const groups = new Map<string, App[]>()
  for (const app of apps) {
    const key = app.category_id != null ? byId.get(app.category_id) ?? 'Autres' : 'Autres'
    if (!groups.has(key)) groups.set(key, [])
    groups.get(key)!.push(app)
  }

  return (
    <>
      {[...groups.entries()]
        .sort(([a], [b]) => a.localeCompare(b))
        .map(([name, list]) => (
          <section key={name} className="cat-group">
            <h3>{name}</h3>
            <AppGrid apps={list} />
          </section>
        ))}
    </>
  )
}
