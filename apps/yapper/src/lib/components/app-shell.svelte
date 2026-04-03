<script lang="ts">
	import { onMount } from 'svelte';
	import CheckCircle2Icon from '@lucide/svelte/icons/check-circle-2';
	import FolderOpenIcon from '@lucide/svelte/icons/folder-open';
	import LoaderCircleIcon from '@lucide/svelte/icons/loader-circle';
	import MicIcon from '@lucide/svelte/icons/mic';
	import NotebookPenIcon from '@lucide/svelte/icons/notebook-pen';
	import SquareIcon from '@lucide/svelte/icons/square';
	import WaveformIcon from '@lucide/svelte/icons/waves';
	import { openPath } from '@tauri-apps/plugin-opener';
	import { api, bindDesktopEvents, isTauriEnvironment, settingValues } from '$lib/desktop';
	import type {
		AppSettings,
		AudioLevels,
		ModelInfo,
		SessionSnapshot,
		Utterance
	} from '$lib/desktop';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { Progress } from '$lib/components/ui/progress';

	let settings = $state<AppSettings | null>(null);
	let models = $state<ModelInfo[]>([]);
	let session = $state<SessionSnapshot | null>(null);
	let levels = $state<AudioLevels>({ microphone: 0, system: 0 });
	let transcript = $state<Utterance[]>([]);
	let loading = $state(true);
	let pendingFolder: 'meeting' | 'voice' | null = $state(null);
	let actionError = $state<string | null>(null);
	let actionSuccess = $state<string | null>(null);

	const recorderBusy = $derived(
		session?.state === 'recording' || session?.state === 'stopping' ? session.state : null
	);
	const activeModel = $derived(models.find((model) => model.active) ?? models[0] ?? null);
	const notePath = $derived(session?.notePath ?? null);

	onMount(() => {
		let dispose = () => {};

		void (async () => {
			if (!isTauriEnvironment()) {
				loading = false;
				actionError = 'Run Yapper inside the desktop app to record audio.';
				return;
			}

			dispose = await bindDesktopEvents({
				onSession(nextSession) {
					session = nextSession;
				},
				onTranscript(utterances) {
					transcript = utterances;
				},
				onLevels(nextLevels) {
					levels = nextLevels;
				},
				onDownload() {},
				onError(message) {
					actionError = message;
				}
			});

			await refresh();
		})();

		return () => {
			dispose();
		};
	});

	async function refresh() {
		loading = true;
		actionError = null;

		try {
			const [nextSettings, nextModels] = await Promise.all([api.getSettings(), api.listModels()]);
			settings = nextSettings;
			models = nextModels;
			if (!session) {
				session = {
					sessionType: nextSettings.general.defaultSessionType,
					state: 'idle',
					sourceApp: null,
					lockedTarget: null,
					statusMessage: 'Ready',
					startedAt: null,
					notePath: null,
					livePreview: []
				};
			}
		} catch (cause) {
			actionError = toMessage(cause);
		} finally {
			loading = false;
		}
	}

	async function chooseFolder(key: 'general.meetingVaultFolder' | 'general.voiceMemoFolder') {
		pendingFolder = key === 'general.meetingVaultFolder' ? 'meeting' : 'voice';
		actionError = null;
		actionSuccess = null;

		try {
			const selected = await api.pickFolder();
			if (!selected) {
				return;
			}

			settings = await api.setSetting(key, settingValues.string(selected));
			actionSuccess = 'Output folder updated.';
		} catch (cause) {
			actionError = toMessage(cause);
		} finally {
			pendingFolder = null;
		}
	}

	async function startVoiceMemo() {
		actionError = null;
		actionSuccess = null;
		transcript = [];

		try {
			session = await api.startVoiceMemo();
		} catch (cause) {
			actionError = toMessage(cause);
		}
	}

	async function startMeetingNote() {
		actionError = null;
		actionSuccess = null;
		transcript = [];

		try {
			session = await api.startMeetingCapture();
		} catch (cause) {
			actionError = toMessage(cause);
		}
	}

	async function stopRecording() {
		actionError = null;

		try {
			session = await api.stopSession();
			actionSuccess = session.notePath ? 'Transcript saved.' : 'Recording stopped.';
		} catch (cause) {
			actionError = toMessage(cause);
		}
	}

	async function openNote() {
		if (!notePath) {
			return;
		}

		try {
			await openPath(notePath);
		} catch (cause) {
			actionError = toMessage(cause);
		}
	}

	function toMessage(cause: unknown) {
		if (cause instanceof Error) {
			return cause.message;
		}

		return String(cause);
	}
</script>

