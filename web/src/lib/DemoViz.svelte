<script lang="ts">
  import { onMount } from 'svelte'

  let {
    activeIndex = 0,
    label = '',
  }: {
    activeIndex?: number
    label?: string
  } = $props()

  let host: HTMLDivElement | undefined = $state()

  let engine: {
    mountScene: (index: number) => void
  } | null = null

  $effect(() => {
    const index = activeIndex
    engine?.mountScene(index)
  })

  onMount(() => {
    if (!host) return

    let disposed = false
    let cleanup: (() => void) | undefined

    ;(async () => {
      const THREE = await import('three')
      const { readPalette } = await import('./demo-viz/palette')
      const { sceneFactories } = await import('./demo-viz/scenes')
      type SceneHandle = import('./demo-viz/scenes').SceneHandle

      if (disposed || !host) return

      const prefersReduced = window.matchMedia('(prefers-reduced-motion: reduce)')
      let reducedMotion = prefersReduced.matches

      const scene = new THREE.Scene()
      const camera = new THREE.PerspectiveCamera(38, 1, 0.1, 100)
      camera.position.set(0, 0.35, 5.2)
      camera.lookAt(0, 0, 0)

      const renderer = new THREE.WebGLRenderer({
        antialias: true,
        alpha: true,
        powerPreference: 'high-performance',
      })
      renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2))
      renderer.setClearColor(0x000000, 0)
      renderer.domElement.style.display = 'block'
      renderer.domElement.style.width = '100%'
      renderer.domElement.style.height = '100%'
      host.appendChild(renderer.domElement)

      let handle: SceneHandle | null = null
      let currentIndex = -1
      let raf = 0
      let running = true
      let visible = true
      let last = performance.now()
      let elapsed = 0
      let entrance = 0

      function applyFog() {
        const p = readPalette()
        scene.fog = new THREE.Fog(p.surface, 6.5, 14)
      }
      applyFog()

      function mountScene(index: number) {
        if (index === currentIndex && handle) return
        if (handle) {
          scene.remove(handle.root)
          handle.dispose()
          handle = null
        }
        const factory = sceneFactories[index] ?? sceneFactories[0]
        handle = factory(readPalette())
        handle.root.scale.setScalar(0.01)
        scene.add(handle.root)
        currentIndex = index
        elapsed = reducedMotion ? 1.2 : 0
        entrance = 0
      }

      engine = { mountScene }
      mountScene(activeIndex)

      function resize() {
        if (!host) return
        const w = host.clientWidth
        const h = host.clientHeight
        if (w < 1 || h < 1) return
        camera.aspect = w / h
        camera.updateProjectionMatrix()
        renderer.setSize(w, h, false)
      }

      const ro = new ResizeObserver(resize)
      ro.observe(host)
      resize()

      const io = new IntersectionObserver(
        ([entry]) => {
          visible = entry.isIntersecting
        },
        { threshold: 0.05 },
      )
      io.observe(host)

      const onMotion = () => {
        reducedMotion = prefersReduced.matches
      }
      prefersReduced.addEventListener('change', onMotion)

      const mediaDark = window.matchMedia('(prefers-color-scheme: dark)')
      const onScheme = () => {
        applyFog()
        const idx = currentIndex
        currentIndex = -1
        mountScene(idx)
      }
      mediaDark.addEventListener('change', onScheme)

      function frame(now: number) {
        if (!running) return
        raf = requestAnimationFrame(frame)
        const dt = Math.min(0.05, (now - last) / 1000)
        last = now

        if (!visible || !handle) return

        if (!reducedMotion) {
          elapsed += dt
          entrance = Math.min(1, entrance + dt * 3.2)
          handle.root.scale.setScalar(entrance)
          handle.update(elapsed, dt)
        } else {
          handle.root.scale.setScalar(1)
          handle.update(1.2, 0)
        }

        renderer.render(scene, camera)
      }
      raf = requestAnimationFrame(frame)

      cleanup = () => {
        running = false
        cancelAnimationFrame(raf)
        engine = null
        ro.disconnect()
        io.disconnect()
        prefersReduced.removeEventListener('change', onMotion)
        mediaDark.removeEventListener('change', onScheme)
        if (handle) {
          scene.remove(handle.root)
          handle.dispose()
        }
        renderer.dispose()
        if (host && renderer.domElement.parentElement === host) {
          host.removeChild(renderer.domElement)
        }
      }
    })()

    return () => {
      disposed = true
      cleanup?.()
    }
  })
</script>

<div class="demo-viz" bind:this={host} role="img" aria-label={label || 'OpenSpeak demonstration'}>
  {#if label}
    <div class="demo-viz-caption" aria-hidden="true">
      <span>{label}</span>
    </div>
  {/if}
</div>

<style>
  .demo-viz {
    position: absolute;
    inset: 0;
    overflow: hidden;
    background: transparent;
  }

  .demo-viz :global(canvas) {
    width: 100% !important;
    height: 100% !important;
  }

  .demo-viz-caption {
    position: absolute;
    top: 16px;
    left: 18px;
    z-index: 2;
    pointer-events: none;
  }

  .demo-viz-caption span {
    display: inline-block;
    padding: 5px 9px;
    border: 1px solid var(--line);
    border-radius: 999px;
    background: color-mix(in srgb, var(--surface) 82%, transparent);
    color: var(--ink-soft);
    font: 500 9px/14px var(--mono);
    letter-spacing: 0.06em;
    text-transform: uppercase;
    backdrop-filter: blur(6px);
  }
</style>
