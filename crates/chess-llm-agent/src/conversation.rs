use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
            timestamp: Utc::now(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
            timestamp: Utc::now(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationManager {
    messages: Vec<Message>,
    max_history: usize,
}

impl ConversationManager {
    pub fn new(system_prompt: impl Into<String>) -> Self {
        let mut manager = Self {
            messages: Vec::new(),
            max_history: 20, // Keep last 20 messages for context
        };
        manager.add_system_message(system_prompt);
        manager
    }

    pub fn add_system_message(&mut self, content: impl Into<String>) {
        self.messages.push(Message::system(content));
    }

    pub fn add_user_message(&mut self, content: impl Into<String>) {
        self.messages.push(Message::user(content));
        self.trim_history();
    }

    pub fn add_assistant_message(&mut self, content: impl Into<String>) {
        self.messages.push(Message::assistant(content));
        self.trim_history();
    }

    pub fn get_messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn get_chat_messages(&self) -> Vec<crate::openrouter::ChatMessage> {
        self.messages
            .iter()
            .map(|msg| crate::openrouter::ChatMessage {
                role: msg.role.clone(),
                content: msg.content.clone(),
            })
            .collect()
    }

    fn trim_history(&mut self) {
        // Keep the system message (first) and last N messages
        if self.messages.len() > self.max_history + 1 {
            let system_msg = self.messages[0].clone();
            let recent_messages: Vec<_> = self.messages
                .iter()
                .skip(self.messages.len() - self.max_history)
                .cloned()
                .collect();

            self.messages.clear();
            self.messages.push(system_msg);
            self.messages.extend(recent_messages);
        }
    }

    pub fn clear(&mut self) {
        let system_msg = self.messages.first().cloned();
        self.messages.clear();
        if let Some(msg) = system_msg {
            self.messages.push(msg);
        }
    }
}

impl Default for ConversationManager {
    fn default() -> Self {
        Self::new("You are a helpful chess coach.")
    }
}
