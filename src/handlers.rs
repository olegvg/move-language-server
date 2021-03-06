use lsp_server::Notification;
use lsp_types::notification::PublishDiagnostics;
use lsp_types::{PublishDiagnosticsParams, Url};

use crate::analysis::AnalysisChange;

use crate::compiler::utils::leak_str;
use crate::main_loop::notification_new;
use crate::world::WorldState;

pub(crate) fn on_document_change(
    world_state: &mut WorldState,
    document_uri: Url,
    new_source_text: &str,
) -> Notification {
    let document_fname = leak_str(document_uri.path());

    let mut change = AnalysisChange::new();
    change.change_file(document_fname, new_source_text.to_owned());
    world_state.apply_change(change);

    let analysis = world_state.analysis();
    let diagnostics = analysis.check_with_libra_compiler(document_fname, new_source_text);
    notification_new::<PublishDiagnostics>(PublishDiagnosticsParams::new(
        document_uri,
        diagnostics,
        None,
    ))
}
