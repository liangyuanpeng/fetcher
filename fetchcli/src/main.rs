use tokio as tokio1;

use fetchadmin;

// #[tokio1::main(crate = "tokio1")]
// async fn main() {
//     println!("Hello world");
// }

use tokio::task;
use std::fs::File;


fn main() {

    fetchadmin::readConfig("/home/lan/repo/git/fetcher/fetchadmin/fetchadmin.toml");
    
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {

            // let file_path = "sample.toml";
            // let mut file = match File::open(file_path) {
            //     Ok(f) => f,
            //     Err(e) => panic!("no such file {} exception:{}", file_path, e)
            // };

<<<<<<< HEAD
            // task::block_in_place(move || {
            //     fetchadmin::fetchRepoFromGitCommand("origin","master","/home/lan/repo/git/pulsar");
            // });

            fetchadmin::fetchReposFromGitCommand().await;
=======
            // let file_path = "sample.toml";
            // let mut file = match File::open(file_path) {
            //     Ok(f) => f,
            //     Err(e) => panic!("no such file {} exception:{}", file_path, e)
            // };

            // task::block_in_place(move || {
            //     fetchadmin::fetchRepoFromGitCommand("origin","master","/home/lan/repo/git/pulsar");
            // });
>>>>>>> 59ed3d88fdc4c50f861e63f1375c66bf9c374143

            println!("Hello world");
            let (tx, rx) = channel();
    
            ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
                .expect("Error setting Ctrl-C handler");
            
            println!("Waiting for Ctrl-C...");
            rx.recv().expect("Could not receive from channel.");
            println!("Got it! Exiting..."); 
        })
}

//doc https://docs.rs/tokio/latest/tokio/attr.main.html

use std::sync::mpsc::channel;
use ctrlc;

// fn main(){
//     let (tx, rx) = channel();
    
//     ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
//         .expect("Error setting Ctrl-C handler");
    
//     println!("Waiting for Ctrl-C...");
//     rx.recv().expect("Could not receive from channel.");
//     println!("Got it! Exiting..."); 
// }