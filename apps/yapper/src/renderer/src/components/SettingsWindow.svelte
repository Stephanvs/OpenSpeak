<script lang="ts">
  import { onMount } from 'svelte'

  import Badge from './ui/Badge.svelte'
  import Kbd from './ui/Kbd.svelte'
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
    { id: 'general', label: 'General', icon: 'GEN' },
    { id: 'voices', label: 'Voices', icon: 'VOC' },
    { id: 'history', label: 'History', icon: 'HIS' },
    { id: 'advanced', label: 'Advanced', icon: 'ADV' },
    { id: 'about', label: 'About', icon: 'APP' }
  ]

  export let hotkey = ''
  export let submitHotkey = ''
  export let onHideWindow: () => void | Promise<void> = () => undefined

  let activePageId: SettingsPageId = defaultPageId

  $: activePage = navItems.find((item) => item.id === activePageId) ?? navItems[0]

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

<main class="settings-shell">
  <aside class="settings-sidebar">
    <div class="settings-sidebar-card surface-card">
      <div class="settings-brand">
        <div class="settings-logo" aria-hidden="true">Y</div>
        <div class="settings-brand-copy">
          <p class="section-eyebrow">Yapper desktop</p>
          <h1 class="settings-brand-title">Settings</h1>
        </div>
      </div>
    </div>

    <nav class="settings-nav settings-sidebar-card surface-card" aria-label="Settings sections">
      {#each navItems as item (item.id)}
        <a
          class:is-active={item.id === activePageId}
          class="settings-nav-item"
          href={`#${item.id}`}
          aria-current={item.id === activePageId ? 'page' : undefined}
        >
          <span class="settings-nav-icon" aria-hidden="true">{item.icon}</span>
          <span class="settings-nav-copy">
            <span class="settings-nav-label">{item.label}</span>
          </span>
        </a>
      {/each}
    </nav>

    <div class="settings-sidebar-card surface-card">
      <div>
        <p class="section-eyebrow">Quick keys</p>
        <div class="settings-shortcut-list">
          <div class="settings-shortcut-row">
            <Kbd>{hotkey}</Kbd>
            <span>Reveal launcher</span>
          </div>
          <div class="settings-shortcut-row">
            <Kbd>Esc</Kbd>
            <span>Hide settings</span>
          </div>
        </div>
      </div>
      <Badge tone="secondary">Yapper TTS</Badge>
    </div>
  </aside>

  <section class="settings-content">
    <div class="settings-content-scroll">
      <header class="settings-page-header surface-card">
        <div class="section-copy">
          <h2 class="settings-page-title">{activePage.label}</h2>
        </div>
      </header>

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
