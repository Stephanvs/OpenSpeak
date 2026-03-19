<script lang="ts">
  export let title = ''
  export let description = ''
  export let align: 'center' | 'start' = 'center'
</script>

<div
  class={`grid gap-4 px-6 py-4 md:grid-cols-[minmax(0,1fr)_auto] ${align === 'start' ? 'items-start' : 'md:items-center'}`}
>
  <div class="space-y-1">
    <h3 class="text-sm font-medium">{title}</h3>

    {#if description}
      <p class="text-sm text-muted-foreground">
        {description}
      </p>
    {/if}
  </div>

  <div class="flex flex-wrap items-center gap-2">
    <slot name="actions" />
  </div>
</div>
