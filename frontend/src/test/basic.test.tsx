import { render, screen } from '@testing-library/react'
import { describe, expect, it } from 'vitest'

import { THEME_NAMES, THEMES, isThemeName } from '../theme/themes'
import { AppCard } from '../components/AppCard'
import type { App } from '../api/types'

describe('themes', () => {
  it('has 10 themes with the shared names', () => {
    expect(THEME_NAMES).toHaveLength(10)
    for (const n of THEME_NAMES) {
      expect(THEMES[n]).toHaveProperty('accent')
    }
    expect(isThemeName('amoled')).toBe(true)
    expect(isThemeName('neon')).toBe(false)
  })
})

describe('AppCard', () => {
  const app: App = {
    id: 1, name: 'Netflix', icon: null, url: 'https://netflix.com',
    category_id: null, mode: 'tv', is_web: true, is_active: true,
  }

  it('renders the name and reacts to open', () => {
    let opened: App | null = null
    render(<AppCard app={app} onOpen={(a) => (opened = a)} />)
    const btn = screen.getByTitle('Netflix')
    btn.click()
    expect(opened).toStrictEqual(app)
    expect(screen.getByText('Netflix')).toBeInTheDocument()
  })
})
