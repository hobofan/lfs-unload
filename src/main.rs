use duct::cmd;
use std::collections::HashSet;
use std::path::PathBuf;

fn main() {
    let git_root = cmd!("git", "rev-parse", "--show-toplevel")
        .stdout_capture()
        .read()
        .unwrap();
    let git_root = PathBuf::from(git_root);
    let cwd = std::env::current_dir().unwrap();

    if git_root != cwd {
        println!("Can only run lfs-unload from git root");
        std::process::exit(1);
    }

    let git_is_clean = cmd!("git", "status", "--porcelain")
        .stdout_capture()
        .read()
        .unwrap();
    let git_is_clean = git_is_clean
        .lines()
        .filter(|n| !n.contains(".gitattributes"))
        .collect::<Vec<_>>();
    if !git_is_clean.is_empty() {
        println!("Can only run lfs-unload in clean working directory");
        std::process::exit(1);
    }

    let path_arg = std::env::args()
        .nth(1)
        .expect("Path argument not specified");
    let git_files = cmd!("git", "ls-files", &path_arg)
        .stdout_capture()
        .read()
        .unwrap();
    let git_files = git_files.lines().collect::<HashSet<_>>();
    let git_lfs_files = cmd!("git", "lfs", "ls-files", "--name-only")
        .stdout_capture()
        .read()
        .unwrap();
    let git_lfs_files = git_lfs_files.lines().collect::<HashSet<_>>();

    let selected_files = git_files.intersection(&git_lfs_files).collect::<Vec<_>>();

    for file in selected_files.iter() {
        cmd!(
            "sh",
            "-c",
            &format!("git show HEAD:'{path}' > '{path}'", path = file)
        )
        .run()
        .unwrap();

        let pointer_contents = std::fs::read_to_string(file).unwrap();
        let pointer_hash = pointer_contents
            .lines()
            .find(|n| n.contains("oid"))
            .unwrap()
            .replace("oid sha256:", "");
        let lfs_object_path = format!(
            ".git/lfs/objects/{pre1}/{pre2}/{hash}",
            pre1 = &pointer_hash[0..2],
            pre2 = &pointer_hash[2..4],
            hash = pointer_hash
        );
        let lfs_object_path = PathBuf::from(lfs_object_path);

        if lfs_object_path.exists() {
            std::fs::remove_file(lfs_object_path).unwrap();
        }
        println!("Removed local copy of {}", file);
    }
    cmd!("git", "add", ".",).run().unwrap();

    let current_exclude = cmd!("git", "config", "lfs.fetchexclude")
        .stdout_capture()
        .read()
        .unwrap();
    cmd!(
        "git",
        "config",
        "lfs.fetchexclude",
        &format!("{},{}", current_exclude, path_arg),
    )
    .run()
    .unwrap();
}
