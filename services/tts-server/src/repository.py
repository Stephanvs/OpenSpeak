from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class ModelRecord:
    id: str
    name: str
    language: str
    backend: str
    sample_rate_hz: int = 22050


# Stub model inventory; will be loaded from manifests in future phases.
MODEL_INVENTORY = [
    ModelRecord(id="piper-en-us-amy", name="Amy (US)", language="en-US", backend="piper"),
    ModelRecord(id="piper-en-gb-alan", name="Alan (UK)", language="en-GB", backend="piper"),
    ModelRecord(id="avspeech-en-us", name="System Voice (macOS)", language="en-US", backend="avspeech"),
]
