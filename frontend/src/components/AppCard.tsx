import type { App } from '../api/types'

interface Props {
  app: App
  favorite?: boolean
  onOpen: (app: App) => void
  onToggleFavorite?: (app: App) => void
}

export function AppCard({ app, favorite, onOpen, onToggleFavorite }: Props) {
  return (
    <button className="card" onClick={() => onOpen(app)} title={app.name}>
      <span className="icon">
        {app.name.charAt(0).toUpperCase()}
        {onToggleFavorite && (
          <span
            className={`star${favorite ? ' on' : ''}`}
            role="button"
            aria-label={favorite ? 'Retirer des favoris' : 'Ajouter aux favoris'}
            onClick={(e) => {
              e.stopPropagation()
              onToggleFavorite(app)
            }}
          >
            ★
          </span>
        )}
      </span>
      <span className="name">{app.name}</span>
    </button>
  )
}
