from __future__ import annotations

from fastapi import FastAPI, HTTPException
from fastapi.responses import Response

from .models import HealthResponse, SynthesizeRequest
from .service import TTSService


def create_app() -> FastAPI:
    app = FastAPI(title="Yapper TTS Server", version="0.1.0")
    service = TTSService()
    audio_store: dict[str, bytes] = {}

    @app.get("/v1/health", response_model=HealthResponse)
    def health() -> HealthResponse:
        return HealthResponse(models_loaded=len(service.list_models()))

    @app.get("/v1/models")
    def list_models() -> list[dict]:
        return [model.model_dump() for model in service.list_models()]

    @app.post("/v1/synthesize")
    def synthesize(request: SynthesizeRequest) -> dict:
        try:
            result, wav_data = service.synthesize(request)
        except ValueError as exc:
            raise HTTPException(status_code=404, detail=str(exc)) from exc
        audio_store[result.request_id] = wav_data
        return result.model_dump()

    @app.get("/v1/audio/{request_id}")
    def get_audio(request_id: str) -> Response:
        payload = audio_store.get(request_id)
        if payload is None:
            raise HTTPException(status_code=404, detail="Audio not found")
        return Response(content=payload, media_type="audio/wav")

    return app


app = create_app()
