use sfa::Writer;
use std::io::Write;
use xxhash_rust::xxh3::xxh3_128;

#[test]
pub fn full_file_checksum() -> Result<(), sfa::Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("chksum");

    let mut writer = Writer::new_at_path(&path)?;
    writer.start("Hello")?;
    writer.write_all(b"World")?;
    let checksum = writer.finish()?;
    let checksum = checksum.into_u128();

    let file_contents = std::fs::read(&path)?;
    let real_checksum = xxh3_128(&file_contents);

    assert_eq!(checksum, real_checksum, "checksum mismatch");

    Ok(())
}
