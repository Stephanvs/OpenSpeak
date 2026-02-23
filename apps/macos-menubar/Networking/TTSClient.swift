import Foundation

struct TTSClient {
    private let baseURL = URL(string: "http://127.0.0.1:5100")!

    func models() async throws -> [TTSModel] {
        let (data, _) = try await URLSession.shared.data(from: baseURL.appending(path: "/v1/models"))
        return try JSONDecoder().decode([TTSModel].self, from: data)
    }

    func synthesize(text: String, modelId: String) async throws -> URL {
        var request = URLRequest(url: baseURL.appending(path: "/v1/synthesize"))
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let payload = SynthesizeRequest(text: text, model_id: modelId, stream: true, format: "wav")
        request.httpBody = try JSONEncoder().encode(payload)

        let (data, _) = try await URLSession.shared.data(for: request)
        let result = try JSONDecoder().decode(SynthesizeResponse.self, from: data)

        let audioURL = baseURL.appending(path: result.stream_url)
        let (audioData, _) = try await URLSession.shared.data(from: audioURL)

        let fileURL = FileManager.default.temporaryDirectory
            .appendingPathComponent("\(result.request_id).wav")
        try audioData.write(to: fileURL)
        return fileURL
    }
}
