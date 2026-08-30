import { useUi } from '../theme/UiContext'
import { useApps, useFavorites } from '../hooks'
import { AppGrid } from '../components/AppGrid'

export function Favorites() {
  const { mode } = useUi()
  const { apps, loading } = useApps(mode)
  const { ids, enabled } = useFavorites()

  if (!enabled) {
    return (
      <div className="page-enter">
        <h2>Favoris</h2>
        <p className="muted">Connecte-toi pour gérer tes favoris.</p>
      </div>
    )
  }

  const favApps = apps.filter((a) => ids.includes(a.id))
  return (
    <div className="page-enter">
      <h2>Favoris</h2>
      <AppGrid
        apps={favApps}
        empty={loading ? 'Chargement…' : "Aucun favori — touche l'étoile sur une app."}
      />
    </div>
  )
}
