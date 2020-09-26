// Copyright (C) 2020 Stephane Raux. Distributed under the Zlib license.

#![deny(warnings)]

use std::{
    error::Error,
    io::{self, BufReader, BufWriter, Write},
};
use structopt::StructOpt;
use thiserror::Error;

/// Pretty-prints JSON without requiring to load it all in memory.
#[derive(Debug, StructOpt)]
#[structopt(author)]
struct Args {
    /// Indentation size in number of spaces
    #[structopt(long, default_value = "2")]
    indent_size: usize,
}

fn main() {
    if let Err(e) = run() {
        print_error(&e);
        std::process::exit(1);
    }
}

fn print_error(mut e: &dyn Error) {
    eprintln!("Error: {}", e);
    while let Some(cause) = e.source() {
        e = cause;
        eprintln!("Because: {}", e);
    }
}

fn run() -> Result<(), AppError> {
    let args = Args::from_args();
    let indent = vec![b' '; args.indent_size];
    let stdout = io::stdout();
    let mut writer = CatchBrokenPipe::new(BufWriter::new(stdout.lock()));
    match transcode(&indent, &mut writer) {
        Err(_) if writer.broken_pipe => Ok(()),
        r => r,
    }
}

fn transcode<W>(indent: &[u8], writer: W) -> Result<(), AppError>
where
    W: Write,
{
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut input = serde_json::Deserializer::from_reader(reader);
    let formatter = serde_json::ser::PrettyFormatter::with_indent(indent);
    let mut output = serde_json::Serializer::with_formatter(writer, formatter);
    serde_transcode::transcode(&mut input, &mut output)?;
    let mut writer = output.into_inner();
    writer.write_all(&[b'\n'])?;
    writer.flush()?;
    Ok(())
}

#[derive(Debug, Error)]
enum AppError {
    #[error("Transcoding error: {0}")]
    Transcode(io::Error),
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Transcode(e.into())
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Transcode(e)
    }
}

#[derive(Debug)]
struct CatchBrokenPipe<W> {
    writer: W,
    broken_pipe: bool,
}

impl<W: Write> CatchBrokenPipe<W> {
    fn new(writer: W) -> Self {
        CatchBrokenPipe {
            writer,
            broken_pipe: false,
        }
    }

    fn catch_error<T, F>(&mut self, f: F) -> io::Result<T>
    where
        F: FnOnce(&mut W) -> io::Result<T>,
    {
        let r = f(&mut self.writer);
        if r.as_ref().err().map_or(false, |e| e.kind() == io::ErrorKind::BrokenPipe) {
            self.broken_pipe = true;
        }
        r
    }
}

impl<W: Write> Write for CatchBrokenPipe<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.catch_error(|writer| writer.write(buf))
    }

    fn flush(&mut self) -> io::Result<()> {
        self.catch_error(|writer| writer.flush())
    }
}
