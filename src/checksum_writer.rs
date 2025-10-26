use crate::Checksum;

pub struct ChecksummedWriter<W: std::io::Write> {
    inner: W,
    hasher: xxhash_rust::xxh3::Xxh3Default,
}

impl<W: std::io::Write> ChecksummedWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            inner: writer,
            hasher: xxhash_rust::xxh3::Xxh3Default::new(),
        }
    }

    pub fn checksum(&self) -> Checksum {
        Checksum::from_raw(self.hasher.digest128())
    }

    pub fn inner(&mut self) -> &mut W {
        &mut self.inner
    }
}

impl<W: std::io::Write> std::io::Write for ChecksummedWriter<W> {
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.hasher.update(buf);
        self.inner.write(buf)
    }
}
