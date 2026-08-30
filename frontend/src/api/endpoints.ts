import { api, setTokens } from './client'
import type {
  AdminUser,
  App,
  Category,
  LogEntry,
  Me,
  ServiceInfo,
  Settings,
  Tokens,
} from './types'

export const auth = {
  async login(username: string, password: string): Promise<Tokens> {
    const t = await api<Tokens>('/auth/login', {
      method: 'POST',
      body: JSON.stringify({ username, password }),
    }, { auth: false })
    setTokens(t)
    return t
  },
  register: (username: string, password: string, email?: string) =>
    api('/auth/register', {
      method: 'POST',
      body: JSON.stringify({ username, password, email: email || null }),
    }, { auth: false }),
  me: () => api<Me>('/auth/me'),
  oauthUrl: (provider: 'google' | 'github') => `/api/auth/oauth/${provider}`,
}

export const catalog = {
  apps: (mode?: string, categoryId?: number) => {
    const q = new URLSearchParams()
    if (mode) q.set('mode', mode)
    if (categoryId != null) q.set('category_id', String(categoryId))
    return api<App[]>(`/apps${q.toString() ? `?${q}` : ''}`, {}, { auth: false })
  },
  app: (id: number) => api<App>(`/apps/${id}`),
  categories: (mode?: string) =>
    api<Category[]>(`/categories${mode ? `?mode=${mode}` : ''}`, {}, { auth: false }),
  createApp: (body: Partial<App>) =>
    api<App>('/apps', { method: 'POST', body: JSON.stringify(body) }),
  updateApp: (id: number, body: Partial<App>) =>
    api<App>(`/apps/${id}`, { method: 'PUT', body: JSON.stringify(body) }),
  deleteApp: (id: number) => api(`/apps/${id}`, { method: 'DELETE' }),
  createCategory: (name: string, mode: string) =>
    api<Category>('/categories', { method: 'POST', body: JSON.stringify({ name, mode }) }),
  deleteCategory: (id: number) => api(`/categories/${id}`, { method: 'DELETE' }),
}

export const user = {
  settings: () => api<Settings>('/settings'),
  updateSettings: (patch: Partial<Settings>) =>
    api<Settings>('/settings', { method: 'PUT', body: JSON.stringify(patch) }),
  favorites: () => api<{ app_ids: number[] }>('/favorites'),
  addFavorite: (id: number) =>
    api<{ app_ids: number[] }>(`/favorites/${id}`, { method: 'POST' }),
  removeFavorite: (id: number) =>
    api<{ app_ids: number[] }>(`/favorites/${id}`, { method: 'DELETE' }),
}

export const admin = {
  users: () => api<AdminUser[]>('/admin/users'),
  logs: (limit = 50, offset = 0) =>
    api<{ total: number; items: LogEntry[] }>(`/admin/logs?limit=${limit}&offset=${offset}`),
  config: () => api<Record<string, unknown>>('/admin/config'),
  saveConfig: (config: Record<string, unknown>, password: string) =>
    api<Record<string, unknown>>('/admin/config', {
      method: 'PUT',
      body: JSON.stringify({ config, password }),
    }),
  services: () => api<ServiceInfo[]>('/admin/services'),
  serviceAction: (name: string, action: 'restart' | 'stop') =>
    api(`/admin/services/${name}/${action}`, { method: 'POST' }),
  reboot: (password: string) =>
    api('/admin/reboot', { method: 'POST', body: JSON.stringify({ password }) }),
}
