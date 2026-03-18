<script lang="ts">
  export let tone: 'default' | 'secondary' | 'outline' | 'success' | 'danger' = 'default'
  export let className = ''
</script>

<span class={`badge badge--${tone} ${className}`.trim()}>
  <slot />
</span>
