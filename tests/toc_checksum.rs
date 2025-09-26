use tft::Reader;

#[test]
pub fn cherry_pie_invalid_checksum() -> Result<(), tft::Error> {
    let path = "test_fixture/cherry_pie_broken.tft";

    assert!(matches!(
        Reader::new(path),
        Err(tft::Error::ChecksumMismatch { .. })
    ));

    Ok(())
}
