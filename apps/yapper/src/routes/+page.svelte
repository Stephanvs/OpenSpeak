<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { openPath } from "@tauri-apps/plugin-opener";

  const HOTKEY = "Cmd/Ctrl + Shift + Y";
  const SUBMIT_HOTKEY = "Cmd/Ctrl + Enter";

  let text = $state("");
  let isSynthesizing = $state(false);
  let statusTone = $state<"neutral" | "success" | "error">("neutral");
  let statusMessage = $state(`Toggle this launcher with ${HOTKEY}.`);
  let outputPath = $state("");

  const canSynthesize = $derived(text.trim().length > 0 && !isSynthesizing);

  async function synthesizeAudio() {
    if (!canSynthesize) {
      return;
    }

    isSynthesizing = true;
    statusTone = "neutral";
    statusMessage = "Generating audio file...";
    outputPath = "";

    try {
      outputPath = await invoke<string>("synthesize_text", { text: text.trim() });
      statusTone = "success";
      statusMessage = `Saved audio to ${outputPath}`;
    } catch (error) {
      statusTone = "error";
      statusMessage = typeof error === "string" ? error : "Failed to generate audio.";
    } finally {
      isSynthesizing = false;
    }
  }

  async function hideLauncher() {
    await getCurrentWindow().hide();
  }

  async function revealOutput() {
    if (!outputPath) {
      return;
    }

    await openPath(outputPath);
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

<svelte:window onkeydown={handleWindowKeydown} />

<main class="shell">
  <section class="panel">
    <div class="titlebar">
      <div class="titlebar-drag" data-tauri-drag-region>
        <span class="titlebar-dot" data-tauri-drag-region aria-hidden="true"></span>
        <span data-tauri-drag-region>Yapper launcher</span>
      </div>
      <button class="window-action" type="button" onclick={hideLauncher}>Hide</button>
    </div>

    <div class="content">
      <p class="eyebrow">Global text-to-speech launcher</p>
      <h1>Yapper</h1>
      <p class="hint">Open or hide this popup with <kbd>{HOTKEY}</kbd>.</p>

      <label for="tts-input">Text to convert</label>
      <textarea
        id="tts-input"
        bind:value={text}
        rows="8"
        placeholder="Paste or type text you want converted to speech..."
      ></textarea>

      <div class="actions">
        <button class="primary" onclick={synthesizeAudio} disabled={!canSynthesize}>
          {isSynthesizing ? "Generating audio..." : "Generate audio file"}
        </button>
        {#if outputPath}
          <button class="secondary" onclick={revealOutput}>Reveal output</button>
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
  :global(body) {
    margin: 0;
    min-height: 100vh;
    background:
      radial-gradient(circle at 15% 15%, rgba(216, 183, 255, 0.3) 0, rgba(216, 183, 255, 0) 45%),
      radial-gradient(circle at 85% 20%, rgba(121, 204, 255, 0.32) 0, rgba(121, 204, 255, 0) 40%),
      linear-gradient(160deg, #f6f2ea 0%, #e8f0f7 55%, #f2ece1 100%);
    font-family: "Space Grotesk", "Avenir Next", "Segoe UI", sans-serif;
    color: #1f2732;
  }

  .shell {
    height: 100vh;
    display: grid;
    place-items: center;
    padding: 1rem;
    box-sizing: border-box;
    overflow: hidden;
  }

  .panel {
    width: min(100%, 520px);
    max-height: calc(100vh - 2rem);
    border-radius: 22px;
    background: rgba(250, 252, 255, 0.9);
    border: 1px solid rgba(44, 57, 75, 0.16);
    box-shadow:
      0 18px 36px rgba(48, 66, 94, 0.22),
      0 2px 12px rgba(48, 66, 94, 0.12);
    overflow: hidden;
    backdrop-filter: blur(6px);
    display: grid;
    grid-template-rows: auto minmax(0, 1fr);
    animation: panel-enter 220ms ease-out;
  }

  .titlebar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    padding: 0.85rem 1rem 0.75rem;
    border-bottom: 1px solid rgba(44, 57, 75, 0.12);
    background: linear-gradient(180deg, rgba(255, 255, 255, 0.58), rgba(255, 255, 255, 0.2));
  }

  .titlebar-drag {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
    flex: 1;
    font-size: 0.76rem;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: #4a5d78;
    user-select: none;
    cursor: grab;
  }

  .titlebar-drag:active {
    cursor: grabbing;
  }

  .titlebar-dot {
    width: 0.7rem;
    height: 0.7rem;
    border-radius: 999px;
    background: linear-gradient(135deg, #315991 0%, #79ccff 100%);
    box-shadow: 0 0 0 4px rgba(80, 124, 184, 0.12);
    flex: 0 0 auto;
  }

  .window-action {
    padding: 0.34rem 0.78rem;
    border-radius: 999px;
    border: 1px solid rgba(53, 73, 99, 0.22);
    background: rgba(247, 249, 253, 0.92);
    color: #2f3b4c;
    font-size: 0.72rem;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }

  .content {
    display: grid;
    gap: 0.8rem;
    padding: 1.05rem 1.2rem 1.2rem;
    min-height: 0;
    overflow-y: auto;
    overscroll-behavior: contain;
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
    .shell {
      padding: 0.75rem;
    }

    .panel {
      border-radius: 16px;
      max-height: calc(100vh - 1.5rem);
    }

    .titlebar {
      padding: 0.75rem 0.85rem 0.7rem;
    }

    .content {
      padding: 0.95rem 1rem 1rem;
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
