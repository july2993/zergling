extern crate zergling;
use zergling::client::Client;
use zergling::util;
use zergling::storage::ReplicaPlacement;

mod common;

#[test]
fn simple_test() {
    util::init_log();
    let mut setter = common::Setter::new(19333);
    setter.add_vserver(18080);
    setter.add_vserver(18081);
    setter.start();

    do_upload_and_download(
        &setter.dir_addr(),
        1 << 10,
        ReplicaPlacement::new("000").unwrap(),
        true,
    );

    do_upload_and_download(
        &setter.dir_addr(),
        1 << 10,
        ReplicaPlacement::new("001").unwrap(),
        true,
    );

    setter.stop();
}

fn do_upload_and_download(dir_addr: &str, size: usize, rp: ReplicaPlacement, check_delete: bool) {
    let mut cli = Client::new(dir_addr).unwrap();
    let content: Vec<u8> = vec![66; size];

    let assign = cli.assign(&rp.string()).unwrap();

    println!("assign: {}", assign);

    let _upload = cli.upload_file_content(
        assign["url"].as_str().unwrap(),
        assign["fid"].as_str().unwrap(),
        &content,
    ).unwrap();

    println!("upload: {}", _upload);

    let lookup = cli.lookup(assign["fid"].as_str().unwrap()).unwrap();

    let locations = lookup["locations"].as_array().unwrap();

    assert_eq!(locations.len(), rp.get_copy_count() as usize);

    for location in locations.iter() {
        let url = location["url"].as_str().unwrap();
        let download = cli.get_file_content_by_fid(url, assign["fid"].as_str().unwrap())
            .unwrap();

        assert_eq!(content, download);
    }

    if !check_delete {
        return;
    }

    let _delete = cli.delete_by_fid(
        locations[0]["url"].as_str().unwrap(),
        assign["fid"].as_str().unwrap(),
    ).unwrap();

    // should be delete
    for location in locations.iter() {
        let url = location["url"].as_str().unwrap();
        let download = cli.get_file_content_by_fid(url, assign["fid"].as_str().unwrap())
            .unwrap();

        assert!(content != download);
    }
}
