<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { Toaster } from '$lib/components/ui/sonner';
	import * as Sidebar from '$lib/components/ui/sidebar';
	import * as Tooltip from '$lib/components/ui/tooltip';

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<Tooltip.Provider delayDuration={150}>
	<Sidebar.Provider style="--sidebar-width: 16.5rem; --sidebar-width-mobile: 18rem;">
		<Toaster richColors />
		{@render children()}
	</Sidebar.Provider>
</Tooltip.Provider>
