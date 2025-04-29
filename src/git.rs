use git2::{Commit as GitCommit, Error as GitError, Oid, Repository as GitRepository};

#[allow(dead_code)]
pub(crate) fn commits_of_branch(
    repository: &GitRepository,
    branch: Oid,
) -> Result<Vec<GitCommit<'_>>, GitError> {
    let mut revwalk = repository.revwalk()?;

    revwalk.push(branch)?;

    revwalk.simplify_first_parent()?;

    Ok(revwalk
        .flatten()
        .map(|oid| repository.find_commit(oid).unwrap())
        .collect())
}

pub(crate) fn commits_in_range(
    repository: &GitRepository,
    range: (Oid, Oid),
) -> Result<Vec<GitCommit<'_>>, GitError> {
    let mut revwalk = repository.revwalk()?;

    revwalk.hide(range.0)?;
    revwalk.push(range.1)?;

    revwalk.simplify_first_parent()?;

    Ok(revwalk
        .flatten()
        .map(|oid| repository.find_commit(oid).unwrap())
        .collect())
}
