import * as path from 'path';
import { workspace, ExtensionContext, window } from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  // Get the LSP binary path from configuration
  const config = workspace.getConfiguration('astrixa');
  const lspPath = config.get<string>('lsp.path') || 'astrixa-lsp';

  // Try to find the LSP binary
  const serverModule = path.join(context.extensionPath, '..', 'lsp', 'target', 'debug', lspPath);
  
  window.showInformationMessage(`ASTRIXA LSP starting (${lspPath})`);

  // Configure server
  const serverOptions: ServerOptions = {
    run: { command: lspPath, transport: TransportKind.STDIO },
    debug: { command: lspPath, transport: TransportKind.STDIO },
  };

  // Configure language client
  const clientOptions: LanguageClientOptions = {
    // Register the client for astrixa documents
    documentSelector: [{ scheme: 'file', language: 'astrixa' }],
    synchronize: {
      // Notify the server about file changes to '.ax' files
      fileEvents: workspace.createFileSystemWatcher('**/*.ax'),
    },
  };

  // Create and start the language client
  client = new LanguageClient('astrixa', 'ASTRIXA Language Server', serverOptions, clientOptions);

  // Start the client
  const disposable = client.start();
  context.subscriptions.push(disposable);

  // Log initialization
  client.onReady().then(() => {
    window.showInformationMessage('ASTRIXA LSP Server running');
  }).catch(error => {
    window.showErrorMessage(`ASTRIXA LSP failed to start: ${error}`);
  });
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
