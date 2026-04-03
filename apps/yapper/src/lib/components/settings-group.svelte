<script lang="ts">
	import type { Snippet } from 'svelte';
	import * as Card from '$lib/components/ui/card';

	let {
		title,
		description = '',
		children
	} = $props<{
		title: string;
		description?: string;
		children?: Snippet;
	}>();
</script>

<section class="space-y-3">
	<div class="px-1">
		<p class="text-[0.68rem] font-semibold tracking-[0.28em] text-muted-foreground/90 uppercase">
			{title}
		</p>
		{#if description}
			<p class="mt-2 max-w-2xl text-sm leading-6 text-muted-foreground">{description}</p>
		{/if}
	</div>
	<Card.Root
		class="overflow-hidden border-border/70 bg-card/80 shadow-[0_18px_40px_rgba(0,0,0,0.18)] backdrop-blur"
	>
		<Card.Content class="divide-y divide-border/70 p-0">
			{@render children?.()}
		</Card.Content>
	</Card.Root>
</section>
