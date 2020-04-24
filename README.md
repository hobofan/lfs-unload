## lfs-unload

Small utility to unload local files from git-lfs.

Relevant issues for a proper implementation:

- <https://github.com/git-lfs/git-lfs/issues/1189>
- <https://github.com/git-lfs/git-lfs/issues/951>

## Installation

```
cargo install --git https://github.com/hobofan/lfs-unload.git
```

## Usage

Before running the first time, run `git config lfs.fetchexclude __` (where `__` is a dummy path).
```
lfs-unload <FILEPATH>
```

Where `<FILEPATH>` can be any file/directory path as understood by `git ls-files <FILEPATH>`.

What the program does:

- Ensure that you are running the command from the git root
- Ensure that you are working from a clean working directory
- Replaces all lfs files matching `<FILEPATH>` with their pointers
- Removes the lfs object for the files from `.git/lfs/objects`
- Runs `git add .` so that the files that were just replace with their pointers disappear from `git status`
- Adds `<FILEPATH>` to `git config lfs.fetchexclude`, so the files won't be accidentally retrieved again from remote on the next pull
