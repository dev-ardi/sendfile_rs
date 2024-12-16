use clap::Parser;
use std::{fs::File, os::fd::AsRawFd, path::PathBuf, ptr::null_mut};

#[derive(Parser)]
struct Cli {
    #[clap(long, short)]
    input: PathBuf,
    #[clap(long, short)]
    output: PathBuf,
    /// How many bytes to copy. will copy the whole thing if unspecified.
    #[clap(long, short)]
    count: Option<usize>,
}

fn main() {
    let Cli {
        input,
        output,
        count,
    } = Cli::parse();

    let input = File::open(input).unwrap();
    let output = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output)
        .unwrap();

    if cfg!(target_os = "linux") {
        let count = count.unwrap_or_else(|| input.metadata().unwrap().len() as usize);
        let input = input.as_raw_fd();
        let output = output.as_raw_fd();

        unsafe {
            if cfg!(target_pointer_width = "64") {
                libc::sendfile64(output, input, null_mut(), count);
            } else {
                libc::sendfile(output, input, null_mut(), count);
            }
        }
    } else {
        panic!("must be linux");
    }
}
