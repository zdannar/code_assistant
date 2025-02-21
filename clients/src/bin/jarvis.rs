use clap::arg;
use libcm::ModelRequestType;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = clap::Command::new("jarvis")
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand_required(true)
        .subcommand(
            clap::Command::new("ut")
                .about("Unit test")
                .arg(arg!(<PROMPT> "Your code"))
                .arg_required_else_help(true),
        )
        .subcommand(
            clap::Command::new("q")
                .about("inquiry")
                .arg(arg!(<PROMPT> "Your inquiry"))
                .arg_required_else_help(true),
        );

    let matches = cmd.get_matches();

    let model_request_type = match matches.subcommand() {
        Some(("ut", sub_matches)) => {
            let prompt = sub_matches.get_one::<String>("PROMPT").expect("required");
            ModelRequestType::UnitTestWriteRequest(prompt.to_owned())
        }
        Some(("q", sub_matches)) => {
            let prompt = sub_matches.get_one::<String>("PROMPT").expect("required");
            ModelRequestType::Inquiry(prompt.to_owned())
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
