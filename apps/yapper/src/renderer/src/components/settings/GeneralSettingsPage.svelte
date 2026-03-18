<script lang="ts">
  import SettingsSection from './SettingsSection.svelte'
  import SettingsRow from './SettingsRow.svelte'
  import Button from '../ui/Button.svelte'
  import Kbd from '../ui/Kbd.svelte'

  export let hotkey = ''
  export let onHideWindow: () => void | Promise<void> = () => undefined
</script>

<SettingsSection
  eyebrow="General"
  title="Desktop behavior"
  description="Keep the launcher close at hand and dismiss the settings window just as quickly."
>
  <SettingsRow
    title="Launcher shortcut"
    description="Bring the text-to-speech launcher forward from anywhere on the desktop."
  >
    <Kbd slot="actions">{hotkey}</Kbd>
  </SettingsRow>

  <SettingsRow
    title="Window controls"
    description="Close this settings view instantly when you are done making adjustments."
  >
    <div slot="actions" class="settings-action-cluster">
      <Kbd>Esc</Kbd>
      <Button variant="outline" size="sm" on:click={() => void onHideWindow()}>Hide window</Button>
    </div>
  </SettingsRow>
</SettingsSection>
