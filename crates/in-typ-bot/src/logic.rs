use std::path::{Path, PathBuf};
use std::process::Output;

use async_writeln::AsyncWriteln;
use filename::{Filename, TMP_DIR};
use itertools::Itertools;
use svg::{node::element::tag::Type, parser::Event};
use thiserror::Error;
use tokio::fs::{self, File, OpenOptions};
use tokio::io;
use tokio::process::Command;

mod async_writeln;
mod filename;

#[derive(Error, Debug)]
pub enum RenderError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("nothing to render")]
    EmptyDocument,

    #[error("invalid syntax")]
    InvalidSyntax(Box<[ErrorDetails]>),
}

#[derive(Debug)]
pub struct ErrorDetails {
    pub coordinates: (u32, u32),
    pub message: Box<str>,
}

impl From<&str> for ErrorDetails {
    fn from(value: &str) -> Self {
        let (location_raw, _, message) = value
            .splitn(3, |c: char| c.is_ascii_whitespace())
            .collect_tuple::<(_, _, _)>()
            .expect("typst should output at least three tokens separated by whitespace");

        let message = Box::from(message);
        let coordinates = parse_location(location_raw)
            .expect("typst should output coordinates in a predetermined format");

        Self {
            coordinates,
            message,
        }
    }
}

fn extract_error(command_output: Output) -> RenderError {
    let err_text = String::from_utf8(command_output.stderr)
        .expect("the typst CLI should output valid utf-8 to stderr");

    let processed = err_text
        .lines()
        .map(ErrorDetails::from)
        .collect::<Box<[_]>>();

    RenderError::InvalidSyntax(processed)
}

pub async fn render<F, T>(
    contents: &str,
) -> Result<impl async FnOnce(F) -> io::Result<T>, RenderError>
where
    F: async FnOnce(&Path) -> T,
{
    let path_to_file = Filename::new();

    let mut file_typ = create_file(&path_to_file.typ()).await?;
    setup_page(&mut file_typ).await?;
    file_typ.writeln(contents.as_bytes()).await?;

    let compile_svg_output = compile(&path_to_file, OutputFileExtension::Svg).await?;
    if let Err(err) = process_output_svg(&path_to_file, compile_svg_output).await {
        let _ = fs::remove_file(path_to_file.typ()).await;
        return Err(err);
    }

    let compile_png_output = compile(&path_to_file, OutputFileExtension::Png).await?;
    let _ = fs::remove_file(path_to_file.typ()).await;

    let path = process_output_png(&path_to_file, compile_png_output)?;

    Ok(async |process: F| {
        let output = process(path.as_ref()).await;
        fs::remove_file(path).await?;
        Ok(output)
    })
}

fn parse_location(raw: &str) -> Option<(u32, u32)> {
    let mut tokens = raw.rsplitn(4, ':').skip(1);

    let column_raw = tokens.next()?;
    let line_raw = tokens.next()?;

    let line = line_raw.parse::<u32>().ok()? - 1;
    let column = column_raw.parse::<u32>().ok()?;

    Some((line, column))
}

async fn create_file(filename_typ: &Path) -> io::Result<File> {
    let _ = fs::create_dir(TMP_DIR).await;

    OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filename_typ)
        .await
}

async fn setup_page(file: &mut File) -> io::Result<()> {
    const MARGIN: &str = "0.5cm";
    let page = format!("#set page(width: auto, height: auto, margin: {MARGIN})");
    file.writeln(page.as_bytes()).await?;
    Ok(())
}

enum OutputFileExtension {
    Svg,
    Png,
}

async fn compile(
    path_to_file: &Filename,
    output_file_extension: OutputFileExtension,
) -> io::Result<Output> {
    fn to_str(path: &Path) -> &str {
        let convert_err_msg = "paths created by this program should be convertible strings";
        path.to_str().expect(convert_err_msg)
    }

    let filename_in = path_to_file.typ();
    let filename_out = match output_file_extension {
        OutputFileExtension::Svg => path_to_file.svg(),
        OutputFileExtension::Png => path_to_file.png(),
    };

    let output = Command::new("typst")
        .args([
            "compile",
            "--diagnostic-format",
            "short",
            "--ppi",
            "400",
            to_str(&filename_in),
            to_str(&filename_out),
        ])
        .output()
        .await?;

    Ok(output)
}

async fn process_output_svg(
    path_to_file: &Filename,
    command_output: Output,
) -> Result<(), RenderError> {
    if command_output.status.success() {
        let mut buffer = String::new();
        let first_tags = svg::open(path_to_file.svg(), &mut buffer)?
            .take(4)
            .collect_tuple::<(_, _, _, _)>()
            .expect("svg files should contain at least 4 tags");
        let _ = fs::remove_file(path_to_file.svg()).await;

        if matches!(
            first_tags,
            (
                Event::Tag("svg", Type::Start, _),
                Event::Tag("path", Type::Empty, _),
                Event::Tag("g", Type::Empty, _),
                Event::Tag("svg", Type::End, _),
            )
        ) {
            Err(RenderError::EmptyDocument)
        } else {
            Ok(())
        }
    } else {
        Err(extract_error(command_output))
    }
}

fn process_output_png(
    path_to_file: &Filename,
    command_output: Output,
) -> Result<PathBuf, RenderError> {
    if command_output.status.success() {
        Ok(path_to_file.png())
    } else {
        Err(extract_error(command_output))
    }
}
