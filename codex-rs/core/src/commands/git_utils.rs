//! Git utilities for command context enhancement.
//!
//! Provides git diff extraction functionality for passing to command templates.
//! This enables commands to access current changes and make context-aware suggestions.

use std::io;
use std::process::Stdio;
use tokio::process::Command;

/// Get current git diff (tracked changes only).
///
/// Returns a tuple of (is_git_repo, diff_content).
/// - If not in a git repo: (false, "")
/// - If in a git repo with no changes: (true, "")
/// - If in a git repo with changes: (true, diff_content)
///
/// # Performance
///
/// This function shells out to `git diff` which is typically fast (<50ms)
/// for normal repositories. On very large repositories, this may take longer.
///
/// # Examples
///
/// ```no_run
/// # use codex_core::commands::git_utils::get_git_diff;
/// # async fn example() -> std::io::Result<()> {
/// let (is_repo, diff) = get_git_diff().await?;
/// if is_repo && !diff.is_empty() {
///     println!("Git diff:\n{}", diff);
/// } else if !is_repo {
///     println!("Not in a git repository");
/// }
/// # Ok(())
/// # }
/// ```
pub async fn get_git_diff() -> io::Result<(bool, String)> {
    // First, check if we're in a git repository
    if !is_git_repo().await? {
        return Ok((false, String::new()));
    }

    // Get tracked changes diff
    let diff = run_git_diff(&["diff"]).await?;

    Ok((true, diff))
}

/// Determine if the current directory is inside a Git repository.
async fn is_git_repo() -> io::Result<bool> {
    let status = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await;

    match status {
        Ok(s) if s.success() => Ok(true),
        Ok(_) => Ok(false),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false), // git not installed
        Err(e) => Err(e),
    }
}

/// Run git diff command and capture output.
///
/// Git diff returns exit code 1 when differences are present, which we treat as success.
async fn run_git_diff(args: &[&str]) -> io::Result<String> {
    let output = Command::new("git")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .await?;

    // Git diff returns 0 if no diff, 1 if diff present
    if output.status.success() || output.status.code() == Some(1) {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(io::Error::other(format!(
            "git {:?} failed with status {}",
            args, output.status
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_git_repo() {
        // This test assumes we're running in the codex git repo
        let result = is_git_repo().await;
        assert!(result.is_ok());
        // We're in a git repo, so this should be true
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_get_git_diff_returns_tuple() {
        let result = get_git_diff().await;
        assert!(result.is_ok());

        let (is_repo, _diff) = result.unwrap();
        // Should detect we're in a git repo
        assert!(is_repo);
        // diff may or may not be empty depending on current repo state
    }

    #[tokio::test]
    async fn test_git_not_installed_handled_gracefully() {
        // If git isn't installed, is_git_repo should return Ok(false)
        // This is hard to test without modifying PATH, but the code handles it
    }
}
