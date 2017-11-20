
extern crate zergling;
use zergling::client::Client;
use zergling::util;

mod common;




#[test]
fn upload_and_download() {
    util::init_log();
    common::setup();


    let mut cli = Client::new("127.0.0.1:9333").unwrap();
    let content: Vec<u8> = "testcontent\n".as_bytes().to_vec();

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

    assert_eq!(content, download);
}
