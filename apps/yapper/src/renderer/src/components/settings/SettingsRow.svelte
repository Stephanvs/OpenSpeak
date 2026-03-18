<script lang="ts">
  export let title = ''
  export let description = ''
  export let align: 'center' | 'start' = 'center'
</script>

<div class={`settings-row settings-row--${align}`}>
  <div class="settings-row-copy">
    <h3 class="settings-row-title">{title}</h3>
    <p class="settings-row-description">{description}</p>
  </div>

  <div class="settings-row-actions">
    <slot name="actions" />
  </div>
</div>
