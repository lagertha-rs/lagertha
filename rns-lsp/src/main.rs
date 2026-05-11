use rns::lexer;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct RnsLanguageServer {
    client: Client,
}

impl RnsLanguageServer {
    async fn analyze_and_publish(&self, uri: Uri, text: String) {
        let (_tokens, errors, _eof_span) = lexer::tokenize(&text);
        let mut diagnostics = Vec::with_capacity(errors.len());
        for err in errors {
            let span = err.primary_location;
            let diagnostic = Diagnostic {
                range: Range {
                    start: Position {
                        line: span.line as u32,
                        character: span.col_start as u32,
                    },
                    end: Position {
                        line: span.line as u32,
                        character: span.col_end as u32,
                    },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                message: err.lsp_msg.into_owned(),
                ..Default::default()
            };
            diagnostics.push(diagnostic);
        }

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

impl LanguageServer for RnsLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "rns-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    // TODO: consider using INCREMENTAL in future
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            offset_encoding: None,
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "rns-lsp initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.analyze_and_publish(params.text_document.uri, params.text_document.text)
            .await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.analyze_and_publish(
            params.text_document.uri,
            params.content_changes.pop().unwrap().text,
        )
        .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| RnsLanguageServer { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
