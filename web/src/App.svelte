<svelte:head>
  <title>yapper — text to speech from your terminal</title>
  <meta
    name="description"
    content="A small command-line tool for turning text into speech with Grok or ElevenLabs."
  />
</svelte:head>

<div class="page">
  <header class="header">
    <a class="wordmark" href="/" aria-label="Yapper home">
      <span class="mark" aria-hidden="true">y</span>
      <span>yapper</span>
    </a>

    <a
      class="github-link"
      href="https://github.com/stephanvs/yapper"
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
        Yapper is a small CLI that turns text into audio using Grok or ElevenLabs.
        Play it now, or save it for later.
      </p>
    </section>

    <section class="terminal" aria-label="Yapper command examples">
      <div class="terminal-bar">
        <span>terminal</span>
        <span class="terminal-status">yap</span>
      </div>

      <div class="terminal-body">
        <div class="command">
          <span class="prompt" aria-hidden="true">$</span>
          <code>yap speak <span class="string">"Good morning."</span></code>
        </div>
        <p class="output">[grok] playing on the default audio device</p>

        <div class="command command-secondary">
          <span class="prompt" aria-hidden="true">$</span>
          <code>yap synthesize <span class="string">"Ship it."</span> <span class="flag">-o</span> ./audio</code>
        </div>
        <p class="output">[grok] wrote audio to ./audio/yap-….mp3</p>
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
      href="https://github.com/stephanvs/yapper/blob/main/README.md"
      target="_blank"
      rel="noreferrer"
    >README ↗</a>
  </footer>
</div>
