<script lang="ts">
  import { onDestroy } from 'svelte';

  const terms: string[] = [
    'quick drafts.',
    'local inference.',
    'background mode.',
    'remote clients.',
  ];

  const waveformHeights: number[] = [24, 46, 33, 61, 40, 73, 45, 58, 31, 52, 28, 43];

  let activeTermIndex = 0;
  let renderedTerm = '';
  let characterCount = 0;
  let isDeleting = false;

  let timer: ReturnType<typeof setTimeout> | undefined;

  function schedule(delay: number) {
    timer = setTimeout(stepTyping, delay);
  }

  function stepTyping() {
    const activeTerm = terms[activeTermIndex];

    if (!isDeleting) {
      characterCount = Math.min(characterCount + 1, activeTerm.length);
      renderedTerm = activeTerm.slice(0, characterCount);

      if (characterCount === activeTerm.length) {
        isDeleting = true;
        schedule(1400);
        return;
      }

      schedule(70);
      return;
    }

    characterCount = Math.max(characterCount - 1, 0);
    renderedTerm = activeTerm.slice(0, characterCount);

    if (characterCount === 0) {
      isDeleting = false;
      activeTermIndex = (activeTermIndex + 1) % terms.length;
      schedule(180);
      return;
    }

    schedule(40);
  }

  schedule(240);

  onDestroy(() => {
    if (timer) {
      clearTimeout(timer);
    }
  });
</script>

