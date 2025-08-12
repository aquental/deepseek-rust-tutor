use anyhow::{Context, Result, bail};
use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs, Role,
};
use std::collections::HashMap;

pub struct SessionData {
    #[allow(dead_code)]
    pub system_prompt: String,
    #[allow(dead_code)]
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

    pub fn create_session(
        &mut self,
        student_id: impl Into<String>,
        session_id: impl Into<String>,
        system_prompt: impl Into<String>,
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

    #[allow(dead_code)]
    pub fn get_session_mut(
        &mut self,
        student_id: &str,
        session_id: &str,
    ) -> Option<&mut SessionData> {
        self.sessions
            .get_mut(student_id)
            .and_then(|m| m.get_mut(session_id))
    }

    #[allow(dead_code)]
    pub fn add_message(
        &mut self,
        student_id: &str,
        session_id: &str,
        role: Role,
        content: &str,
    ) -> Result<()> {
        let session = self
            .get_session_mut(student_id, session_id)
            .context("Session not found")?;

        let msg = match role {
            Role::System => ChatCompletionRequestSystemMessageArgs::default()
                .content(content)
                .build()?
                .into(),
            Role::User => ChatCompletionRequestUserMessageArgs::default()
                .content(content)
                .build()?
                .into(),
            Role::Assistant => ChatCompletionRequestAssistantMessageArgs::default()
                .content(content)
                .build()?
                .into(),
            _ => bail!("Unsupported role: {:?}", role),
        };

        session.messages.push(msg);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_conversation(
        &self,
        student_id: &str,
        session_id: &str,
    ) -> Result<Vec<ChatCompletionRequestMessage>> {
        let session = self
            .sessions
            .get(student_id)
            .and_then(|m| m.get(session_id))
            .context("Session not found")?;

        let mut convo = Vec::with_capacity(1 + session.messages.len());

        let system_msg = ChatCompletionRequestSystemMessageArgs::default()
            .content(session.system_prompt.as_str())
            .build()?
            .into();

        convo.push(system_msg);
        convo.extend(session.messages.clone());

        Ok(convo)
    }
}
