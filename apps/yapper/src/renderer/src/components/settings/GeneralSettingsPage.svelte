<script lang="ts">
  export let hotkey = ''
  export let onHideWindow: () => void | Promise<void> = () => undefined
</script>

<section class="settings-group">
  <p class="group-label">General</p>
  <div class="group-card">
    <div class="setting-row">
      <div>
        <h2>Launcher Shortcut</h2>
        <p>Bring the launcher forward from anywhere.</p>
      </div>
      <div class="row-actions">
        <span class="keycap">{hotkey}</span>
      </div>
    </div>

    <div class="setting-row">
      <div>
        <h2>Window</h2>
        <p>Hide this window quickly when you are done.</p>
      </div>
      <div class="row-actions">
        <button class="ghost-button" type="button" on:click={() => void onHideWindow()}>Hide</button
        >
      </div>
    </div>
  </div>
</section>
