import { listen } from '@tauri-apps/api/event';

import {
	commands,
	type AppSettings,
	type AudioLevels,
	type ModelDownload,
	type ModelInfo,
	type SessionSnapshot,
	type SessionType,
	type SettingValue,
	type Utterance
} from '$lib/bindings';

type ResultEnvelope<T> =
	| {
			status: 'ok';
			data: T;
	  }
	| {
			status: 'error';
			error: string;
	  };

type SessionStatePayload = { session: SessionSnapshot };
type FinalTranscriptPayload = { utterances: Utterance[] };
type AudioLevelsPayload = { levels: AudioLevels };
type ModelDownloadPayload = { progress: ModelDownload };
type CaptureErrorPayload = { message: string };

export const settingValues = {
	string: (value: string) => ({ kind: 'string', value }) as SettingValue
};

export const api = {
	getSettings: () => commands.getSettings(),
	setSetting: async (key: string, value: SettingValue) =>
		unwrap(await commands.setSetting(key, value)),
	pickFolder: () => commands.pickVaultFolder(),
	listModels: () => commands.listModels(),
	downloadModel: async (modelId: string) => unwrap(await commands.downloadModel(modelId)),
	startVoiceMemo: async () => unwrap(await commands.startVoiceMemo()),
	startMeetingCapture: async () => unwrap(await commands.startMeetingCapture()),
	stopSession: async () => unwrap(await commands.stopSession())
};

export async function bindDesktopEvents(handlers: {
	onSession: (session: SessionSnapshot) => void;
	onTranscript: (utterances: Utterance[]) => void;
	onLevels: (levels: AudioLevels) => void;
	onDownload: (download: ModelDownload) => void;
	onError: (message: string) => void;
}) {
	if (!isTauriEnvironment()) {
		return () => {};
	}

	const disposers = await Promise.all([
		listen<SessionStatePayload>('session-state', (event) =>
			handlers.onSession(event.payload.session)
		),
		listen<FinalTranscriptPayload>('final-transcript', (event) =>
			handlers.onTranscript(event.payload.utterances)
		),
		listen<AudioLevelsPayload>('audio-levels', (event) => handlers.onLevels(event.payload.levels)),
		listen<ModelDownloadPayload>('model-download-progress', (event) =>
			handlers.onDownload(event.payload.progress)
		),
		listen<CaptureErrorPayload>('capture-error', (event) => handlers.onError(event.payload.message))
	]);

	return () => {
		for (const dispose of disposers) {
			dispose();
		}
	};
}

export function isTauriEnvironment() {
	return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

function unwrap<T>(result: ResultEnvelope<T> | T): T {
	if (typeof result === 'object' && result !== null && 'status' in result) {
		if (result.status === 'error') {
			throw new Error(result.error);
		}

		return result.data;
	}

	return result;
}

export type {
	AppSettings,
	AudioLevels,
	ModelDownload,
	ModelInfo,
	SessionSnapshot,
	SessionType,
	Utterance
};
