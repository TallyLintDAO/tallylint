#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

use chatgpt::prelude::*;
#[ic_cdk::query]
#[tokio::main]
async fn gpt(prompt: String) -> Result<()> {
    let client = ChatGPT::new(String::from(
        "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",
    ))?;
    let prompt = "Describe in five words of the JS programming language.";
    let response = client.send_message(prompt).await?;
    let Q1 = format!("{}{}", String::from("Q: "), prompt);
    let A1 = format!("{}{}", String::from("A: "), response.message().content);
    println!("{}", Q1);
    println!("{}", A1);
    Ok(())
}

use chatgpt::prelude::*;

// #[tokio::main]
#[ic_cdk::query]
async fn gpt(prompt: String) -> Result<String> {
    let client = ChatGPT::new(String::from(
        "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",
    ))?;
    let response = client.send_message(&prompt).await?;
    return Ok(response.message().content.to_string());
}
