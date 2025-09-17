use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub enum Error {
    FolderNotFound,
    AccessDenied,
    FileNotFound,

    CreateOutputError,
    WriteFileError,
    CommandError,
}

fn read_dir(path: &Path, prefix: &str, ext: &str) -> Result<Vec<String>, Error> {
    if !path.exists() {
        return Err(Error::FolderNotFound);
    }

    let entries = std::fs::read_dir(path).map_err(|_| Error::AccessDenied)?;
    let mut paths = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_file()
                    && path
                        .extension()
                        .is_some_and(|e| e.eq_ignore_ascii_case(ext))
                    && path
                        .file_name()
                        .and_then(|e| e.to_str())
                        .is_some_and(|s| s.starts_with(prefix))
                {
                    Some(path.to_str().unwrap().to_string())
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    paths.sort_unstable();
    Ok(paths)
}

pub fn concat(path: &Path, prefix: &str, ext: &str, output: &Path) -> Result<bool, Error> {
    let tmp_path = "/tmp/file_list.txt";
    let mut f = File::create(tmp_path).map_err(|_| Error::CreateOutputError)?;

    for file in read_dir(path, prefix, ext)? {
        writeln!(f, "file '{}'", file).map_err(|_| Error::WriteFileError)?;
    }

    // Step 2: run ffmpeg concat
    let status = Command::new("ffmpeg")
        .args([
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            tmp_path,
            "-c",
            "copy",
            output.as_os_str().to_str().unwrap(),
        ])
        .status()
        .map_err(|_| Error::CommandError)?;

    Ok(status.success())
}
