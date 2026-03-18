<script lang="ts">
  import LauncherWindow from './components/LauncherWindow.svelte'
  import SettingsWindow from './components/SettingsWindow.svelte'
  import type { StatusTone } from './types'

  const HOTKEY = 'Meta + Shift + Y'
  const SUBMIT_HOTKEY = 'Cmd/Ctrl + Enter'
  type AppApi = {
    hideWindow: () => Promise<void>
    revealOutput: (outputPath: string) => Promise<void>
    synthesizeText: (text: string) => Promise<string>
  }

  type WindowView = 'launcher' | 'settings'
  const api = window.api as unknown as AppApi
  const windowView: WindowView =
    new URLSearchParams(window.location.search).get('view') === 'settings' ? 'settings' : 'launcher'

  let text = ''
  let isSynthesizing = false
  let statusTone: StatusTone = 'neutral'
  let statusMessage = ''
  let outputPath = ''
  let canSynthesize = false

  $: canSynthesize = text.trim().length > 0 && !isSynthesizing

  async function synthesizeAudio(): Promise<void> {
    if (!canSynthesize) {
      return
    }

    isSynthesizing = true
    statusTone = 'neutral'
    statusMessage = 'Generating audio file...'
    outputPath = ''

    try {
      outputPath = await api.synthesizeText(text.trim())
      statusTone = 'success'
      statusMessage = `Saved audio to ${outputPath}`
    } catch (error) {
      statusTone = 'error'
      statusMessage = error instanceof Error ? error.message : 'Failed to generate audio.'
    } finally {
      isSynthesizing = false
    }
  }

  async function hideWindow(): Promise<void> {
    await api.hideWindow()
  }

  async function revealOutput(): Promise<void> {
    if (!outputPath) {
      return
    }

    await api.revealOutput(outputPath)
  }
</script>

{#if windowView === 'settings'}
  <SettingsWindow hotkey={HOTKEY} submitHotkey={SUBMIT_HOTKEY} onHideWindow={hideWindow} />
{:else}
  <LauncherWindow
    bind:text
    {isSynthesizing}
    {statusTone}
    {statusMessage}
    {outputPath}
    {canSynthesize}
    onHideWindow={hideWindow}
    onSynthesizeAudio={synthesizeAudio}
    onRevealOutput={revealOutput}
  />
{/if}
