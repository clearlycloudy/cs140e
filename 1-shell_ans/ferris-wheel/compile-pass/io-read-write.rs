// FIXME: Make me compile! Diff budget: 2 lines.
use std::io;

struct ReadWrapper<T: io::Read> {
    inner: T
}

impl < T > io::Read for ReadWrapper<T> where T: io::Read {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

fn main() { }
