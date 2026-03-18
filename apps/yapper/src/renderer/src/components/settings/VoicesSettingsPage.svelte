<script lang="ts">
  import SettingsSection from './SettingsSection.svelte'
  import SettingsRow from './SettingsRow.svelte'
  import Badge from '../ui/Badge.svelte'
</script>

<SettingsSection
  title="Voices"
>
  <SettingsRow title="Installed voices">
    <Badge slot="actions" tone="secondary">Soon</Badge>
  </SettingsRow>

  <SettingsRow title="Default voice">
    <Badge slot="actions" tone="outline">System default</Badge>
  </SettingsRow>

  <SettingsRow title="Style">
    <Badge slot="actions" tone="outline">Natural</Badge>
  </SettingsRow>
</SettingsSection>

<SettingsSection title="Preview">
  <SettingsRow title="Current flow">
    <Badge slot="actions" tone="secondary">Launcher</Badge>
  </SettingsRow>
</SettingsSection>
