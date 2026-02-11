from __future__ import annotations

import json

import typer
import uvicorn
from httpx import Client

app = typer.Typer(help="Yapper TTS server + standalone client CLI")


@app.command("serve")
def serve(host: str = "127.0.0.1", port: int = 5100) -> None:
    """Run the TTS server process."""
    uvicorn.run("src.app:app", host=host, port=port, reload=False)


@app.command("health")
def health(base_url: str = "http://127.0.0.1:5100") -> None:
    """Check server health from CLI mode."""
    with Client(base_url=base_url, timeout=10) as client:
        data = client.get("/v1/health").json()
    typer.echo(json.dumps(data, indent=2))


@app.command("models")
def models(base_url: str = "http://127.0.0.1:5100") -> None:
    """List server models from CLI mode."""
    with Client(base_url=base_url, timeout=10) as client:
        data = client.get("/v1/models").json()
    typer.echo(json.dumps(data, indent=2))


@app.command("synthesize")
def synthesize(
    text: str,
    model_id: str = "piper-en-us-amy",
    output: str = "output.wav",
    base_url: str = "http://127.0.0.1:5100",
) -> None:
    """Synthesize text and save WAV output."""
    with Client(base_url=base_url, timeout=30) as client:
        created = client.post(
            "/v1/synthesize",
            json={"text": text, "model_id": model_id, "stream": True, "format": "wav"},
        )
        created.raise_for_status()
        synth = created.json()
        wav = client.get(synth["stream_url"])
        wav.raise_for_status()
    with open(output, "wb") as handle:
        handle.write(wav.content)
    typer.echo(f"Saved audio to {output}")


if __name__ == "__main__":
    app()
