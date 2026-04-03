<script lang="ts">
	import CircleHelpIcon from '@lucide/svelte/icons/circle-help';
	import CpuIcon from '@lucide/svelte/icons/cpu';
	import Settings2Icon from '@lucide/svelte/icons/settings-2';
	import SlidersHorizontalIcon from '@lucide/svelte/icons/sliders-horizontal';
	import * as Sidebar from '$lib/components/ui/sidebar';
	import type { SectionId } from '$lib/stores';

	let { activeSection, onSelect } = $props<{
		activeSection: SectionId;
		onSelect: (section: SectionId) => void;
	}>();

	const sections = [
		{ id: 'general' as const, label: 'General', icon: Settings2Icon },
		{ id: 'models' as const, label: 'Models', icon: CpuIcon },
		{ id: 'advanced' as const, label: 'Advanced', icon: SlidersHorizontalIcon },
		{ id: 'about' as const, label: 'About', icon: CircleHelpIcon }
	];
</script>

<Sidebar.Root
	collapsible="none"
	class="hidden border-r border-sidebar-border/70 bg-sidebar/90 md:flex"
>
	<Sidebar.Header class="px-3 py-4">
		<div class="rounded-2xl border border-sidebar-border/70 bg-sidebar-accent/40 px-3 py-3">
			<p class="text-[0.65rem] tracking-[0.28em] text-sidebar-foreground/60 uppercase">OpenSpeak</p>
			<h1 class="mt-2 text-lg font-semibold tracking-tight text-sidebar-foreground">Desktop</h1>
		</div>
	</Sidebar.Header>
	<Sidebar.Separator />
	<Sidebar.Content class="px-2 py-3">
		<Sidebar.Group>
			<Sidebar.GroupLabel
				class="px-2 text-sidebar-foreground/55 uppercase"
			>
				Settings
			</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					{#each sections as section (section.id)}
						<Sidebar.MenuItem>
							<Sidebar.MenuButton
								isActive={activeSection === section.id}
								onclick={() => onSelect(section.id)}
							>
								<section.icon class="size-4" />
								<span>{section.label}</span>
							</Sidebar.MenuButton>
						</Sidebar.MenuItem>
					{/each}
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
	<Sidebar.Footer class="px-3 pb-4">
		<div
			class="rounded-2xl border border-sidebar-border/70 bg-sidebar-accent/30 px-3 py-3 text-sm text-sidebar-foreground/70"
		>
			&copy; 2026 Stephanvs
		</div>
	</Sidebar.Footer>
</Sidebar.Root>
