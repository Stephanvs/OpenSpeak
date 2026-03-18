<script lang="ts">
  import { onMount } from 'svelte'

  import AboutSettingsPage from './settings/AboutSettingsPage.svelte'
  import AdvancedSettingsPage from './settings/AdvancedSettingsPage.svelte'
  import GeneralSettingsPage from './settings/GeneralSettingsPage.svelte'
  import HistorySettingsPage from './settings/HistorySettingsPage.svelte'
  import VoicesSettingsPage from './settings/VoicesSettingsPage.svelte'

  const pageIds = ['general', 'voices', 'history', 'advanced', 'about'] as const
  const defaultPageId = 'general'

  type SettingsPageId = (typeof pageIds)[number]
  type NavItem = {
    id: SettingsPageId
    label: string
    icon: string
  }

  const navItems: NavItem[] = [
    { id: 'general', label: 'General', icon: 'G' },
    { id: 'voices', label: 'Voices', icon: 'V' },
    { id: 'history', label: 'History', icon: 'H' },
    { id: 'advanced', label: 'Advanced', icon: 'A' },
    { id: 'about', label: 'About', icon: 'i' }
  ]

  export let hotkey = ''
  export let submitHotkey = ''
  export let onHideWindow: () => void | Promise<void> = () => undefined

  let activePageId: SettingsPageId = defaultPageId

  function isSettingsPageId(value: string): value is SettingsPageId {
    return pageIds.includes(value as SettingsPageId)
  }

  function readPageIdFromHash(hash: string): SettingsPageId {
    const candidate = hash.replace(/^#/, '')
    return isSettingsPageId(candidate) ? candidate : defaultPageId
  }

  onMount(() => {
    const syncActivePage = (): void => {
      activePageId = readPageIdFromHash(window.location.hash)
    }

    const normalizedPageId = readPageIdFromHash(window.location.hash)

    if (window.location.hash !== `#${normalizedPageId}`) {
      window.location.hash = normalizedPageId
    } else {
      activePageId = normalizedPageId
    }

    window.addEventListener('hashchange', syncActivePage)

    return () => {
      window.removeEventListener('hashchange', syncActivePage)
    }
  })

  function handleWindowKeydown(event: KeyboardEvent): void {
    if (event.key === 'Escape') {
      event.preventDefault()
      void onHideWindow()
    }
  }
</script>

<svelte:window on:keydown={handleWindowKeydown} />

<main class="app-shell">
  <aside class="sidebar">
    <div class="brand-block">
      <div class="logo-placeholder" aria-hidden="true">Y</div>
      <div class="brand-copy">
        <p class="brand-kicker">Yapper desktop</p>
        <h1>Settings</h1>
      </div>
    </div>

    <nav class="nav-list" aria-label="Settings sections">
      {#each navItems as item (item.id)}
        <a
          class:active={item.id === activePageId}
          class="nav-item"
          href={`#${item.id}`}
          aria-current={item.id === activePageId ? 'page' : undefined}
        >
          <span class="nav-icon" aria-hidden="true">{item.icon}</span>
          <span>{item.label}</span>
        </a>
      {/each}
    </nav>

    <div class="sidebar-footer">
      <div>
        <p class="footer-label">Engine</p>
        <p class="footer-value">Yapper TTS</p>
      </div>
      <span class="footer-chip">Desktop</span>
    </div>
  </aside>

  <section class="content-pane">
    <div class="content-scroll">
      {#if activePageId === 'general'}
        <GeneralSettingsPage {hotkey} {onHideWindow} />
      {:else if activePageId === 'voices'}
        <VoicesSettingsPage />
      {:else if activePageId === 'history'}
        <HistorySettingsPage />
      {:else if activePageId === 'advanced'}
        <AdvancedSettingsPage />
      {:else}
        <AboutSettingsPage {hotkey} {submitHotkey} />
      {/if}
    </div>
  </section>
</main>
