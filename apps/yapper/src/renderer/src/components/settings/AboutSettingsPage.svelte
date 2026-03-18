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
  title="About"
>
  <SettingsRow title="App">
    <Badge slot="actions" tone="secondary">Desktop utility</Badge>
  </SettingsRow>

  <SettingsRow title="Launcher shortcut">
    <Kbd slot="actions">{hotkey}</Kbd>
  </SettingsRow>

  <SettingsRow title="Generate shortcut">
    <Kbd slot="actions">{submitHotkey}</Kbd>
  </SettingsRow>
</SettingsSection>

<SettingsSection title="Runtime">
  <SettingsRow title="Electron">
    <Badge slot="actions" tone="outline">v{versions.electron}</Badge>
  </SettingsRow>

  <SettingsRow title="Chromium">
    <Badge slot="actions" tone="outline">v{versions.chrome}</Badge>
  </SettingsRow>

  <SettingsRow title="Node">
    <Badge slot="actions" tone="outline">v{versions.node}</Badge>
  </SettingsRow>
</SettingsSection>
