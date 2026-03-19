<script lang="ts">
  import { Badge } from '$lib/components/ui/badge/index.js'
  import { Separator } from '$lib/components/ui/separator/index.js'

  import SettingsSection from './SettingsSection.svelte'
  import SettingsRow from './SettingsRow.svelte'
</script>

<div class="space-y-4">
  <SettingsSection
    eyebrow="Audio"
    title="Voices"
    description="Preview space for the voice controls that will eventually shape tone, cadence, and the default rendering profile."
  >
    <SettingsRow
      title="Installed voices"
      description="Voice discovery is planned for a future pass, so this surface is reserved for the incoming catalog."
    >
      <Badge slot="actions" variant="secondary">Soon</Badge>
    </SettingsRow>
    <Separator />
    <SettingsRow
      title="Default voice"
      description="Today Yapper follows the system default so output stays consistent with the rest of your device setup."
    >
      <Badge slot="actions" variant="outline">System default</Badge>
    </SettingsRow>
    <Separator />
    <SettingsRow
      title="Delivery style"
      description="The current render path is tuned for natural phrasing without asking you to manage extra presets."
    >
      <Badge slot="actions" variant="outline">Natural</Badge>
    </SettingsRow>
  </SettingsSection>

  <SettingsSection
    eyebrow="Preview"
    title="Current flow"
    description="The present voice experience stays focused on the launcher so you can move from paste to playback with minimal friction."
  >
    <SettingsRow
      title="Entry point"
      description="All voice work currently begins inside the launcher composer rather than a dedicated audio lab."
    >
      <Badge slot="actions" variant="secondary">Launcher</Badge>
    </SettingsRow>
  </SettingsSection>
</div>
