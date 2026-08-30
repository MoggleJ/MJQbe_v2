// Thin fetch wrapper: base path `/api`, bearer token, transparent refresh on 401.
import type { Tokens } from './types'

const BASE = '/api'
const ACCESS = 'mjqbe.access'
const REFRESH = 'mjqbe.refresh'

export function getAccess(): string | null {
  return localStorage.getItem(ACCESS)
}
export function getRefresh(): string | null {
  return localStorage.getItem(REFRESH)
}
export function setTokens(t: Tokens): void {
  localStorage.setItem(ACCESS, t.access_token)
  localStorage.setItem(REFRESH, t.refresh_token)
}
export function clearTokens(): void {
  localStorage.removeItem(ACCESS)
  localStorage.removeItem(REFRESH)
}

export class ApiError extends Error {
  status: number
  constructor(status: number, message: string) {
    super(message)
    this.status = status
  }
}

async function raw(
  path: string,
  init: RequestInit = {},
  withAuth = true,
): Promise<Response> {
  const headers = new Headers(init.headers)
  if (init.body && !headers.has('Content-Type')) {
    headers.set('Content-Type', 'application/json')
  }
  const token = getAccess()
  if (withAuth && token) headers.set('Authorization', `Bearer ${token}`)
  return fetch(BASE + path, { ...init, headers })
}

let refreshing: Promise<boolean> | null = null

async function tryRefresh(): Promise<boolean> {
  const rt = getRefresh()
  if (!rt) return false
  if (!refreshing) {
    refreshing = raw('/auth/refresh', {
      method: 'POST',
      body: JSON.stringify({ refresh_token: rt }),
    }, false)
      .then(async (r) => {
        if (!r.ok) {
          clearTokens()
          return false
        }
        setTokens((await r.json()) as Tokens)
        return true
      })
      .finally(() => {
        refreshing = null
      })
  }
  return refreshing
}

export async function api<T = unknown>(
  path: string,
  init: RequestInit = {},
  opts: { auth?: boolean; retry?: boolean } = {},
): Promise<T> {
  const withAuth = opts.auth ?? true
  let res = await raw(path, init, withAuth)

  if (res.status === 401 && withAuth && opts.retry !== false && (await tryRefresh())) {
    res = await raw(path, init, true)
  }

  if (res.status === 204) return undefined as T
  const text = await res.text()
  const data = text ? JSON.parse(text) : undefined
  if (!res.ok) {
    const detail =
      (data && (data.detail || data.message)) || res.statusText || 'request failed'
    throw new ApiError(res.status, typeof detail === 'string' ? detail : JSON.stringify(detail))
  }
  return data as T
}
