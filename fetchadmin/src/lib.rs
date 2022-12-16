use git2::Repository;
use std::process::Command;
use tokio::time;

async fn fetchRepoFromGitCommand(remote: &str,branch: &str,path: &str) {
    let mut interval = time::interval(time::Duration::from_secs(1));
    loop {
        interval.tick().await;
        Command::new("git")
            .arg("fetch")
            .arg(remote)
            .arg(branch)
            .current_dir(path)
            .spawn()
            .expect("git fetch command failed to start");
        // tokio::spawn(print());
        // std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
// fetchRepoFromGitCommand("origin","master","/home/lan/repo/git/pulsar")

fn fetchRepoFromGitClient(){
    let repo = match Repository::open("/home/lan/repo/git/pulsar") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    fetch_origin_main(repo).unwrap();
}

fn fetch_origin_main(repo: git2::Repository) -> Result<(), git2::Error> {
    repo.find_remote("origin")?.fetch(&["master"], None, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
    
}
