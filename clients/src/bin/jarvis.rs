use libcm::ModelRequestType;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = clap::Command::new("jarvis")
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand_required(true)
        .subcommand(
            clap::Command::new("ut").about("Unit test"), // .arg(arg!(<PROMPT> "Your code")),
        )
        .subcommand(
            clap::Command::new("q").about("inquiry"), // .arg(arg!(<PROMPT> "Your inquiry")),
        );

    let matches = cmd.get_matches();

    let model_request_type = match matches.subcommand() {
        Some(("ut", _)) => {
            let prompt = read_stdin();
            ModelRequestType::UnitTestWriteRequest(prompt)
        }
        Some(("q", _)) => {
            let prompt = read_stdin();
            ModelRequestType::Inquiry(prompt)
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };

    let resp = reqwest::Client::new()
        .post("http://127.0.0.1:8080")
        .json(&model_request_type)
        .send()
        .await?
        .json::<libcm::ModelResponse>()
        .await?;

    println!("{resp}");

    Ok(())
}
