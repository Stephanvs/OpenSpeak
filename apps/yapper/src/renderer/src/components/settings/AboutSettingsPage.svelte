<script lang="ts">
  import { Badge } from '$lib/components/ui/badge/index.js'
  import * as Kbd from '$lib/components/ui/kbd/index.js'
  import { Separator } from '$lib/components/ui/separator/index.js'

  import SettingsSection from './SettingsSection.svelte'
  import SettingsRow from './SettingsRow.svelte'

  export let hotkey = ''
  export let submitHotkey = ''

  const versions = window.electron.process.versions
</script>

<div class="space-y-4">
  <SettingsSection
    eyebrow="Identity"
    title="About"
    description="Yapper is a compact desktop utility for turning loose text into fast speech renders without leaving your current flow."
  >
    <SettingsRow
      title="App role"
      description="A lightweight command-center for quick text-to-speech generation on desktop."
    >
      <Badge slot="actions" variant="secondary">Desktop utility</Badge>
    </SettingsRow>
    <Separator />
    <SettingsRow
      title="Launcher shortcut"
      description="The global key chord that opens the drafting window from anywhere."
    >
      <Kbd.Root slot="actions">{hotkey}</Kbd.Root>
    </SettingsRow>
    <Separator />
    <SettingsRow
      title="Render shortcut"
      description="Use the composer hotkey whenever you want to synthesize without touching the pointer."
    >
      <Kbd.Root slot="actions">{submitHotkey}</Kbd.Root>
    </SettingsRow>
  </SettingsSection>

  <SettingsSection
    eyebrow="Runtime"
    title="Versions"
    description="Current engine versions bundled with the desktop shell."
  >
    <SettingsRow title="Electron" description="Desktop runtime and windowing layer.">
      <Badge slot="actions" variant="outline">v{versions.electron}</Badge>
    </SettingsRow>
    <Separator />
    <SettingsRow title="Chromium" description="Rendering engine powering the UI.">
      <Badge slot="actions" variant="outline">v{versions.chrome}</Badge>
    </SettingsRow>
    <Separator />
    <SettingsRow title="Node" description="Local process runtime for the Electron application.">
      <Badge slot="actions" variant="outline">v{versions.node}</Badge>
    </SettingsRow>
  </SettingsSection>
</div>
