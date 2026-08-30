import { useState } from 'react'
import { useNavigate } from 'react-router-dom'

import { useAuth } from '../auth/AuthContext'
import { auth } from '../api/endpoints'
import { ApiError } from '../api/client'

export function Login() {
  const { login } = useAuth()
  const navigate = useNavigate()
  const [mode, setMode] = useState<'login' | 'register'>('login')
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [email, setEmail] = useState('')
  const [error, setError] = useState('')
  const [busy, setBusy] = useState(false)

  const submit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError('')
    setBusy(true)
    try {
      if (mode === 'register') {
        await auth.register(username, password, email || undefined)
      }
      await login(username, password)
      navigate('/')
    } catch (err) {
      setError(err instanceof ApiError ? err.message : 'Échec de la connexion')
    } finally {
      setBusy(false)
    }
  }

  return (
    <div className="login-wrap">
      <form className="login-card" onSubmit={submit}>
        <strong>{mode === 'login' ? 'Connexion' : 'Créer un compte'}</strong>
        <input
          placeholder="Nom d'utilisateur"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          autoFocus
        />
        {mode === 'register' && (
          <input
            placeholder="Email (optionnel)"
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
          />
        )}
        <input
          placeholder="Mot de passe"
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
        {error && <div className="error">{error}</div>}
        <button className="primary" type="submit" disabled={busy}>
          {mode === 'login' ? 'Se connecter' : "S'inscrire"}
        </button>
        <div className="oauth-row">
          <a href={auth.oauthUrl('google')}>Google</a>
          <a href={auth.oauthUrl('github')}>GitHub</a>
        </div>
        <button
          type="button"
          className="nav-item"
          onClick={() => setMode(mode === 'login' ? 'register' : 'login')}
        >
          {mode === 'login' ? 'Créer un compte' : "J'ai déjà un compte"}
        </button>
      </form>
    </div>
  )
}
