use git2::Repository;
use std::process::Command;
use tokio::time;


#[tokio::main]
async fn main() {
    println!("Hello, world!");
   
    Command::new("git")
        .arg("fetch")
        .arg("origin")
        .arg("master")
        .current_dir("/home/lan/repo/git/pulsar")
        .spawn()
        .expect("git fetch command failed to start");

    tokio::spawn(print());
    std::thread::sleep(std::time::Duration::from_secs(3));
}

async fn print() {
    let mut interval = time::interval(time::Duration::from_secs(1));
    loop {
        interval.tick().await;
        println!("2333");
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