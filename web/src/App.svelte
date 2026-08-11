<script lang="ts">
  import { onMount } from 'svelte'
  import DemoViz from './lib/DemoViz.svelte'

  type InstallOS = 'unix' | 'windows'

  const installCommands: Record<InstallOS, string> = {
    unix: 'curl -fsSL https://openspeak.sh/install | sh',
    windows: 'irm https://openspeak.sh/install.ps1 | iex',
  }

  const demoTabs = [
    { title: 'Agent speak', detail: 'Hear task updates', caption: 'Neural voice core' },
    { title: 'Articles', detail: 'Audio for every post', caption: 'Living document' },
    { title: 'Podcasts', detail: 'Narrate transcripts', caption: 'Broadcast disc' },
    { title: 'Providers', detail: 'Grok + ElevenLabs', caption: 'Dual engines' },
    { title: 'CLI control', detail: 'Override every call', caption: 'Command lattice' },
  ]

  let activeDemo = $state(0)
  let installOS = $state<InstallOS>('unix')
  let copied = $state(false)
  let copyTimeout: ReturnType<typeof setTimeout> | undefined
  let demoInterval: ReturnType<typeof setInterval> | undefined

  function selectDemo(index: number) {
    activeDemo = index
    restartDemoInterval()
  }

  function restartDemoInterval() {
    clearInterval(demoInterval)
    demoInterval = setInterval(() => {
      activeDemo = (activeDemo + 1) % demoTabs.length
    }, 8000)
  }

  function selectInstallOS(os: InstallOS) {
    installOS = os
    copied = false
  }

  async function copyInstallCommand() {
    try {
      await navigator.clipboard.writeText(installCommands[installOS])
      copied = true
      clearTimeout(copyTimeout)
      copyTimeout = setTimeout(() => (copied = false), 1600)
    } catch {
      copied = false
    }
  }

  onMount(() => {
    installOS = /windows/i.test(navigator.userAgent) ? 'windows' : 'unix'
    restartDemoInterval()

    return () => {
      clearTimeout(copyTimeout)
      clearInterval(demoInterval)
    }
  })
</script>

<svelte:head>
  <title>OpenSpeak — text to speech from your terminal</title>
  <meta
    name="description"
    content="OpenSpeak is a small Rust command-line tool that turns text into speech with Grok Voice or ElevenLabs, played instantly or saved to a file."
  />
</svelte:head>

<a class="skip-link" href="#main">Skip to content</a>

