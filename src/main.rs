use kak_ui::OutgoingRequest;
use std::io::Write;
use std::process::{Command, Stdio};
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(
    name = "kak-broot",
    about = "A wrapper around kakoune and broot that allows you to use broot to open files in kakoune"
)]
struct Args {
    /// The kakoune session to connect to
    #[structopt(name = "kak-session")] 
    session: String,
    /// The directory in which to open broot
    base_dir: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let Args { session, base_dir } = Args::from_args();

    let broot_output = get_broot_output(base_dir)?;
    let broot_output = broot_output.trim();
    
    loop {
        if broot_output.is_empty() {
            break;
        } else {
            edit_file(&session, broot_output)?;
        }
    }
    Ok(())
}

fn edit_file(session: &str, file: &str) -> anyhow::Result<()> {
    let mut kak_command = Command::new("kak")
        .args(&["-ui", "json"])
        .args(&["-c", session])
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .stdin(Stdio::piped())
        .spawn()?;

    let request = serde_json::to_string(&OutgoingRequest::Keys(vec![
        "<esc><esc>".to_string(),
        format!(":edit {} <ret>", file),
        ":q<ret>".to_string(),
    ]))?;

    kak_command
        .stdin
        .as_mut()
        .unwrap()
        .write_all(request.as_bytes())?;
    kak_command.wait()?;
    Ok(())
}

fn get_broot_output(base_dir: Option<String>) -> anyhow::Result<String> {
    let broot_output = Command::new("broot")
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .stdin(Stdio::piped())
        .args(base_dir)
        .output()?;
    Ok(String::from_utf8(broot_output.stdout)?)
}