<div class="page-shell">
  <header class="site-header">
    <div class="site-header-inner">
      <div class="site-header-side"></div>

      <nav class="site-nav" aria-label="Primary">
        <a href="#features">Features</a>
        <a href="#faq">FAQ</a>
        <a href="#direction">Current direction</a>
      </nav>

      <div class="site-header-side site-header-actions">
        <a
          class="header-button"
          href="https://github.com/stephanvs/yapper"
          target="_blank"
          rel="noreferrer"
        >
          GitHub
        </a>
      </div>
    </div>
  </header>

  <main class="site-main" id="overview">
    <div class="hero-mark">
      <span class="brand-lockup" aria-hidden="true">
        <span></span>
        <span></span>
        <span></span>
      </span>
      <h1>yapper</h1>
    </div>

    <p class="tagline">
      The voice stack built for <span class="typed-term">{renderedTerm}</span
      ><span class="typed-cursor" aria-hidden="true"></span>
    </p>

    <p class="subtitle">
      Local-first text to speech with a decoupled client and server architecture. Start
      with a macOS launcher, then grow into background, CLI, and remote workflows.
    </p>

    <div class="hero-actions">
      <a
        class="button-primary"
        href="https://github.com/stephanvs/yapper"
        target="_blank"
        rel="noreferrer"
      >
        View on GitHub
      </a>
      <a class="button-secondary" href="#screenshot">See the launcher</a>
    </div>

    <section class="content-section" id="features">
      <h2 class="section-label">Features</h2>

      <ul class="feature-list">
        <li>
          <span class="bullet">-</span>
          <span><strong>Launcher first:</strong> a macOS client for short, fast text-to-speech turns.</span>
        </li>
        <li>
          <span class="bullet">-</span>
          <span><strong>Local inference:</strong> keep drafts and generation on your own machine.</span>
        </li>
        <li>
          <span class="bullet">-</span>
          <span><strong>Decoupled core:</strong> keep the synthesis layer separate from the UI surface.</span>
        </li>
        <li>
          <span class="bullet">-</span>
          <span><strong>File-based output:</strong> generate audio locally and reveal the result immediately.</span>
        </li>
        <li>
          <span class="bullet">-</span>
          <span><strong>Background ready:</strong> support a standalone service when the launcher is not frontmost.</span>
        </li>
        <li>
          <span class="bullet">-</span>
          <span><strong>CLI and remote next:</strong> the architecture already leaves room for more than one client.</span>
        </li>
        <li>
          <span class="bullet">-</span>
          <span><strong>Minimal surface:</strong> keep the experience focused on text in, audio out.</span>
        </li>
      </ul>
    </section>
  </main>

  <section class="screenshot-shell" id="screenshot" aria-label="Yapper launcher preview">
    <div class="product-shot">
      <div class="product-shot-topbar">
        <div class="traffic-lights" aria-hidden="true">
          <span></span>
          <span></span>
          <span></span>
        </div>

        <div class="topbar-path">Yapper / local-first text to speech</div>
      </div>

      <div class="product-shot-body">
        <aside class="shot-sidebar">
          <div class="shot-branding">
            <div class="shot-logo" aria-hidden="true">
              <span></span>
              <span></span>
              <span></span>
            </div>
            <div>
              <p class="shot-eyebrow">Launcher</p>
              <p class="shot-title">Yapper</p>
            </div>
          </div>

          <div class="shot-nav">
            <div class="shot-nav-item is-active">General</div>
            <div class="shot-nav-item">Voices</div>
            <div class="shot-nav-item">History</div>
            <div class="shot-nav-item">Advanced</div>
          </div>

          <div class="shot-sidebar-note">
            <p class="shot-eyebrow">Current focus</p>
            <p>Menu bar client first.</p>
          </div>
        </aside>

        <div class="shot-content">
          <div class="shot-panel">
            <div class="panel-row">
              <div>
                <p class="shot-eyebrow">Prompt</p>
                <h3>Drop in a paragraph and render it locally.</h3>
              </div>
              <span class="panel-chip">Cmd + Shift + Y</span>
            </div>

            <div class="draft-block">
              <p>
                Record a calm build update and finish with the single next action the team
                should take.
              </p>
            </div>

            <div class="panel-actions">
              <span class="soft-chip">Local inference</span>
              <span class="soft-chip">Finder reveal</span>
              <span class="soft-chip">No web hop</span>
            </div>
          </div>

          <div class="shot-panel">
            <div class="panel-row">
              <div>
                <p class="shot-eyebrow">Output</p>
                <h3>Generated audio lands where you work.</h3>
              </div>
              <span class="panel-chip panel-chip-live">Ready</span>
            </div>

            <div class="waveform" aria-hidden="true">
              {#each waveformHeights as height}
                <span style={`--bar-height: ${height}%`}></span>
              {/each}
            </div>

            <div class="file-row">
              <div>
                <p class="shot-eyebrow">Latest file</p>
                <strong>team-briefing.wav</strong>
              </div>
              <span class="saved-pill">saved locally</span>
            </div>

            <div class="meta-grid">
              <div>
                <p class="shot-eyebrow">Path</p>
                <p>~/Audio/yapper/team-briefing.wav</p>
              </div>
              <div>
                <p class="shot-eyebrow">Next surfaces</p>
                <p>Background mode, CLI, remote calls</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>

  <main class="site-main">
    <section class="content-section" id="faq">
      <h2 class="section-label">FAQ</h2>

      <div class="stacked-copy">
        <div>
          <p class="question">Why local-first text to speech?</p>
          <p>
            It keeps drafts private, removes API latency from the main workflow, and makes
            synthesis feel like a machine capability instead of a request to another
            service.
          </p>
        </div>

        <div>
          <p class="question">What ships first?</p>
          <p>
            The current focus is a macOS menu bar launcher that can generate audio quickly
            and reveal the output file immediately.
          </p>
        </div>

        <div>
          <p class="question">Why split the client from the engine?</p>
          <p>
            A decoupled core keeps the inference layer reusable so the desktop launcher,
            future CLI tooling, and remote callers can all use the same system.
          </p>
        </div>

        <div>
          <p class="question">Does Yapper stop at the desktop app?</p>
          <p>
            No. The repo direction already points toward background, standalone CLI, and
            remote modes built on top of the same engine.
          </p>
        </div>

        <div>
          <p class="question">What should the web site communicate?</p>
          <p>
            That Yapper is small on purpose: one clean surface, one reusable inference
            core, and a clear path to more interfaces later.
          </p>
        </div>
      </div>
    </section>

    <section class="content-section" id="direction">
      <h2 class="section-label">Current direction</h2>

      <ul class="quote-list">
        <li>
          <span class="quote-text">"Make text to speech feel like a local utility, not a destination app."</span>
          <span class="quote-source">- product direction</span>
        </li>
        <li>
          <span class="quote-text">"Start with the menu bar, then expand into background, CLI, and remote modes."</span>
          <span class="quote-source">- repo focus</span>
        </li>
        <li>
          <span class="quote-text">"Keep the synthesis layer reusable so every new client stays thin."</span>
          <span class="quote-source">- architecture principle</span>
        </li>
      </ul>
    </section>

    <div class="bottom-actions">
      <a
        class="button-primary"
        href="https://github.com/stephanvs/yapper"
        target="_blank"
        rel="noreferrer"
      >
        View on GitHub
      </a>
      <a class="button-secondary" href="#features">Read the feature list</a>
    </div>

    <div class="utility-links">
      <a href="#faq">Read the FAQ</a>
      <a href="#direction">See the product direction</a>
    </div>
  </main>

  <footer class="site-footer">
    <div class="site-main site-footer-inner">
      <div class="footer-grid">
        <div>
          <p class="section-label">Product</p>
          <a href="#features">Features</a>
          <a href="#screenshot">Launcher preview</a>
          <a href="#faq">FAQ</a>
        </div>

        <div>
          <p class="section-label">Modes</p>
          <a href="#direction">Desktop first</a>
          <a href="#direction">Background next</a>
          <a href="#direction">Remote later</a>
        </div>

        <div>
          <p class="section-label">Architecture</p>
          <a href="#features">Local inference</a>
          <a href="#features">Decoupled core</a>
          <a href="#features">File output</a>
        </div>

        <div>
          <p class="section-label">Links</p>
          <a href="https://github.com/stephanvs/yapper" target="_blank" rel="noreferrer">GitHub</a>
          <a href="https://github.com/stephanvs/yapper/blob/main/README.md" target="_blank" rel="noreferrer">README</a>
          <a href="#overview">Top</a>
        </div>
      </div>

      <div class="footer-bottom">
        <p>© 2026 Yapper</p>
        <p>Built with Bun and Svelte.</p>
      </div>
    </div>
  </footer>
</div>
