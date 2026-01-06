import * as path from 'path';
import { workspace, ExtensionContext, window, commands } from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  // Get configuration
  const config = workspace.getConfiguration('astrixa');
  const lspPath = config.get<string>('lsp.path') || 'astrixa-lsp';
  const debugMode = config.get<boolean>('lsp.debug') || false;

  // Try multiple paths for LSP binary
  const possiblePaths = [
    lspPath, // User-configured path
    path.join(context.extensionPath, '..', '..', 'lsp', 'target', 'release', 'astrixa-lsp'),
    path.join(context.extensionPath, '..', '..', 'lsp', 'target', 'debug', 'astrixa-lsp'),
    '/usr/local/bin/astrixa-lsp',
    path.join(process.env.HOME || '', '.cargo', 'bin', 'astrixa-lsp'),
  ];

  let serverCommand = lspPath;

  // Try to find the binary
  const fs = require('fs');
  for (const testPath of possiblePaths) {
    if (fs.existsSync(testPath)) {
      serverCommand = testPath;
      break;
    }
  }

  if (debugMode) {
    window.showInformationMessage(`ASTRIXA LSP: Using binary at ${serverCommand}`);
  }

  // Configure server options
  const serverOptions: ServerOptions = {
    run: {
      command: serverCommand,
      transport: TransportKind.stdio,
    },
    debug: {
      command: serverCommand,
      transport: TransportKind.stdio,
      options: {
        env: {
          ...process.env,
          RUST_LOG: 'info',
          RUST_BACKTRACE: '1',
        },
      },
    },
  };

  // Configure language client options
  const clientOptions: LanguageClientOptions = {
    // Register the server for ASTRIXA documents
    documentSelector: [
      { scheme: 'file', language: 'astrixa' },
      { scheme: 'untitled', language: 'astrixa' },
    ],
    synchronize: {
      // Notify server about file changes to '.ax' files
      fileEvents: workspace.createFileSystemWatcher('**/*.ax'),
    },
    outputChannelName: 'ASTRIXA Language Server',
    revealOutputChannelOn: debugMode ? 1 : 4, // Show on error only unless debug mode
  };

  // Create and start the language client
  client = new LanguageClient(
    'astrixa-lsp',
    'ASTRIXA Language Server',
    serverOptions,
    clientOptions
  );

  // Register commands
  context.subscriptions.push(
    commands.registerCommand('astrixa.restartServer', async () => {
      window.showInformationMessage('Restarting ASTRIXA Language Server...');
      await client.stop();
      await client.start();
      window.showInformationMessage('ASTRIXA Language Server restarted');
    })
  );

  context.subscriptions.push(
    commands.registerCommand('astrixa.showOutput', () => {
      client.outputChannel.show();
    })
  );

  // Start the client
  const disposable = client.start();
  context.subscriptions.push(disposable);

  // Log successful initialization
  client
    .onReady()
    .then(() => {
      window.showInformationMessage('✓ ASTRIXA LSP Server running');
      
      if (debugMode) {
        window.showInformationMessage(
          'Debug mode enabled. Check Output panel for logs.'
        );
      }
    })
    .catch((error) => {
      window.showErrorMessage(
        `ASTRIXA LSP failed to start: ${error.message}\n\n` +
        `Make sure the LSP binary is built:\n` +
        `cd lsp && cargo build --release`
      );
      
      // Show output channel with error details
      client.outputChannel.show();
    });
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}

