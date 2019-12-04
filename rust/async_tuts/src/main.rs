use futures;
use futures::executor::block_on;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
struct Song {
    lyrics: String,
}

async fn learn_song() -> Song {
    sleep(Duration::new(5, 0));
    Song { lyrics: String::from("Whateva whateva, yea yea yea") }
}

async fn sing_song(song: Song) {
    println!("{:#?}", song);
}

async fn dance() {
    println!("I'm gonna dance!");
}

//async fn learn_song() -> Song { }
//async fn sing_song(song: Song) { }
//async fn dance() { }

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    // note what happens to elision if you remove learn_song's .await...
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    // note what happens to elision if you remove learn_and_sing's .await...
    let f1 = learn_and_sing().await;
    let f2 = dance().await;

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main())
}
