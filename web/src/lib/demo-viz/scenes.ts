import * as THREE from 'three'
import type { DemoPalette } from './palette'

export type SceneHandle = {
  root: THREE.Group
  update: (t: number, dt: number) => void
  dispose: () => void
}

function lineMat(color: number, opacity = 0.85): THREE.LineBasicMaterial {
  return new THREE.LineBasicMaterial({
    color,
    transparent: opacity < 1,
    opacity,
    depthWrite: false,
  })
}

function wire(geo: THREE.BufferGeometry, color: number, opacity = 0.85): THREE.LineSegments {
  const edges = new THREE.EdgesGeometry(geo, 18)
  const mesh = new THREE.LineSegments(edges, lineMat(color, opacity))
  geo.dispose()
  return mesh
}

function disposeObject(obj: THREE.Object3D) {
  obj.traverse((child) => {
    const mesh = child as THREE.Mesh | THREE.LineSegments | THREE.Line | THREE.Points
    if ('geometry' in mesh && mesh.geometry) mesh.geometry.dispose()
    if ('material' in mesh && mesh.material) {
      const mats = Array.isArray(mesh.material) ? mesh.material : [mesh.material]
      for (const m of mats) m.dispose()
    }
  })
}

function pointsFrom(positions: Float32Array, color: number, size = 0.035, opacity = 0.9): THREE.Points {
  const geo = new THREE.BufferGeometry()
  geo.setAttribute('position', new THREE.BufferAttribute(positions, 3))
  const mat = new THREE.PointsMaterial({
    color,
    size,
    transparent: true,
    opacity,
    depthWrite: false,
    sizeAttenuation: true,
  })
  return new THREE.Points(geo, mat)
}

/* ─────────────────────────────────────────────────────────────
 * 0 · Agent speak — Neural Voice Core
 *    Icosahedron mind with expanding speech rings + orbit graph
 * ───────────────────────────────────────────────────────────── */

