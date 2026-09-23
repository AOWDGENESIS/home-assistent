use clap::{Parser, Subcommand};
use serde_json::json;

#[derive(Parser)]
#[command(
    name = "ur-cli",
    about = "Offline UniversalRecovery command line interface"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print a machine-readable health response.
    ListDrives,
    /// Run safe local diagnostics without opening a real device.
    Selftest,
    /// Scan a regular disk image without touching physical devices.
    ScanImage { path: std::path::PathBuf },
}

fn main() {
    let result = match Cli::parse().command {
        Command::ListDrives => {
            println!("{}", json!({"ok": true, "drives": [], "offline": true}));
            Ok(())
        }
        Command::Selftest => run_selftest(),
        Command::ScanImage { path } => scan_image(&path),
    };

    if let Err(message) = result {
        eprintln!("{}", json!({"ok": false, "error": message}));
        std::process::exit(1);
    }

    fn scan_image(path: &std::path::Path) -> Result<(), String> {
        use std::io::Read;
        let mut file = std::fs::File::open(path).map_err(|error| error.to_string())?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .map_err(|error| error.to_string())?;
        let files = ur_core::recovery::carve_headers(&data);
        println!(
            "{}",
            json!({"ok": true, "offline": true, "bytes": data.len(), "files": files})
        );
        Ok(())
    }
}

fn run_selftest() -> Result<(), String> {
    use std::path::PathBuf;
    use ur_core::{recovery, FileEntry, FilesystemType};

    let entry = FileEntry::new(PathBuf::from("selftest.bin"), 4, FilesystemType::Unknown, 0);
    let encoded = serde_json::to_string(&entry).map_err(|error| error.to_string())?;
    let decoded: FileEntry = serde_json::from_str(&encoded).map_err(|error| error.to_string())?;
    if decoded != entry {
        return Err("model round-trip mismatch".to_string());
    }
    if recovery::find_signature(b"prefix\xFF\xD8\xFFsuffix", &[0xFF, 0xD8, 0xFF]) != Some(6) {
        return Err("signature detector mismatch".to_string());
    }
    recovery::validate_confidence(0.95).map_err(|error| error.to_string())?;
    println!(
        "{}",
        json!({"ok": true, "selftest": "passed", "offline": true})
    );
    Ok(())
}
