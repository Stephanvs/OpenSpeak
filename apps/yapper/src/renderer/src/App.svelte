<script lang="ts">
  import { onMount } from "svelte";

  import AboutSettingsPage from "./components/settings/AboutSettingsPage.svelte";
  import AdvancedSettingsPage from "./components/settings/AdvancedSettingsPage.svelte";
  import GeneralSettingsPage from "./components/settings/GeneralSettingsPage.svelte";
  import HistorySettingsPage from "./components/settings/HistorySettingsPage.svelte";
  import VoicesSettingsPage from "./components/settings/VoicesSettingsPage.svelte";

  const HOTKEY = "Cmd/Ctrl + Shift + Y";
  const SUBMIT_HOTKEY = "Cmd/Ctrl + Enter";
  const pageIds = ["general", "voices", "history", "advanced", "about"] as const;

  type SettingsPageId = (typeof pageIds)[number];
  type NavItem = {
    id: SettingsPageId;
    label: string;
    icon: string;
  };
  type StatusTone = "neutral" | "success" | "error";

  const api = window.api;
  const defaultPageId: SettingsPageId = "general";
  const navItems: NavItem[] = [
    { id: "general", label: "General", icon: "G" },
    { id: "voices", label: "Voices", icon: "V" },
    { id: "history", label: "History", icon: "H" },
    { id: "advanced", label: "Advanced", icon: "A" },
    { id: "about", label: "About", icon: "i" }
  ];

  let activePageId: SettingsPageId = defaultPageId;
  let text = "";
  let isSynthesizing = false;
  let statusTone: StatusTone = "neutral";
  let statusMessage = `Toggle this launcher with ${HOTKEY}.`;
  let outputPath = "";
  let canSynthesize = false;

  $: canSynthesize = text.trim().length > 0 && !isSynthesizing;

  function isSettingsPageId(value: string): value is SettingsPageId {
    return pageIds.includes(value as SettingsPageId);
  }

  function readPageIdFromHash(hash: string): SettingsPageId {
    const candidate = hash.replace(/^#/, "");
    return isSettingsPageId(candidate) ? candidate : defaultPageId;
  }

  onMount(() => {
    const syncActivePage = () => {
      activePageId = readPageIdFromHash(window.location.hash);
    };

    const normalizedPageId = readPageIdFromHash(window.location.hash);

    if (window.location.hash !== `#${normalizedPageId}`) {
      window.location.hash = normalizedPageId;
    } else {
      activePageId = normalizedPageId;
    }

    window.addEventListener("hashchange", syncActivePage);

    return () => {
      window.removeEventListener("hashchange", syncActivePage);
    };
  });

  async function synthesizeAudio() {
    if (!canSynthesize) {
      return;
    }

    isSynthesizing = true;
    statusTone = "neutral";
    statusMessage = "Generating audio file...";
    outputPath = "";

    try {
      outputPath = await api.synthesizeText(text.trim());
      statusTone = "success";
      statusMessage = `Saved audio to ${outputPath}`;
    } catch (error) {
      statusTone = "error";
      statusMessage = error instanceof Error ? error.message : "Failed to generate audio.";
    } finally {
      isSynthesizing = false;
    }
  }

  async function hideLauncher() {
    await api.hideLauncher();
  }

  async function revealOutput() {
    if (!outputPath) {
      return;
    }

    await api.revealOutput(outputPath);
  }

  function handleWindowKeydown(event: KeyboardEvent) {
    if (activePageId === "general" && (event.metaKey || event.ctrlKey) && event.key === "Enter") {
      event.preventDefault();
      void synthesizeAudio();
      return;
    }

    if (event.key === "Escape") {
      event.preventDefault();
      void hideLauncher();
    }
  }
</script>

<svelte:window on:keydown={handleWindowKeydown} />

<main class="app-shell">
  <aside class="sidebar">
    <div class="brand-block">
      <div class="logo-placeholder" aria-hidden="true">Y</div>
      <div class="brand-copy">
        <p class="brand-kicker">Text-to-speech launcher</p>
        <h1>Yapper</h1>
      </div>
    </div>

    <nav class="nav-list" aria-label="Settings sections">
      {#each navItems as item}
        <a
          class:active={item.id === activePageId}
          class="nav-item"
          href={`#${item.id}`}
          aria-current={item.id === activePageId ? "page" : undefined}
        >
          <span class="nav-icon" aria-hidden="true">{item.icon}</span>
          <span>{item.label}</span>
        </a>
      {/each}
    </nav>

    <div class="sidebar-footer">
      <div>
        <p class="footer-label">Engine</p>
        <p class="footer-value">Yapper TTS</p>
      </div>
      <span class="footer-chip">Desktop</span>
    </div>
  </aside>

  <section class="content-pane">
    <div class="content-scroll">
      {#if activePageId === "general"}
        <GeneralSettingsPage
          bind:text={text}
          hotkey={HOTKEY}
          submitHotkey={SUBMIT_HOTKEY}
          {isSynthesizing}
          {statusTone}
          {statusMessage}
          {outputPath}
          {canSynthesize}
          onHideLauncher={hideLauncher}
          onSynthesizeAudio={synthesizeAudio}
          onRevealOutput={revealOutput}
        />
      {:else if activePageId === "voices"}
        <VoicesSettingsPage />
      {:else if activePageId === "history"}
        <HistorySettingsPage />
      {:else if activePageId === "advanced"}
        <AdvancedSettingsPage />
      {:else}
        <AboutSettingsPage hotkey={HOTKEY} submitHotkey={SUBMIT_HOTKEY} />
      {/if}
    </div>
  </section>
</main>
