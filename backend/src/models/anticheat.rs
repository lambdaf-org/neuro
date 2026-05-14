use serde::Serialize;
use serde_json::Value;
use serde_json::json;

#[derive(Debug, Clone, Serialize)]
pub struct AnticheatFlag {
    pub code: &'static str,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnticheatAction {
    Allow,
    Flag,
    Ban,
}

impl AnticheatAction {
    pub fn as_str(self) -> &'static str {
        match self {
            AnticheatAction::Allow => "allow",
            AnticheatAction::Flag => "flag",
            AnticheatAction::Ban => "ban",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnticheatVerdict {
    pub action: AnticheatAction,
    pub flags: Vec<AnticheatFlag>,
}

impl AnticheatVerdict {
    pub fn allow() -> Self {
        Self {
            action: AnticheatAction::Allow,
            flags: Vec::new(),
        }
    }

    pub fn is_ban(&self) -> bool {
        matches!(self.action, AnticheatAction::Ban)
    }

    pub fn flags_summary(&self) -> Value {
        json!({
            "action": self.action.as_str(),
            "flags": self
                .flags
                .iter()
                .map(|f| json!({"code": f.code, "reason": f.reason}))
                .collect::<Vec<_>>(),
        })
    }
}
