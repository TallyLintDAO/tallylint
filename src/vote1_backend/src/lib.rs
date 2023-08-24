#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// cargo.toml:
// openai_api_rust = "0.1.8"
// export OPENAI_API_KEY="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"
use openai_api_rust::*;
use openai_api_rust::chat::*;
use openai_api_rust::completions::*;

#[ic_cdk::query]
fn gpt(prompt: String) -> String {
 // Load API key from environment OPENAI_API_KEY.
    // You can also hadcode through `Auth::new(<your_api_key>)`, but it is not recommended.
    // let auth = Auth::from_env().unwrap();
    let auth = Auth::new("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");
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: "gpt-3.5-turbo".to_string(),
        max_tokens: Some(7),
        temperature: Some(0_f32),
        top_p: Some(0_f32),
        n: Some(2),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: vec![Message { role: Role::User, content: prompt}],
    };
    let result = openai.chat_completion_create(&body);
    let choice = result.unwrap().choices;
    let message = &choice[0].message.as_ref().unwrap();
    return message.content.to_string();
    // println!("{}",message.content);
    // assert!(message.content.contains("Hello"));
}