<main id="main" class="landing">
  <section class="top-experience" aria-labelledby="page-title">
    <div class="page-rails" aria-hidden="true"></div>

    <nav class="site-header" aria-label="OpenSpeak navigation">
      <a class="brand" href="/" aria-label="OpenSpeak home">
        <span class="brand-mark" aria-hidden="true">&gt;_</span>
        <span class="brand-wordmark">OpenSpeak</span>
      </a>
      <a class="marketing-button signup-button" href="https://openspeak.sh" target="_blank" rel="noreferrer">
        <span>Documentation</span>
      </a>
    </nav>

    <header class="hero-copy">
      <div class="eyebrow-logo" aria-hidden="true">
        <span>&gt;_</span>
        <b>OpenSpeak</b>
      </div>
      <h1 id="page-title">Your terminal just<br />found its voice.</h1>
      <p>
        Turn text into speech with Grok Voice or ElevenLabs. Play it instantly,
        save it to a file, and control everything from one small Rust command.
      </p>
      <div class="hero-meta">
        <a class="github-link" href="https://github.com/Stephanvs/OpenSpeak" target="_blank" rel="noreferrer">
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <path fill="currentColor" d="M12 2C6.48 2 2 6.58 2 12.23c0 4.52 2.87 8.35 6.84 9.71.5.1.68-.22.68-.49 0-.24-.01-1.05-.02-1.9-2.78.62-3.37-1.21-3.37-1.21-.45-1.18-1.11-1.49-1.11-1.49-.91-.64.07-.63.07-.63 1 .08 1.53 1.06 1.53 1.06.9 1.57 2.35 1.12 2.92.86.09-.66.35-1.12.64-1.38-2.22-.26-4.56-1.14-4.56-5.06 0-1.12.39-2.03 1.03-2.75-.1-.26-.45-1.3.1-2.71 0 0 .84-.28 2.75 1.05A9.3 9.3 0 0 1 12 7.14a9.3 9.3 0 0 1 2.5.35c1.91-1.33 2.75-1.05 2.75-1.05.55 1.41.2 2.45.1 2.71.64.72 1.03 1.63 1.03 2.75 0 3.93-2.35 4.8-4.58 5.05.36.32.68.94.68 1.9 0 1.37-.01 2.47-.01 2.81 0 .27.18.59.69.49A10.25 10.25 0 0 0 22 12.23C22 6.58 17.52 2 12 2Z" />
          </svg>
          <span>View on GitHub</span>
        </a>
      </div>
    </header>

    <section class="showcase" aria-label="OpenSpeak examples">
      <div class="tabs" role="tablist" aria-label="OpenSpeak demonstrations">
        {#each demoTabs as tab, index}
          <button
            class:active={activeDemo === index}
            class="demo-tab"
            type="button"
            role="tab"
            aria-selected={activeDemo === index}
            onclick={() => selectDemo(index)}
          >
            <span>0{index + 1}</span>
            <strong>{tab.title}</strong>
            <small>{tab.detail}</small>
            <i aria-hidden="true"><b></b></i>
          </button>
        {/each}
      </div>

      <div class="composition-well">
        <div class="composition" aria-label={`${demoTabs[activeDemo].title} wireframe demonstration`}>
          <div class="demo-window">
            <div class="demo-sidebar" aria-hidden="true">
              <div class="demo-product"><span>&gt;_</span><b>openspeak</b></div>
              <div class="demo-nav-line active"></div>
              <div class="demo-nav-line"></div>
              <div class="demo-nav-line short"></div>
              <div class="demo-sidebar-rule"></div>
              <div class="demo-nav-line"></div>
              <div class="demo-nav-line short"></div>
            </div>

            <div class="demo-workspace">
              <div class="workspace-bar">
                <span>{demoTabs[activeDemo].title}</span>
                <div><i></i><i></i><i></i></div>
              </div>

              <div class="demo-stage">
                <DemoViz
                  activeIndex={activeDemo}
                  label={demoTabs[activeDemo].caption}
                />
                <div class="demo-stage-hud" aria-hidden="true">
                  {#if activeDemo === 0}
                    <div class="command-card">
                      <span class="prompt">$</span>
                      <code>openspeak speak "$SUMMARY"</code>
                      <span class="command-status">played</span>
                    </div>
                  {:else if activeDemo === 1}
                    <div class="audio-player"><b>▶</b><span></span><span></span><span></span><span></span><small>02:14</small></div>
                  {:else if activeDemo === 2}
                    <div class="command-card">
                      <span class="prompt">$</span>
                      <code>openspeak synthesize "$(cat transcript.txt)"</code>
                      <span class="command-status">saved</span>
                    </div>
                  {:else if activeDemo === 3}
                    <div class="provider-hud">
                      <span>Grok Voice</span>
                      <i></i>
                      <span>ElevenLabs</span>
                    </div>
                  {:else}
                    <div class="command-card">
                      <span class="prompt">$</span>
                      <code>openspeak speak "Hello" --voice eve --speed 1.1</code>
                      <span class="command-status">ready</span>
                    </div>
                  {/if}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  </section>

  <div class="below-fold">
    <div class="content-rails">
      <section class="platform-block getting-started" id="getting-started" aria-labelledby="getting-started-title">
        <div class="block-copy">
          <div class="section-kicker">GETTING STARTED</div>
          <h2 id="getting-started-title">Install in one command.</h2>
          <p>
            Detects your platform and puts <code>openspeak</code> on your PATH —
            no package manager and no manual download.
          </p>
          <a class="section-link" href="https://openspeak.sh" target="_blank" rel="noreferrer">
            <span>Read the documentation</span><span aria-hidden="true">↗</span>
          </a>
        </div>

        <div class="install-panel">
          <div class="install-toggle" role="group" aria-label="Choose your platform">
            <button type="button" class:active={installOS === 'unix'} aria-pressed={installOS === 'unix'} onclick={() => selectInstallOS('unix')}>macOS / Linux</button>
            <button type="button" class:active={installOS === 'windows'} aria-pressed={installOS === 'windows'} onclick={() => selectInstallOS('windows')}>Windows</button>
          </div>
          <div class="install-terminal">
            <div class="terminal-bar">
              <span>{installOS === 'windows' ? 'powershell' : 'terminal'}</span>
              <i aria-hidden="true"></i>
            </div>
            <div class="install-command">
              <span class="prompt">{installOS === 'windows' ? '>' : '$'}</span>
              <code>{installCommands[installOS]}</code>
              <button type="button" onclick={copyInstallCommand}>{copied ? 'Copied' : 'Copy'}</button>
            </div>
          </div>
          <p class="install-note"><span></span> No config file required</p>
        </div>
      </section>

      <section class="platform-block context-block" id="showcase" aria-labelledby="showcase-title">
        <div class="context-reads" aria-hidden="true">
          <div class="context-track">
            <span>Agent finished a task <b>›</b></span>
            <span>Article ready to publish <b>›</b></span>
            <span>Transcript ready to narrate <b>›</b></span>
            <span>Agent finished a task <b>›</b></span>
            <span>Article ready to publish <b>›</b></span>
            <span>Transcript ready to narrate <b>›</b></span>
          </div>
        </div>
        <div class="section-kicker">WHAT WILL YOU MAKE SPEAK?</div>
        <h2 id="showcase-title">One small command, many ways to be heard.</h2>
        <p>
          Give an agent a voice, add audio to every article, narrate a transcript,
          voice a notification, or build something entirely new.
        </p>
      </section>

      <section class="platform-block providers-block" id="providers" aria-labelledby="providers-title">
        <div class="block-copy providers-copy">
          <div class="section-kicker">PROVIDERS</div>
          <h2 id="providers-title">Configured entirely by environment variables.</h2>
          <p>
            Set your API key and defaults once. Use <code>--provider</code> or
            <code>OPENSPEAK_DEFAULT_PROVIDER</code> to choose between them.
          </p>
        </div>
        <div class="provider-list">
          <article>
            <span class="point-index">01</span>
            <div><h3>Grok Voice <b>DEFAULT</b></h3><p><code>XAI_API_KEY</code>, voice <code>eve</code>, MP3 at 24 kHz.</p></div>
          </article>
          <article>
            <span class="point-index">02</span>
            <div><h3>ElevenLabs</h3><p><code>ELEVENLABS_API_KEY</code>, voice ID, model, and output format.</p></div>
          </article>
          <article>
            <span class="point-index">03</span>
            <div><h3>No config file</h3><p>Everything is explicit, portable, and easy to automate.</p></div>
          </article>
        </div>
      </section>

      <section class="platform-block options-block" id="options" aria-labelledby="options-title">
        <div class="block-copy">
          <div class="section-kicker">OPTIONS</div>
          <h2 id="options-title">Override anything, per call.</h2>
          <p>Keep sensible defaults, then tune a single request without touching global configuration.</p>
        </div>
        <dl class="options-grid">
          <div><dt><code>--provider</code></dt><dd>grok or elevenlabs</dd></div>
          <div><dt><code>--voice</code></dt><dd>voice or voice ID</dd></div>
          <div><dt><code>--model</code></dt><dd>model override</dd></div>
          <div><dt><code>--codec</code></dt><dd>mp3, wav, pcm, mulaw, alaw</dd></div>
          <div><dt><code>--speed</code></dt><dd>speech speed</dd></div>
          <div><dt><code>--sample-rate</code></dt><dd>output sample rate</dd></div>
          <div><dt><code>--bit-rate</code></dt><dd>output bit rate</dd></div>
          <div><dt><code>--language</code></dt><dd>BCP-47 language code</dd></div>
        </dl>
      </section>

      <section class="platform-block playback-block" id="playback" aria-labelledby="playback-title">
        <div class="block-copy playback-copy">
          <div class="section-kicker">PLAYBACK</div>
          <h2 id="playback-title">Plays through your own system.</h2>
          <p>
            <code>speak</code> hands generated audio to the tools your operating
            system already has. OpenSpeak does not ship another audio stack.
          </p>
        </div>
        <ol class="platform-list">
          <li><span class="point-index">01</span><div><h3>macOS</h3><p>The built-in <code>afplay</code> command.</p></div></li>
          <li><span class="point-index">02</span><div><h3>Linux</h3><p><code>mpg123</code> for MP3, <code>aplay</code> for other formats.</p></div></li>
          <li><span class="point-index">03</span><div><h3>Windows</h3><p>The built-in WinMM MCI API.</p></div></li>
        </ol>
      </section>

      <section class="deploy-block">
        <div class="section-kicker">GET STARTED</div>
        <h2>Give your terminal a voice.</h2>
        <a class="marketing-button deploy-button" href="#getting-started"><span>Install OpenSpeak</span><span aria-hidden="true">↓</span></a>
      </section>

      <footer class="site-footer">
        <span>OpenSpeak. Small on purpose.</span>
        <nav aria-label="Footer navigation">
          <a href="#getting-started">Install</a>
          <a href="https://openspeak.sh" target="_blank" rel="noreferrer">Documentation</a>
          <a href="https://github.com/Stephanvs/yapper" target="_blank" rel="noreferrer">GitHub</a>
        </nav>
      </footer>
    </div>
  </div>
</main>
