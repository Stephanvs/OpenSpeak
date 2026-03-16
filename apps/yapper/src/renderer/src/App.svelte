<script lang="ts">
  const HOTKEY = "Cmd/Ctrl + Shift + Y";
  const SUBMIT_HOTKEY = "Cmd/Ctrl + Enter";

  type YapperApi = {
    hideLauncher: () => Promise<void>;
    revealOutput: (outputPath: string) => Promise<void>;
    synthesizeText: (text: string) => Promise<string>;
  };

  const api = window.api as YapperApi;

  let text = "";
  let isSynthesizing = false;
  let statusTone: "neutral" | "success" | "error" = "neutral";
  let statusMessage = `Toggle this launcher with ${HOTKEY}.`;
  let outputPath = "";
  let canSynthesize = false;

  $: canSynthesize = text.trim().length > 0 && !isSynthesizing;

  async function synthesizeAudio() {
    if (!canSynthesize) {
      return;
    }

    isSynthesizing = true;
    statusTone = "neutral";
    statusMessage = "Generating audio file...";
    outputPath = "";

    try {
      outputPath = await api.synthesizeText(text.trim());
      statusTone = "success";
      statusMessage = `Saved audio to ${outputPath}`;
    } catch (error) {
      statusTone = "error";
      statusMessage = error instanceof Error ? error.message : "Failed to generate audio.";
    } finally {
      isSynthesizing = false;
    }
  }

  async function hideLauncher() {
    await api.hideLauncher();
  }

  async function revealOutput() {
    if (!outputPath) {
      return;
    }

    await api.revealOutput(outputPath);
  }

  function handleWindowKeydown(event: KeyboardEvent) {
    if ((event.metaKey || event.ctrlKey) && event.key === "Enter") {
      event.preventDefault();
      void synthesizeAudio();
      return;
    }

    if (event.key === "Escape") {
      event.preventDefault();
      void hideLauncher();
    }
  }
</script>

<svelte:window on:keydown={handleWindowKeydown} />

