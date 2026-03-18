<script lang="ts">
  import SettingsSection from './SettingsSection.svelte'
  import SettingsRow from './SettingsRow.svelte'
  import Badge from '../ui/Badge.svelte'
</script>

<SettingsSection
  eyebrow="Advanced"
  title="Power-user controls"
  description="Output rules and diagnostics stay grouped here so the everyday flow remains uncluttered."
>
  <SettingsRow
    title="Output naming"
    description="Template-based filenames can be added here once export settings become configurable."
  >
    <Badge slot="actions" tone="outline">Planned</Badge>
  </SettingsRow>

  <SettingsRow
    title="Output folder"
    description="Custom save locations can surface here when the desktop workflow needs them."
  >
    <Badge slot="actions" tone="outline">Default app path</Badge>
  </SettingsRow>

  <SettingsRow
    title="Diagnostics"
    description="Engine logging and troubleshooting tools can live in this section when deeper debugging is exposed."
  >
    <Badge slot="actions" tone="secondary">Hidden</Badge>
  </SettingsRow>
</SettingsSection>
