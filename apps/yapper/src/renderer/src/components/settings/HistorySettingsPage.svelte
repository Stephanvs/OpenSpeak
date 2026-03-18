<script lang="ts">
  import SettingsSection from './SettingsSection.svelte'
  import SettingsRow from './SettingsRow.svelte'
  import Badge from '../ui/Badge.svelte'
</script>

<SettingsSection
  eyebrow="History"
  title="Recent renders"
  description="History becomes more useful once the launcher persists exports beyond the latest clip."
>
  <SettingsRow
    title="Render timeline"
    description="Generated clips will show up here once the launcher stores a list of recent exports."
  >
    <Badge slot="actions" tone="secondary">Empty</Badge>
  </SettingsRow>

  <SettingsRow
    title="Saved location"
    description="For now, the launcher only keeps track of the most recent output path after each synthesis run."
  >
    <Badge slot="actions" tone="outline">Latest file only</Badge>
  </SettingsRow>
</SettingsSection>

<SettingsSection
  eyebrow="Cleanup"
  title="Retention"
  description="Archive and delete controls can plug into this section when history management is implemented."
>
  <SettingsRow
    title="Lifecycle rules"
    description="Automatic cleanup settings will live here once export retention has a configurable policy."
  >
    <Badge slot="actions" tone="outline">Planned</Badge>
  </SettingsRow>
</SettingsSection>
