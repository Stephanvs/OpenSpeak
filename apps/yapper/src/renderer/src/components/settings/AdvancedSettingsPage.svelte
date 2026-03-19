<script lang="ts">
  import { Badge } from '$lib/components/ui/badge/index.js'
  import { Separator } from '$lib/components/ui/separator/index.js'

  import SettingsSection from './SettingsSection.svelte'
  import SettingsRow from './SettingsRow.svelte'
</script>

<div class="space-y-4">
  <SettingsSection
    eyebrow="Pipeline"
    title="Advanced"
    description="Reserved space for power-user controls such as naming templates, output destinations, and debugging visibility."
  >
    <SettingsRow
      title="Output naming"
      description="Filename templates will eventually let you stamp renders with structure instead of relying on the default export pattern."
    >
      <Badge slot="actions" variant="outline">Planned</Badge>
    </SettingsRow>
    <Separator />
    <SettingsRow
      title="Output folder"
      description="Yapper currently writes to the app's default temporary audio path to keep the launcher frictionless."
    >
      <Badge slot="actions" variant="outline">Default app path</Badge>
    </SettingsRow>
    <Separator />
    <SettingsRow
      title="Diagnostics"
      description="Detailed render traces stay hidden right now while the desktop flow remains intentionally minimal."
    >
      <Badge slot="actions" variant="secondary">Hidden</Badge>
    </SettingsRow>
  </SettingsSection>
</div>
