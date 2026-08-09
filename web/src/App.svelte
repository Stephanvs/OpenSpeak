<script lang="ts">
  import { onMount } from 'svelte'

  const sectionIds = ['commands', 'providers', 'options', 'playback']
  let activeSection = $state('')

  onMount(() => {
    const sections = sectionIds
      .map((id) => document.getElementById(id))
      .filter((el): el is HTMLElement => el !== null)

    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) activeSection = entry.target.id
        }
      },
      { rootMargin: '-40% 0px -55% 0px', threshold: 0 },
    )

    for (const section of sections) observer.observe(section)

    return () => observer.disconnect()
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

<div class="page">
  <header class="header">
    <a class="wordmark" href="/" aria-label="OpenSpeak home">
      <span class="mark" aria-hidden="true">&gt;_</span>
      <span>OpenSpeak</span>
    </a>

    <nav class="nav" aria-label="Primary">
      <a href="#commands" class:active={activeSection === 'commands'} aria-current={activeSection === 'commands' ? 'true' : undefined}>Commands</a>
      <a href="#providers" class:active={activeSection === 'providers'} aria-current={activeSection === 'providers' ? 'true' : undefined}>Providers</a>
      <a href="#options" class:active={activeSection === 'options'} aria-current={activeSection === 'options' ? 'true' : undefined}>Options</a>
      <a href="#playback" class:active={activeSection === 'playback'} aria-current={activeSection === 'playback' ? 'true' : undefined}>Playback</a>
      <a class="nav-cta" href="https://openspeak.sh" target="_blank" rel="noreferrer">
        Documentation <span aria-hidden="true">↗</span>
      </a>
    </nav>
  </header>

  <main id="main">
    <section class="hero" aria-labelledby="page-title">
      <p class="kicker">Command-line text to speech, written in Rust</p>
      <h1 id="page-title">Your terminal just found its voice</h1>
      <p class="lede">
        OpenSpeak turns text into speech with Grok Voice or ElevenLabs. Play the
        result instantly through your system's audio output, or write it straight
        to a file — one command, no config file to manage.
      </p>

      <div class="terminal" aria-label="OpenSpeak command examples">
        <div class="terminal-bar">
          <span class="dots" aria-hidden="true"><i></i><i></i><i></i></span>
          <span class="terminal-title">openspeak</span>
        </div>

        <div class="terminal-body">
          <div class="command">
            <span class="prompt" aria-hidden="true">$</span>
            <code>export <span class="flag">XAI_API_KEY</span>=<span class="string">"..."</span></code>
          </div>

          <div class="command">
            <span class="prompt" aria-hidden="true">$</span>
            <code>openspeak speak <span class="string">"Good morning."</span></code>
          </div>
          <p class="output">[grok] played 51200 bytes (audio/mpeg) on the default audio device</p>

          <div class="command command-secondary">
            <span class="prompt" aria-hidden="true">$</span>
            <code
              >openspeak synthesize <span class="string">"Ship it."</span>
              <span class="flag">--output</span> ./audio</code
            >
          </div>
          <p class="output">
            [grok] wrote 51200 bytes to
            ./audio/openspeak-20260809101530-3f9a2b6c1e4d4a2f9b6c1e4d4a2f9b6c.mp3
          </p>

          <div class="command command-secondary">
            <span class="prompt" aria-hidden="true">$</span>
            <span class="cursor" aria-hidden="true"></span>
          </div>
        </div>
      </div>
    </section>

    <section class="commands" id="commands" aria-labelledby="commands-title">
      <div class="section-head">
        <p class="eyebrow">Commands</p>
        <h2 id="commands-title" class="section-heading">Two commands</h2>
      </div>
      <div class="commands-grid">
        <article class="command-card">
          <span class="card-index" aria-hidden="true">01</span>
          <h3><code>speak</code> <span class="alias">alias: say</span></h3>
          <p>
            Synthesizes text and plays it immediately through your system's default
            audio device.
          </p>
        </article>
        <article class="command-card">
          <span class="card-index" aria-hidden="true">02</span>
          <h3><code>synthesize</code></h3>
          <p>
            Synthesizes text and writes it to a uniquely named audio file in the
            directory you choose, instead of playing it.
          </p>
        </article>
      </div>
    </section>

    <section class="providers" id="providers" aria-labelledby="providers-title">
      <div class="section-head">
        <p class="eyebrow">Providers</p>
        <h2 id="providers-title" class="section-heading">Configured entirely by environment variables</h2>
      </div>
      <p class="section-lede">
        There is no config file to manage. Set a few environment variables and
        OpenSpeak picks a default provider, voice, and output format for every
        request; <code>--provider</code> or <code>OPENSPEAK_DEFAULT_PROVIDER</code>
        chooses between them (<code>grok</code> by default).
      </p>

      <div class="provider-grid">
        <article class="provider-card">
          <h3>Grok Voice <span class="tag">default</span></h3>
          <dl>
            <div><dt><code>XAI_API_KEY</code></dt><dd>xAI API key</dd></div>
            <div><dt><code>XAI_DEFAULT_VOICE_ID</code></dt><dd>default <code>eve</code></dd></div>
            <div><dt><code>XAI_DEFAULT_OUTPUT_FORMAT</code></dt><dd>default <code>mp3</code></dd></div>
            <div><dt><code>XAI_DEFAULT_SAMPLE_RATE</code></dt><dd>default <code>24000</code> Hz</dd></div>
          </dl>
        </article>
        <article class="provider-card">
          <h3>ElevenLabs</h3>
          <dl>
            <div><dt><code>ELEVENLABS_API_KEY</code></dt><dd>ElevenLabs API key</dd></div>
            <div>
              <dt><code>ELEVENLABS_DEFAULT_VOICE_ID</code></dt>
              <dd>required, unless <code>--voice</code> is passed per call</dd>
            </div>
            <div>
              <dt><code>ELEVENLABS_DEFAULT_MODEL_ID</code></dt>
              <dd>default <code>eleven_multilingual_v2</code></dd>
            </div>
            <div>
              <dt><code>ELEVENLABS_DEFAULT_OUTPUT_FORMAT</code></dt>
              <dd>default <code>mp3_44100_128</code></dd>
            </div>
          </dl>
        </article>
      </div>
    </section>

    <section class="options" id="options" aria-labelledby="options-title">
      <div class="section-head">
        <p class="eyebrow">Options</p>
        <h2 id="options-title" class="section-heading">Override anything, per call</h2>
      </div>
      <dl class="options-grid">
        <div><dt><code>--provider</code></dt><dd>use <code>grok</code> or <code>elevenlabs</code> for this call</dd></div>
        <div><dt><code>--voice</code></dt><dd>provider-specific voice or voice ID</dd></div>
        <div>
          <dt><code>--model</code></dt>
          <dd>provider-specific model override (ElevenLabs only — xAI voices don't take a model)</dd>
        </div>
        <div><dt><code>--codec</code></dt><dd><code>mp3</code>, <code>wav</code>, <code>pcm</code>, <code>mulaw</code>, or <code>alaw</code></dd></div>
        <div><dt><code>--speed</code></dt><dd>speech speed override</dd></div>
        <div><dt><code>--sample-rate</code></dt><dd>output sample rate in Hz, where the provider supports it</dd></div>
        <div><dt><code>--bit-rate</code></dt><dd>output bit rate in bits per second, where the provider supports it</dd></div>
        <div><dt><code>--language</code></dt><dd>BCP-47 language code, where the provider supports it</dd></div>
      </dl>
    </section>

    <section class="playback" id="playback" aria-labelledby="playback-title">
      <div class="section-head">
        <p class="eyebrow">Playback</p>
        <h2 id="playback-title" class="section-heading">Plays through your own system</h2>
      </div>
      <p class="section-lede">
        OpenSpeak doesn't ship its own audio stack. <code>speak</code> hands the
        generated audio to whatever your OS already has.
      </p>
      <ul class="platform-list">
        <li><span>macOS</span><p>the built-in <code>afplay</code> command</p></li>
        <li><span>Linux</span><p><code>mpg123</code> for MP3, <code>aplay</code> for other formats</p></li>
        <li><span>Windows</span><p>the built-in WinMM MCI API</p></li>
      </ul>
    </section>
  </main>

  <footer>
    <p>Small on purpose.</p>
    <a href="https://openspeak.sh" target="_blank" rel="noreferrer">Documentation ↗</a>
  </footer>
</div>
