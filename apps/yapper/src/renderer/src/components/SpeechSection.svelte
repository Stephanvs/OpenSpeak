<script lang="ts">
  import { afterUpdate } from 'svelte'

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

  const composerMinHeight = 172
  const composerMaxHeight = 236

  let textareaElement: HTMLTextAreaElement | null = null

  $: statusMeta = statusMetaByTone[statusTone]
  $: trimmedText = text.trim()
  $: wordCount = trimmedText ? trimmedText.split(/\s+/).length : 0
  $: characterCount = text.length

  afterUpdate(() => {
    syncTextareaHeight()
  })

  function syncTextareaHeight(): void {
    if (!textareaElement) {
      return
    }

    textareaElement.style.height = '0px'

    const nextHeight = Math.min(
      Math.max(textareaElement.scrollHeight, composerMinHeight),
      composerMaxHeight
    )

    textareaElement.style.height = `${nextHeight}px`
    textareaElement.style.overflowY =
      textareaElement.scrollHeight > composerMaxHeight ? 'auto' : 'hidden'
  }
</script>

<section class="launcher-frame surface-card">
  <div class="launcher-grid">
    <form
      class="launcher-composer surface-card surface-card--flat"
      aria-busy={isSynthesizing}
      on:submit|preventDefault={() => void onSynthesizeAudio()}
    >
      <div class="launcher-composer-shell">
        <div class="launcher-composer-glow launcher-composer-glow--left" aria-hidden="true"></div>
        <div class="launcher-composer-glow launcher-composer-glow--right" aria-hidden="true"></div>

        <div class="launcher-composer-editor">
          <textarea
            aria-label="Text to synthesize"
            class="launcher-textarea"
            id="tts-input"
            bind:this={textareaElement}
            bind:value={text}
            rows="1"
            placeholder="Paste a script, paragraph, or rough notes to turn into audio..."
            spellcheck="false"
          ></textarea>
        </div>

        <div class="launcher-composer-toolbar">
          <div class="launcher-composer-toolbar-left">
            <div class="launcher-toolbar-pill launcher-toolbar-pill--primary">
              <span class="launcher-toolbar-pill-icon" aria-hidden="true">
                <svg viewBox="0 0 16 16" fill="none">
                  <path
                    d="M5.25 3.25a2.75 2.75 0 1 1 5.5 0v3.5a2.75 2.75 0 1 1-5.5 0v-3.5Z"
                    stroke="currentColor"
                    stroke-width="1.4"
                  ></path>
                  <path
                    d="M3.5 7.25a4.5 4.5 0 1 0 9 0M8 11.75v2.5M5.75 14.25h4.5"
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-width="1.4"
                  ></path>
                </svg>
              </span>
              <span>Speech Studio</span>
            </div>

            <div class="launcher-toolbar-hint">
              <span>Quick render</span>
              <span class="launcher-toolbar-shortcut">Cmd/Ctrl + Enter</span>
            </div>
          </div>

          <div class="launcher-composer-toolbar-right">
            <div class="launcher-composer-metrics" aria-label="Input metrics">
              <span>{wordCount} words</span>
              <span aria-hidden="true">/</span>
              <span>{characterCount} chars</span>
            </div>

            <button
              class="launcher-send-button"
              type="submit"
              disabled={!canSynthesize}
              aria-label={isSynthesizing ? 'Generating audio' : 'Generate audio'}
            >
              {#if isSynthesizing}
                <span class="launcher-send-spinner" aria-hidden="true"></span>
              {:else}
                <svg viewBox="0 0 16 16" fill="none" aria-hidden="true">
                  <path
                    d="M8 1.5L8 11.25M8 1.5L4.5 5M8 1.5L11.5 5"
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="1.5"
                  ></path>
                  <path
                    d="M3 14.5H13"
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-width="1.5"
                  ></path>
                </svg>
              {/if}
            </button>
          </div>
        </div>
      </div>

      <div class="launcher-composer-footer">
        <div class="launcher-status-line launcher-status-line--composer">
          <Badge tone={statusMeta.tone}>{statusMeta.label}</Badge>
          <p class="launcher-status-message">
            {statusMessage ||
              'Paste anything with pauses and line breaks preserved. Yapper renders the cleaned text into a fresh audio file.'}
          </p>
        </div>
      </div>
    </form>

    <div class="launcher-card launcher-output-card surface-card surface-card--flat">
      <div class="launcher-card-head launcher-card-head--output">
        <div class="launcher-card-copy">
          <h2 class="launcher-card-title">Output</h2>
          <p class="launcher-card-caption">Where the latest render lands</p>
        </div>

        <div class="launcher-card-meta">
          <Badge tone={outputPath ? 'success' : 'outline'}>
            {outputPath ? 'Saved' : 'Waiting'}
          </Badge>
          <Button
            variant="outline"
            size="sm"
            on:click={() => void onRevealOutput()}
            disabled={!outputPath}
          >
            Reveal
          </Button>
        </div>
      </div>

      <div class:is-empty={!outputPath} class="launcher-output-path">
        {outputPath ? outputPath : 'No output yet.'}
      </div>

      <p class="launcher-output-note">
        {outputPath
          ? 'Use Reveal to open the folder that contains the exported audio file.'
          : 'The save path appears here after you generate audio.'}
      </p>
    </div>
  </div>
</section>
