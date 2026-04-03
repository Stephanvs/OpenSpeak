import { writable } from 'svelte/store';

import type {
	AppSettings,
	AudioLevels,
	DetectedApp,
	ModelDownload,
	ModelInfo,
	SessionSnapshot,
	Utterance,
	UpdateInfo
} from '$lib/bindings';

export type SectionId = 'general' | 'models' | 'advanced' | 'about';

type SettingsStoreState = {
	loading: boolean;
	settings: AppSettings | null;
	supportedApps: DetectedApp[];
	detectedApp: DetectedApp | null;
	error: string | null;
};

type SessionStoreState = {
	session: SessionSnapshot | null;
	liveTranscript: Utterance[];
	finalTranscript: Utterance[];
	error: string | null;
};

type ModelStoreState = {
	loading: boolean;
	models: ModelInfo[];
	download: ModelDownload | null;
	error: string | null;
};

type AudioStoreState = {
	levels: AudioLevels;
};

type UpdatesStoreState = {
	loading: boolean;
	update: UpdateInfo | null;
	error: string | null;
};

export const settingsStore = writable<SettingsStoreState>({
	loading: true,
	settings: null,
	supportedApps: [],
	detectedApp: null,
	error: null
});

export const sessionStore = writable<SessionStoreState>({
	session: null,
	liveTranscript: [],
	finalTranscript: [],
	error: null
});

export const modelStore = writable<ModelStoreState>({
	loading: true,
	models: [],
	download: null,
	error: null
});

export const audioStore = writable<AudioStoreState>({
	levels: {
		microphone: 0,
		system: 0
	}
});

export const updatesStore = writable<UpdatesStoreState>({
	loading: true,
	update: null,
	error: null
});

export function applySettings(settings: AppSettings) {
	settingsStore.update((current) => ({ ...current, loading: false, settings, error: null }));
}

export function setSupportedApps(supportedApps: DetectedApp[]) {
	settingsStore.update((current) => ({ ...current, supportedApps }));
}

export function setDetectedApp(detectedApp: DetectedApp | null) {
	settingsStore.update((current) => ({ ...current, detectedApp }));
}

export function setSettingsError(error: string) {
	settingsStore.update((current) => ({ ...current, loading: false, error }));
}

export function applySession(session: SessionSnapshot) {
	sessionStore.update((current) => ({
		...current,
		session,
		liveTranscript: session.livePreview,
		error: null
	}));
}

export function appendPartialTranscript(utterance: Utterance) {
	sessionStore.update((current) => ({
		...current,
		liveTranscript: dedupeUtterances([...current.liveTranscript, utterance])
	}));
}

export function setFinalTranscript(utterances: Utterance[]) {
	sessionStore.update((current) => ({
		...current,
		finalTranscript: utterances,
		liveTranscript: utterances
	}));
}

export function setSessionError(error: string) {
	sessionStore.update((current) => ({ ...current, error }));
}

export function applyModels(models: ModelInfo[]) {
	modelStore.update((current) => ({ ...current, loading: false, models, error: null }));
}

export function applyDownload(download: ModelDownload | null) {
	modelStore.update((current) => ({ ...current, download }));
}

export function setModelError(error: string) {
	modelStore.update((current) => ({ ...current, loading: false, error }));
}

export function applyAudioLevels(levels: AudioLevels) {
	audioStore.set({ levels });
}

export function applyUpdate(update: UpdateInfo) {
	updatesStore.update((current) => ({ ...current, loading: false, update, error: null }));
}

export function setUpdateError(error: string) {
	updatesStore.update((current) => ({ ...current, loading: false, error }));
}

function dedupeUtterances(utterances: Utterance[]) {
	const seen = new Set<string>();
	return utterances.filter((utterance) => {
		const key = `${utterance.timestamp}-${utterance.speaker.label}-${utterance.text}`;
		if (seen.has(key)) {
			return false;
		}
		seen.add(key);
		return true;
	});
}
