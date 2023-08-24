use git2::{Commit as GitCommit, Error as GitError, Oid, Repository as GitRepository};

pub(crate) fn commits_of_branch<'a>(
    repository: &'a GitRepository,
    branch: Oid,
) -> Result<Vec<GitCommit<'a>>, GitError> {
    let mut revwalk = repository.revwalk()?;

    revwalk.push(branch)?;

    revwalk.simplify_first_parent()?;

    Ok(revwalk
        .flatten()
        .map(|oid| repository.find_commit(oid).unwrap())
        .collect())
}

pub(crate) fn commits_in_range<'a>(
    repository: &'a GitRepository,
    range: (Oid, Oid),
) -> Result<Vec<GitCommit<'a>>, GitError> {
    let mut revwalk = repository.revwalk()?;

    revwalk.hide(range.0)?;
    revwalk.push(range.1)?;

    revwalk.simplify_first_parent()?;

    Ok(revwalk
        .flatten()
        .map(|oid| repository.find_commit(oid).unwrap())
        .collect())
}
