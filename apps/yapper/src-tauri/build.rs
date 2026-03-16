use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const YAP_BINARY_NAME: &str = "yap";

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR"));
    let yap_project_dir = manifest_dir.join("../../../services/yap");

    println!("cargo:rerun-if-env-changed=TAURI_ENV_TARGET_TRIPLE");
    println!("cargo:rerun-if-env-changed=TARGET");
    emit_rerun_if_changed(&yap_project_dir);
    build_yap_sidecar(&manifest_dir, &yap_project_dir);

    tauri_build::build()
}

fn build_yap_sidecar(tauri_dir: &Path, yap_project_dir: &Path) {
    let target_triple = env::var("TAURI_ENV_TARGET_TRIPLE")
        .or_else(|_| env::var("TARGET"))
        .expect("missing target triple for yap sidecar build");
    let runtime_identifier = dotnet_runtime_identifier(&target_triple).unwrap_or_else(|| {
        panic!("unsupported target triple `{target_triple}` for yap sidecar build")
    });
    let binary_extension = binary_extension(&target_triple);
    let binaries_dir = tauri_dir.join("binaries");
    let publish_dir = PathBuf::from(env::var("OUT_DIR").expect("missing OUT_DIR"))
        .join("yap-sidecar")
        .join(&target_triple);
    let sidecar_path = binaries_dir.join(format!(
        "{YAP_BINARY_NAME}-{target_triple}{binary_extension}"
    ));
    let published_binary = publish_dir.join(format!("{YAP_BINARY_NAME}{binary_extension}"));

    if publish_dir.exists() {
        fs::remove_dir_all(&publish_dir).unwrap_or_else(|error| {
            panic!(
                "failed to clear existing yap publish directory `{}`: {error}",
                publish_dir.display()
            )
        });
    }

    fs::create_dir_all(&publish_dir).unwrap_or_else(|error| {
        panic!(
            "failed to create yap publish directory `{}`: {error}",
            publish_dir.display()
        )
    });
    fs::create_dir_all(&binaries_dir).unwrap_or_else(|error| {
        panic!(
            "failed to create Tauri sidecar directory `{}`: {error}",
            binaries_dir.display()
        )
    });

    println!(
        "cargo:warning=publishing `{YAP_BINARY_NAME}` sidecar for `{target_triple}` ({runtime_identifier})"
    );

    let status = Command::new("dotnet")
        .arg("publish")
        .arg(yap_project_dir.join("yap.csproj"))
        .args(["-c", "Release", "-r", runtime_identifier, "-o"])
        .arg(&publish_dir)
        .current_dir(yap_project_dir)
        .status()
        .unwrap_or_else(|error| panic!("failed to run `dotnet publish` for yap: {error}"));

    if !status.success() {
        panic!(
            "`dotnet publish` failed while building the `{YAP_BINARY_NAME}` sidecar for `{runtime_identifier}`"
        );
    }

    if !published_binary.exists() {
        panic!(
            "expected published yap sidecar at `{}`, but it was not created",
            published_binary.display()
        );
    }

    if sidecar_path.exists() {
        fs::remove_file(&sidecar_path).unwrap_or_else(|error| {
            panic!(
                "failed to replace existing Tauri sidecar `{}`: {error}",
                sidecar_path.display()
            )
        });
    }

    fs::copy(&published_binary, &sidecar_path).unwrap_or_else(|error| {
        panic!(
            "failed to copy published yap sidecar from `{}` to `{}`: {error}",
            published_binary.display(),
            sidecar_path.display()
        )
    });

    #[cfg(unix)]
    if binary_extension.is_empty() {
        use std::os::unix::fs::PermissionsExt;

        fs::set_permissions(&sidecar_path, fs::Permissions::from_mode(0o755)).unwrap_or_else(
            |error| {
                panic!(
                    "failed to mark `{}` as executable: {error}",
                    sidecar_path.display()
                )
            },
        );
    }
}

fn emit_rerun_if_changed(path: &Path) {
    if matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some("bin" | "obj")
    ) {
        return;
    }

    println!("cargo:rerun-if-changed={}", path.display());

    if path.is_dir() {
        let entries = fs::read_dir(path).unwrap_or_else(|error| {
            panic!("failed to read directory `{}`: {error}", path.display())
        });

        for entry in entries {
            let entry = entry.unwrap_or_else(|error| {
                panic!(
                    "failed to inspect directory entry under `{}`: {error}",
                    path.display()
                )
            });
            emit_rerun_if_changed(&entry.path());
        }
    }
}

fn dotnet_runtime_identifier(target_triple: &str) -> Option<&'static str> {
    match target_triple {
        "aarch64-apple-darwin" => Some("osx-arm64"),
        "x86_64-apple-darwin" => Some("osx-x64"),
        "x86_64-pc-windows-msvc" => Some("win-x64"),
        "aarch64-pc-windows-msvc" => Some("win-arm64"),
        "x86_64-unknown-linux-gnu" => Some("linux-x64"),
        "aarch64-unknown-linux-gnu" => Some("linux-arm64"),
        _ => None,
    }
}

fn binary_extension(target_triple: &str) -> &'static str {
    if target_triple.contains("windows") {
        ".exe"
    } else {
        ""
    }
}
