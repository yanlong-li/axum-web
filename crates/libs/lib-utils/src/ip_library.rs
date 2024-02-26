use std::env;

use maxminddb::{Mmap, Reader};

pub fn get_reader() -> Reader<Mmap> {
    let mmdb_file = env::var("MMDB_FILE").expect("无法获取mmdb文件");
    maxminddb::Reader::open_mmap(mmdb_file)
        .unwrap()
}