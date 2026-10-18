use std::fs;
use std::io::Write;

use super::extract_zip;

fn write_zip(path: &std::path::Path, name: &str, bytes: &[u8]) {
    let mut writer = zip::ZipWriter::new(fs::File::create(path).unwrap());
    writer
        .start_file(name, zip::write::SimpleFileOptions::default())
        .unwrap();
    writer.write_all(bytes).unwrap();
    writer.finish().unwrap();
}

#[test]
fn nested_zip_entry_preserves_file_bytes() {
    let temp = tempfile::tempdir().unwrap();
    let archive = temp.path().join("plugin.zip");
    let destination = temp.path().join("unpacked");
    write_zip(
        &archive,
        "plugin/nested/config.json",
        b"{\"enabled\":true}\n",
    );

    extract_zip(&archive, &destination).unwrap();

    assert_eq!(
        fs::read(destination.join("plugin/nested/config.json")).unwrap(),
        b"{\"enabled\":true}\n"
    );
}

#[test]
fn traversal_zip_entry_cannot_replace_an_existing_outside_file() {
    let temp = tempfile::tempdir().unwrap();
    let archive = temp.path().join("plugin.zip");
    let destination = temp.path().join("unpacked");
    let outside = temp.path().join("outside.txt");
    fs::write(&outside, b"keep").unwrap();
    write_zip(&archive, "../outside.txt", b"replace");

    let error = extract_zip(&archive, &destination).unwrap_err();

    assert!(error.to_string().contains("unsafe path"));
    assert_eq!(fs::read(outside).unwrap(), b"keep");
}
