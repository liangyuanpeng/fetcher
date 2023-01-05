#[macro_use]
extern crate serde_derive;
extern crate toml;

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

#[derive(Deserialize)]
#[derive(Debug)]
struct RepoInfo{
    name: String,
    remote: String,
    path: String,
    branch: String
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Config{    
    time: String,  // string or time?
    repoinfo: Option<Vec<RepoInfo>>
}

pub async fn fetchReposFromGitCommand() {
    let mut v: Vec<RepoInfo> = Vec::new();
    let r1 = RepoInfo{
            remote: String::from("origin"),
            name: String::from("origin"),
            path: String::from("origin"),
            branch: String::from("master"),
    };
    v.push(r1);

    let mut interval = time::interval(time::Duration::from_secs(1));
    loop {

        for i in &v{
            print!("{}",i.name)
        }

        Command::new("git")
            .arg("fetch")
            .arg("origin")
            .arg("master")
            .current_dir("/home/lan/repo/git/pulsar")
            .spawn()
            .expect("git fetch command failed to start");

        interval.tick().await;
    }
}

pub  fn fetchAllReposFromGitCommand(config: Config) {
    for x in config.repoinfo.unwrap() {
        println!("begin fetch from {:?}|{:?}|{:?}",x.path,x.remote,x.branch);
        Command::new("git")
            .arg("fetch")
            .arg(x.remote)
            .arg(x.branch)
            .current_dir(x.path)
            .spawn()
            .expect("git fetch command failed to start");
    }


    // Command::new("git")
    //         .arg("fetch")
    //         .arg("origin")
    //         .arg("master")
    //         .current_dir("/home/lan/repo/git/pulsar")
    //         .spawn()
    //         .expect("git fetch command failed to start");

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

use std::fs::File;
use std::io::prelude::*;

pub fn readConfig(file_path: &str){
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", file_path, e)
    };
    let mut str_val = String::new();
    file.read_to_string(&mut str_val).unwrap();

    let config: Config = toml::from_str(&str_val).unwrap();
 
    fetchAllReposFromGitCommand(config)

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
