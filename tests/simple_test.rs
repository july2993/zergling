extern crate zergling;
use zergling::client::Client;
use zergling::util;

mod common;


#[test]
#[ignore]
fn upload_and_download_one_1b() {
    do_upload_and_download(1);
}

#[test]
#[ignore]
fn upload_and_download_1k() {
    do_upload_and_download(1 << 10);
}

#[test]
fn upload_and_download_1m() {
    do_upload_and_download(1 << 20);
}

#[test]
#[ignore]
fn upload_and_download_100m() {
    do_upload_and_download(100 * (1 << 20));
}

fn do_upload_and_download(size: usize) {
    util::init_log();
    let mut setter = common::Setter::new();
    setter.start();

    let mut cli = Client::new("127.0.0.1:9333").unwrap();
    let content: Vec<u8> = vec![66; size];

    let assign = cli.assign().unwrap();

    println!("assign: {}", assign);

    let _upload = cli.upload_file_content(
        assign["url"].as_str().unwrap(),
        assign["fid"].as_str().unwrap(),
        &content,
    ).unwrap();

    println!("upload: {}", _upload);

    let download = cli.get_file_content_by_fid(
        assign["url"].as_str().unwrap(),
        assign["fid"].as_str().unwrap(),
    ).unwrap();

    setter.stop();
    assert_eq!(content, download);
}
