use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    file_path: std::path::PathBuf,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let path = "caaspctl.yml";
    let content = std::fs::read_to_string(path)?;
    println!("file content: {}", content);
    Ok(())
}