export function createAgentScene(p: DemoPalette): SceneHandle {
  const root = new THREE.Group()
  const disposables: THREE.BufferGeometry[] = []

  const core = wire(new THREE.IcosahedronGeometry(0.9, 1), p.ink, 0.75)
  root.add(core)

  const inner = wire(new THREE.IcosahedronGeometry(0.45, 0), p.orange, 0.9)
  root.add(inner)

  // Expanding speech rings
  const rings: THREE.Line[] = []
  for (let i = 0; i < 4; i++) {
    const geo = new THREE.BufferGeometry()
    const segs = 96
    const pos = new Float32Array((segs + 1) * 3)
    for (let s = 0; s <= segs; s++) {
      const a = (s / segs) * Math.PI * 2
      pos[s * 3] = Math.cos(a)
      pos[s * 3 + 1] = 0
      pos[s * 3 + 2] = Math.sin(a)
    }
    geo.setAttribute('position', new THREE.BufferAttribute(pos, 3))
    disposables.push(geo)
    const ring = new THREE.Line(geo, lineMat(p.orange, 0.55))
    ring.rotation.x = Math.PI * 0.12 * (i % 2 === 0 ? 1 : -1)
    root.add(ring)
    rings.push(ring)
  }

  // Orbiting agent nodes
  const nodeCount = 14
  const nodePos = new Float32Array(nodeCount * 3)
  const nodes = pointsFrom(nodePos, p.orangeDark, 0.05, 0.95)
  root.add(nodes)

  // Faint connection lines from core to nodes
  const linkGeo = new THREE.BufferGeometry()
  const linkPos = new Float32Array(nodeCount * 2 * 3)
  linkGeo.setAttribute('position', new THREE.BufferAttribute(linkPos, 3))
  disposables.push(linkGeo)
  const links = new THREE.LineSegments(linkGeo, lineMat(p.inkSoft, 0.28))
  root.add(links)

  // Floating speech particles
  const sparkCount = 48
  const sparkPos = new Float32Array(sparkCount * 3)
  const sparkVel = new Float32Array(sparkCount)
  for (let i = 0; i < sparkCount; i++) {
    const a = Math.random() * Math.PI * 2
    const r = 1.2 + Math.random() * 1.4
    sparkPos[i * 3] = Math.cos(a) * r
    sparkPos[i * 3 + 1] = (Math.random() - 0.5) * 1.6
    sparkPos[i * 3 + 2] = Math.sin(a) * r
    sparkVel[i] = 0.15 + Math.random() * 0.35
  }
  const sparks = pointsFrom(sparkPos, p.orange, 0.028, 0.7)
  root.add(sparks)

  return {
    root,
    update(t) {
      core.rotation.y = t * 0.22
      core.rotation.x = Math.sin(t * 0.3) * 0.12
      inner.rotation.y = -t * 0.55
      inner.rotation.z = t * 0.18
      const pulse = 1 + Math.sin(t * 2.4) * 0.06
      core.scale.setScalar(pulse)
      inner.scale.setScalar(1 + Math.sin(t * 3.1 + 1) * 0.1)

      for (let i = 0; i < rings.length; i++) {
        const phase = (t * 0.55 + i * 0.28) % 1
        const s = 1.1 + phase * 2.4
        rings[i].scale.set(s, s, s)
        const mat = rings[i].material as THREE.LineBasicMaterial
        mat.opacity = (1 - phase) * 0.55
        rings[i].rotation.z = t * 0.15 * (i % 2 === 0 ? 1 : -1)
      }

      const nPos = nodes.geometry.attributes.position.array as Float32Array
      const lPos = linkGeo.attributes.position.array as Float32Array
      for (let i = 0; i < nodeCount; i++) {
        const a = t * 0.4 + (i / nodeCount) * Math.PI * 2
        const elev = Math.sin(t * 0.7 + i) * 0.45
        const r = 1.55 + Math.sin(t + i * 1.7) * 0.15
        const x = Math.cos(a) * r
        const y = elev
        const z = Math.sin(a) * r * 0.72
        nPos[i * 3] = x
        nPos[i * 3 + 1] = y
        nPos[i * 3 + 2] = z
        lPos[i * 6] = 0
        lPos[i * 6 + 1] = 0
        lPos[i * 6 + 2] = 0
        lPos[i * 6 + 3] = x
        lPos[i * 6 + 4] = y
        lPos[i * 6 + 5] = z
      }
      nodes.geometry.attributes.position.needsUpdate = true
      linkGeo.attributes.position.needsUpdate = true

      const sPos = sparks.geometry.attributes.position.array as Float32Array
      for (let i = 0; i < sparkCount; i++) {
        sPos[i * 3 + 1] += sparkVel[i] * 0.016
        if (sPos[i * 3 + 1] > 1.4) {
          sPos[i * 3 + 1] = -1.4
          const a = Math.random() * Math.PI * 2
          const r = 1.1 + Math.random() * 1.5
          sPos[i * 3] = Math.cos(a) * r
          sPos[i * 3 + 2] = Math.sin(a) * r
        }
      }
      sparks.geometry.attributes.position.needsUpdate = true
    },
    dispose() {
      disposeObject(root)
      for (const g of disposables) g.dispose()
    },
  }
}

/* ─────────────────────────────────────────────────────────────
 * 1 · Articles — Living Document
 *    Floating wireframe pages with a sonic ribbon weaving through
 * ───────────────────────────────────────────────────────────── */

