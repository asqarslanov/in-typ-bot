use std::path::{Path, PathBuf};
use std::process::Output;

use itertools::Itertools;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{self, AsyncWriteExt};
use tokio::process::Command;
use uuid::Uuid;

const TMP_DIR: &str = "tmp";

pub enum Quality {
    Low,
    High,
}

pub async fn render(contents: &str, quality: Quality) -> io::Result<Result<PathBuf, String>> {
    let (filename_typ, filename_png) = gen_filenames();
    let mut file = create_file(&filename_typ).await?;

    setup_page(&mut file).await?;
    file.writeln(contents.as_bytes()).await?;

    let output = compile(&filename_typ, &filename_png, quality).await?;
    fs::remove_file(filename_typ).await?;

    let result = output
        .status
        .success()
        .then_some(filename_png)
        .ok_or_else(|| {
            String::from_utf8(output.stderr)
                .expect("the typst CLI should output valid utf-8 to stderr")
                .lines()
                .map(|line| {
                    line.splitn(3, |c: char| c.is_ascii_whitespace())
                        .collect_tuple::<(_, _, _)>()
                        .map(|(location_raw, _, message)| {
                            format!("{} {message}", process_location(location_raw))
                        })
                        .unwrap()
                })
                .collect::<Box<[_]>>()
                .join("\n")
        });

    Ok(result)
}

fn process_location(location_raw: &str) -> String {
    let mut tokens = location_raw.split(':').skip(1);

    let line = tokens.next().unwrap();
    let column = tokens.next().unwrap();

    format!("{}:{}:", line.parse::<u32>().unwrap() - 1, column)
}

fn gen_filenames() -> (PathBuf, PathBuf) {
    let uuid = Uuid::new_v4();
    let filename_raw = format!("{TMP_DIR}/{uuid}");

    let filename_no_ext = Path::new(&filename_raw);
    let filename_typ = filename_no_ext.with_extension("typ");
    let filename_png = filename_no_ext.with_extension("png");

    (filename_typ, filename_png)
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
