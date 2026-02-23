import SwiftUI

@main
struct YapperApp: App {
    var body: some Scene {
        MenuBarExtra("Yapper", systemImage: "waveform") {
            MenubarContentView(viewModel: MenubarViewModel())
                .frame(width: 320)
                .padding(12)
        }
        .menuBarExtraStyle(.window)
    }
}
