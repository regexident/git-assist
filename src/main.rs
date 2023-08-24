use args::bisect::Command::SkipPullRequests;
use clap::Parser;

use self::args::*;

mod args;

// Github's "Rebase and Merge" action ends up modifying the PR's commits
// which causes their associated SHAs to change, too.
// https://github.com/orgs/community/discussions/5524
// As such the `pr.head.sha` will not match the equivalent merged commit's SHA.

// Afaict “all” you’d need to do to make git bisect work with “rebase and merge”
// (i.e. skip PR-internal commits) is to run git bisect skip BASE..MERGE^
// for each merged pull request (where BASE is the PR’s base commit and HEAD
// is the PR’s merge commit), before providing git `bisect good <REV>`/`git bisect bad <REV>`.
// After that it will automatically skip internal commits while

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let Some(args_command) = args.command else {
        anyhow::bail!("No command specified");
    };

    let result = match args_command {
        Command::Bisect(SkipPullRequests(command)) => command.run().await,
    };

    if let Ok(exit_status) = &result {
        if let Some(code) = exit_status.code() {
            std::process::exit(code);
        }
    };

    result.map(|_exit_status| ())
}
