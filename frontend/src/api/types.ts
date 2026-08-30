export type Mode = 'tv' | 'desktop'

export interface App {
  id: number
  name: string
  icon: string | null
  url: string | null
  category_id: number | null
  mode: string
  is_web: boolean
  is_active: boolean
}

export interface Category {
  id: number
  name: string
  mode: string
}

export interface Settings {
  user_id: number
  theme: string
  layout: 'grid' | 'list'
  icon_size: 'small' | 'medium' | 'large'
  default_mode: 'tv' | 'desktop'
}

export interface Me {
  id: number
  username: string
  email: string | null
  role: 'user' | 'admin'
}

export interface Tokens {
  access_token: string
  refresh_token: string
  token_type: string
}

export interface AdminUser {
  id: number
  username: string
  email: string | null
  role: string
  oauth_provider: string | null
  created_at: string | null
  last_login: string | null
}

export interface LogEntry {
  id: number
  user_id: number | null
  action: string
  metadata: Record<string, unknown> | null
  created_at: string
}

export interface ServiceInfo {
  service: string
  name: string
  id: string
  image: string
  state: string
  status: string
}
