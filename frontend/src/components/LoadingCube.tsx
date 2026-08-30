// MJQbe loading cube — pure CSS 3D (6 faces, continuous rotation).
export function LoadingCube({ label = 'MJQbe' }: { label?: string }) {
  return (
    <div className="cube-overlay">
      <div className="cube-scene">
        <div className="cube">
          <span className="cube-face cf-front" />
          <span className="cube-face cf-back" />
          <span className="cube-face cf-right" />
          <span className="cube-face cf-left" />
          <span className="cube-face cf-top" />
          <span className="cube-face cf-bottom" />
        </div>
      </div>
      <div className="cube-label">{label}</div>
    </div>
  )
}
