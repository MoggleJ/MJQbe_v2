import { useUi } from '../theme/UiContext'
import { useAuth } from '../auth/AuthContext'
import { THEME_NAMES } from '../theme/themes'

export function SettingsPage() {
  const { theme, setTheme, layout, setLayout, iconSize, setIconSize } = useUi()
  const { me } = useAuth()

  return (
    <div className="page-enter">
      <h2>Paramètres</h2>

      <div className="section">
        <h3>Thème</h3>
        <div className="chips">
          {THEME_NAMES.map((t) => (
            <button
              key={t}
              className={`chip${theme === t ? ' active' : ''}`}
              onClick={() => setTheme(t)}
            >
              {t}
            </button>
          ))}
        </div>
      </div>

      <div className="section">
        <h3>Disposition</h3>
        <div className="chips">
          {(['grid', 'list'] as const).map((l) => (
            <button
              key={l}
              className={`chip${layout === l ? ' active' : ''}`}
              onClick={() => setLayout(l)}
            >
              {l}
            </button>
          ))}
        </div>
        <h3>Taille des icônes</h3>
        <div className="chips">
          {(['small', 'medium', 'large'] as const).map((s) => (
            <button
              key={s}
              className={`chip${iconSize === s ? ' active' : ''}`}
              onClick={() => setIconSize(s)}
            >
              {s}
            </button>
          ))}
        </div>
      </div>

      {!me && <p className="muted">Connecte-toi pour synchroniser tes préférences.</p>}
    </div>
  )
}
