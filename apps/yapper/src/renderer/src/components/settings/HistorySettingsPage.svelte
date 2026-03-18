<script lang="ts">
  import SettingsSection from './SettingsSection.svelte'
  import SettingsRow from './SettingsRow.svelte'
  import Badge from '../ui/Badge.svelte'
</script>

<SettingsSection
  title="History"
>
  <SettingsRow title="Recent renders">
    <Badge slot="actions" tone="secondary">Empty</Badge>
  </SettingsRow>

  <SettingsRow title="Saved location">
    <Badge slot="actions" tone="outline">Latest file</Badge>
  </SettingsRow>
</SettingsSection>

<SettingsSection title="Cleanup">
  <SettingsRow title="Retention">
    <Badge slot="actions" tone="outline">Planned</Badge>
  </SettingsRow>
</SettingsSection>
