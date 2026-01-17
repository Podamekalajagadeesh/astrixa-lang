#!/usr/bin/env node

/**
 * ASTRIXA WASM Runtime
 * 
 * This is the host runtime that provides standard library functions
 * to ASTRIXA programs compiled to WebAssembly.
 * 
 * Design principles:
 * - Thin runtime (minimal logic)
 * - Host-powered (uses Node.js/JS capabilities)
 * - Deterministic-friendly (Web3 safe)
 * - Extensible (easy to add more stdlib functions)
 */

const fs = require("fs");
const path = require("path");
const crypto = require("crypto");

// ASTRIXA Standard Library
// Each function here is callable from ASTRIXA code
let memoryInstance = null; // Will be set when module is instantiated

const astrixaStdlib = {
  env: {
    // ==========================================
    // CORE I/O FUNCTIONS
    // ==========================================
    
    // Print a string to console (no newline)
    // Parameters: ptr (i32), len (i32)
    print_str: (ptr, len) => {
      if (!memoryInstance) {
        console.error("Memory not initialized");
        return;
      }
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const text = new TextDecoder("utf-8").decode(bytes);
      process.stdout.write(text);
    },

    // Print a string with newline
    // Parameters: ptr (i32), len (i32)
    println_str: (ptr, len) => {
      if (!memoryInstance) {
        console.error("Memory not initialized");
        return;
      }
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const text = new TextDecoder("utf-8").decode(bytes);
      console.log(text);
    },

    // Read a line from stdin (returns ptr to string)
    // Returns: i32 (pointer to string in memory)
    input: () => {
      // Not implemented in v0.1.0; returns 0 (empty string pointer)
      console.warn("input() is not implemented in this version");
      return 0;
    },

    // Get length of a string
    // Parameters: ptr (i32)
    // Returns: i32 (length)
    len: (ptr) => {
      if (!memoryInstance) return 0;
      // Assume null-terminated string for now
      let len = 0;
      const bytes = new Uint8Array(memoryInstance.buffer);
      while (bytes[ptr + len] !== 0 && ptr + len < bytes.length) {
        len++;
      }
      return len;
    },

    // Exit program with status code
    // Parameters: code (i32)
    exit: (code) => {
      console.log(`\n🔚 Program exited with code: ${code}`);
      process.exit(code);
    },

    // STEP 48: Panic handler - abort with error message
    // Parameters: ptr (i32), len (i32)
    panic: (ptr, len) => {
      if (!memoryInstance) {
        console.error("⚠️  ASTRIXA PANIC: Memory not initialized");
        process.exit(1);
      }
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const message = new TextDecoder("utf-8").decode(bytes);
      console.error(`\n❌ ASTRIXA PANIC: ${message}`);
      console.error("Program terminated.\n");
      process.exit(1);
    },

    // ==========================================
    // MATH FUNCTIONS
    // ==========================================
    
    // Absolute value
    abs: (n) => Math.abs(n),
    
    // Power function (base^exp)
    pow: (base, exp) => Math.pow(base, exp) | 0,
    
    // Square root (integer)
    sqrt: (n) => Math.sqrt(n) | 0,
    
    // Minimum of two numbers
    min: (a, b) => Math.min(a, b),
    
    // Maximum of two numbers
    max: (a, b) => Math.max(a, b),
    
    // Random integer in range [0, max)
    rand: (max) => Math.floor(Math.random() * max),

    // ==========================================
    // TIME FUNCTIONS
    // ==========================================
    
    // Current Unix timestamp in milliseconds
    time: () => Date.now() | 0,
    
    // Sleep for specified milliseconds
    // Blocking sleep is not supported in synchronous WASM environments
    sleep: (ms) => {
      console.warn(`sleep(${ms}) called - blocking sleep is not supported in synchronous WASM runtime`);
      // No-op in the current synchronous runtime
    },

    // ==========================================
    // CRYPTO FUNCTIONS (Web3-ready)
    // ==========================================
    
    // Generic hash (defaults to keccak256)
    // Parameters: ptr (i32), len (i32)
    // Returns: i32 (pointer to hash string)
    hash: (ptr, len) => {
      if (!memoryInstance) return 0;
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const data = Buffer.from(bytes);
      
      // Use keccak256 (requires ethereum-cryptography or similar)
      // For now, use sha256 as fallback
      const hash = crypto.createHash("sha256").update(data).digest("hex");
      console.log(`[hash] ${hash}`);
      
      // Hash string is not stored back to memory in v0.1.0; returning 0
      return 0;
    },
    
    // Keccak-256 hash (Ethereum standard)
    // Parameters: ptr (i32), len (i32)
    // Returns: i32 (pointer to hash)
    keccak: (ptr, len) => {
      if (!memoryInstance) return 0;
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const data = Buffer.from(bytes);
      
      // Requires keccak256 library
      // Using SHA256 approximation in v0.1.0
      const hash = crypto.createHash("sha256").update(data).digest("hex");
      console.log(`[keccak256] ${hash} (using SHA256 approximation)`);
      
      return 0;
    },
    
    // SHA-256 hash
    // Parameters: ptr (i32), len (i32)
    // Returns: i32 (pointer to hash)
    sha256: (ptr, len) => {
      if (!memoryInstance) return 0;
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const data = Buffer.from(bytes);
      
      const hash = crypto.createHash("sha256").update(data).digest("hex");
      console.log(`[sha256] ${hash}`);
      
      return 0;
    },

    // ==========================================
    // LEGACY FUNCTIONS (backwards compatibility)
    // ==========================================
    
    // Legacy: Print a numeric value
    print: (value) => {
      process.stdout.write(value.toString());
    },

    // Legacy: Print a numeric value with newline
    println: (value) => {
      console.log(value);
    },

    // ==========================================
    // AI FUNCTIONS (STEP 52)
    // ==========================================
    
    // AI text generation (takes prompt, returns generated text)
    // Parameters: ptr (i32), len (i32)
    // Returns: i32 (pointer to generated string)
    ai_generate: (ptr, len) => {
      if (!memoryInstance) {
        console.error("Memory not initialized");
        return 0;
      }
      
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const prompt = new TextDecoder("utf-8").decode(bytes);
      
      // Mock AI response (in production, call OpenAI/Anthropic/local LLM)
      const response = `[AI Generated] Response to: "${prompt}"`;
      
      console.log(`🤖 AI Generate: ${prompt} -> ${response}`);
      
      // Write response back to memory (simple allocation at static offset)
      const responseBytes = new TextEncoder().encode(response);
      const outPtr = 2048; // Static offset for AI responses
      
      // Ensure we don't overflow memory
      if (outPtr + responseBytes.length < memoryInstance.buffer.byteLength) {
        new Uint8Array(memoryInstance.buffer, outPtr, responseBytes.length).set(responseBytes);
      }
      
      // Return pointer (WASM will need to know length too - for V1, caller manages)
      return outPtr;
    },
    
    // AI embedding generation
    // Parameters: ptr (i32), len (i32)
    // Returns: i32 (pointer to embedding vector)
    ai_embed: (ptr, len) => {
      if (!memoryInstance) {
        console.error("Memory not initialized");
        return 0;
      }
      
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const text = new TextDecoder("utf-8").decode(bytes);
      
      // Mock embedding (in production, call an embedding API)
      const embedding = `[0.1, 0.2, 0.3, ...]`; // Simulated vector
      
      console.log(`🔢 AI Embed: ${text} -> ${embedding}`);
      
      const embeddingBytes = new TextEncoder().encode(embedding);
      const outPtr = 3072; // Static offset for embeddings
      
      if (outPtr + embeddingBytes.length < memoryInstance.buffer.byteLength) {
        new Uint8Array(memoryInstance.buffer, outPtr, embeddingBytes.length).set(embeddingBytes);
      }
      
      return outPtr;
    },
    
    // AI text classification
    // Parameters: ptr (i32), len (i32)
    // Returns: i32 (pointer to classification result)
    ai_classify: (ptr, len) => {
      if (!memoryInstance) {
        console.error("Memory not initialized");
        return 0;
      }
      
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const text = new TextDecoder("utf-8").decode(bytes);
      
      // Mock classification (in production, call a classification API)
      const classification = "positive"; // Simulated sentiment
      
      console.log(`🏷️  AI Classify: ${text} -> ${classification}`);
      
      const classBytes = new TextEncoder().encode(classification);
      const outPtr = 4096; // Static offset for classifications
      
      if (outPtr + classBytes.length < memoryInstance.buffer.byteLength) {
        new Uint8Array(memoryInstance.buffer, outPtr, classBytes.length).set(classBytes);
      }
      
      return outPtr;
    },

    // ==========================================
    // WEB3 FUNCTIONS (STEP 53)
    // ==========================================
    
    // Get wallet address (mock for V1)
    // Returns: i32 (pointer to wallet address string)
    web3_wallet: () => {
      if (!memoryInstance) {
        console.error("Memory not initialized");
        return 0;
      }
      
      // Mock wallet address (in production, connect to MetaMask/WalletConnect)
      const walletAddress = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb5";
      
      console.log(`💼 Web3 Wallet: ${walletAddress}`);
      
      const walletBytes = new TextEncoder().encode(walletAddress);
      const outPtr = 5120; // Static offset for wallet data
      
      if (outPtr + walletBytes.length < memoryInstance.buffer.byteLength) {
        new Uint8Array(memoryInstance.buffer, outPtr, walletBytes.length).set(walletBytes);
      }
      
      return outPtr;
    },
    
    // Sign a message with wallet
    // Parameters: ptr (i32), len (i32) - message to sign
    // Returns: i32 (pointer to signature string)
    web3_sign: (ptr, len) => {
      if (!memoryInstance) {
        console.error("Memory not initialized");
        return 0;
      }
      
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const message = new TextDecoder("utf-8").decode(bytes);
      
      // V1: Mock signature using SHA256 (in production, use eth_sign or similar)
      const signature = crypto
        .createHash("sha256")
        .update(message)
        .digest("hex");
      
      console.log(`✍️  Web3 Sign: "${message}" -> 0x${signature.substring(0, 16)}...`);
      
      const sigBytes = new TextEncoder().encode("0x" + signature);
      const outPtr = 6144; // Static offset for signatures
      
      if (outPtr + sigBytes.length < memoryInstance.buffer.byteLength) {
        new Uint8Array(memoryInstance.buffer, outPtr, sigBytes.length).set(sigBytes);
      }
      
      return outPtr;
    },
    
    // Verify a signature
    // Parameters: msgPtr, msgLen, sigPtr, sigLen
    // Returns: i32 (1 for valid, 0 for invalid)
    web3_verify: (msgPtr, msgLen, sigPtr, sigLen) => {
      if (!memoryInstance) {
        console.error("Memory not initialized");
        return 0;
      }
      
      const msgBytes = new Uint8Array(memoryInstance.buffer, msgPtr, msgLen);
      const message = new TextDecoder("utf-8").decode(msgBytes);
      
      const sigBytes = new Uint8Array(memoryInstance.buffer, sigPtr, sigLen);
      const signature = new TextDecoder("utf-8").decode(sigBytes);
      
      // V1: Mock verification (in production, use ecrecover or similar)
      console.log(`✅ Web3 Verify: "${message}" with ${signature.substring(0, 16)}...`);
      
      // For mock, just return true
      return 1;
    },
    
    // Compute Keccak-256 hash (Ethereum standard)
    // Parameters: ptr (i32), len (i32)
    // Returns: i32 (pointer to hash string)
    web3_keccak: (ptr, len) => {
      if (!memoryInstance) {
        console.error("Memory not initialized");
        return 0;
      }
      
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const data = new TextDecoder("utf-8").decode(bytes);
      
      // V1: Use SHA3-256 as Keccak approximation (in production, use keccak256 from ethereum libs)
      const hash = crypto
        .createHash("sha3-256")
        .update(data)
        .digest("hex");
      
      console.log(`🔐 Web3 Keccak: "${data}" -> 0x${hash.substring(0, 16)}...`);
      
      const hashBytes = new TextEncoder().encode("0x" + hash);
      const outPtr = 7168; // Static offset for keccak hashes
      
      if (outPtr + hashBytes.length < memoryInstance.buffer.byteLength) {
        new Uint8Array(memoryInstance.buffer, outPtr, hashBytes.length).set(hashBytes);
      }
      
      return outPtr;
    },
    
    // Get wallet balance (mock for V1)
    // Parameters: ptr (i32) - wallet address pointer
    // Returns: i32 (balance in wei, as integer)
    web3_balance: (ptr) => {
      if (!memoryInstance) {
        console.error("Memory not initialized");
        return 0;
      }
      
      // V1: Mock balance (in production, query blockchain via RPC)
      const balance = 1000000000000000000; // 1 ETH in wei
      
      console.log(`💰 Web3 Balance: ${balance} wei (1 ETH)`);
      
      // For simplicity, return as integer (in reality, would need BigInt handling)
      return balance | 0; // Truncate to 32-bit
    },
    
    // Send transaction (mock for V1)
    // Parameters: txPtr (i32), txLen (i32) - transaction data
    // Returns: i32 (pointer to transaction hash)
    web3_send: (txPtr, txLen) => {
      if (!memoryInstance) {
        console.error("Memory not initialized");
        return 0;
      }
      
      const bytes = new Uint8Array(memoryInstance.buffer, txPtr, txLen);
      const txData = new TextDecoder("utf-8").decode(bytes);
      
      // V1: Mock transaction (in production, broadcast via Web3 provider)
      const txHash = crypto.randomBytes(32).toString("hex");
      
      console.log(`📤 Web3 Send: ${txData} -> 0x${txHash.substring(0, 16)}...`);
      
      const txHashBytes = new TextEncoder().encode("0x" + txHash);
      const outPtr = 8192; // Static offset for transaction hashes
      
      if (outPtr + txHashBytes.length < memoryInstance.buffer.byteLength) {
        new Uint8Array(memoryInstance.buffer, outPtr, txHashBytes.length).set(txHashBytes);
      }
      
      return outPtr;
    },
  },
};

