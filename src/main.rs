use anyhow::{Context, Result};
use colored::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

// Define structures for OpenAI-compatible API requests/responses
#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

struct AiConfig {
    api_key: String,
    base_url: String,
    model: String,
    name: String,
}

// Structure to hold a single conversation turn
struct ConversationTurn {
    user_question: String,
    moonshot_answer: String,
    deepseek_review: String,
    timestamp: String,
}

impl ConversationTurn {
    fn new(user_question: String, moonshot_answer: String, deepseek_review: String) -> Self {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            user_question,
            moonshot_answer,
            deepseek_review,
            timestamp,
        }
    }
}

impl AiConfig {
    fn get_config_path() -> Result<std::path::PathBuf> {
        let home = env::var("HOME").context("Could not find HOME environment variable")?;
        let config_path = std::path::Path::new(&home).join(".ai_vs_ai_config");
        Ok(config_path)
    }

    fn get_api_key(env_var: &str, provider_name: &str) -> Result<String> {
        // 1. Try to get from environment (loaded from config file)
        if let Ok(key) = env::var(env_var) {
            if !key.is_empty() {
                return Ok(key);
            }
        }

        // 2. Prompt user
        print!("Enter API Key for {}: ", provider_name);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim().to_string();
        if trimmed.is_empty() {
            anyhow::bail!("API Key for {} cannot be empty", provider_name);
        }

        // 3. Persist to global config file
        let config_path = Self::get_config_path()?;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&config_path)
            .context(format!("Failed to open config file at {:?}", config_path))?;
        
        writeln!(file, "{}={}", env_var, trimmed)?;
        println!("{}", format!("Saved {} to {:?}", env_var, config_path).dimmed());

        // Also set it in the current process environment so subsequent calls work
        env::set_var(env_var, &trimmed);

        Ok(trimmed)
    }

    fn moonshot() -> Result<Self> {
        Ok(Self {
            api_key: Self::get_api_key("MOONSHOT_API_KEY", "Moonshot AI")?,
            base_url: "https://api.moonshot.cn/v1/chat/completions".to_string(),
            model: "moonshot-v1-8k".to_string(),
            name: "Moonshot AI".to_string(),
        })
    }

    fn deepseek() -> Result<Self> {
        Ok(Self {
            api_key: Self::get_api_key("DEEPSEEK_API_KEY", "DeepSeek AI")?,
            base_url: "https://api.deepseek.com/chat/completions".to_string(),
            model: "deepseek-chat".to_string(),
            name: "DeepSeek AI".to_string(),
        })
    }
}

async fn call_ai_api(client: &Client, config: &AiConfig, messages: Vec<ChatMessage>) -> Result<String> {
    println!("{}", format!("Thinking ({}) ...", config.name).dimmed());

    let request_body = ChatRequest {
        model: config.model.clone(),
        messages,
        temperature: 0.7,
    };

    let response = client
        .post(&config.base_url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .context(format!("Failed to send request to {}", config.name))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!("API Error from {}: {}", config.name, error_text));
    }

    let chat_response: ChatResponse = response
        .json()
        .await
        .context(format!("Failed to parse response from {}", config.name))?;

    chat_response
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| anyhow::anyhow!("No choices returned from {}", config.name))
}

// Generate filename from timestamp and user question
fn generate_filename(_timestamp: &str, question: &str) -> String {
    // Extract first 20 chars of question, remove punctuation, replace spaces with underscores
    let summary: String = question
        .chars()
        .take(20)
        .map(|c| {
            if c.is_ascii_punctuation() || c.is_ascii_whitespace() {
                '_'
            } else {
                c
            }
        })
        .collect();
    
    // Format timestamp for filename: YYYY-MM-DD_HH-MM-SS
    let filename_timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    
    format!("{}_{}.md", filename_timestamp, summary)
}

