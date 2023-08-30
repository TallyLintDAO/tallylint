#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// use chatgpt::prelude::*;

// #[ic_cdk::query]
// async fn gpt(prompt: String) -> Result<String> {
//     let client = ChatGPT::new(String::from(
//         "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",
//     ))?;
//     let response = client.send_message(&prompt).await?;
//     return Ok(response.message().content.to_string());
// }
