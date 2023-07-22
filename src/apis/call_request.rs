use std::env;
use dotenv::dotenv;
use crate::models::general::llm::Message;

//Call large language model (i.e. GPT-4)
pub async fn call_gpt(messages:Vec<Message>) {
	dotenv().ok();

	//Extract API Key information
	let api_key:String = env::var("OPEN_AI_KEY")
		.expect("OPEN_API_KEY not found in .env");
	let api_org:String = env::var("OPEN_AI_ORG")
		.expect("OPEN_API_ORG not found in .env");

	//Confirm endpoint
	let url:&str = "https:://api.openai.com/v1/chat/completions";


}