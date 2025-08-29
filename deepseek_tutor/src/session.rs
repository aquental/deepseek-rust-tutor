use anyhow::{Result, bail};
use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
};
use std::collections::HashMap;

pub struct SessionData {
    pub system_prompt: String,
    pub messages: Vec<ChatCompletionRequestMessage>,
}

pub struct SessionManager {
    sessions: HashMap<String, HashMap<String, SessionData>>,
}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager {
            sessions: HashMap::new(),
        }
    }

    pub fn create_session<S: Into<String>>(
        &mut self,
        student_id: S,
        session_id: S,
        system_prompt: S,
    ) {
        let sid = student_id.into();
        let sess = session_id.into();
        let prompt = system_prompt.into();
        let data = SessionData {
            system_prompt: prompt,
            messages: Vec::new(),
        };
        self.sessions.entry(sid).or_default().insert(sess, data);
    }

    pub fn get_session(&self, student_id: &str, session_id: &str) -> Option<&SessionData> {
        self.sessions
            .get(student_id)
            .and_then(|m| m.get(session_id))
    }

    pub fn add_message(
        &mut self,
        student_id: &str,
        session_id: &str,
        role: &str,
        content: &str,
    ) -> Result<()> {
        let session = self
            .sessions
            .get_mut(student_id)
            .and_then(|m| m.get_mut(session_id))
            .ok_or_else(|| anyhow::anyhow!("Session not found"))?;

        let msg = match role {
            "system" => ChatCompletionRequestSystemMessageArgs::default()
                .content(content)
                .build()?
                .into(),
            "user" => ChatCompletionRequestUserMessageArgs::default()
                .content(content)
                .build()?
                .into(),
            "assistant" => ChatCompletionRequestAssistantMessageArgs::default()
                .content(content)
                .build()?
                .into(),
            _ => bail!("Unknown role: {}", role),
        };

        session.messages.push(msg);
        Ok(())
    }

    pub fn get_conversation(
        &self,
        student_id: &str,
        session_id: &str,
    ) -> Vec<ChatCompletionRequestMessage> {
        if let Some(session) = self.get_session(student_id, session_id) {
            let mut convo = Vec::with_capacity(1 + session.messages.len());
            convo.push(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(session.system_prompt.as_str()) // Changed from &session.system_prompt
                    .build()
                    .unwrap()
                    .into(),
            );
            convo.extend(session.messages.clone());
            convo
        } else {
            Vec::new()
        }
    }
}
