use crate::util::create_hidden_command;
use std::env::current_dir;
use std::path::Path;
use std::process::Command;

/// Find if 7z exists in PATH
fn exist_7z() -> bool {
    match Command::new("7z").arg("--help").output() {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

type Result<T> = std::result::Result<T, std::io::Error>;

pub fn compress(
    input_dir: impl AsRef<Path>,
    output_file: impl AsRef<Path>,
    password: Option<&str>,
    compression_level: Option<u32>,
) -> Result<()> {
    if !exist_7z() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "7z not found in path",
        ));
    }

    let input_path = input_dir.as_ref();
    let output_path = output_file.as_ref();
    let compression_level = match compression_level {
        Some(level) => {
            if level <= 9 {
                level
            } else {
                9
            }
        }
        None => 9,
    };

    let mut command = create_hidden_command("7z");

    command.arg("a").arg(format!("-mx={compression_level}"));

    if let Some(pwd) = password {
        command.arg(format!("-p{pwd}")).arg("-mhe=on");
    }

    command.arg(
        current_dir()
            .expect("Failed to get current directory")
            .join(output_path),
    );
    command.current_dir(input_path);
    command.arg("*");

    let output = command.output()?;
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed in 7z compression command: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    Ok(())
}

pub fn decompress(
    input_file: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    password: Option<&str>,
) -> Result<()> {
    if !exist_7z() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "7z not found in path",
        ));
    }

    let input_path = input_file.as_ref();
    let output_path = output_dir.as_ref();

    let mut command = create_hidden_command("7z");
    command
        .arg("x")
        .arg(input_path)
        .arg(format!("-o{}", output_path.display()))
        .arg("-aoa");

    if let Some(pwd) = password {
        command.arg(format!("-p{pwd}"));
    }

    let output = command.output()?;
    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed in 7z decompression command: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    Ok(())
}
