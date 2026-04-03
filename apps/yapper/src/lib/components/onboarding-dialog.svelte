<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Card from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Progress } from '$lib/components/ui/progress';
	import type { DetectedApp } from '$lib/bindings';

	let {
		open = $bindable(false),
		meetingFolder,
		voiceFolder,
		shortcut,
		downloadProgress,
		supportedApps,
		onPickMeeting,
		onPickVoice,
		onDownloadModel,
		onSaveShortcut,
		onComplete
	} = $props<{
		open?: boolean;
		meetingFolder: string;
		voiceFolder: string;
		shortcut: string;
		downloadProgress: number | null;
		supportedApps: DetectedApp[];
		onPickMeeting: () => Promise<void> | void;
		onPickVoice: () => Promise<void> | void;
		onDownloadModel: () => Promise<void> | void;
		onSaveShortcut: (shortcut: string) => Promise<void> | void;
		onComplete: () => Promise<void> | void;
	}>();

	let shortcutDraft = $state('');

	$effect(() => {
		shortcutDraft = shortcut;
	});
</script>

<Dialog.Root bind:open>
	<Dialog.Content
		class="max-w-3xl border border-border/70 bg-[#081017]/96 p-0 shadow-[0_32px_80px_rgba(0,0,0,0.45)]"
	>
		<div class="border-b border-border/70 px-6 py-5">
			<p class="text-[0.68rem] tracking-[0.32em] text-primary/80 uppercase">First run</p>
			<Dialog.Header class="pt-3">
				<Dialog.Title class="text-2xl font-semibold tracking-tight"
					>Finish the Windows scaffold setup</Dialog.Title
				>
				<Dialog.Description class="max-w-2xl text-sm leading-6 text-muted-foreground">
					This onboarding keeps the flow minimal: choose folders, download the baseline model, set
					the global shortcut, and confirm the supported app policy.
				</Dialog.Description>
			</Dialog.Header>
		</div>

		<div class="grid gap-4 px-6 py-5 md:grid-cols-2">
			<Card.Root class="border-border/70 bg-card/70">
				<Card.Header>
					<Card.Title class="text-base">1. Vault folders</Card.Title>
					<Card.Description>Keep meetings and voice memos separate from day one.</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-3 text-sm">
					<div>
						<p class="font-medium text-foreground">Meetings</p>
						<p class="mt-1 break-all text-muted-foreground">{meetingFolder}</p>
						<Button class="mt-3 w-full" variant="outline" onclick={onPickMeeting}
							>Choose meetings folder</Button
						>
					</div>
					<div>
						<p class="font-medium text-foreground">Voice memos</p>
						<p class="mt-1 break-all text-muted-foreground">{voiceFolder}</p>
						<Button class="mt-3 w-full" variant="outline" onclick={onPickVoice}
							>Choose voice memo folder</Button
						>
					</div>
				</Card.Content>
			</Card.Root>

			<Card.Root class="border-border/70 bg-card/70">
				<Card.Header>
					<Card.Title class="text-base">2. Baseline model</Card.Title>
					<Card.Description>Parakeet ONNX stays the default CPU-first path.</Card.Description>
				</Card.Header>
				<Card.Content class="space-y-3 text-sm">
					<Button class="w-full" onclick={onDownloadModel}>Download default model</Button>
					{#if downloadProgress !== null}
						<div class="space-y-2">
							<Progress value={downloadProgress} />
							<p class="text-muted-foreground">{downloadProgress.toFixed(0)}% complete</p>
						</div>
					{/if}
				</Card.Content>
			</Card.Root>

			<Card.Root class="border-border/70 bg-card/70 md:col-span-2">
				<Card.Header>
					<Card.Title class="text-base">3. Recording shortcut and supported apps</Card.Title>
					<Card.Description>
						Windows 11+ first. Browser meetings stay out of scope in this scaffold.
					</Card.Description>
				</Card.Header>
				<Card.Content class="grid gap-4 md:grid-cols-[0.9fr_1.1fr]">
					<div class="space-y-3">
						<Input bind:value={shortcutDraft} placeholder="Ctrl+Shift+Space" />
						<Button class="w-full" variant="outline" onclick={() => onSaveShortcut(shortcutDraft)}>
							Save shortcut
						</Button>
					</div>
					<div class="space-y-3 text-sm text-muted-foreground">
						<div class="flex flex-wrap gap-2">
							{#each supportedApps as app (app.kind)}
								<Badge variant="outline">{app.displayName}</Badge>
							{/each}
							<Badge variant="outline">Browser meetings not yet supported</Badge>
						</div>
						<p>
							Microphone privacy still needs Windows approval, and process-targeted loopback remains
							a later phase after the full-system loopback spike.
						</p>
					</div>
				</Card.Content>
			</Card.Root>
		</div>

		<Dialog.Footer class="border-t border-border/70 px-6 py-4">
			<Button variant="outline" onclick={() => (open = false)}>Review later</Button>
			<Button onclick={onComplete}>Finish setup</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
