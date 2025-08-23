// use std::path::{Path, PathBuf};
// use std::sync::mpsc::{self, channel};
// use std::{fs, io, thread};

// use rand::seq::index;
// fn main() {
//     //
// }

// /// 读取文件 把文件传入管道
// fn start_file_reader_thread(
//     documents: Vec<PathBuf>,
// ) -> (mpsc::Receiver<String>, thread::JoinHandle<io::Result<()>>) {
//     let (tx, rx) = mpsc::channel();
//     let handle = thread::spawn(move || {
//         for filename in documents {
//             let text = fs::read_to_string(filename)?;
//             if tx.send(text).is_err() {
//                 break;
//             }
//         }
//         Ok(())
//     });
//     (rx, handle)
// }

// //接受另一个通道的的文件进行编号 然后发送给下一个通道

// fn start_file_indexing_thread(
//     texts: mpsc::Receiver<String>,
// ) -> (mpsc::Receiver<InMemoryIndex>, thread::JoinHandle<()>) {
//     //
//     let (tx, rx) = channel();

//     let handle = thread::spawn(move || {
//         //
//         for (doc_id, text) in texts.into_iter().enumerate() {
//             let idx = InMeomoryIndex::from_single_document(doc_id, text);

//             if tx.send(idx).is_err() {
//                 break;
//             }
//         }
//     });
//     (rx, handle)
// }

// //在内存中合并index
// fn start_file_memory_thread(
//     file_indexes: mpsc::Receiver<InMemoryIndex>,
// ) -> (mpsc::Receiver<InMeoryIndex>, thread::JoinHandle<()>) {
//     //
// }

// //写入磁盘
// fn start_file_writing_thread(
//     big_indexes: mpsc::Receiver<InMemoryIndex>,
//     output_dir: &Path,
// ) -> (mpsc::Receiver<PathBuf>, thread::JoinHandle<()>) {
//     //
// }

// //合并多个大文件
// fn merge_index_files(files: mpsc::Receiver<PathBuf>, output_dir: &Path) -> io::Result<()> {
//     //
// }

// fn run_pipeline(documents: Vec<PathBuf>, output_dir: &Path) -> io::Result<()> {
//     let (texts, handle1) = start_file_reader_thread(documents);
//     let (idx, handle2) = start_file_indexing_thread(texts);

//     let (merge, handle3) = start_file_memory_thread(idx);
//     let (files, handle4) = start_file_writing_thread(merge, &output_dir);

//     let result = merge_index_files(files, &output_dir);
//     //等待以上的所有线程集合
//     let err1 = handle1.join().unwrap();
//     handle2.join().unwrap();
//     handle3.join().unwrap();
//     let err2 = handle4.join().unwrap();
//     err1?;
//     err2?;
//     result
// }
// use std::ops::Sub;
// use std::sync::atomic::{AtomicU64, Ordering};
// use std::thread::{self, JoinHandle};
// use std::time::Instant;

// const N_TIMES: u64 = 10000000;
// const N_THREADS: usize = 10;

// static R: AtomicU64 = AtomicU64::new(0);

// fn add_n_times(n: u64) -> JoinHandle<()> {
//     thread::spawn(move || {
//         for _ in 0..n {
//             R.fetch_add(1, Ordering::Relaxed);
//         }
//     })
// }

// fn main() {
//     let s = Instant::now();
//     let mut threads = Vec::with_capacity(N_THREADS);

//     for _ in 0..N_THREADS {
//         threads.push(add_n_times(N_TIMES));
//     }

//     for thread in threads {
//         thread.join().unwrap();
//     }

//     assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));

//     println!("{:?}", Instant::now().sub(s));
// }

use std::io::{self};
use std::net::TcpListener;
use std::thread;
mod client;
mod handle;
mod server;
fn main() -> io::Result<()> {
    //
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("server is listening 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                //
                thread::spawn(|| {
                    if let Err(e) = handle::handle_client(stream) {
                        eprintln!("handle client error: {}", e);
                    }
                });
            }
            Err(e) => {
                //
                eprintln!("accept connect error: {}", e);
            }
        }
    }

    Ok(())
}