<div class="mx-auto flex min-h-screen w-full max-w-6xl items-start px-5 py-10">
	<div class="w-full space-y-6">
		<Card.Root
			class="overflow-hidden border-border/70"
		>
			<Card.Header class="gap-4">
				<div class="flex flex-wrap items-center gap-2">
					{#if activeModel}
						<Badge variant="outline">{activeModel.name}</Badge>
					{/if}
				</div>
				<Card.Title class="text-3xl font-semibold tracking-tight">OpenSpeak</Card.Title>
				<Card.Description class="max-w-3xl text-sm leading-6 text-muted-foreground">
					Record a voice memo or a meeting note, transcribe the microphone locally, and save a
					Tome-style markdown note into your chosen folder.
				</Card.Description>
			</Card.Header>
		</Card.Root>

		{#if loading}
			<Card.Root class="border-border/70 bg-card/85">
				<Card.Content class="flex items-center gap-3 py-8 text-sm text-muted-foreground">
					<LoaderCircleIcon class="size-4 animate-spin" />
					Loading desktop state...
				</Card.Content>
			</Card.Root>
		{:else}
			<div class="grid gap-6 lg:grid-cols-[0.95fr_1.05fr]">
				<Card.Root class="border-border/70 bg-card/85">
					<Card.Header>
						<Card.Title class="text-xl">Output folders</Card.Title>
						<Card.Description>
							Meeting notes and voice memos are written directly to these locations.
						</Card.Description>
					</Card.Header>
					<Card.Content class="space-y-4">
						<div class="rounded-2xl border border-border/70 bg-background/60 p-4">
							<div class="flex items-start justify-between gap-3">
								<div class="space-y-2">
									<p class="text-sm font-medium text-foreground">Meeting notes</p>
									<p class="text-sm leading-6 break-all text-muted-foreground">
										{settings?.general.meetingVaultFolder}
									</p>
								</div>
								<Button
									variant="outline"
									onclick={() => chooseFolder('general.meetingVaultFolder')}
									disabled={pendingFolder !== null || recorderBusy !== null}
								>
									<FolderOpenIcon class="size-4" />
									{pendingFolder === 'meeting' ? 'Choosing...' : 'Choose'}
								</Button>
							</div>
						</div>

						<div class="rounded-2xl border border-border/70 bg-background/60 p-4">
							<div class="flex items-start justify-between gap-3">
								<div class="space-y-2">
									<p class="text-sm font-medium text-foreground">Voice memos</p>
									<p class="text-sm leading-6 break-all text-muted-foreground">
										{settings?.general.voiceMemoFolder}
									</p>
								</div>
								<Button
									variant="outline"
									onclick={() => chooseFolder('general.voiceMemoFolder')}
									disabled={pendingFolder !== null || recorderBusy !== null}
								>
									<FolderOpenIcon class="size-4" />
									{pendingFolder === 'voice' ? 'Choosing...' : 'Choose'}
								</Button>
							</div>
						</div>
					</Card.Content>
				</Card.Root>

				<Card.Root class="border-border/70 bg-card/85">
					<Card.Header>
						<Card.Title class="text-xl">Recorder</Card.Title>
						<Card.Description>
							Voice memos and meeting notes are transcribed from the microphone and saved when you
							stop the recording.
						</Card.Description>
					</Card.Header>
					<Card.Content class="space-y-5">
						<div class="flex flex-wrap items-center gap-2">
							<Badge variant="outline">{activeModel?.name ?? 'Recognizer unavailable'}</Badge>
							<Badge variant="outline">{session?.state ?? 'idle'}</Badge>
						</div>

						<div class="grid gap-3 sm:grid-cols-3">
							<Button onclick={startVoiceMemo} disabled={recorderBusy !== null}>
								<MicIcon class="size-4" />
								Record voice memo
							</Button>
							<Button
								variant="secondary"
								onclick={startMeetingNote}
								disabled={recorderBusy !== null}
							>
								<NotebookPenIcon class="size-4" />
								Record meeting note
							</Button>
							<Button
								variant="outline"
								onclick={stopRecording}
								disabled={recorderBusy !== 'recording'}
							>
								<SquareIcon class="size-4" />
								Stop
							</Button>
						</div>

						<div class="rounded-2xl border border-border/70 bg-background/60 p-4">
							<div class="flex items-center gap-2 text-sm font-medium text-foreground">
								<WaveformIcon class="size-4 text-primary" />
								Microphone level
							</div>
							<Progress class="mt-4" value={levels.microphone * 100} />
							<p class="mt-2 text-sm text-muted-foreground">
								{session?.statusMessage ?? 'Ready'}
							</p>
						</div>
					</Card.Content>
				</Card.Root>
			</div>

			{#if actionSuccess}
				<Card.Root class="border-emerald-500/30 bg-emerald-500/10">
					<Card.Content class="flex items-center gap-3 py-4 text-sm text-emerald-100">
						<CheckCircle2Icon class="size-4" />
						{actionSuccess}
					</Card.Content>
				</Card.Root>
			{/if}

			{#if actionError}
				<Card.Root class="border-amber-500/30 bg-amber-500/10">
					<Card.Content class="py-4 text-sm leading-6 text-amber-100">{actionError}</Card.Content>
				</Card.Root>
			{/if}
		{/if}
	</div>
</div>
