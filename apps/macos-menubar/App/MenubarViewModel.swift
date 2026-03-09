import Foundation
import AVFoundation
internal import Combine

@MainActor
final class MenubarViewModel: ObservableObject {
    @Published var text: String = ""
    @Published var models: [TTSModel] = []
    @Published var selectedModelId: String = ""
    @Published var statusMessage: String = ""

    private let client = TTSClient()
    private var player: AVAudioPlayer?

    func refreshModels() async {
        do {
            let result = try await client.models()
            models = result
            if selectedModelId.isEmpty {
                selectedModelId = result.first?.id ?? ""
            }
            statusMessage = "Loaded \(result.count) models"
        } catch {
            statusMessage = "Failed to load models: \(error.localizedDescription)"
        }
    }

    func synthesize() async {
        guard !selectedModelId.isEmpty else {
            statusMessage = "Select a model first"
            return
        }

        do {
            let outputPath = try await client.synthesize(text: text, modelId: selectedModelId)
            player?.stop()
            player = try AVAudioPlayer(contentsOf: outputPath)
            player?.prepareToPlay()
            if player?.play() == true {
                statusMessage = "Playing: \(outputPath.lastPathComponent)"
            } else {
                statusMessage = "Saved audio: \(outputPath.lastPathComponent)"
            }
        } catch {
            statusMessage = "Synthesis failed: \(error.localizedDescription)"
        }
    }
}
