<script lang="ts">
  import { afterUpdate } from 'svelte'

  import { Badge } from '$lib/components/ui/badge/index.js'
  import { Button } from '$lib/components/ui/button/index.js'
  import * as Kbd from '$lib/components/ui/kbd/index.js'
  import { Separator } from '$lib/components/ui/separator/index.js'
  import { Spinner } from '$lib/components/ui/spinner/index.js'
  import { Textarea } from '$lib/components/ui/textarea/index.js'

  import type { StatusTone } from '../types'

  const statusMetaByTone: Record<
    StatusTone,
    { label: string; variant: 'secondary' | 'outline' | 'destructive' }
  > = {
    neutral: { label: 'Ready', variant: 'secondary' },
    success: { label: 'Saved', variant: 'outline' },
    error: { label: 'Problem', variant: 'destructive' }
  }

  const composerMinHeight = 132
  const composerMaxHeight = 184

  export let text = ''
  export let isSynthesizing = false
  export let statusTone: StatusTone = 'neutral'
  export let statusMessage = ''
  export let outputPath = ''
  export let canSynthesize = false
  export let onSynthesizeAudio: () => void | Promise<void> = () => undefined
  export let onRevealOutput: () => void | Promise<void> = () => undefined

  let textareaElement: HTMLTextAreaElement | null = null
  let composerFocused = false

  $: statusMeta = statusMetaByTone[statusTone]
  $: trimmedText = text.trim()
  $: wordCount = trimmedText ? trimmedText.split(/\s+/).length : 0
  $: characterCount = text.length
  $: outputText = outputPath || 'No output yet.'

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

<section class="grid h-full w-full min-h-0 grid-rows-[auto_minmax(0,1fr)_auto] gap-4">
  <div class="space-y-2">
    <div class="flex flex-wrap items-center justify-between gap-2">
      <Badge variant="secondary">Speech Studio</Badge>
      <Kbd.Root>Cmd/Ctrl + Enter</Kbd.Root>
    </div>

    <div class="space-y-1">
      <h1 class="text-2xl font-semibold tracking-tight">Text to speech</h1>
      <p class="text-sm text-muted-foreground">
        Paste your text, keep the red focus bar in view, and generate the latest audio export from
        the launcher.
      </p>
    </div>
  </div>

  <form
    aria-busy={isSynthesizing}
    class="flex h-full min-h-0 flex-col gap-4"
    on:focusin={() => {
      composerFocused = true
    }}
    on:focusout={() => {
      composerFocused = false
    }}
    on:submit|preventDefault={() => void onSynthesizeAudio()}
  >
    <div class="flex min-h-0 flex-1 flex-col gap-3">
      <div
        aria-hidden="true"
        class={`h-1 w-full rounded-full bg-destructive transition-opacity ${composerFocused ? 'opacity-100' : 'opacity-30'}`}
      ></div>

      <Textarea
        aria-label="Text to synthesize"
        bind:ref={textareaElement}
        bind:value={text}
        class="h-full resize-none"
        id="tts-input"
        rows="7"
        placeholder="Paste a script, paragraph, or rough notes to turn into audio..."
        spellcheck="false"
      />
    </div>

    <Separator />

    <div class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
      <div class="space-y-2">
        <div class="flex flex-wrap items-center gap-2">
          <Badge variant={statusMeta.variant}>{statusMeta.label}</Badge>
          <span class="text-sm text-muted-foreground">{wordCount} words</span>
          <span class="text-sm text-muted-foreground">{characterCount} chars</span>
        </div>

        <p class="text-sm text-muted-foreground">
          {statusMessage ||
            'Line breaks stay intact so narration rhythm survives the trip from draft to export.'}
        </p>
      </div>

      <Button type="submit" disabled={!canSynthesize}>
        {#if isSynthesizing}
          <Spinner aria-label="Generating audio" />
        {/if}
        {isSynthesizing ? 'Generating audio' : 'Generate audio'}
      </Button>
    </div>
  </form>

  <div class="space-y-4">
    <div class="flex flex-wrap items-center justify-between gap-2">
      <div class="space-y-1">
        <h2 class="text-lg font-semibold tracking-tight">Output</h2>
        <p class="text-sm text-muted-foreground">Where the latest render is saved.</p>
      </div>
      <Badge variant={outputPath ? 'secondary' : 'outline'}>
        {outputPath ? 'Saved' : 'Waiting'}
      </Badge>
    </div>

    <Textarea aria-label="Latest output path" readonly rows="3" value={outputText} />

    <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
      <p class="text-sm text-muted-foreground">
        {outputPath
          ? 'Use Reveal to open the folder that contains the exported audio file.'
          : 'The save path appears here after you generate audio.'}
      </p>

      <Button variant="outline" onclick={() => void onRevealOutput()} disabled={!outputPath}>
        Reveal
      </Button>
    </div>
  </div>
</section>
