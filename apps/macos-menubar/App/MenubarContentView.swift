import SwiftUI

struct MenubarContentView: View {
    @StateObject var viewModel: MenubarViewModel

    var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            Text("Yapper TTS")
                .font(.headline)

            Picker("Voice model", selection: $viewModel.selectedModelId) {
                ForEach(viewModel.models, id: \.id) { model in
                    Text(model.name).tag(model.id)
                }
            }

            TextEditor(text: $viewModel.text)
                .frame(minHeight: 80)

            HStack {
                Button("Refresh Models") {
                    Task { await viewModel.refreshModels() }
                }
                Button("Synthesize") {
                    Task { await viewModel.synthesize() }
                }
                .disabled(viewModel.text.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
            }

            if !viewModel.statusMessage.isEmpty {
                Text(viewModel.statusMessage)
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }
        }
        .task {
            await viewModel.refreshModels()
        }
    }
}
