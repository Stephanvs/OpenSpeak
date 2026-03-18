<script lang="ts">
  export let className = ''
</script>

<kbd class={`kbd ${className}`.trim()}>
  <slot />
</kbd>
