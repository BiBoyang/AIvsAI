use anyhow::{Context, Result};
use colored::*;
use reqwest::Client;
use rustyline::DefaultEditor;
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
    _timestamp: String,
    round: usize, // 第几轮对话
}

impl ConversationTurn {
    fn new(round: usize, user_question: String, moonshot_answer: String, deepseek_review: String) -> Self {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            round,
            user_question,
            moonshot_answer,
            deepseek_review,
            _timestamp: timestamp,
        }
    }
}

// Structure to hold the entire conversation session
struct ConversationSession {
    turns: Vec<ConversationTurn>,
    start_time: String,
}

impl ConversationSession {
    fn new() -> Self {
        let start_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            turns: Vec::new(),
            start_time,
        }
    }

    fn add_turn(&mut self, turn: ConversationTurn) {
        self.turns.push(turn);
    }

    fn is_empty(&self) -> bool {
        self.turns.is_empty()
    }

    fn len(&self) -> usize {
        self.turns.len()
    }

    fn first_question(&self) -> Option<&str> {
        self.turns.first().map(|t| t.user_question.as_str())
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

        // 2. Prompt user using standard io (not rustyline, as this is one-time setup)
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

// Generate filename from timestamp and first question
fn generate_filename(_start_time: &str, question: &str) -> String {
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

// Format content with proper line prefixing
fn format_content_with_prefix(content: &str, prefix: &str) -> String {
    content.lines().map(|line| format!("{}{}", prefix, line)).collect::<Vec<_>>().join("\n")
}

// Find project directory by looking for Cargo.toml in current dir or parents
fn find_project_dir() -> Result<PathBuf> {
    let mut current_dir = env::current_dir()
        .context("Failed to get current directory")?;
    
    loop {
        // Check if Cargo.toml exists in current directory
        let cargo_toml = current_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            return Ok(current_dir);
        }
        
        // Try parent directory
        match current_dir.parent() {
            Some(parent) => current_dir = parent.to_path_buf(),
            None => break,
        }
    }
    
    // If no Cargo.toml found, fall back to current directory
    env::current_dir().context("Failed to get current directory")
}

// Save entire conversation session to markdown file
fn save_conversation_session(
    session: &ConversationSession,
    moonshot_model: &str,
    deepseek_model: &str,
) -> Result<PathBuf> {
    // Find project directory and create conversations subdirectory
    let project_dir = find_project_dir()?;
    let conversations_dir = project_dir.join("conversations");
    
    // Create conversations directory if it doesn't exist
    if !conversations_dir.exists() {
        std::fs::create_dir_all(&conversations_dir)
            .context("Failed to create conversations directory")?;
    }
    
    // Generate filename using first question
    let first_question = session.first_question().unwrap_or("conversation");
    let filename = generate_filename(&session.start_time, first_question);
    let filepath = conversations_dir.join(&filename);
    
    // Build markdown content
    let mut content = format!(r#"---
session_start: {}
total_rounds: {}
moonshot_model: {}
deepseek_model: {}
---

# AIvsAI 对话记录

"#, session.start_time, session.len(), moonshot_model, deepseek_model);
    
    // Add each turn
    for turn in &session.turns {
        content.push_str(&format!(r#"## 第 {} 轮

> 💬 **用户**：{}

---

> 🤖 **Moonshot** ({})
> 
{}

---

> 🔍 **DeepSeek** ({})
> 
{}

---

"#,
            turn.round,
            turn.user_question,
            moonshot_model,
            format_content_with_prefix(&turn.moonshot_answer, "> "),
            deepseek_model,
            format_content_with_prefix(&turn.deepseek_review, "> "),
        ));
    }
    
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

    // Create rustyline editor for better input handling (supports Chinese characters properly)
    let mut rl = DefaultEditor::new()?;
    
    // Store the entire conversation session
    let mut session = ConversationSession::new();
    let mut round_counter: usize = 0;

    loop {
        // Use rustyline for reading input with proper Unicode support
        let readline = rl.readline("\nUser > ");
        
        let input = match readline {
            Ok(line) => {
                // Add to history (optional, allows up-arrow to recall previous inputs)
                let _ = rl.add_history_entry(line.as_str());
                line.trim().to_string()
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                // Handle Ctrl+C
                println!("{}", "\nUse 'exit' or 'quit' to exit.".dimmed());
                continue;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                // Handle Ctrl+D
                break;
            }
            Err(err) => {
                eprintln!("{}", format!("Error reading input: {}", err).red());
                continue;
            }
        };

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            break;
        }

        if input.is_empty() {
            continue;
        }

        // Handle /save command
        if input.eq_ignore_ascii_case("/save") {
            if session.is_empty() {
                println!("{}", "⚠ No conversation to save yet. Ask a question first!".yellow());
            } else {
                match save_conversation_session(&session, &moonshot_config.model, &deepseek_config.model) {
                    Ok(filepath) => {
                        println!("{}", format!("✓ Conversation saved to: {}", filepath.display()).green());
                        println!("{}", format!("  Total rounds saved: {}", session.len()).dimmed());
                    }
                    Err(e) => {
                        eprintln!("{}", format!("✗ Failed to save conversation: {}", e).red());
                    }
                }
            }
            continue;
        }

        // Increment round counter
        round_counter += 1;

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
        println!("{}", format!("Round {} completed. Type /save to save this conversation", round_counter).dimmed());

        // Store the conversation turn
        session.add_turn(ConversationTurn::new(
            round_counter,
            input.to_string(),
            moonshot_answer,
            deepseek_review,
        ));
    }

    Ok(())
}
