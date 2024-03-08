use git2::Repository;

use std::process::Command;
use tokio::time;

pub async fn fetchRepoFromGitCommand(remote: &str,branch: &str,path: &str) {
        let mut interval = time::interval(time::Duration::from_secs(1));
        interval.tick().await;
        loop {
            print!("hello fetchRepoFromGitCommand {}",path);
            Command::new("git")
                .arg("fetch")
                .arg(remote)
                .arg(branch)
                .current_dir(path)
                .spawn()
                .expect("git fetch command failed to start");
                interval.tick().await;
        }
}

struct RepoInfo{
    name: String,
    remote: String,
    path: String,
}

pub async fn fetchReposFromGitCommand() {
    let mut v: Vec<RepoInfo> = Vec::new();
    let r1 = RepoInfo{
        remote: String::from("origin"),
        name: String::from("main"),
        path: String::from("H:\\repo\\git\\oras-go"),
    };
    let r2 = RepoInfo{
        remote: String::from("origin"),
        name: String::from("master"),
        path: String::from("H:\\repo\\git\\pulsar"),
    };
    let r3 = RepoInfo{
        remote: String::from("origin"),
        name: String::from("master"),
        path: String::from("H:\\repo\\git\\bookkeeper"),
    };
    let r4 = RepoInfo{
        remote: String::from("origin"),
        name: String::from("master"),
        path: String::from("H:\\repo\\git\\kafeidou"),
    };
    let r5 = RepoInfo{
        remote: String::from("origin"),
        name: String::from("main"),
        path: String::from("H:\\repo\\git\\maquan"),
    };
    v.push(r1);
    v.push(r2);
    v.push(r3);

    let mut interval = time::interval(time::Duration::from_secs(60*10));
    loop {

        interval.tick().await;

        for i in &v{
            print!("{}",i.name);
            Command::new("git")
            .arg("fetch")
            .arg(i.remote.to_string())
            .arg(i.name.to_string())
            .current_dir(i.path.to_string())
            .spawn()
            .expect("git fetch command failed to start");
        }

        // Command::new("git")
        //     .arg("fetch")
        //     .arg("origin")
        //     .arg("master")
        //     .current_dir("/home/lan/repo/git/pulsar")
        //     .spawn()
        //     .expect("git fetch command failed to start");

        

    }
}

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
