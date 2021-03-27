use md5::{Digest, Md5};
use ring::digest;
use std::{
    fmt,
    io::{self, prelude::*},
};

trait Hasher {
    fn update(&mut self, data: &[u8]);
    fn finish(self) -> String;

    fn calculate<R: BufRead>(mut self, reader: &mut R) -> io::Result<String>
    where
        Self: Sized,
    {
        let mut size_read;

        loop {
            let buffer = reader.fill_buf()?;
            if buffer.is_empty() {
                break;
            }
            size_read = buffer.len();
            self.update(buffer);
            reader.consume(size_read);
        }

        Ok(self.finish())
    }
}

impl Hasher for digest::Context {
    fn update(&mut self, data: &[u8]) { self.update(data) }

    fn finish(self) -> String {
        data_encoding::HEXLOWER.encode(self.finish().as_ref())
    }
}

impl Hasher for Md5 {
    fn update(&mut self, data: &[u8]) {
        <Md5 as md5::Digest>::update(self, data)
    }

    fn finish(self) -> String { format!("{:x}", self.finalize()) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashingAlgorithm {
    SHA512_256,
    SHA512,
    SHA384,
    SHA256,
    SHA1,
    MD5,
}

fn ring_calculate<R: BufRead>(
    reader: &mut R,
    ring_algorithm: &'static digest::Algorithm,
) -> io::Result<String> {
    let context = digest::Context::new(ring_algorithm);
    context.calculate(reader)
}

impl HashingAlgorithm {
    pub const ALL: [HashingAlgorithm; 6] = [
        HashingAlgorithm::SHA512_256,
        HashingAlgorithm::SHA512,
        HashingAlgorithm::SHA384,
        HashingAlgorithm::SHA256,
        HashingAlgorithm::SHA1,
        HashingAlgorithm::MD5,
    ];

    pub fn calculate<R: BufRead>(self, reader: &mut R) -> io::Result<String> {
        match self {
            HashingAlgorithm::SHA512_256 => {
                ring_calculate(reader, &digest::SHA512_256)
            },
            HashingAlgorithm::SHA512 => ring_calculate(reader, &digest::SHA512),
            HashingAlgorithm::SHA384 => ring_calculate(reader, &digest::SHA384),
            HashingAlgorithm::SHA256 => ring_calculate(reader, &digest::SHA256),
            HashingAlgorithm::SHA1 => {
                ring_calculate(reader, &digest::SHA1_FOR_LEGACY_USE_ONLY)
            },
            HashingAlgorithm::MD5 => Md5::new().calculate(reader),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            HashingAlgorithm::SHA512_256 => "SHA512/256",
            HashingAlgorithm::SHA512 => "SHA512",
            HashingAlgorithm::SHA384 => "SHA384",
            HashingAlgorithm::SHA256 => "SHA256",
            HashingAlgorithm::SHA1 => "SHA1",
            HashingAlgorithm::MD5 => "MD5",
        }
    }
}

impl fmt::Display for HashingAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
