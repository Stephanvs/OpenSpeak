<script lang="ts">
  import { Badge } from '$lib/components/ui/badge/index.js'
  import { Button } from '$lib/components/ui/button/index.js'
  import * as Kbd from '$lib/components/ui/kbd/index.js'
  import { Separator } from '$lib/components/ui/separator/index.js'

  import SettingsSection from './SettingsSection.svelte'
  import SettingsRow from './SettingsRow.svelte'

  export let hotkey = ''
  export let onHideWindow: () => void | Promise<void> = () => undefined
</script>

<div class="space-y-4">
  <SettingsSection
    eyebrow="Window"
    title="General"
    description="Core desktop controls for opening the launcher, closing it quickly, and keeping the drafting flow crisp."
  >
    <SettingsRow
      title="Launcher shortcut"
      description="This global shortcut reveals Yapper no matter what app you are currently in."
    >
      <Kbd.Root slot="actions">{hotkey}</Kbd.Root>
    </SettingsRow>
    <Separator />
    <SettingsRow
      title="Dismiss quickly"
      description="Hide the settings window instantly when you are done adjusting your workspace."
    >
      <div slot="actions" class="flex flex-wrap items-center gap-2">
        <Kbd.Root>Esc</Kbd.Root>
        <Button variant="outline" size="sm" onclick={() => void onHideWindow()}>Hide window</Button>
      </div>
    </SettingsRow>
    <Separator />
    <SettingsRow
      title="Window posture"
      description="The launcher stays lightweight and intentionally ephemeral so you can get in, render, and get out."
    >
      <Badge slot="actions" variant="outline">Always on top</Badge>
    </SettingsRow>
  </SettingsSection>
</div>
