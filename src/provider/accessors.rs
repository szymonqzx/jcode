use super::*;

/// Read helper that tolerates a poisoned RwLock (common during provider auth panics)
macro_rules! read_provider {
    ($r:expr) => {
        match $r.read() {
            Ok(guard) => guard.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        }
    };
}

impl MultiProvider {
    pub(super) fn claude_provider(&self) -> Option<Arc<claude::ClaudeProvider>> {
        read_provider!(self.claude)
    }

    pub(super) fn anthropic_provider(&self) -> Option<Arc<anthropic::AnthropicProvider>> {
        read_provider!(self.anthropic)
    }

    pub(super) fn openai_provider(&self) -> Option<Arc<openai::OpenAIProvider>> {
        read_provider!(self.openai)
    }

    pub(super) fn antigravity_provider(&self) -> Option<Arc<antigravity::AntigravityCliProvider>> {
        read_provider!(self.antigravity)
    }

    pub(super) fn gemini_provider(&self) -> Option<Arc<gemini::GeminiProvider>> {
        read_provider!(self.gemini)
    }

    pub(super) fn copilot_provider(&self) -> Option<Arc<copilot::CopilotApiProvider>> {
        read_provider!(self.copilot_api)
    }

    pub(super) fn cursor_provider(&self) -> Option<Arc<cursor::CursorCliProvider>> {
        read_provider!(self.cursor)
    }

    pub(super) fn windsurf_provider(&self) -> Option<Arc<windsurf::WindsurfProvider>> {
        read_provider!(self.windsurf)
    }

    pub(super) fn openrouter_provider(&self) -> Option<Arc<openrouter::OpenRouterProvider>> {
        read_provider!(self.openrouter)
    }

    pub(super) fn opencode_go_provider(&self) -> Option<Arc<opencode_go::OpenCodeGoProvider>> {
        read_provider!(self.opencode_go)
    }

    pub(super) fn has_claude_runtime(&self) -> bool {
        self.anthropic_provider().is_some() || self.claude_provider().is_some()
    }

    pub(super) fn provider_slot_available(&self, provider: ActiveProvider) -> bool {
        match provider {
            ActiveProvider::Claude => self.has_claude_runtime(),
            ActiveProvider::OpenAI => self.openai_provider().is_some(),
            ActiveProvider::Copilot => self.copilot_provider().is_some(),
            ActiveProvider::Antigravity => self.antigravity_provider().is_some(),
            ActiveProvider::Gemini => self.gemini_provider().is_some(),
            ActiveProvider::Cursor => self.cursor_provider().is_some(),
            ActiveProvider::Windsurf => self.windsurf_provider().is_some(),
            ActiveProvider::OpenRouter => self.openrouter_provider().is_some(),
            ActiveProvider::OpenCodeGo => self.opencode_go_provider().is_some(),
        }
    }

    pub(super) fn reconcile_auth_if_provider_missing(&self, provider: ActiveProvider) -> bool {
        if self.provider_slot_available(provider) {
            return true;
        }

        crate::logging::info(&format!(
            "Provider {} missing at use site; reconciling auth from disk",
            Self::provider_label(provider)
        ));
        Provider::on_auth_changed(self);
        self.provider_slot_available(provider)
    }
}
