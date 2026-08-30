import { useUi } from '../theme/UiContext'
import { useApps, useFavorites } from '../hooks'
import { AppGrid } from '../components/AppGrid'

export function Home() {
  const { mode } = useUi()
  const { apps, loading } = useApps(mode)
  const { ids, enabled } = useFavorites()

  const favApps = apps.filter((a) => ids.includes(a.id))

  return (
    <div className="page-enter">
      <h2>Accueil</h2>
      {loading && <p className="muted">Chargement…</p>}

      {enabled && favApps.length > 0 && (
        <>
          <h3>Favoris</h3>
          <AppGrid apps={favApps} />
        </>
      )}

      <h3>Toutes les applications</h3>
      <AppGrid apps={apps} empty={loading ? 'Chargement…' : 'Aucune application.'} />
    </div>
  )
}
