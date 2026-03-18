<script lang="ts">
  import SettingsSection from './SettingsSection.svelte'
  import SettingsRow from './SettingsRow.svelte'
  import Badge from '../ui/Badge.svelte'
</script>

<SettingsSection
  title="Advanced"
>
  <SettingsRow title="Output naming">
    <Badge slot="actions" tone="outline">Planned</Badge>
  </SettingsRow>

  <SettingsRow title="Output folder">
    <Badge slot="actions" tone="outline">Default app path</Badge>
  </SettingsRow>

  <SettingsRow title="Diagnostics">
    <Badge slot="actions" tone="secondary">Hidden</Badge>
  </SettingsRow>
</SettingsSection>
