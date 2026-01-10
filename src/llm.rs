use serde::{Deserialize, Serialize};
use reqwest::{self, blocking};
use std::time::{Instant};
use colored::*;

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

pub struct LLM {
    client: blocking::Client,
    url: String,
    conversation_history: Vec<Message>
}

impl LLM {
    
    pub fn new(url: &str) -> Self{

        //Client setup
        let client = reqwest::blocking::Client::new();
        let url = url.to_string();

        //Chat history
        let conversation_history = vec![];

        Self {client, url, conversation_history}

    }

    fn is_emoji(&mut self, c: &char) -> bool {

        match *c {
            '\u{1F600}'..='\u{1F64F}' | // Emoticons
            '\u{1F300}'..='\u{1F5FF}' | // Misc Symbols and Pictographs
            '\u{1F680}'..='\u{1F6FF}' | // Transport and Map
            '\u{2600}'..='\u{26FF}'   | // Misc Symbols
            '\u{2700}'..='\u{27BF}'   | // Dingbats
            '\u{1F900}'..='\u{1F9FF}'   // Supplemental Symbols and Pictographs
            => true,
            _ => false,
        }
    }

    fn post_process_text(&mut self, text: &str) -> String{

        let cleaned_text: String = text.chars()
            .filter(|c| !self.is_emoji(c) && *c != '*') 
            .collect::<String>()             
            .split_whitespace()         
            .collect::<Vec<_>>()   
            .join(" "); 

        cleaned_text

    }

    pub fn chat(&mut self, user_input: &str)-> Result<String, Box<dyn std::error::Error>> {

        let start: Instant;
        start = Instant::now();

        self.conversation_history.push(
            Message {
                role: "user".to_string(),
                content: user_input.to_string(),
            }
        );


        //JSON body
        let body = serde_json::json!({
            "messages": &self.conversation_history
        });

        //Send the POST request
        let res = self.client.post(&self.url)
            .json(&body)
            .send()?;

        //Parse the response
        let response_data: ChatResponse = res.json()?;
        
        if let Some(choice) = response_data.choices.first() {

            let ai_response = &choice.message.content;  

            let cleaned_ai_response = self.post_process_text(ai_response);  

            self.conversation_history.push(
                Message {
                    role: "assistant".to_string(),
                    content: cleaned_ai_response.to_string(),
                }
            );

            println!("{} {:?}", "Response generated in:".green(), start.elapsed());

            Ok(cleaned_ai_response)

        }else {
            Err("No response choices returned from the model".into())
        }
    }

}