export function createArticlesScene(p: DemoPalette): SceneHandle {
  const root = new THREE.Group()
  const pages: THREE.Group[] = []
  const pageCount = 5

  for (let i = 0; i < pageCount; i++) {
    const g = new THREE.Group()
    const page = wire(new THREE.PlaneGeometry(1.6, 2.1), i === 0 ? p.ink : p.inkSoft, i === 0 ? 0.8 : 0.4)
    g.add(page)

    // Text lines as short horizontal segments
    const lineCount = 8
    const positions: number[] = []
    for (let l = 0; l < lineCount; l++) {
      const y = 0.75 - l * 0.2
      const w = 0.45 + (l % 3) * 0.12
      positions.push(-0.55, y, 0.01, -0.55 + w * 2, y, 0.01)
    }
    // Title bar
    positions.push(-0.55, 0.92, 0.01, 0.35, 0.92, 0.01)

    const textGeo = new THREE.BufferGeometry()
    textGeo.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3))
    const textLines = new THREE.LineSegments(textGeo, lineMat(i === 0 ? p.orange : p.inkDim, i === 0 ? 0.75 : 0.35))
    g.add(textLines)

    g.position.set((i - 2) * 0.18, (i - 2) * 0.08, -i * 0.22)
    g.rotation.y = (i - 2) * 0.08
    g.rotation.x = -0.12
    root.add(g)
    pages.push(g)
  }

  // Sonic ribbon weaving through pages
  const ribbonSegs = 80
  const ribbonGeo = new THREE.BufferGeometry()
  const ribbonPos = new Float32Array(ribbonSegs * 3)
  ribbonGeo.setAttribute('position', new THREE.BufferAttribute(ribbonPos, 3))
  const ribbon = new THREE.Line(ribbonGeo, lineMat(p.orange, 0.85))
  root.add(ribbon)

  // Secondary thinner ribbon
  const ribbon2Geo = new THREE.BufferGeometry()
  const ribbon2Pos = new Float32Array(ribbonSegs * 3)
  ribbon2Geo.setAttribute('position', new THREE.BufferAttribute(ribbon2Pos, 3))
  const ribbon2 = new THREE.Line(ribbon2Geo, lineMat(p.orangeDark, 0.45))
  root.add(ribbon2)

  // Floating audio dots along the ribbon
  const dots = pointsFrom(new Float32Array(16 * 3), p.orange, 0.04, 0.9)
  root.add(dots)

  return {
    root,
    update(t) {
      for (let i = 0; i < pages.length; i++) {
        const base = (i - 2) * 0.08
        pages[i].position.y = base + Math.sin(t * 0.9 + i * 0.7) * 0.08
        pages[i].rotation.z = Math.sin(t * 0.5 + i) * 0.03
        pages[i].rotation.y = (i - 2) * 0.08 + Math.sin(t * 0.35 + i * 0.4) * 0.04
      }

      for (let s = 0; s < ribbonSegs; s++) {
        const u = s / (ribbonSegs - 1)
        const x = -1.4 + u * 2.8
        const y = Math.sin(u * Math.PI * 3 + t * 1.6) * 0.55 + Math.sin(t * 0.8) * 0.1
        const z = Math.cos(u * Math.PI * 2 + t) * 0.35 + 0.4
        ribbonPos[s * 3] = x
        ribbonPos[s * 3 + 1] = y
        ribbonPos[s * 3 + 2] = z
        ribbon2Pos[s * 3] = x
        ribbon2Pos[s * 3 + 1] = y * 0.6 - 0.25
        ribbon2Pos[s * 3 + 2] = z * 0.8 - 0.15
      }
      ribbonGeo.attributes.position.needsUpdate = true
      ribbon2Geo.attributes.position.needsUpdate = true

      const dPos = dots.geometry.attributes.position.array as Float32Array
      for (let i = 0; i < 16; i++) {
        const u = ((t * 0.12 + i / 16) % 1)
        const idx = Math.floor(u * (ribbonSegs - 1))
        dPos[i * 3] = ribbonPos[idx * 3]
        dPos[i * 3 + 1] = ribbonPos[idx * 3 + 1]
        dPos[i * 3 + 2] = ribbonPos[idx * 3 + 2]
      }
      dots.geometry.attributes.position.needsUpdate = true

      root.rotation.y = Math.sin(t * 0.2) * 0.15
    },
    dispose() {
      disposeObject(root)
    },
  }
}

/* ─────────────────────────────────────────────────────────────
 * 2 · Podcasts — Broadcast Disc
 *    Spinning wireframe platter + mic + radiating rings
 * ───────────────────────────────────────────────────────────── */

