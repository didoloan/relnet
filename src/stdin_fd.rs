use std::{future::ready, os::fd::FromRawFd};

pub struct StdInFd {
    cur: usize,
    backing_file: monoio::fs::File,
}

impl StdInFd {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let backing_file = monoio::fs::File::from_std(unsafe { std::fs::File::from_raw_fd(0) })?;
        Ok(Self {
            cur: 0,
            backing_file,
        })
    }
}

impl monoio::io::AsyncReadRent for StdInFd {
    fn read<T: monoio::buf::IoBufMut>(
        &mut self,
        buf: T,
    ) -> impl std::future::Future<Output = monoio::BufResult<usize, T>> {
        async {
            let buf_res = self.backing_file.read_at(buf, self.cur as u64).await;
            match buf_res {
                (Ok(size), _) => {
                    self.cur += size;
                }
                _ => {}
            }

            buf_res
        }
    }

    fn readv<T: monoio::buf::IoVecBufMut>(
        &mut self,
        buf: T,
    ) -> impl std::future::Future<Output = monoio::BufResult<usize, T>> {
        // TODO: Not implemented

        ready((Ok(0), buf))
    }
}
