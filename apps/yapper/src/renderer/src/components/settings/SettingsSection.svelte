<script lang="ts">
  import { Badge } from '$lib/components/ui/badge/index.js'

  export let eyebrow = ''
  export let title = ''
  export let description = ''
</script>

<section class="space-y-4">
  {#if eyebrow || title || description}
    <div class="space-y-2">
      {#if eyebrow}
        <Badge variant="outline">{eyebrow}</Badge>
      {/if}

      {#if title}
        <h2 class="text-lg font-semibold tracking-tight">{title}</h2>
      {/if}

      {#if description}
        <p class="text-sm text-muted-foreground">{description}</p>
      {/if}
    </div>
  {/if}

  <div>
    <slot />
  </div>
</section>
