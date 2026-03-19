<script lang="ts">
  import { Badge } from '$lib/components/ui/badge/index.js'
  import { Separator } from '$lib/components/ui/separator/index.js'

  import SettingsSection from './SettingsSection.svelte'
  import SettingsRow from './SettingsRow.svelte'
</script>

<div class="space-y-4">
  <SettingsSection
    eyebrow="Library"
    title="History"
    description="A placeholder home for recent renders, reusable takes, and the housekeeping controls that will eventually support them."
  >
    <SettingsRow
      title="Recent renders"
      description="Recent export tracking is not surfaced yet, so the timeline remains intentionally empty for now."
    >
      <Badge slot="actions" variant="secondary">Empty</Badge>
    </SettingsRow>
    <Separator />
    <SettingsRow
      title="Saved location"
      description="When history lands, the newest file path will anchor the list so you can jump back into the last session fast."
    >
      <Badge slot="actions" variant="outline">Latest file</Badge>
    </SettingsRow>
  </SettingsSection>

  <SettingsSection
    eyebrow="Cleanup"
    title="Retention"
    description="Future cleanup rules will help keep temporary audio exports tidy without interrupting your immediate workflow."
  >
    <SettingsRow
      title="Retention policy"
      description="Automatic pruning is still planned, which means generated files currently remain available until you remove them yourself."
    >
      <Badge slot="actions" variant="outline">Planned</Badge>
    </SettingsRow>
  </SettingsSection>
</div>
