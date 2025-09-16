use std::fs::File;
use std::io::Write;
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

/// Reads the contents of a directory and returns a sorted list of file paths with a specific file extension.
///
/// # Parameters
///
/// - `path`: A string slice that represents the directory path to read.
/// - `ext`: A string slice representing the target file extension to filter (case-insensitive).
///
/// # Returns
///
/// - `Ok(Vec<String>)`: A vector of strings containing the file paths with the specified extension, sorted in ascending order.
/// - `Err(Error)`: An error if the directory does not exist or cannot be read.
///
/// # Errors
///
/// - `Error::FolderNotFound`: Returned if the specified folder does not exist.
/// - `Error::AccessDenied`: Returned if the specified folder cannot be accessed.
///
/// # Example
///
/// ```
/// use your_module::read_dir;
/// use your_module::Error;
///
/// let result = read_dir("path/to/directory", "txt");
/// match result {
///     Ok(files) => {
///         for file in files {
///             println!("{}", file);
///         }
///     }
///     Err(Error::FolderNotFound) => {
///         eprintln!("The specified directory was not found.");
///     }
/// }
/// ```
///
/// # Notes
///
/// - This function only processes regular files and skips directories.
/// - The extension comparison is performed in a case-insensitive manner.
/// - If no files matching the specified extension are found, the function returns an empty vector (`Ok(Vec::new())`).
///
/// # Panics
///
/// - The function will panic if a file path cannot be converted to a `&str`, though this is unlikely in well-formed input.
///
/// # Dependencies
///
/// - `std::fs`: Used for directory reading.
/// - `std::path`: Used for manipulating file and directory paths.
fn read_dir(path: &str, ext: &str) -> Result<Vec<String>, Error> {
    let p = std::path::Path::new(path);
    if !p.exists() {
        return Err(Error::FolderNotFound);
    }

    let entries = std::fs::read_dir(p).map_err(|_| Error::AccessDenied)?;
    let mut paths = entries
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_file()
                    && path
                    .extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case(ext))
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

/// Concatenates media files in a specified directory into a single output file using FFmpeg.
///
/// # Parameters
///
/// - `path`: A string slice that holds the path to the directory containing the media files to concatenate.
/// - `ext`: A string slice that represents the file extension of the media files to be concatenated
///   (e.g., `"mp4"` for MP4 files).
///
/// # Returns
///
/// - `Result<bool, Error>`:
///   - `Ok(true)`: If the concatenation process completes successfully.
///   - `Ok(false)`: If the FFmpeg command executes successfully but indicates a failure (non-zero exit status).
///   - `Err(Error)`: If there is an error during file operations, reading the directory, or running the FFmpeg command.
///
/// # Errors
///
/// This function can return the following custom errors:
/// - `Error::CreateOutputError`: If the function fails to create the temporary file `file_list.txt`
///   for storing the file list used by FFmpeg.
/// - `Error::WriteFileError`: If the function fails to write to the temporary file `file_list.txt`.
/// - `Error::ReadDirError`: If there is an error reading the contents of the specified directory.
/// - `Error::CommandError`: If the FFmpeg command fails to execute (e.g., the executable is not found).
///
/// # Details
///
/// 1. The function generates a temporary file, `file_list.txt`, in the specified directory. This file contains
///    a list of the media files to be concatenated, formatted as required by FFmpeg (e.g., `file 'filename'`).
///
/// 2. It reads the directory specified by `path` and includes all files matching the given file extension (`ext`)
///    in the `file_list.txt`.
///
/// 3. It runs the FFmpeg command with the concat demuxer to create the final output file, `output.mp4`,
///    in the specified directory.
///
/// 4. The success of the FFmpeg process is returned as a boolean (`true` for success, `false` otherwise).
///
/// # Examples
///
/// ```rust
/// use crate::concat;
/// use crate::Error;
///
/// fn main() -> Result<(), Error> {
///     let path = "/path/to/directory";
///     let ext = "mp4";
///
///     let success = concat(path, ext)?;
///
///     if success {
///         println!("Successfully concatenated files into output.mp4");
///     } else {
///         eprintln!("FFmpeg command ran but failed to produce the desired output");
///     }
///
///     Ok(())
/// }
/// ```
///
/// # Notes
///
/// - This function requires FFmpeg to be installed and available in the system's PATH.
/// - All media files in the directory with the specified extension are included in the concatenation process.
///   Ensure that the files are in the correct order within the directory or rename them appropriately.
/// - The output file, `output.mp4`, will overwrite any existing file with the same name in the specified directory.
///
/// # Dependencies
///
/// - `std::fs::File`: For file creation and writing.
/// - `std::io::Write`: For writing to the temporary `file_list.txt`.
/// - `std::process::Command`: For executing the FFmpeg command.
/// - `std::result::Result`: For result handling.
/// - Custom `Error` type for error handling.
///
/// # Caveats
///
/// - This function assumes that the input media files are compatible with the FFmpeg concat demuxer (no re-encoding).
///   If the input files are not compatible, the FFmpeg process will fail.
/// - Ensure that the input files are named appropriately for the desired order of concatenation.
///
/// # Related
///
/// - [`std::fs::read_dir`](https://doc.rust-lang.org/std/fs/fn.read_dir.html): To list files in a directory.
/// - [FFmpeg concat documentation](https://ffmpeg.org/ffmpeg-formats.html#concat): Official documentation for
///   FFmpeg's concat demuxer.
pub fn concat(path: &str, ext: &str) -> Result<bool, Error> {
    let tmp_path = format!("{}/file_list.txt", path);
    let output_path = format!("{}/output.mp4", path);
    let mut f = File::create(&tmp_path).map_err(|_| Error::CreateOutputError)?;

    for file in read_dir(path, ext)? {
        writeln!(f, "file '{}'", file).map_err(|_| Error::WriteFileError)?;
    }

    // Step 2: run ffmpeg concat
    let status = Command::new("ffmpeg")
        .args([
            "-f", "concat",
            "-safe", "0",
            "-i", tmp_path.as_str(),
            "-c", "copy",
            output_path.as_str(),
        ])
        .status()
        .map_err(|_| Error::CommandError)?;

    Ok(status.success())
}
