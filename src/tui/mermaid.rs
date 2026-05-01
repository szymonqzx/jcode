pub use crate::tui::mermaid::*;

pub fn install_jcode_mermaid_hooks() {
    crate::tui::mermaid::set_log_hooks(crate::logging::info, crate::logging::warn);
    crate::tui::mermaid::set_render_completed_hook(|| {
        crate::bus::Bus::global().publish(crate::bus::BusEvent::MermaidRenderCompleted);
    });
    crate::tui::mermaid::set_memory_snapshot_hook(|| {
        let snapshot = crate::process_memory::snapshot_with_source("client:mermaid:memory");
        crate::tui::mermaid::ProcessMemorySnapshot {
            rss_bytes: snapshot.rss_bytes,
            peak_rss_bytes: snapshot.peak_rss_bytes,
            virtual_bytes: snapshot.virtual_bytes,
        }
    });
}
