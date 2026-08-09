<svelte:head>
  <title>OpenSpeak — text to speech from your terminal</title>
  <meta
    name="description"
    content="A small command-line tool for turning text into speech with Grok or ElevenLabs."
  />
</svelte:head>

<div class="page">
  <header class="header">
    <a class="wordmark" href="/" aria-label="OpenSpeak home">
      <span class="mark" aria-hidden="true">O</span>
      <span>OpenSpeak</span>
    </a>

    <a
      class="github-link"
      href="https://openspeak.sh"
      target="_blank"
      rel="noreferrer"
    >
      GitHub
      <span aria-hidden="true">↗</span>
    </a>
  </header>

  <main>
    <section class="intro" aria-labelledby="page-title">
      <p class="kicker">Text to speech, without the ceremony.</p>
      <h1 id="page-title">Make your terminal<br />say something.</h1>
      <p class="lede">
        OpenSpeak is a small CLI that turns text into audio using Grok or ElevenLabs.
        Play it now, or save it for later.
      </p>
    </section>

    <section class="terminal" aria-label="OpenSpeak command examples">
      <div class="terminal-bar">
        <span>terminal</span>
        <span class="terminal-status">openspeak</span>
      </div>

      <div class="terminal-body">
        <div class="command">
          <span class="prompt" aria-hidden="true">$</span>
          <code>openspeak speak <span class="string">"Good morning."</span></code>
        </div>
        <p class="output">[grok] playing on the default audio device</p>

        <div class="command command-secondary">
          <span class="prompt" aria-hidden="true">$</span>
          <code>openspeak synthesize <span class="string">"Ship it."</span> <span class="flag">-o</span> ./audio</code>
        </div>
        <p class="output">[grok] wrote audio to ./audio/openspeak-….mp3</p>
      </div>
    </section>

    <section class="details" aria-label="Project details">
      <div>
        <h2>Providers</h2>
        <p>Grok Voice and ElevenLabs.</p>
      </div>
      <div>
        <h2>Output</h2>
        <p>MP3, WAV, PCM, μ-law, and A-law.</p>
      </div>
      <div>
        <h2>Also included</h2>
        <p>A compact desktop launcher for macOS.</p>
      </div>
    </section>
  </main>

  <footer>
    <p>Small on purpose.</p>
    <a
      href="https://openspeak.sh"
      target="_blank"
      rel="noreferrer"
    >README ↗</a>
  </footer>
</div>
