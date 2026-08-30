import { useEffect, useState } from 'react'

export function Clock() {
  const [now, setNow] = useState(() => new Date())
  useEffect(() => {
    const id = setInterval(() => setNow(new Date()), 1000)
    return () => clearInterval(id)
  }, [])
  return (
    <span>
      {now.toLocaleDateString('fr-FR', { weekday: 'short', day: '2-digit', month: 'short' })}{' '}
      {now.toLocaleTimeString('fr-FR')}
    </span>
  )
}
