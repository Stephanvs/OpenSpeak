from __future__ import annotations

from typing import Literal

from pydantic import BaseModel, Field


class VoiceModel(BaseModel):
    id: str
    name: str
    language: str
    sample_rate_hz: int = 22050
    backend: str


class SynthesizeRequest(BaseModel):
    text: str = Field(min_length=1, max_length=5000)
    model_id: str
    stream: bool = True
    format: Literal["wav", "pcm_s16le"] = "wav"


class SynthesizeResponse(BaseModel):
    request_id: str
    stream_url: str
    format: str
    sample_rate_hz: int


class HealthResponse(BaseModel):
    status: Literal["ok"] = "ok"
    models_loaded: int


class ErrorResponse(BaseModel):
    error_code: str
    message: str
