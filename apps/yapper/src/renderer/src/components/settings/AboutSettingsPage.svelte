<script lang="ts">
  export let hotkey = ''
  export let submitHotkey = ''

  const versions = window.electron.process.versions
</script>

<section class="settings-group">
  <p class="group-label">About</p>
  <div class="group-card">
    <div class="setting-row">
      <div>
        <h2>App</h2>
        <p>Yapper pairs a quick text-to-speech launcher with a dedicated settings window.</p>
      </div>
      <div class="row-actions">
        <span class="keycap">Desktop utility</span>
      </div>
    </div>

    <div class="setting-row">
      <div>
        <h2>Launcher Shortcut</h2>
        <p>Open the launcher from anywhere without leaving the keyboard.</p>
      </div>
      <div class="row-actions">
        <span class="keycap">{hotkey}</span>
      </div>
    </div>

    <div class="setting-row">
      <div>
        <h2>Generate Shortcut</h2>
        <p>Submit the current draft from the launcher without reaching for the mouse.</p>
      </div>
      <div class="row-actions">
        <span class="keycap">{submitHotkey}</span>
      </div>
    </div>
  </div>
</section>

<section class="settings-group">
  <p class="group-label">Runtime</p>
  <div class="group-card">
    <div class="setting-row">
      <div>
        <h2>Electron</h2>
        <p>The desktop shell and native integrations.</p>
      </div>
      <div class="row-actions">
        <span class="keycap">v{versions.electron}</span>
      </div>
    </div>

    <div class="setting-row">
      <div>
        <h2>Chromium</h2>
        <p>The browser engine rendering the interface.</p>
      </div>
      <div class="row-actions">
        <span class="keycap">v{versions.chrome}</span>
      </div>
    </div>

    <div class="setting-row">
      <div>
        <h2>Node</h2>
        <p>The runtime backing the Electron process and local tooling.</p>
      </div>
      <div class="row-actions">
        <span class="keycap">v{versions.node}</span>
      </div>
    </div>
  </div>
</section>
