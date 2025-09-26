use sfa::Reader;

#[test]
pub fn cherry_pie_invalid_checksum() -> Result<(), sfa::Error> {
    let path = "test_fixture/cherry_pie_broken";

    assert!(matches!(
        Reader::new(path),
        Err(sfa::Error::ChecksumMismatch { .. })
    ));

    Ok(())
}
