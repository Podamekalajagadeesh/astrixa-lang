use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LsBuilder};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

mod diagnostics;
mod completion;
mod hover;
mod symbols;

use diagnostics::DiagnosticsEngine;
use completion::CompletionProvider;
use hover::HoverProvider;
use symbols::SymbolProvider;

#[derive(Clone)]
struct AstrixaLanguageServer {
    client: Client,
    documents: Arc<RwLock<HashMap<String, String>>>,
    diagnostics: Arc<DiagnosticsEngine>,
    completion: Arc<CompletionProvider>,
    hover: Arc<HoverProvider>,
    symbols: Arc<SymbolProvider>,
}

#[tower_lsp::async_trait]
impl LanguageServer for AstrixaLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        info!("ASTRIXA LSP Server initializing");
        
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![":".to_string(), ".".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverServerCapabilities::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        inter_file_dependencies: false,
                        work_du_file_diagnostics: false,
                        ..Default::default()
                    }
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "ASTRIXA".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("ASTRIXA LSP Server initialized");
        self.client
            .log_message(MessageType::INFO, "ASTRIXA LSP Server started")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        info!("ASTRIXA LSP Server shutting down");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text;

        info!("Document opened: {}", uri);
        
        // Store document
        self.documents.write().await.insert(uri.clone(), text.clone());
        
        // Run diagnostics
        self.diagnostics.check(&uri, &text, &self.client).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        
        if let Some(change) = params.content_changes.first() {
            let text = match change {
                TextDocumentContentChangeEvent {
                    range: None,
                    text,
                    ..
                } => text.clone(),
                _ => return,
            };

            info!("Document changed: {}", uri);
            
            // Update document
            self.documents.write().await.insert(uri.clone(), text.clone());
            
            // Run diagnostics on change
            self.diagnostics.check(&uri, &text, &self.client).await;
        }
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri.to_string();
        let position = params.text_document_position.position;
        
        let documents = self.documents.read().await;
        if let Some(text) = documents.get(&uri) {
            let completions = self.completion.get_completions(text, position).await;
            Ok(Some(CompletionResponse::Array(completions)))
        } else {
            Ok(None)
        }
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri.to_string();
        let position = params.text_document_position_params.position;
        
        let documents = self.documents.read().await;
        if let Some(text) = documents.get(&uri) {
            Ok(self.hover.get_hover(text, position).await)
        } else {
            Ok(None)
        }
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri.to_string();
        let position = params.text_document_position_params.position;
        
        let documents = self.documents.read().await;
        if let Some(text) = documents.get(&uri) {
            let location = self.symbols.get_definition_location(text, position).await;
            Ok(location.map(GotoDefinitionResponse::Scalar))
        } else {
            Ok(None)
        }
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri.to_string();
        
        let documents = self.documents.read().await;
        if let Some(text) = documents.get(&uri) {
            let symbols = self.symbols.get_document_symbols(text).await;
            Ok(Some(DocumentSymbolResponse::Flat(symbols)))
        } else {
            Ok(None)
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LsBuilder::new(AstrixaLanguageServer {
        client: Client::new(),
        documents: Arc::new(RwLock::new(HashMap::new())),
        diagnostics: Arc::new(DiagnosticsEngine::new()),
        completion: Arc::new(CompletionProvider::new()),
        hover: Arc::new(HoverProvider::new()),
        symbols: Arc::new(SymbolProvider::new()),
    })
    .build_socket(stdin, stdout);

    info!("ASTRIXA LSP starting");
    tower_lsp::run(service, socket)
        .await
        .expect("LSP server crashed");
}