/**
 * Load and execute a WASM module
 */
async function runWasm(wasmPath) {
  try {
    // Read the WASM file
    const wasmBuffer = fs.readFileSync(wasmPath);

    // Instantiate the WASM module with stdlib imports
    const wasmModule = await WebAssembly.instantiate(wasmBuffer, astrixaStdlib);

    // Get the exports
    const exports = wasmModule.instance.exports;
    
    // Set memory instance for stdlib functions
    if (exports.memory) {
      memoryInstance = exports.memory;
    }

    console.log("🚀 ASTRIXA Runtime - Executing WASM\n");

    // Call main function if it exists
    if (exports.main) {
      const result = exports.main();
      console.log(`\n✅ Program completed (exit code: ${result})`);
      return result;
    } else {
      console.error("❌ Error: No 'main' function found in WASM module");
      process.exit(1);
    }
  } catch (error) {
    console.error("❌ Runtime Error:", error.message);
    if (error.stack) {
      console.error(error.stack);
    }
    process.exit(1);
  }
}

/**
 * Load and execute a WAT (WebAssembly Text) file
 * by converting it to WASM first
 */
async function runWat(watPath) {
  try {
    // For WAT files, we need to convert them to WASM first
    // This requires the 'wat2wasm' tool from WABT
    const { execSync } = require("child_process");
    
    const wasmPath = watPath.replace(/\.wat$/, ".wasm");
    
    console.log("🔧 Converting WAT to WASM...");
    execSync(`wat2wasm ${watPath} -o ${wasmPath}`, { stdio: "inherit" });
    
    await runWasm(wasmPath);
  } catch (error) {
    console.error("❌ Error converting WAT to WASM:", error.message);
    console.error("\n💡 Tip: Make sure 'wat2wasm' is installed (from WABT toolkit)");
    console.error("   Install: npm install -g wabt");
    process.exit(1);
  }
}

// Main entry point
async function main() {
  const args = process.argv.slice(2);

  if (args.length === 0) {
    console.log("ASTRIXA WASM Runtime");
    console.log("\nUsage:");
    console.log("  node run.js <file.wasm>    Run compiled WASM");
    console.log("  node run.js <file.wat>     Run WAT (converts to WASM first)");
    console.log("\nExample:");
    console.log("  node run.js output.wasm");
    process.exit(0);
  }

  const filePath = args[0];

  if (!fs.existsSync(filePath)) {
    console.error(`❌ Error: File not found: ${filePath}`);
    process.exit(1);
  }

  const ext = path.extname(filePath);

  if (ext === ".wasm") {
    await runWasm(filePath);
  } else if (ext === ".wat") {
    await runWat(filePath);
  } else {
    console.error(`❌ Error: Unsupported file type: ${ext}`);
    console.error("   Supported: .wasm, .wat");
    process.exit(1);
  }
}

// Run the runtime
main().catch((error) => {
  console.error("❌ Fatal error:", error);
  process.exit(1);
});
