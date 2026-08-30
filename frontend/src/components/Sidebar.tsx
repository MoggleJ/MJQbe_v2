import { NavLink, useNavigate } from 'react-router-dom'

import { useAuth } from '../auth/AuthContext'
import { useUi } from '../theme/UiContext'
import { Clock } from './Clock'

const TITLES: Record<string, string> = { tv: 'MJ TV', desktop: 'MJ Desktop' }

export function Sidebar() {
  const { mode, toggleMode } = useUi()
  const { me, isAdmin, logout } = useAuth()
  const navigate = useNavigate()

  const item = (to: string, label: string) => (
    <NavLink to={to} end className={({ isActive }) => `nav-item${isActive ? ' active' : ''}`}>
      {label}
    </NavLink>
  )

  return (
    <aside className="sidebar">
      <h1>{TITLES[mode]}</h1>
      <nav>
        {item('/', 'Accueil')}
        {item('/apps', 'Toutes les apps')}
        {item('/search', 'Recherche')}
        <button className="nav-item" onClick={toggleMode}>
          ⇄ {mode === 'tv' ? 'MJ Desktop' : 'MJ TV'}
        </button>
        {isAdmin && item('/admin', 'Admin')}
      </nav>
      <div className="spacer" />
      <div className="foot">
        {item('/settings', 'Paramètres')}
        <Clock />
        {me ? (
          <button
            className="nav-item"
            onClick={() => {
              logout()
              navigate('/login')
            }}
          >
            Déconnexion ({me.username})
          </button>
        ) : (
          item('/login', 'Connexion')
        )}
      </div>
    </aside>
  )
}
