use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};

use log::LevelFilter;
use tempfile::TempDir;

pub(crate) fn get_test_logger() {
    let mut builder = env_logger::Builder::new();
    builder.filter(None, LevelFilter::Debug);
    builder.format_timestamp_secs().format_module_path(false);
    let _ = builder.try_init();
}

/// Process-wide lock serializing tests that change the current directory.
///
/// The current directory is process-global, so tests that mutate it (or read
/// files relative to it) must not run concurrently. Poisoning is recovered from
/// so a single panicking test does not disable the rest.
static CWD_LOCK: Mutex<()> = Mutex::new(());

/// RAII guard that restores the original working directory on drop.
struct CwdGuard {
    _lock: MutexGuard<'static, ()>,
    original: PathBuf,
    // Kept alive for the duration of the guard; removed after the CWD is
    // restored so the temp dir is never the current directory when deleted.
    _temp: TempDir,
}

impl Drop for CwdGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.original);
    }
}

/// Runs `f` inside a fresh temporary directory as the current working
/// directory, serialized against other current-directory-sensitive tests.
///
/// Use this for any test that reads or writes files via a path relative to the
/// current directory (e.g. [`ChangeLog::save`](crate::ChangeLog) with a bare
/// filename, or [`ChangeLogConfig::from_file_or_default`](crate::ChangeLogConfig))
/// so tests never touch files in the crate directory. The original working
/// directory is restored even if `f` panics.
pub(crate) fn with_isolated_cwd<T>(f: impl FnOnce(&Path) -> T) -> T {
    let lock = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let original = std::env::current_dir().expect("read current dir");
    let temp = TempDir::new().expect("create temp dir");
    std::env::set_current_dir(temp.path()).expect("set current dir to temp");

    let guard = CwdGuard {
        _lock: lock,
        original,
        _temp: temp,
    };
    // Resolve the temp path through the guard so the borrow is valid for `f`.
    let temp_path = guard._temp.path().to_path_buf();
    f(&temp_path)
    // `guard` drops here: CWD restored, then temp dir removed, then lock freed.
}
