mod args;
use args::Args;
use std::fs;
use std::process::Command;

fn main() {
    let args = Args::parse();

    let files = get_changed_files(&args.start_commit, &args.end_commit);
    fs::create_dir_all(&args.dest_dir).expect("Failed to create destination directory");
    copy_files(files, &args.dest_dir, args.ignore_exts.as_ref());
}

/// Get the list of changed files between two commits
fn get_changed_files(start_commit: &str, end_commit: &str) -> Vec<String> {
    let output = Command::new("git")
        .args(["diff", "--name-only", start_commit, end_commit])
        .output()
        .expect("Failed to execute git diff command");

    if !output.status.success() {
        eprintln!(
            "Git command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(String::from)
        .collect()
}

/// Copy the files to the destination directory, ignoring the specified extensions
fn copy_files(files: Vec<String>, dest_dir: &String, ignore_exts: Option<&Vec<String>>) {
    files
        .iter()
        .filter(|file| ignore_exts.map_or(true, |exts| !exts.iter().any(|ext| file.ends_with(ext))))
        .for_each(|file| {
            let dest_path = format!("{}/{}", dest_dir, file);

            std::path::Path::new(&dest_path)
                .parent()
                .map(fs::create_dir_all)
                .transpose()
                .expect("Failed to create parent directories");

            fs::copy(file, &dest_path).unwrap_or_else(|e| {
                eprintln!("Failed to copy file {}: {}", file, e);
                std::process::exit(1);
            });
        });
}
