use tokio as tokio1;



// #[tokio1::main(crate = "tokio1")]
// async fn main() {
//     println!("Hello world");
// }

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        // .start_paused(true)
        .build()
        .unwrap()
        .block_on(async {
            println!("Hello world");
        })
}

//doc https://docs.rs/tokio/latest/tokio/attr.main.html