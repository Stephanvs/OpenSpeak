<script lang="ts">
  export let hotkey = "";
  export let submitHotkey = "";
  export let text = "";
  export let isSynthesizing = false;
  export let statusTone: "neutral" | "success" | "error" = "neutral";
  export let statusMessage = "";
  export let outputPath = "";
  export let canSynthesize = false;
  export let onHideLauncher: () => void | Promise<void> = () => undefined;
  export let onSynthesizeAudio: () => void | Promise<void> = () => undefined;
  export let onRevealOutput: () => void | Promise<void> = () => undefined;
</script>

<section class="settings-group">
  <p class="group-label">General</p>
  <div class="group-card">
    <div class="setting-row">
      <div>
        <h2>Launcher Shortcut</h2>
        <p>Bring the window forward from anywhere.</p>
      </div>
      <div class="row-actions">
        <span class="keycap">{hotkey}</span>
      </div>
    </div>

    <div class="setting-row">
      <div>
        <h2>Generate Shortcut</h2>
        <p>Convert the current draft without leaving the keyboard.</p>
      </div>
      <div class="row-actions">
        <span class="keycap">{submitHotkey}</span>
      </div>
    </div>

    <div class="setting-row">
      <div>
        <h2>Window</h2>
        <p>Close the launcher quickly when you are done.</p>
      </div>
      <div class="row-actions">
        <button class="ghost-button" type="button" on:click={() => void onHideLauncher()}>Hide</button>
      </div>
    </div>
  </div>
</section>

<section class="settings-group">
  <p class="group-label">Speech</p>
  <div class="group-card compose-card">
    <div class="setting-row textarea-row">
      <div class="textarea-copy">
        <h2>Text to Convert</h2>
        <p>Paste or type the script you want to render as audio.</p>
      </div>

      <label class="textarea-wrap" for="tts-input">
        <textarea
          id="tts-input"
          bind:value={text}
          rows="8"
          placeholder="Paste or type text you want converted to speech..."
        ></textarea>
      </label>
    </div>

    <div class="setting-row output-row">
      <div>
        <h2>Output</h2>
        <p>{outputPath ? outputPath : "Generated audio files appear here after synthesis."}</p>
      </div>
      <div class="row-actions action-cluster">
        <button class="primary-button" type="button" on:click={() => void onSynthesizeAudio()} disabled={!canSynthesize}>
          {isSynthesizing ? "Generating..." : "Generate audio"}
        </button>
        <button class="ghost-button" type="button" on:click={() => void onRevealOutput()} disabled={!outputPath}>
          Reveal output
        </button>
      </div>
    </div>

    <div class="status-row">
      <p class="status" class:success={statusTone === "success"} class:error={statusTone === "error"}>
        {statusMessage}
      </p>
      <p class="shortcut-note">Press <kbd>{submitHotkey}</kbd> to generate and <kbd>Esc</kbd> to hide.</p>
    </div>
  </div>
</section>
