import { Navigate, Route, Routes } from 'react-router-dom'

import { Sidebar } from './components/Sidebar'
import { useAuth } from './auth/AuthContext'
import { Home } from './pages/Home'
import { AllApps } from './pages/AllApps'
import { Search } from './pages/Search'
import { SettingsPage } from './pages/Settings'
import { Login } from './pages/Login'
import { Admin } from './pages/Admin'

function Shell({ children }: { children: React.ReactNode }) {
  return (
    <div className="layout">
      <Sidebar />
      <main className="content">{children}</main>
    </div>
  )
}

function RequireAdmin({ children }: { children: React.ReactNode }) {
  const { isAdmin, loading } = useAuth()
  if (loading) return <p className="muted">…</p>
  return isAdmin ? <>{children}</> : <Navigate to="/login" replace />
}

export function App() {
  return (
    <Routes>
      <Route path="/login" element={<Login />} />
      <Route path="/" element={<Shell><Home /></Shell>} />
      <Route path="/apps" element={<Shell><AllApps /></Shell>} />
      <Route path="/search" element={<Shell><Search /></Shell>} />
      <Route path="/settings" element={<Shell><SettingsPage /></Shell>} />
      <Route
        path="/admin"
        element={
          <Shell>
            <RequireAdmin>
              <Admin />
            </RequireAdmin>
          </Shell>
        }
      />
      <Route path="*" element={<Navigate to="/" replace />} />
    </Routes>
  )
}
