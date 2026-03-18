<script lang="ts">
  export let type: 'button' | 'submit' | 'reset' = 'button'
  export let variant: 'default' | 'secondary' | 'outline' = 'default'
  export let size: 'sm' | 'md' = 'md'
  export let disabled = false
  export let className = ''
</script>

<button
  class={`button button--${variant} button--${size} ${className}`.trim()}
  {type}
  {disabled}
  on:click
>
  <slot />
</button>