<main class="shell">
  <section class="panel">
    <div class="content">
      <div class="content-header">
        <div class="content-copy">
          <p class="eyebrow">Global text-to-speech launcher</p>
          <h1>Yapper</h1>
        </div>
        <button class="window-action" type="button" on:click={hideLauncher}>Hide</button>
      </div>

      <p class="hint">Open or hide this popup with <kbd>{HOTKEY}</kbd>.</p>

      <label for="tts-input">Text to convert</label>
      <textarea
        id="tts-input"
        bind:value={text}
        rows="8"
        placeholder="Paste or type text you want converted to speech..."
      ></textarea>

      <div class="actions">
        <button class="primary" on:click={synthesizeAudio} disabled={!canSynthesize}>
          {isSynthesizing ? "Generating audio..." : "Generate audio file"}
        </button>
        {#if outputPath}
          <button class="secondary" on:click={revealOutput}>Reveal output</button>
        {/if}
      </div>

      <p class="status" class:success={statusTone === "success"} class:error={statusTone === "error"}>
        {statusMessage}
      </p>
      <p class="shortcut-note">Use <kbd>{SUBMIT_HOTKEY}</kbd> to generate and <kbd>Esc</kbd> to hide.</p>
    </div>
  </section>
</main>

<style>
  :global(html) {
    height: 100%;
  }

  :global(body) {
    margin: 0;
    height: 100%;
    min-height: 100%;
    background:
      radial-gradient(circle at 15% 15%, rgba(216, 183, 255, 0.26) 0, rgba(216, 183, 255, 0) 45%),
      radial-gradient(circle at 85% 20%, rgba(121, 204, 255, 0.28) 0, rgba(121, 204, 255, 0) 40%),
      linear-gradient(160deg, #f6f2ea 0%, #e8f0f7 55%, #f2ece1 100%);
    font-family: "Space Grotesk", "Avenir Next", "Segoe UI", sans-serif;
    color: #1f2732;
  }

  .shell {
    width: 100vw;
    height: 100vh;
    display: grid;
    box-sizing: border-box;
    overflow: hidden;
  }

  .panel {
    width: 100%;
    height: 100%;
    border-radius: 0;
    background:
      radial-gradient(circle at 15% 15%, rgba(216, 183, 255, 0.26) 0, rgba(216, 183, 255, 0) 45%),
      radial-gradient(circle at 85% 20%, rgba(121, 204, 255, 0.28) 0, rgba(121, 204, 255, 0) 40%),
      linear-gradient(160deg, rgba(246, 242, 234, 0.96) 0%, rgba(232, 240, 247, 0.96) 55%, rgba(242, 236, 225, 0.96) 100%);
    box-shadow: inset 0 0 0 1px rgba(44, 57, 75, 0.14);
    overflow: hidden;
    backdrop-filter: blur(6px);
    display: grid;
    animation: panel-enter 220ms ease-out;
  }

  .content {
    display: grid;
    gap: 0.9rem;
    padding: 1.2rem;
    min-height: 0;
    overflow-y: auto;
    overscroll-behavior: contain;
  }

  .content-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }

  .content-copy {
    min-width: 0;
    display: grid;
    gap: 0.3rem;
  }

  .window-action {
    flex: 0 0 auto;
    padding: 0.34rem 0.78rem;
    border-radius: 999px;
    border: 1px solid rgba(53, 73, 99, 0.22);
    background: rgba(247, 249, 253, 0.92);
    color: #2f3b4c;
    font-size: 0.72rem;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }

  .eyebrow {
    margin: 0;
    font-size: 0.78rem;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: #4a5d78;
  }

  h1 {
    margin: 0;
    font-size: 1.65rem;
    line-height: 1.15;
    font-family: "Sora", "Space Grotesk", sans-serif;
  }

  .hint,
  .shortcut-note,
  .status {
    margin: 0;
    font-size: 0.86rem;
    line-height: 1.4;
    color: #4a5d78;
  }

  label {
    font-size: 0.84rem;
    font-weight: 600;
    color: #2f3b4c;
  }

  textarea {
    width: 100%;
    box-sizing: border-box;
    resize: vertical;
    min-height: 140px;
    max-height: 260px;
    border-radius: 14px;
    border: 1px solid rgba(53, 73, 99, 0.25);
    background: #fbfcfe;
    color: #1f2732;
    padding: 0.78rem 0.82rem;
    font-size: 0.95rem;
    line-height: 1.45;
    font-family: "IBM Plex Sans", "Space Grotesk", sans-serif;
  }

  textarea:focus-visible {
    outline: 2px solid rgba(80, 122, 191, 0.42);
    outline-offset: 2px;
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.55rem;
  }

  button {
    border-radius: 999px;
    border: 1px solid transparent;
    padding: 0.52rem 0.95rem;
    font: inherit;
    font-weight: 600;
    font-size: 0.83rem;
    cursor: pointer;
    transition: transform 120ms ease, filter 120ms ease;
  }

  button:hover:not(:disabled) {
    transform: translateY(-1px);
    filter: brightness(1.03);
  }

  button:disabled {
    cursor: not-allowed;
    opacity: 0.65;
  }

  .primary {
    background: linear-gradient(120deg, #315991 0%, #507cb8 100%);
    color: #f6fbff;
    box-shadow: 0 6px 14px rgba(53, 82, 124, 0.28);
  }

  .secondary {
    border-color: rgba(53, 73, 99, 0.3);
    background: #f7f9fd;
    color: #2f3b4c;
  }

  .status.success {
    color: #1f6b46;
  }

  .status.error {
    color: #8a2633;
  }

  kbd {
    border-radius: 6px;
    border: 1px solid rgba(43, 55, 72, 0.2);
    padding: 0.1rem 0.35rem;
    background: rgba(255, 255, 255, 0.75);
    font-size: 0.74rem;
    font-family: "IBM Plex Mono", "Menlo", monospace;
  }

  @media (max-width: 640px) {
    .panel {
      box-shadow: inset 0 0 0 1px rgba(44, 57, 75, 0.12);
    }

    .content {
      padding: 1rem;
    }

    .content-header {
      align-items: flex-start;
    }

    textarea {
      min-height: 120px;
    }
  }

  @keyframes panel-enter {
    from {
      opacity: 0;
      transform: translateY(8px) scale(0.985);
    }

    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }
</style>
