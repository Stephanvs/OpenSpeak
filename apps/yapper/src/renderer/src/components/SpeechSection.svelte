<script lang="ts">
  import Badge from './ui/Badge.svelte'
  import Button from './ui/Button.svelte'

  import type { StatusTone } from '../types'

  export let text = ''
  export let isSynthesizing = false
  export let statusTone: StatusTone = 'neutral'
  export let statusMessage = ''
  export let outputPath = ''
  export let canSynthesize = false
  export let onSynthesizeAudio: () => void | Promise<void> = () => undefined
  export let onRevealOutput: () => void | Promise<void> = () => undefined

  const statusMetaByTone = {
    neutral: { label: 'Ready', tone: 'secondary' as const },
    success: { label: 'Saved', tone: 'success' as const },
    error: { label: 'Problem', tone: 'danger' as const }
  }

  $: statusMeta = statusMetaByTone[statusTone]
  $: characterCount = text.length
</script>

<section class="launcher-frame surface-card">
  <div class="launcher-grid">
    <div class="launcher-card launcher-card--editor surface-card surface-card--flat">
      <div class="launcher-card-head">
        <h1 id="tts-input-title" class="launcher-card-title">Text</h1>
        <Badge tone="outline">{characterCount} chars</Badge>
      </div>

      <textarea
        aria-labelledby="tts-input-title"
        class="launcher-textarea"
        id="tts-input"
        bind:value={text}
        rows="8"
        placeholder="Enter text..."
      ></textarea>
    </div>

    <div class="launcher-card surface-card surface-card--flat">
      <div class="launcher-card-head">
        <h2 class="launcher-card-title">Output</h2>
        <Badge tone={outputPath ? 'success' : 'outline'}>
          {outputPath ? 'Saved' : 'Empty'}
        </Badge>
      </div>

      <div class:is-empty={!outputPath} class="launcher-output-path">
        {outputPath ? outputPath : 'No output yet.'}
      </div>

      <div class="launcher-status-line">
        <Badge tone={statusMeta.tone}>{statusMeta.label}</Badge>
        {#if statusMessage}
          <p class="launcher-status-message">{statusMessage}</p>
        {/if}
      </div>
    </div>
  </div>

  <div class="launcher-actions">
    <div class="launcher-action-row">
      <Button on:click={() => void onSynthesizeAudio()} disabled={!canSynthesize}>
        {isSynthesizing ? 'Generating...' : 'Generate'}
      </Button>
      <Button
        variant="outline"
        on:click={() => void onRevealOutput()}
        disabled={!outputPath}
      >
        Reveal
      </Button>
    </div>
  </div>
</section>