// Save conversation to markdown file
fn save_conversation(
    turn: &ConversationTurn,
    moonshot_model: &str,
    deepseek_model: &str,
) -> Result<PathBuf> {
    // Get project root directory (where Cargo.toml is located)
    let project_root = env::current_dir()?;
    let conversations_dir = project_root.join("conversations");
    
    // Create conversations directory if it doesn't exist
    if !conversations_dir.exists() {
        std::fs::create_dir_all(&conversations_dir)
            .context("Failed to create conversations directory")?;
    }
    
    // Generate filename
    let filename = generate_filename(&turn.timestamp, &turn.user_question);
    let filepath = conversations_dir.join(&filename);
    
    // Build markdown content
    let content = format!(r#"---
created_at: {}
moonshot_model: {}
deepseek_model: {}
---

# AIvsAI 对话记录

> 💬 **用户**：{}

---

> 🤖 **Moonshot** ({})
> 
{}

---

> 🔍 **DeepSeek** ({})
> 
{}
"#,
        turn.timestamp,
        moonshot_model,
        deepseek_model,
        turn.user_question,
        moonshot_model,
        turn.moonshot_answer.lines().map(|line| format!("> {}", line)).collect::<Vec<_>>().join("\n> "),
        deepseek_model,
        turn.deepseek_review.lines().map(|line| format!("> {}", line)).collect::<Vec<_>>().join("\n> "),
    );
    
    // Write to file
    std::fs::write(&filepath, content)
        .context("Failed to write conversation file")?;
    
    Ok(filepath)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load config from global file
    if let Ok(home) = env::var("HOME") {
        let config_path = std::path::Path::new(&home).join(".ai_vs_ai_config");
        if config_path.exists() {
             dotenvy::from_path(&config_path).ok();
        }
    }

    let client = Client::new();

    println!("{}", "==========================================".cyan().bold());
    println!("{}", "   AI Pair: Moonshot (Answer) + DeepSeek (Review)   ".cyan().bold());
    println!("{}", "==========================================".cyan().bold());
    println!("{}", "Commands: /save = save conversation, exit/quit = exit".dimmed());

    // Check configuration early
    let moonshot_config = match AiConfig::moonshot() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", format!("Configuration Error: {}", e).red());
            return Ok(());
        }
    };

    let deepseek_config = match AiConfig::deepseek() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", format!("Configuration Error: {}", e).red());
            return Ok(());
        }
    };

    // Store the last conversation turn for saving
    let mut last_turn: Option<ConversationTurn> = None;

    loop {
        print!("{}", "\nUser > ".green().bold());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            break;
        }

        if input.is_empty() {
            continue;
        }

        // Handle /save command
        if input.eq_ignore_ascii_case("/save") {
            match &last_turn {
                Some(turn) => {
                    match save_conversation(turn, &moonshot_config.model, &deepseek_config.model) {
                        Ok(filepath) => {
                            println!("{}", format!("✓ Conversation saved to: {}", filepath.display()).green());
                        }
                        Err(e) => {
                            eprintln!("{}", format!("✗ Failed to save conversation: {}", e).red());
                        }
                    }
                }
                None => {
                    println!("{}", "⚠ No conversation to save yet. Ask a question first!".yellow());
                }
            }
            continue;
        }

        // --- Step 1: Moonshot Answers ---
        let moonshot_messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a helpful AI assistant.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: input.to_string(),
            },
        ];

        let moonshot_answer = match call_ai_api(&client, &moonshot_config, moonshot_messages).await {
            Ok(ans) => ans,
            Err(e) => {
                eprintln!("{}", format!("Moonshot Error: {}", e).red());
                continue;
            }
        };

        println!("\n{}", "--- Moonshot AI Answer ---".blue().bold());
        println!("{}", moonshot_answer);

        // --- Step 2: DeepSeek Reviews ---
        let review_prompt = format!(
            "The user asked: \"{}\"\n\nAnother AI assistant provided the following answer:\n\"{}\"\n\nPlease review this answer. Point out any errors, hallucinations, or missing information. If the code is provided, check for bugs. If the answer is perfect, verify it.\n\nIMPORTANT: Please provide your review entirely in Chinese.",
            input, moonshot_answer
        );

        let deepseek_messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are an expert technical reviewer. Your goal is to verify the accuracy and quality of answers provided by other AI models. You must output your review in Chinese.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: review_prompt,
            },
        ];

        let deepseek_review = match call_ai_api(&client, &deepseek_config, deepseek_messages).await {
            Ok(ans) => ans,
            Err(e) => {
                eprintln!("{}", format!("DeepSeek Error: {}", e).red());
                continue;
            }
        };

        println!("\n{}", "--- DeepSeek AI Review ---".magenta().bold());
        println!("{}", deepseek_review);
        
        println!("\n{}", "------------------------------------------".dimmed());
        println!("{}", "Type /save to save this conversation".dimmed());

        // Store the conversation turn for potential saving
        last_turn = Some(ConversationTurn::new(
            input.to_string(),
            moonshot_answer,
            deepseek_review,
        ));
    }

    Ok(())
}
