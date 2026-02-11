from __future__ import annotations

import io
import math
import wave
from uuid import uuid4

from .models import SynthesizeRequest, SynthesizeResponse, VoiceModel
from .repository import MODEL_INVENTORY


class TTSService:
    def list_models(self) -> list[VoiceModel]:
        return [
            VoiceModel(
                id=record.id,
                name=record.name,
                language=record.language,
                sample_rate_hz=record.sample_rate_hz,
                backend=record.backend,
            )
            for record in MODEL_INVENTORY
        ]

    def synthesize(self, request: SynthesizeRequest) -> tuple[SynthesizeResponse, bytes]:
        selected = next((m for m in MODEL_INVENTORY if m.id == request.model_id), None)
        if selected is None:
            raise ValueError(f"Unknown model_id: {request.model_id}")

        # Stub audio generation (sine wave) so contract and clients can be built now.
        sample_rate = selected.sample_rate_hz
        duration_s = min(0.25 + len(request.text) * 0.01, 4.0)
        pcm = self._sine_wave(sample_rate=sample_rate, duration_s=duration_s)
        wav = self._to_wav_bytes(sample_rate=sample_rate, pcm_data=pcm)

        request_id = str(uuid4())
        response = SynthesizeResponse(
            request_id=request_id,
            stream_url=f"/v1/audio/{request_id}",
            format=request.format,
            sample_rate_hz=sample_rate,
        )
        return response, wav

    @staticmethod
    def _sine_wave(sample_rate: int, duration_s: float, freq: float = 440.0) -> bytes:
        samples = int(sample_rate * duration_s)
        data = bytearray()
        amplitude = 0.2 * 32767
        for i in range(samples):
            value = int(amplitude * math.sin(2 * math.pi * freq * (i / sample_rate)))
            data.extend(value.to_bytes(2, byteorder="little", signed=True))
        return bytes(data)

    @staticmethod
    def _to_wav_bytes(sample_rate: int, pcm_data: bytes) -> bytes:
        buffer = io.BytesIO()
        with wave.open(buffer, "wb") as wav_file:
            wav_file.setnchannels(1)
            wav_file.setsampwidth(2)
            wav_file.setframerate(sample_rate)
            wav_file.writeframes(pcm_data)
        return buffer.getvalue()
