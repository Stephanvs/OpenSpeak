<script lang="ts">
  import SpeechSection from './SpeechSection.svelte'
  import type { StatusTone } from '../types'

  export let text = ''
  export let isSynthesizing = false
  export let statusTone: StatusTone = 'neutral'
  export let statusMessage = ''
  export let outputPath = ''
  export let canSynthesize = false
  export let onHideWindow: () => void | Promise<void> = () => undefined
  export let onSynthesizeAudio: () => void | Promise<void> = () => undefined
  export let onRevealOutput: () => void | Promise<void> = () => undefined

  function handleWindowKeydown(event: KeyboardEvent): void {
    if ((event.metaKey || event.ctrlKey) && event.key === 'Enter') {
      event.preventDefault()
      void onSynthesizeAudio()
      return
    }

    if (event.key === 'Escape') {
      event.preventDefault()
      void onHideWindow()
    }
  }
</script>

<svelte:window on:keydown={handleWindowKeydown} />

<main class="h-full w-full">
  <SpeechSection
    bind:text
    {isSynthesizing}
    {statusTone}
    {statusMessage}
    {outputPath}
    {canSynthesize}
    {onSynthesizeAudio}
    {onRevealOutput}
  />
</main>
