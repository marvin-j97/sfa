use std::io::{Read, Write};
use tft::{Reader, Writer};

#[test]
pub fn cherry_pie() -> Result<(), tft::Error> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("cherry_pie.tft");

    let mut writer = Writer::new_at_path(&path)?;
    writer.start("Verse 1")?;
    writer.write_all(b"Glazed eyes and cherry pie\n")?;
    writer.write_all(b"We are high spirits in those L.A. skies\n")?;
    writer.write_all(b"Griffith Lookout, look out\n")?;
    writer.write_all(b"The gates will shut and the sky will glow dark\n")?;
    writer.write_all(b"Can you see it changing?\n")?;
    writer.start("Chorus")?;
    writer.write_all(b"Youth is running out, we finally feel it now\n")?;
    writer
        .write_all(b"The years from here get faster as the lights keep blurring past this car\n")?;
    writer.write_all(b"My mind is changing, forever is fading\n")?;
    writer.write_all(b"So we search for something else\n")?;
    writer.write_all(b"And as we watch the sun go down\n")?;
    writer.write_all(b"Everything that we knew then we don't know now\n")?;
    writer.write_all(b"There's a silence breathing through the car\n")?;
    writer.write_all(b"My mind is changing, forever is fading\n")?;
    writer.write_all(b"So we search for something else\n")?;
    writer.start("Verse 2")?;
    writer.write_all(b"Your phone glow face in the dark\n")?;
    writer.write_all(b"Lights up the living room and our whispers start\n")?;
    writer.write_all(b"Can you sleep well? Nor can I\n")?;
    writer.write_all(b"Should we just talk all night and compare our minds?\n")?;
    writer.write_all(b"Do you feel the same thing?\n")?;
    writer.start("Chorus 2")?;
    writer.write_all(b"Youth is running out, we finally feel it now\n")?;
    writer
        .write_all(b"The years from here get faster as the lights keep blurring past this car\n")?;
    writer.write_all(b"My mind is changing, forever is fading\n")?;
    writer.write_all(b"So we search for something else\n")?;
    writer.write_all(b"And as we watch the sun go down\n")?;
    writer.write_all(b"Everything that we knew then we don't know now\n")?;
    writer.write_all(b"There's a silence breathing through the car\n")?;
    writer.write_all(b"My mind is changing, forever is fading\n")?;
    writer.write_all(b"So we search for something else\n")?;
    writer.start("Outro")?;
    writer.write_all(b"There's a hush now in our hearts\n")?;
    writer.write_all(b"There's a hush now, it glows dark\n")?;
    writer.write_all(b"There's a hush now in our hearts\n")?;
    writer.write_all(b"There's a hush now, it glows dark (Do you feel the same thing?)\n")?;
    writer.write_all(b"There's a hush now in our hearts (All the lights keep blurring)\n")?;
    writer.write_all(b"There's a hush now, it glows dark (There's a silence breathing)\n")?;
    writer.write_all(b"There's a hush now in our hearts (Can you see it changing?)\n")?;
    writer.write_all(b"There's a hush now\n")?;
    writer.finish()?;

    let reader = Reader::new(&path)?;
    {
        let toc = reader.toc();
        assert_eq!(5, toc.len());
        assert!(!toc.is_empty());

        assert_eq!(b"Verse 1", toc[0].name());
        let bytes = toc[0]
            .buf_reader(&path)?
            .bytes()
            .collect::<std::io::Result<Vec<_>>>()?;
        assert_eq!(bytes, b"Glazed eyes and cherry pie\nWe are high spirits in those L.A. skies\nGriffith Lookout, look out\nThe gates will shut and the sky will glow dark\nCan you see it changing?\n");

        assert_eq!(b"Chorus", toc[1].name());
        let bytes = toc[1]
            .buf_reader(&path)?
            .bytes()
            .collect::<std::io::Result<Vec<_>>>()?;
        assert_eq!(bytes, b"Youth is running out, we finally feel it now\nThe years from here get faster as the lights keep blurring past this car\nMy mind is changing, forever is fading\nSo we search for something else\nAnd as we watch the sun go down\nEverything that we knew then we don't know now\nThere's a silence breathing through the car\nMy mind is changing, forever is fading\nSo we search for something else\n");

        assert_eq!(b"Verse 2", toc[2].name());
        let bytes = toc[2]
            .buf_reader(&path)?
            .bytes()
            .collect::<std::io::Result<Vec<_>>>()?;

        assert_eq!(
            bytes,
            b"Your phone glow face in the dark\nLights up the living room and our whispers start\nCan you sleep well? Nor can I\nShould we just talk all night and compare our minds?\nDo you feel the same thing?\n"
        );

        assert_eq!(b"Chorus 2", toc[3].name());
        let bytes = toc[3]
            .buf_reader(&path)?
            .bytes()
            .collect::<std::io::Result<Vec<_>>>()?;
        assert_eq!(bytes, b"Youth is running out, we finally feel it now\nThe years from here get faster as the lights keep blurring past this car\nMy mind is changing, forever is fading\nSo we search for something else\nAnd as we watch the sun go down\nEverything that we knew then we don't know now\nThere's a silence breathing through the car\nMy mind is changing, forever is fading\nSo we search for something else\n");

        assert_eq!(b"Outro", toc[4].name());
        let bytes = toc[4]
            .buf_reader(&path)?
            .bytes()
            .collect::<std::io::Result<Vec<_>>>()?;
        assert_eq!(bytes, b"There's a hush now in our hearts\nThere's a hush now, it glows dark\nThere's a hush now in our hearts\nThere's a hush now, it glows dark (Do you feel the same thing?)\nThere's a hush now in our hearts (All the lights keep blurring)\nThere's a hush now, it glows dark (There's a silence breathing)\nThere's a hush now in our hearts (Can you see it changing?)\nThere's a hush now\n");
    }

    Ok(())
}
