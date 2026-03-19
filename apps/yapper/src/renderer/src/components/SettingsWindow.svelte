<script lang="ts">
  import { onMount } from 'svelte'

  import { Badge } from '$lib/components/ui/badge/index.js'
  import { Button } from '$lib/components/ui/button/index.js'
  import * as Kbd from '$lib/components/ui/kbd/index.js'
  import { ScrollArea } from '$lib/components/ui/scroll-area/index.js'
  import { Separator } from '$lib/components/ui/separator/index.js'

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
    description: string
    eyebrow: string
  }

  const navItems: NavItem[] = [
    {
      id: 'general',
      label: 'General',
      description: 'Shortcuts, launch flow, and the basics.',
      eyebrow: 'Window controls'
    },
    {
      id: 'voices',
      label: 'Voices',
      description: 'Upcoming voice presets and defaults.',
      eyebrow: 'Audio pipeline'
    },
    {
      id: 'history',
      label: 'History',
      description: 'Recent exports and retention plans.',
      eyebrow: 'Render library'
    },
    {
      id: 'advanced',
      label: 'Advanced',
      description: 'Paths, naming, and diagnostics.',
      eyebrow: 'Power tools'
    },
    {
      id: 'about',
      label: 'About',
      description: 'Runtime versions and core shortcuts.',
      eyebrow: 'Application info'
    }
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

  function selectPage(pageId: SettingsPageId): void {
    if (window.location.hash !== `#${pageId}`) {
      window.location.hash = pageId
      return
    }

    activePageId = pageId
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

<main class="grid h-full min-h-0 gap-4 lg:grid-cols-[240px_minmax(0,1fr)]">
  <aside class="flex min-h-0 flex-col gap-4">
    <div class="space-y-2">
      <Badge variant="secondary">Yapper desktop</Badge>
      <div class="space-y-1">
        <h1 class="text-2xl font-semibold tracking-tight">Settings</h1>
        <p class="text-sm text-muted-foreground">
          Adjust the launcher and desktop workflow using the default shadcn component set.
        </p>
      </div>
    </div>

    <Separator />

    <div class="grid gap-2">
      {#each navItems as item (item.id)}
        <Button
          variant={item.id === activePageId ? 'secondary' : 'ghost'}
          class="w-full justify-start"
          onclick={() => selectPage(item.id)}
        >
          {item.label}
        </Button>
      {/each}
    </div>

    <Separator />

    <div class="space-y-3">
      <div class="space-y-1">
        <h2 class="text-sm font-medium">Quick keys</h2>
        <p class="text-sm text-muted-foreground">Shortcuts available from the desktop shell.</p>
      </div>

      <div class="flex items-center justify-between gap-2">
        <span class="text-sm text-muted-foreground">Reveal launcher</span>
        <Kbd.Root>{hotkey}</Kbd.Root>
      </div>

      <div class="flex items-center justify-between gap-2">
        <span class="text-sm text-muted-foreground">Hide settings</span>
        <Kbd.Root>Esc</Kbd.Root>
      </div>
    </div>
  </aside>

  <ScrollArea class="h-full min-h-0">
    <div class="space-y-4">
      <div class="space-y-2">
        <Badge variant="outline">{activePage.eyebrow}</Badge>
        <div class="space-y-1">
          <h2 class="text-xl font-semibold tracking-tight">{activePage.label}</h2>
          <p class="text-sm text-muted-foreground">{activePage.description}</p>
        </div>
      </div>

      <Separator />

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
  </ScrollArea>
</main>
