import { useEffect, useState } from 'react'

import { admin, catalog } from '../api/endpoints'
import { ApiError } from '../api/client'
import { Modal } from '../components/Modal'
import type { AdminUser, App, Category, LogEntry, ServiceInfo } from '../api/types'

type Tab = 'users' | 'catalog' | 'system' | 'logs'

export function Admin() {
  const [tab, setTab] = useState<Tab>('users')
  return (
    <div className="page-enter">
      <h2>Administration</h2>
      <div className="admin-tabs">
        {(['users', 'catalog', 'logs', 'system'] as Tab[]).map((t) => (
          <button
            key={t}
            className={`chip${tab === t ? ' active' : ''}`}
            onClick={() => setTab(t)}
          >
            {t}
          </button>
        ))}
      </div>
      {tab === 'users' && <Users />}
      {tab === 'catalog' && <CatalogAdmin />}
      {tab === 'logs' && <Logs />}
      {tab === 'system' && <SystemAdmin />}
    </div>
  )
}

function Users() {
  const [users, setUsers] = useState<AdminUser[]>([])
  useEffect(() => {
    admin.users().then(setUsers).catch(() => {})
  }, [])
  return (
    <div className="section">
      <table>
        <thead>
          <tr><th>id</th><th>username</th><th>email</th><th>rôle</th><th>oauth</th><th>dernière connexion</th></tr>
        </thead>
        <tbody>
          {users.map((u) => (
            <tr key={u.id}>
              <td>{u.id}</td><td>{u.username}</td><td>{u.email ?? '—'}</td>
              <td>{u.role}</td><td>{u.oauth_provider ?? '—'}</td>
              <td>{u.last_login ? new Date(u.last_login).toLocaleString('fr-FR') : '—'}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}

function CatalogAdmin() {
  const [apps, setApps] = useState<App[]>([])
  const [cats, setCats] = useState<Category[]>([])
  const [name, setName] = useState('')
  const [mode, setMode] = useState('tv')
  const [url, setUrl] = useState('')
  const [catName, setCatName] = useState('')
  const [err, setErr] = useState('')

  const reload = () => {
    Promise.all([
      Promise.all(['tv', 'desktop', 'dev'].map((m) => catalog.apps(m))).then((r) => r.flat()),
      Promise.all(['tv', 'desktop', 'dev'].map((m) => catalog.categories(m))).then((r) => r.flat()),
    ]).then(([a, c]) => {
      setApps(a)
      setCats(c)
    })
  }
  useEffect(reload, [])

  const addApp = async () => {
    setErr('')
    try {
      await catalog.createApp({ name, mode, url: url || null })
      setName('')
      setUrl('')
      reload()
    } catch (e) {
      setErr(e instanceof ApiError ? e.message : 'erreur')
    }
  }

  return (
    <>
      <div className="section">
        <h3>Nouvelle app</h3>
        <div className="row">
          <input placeholder="nom" value={name} onChange={(e) => setName(e.target.value)} />
          <select value={mode} onChange={(e) => setMode(e.target.value)}>
            <option>tv</option><option>desktop</option><option>dev</option>
          </select>
          <input placeholder="url" value={url} onChange={(e) => setUrl(e.target.value)} />
          <button className="primary" onClick={addApp} disabled={!name}>Ajouter</button>
        </div>
        {err && <div className="error">{err}</div>}
      </div>

      <div className="section">
        <h3>Apps ({apps.length})</h3>
        <table>
          <thead><tr><th>id</th><th>nom</th><th>mode</th><th>actif</th><th></th></tr></thead>
          <tbody>
            {apps.map((a) => (
              <tr key={a.id}>
                <td>{a.id}</td><td>{a.name}</td><td>{a.mode}</td>
                <td>{a.is_active ? '✓' : '—'}</td>
                <td>
                  <button onClick={() => catalog.updateApp(a.id, { is_active: !a.is_active }).then(reload)}>
                    {a.is_active ? 'désactiver' : 'activer'}
                  </button>{' '}
                  <button onClick={() => catalog.deleteApp(a.id).then(reload)}>suppr</button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="section">
        <h3>Catégories</h3>
        <div className="row">
          <input placeholder="nom" value={catName} onChange={(e) => setCatName(e.target.value)} />
          <select value={mode} onChange={(e) => setMode(e.target.value)}>
            <option>tv</option><option>desktop</option><option>dev</option>
          </select>
          <button
            className="primary"
            disabled={!catName}
            onClick={() => catalog.createCategory(catName, mode).then(() => { setCatName(''); reload() })}
          >
            Ajouter
          </button>
        </div>
        <table>
          <thead><tr><th>id</th><th>nom</th><th>mode</th><th></th></tr></thead>
          <tbody>
            {cats.map((c) => (
              <tr key={c.id}>
                <td>{c.id}</td><td>{c.name}</td><td>{c.mode}</td>
                <td><button onClick={() => catalog.deleteCategory(c.id).then(reload)}>suppr</button></td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </>
  )
}

function Logs() {
  const [data, setData] = useState<{ total: number; items: LogEntry[] }>({ total: 0, items: [] })
  useEffect(() => {
    admin.logs(100).then(setData).catch(() => {})
  }, [])
  return (
    <div className="section">
      <p className="muted">{data.total} entrées</p>
      <table>
        <thead><tr><th>date</th><th>user</th><th>action</th><th>métadonnées</th></tr></thead>
        <tbody>
          {data.items.map((l) => (
            <tr key={l.id}>
              <td>{new Date(l.created_at).toLocaleString('fr-FR')}</td>
              <td>{l.user_id ?? '—'}</td>
              <td>{l.action}</td>
              <td><code>{JSON.stringify(l.metadata)}</code></td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}

function SystemAdmin() {
  const [services, setServices] = useState<ServiceInfo[]>([])
  const [servicesErr, setServicesErr] = useState('')
  const [config, setConfig] = useState('')
  const [configMsg, setConfigMsg] = useState('')
  const [reauth, setReauth] = useState<null | { action: () => void; label: string }>(null)
  const [password, setPassword] = useState('')

  const loadServices = () =>
    admin.services().then(setServices).catch((e) => setServicesErr(String(e)))
  useEffect(() => {
    loadServices()
    admin.config().then((c) => setConfig(JSON.stringify(c, null, 2))).catch(() => {})
  }, [])

  const saveConfig = (pwd: string) => {
    setConfigMsg('')
    try {
      admin
        .saveConfig(JSON.parse(config), pwd)
        .then((c) => {
          setConfig(JSON.stringify(c, null, 2))
          setConfigMsg('Enregistré.')
        })
        .catch((e) => setConfigMsg(e instanceof ApiError ? e.message : 'erreur'))
    } catch {
      setConfigMsg('JSON invalide')
    }
  }

  return (
    <>
      <div className="section">
        <h3>Services Docker</h3>
        {servicesErr && <div className="error">{servicesErr}</div>}
        <table>
          <thead><tr><th>service</th><th>état</th><th>statut</th><th></th></tr></thead>
          <tbody>
            {services.map((s) => (
              <tr key={s.id}>
                <td>{s.service}</td>
                <td style={{ color: s.state === 'running' ? 'var(--accent)' : 'var(--text-dim)' }}>{s.state}</td>
                <td className="muted">{s.status}</td>
                <td>
                  <button onClick={() => admin.serviceAction(s.service, 'restart').then(loadServices)}>restart</button>{' '}
                  <button onClick={() => admin.serviceAction(s.service, 'stop').then(loadServices)}>stop</button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        <button
          className="primary"
          onClick={() => setReauth({ label: 'Redémarrer tous les services', action: () => admin.reboot(password).then(loadServices) })}
        >
          Reboot tout
        </button>
      </div>

      <div className="section">
        <h3>config.yml</h3>
        <textarea className="config" value={config} onChange={(e) => setConfig(e.target.value)} />
        <div className="row">
          <button
            className="primary"
            onClick={() => setReauth({ label: 'Enregistrer config.yml', action: () => saveConfig(password) })}
          >
            Enregistrer
          </button>
          {configMsg && <span className="muted">{configMsg}</span>}
        </div>
      </div>

      {reauth && (
        <Modal title="Ré-authentification" onClose={() => setReauth(null)}>
          <p className="muted">{reauth.label}</p>
          <input
            type="password"
            placeholder="Mot de passe admin"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            autoFocus
          />
          <button
            className="primary"
            onClick={() => {
              reauth.action()
              setReauth(null)
              setPassword('')
            }}
          >
            Confirmer
          </button>
        </Modal>
      )}
    </>
  )
}
