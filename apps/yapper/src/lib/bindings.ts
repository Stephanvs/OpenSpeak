import { invoke as TAURI_INVOKE } from '@tauri-apps/api/core';

export type Result<T, E> =
	| {
			status: 'ok';
			data: T;
	  }
	| {
			status: 'error';
			error: E;
	  };

export type SessionType = 'voiceMemo' | 'meetingCapture';
export type SessionState = 'idle' | 'preparing' | 'recording' | 'stopping' | 'error';
export type DetectedAppKind = 'teams' | 'slack' | 'zoom';
export type TargetAppMode = 'auto' | 'none' | { manual: DetectedAppKind };
export type DownloadState = 'notStarted' | 'downloading' | 'installed' | 'cancelled' | 'failed';
export type UpdateStatus =
	| 'idle'
	| 'checking'
	| 'available'
	| 'downloading'
	| 'readyToInstall'
	| 'upToDate';

export type Speaker = {
	label: string;
};

export type Utterance = {
	speaker: Speaker;
	text: string;
	timestamp: string;
};

export type DetectedApp = {
	kind: DetectedAppKind;
	displayName: string;
	executableNames: string[];
	executablePath: string | null;
};

export type AudioLevels = {
	microphone: number;
	system: number;
};

export type ModelInfo = {
	id: string;
	name: string;
	description: string;
	sizeMb: number;
	installed: boolean;
	active: boolean;
};

export type ModelDownload = {
	modelId: string;
	progress: number;
	state: DownloadState;
	message: string;
};

export type UpdateInfo = {
	status: UpdateStatus;
	currentVersion: string;
	latestVersion: string | null;
	releaseNotesUrl: string | null;
};

export type SessionSnapshot = {
	sessionType: SessionType;
	state: SessionState;
	sourceApp: DetectedApp | null;
	lockedTarget: DetectedApp | null;
	statusMessage: string;
	startedAt: string | null;
	notePath: string | null;
	livePreview: Utterance[];
};

export type SettingValue =
	| { kind: 'string'; value: string }
	| { kind: 'bool'; value: boolean }
	| { kind: 'u32'; value: number }
	| { kind: 'sessionType'; value: SessionType }
	| { kind: 'targetAppMode'; value: TargetAppMode }
	| { kind: 'optionalApp'; value: DetectedAppKind | null };

export type GeneralSettings = {
	recordingShortcut: string;
	defaultSessionType: SessionType;
	selectedMicrophone: string | null;
	meetingVaultFolder: string;
	voiceMemoFolder: string;
	targetAppMode: TargetAppMode;
	silenceAutoStopSeconds: number;
};

export type ModelSettings = {
	activeModelId: string;
	modelStorageFolder: string;
	preferDirectMl: boolean;
};

export type AdvancedSettings = {
	startHidden: boolean;
	launchAtLogin: boolean;
	trayIcon: boolean;
	checkForUpdates: boolean;
	logFolder: string;
	tempAudioRetentionDays: number;
	manualTargetApp: DetectedAppKind | null;
};

export type AppSettings = {
	schemaVersion: number;
	onboardingComplete: boolean;
	general: GeneralSettings;
	models: ModelSettings;
	advanced: AdvancedSettings;
};

function wrapResult<T>(promise: Promise<T>): Promise<Result<T, string>> {
	return promise
		.then((data) => ({ status: 'ok', data }) as Result<T, string>)
		.catch(
			(error) =>
				({
					status: 'error',
					error: error instanceof Error ? error.message : String(error)
				}) as Result<T, string>
		);
}

export const commands = {
	async getSettings(): Promise<AppSettings> {
		return await TAURI_INVOKE('get_settings');
	},
	async setSetting(key: string, value: SettingValue): Promise<Result<AppSettings, string>> {
		return await wrapResult(TAURI_INVOKE('set_setting', { key, value }));
	},
	async pickVaultFolder(): Promise<string | null> {
		return await TAURI_INVOKE('pick_vault_folder');
	},
	async listModels(): Promise<ModelInfo[]> {
		return await TAURI_INVOKE('list_models');
	},
	async downloadModel(modelId: string): Promise<Result<ModelDownload, string>> {
		return await wrapResult(TAURI_INVOKE('download_model', { modelId }));
	},
	async cancelModelDownload(modelId: string): Promise<Result<null, string>> {
		return await wrapResult(TAURI_INVOKE('cancel_model_download', { modelId }));
	},
	async startVoiceMemo(): Promise<Result<SessionSnapshot, string>> {
		return await wrapResult(TAURI_INVOKE('start_voice_memo'));
	},
	async startMeetingCapture(): Promise<Result<SessionSnapshot, string>> {
		return await wrapResult(TAURI_INVOKE('start_meeting_capture'));
	},
	async stopSession(): Promise<Result<SessionSnapshot, string>> {
		return await wrapResult(TAURI_INVOKE('stop_session'));
	},
	async getSupportedApps(): Promise<DetectedApp[]> {
		return await TAURI_INVOKE('get_supported_apps');
	},
	async getDetectedForegroundApp(): Promise<DetectedApp | null> {
		return await TAURI_INVOKE('get_detected_foreground_app');
	},
	async setTargetAppMode(mode: TargetAppMode): Promise<Result<TargetAppMode, string>> {
		return await wrapResult(TAURI_INVOKE('set_target_app_mode', { mode }));
	},
	async setShortcut(shortcut: string): Promise<Result<string, string>> {
		return await wrapResult(TAURI_INVOKE('set_shortcut', { shortcut }));
	},
	async checkForUpdates(): Promise<UpdateInfo> {
		return await TAURI_INVOKE('check_for_updates');
	},
	async installUpdate(): Promise<Result<UpdateInfo, string>> {
		return await wrapResult(TAURI_INVOKE('install_update'));
	}
};
