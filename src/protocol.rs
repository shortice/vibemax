use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub choices: Vec<Choice>,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: String,
    #[serde(rename = "object")]
    pub object_type: String,
    pub usage: Usage,
    pub id: String,
    pub timings: Timings,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    pub finish_reason: String,
    pub index: i32,
    pub message: Message,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Usage {
    pub completion_tokens: i32,
    pub prompt_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Timings {
    pub prompt_n: i32,
    pub prompt_ms: f64,
    pub prompt_per_token_ms: f64,
    pub prompt_per_second: f64,
    pub predicted_n: i32,
    pub predicted_ms: f64,
    pub predicted_per_token_ms: f64,
    pub predicted_per_second: f64,
}

pub fn generate_answer<T: ToString>(x: &Vec<T>) -> i64 {
    let mut arr_data = String::new();

    for (pos, value) in x.iter().enumerate() {
        arr_data += value.to_string().as_str();

        if pos != x.len() - 1 {
            arr_data += ",";
        }
    }

    let data = json!(
    {
        "n_predict": 200,
        "messages": [
            {
                "role": "user", 
                "content": format!(
                    "What is maximum value in [{arr_data}]? Answer with only the number value, nothing else. /no_think",
                )
            }
        ]
    });


    let client = Client::new();
    let req = client.post("http://127.0.0.1:8080/v1/chat/completions")
        .body(data.to_string())
        .send();

    let response = req.expect("Err.").text().expect("Err");

    let result: Response = serde_json::from_str(
        response.as_str()
    ).expect("Err.");

    let answer: Vec<_> = {
        let content = &result.choices[0].message.content;
        content.split("\n\n").collect()
    };

    answer.get(answer.len() - 1)
        .unwrap()
        .parse()
        .unwrap()
}
