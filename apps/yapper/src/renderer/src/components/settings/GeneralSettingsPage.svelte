<script lang="ts">
  import SettingsSection from './SettingsSection.svelte'
  import SettingsRow from './SettingsRow.svelte'
  import Button from '../ui/Button.svelte'
  import Kbd from '../ui/Kbd.svelte'

  export let hotkey = ''
  export let onHideWindow: () => void | Promise<void> = () => undefined
</script>

<SettingsSection
  title="General"
>
  <SettingsRow title="Launcher shortcut">
    <Kbd slot="actions">{hotkey}</Kbd>
  </SettingsRow>

  <SettingsRow title="Hide window">
    <div slot="actions" class="settings-action-cluster">
      <Kbd>Esc</Kbd>
      <Button variant="outline" size="sm" on:click={() => void onHideWindow()}>Hide window</Button>
    </div>
  </SettingsRow>
</SettingsSection>
