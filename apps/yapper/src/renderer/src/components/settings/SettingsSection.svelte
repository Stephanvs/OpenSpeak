<script lang="ts">
  export let eyebrow = ''
  export let title = ''
  export let description = ''
</script>

<section class="settings-section">
  <div class="section-heading">
    {#if eyebrow}
      <p class="section-eyebrow">{eyebrow}</p>
    {/if}

    {#if title || description}
      <div class="section-copy">
        {#if title}
          <h2 class="section-title">{title}</h2>
        {/if}

        {#if description}
          <p class="section-description">{description}</p>
        {/if}
      </div>
    {/if}
  </div>

  <div class="settings-card surface-card surface-card--flat">
    <slot />
  </div>
</section>
