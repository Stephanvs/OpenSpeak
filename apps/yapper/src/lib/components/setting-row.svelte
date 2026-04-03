<script lang="ts">
	import type { Snippet } from 'svelte';
	import InfoIcon from '@lucide/svelte/icons/info';

	let {
		title,
		description,
		tip = '',
		children
	} = $props<{
		title: string;
		description: string;
		tip?: string;
		children?: Snippet;
	}>();
</script>

<div class="flex flex-col gap-3 px-4 py-3 md:flex-row md:items-center md:justify-between">
	<div class="max-w-2xl">
		<div class="flex items-center gap-2">
			<h3 class="text-sm font-medium text-foreground">{title}</h3>
			{#if tip}
				<span
					title={tip}
					class="inline-flex size-5 items-center justify-center rounded-full border border-border/80 bg-muted/40 text-muted-foreground"
				>
					<InfoIcon class="size-3.5" />
				</span>
			{/if}
		</div>
		<p class="mt-1 text-sm leading-6 text-muted-foreground">{description}</p>
	</div>
	<div class="max-w-full min-w-[220px] md:w-[300px]">
		{@render children?.()}
	</div>
</div>