export function createPodcastsScene(p: DemoPalette): SceneHandle {
  const root = new THREE.Group()

  // Vinyl / broadcast disc
  const disc = wire(new THREE.CylinderGeometry(1.35, 1.35, 0.06, 48, 1, true), p.ink, 0.55)
  disc.rotation.x = Math.PI * 0.5
  root.add(disc)

  const discTop = wire(new THREE.CircleGeometry(1.35, 48), p.inkSoft, 0.4)
  discTop.rotation.x = -Math.PI * 0.5
  discTop.position.y = 0.04
  root.add(discTop)

  // Groove rings
  const grooves: THREE.Line[] = []
  for (let i = 0; i < 6; i++) {
    const r = 0.35 + i * 0.16
    const geo = new THREE.BufferGeometry()
    const segs = 64
    const pos = new Float32Array((segs + 1) * 3)
    for (let s = 0; s <= segs; s++) {
      const a = (s / segs) * Math.PI * 2
      pos[s * 3] = Math.cos(a) * r
      pos[s * 3 + 1] = 0.05
      pos[s * 3 + 2] = Math.sin(a) * r
    }
    geo.setAttribute('position', new THREE.BufferAttribute(pos, 3))
    const groove = new THREE.Line(geo, lineMat(p.inkDim, 0.35))
    root.add(groove)
    grooves.push(groove)
  }

  // Center hub
  const hub = wire(new THREE.TorusGeometry(0.18, 0.04, 8, 24), p.orange, 0.95)
  hub.rotation.x = Math.PI * 0.5
  hub.position.y = 0.06
  root.add(hub)

  // Wireframe microphone
  const mic = new THREE.Group()
  const grille = wire(new THREE.SphereGeometry(0.28, 12, 10), p.ink, 0.8)
  grille.scale.set(1, 1.25, 1)
  mic.add(grille)
  const stem = wire(new THREE.CylinderGeometry(0.05, 0.08, 0.7, 8), p.inkSoft, 0.7)
  stem.position.y = -0.55
  mic.add(stem)
  const base = wire(new THREE.CylinderGeometry(0.22, 0.28, 0.08, 12), p.inkSoft, 0.6)
  base.position.y = -0.92
  mic.add(base)
  mic.position.set(1.55, 0.55, 0.6)
  mic.rotation.z = -0.25
  mic.rotation.y = -0.4
  root.add(mic)

  // Radiating broadcast rings (vertical plane facing camera-ish)
  const broadcast: THREE.Line[] = []
  for (let i = 0; i < 5; i++) {
    const geo = new THREE.BufferGeometry()
    const segs = 48
    const pos = new Float32Array((segs + 1) * 3)
    // Arc in front of mic
    for (let s = 0; s <= segs; s++) {
      const a = -Math.PI * 0.55 + (s / segs) * Math.PI * 1.1
      pos[s * 3] = Math.cos(a)
      pos[s * 3 + 1] = Math.sin(a)
      pos[s * 3 + 2] = 0
    }
    geo.setAttribute('position', new THREE.BufferAttribute(pos, 3))
    const arc = new THREE.Line(geo, lineMat(p.orange, 0.6))
    arc.position.copy(mic.position)
    arc.position.x -= 0.2
    root.add(arc)
    broadcast.push(arc)
  }

  // Perimeter waveform bars
  const barCount = 36
  const bars: THREE.Line[] = []
  for (let i = 0; i < barCount; i++) {
    const a = (i / barCount) * Math.PI * 2
    const geo = new THREE.BufferGeometry()
    const pos = new Float32Array(6)
    pos[0] = Math.cos(a) * 1.45
    pos[1] = 0.05
    pos[2] = Math.sin(a) * 1.45
    pos[3] = Math.cos(a) * 1.45
    pos[4] = 0.35
    pos[5] = Math.sin(a) * 1.45
    geo.setAttribute('position', new THREE.BufferAttribute(pos, 3))
    const bar = new THREE.Line(geo, lineMat(p.orangeDark, 0.55))
    root.add(bar)
    bars.push(bar)
  }

  root.rotation.x = 0.55
  root.position.y = -0.15

  return {
    root,
    update(t) {
      disc.rotation.z = t * 0.35
      discTop.rotation.z = t * 0.35
      for (const g of grooves) g.rotation.y = t * 0.35
      hub.rotation.z = -t * 0.8

      mic.position.y = 0.55 + Math.sin(t * 1.4) * 0.04
      mic.rotation.z = -0.25 + Math.sin(t * 0.9) * 0.03

      for (let i = 0; i < broadcast.length; i++) {
        const phase = (t * 0.7 + i * 0.22) % 1
        const s = 0.35 + phase * 1.4
        broadcast[i].scale.set(s, s, s)
        const mat = broadcast[i].material as THREE.LineBasicMaterial
        mat.opacity = (1 - phase) * 0.65
      }

      for (let i = 0; i < bars.length; i++) {
        const h = 0.12 + Math.abs(Math.sin(t * 3.2 + i * 0.45)) * 0.55
        const a = (i / barCount) * Math.PI * 2
        const pos = bars[i].geometry.attributes.position.array as Float32Array
        pos[4] = 0.05 + h
        pos[0] = Math.cos(a + t * 0.35) * 1.45
        pos[2] = Math.sin(a + t * 0.35) * 1.45
        pos[3] = pos[0]
        pos[5] = pos[2]
        bars[i].geometry.attributes.position.needsUpdate = true
      }

      root.rotation.y = Math.sin(t * 0.18) * 0.12
    },
    dispose() {
      disposeObject(root)
    },
  }
}

