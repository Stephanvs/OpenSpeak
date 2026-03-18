<script lang="ts">
  import SettingsSection from './SettingsSection.svelte'
  import SettingsRow from './SettingsRow.svelte'
  import Badge from '../ui/Badge.svelte'
  import Kbd from '../ui/Kbd.svelte'

  export let hotkey = ''
  export let submitHotkey = ''

  const versions = window.electron.process.versions
</script>

<SettingsSection
  eyebrow="About"
  title="App overview"
  description="Yapper pairs a quick text-to-speech launcher with a dedicated settings workspace."
>
  <SettingsRow
    title="App role"
    description="Use the launcher for fast synths and the settings window for the rest of the desktop configuration."
  >
    <Badge slot="actions" tone="secondary">Desktop utility</Badge>
  </SettingsRow>

  <SettingsRow
    title="Launcher shortcut"
    description="Open the launcher from anywhere without leaving the keyboard."
  >
    <Kbd slot="actions">{hotkey}</Kbd>
  </SettingsRow>

  <SettingsRow
    title="Generate shortcut"
    description="Submit the current draft from the launcher without reaching for the mouse."
  >
    <Kbd slot="actions">{submitHotkey}</Kbd>
  </SettingsRow>
</SettingsSection>

<SettingsSection
  eyebrow="Runtime"
  title="Desktop stack"
  description="Version details for the shell and runtimes backing the interface."
>
  <SettingsRow title="Electron" description="The desktop shell and native integrations.">
    <Badge slot="actions" tone="outline">v{versions.electron}</Badge>
  </SettingsRow>

  <SettingsRow title="Chromium" description="The browser engine rendering the interface.">
    <Badge slot="actions" tone="outline">v{versions.chrome}</Badge>
  </SettingsRow>

  <SettingsRow title="Node" description="The runtime backing the Electron process and local tooling.">
    <Badge slot="actions" tone="outline">v{versions.node}</Badge>
  </SettingsRow>
</SettingsSection>
