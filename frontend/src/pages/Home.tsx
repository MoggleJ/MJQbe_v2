import { useUi } from '../theme/UiContext'
import { useApps, useCategories, useFavorites } from '../hooks'
import { AppGrid } from '../components/AppGrid'
import { GroupedApps } from '../components/GroupedApps'

export function Home() {
  const { mode } = useUi()
  const { apps, loading } = useApps(mode)
  const categories = useCategories(mode)
  const { ids, enabled } = useFavorites()

  const favApps = apps.filter((a) => ids.includes(a.id))

  return (
    <div className="page-enter">
      <h2>Accueil</h2>
      {loading && <p className="muted">Chargement…</p>}

      {enabled && favApps.length > 0 && (
        <section className="cat-group">
          <h3>Favoris</h3>
          <AppGrid apps={favApps} />
        </section>
      )}

      <h3>Toutes les applications</h3>
      {mode === 'desktop' ? (
        <GroupedApps
          apps={apps}
          categories={categories}
          empty={loading ? 'Chargement…' : 'Aucune application.'}
        />
      ) : (
        <AppGrid apps={apps} empty={loading ? 'Chargement…' : 'Aucune application.'} />
      )}
    </div>
  )
}
