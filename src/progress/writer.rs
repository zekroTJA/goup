use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::io::{self, Read};

pub struct Reader<R> {
    r: R,
    progress_bar: ProgressBar,
}

impl<R> Reader<R> {
    pub fn new(size: Option<u64>, r: R, target: ProgressDrawTarget) -> Self {
        let progress_bar = ProgressBar::with_draw_target(size, target);

        progress_bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{wide_bar:.cyan/dim}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

        Self { progress_bar, r }
    }

    pub fn finish(&self) {
        self.progress_bar.finish();
    }
}

impl<R> Read for Reader<R>
where
    R: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = self.r.read(buf)?;
        self.progress_bar.inc(size as u64);
        Ok(size)
    }
}

impl<R> Drop for Reader<R> {
    fn drop(&mut self) {
        self.finish()
    }
}
