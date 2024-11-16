use std::path::{Path, PathBuf};
use std::process::Output;

use itertools::Itertools;
use svg::node::element::tag::Type;
use svg::parser::Event;
use thiserror::Error;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{self, AsyncWriteExt};
use tokio::process::Command;
use uuid::Uuid;

const TMP_DIR: &str = "tmp";

pub enum Quality {
    Low,
    High,
}

#[derive(Error, Debug)]
#[error("{message}")]
pub struct TypstError {
    pub coordinates: (u32, u32),
    message: Box<str>,
}

pub async fn render(
    contents: &str,
    quality: Quality,
) -> io::Result<Result<Option<PathBuf>, TypstError>> {
    let (filename_typ, filename_svg, filename_png) = gen_filenames();
    let mut file = create_file(&filename_typ).await?;

    setup_page(&mut file).await?;
    file.writeln(contents.as_bytes()).await?;

    let svg_output = compile_svg(&filename_typ, &filename_svg).await?;
    if svg_output.status.success() {
        let (
            Event::Tag("svg", Type::Start, _),
            Event::Tag("path", Type::Empty, _),
            Event::Tag("g", Type::Empty, _),
            Event::Tag("svg", Type::End, _),
        ) = svg::open(&filename_svg, &mut String::new())?
            .take(4)
            .collect_tuple::<(_, _, _, _)>()
            .expect("svg files should contain at least 4 tags")
        else {
            fs::remove_file(filename_svg).await?;
            return Ok(Ok(None));
        };
        fs::remove_file(filename_svg).await?;
    }

    let output = compile(&filename_typ, &filename_png, quality).await?;
    fs::remove_file(filename_typ).await?;

    let result = output
        .status
        .success()
        .then_some(Some(filename_png))
        .ok_or_else(|| {
            let err_msg_full = String::from_utf8(output.stderr)
                .expect("the typst CLI should output valid utf-8 to stderr");

            let (location_raw, _, message) = err_msg_full
                .splitn(3, |c: char| c.is_ascii_whitespace())
                .collect_tuple::<(_, _, _)>()
                .expect("typst should output at least three tokens separated by whitespace");

            let message = Box::from(message);
            let coordinates = parse_location(location_raw)
                .expect("typst should output coordinates in a predetermined format");

            TypstError {
                coordinates,
                message,
            }
        });

    Ok(result)
}

fn parse_location(raw: &str) -> Option<(u32, u32)> {
    let mut tokens = raw.split(':').skip(1);

    let line_raw = tokens.next()?;
    let column_raw = tokens.next()?;

    let line = line_raw.parse::<u32>().ok()? - 1;
    let column = column_raw.parse::<u32>().ok()?;

    Some((line, column))
}

fn gen_filenames() -> (PathBuf, PathBuf, PathBuf) {
    let uuid = Uuid::new_v4();
    let filename_raw = format!("{TMP_DIR}/{uuid}");

    let filename_no_ext = Path::new(&filename_raw);
    let filename_typ = filename_no_ext.with_extension("typ");
    let filename_svg = filename_no_ext.with_extension("svg");
    let filename_png = filename_no_ext.with_extension("png");

    (filename_typ, filename_svg, filename_png)
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

async fn compile_svg(filename_typ: &Path, filename_svg: &Path) -> io::Result<Output> {
    let convert_err_msg = "paths created by this program should be convertible strings";

    let input = filename_typ.to_str().expect(convert_err_msg);
    let output = filename_svg.to_str().expect(convert_err_msg);

    let output = Command::new("typst")
        .args(["compile", input, output])
        .output()
        .await?;

    Ok(output)
}
async fn compile(filename_typ: &Path, filename_png: &Path, quality: Quality) -> io::Result<Output> {
    let convert_err_msg = "paths created by this program should be convertible strings";

    let ppi = match quality {
        Quality::Low => "300",
        Quality::High => "300",
    };
    let input = filename_typ.to_str().expect(convert_err_msg);
    let output = filename_png.to_str().expect(convert_err_msg);

    let output = Command::new("typst")
        .args([
            "compile",
            "--diagnostic-format",
            "short",
            "--ppi",
            ppi,
            input,
            output,
        ])
        .output()
        .await?;

    Ok(output)
}

trait AsyncWriteln {
    async fn writeln(&mut self, contents: &[u8]) -> io::Result<()>;
}

impl AsyncWriteln for File {
    async fn writeln(&mut self, contents: &[u8]) -> io::Result<()> {
        self.write_all(contents).await?;
        self.write_all(b"\n").await?;
        Ok(())
    }
}