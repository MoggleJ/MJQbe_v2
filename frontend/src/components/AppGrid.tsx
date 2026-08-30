import { catalog } from '../api/endpoints'
import { useUi } from '../theme/UiContext'
import { openApp, useFavorites } from '../hooks'
import type { App } from '../api/types'
import { AppCard } from './AppCard'

export function AppGrid({ apps, empty }: { apps: App[]; empty?: string }) {
  const { mode, layout, iconSize } = useUi()
  const { ids, toggle, enabled } = useFavorites()

  if (apps.length === 0) {
    return <p className="muted">{empty ?? 'Aucune application.'}</p>
  }

  return (
    <div className={`grid ${mode} ${layout} icon-${iconSize}`}>
      {apps.map((app) => (
        <AppCard
          key={app.id}
          app={app}
          favorite={ids.includes(app.id)}
          onOpen={(a) => {
            // record the launch (fire and forget), then open
            catalog.app(a.id).catch(() => {})
            openApp(a)
          }}
          onToggleFavorite={enabled ? (a) => void toggle(a.id) : undefined}
        />
      ))}
    </div>
  )
}
