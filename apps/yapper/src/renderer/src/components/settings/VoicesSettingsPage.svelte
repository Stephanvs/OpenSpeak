<script lang="ts">
  import SettingsSection from './SettingsSection.svelte'
  import SettingsRow from './SettingsRow.svelte'
  import Badge from '../ui/Badge.svelte'
</script>

<SettingsSection
  eyebrow="Voices"
  title="Voice library"
  description="These placeholders show where model controls will land once the sidecar exposes them."
>
  <SettingsRow
    title="Installed voices"
    description="Switch between local voice models here as soon as the engine reports them to the app."
  >
    <Badge slot="actions" tone="secondary">Soon</Badge>
  </SettingsRow>

  <SettingsRow
    title="Default voice"
    description="The launcher currently renders with the speech engine's default profile."
  >
    <Badge slot="actions" tone="outline">System default</Badge>
  </SettingsRow>

  <SettingsRow
    title="Speaking style"
    description="Preset tone controls can live here when voice styles are available."
  >
    <Badge slot="actions" tone="outline">Natural</Badge>
  </SettingsRow>
</SettingsSection>

<SettingsSection
  eyebrow="Preview"
  title="Preview workflow"
  description="Short validation clips still run through the launcher until a dedicated preview player exists."
>
  <SettingsRow
    title="Current flow"
    description="Use the main launcher to generate a quick sample and confirm the result in Finder or Explorer."
  >
    <Badge slot="actions" tone="secondary">Uses launcher</Badge>
  </SettingsRow>
</SettingsSection>
