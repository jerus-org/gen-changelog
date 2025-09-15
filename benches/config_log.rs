use criterion::{Criterion, criterion_group, criterion_main};

use gen_changelog::ChangeLog;
use lazy_regex::{Lazy, Regex, lazy_regex};

/// Regular expression pattern for matching GitHub repository URLs.
///
/// Supports both HTTPS and SSH formats:
/// - HTTPS: `https://github.com/owner/repo.git`
/// - SSH: `git@github.com:owner/repo.git`
///
/// Captures named groups:
/// - `owner`: The repository owner/organization name
/// - `repo`: The repository name
static REMOTE: Lazy<Regex> = lazy_regex!(
    r"^((https://github\.com/)|(git@github.com:))(?P<owner>[a-z\-|A-Z]+)/(?P<repo>[a-z\-_A-Z]+)\.git$$"
);

fn benchmark_changelog_creation(c: &mut Criterion) {
    c.bench_function("changelog_creation", |b| {
        b.iter(|| {
            ChangeLog::builder()
                .with_header("Test", &["Description"])
                .build()
        })
    });
}

fn benchmark_remote_regex(c: &mut Criterion) {
    let urls = vec![
        "https://github.com/user/repo.git",
        "git@github.com:org/project.git",
        "https://github.com/complex-org/complex-repo-name.git",
    ];

    c.bench_function("remote_regex_matching", |b| {
        b.iter(|| {
            for url in &urls {
                let _ = REMOTE.captures(url);
            }
        })
    });
}

criterion_group!(
    benches,
    benchmark_changelog_creation,
    benchmark_remote_regex
);
criterion_main!(benches);
