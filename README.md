# Git - Copy files between 2 commits

Copy the files changed between two commits to a destination directory, ignoring the specified extensions.

## Usage

```bash
git-copy <start_commit> <end_commit> <dest_dir> [ignore_ext]
```

## Examples

```bash
# Will copy all files found between the commits, except .txt and .md
git-copy 1234567890 9876543210 ./dest_dir txt,md
```

```bash
# Will copy all files found between the commits
git-copy 1234567890 9876543210 ./dest_dir
```
