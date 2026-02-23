import Foundation

struct TTSModel: Codable {
    let id: String
    let name: String
    let language: String
    let sample_rate_hz: Int
    let backend: String
}

struct SynthesizeRequest: Codable {
    let text: String
    let model_id: String
    let stream: Bool
    let format: String
}

struct SynthesizeResponse: Codable {
    let request_id: String
    let stream_url: String
    let format: String
    let sample_rate_hz: Int
}
