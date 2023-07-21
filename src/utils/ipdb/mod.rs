use std::path::PathBuf;

use maxminddb::{Mmap, Reader};

pub fn get_reader() -> Reader<Mmap> {
    maxminddb::Reader::open_mmap(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/dbip-full.mmdb"))
        .unwrap()
}