/* ─────────────────────────────────────────────────────────────
 * 3 · Providers — Dual Engines
 *    Two cores linked by a flowing energy helix
 * ───────────────────────────────────────────────────────────── */

export function createProvidersScene(p: DemoPalette): SceneHandle {
  const root = new THREE.Group()

  const left = new THREE.Group()
  const right = new THREE.Group()

  const leftCore = wire(new THREE.IcosahedronGeometry(0.7, 1), p.ink, 0.75)
  const leftInner = wire(new THREE.OctahedronGeometry(0.32), p.orange, 0.95)
  left.add(leftCore, leftInner)
  left.position.x = -1.45

  const rightCore = wire(new THREE.DodecahedronGeometry(0.65, 0), p.ink, 0.75)
  const rightInner = wire(new THREE.OctahedronGeometry(0.32), p.live, 0.9)
  right.add(rightCore, rightInner)
  right.position.x = 1.45

  root.add(left, right)

  // Orbit rings around each
  const leftRing = wire(new THREE.TorusGeometry(1.0, 0.012, 6, 48), p.orange, 0.55)
  leftRing.rotation.x = Math.PI * 0.55
  left.add(leftRing)
  const rightRing = wire(new THREE.TorusGeometry(1.0, 0.012, 6, 48), p.live, 0.5)
  rightRing.rotation.x = Math.PI * 0.4
  rightRing.rotation.y = 0.4
  right.add(rightRing)

  // Energy helix between them
  const helixSegs = 100
  const helixGeo = new THREE.BufferGeometry()
  const helixPos = new Float32Array(helixSegs * 3)
  helixGeo.setAttribute('position', new THREE.BufferAttribute(helixPos, 3))
  const helix = new THREE.Line(helixGeo, lineMat(p.orange, 0.7))
  root.add(helix)

  const helix2Geo = new THREE.BufferGeometry()
  const helix2Pos = new Float32Array(helixSegs * 3)
  helix2Geo.setAttribute('position', new THREE.BufferAttribute(helix2Pos, 3))
  const helix2 = new THREE.Line(helix2Geo, lineMat(p.live, 0.55))
  root.add(helix2)

  // Traveling pulse particles on the link
  const pulseCount = 12
  const pulses = pointsFrom(new Float32Array(pulseCount * 3), p.orangeDark, 0.05, 0.95)
  root.add(pulses)

  // Ambient orbit dots
  const ambient = pointsFrom(new Float32Array(24 * 3), p.inkDim, 0.025, 0.5)
  root.add(ambient)

  return {
    root,
    update(t) {
      left.rotation.y = t * 0.45
      left.rotation.x = Math.sin(t * 0.4) * 0.15
      right.rotation.y = -t * 0.38
      right.rotation.z = Math.sin(t * 0.35 + 1) * 0.12
      leftInner.rotation.y = -t * 1.2
      rightInner.rotation.x = t * 1.1
      leftRing.rotation.z = t * 0.6
      rightRing.rotation.z = -t * 0.5

      left.position.y = Math.sin(t * 0.9) * 0.1
      right.position.y = Math.sin(t * 0.9 + Math.PI) * 0.1

      for (let s = 0; s < helixSegs; s++) {
        const u = s / (helixSegs - 1)
        const x = -1.2 + u * 2.4
        const a = u * Math.PI * 6 + t * 2.2
        helixPos[s * 3] = x
        helixPos[s * 3 + 1] = Math.sin(a) * 0.35
        helixPos[s * 3 + 2] = Math.cos(a) * 0.35
        helix2Pos[s * 3] = x
        helix2Pos[s * 3 + 1] = Math.sin(a + Math.PI) * 0.35
        helix2Pos[s * 3 + 2] = Math.cos(a + Math.PI) * 0.35
      }
      helixGeo.attributes.position.needsUpdate = true
      helix2Geo.attributes.position.needsUpdate = true

      const pPos = pulses.geometry.attributes.position.array as Float32Array
      for (let i = 0; i < pulseCount; i++) {
        const u = (t * 0.25 + i / pulseCount) % 1
        const idx = Math.floor(u * (helixSegs - 1))
        pPos[i * 3] = helixPos[idx * 3]
        pPos[i * 3 + 1] = helixPos[idx * 3 + 1]
        pPos[i * 3 + 2] = helixPos[idx * 3 + 2]
      }
      pulses.geometry.attributes.position.needsUpdate = true

      const aPos = ambient.geometry.attributes.position.array as Float32Array
      for (let i = 0; i < 24; i++) {
        const a = t * 0.3 + (i / 24) * Math.PI * 2
        const r = 2.2 + Math.sin(t + i) * 0.15
        aPos[i * 3] = Math.cos(a) * r
        aPos[i * 3 + 1] = Math.sin(a * 2 + t) * 0.6
        aPos[i * 3 + 2] = Math.sin(a) * r * 0.4
      }
      ambient.geometry.attributes.position.needsUpdate = true

      root.rotation.y = Math.sin(t * 0.15) * 0.1
    },
    dispose() {
      disposeObject(root)
    },
  }
}

/* ─────────────────────────────────────────────────────────────
 * 4 · CLI control — Command Lattice
 *    Nested wireframe cubes + orbiting control nodes
 * ───────────────────────────────────────────────────────────── */

export function createCliScene(p: DemoPalette): SceneHandle {
  const root = new THREE.Group()

  // Nested cubes
  const cubes: THREE.LineSegments[] = []
  const sizes = [0.55, 1.05, 1.65]
  const colors = [p.orange, p.ink, p.inkSoft]
  const opacities = [0.95, 0.7, 0.4]
  for (let i = 0; i < sizes.length; i++) {
    const c = wire(new THREE.BoxGeometry(sizes[i], sizes[i], sizes[i]), colors[i], opacities[i])
    root.add(c)
    cubes.push(c)
  }

  // Diagonal cross-braces on outer cube
  const bracePositions = [
    -0.825, -0.825, -0.825, 0.825, 0.825, 0.825,
    0.825, -0.825, -0.825, -0.825, 0.825, 0.825,
    -0.825, 0.825, -0.825, 0.825, -0.825, 0.825,
    0.825, 0.825, -0.825, -0.825, -0.825, 0.825,
  ]
  const braceGeo = new THREE.BufferGeometry()
  braceGeo.setAttribute('position', new THREE.Float32BufferAttribute(bracePositions, 3))
  const braces = new THREE.LineSegments(braceGeo, lineMat(p.inkDim, 0.25))
  root.add(braces)

  // Orbiting flag nodes (CLI flags as octahedrons)
  const flagGroup = new THREE.Group()
  const flags: THREE.LineSegments[] = []
  const flagCount = 6
  for (let i = 0; i < flagCount; i++) {
    const f = wire(new THREE.OctahedronGeometry(0.12), i % 2 === 0 ? p.orange : p.orangeDark, 0.9)
    flagGroup.add(f)
    flags.push(f)
  }
  root.add(flagGroup)

  // Connection trails from center to flags
  const trailGeo = new THREE.BufferGeometry()
  const trailPos = new Float32Array(flagCount * 2 * 3)
  trailGeo.setAttribute('position', new THREE.BufferAttribute(trailPos, 3))
  const trails = new THREE.LineSegments(trailGeo, lineMat(p.orange, 0.3))
  root.add(trails)

  // Grid floor
  const grid = new THREE.GridHelper(5, 10, p.line, p.line)
  grid.position.y = -1.35
  const gridMat = grid.material as THREE.Material | THREE.Material[]
  if (Array.isArray(gridMat)) {
    for (const m of gridMat) {
      m.transparent = true
      ;(m as THREE.LineBasicMaterial).opacity = 0.35
    }
  } else {
    gridMat.transparent = true
    ;(gridMat as THREE.LineBasicMaterial).opacity = 0.35
  }
  root.add(grid)

  // Floating bit particles
  const bitCount = 40
  const bits = pointsFrom(new Float32Array(bitCount * 3), p.inkDim, 0.022, 0.55)
  root.add(bits)

  return {
    root,
    update(t) {
      cubes[0].rotation.x = t * 0.7
      cubes[0].rotation.y = t * 0.9
      cubes[1].rotation.y = -t * 0.35
      cubes[1].rotation.z = t * 0.2
      cubes[2].rotation.x = Math.sin(t * 0.3) * 0.2
      cubes[2].rotation.y = t * 0.18

      const pulse = 1 + Math.sin(t * 2.5) * 0.05
      cubes[0].scale.setScalar(pulse)

      for (let i = 0; i < flagCount; i++) {
        const a = t * 0.55 + (i / flagCount) * Math.PI * 2
        const elev = Math.sin(t * 1.1 + i * 1.3) * 0.55
        const r = 1.35 + Math.sin(t * 0.8 + i) * 0.1
        const x = Math.cos(a) * r
        const y = elev
        const z = Math.sin(a) * r
        flags[i].position.set(x, y, z)
        flags[i].rotation.x = t * (1 + i * 0.2)
        flags[i].rotation.y = t * 0.8

        trailPos[i * 6] = 0
        trailPos[i * 6 + 1] = 0
        trailPos[i * 6 + 2] = 0
        trailPos[i * 6 + 3] = x
        trailPos[i * 6 + 4] = y
        trailPos[i * 6 + 5] = z
      }
      trailGeo.attributes.position.needsUpdate = true

      const bPos = bits.geometry.attributes.position.array as Float32Array
      for (let i = 0; i < bitCount; i++) {
        const a = t * 0.2 + i * 0.9
        const r = 0.8 + (i % 5) * 0.35
        bPos[i * 3] = Math.cos(a) * r
        bPos[i * 3 + 1] = ((t * 0.3 + i * 0.17) % 2.4) - 1.2
        bPos[i * 3 + 2] = Math.sin(a * 1.3) * r
      }
      bits.geometry.attributes.position.needsUpdate = true

      root.rotation.y = t * 0.12
    },
    dispose() {
      disposeObject(root)
    },
  }
}

export type SceneFactory = (p: DemoPalette) => SceneHandle

export const sceneFactories: SceneFactory[] = [
  createAgentScene,
  createArticlesScene,
  createPodcastsScene,
  createProvidersScene,
  createCliScene,
]
