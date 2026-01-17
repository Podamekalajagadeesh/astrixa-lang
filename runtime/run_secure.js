#!/usr/bin/env node

/**
 * ASTRIXA WASM Runtime with Capability-Based Security
 * 
 * Design principles:
 * - Thin runtime (minimal logic)
 * - Host-powered (uses Node.js/JS capabilities)
 * - Secure by default (capability system)
 * - Default-deny (all operations blocked unless explicitly allowed)
 * - Extensible (easy to add more stdlib functions)
 * 
 * Security Model:
 * - Each module has a set of capabilities
 * - Operations check capabilities before executing
 * - Any denied operation throws a SecurityError
 * - Memory access is sandboxed
 */

const fs = require("fs");
const path = require("path");
const crypto = require("crypto");

// ============================================================
// CAPABILITY SYSTEM
// ============================================================

/**
 * Capability flags - what operations are allowed
 */
const Capabilities = {
  // I/O
  IO_PRINT: "io.print",
  IO_INPUT: "io.input",
  
  // File System
  FS_READ: "fs.read",
  FS_WRITE: "fs.write",
  FS_DELETE: "fs.delete",
  FS_STAT: "fs.stat",
  
  // Network
  NET_CONNECT: "net.connect",
  NET_LISTEN: "net.listen",
  NET_HTTP: "net.http",
  
  // Process
  PROCESS_EXIT: "process.exit",
  PROCESS_EXEC: "process.exec",
  
  // Crypto
  CRYPTO_SIGN: "crypto.sign",
  CRYPTO_VERIFY: "crypto.verify",

  // Web3
  WEB3: "web3",
  
  // Memory
  MEM_UNSAFE: "mem.unsafe",
};

/**
 * Default capabilities - what's allowed by default
 */
const DEFAULT_CAPABILITIES = new Set([
  Capabilities.IO_PRINT,  // Can print
  // Note: Everything else is DENIED by default
]);

/**
 * Security context - tracks capabilities and enforces checks
 */
class SecurityContext {
  constructor(capabilities = DEFAULT_CAPABILITIES) {
    this.capabilities = new Set(capabilities);
  }
  
  /**
   * Check if a capability is granted
   * @throws {SecurityError} if capability is not granted
   */
  require(capability) {
    if (!this.capabilities.has(capability)) {
      throw new SecurityError(
        `Operation denied: ${capability}`,
        "CAPABILITY_DENIED"
      );
    }
  }
  
  /**
   * Check if a capability is granted (returns boolean)
   */
  has(capability) {
    return this.capabilities.has(capability);
  }
  
  /**
   * Grant additional capabilities
   */
  grant(...caps) {
    caps.forEach(cap => this.capabilities.add(cap));
  }
  
  /**
   * Revoke capabilities
   */
  revoke(...caps) {
    caps.forEach(cap => this.capabilities.delete(cap));
  }
}

/**
 * Custom SecurityError for denied operations
 */
class SecurityError extends Error {
  constructor(message, code = "SECURITY_ERROR") {
    super(message);
    this.name = "SecurityError";
    this.code = code;
  }
}

// ============================================================
// RUNTIME STATE
// ============================================================

let memoryInstance = null;
let securityContext = new SecurityContext();

// ============================================================
// STDLIB FUNCTIONS WITH SECURITY
// ============================================================

const astrixaStdlib = {
  env: {
    // ==========================================
    // CORE I/O FUNCTIONS
    // ==========================================
    
    print_str: (ptr, len) => {
      // Check capability
      securityContext.require(Capabilities.IO_PRINT);
      
      if (!memoryInstance) {
        throw new SecurityError("Memory not initialized");
      }
      
      if (ptr < 0 || len < 0 || ptr + len > memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const text = new TextDecoder("utf-8").decode(bytes);
      process.stdout.write(text);
    },

    println_str: (ptr, len) => {
      securityContext.require(Capabilities.IO_PRINT);
      
      if (!memoryInstance) {
        throw new SecurityError("Memory not initialized");
      }
      
      if (ptr < 0 || len < 0 || ptr + len > memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const text = new TextDecoder("utf-8").decode(bytes);
      console.log(text);
    },

    input: () => {
      securityContext.require(Capabilities.IO_INPUT);
      console.warn("input() not yet implemented");
      return 0;
    },

    len: (ptr) => {
      if (!memoryInstance) return 0;
      if (ptr < 0 || ptr >= memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      
      let len = 0;
      const bytes = new Uint8Array(memoryInstance.buffer);
      while (bytes[ptr + len] !== 0 && ptr + len < bytes.length) {
        len++;
      }
      return len;
    },

    exit: (code) => {
      securityContext.require(Capabilities.PROCESS_EXIT);
      console.log(`\n🔚 Program exited with code: ${code}`);
      process.exit(code);
    },

    panic: (ptr, len) => {
      if (!memoryInstance) {
        throw new SecurityError("Memory not initialized");
      }
      
      if (ptr < 0 || len < 0 || ptr + len > memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const message = new TextDecoder("utf-8").decode(bytes);
      console.error(`\n❌ ASTRIXA PANIC: ${message}`);
      process.exit(1);
    },

    // ==========================================
    // FILE SYSTEM FUNCTIONS (WITH SECURITY)
    // ==========================================
    
    /**
     * fs.read(path_ptr, path_len) -> ptr to file contents
     * Requires: FS_READ capability
     */
    fs_read: (pathPtr, pathLen) => {
      // SECURITY: Require capability
      securityContext.require(Capabilities.FS_READ);
      
      if (!memoryInstance) {
        throw new SecurityError("Memory not initialized");
      }
      
      if (pathPtr < 0 || pathLen < 0 || pathPtr + pathLen > memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      
      // Read the path from memory
      const pathBytes = new Uint8Array(memoryInstance.buffer, pathPtr, pathLen);
      const filePath = new TextDecoder("utf-8").decode(pathBytes);
      
      try {
        // Read the file
        const content = fs.readFileSync(filePath, "utf-8");
        
        // Store in memory and return pointer
        // Not implemented in secure runtime; returning 0 (no data pointer)
        console.log(`[fs.read] File: ${filePath}`);
        return 0;
      } catch (error) {
        throw new SecurityError(`File read error: ${error.message}`);
      }
    },
    
    /**
     * fs.write(path_ptr, path_len, data_ptr, data_len)
     * Requires: FS_WRITE capability
     */
    fs_write: (pathPtr, pathLen, dataPtr, dataLen) => {
      securityContext.require(Capabilities.FS_WRITE);
      
      if (!memoryInstance) {
        throw new SecurityError("Memory not initialized");
      }
      
      // Validate memory bounds
      if (pathPtr < 0 || pathLen < 0 || pathPtr + pathLen > memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      if (dataPtr < 0 || dataLen < 0 || dataPtr + dataLen > memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      
      const pathBytes = new Uint8Array(memoryInstance.buffer, pathPtr, pathLen);
      const dataBytes = new Uint8Array(memoryInstance.buffer, dataPtr, dataLen);
      const filePath = new TextDecoder("utf-8").decode(pathBytes);
      const fileData = Buffer.from(dataBytes);
      
      try {
        fs.writeFileSync(filePath, fileData);
        return 0;
      } catch (error) {
        throw new SecurityError(`File write error: ${error.message}`);
      }
    },
    
    /**
     * fs.delete(path_ptr, path_len)
     * Requires: FS_DELETE capability
     */
    fs_delete: (pathPtr, pathLen) => {
      securityContext.require(Capabilities.FS_DELETE);
      
      if (!memoryInstance) {
        throw new SecurityError("Memory not initialized");
      }
      
      if (pathPtr < 0 || pathLen < 0 || pathPtr + pathLen > memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      
      const pathBytes = new Uint8Array(memoryInstance.buffer, pathPtr, pathLen);
      const filePath = new TextDecoder("utf-8").decode(pathBytes);
      
      try {
        fs.unlinkSync(filePath);
        return 0;
      } catch (error) {
        throw new SecurityError(`File delete error: ${error.message}`);
      }
    },

    // ==========================================
    // AI FUNCTIONS (MOCK IMPLEMENTATION)
    // ==========================================
    
    /**
     * ai.generate(prompt_ptr, prompt_len) -> i32
     * ALPHA: Mock implementation - returns fixed response
     * No capability check needed (fallback available)
     */
    ai_generate: (promptPtr, promptLen) => {
      if (!memoryInstance) {
        throw new SecurityError("Memory not initialized");
      }
      
      if (promptPtr < 0 || promptLen < 0 || promptPtr + promptLen > memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      
      try {
        const promptBytes = new Uint8Array(memoryInstance.buffer, promptPtr, promptLen);
        const prompt = new TextDecoder("utf-8").decode(promptBytes);
        
        // ALPHA: Mock AI response based on prompt
        let response = "";
        if (prompt.toLowerCase().includes("hello")) {
          response = "Hello! I'm ASTRIXA AI. How can I help you?";
        } else if (prompt.toLowerCase().includes("what")) {
          response = "I'm an AI assistant running in the ASTRIXA runtime.";
        } else {
          response = `[AI Mock] Response to: "${prompt.substring(0, 30)}..."`;
        }
        
        // Write response to memory at offset 256 (after common data)
        const respBytes = new TextEncoder().encode(response);
        const respPtr = 256;
        if (respPtr + respBytes.length > memoryInstance.buffer.byteLength) {
          throw new SecurityError("Not enough memory for AI response");
        }
        
        const memView = new Uint8Array(memoryInstance.buffer, respPtr, respBytes.length);
        memView.set(respBytes);
        
        // Return success (0)
        return 0;
      } catch (error) {
        if (error instanceof SecurityError) throw error;
        throw new SecurityError(`AI generation error: ${error.message}`);
      }
    },
    
    /**
     * ai.embed(text_ptr, text_len) -> i32
     * ALPHA: Mock implementation - returns fixed embedding
     */
    ai_embed: (textPtr, textLen) => {
      if (!memoryInstance) {
        throw new SecurityError("Memory not initialized");
      }
      
      if (textPtr < 0 || textLen < 0 || textPtr + textLen > memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      
      try {
        const textBytes = new Uint8Array(memoryInstance.buffer, textPtr, textLen);
        const text = new TextDecoder("utf-8").decode(textBytes);
        
        // ALPHA: Mock embedding - just return a hash-like value
        let hash = 0;
        for (let i = 0; i < text.length; i++) {
          hash = ((hash << 5) - hash) + text.charCodeAt(i);
          hash = hash & hash; // Convert to 32-bit integer
        }
        
        return Math.abs(hash);
      } catch (error) {
        if (error instanceof SecurityError) throw error;
        throw new SecurityError(`AI embedding error: ${error.message}`);
      }
    },
    
    /**
     * ai.classify(text_ptr, text_len) -> i32
     * ALPHA: Mock implementation - returns classification code
     */
    ai_classify: (textPtr, textLen) => {
      if (!memoryInstance) {
        throw new SecurityError("Memory not initialized");
      }
      
      if (textPtr < 0 || textLen < 0 || textPtr + textLen > memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      
      try {
        const textBytes = new Uint8Array(memoryInstance.buffer, textPtr, textLen);
        const text = new TextDecoder("utf-8").decode(textBytes);
        
        // ALPHA: Mock classification based on text content
        if (text.length === 0) return 0; // UNKNOWN
        if (text.toLowerCase().includes("hello") || text.toLowerCase().includes("hi")) return 1; // GREETING
        if (text.includes("?")) return 2; // QUESTION
        if (text.includes("!")) return 3; // EXCLAMATION
        return 4; // OTHER
      } catch (error) {
        if (error instanceof SecurityError) throw error;
        throw new SecurityError(`AI classification error: ${error.message}`);
      }
    },

    // ==========================================
    // MATH FUNCTIONS (ALWAYS SAFE)
    // ==========================================
    
    abs: (n) => Math.abs(n),
    pow: (base, exp) => Math.pow(base, exp) | 0,
    sqrt: (n) => Math.sqrt(n) | 0,
    min: (a, b) => Math.min(a, b),
    max: (a, b) => Math.max(a, b),
    rand: (max) => Math.floor(Math.random() * max),

    // ==========================================
    // TIME FUNCTIONS (ALWAYS SAFE)
    // ==========================================
    
    time: () => Date.now() | 0,
    sleep: (ms) => {
      // Sync sleep not supported
      console.warn("sleep() not supported in sync WASM");
    },

    // ==========================================
    // CRYPTO FUNCTIONS (WITH SECURITY)
    // ==========================================
    
    hash: (ptr, len) => {
      // Crypto operations should probably require explicit capability
      // For now, allow hashing as it's read-only
      if (!memoryInstance) return 0;
      if (ptr < 0 || len < 0 || ptr + len > memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const data = Buffer.from(bytes);
      const hash = crypto.createHash("sha256").update(data).digest("hex");
      console.log(`[hash] ${hash}`);
      return 0;
    },

    keccak: (ptr, len) => {
      if (!memoryInstance) return 0;
      if (ptr < 0 || len < 0 || ptr + len > memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      // Keccak would require additional library
      console.log(`[keccak] hash operation`);
      return 0;
    },

    sha256: (ptr, len) => {
      if (!memoryInstance) return 0;
      if (ptr < 0 || len < 0 || ptr + len > memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      
      const bytes = new Uint8Array(memoryInstance.buffer, ptr, len);
      const data = Buffer.from(bytes);
      const hash = crypto.createHash("sha256").update(data).digest("hex");
      console.log(`[sha256] ${hash}`);
      return 0;
    },

    // ==========================================
    // WEB3 FUNCTIONS (MOCKED)
    // ==========================================

    /**
     * web3.wallet() -> i32
     * Returns success (mock wallet). No keys stored here.
     */
    web3_wallet: () => {
      securityContext.require(Capabilities.WEB3);
      console.log(`[web3.wallet] returning mock wallet handle`);
      return 0;
    },

    /**
     * web3.sign(msg_ptr, msg_len) -> i32
     * Mock signing: logs message and returns success code.
     */
    web3_sign: (msgPtr, msgLen) => {
      securityContext.require(Capabilities.WEB3);
      if (!memoryInstance) return 0;
      if (msgPtr < 0 || msgLen < 0 || msgPtr + msgLen > memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      const bytes = new Uint8Array(memoryInstance.buffer, msgPtr, msgLen);
      const msg = new TextDecoder("utf-8").decode(bytes);
      console.log(`[web3.sign] mock-signed message: ${msg.substring(0, 64)}${msg.length > 64 ? "..." : ""}`);
      return 0;
    },

    /**
     * web3.verify(msg_ptr, msg_len, sig_ptr, sig_len) -> i32
     * Mock verify: always returns 1 (true) when capability granted.
     */
    web3_verify: (msgPtr, msgLen, sigPtr, sigLen) => {
      securityContext.require(Capabilities.WEB3);
      if (!memoryInstance) return 1;
      if (
        msgPtr < 0 || msgLen < 0 || msgPtr + msgLen > memoryInstance.buffer.byteLength ||
        sigPtr < 0 || sigLen < 0 || sigPtr + sigLen > memoryInstance.buffer.byteLength
      ) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      console.log(`[web3.verify] mock-verified signature`);
      return 1;
    },

    /**
     * web3.keccak(data_ptr, data_len) -> i32
     * Mock keccak: logs and returns success.
     */
    web3_keccak: (ptr, len) => {
      securityContext.require(Capabilities.WEB3);
      if (!memoryInstance) return 0;
      if (ptr < 0 || len < 0 || ptr + len > memoryInstance.buffer.byteLength) {
        throw new SecurityError("Memory access out of bounds", "MEM_BOUNDS");
      }
      console.log(`[web3.keccak] mock hash`);
      return 0;
    },

    /**
     * web3.balance(addr_ptr) -> i32
     * Mock balance: returns 0 (no funds).
     */
    web3_balance: (addrPtr) => {
      securityContext.require(Capabilities.WEB3);
      console.log(`[web3.balance] mock balance 0`);
      return 0;
    },

    /**
     * web3.send(addr_ptr, amount_ptr) -> i32
     * Mock send: logs and returns success.
     */
    web3_send: (addrPtr, amountPtr) => {
      securityContext.require(Capabilities.WEB3);
      console.log(`[web3.send] mock transfer`);
      return 0;
    },
  }
};

// ============================================================
// WASM LOADING AND EXECUTION
// ============================================================

/**
 * Load and run a WASM binary with security context
 */
async function runWasm(wasmPath, capabilities = DEFAULT_CAPABILITIES) {
  try {
    // Initialize security context
    securityContext = new SecurityContext(capabilities);
    
    console.log("🔒 ASTRIXA Runtime - Security Model: Default-Deny");
    console.log(`   Capabilities: ${Array.from(securityContext.capabilities).join(", ")}\n`);
    
    // Read WASM file
    const wasmBuffer = fs.readFileSync(wasmPath);
    
    // Instantiate WASM module with stdlib imports
    const wasmModule = await WebAssembly.instantiate(wasmBuffer, astrixaStdlib);
    const exports = wasmModule.instance.exports;
    
    // Set memory instance
    if (exports.memory) {
      memoryInstance = exports.memory;
    }

    console.log("🚀 ASTRIXA Runtime - Executing WASM\n");

    // Call main function
    if (exports.main) {
      try {
        const result = exports.main();
        console.log(`\n✅ Program completed (exit code: ${result})`);
        return result;
      } catch (error) {
        if (error instanceof SecurityError) {
          console.error(`\n❌ ${error.name}: ${error.message}`);
          process.exit(1);
        }
        throw error;
      }
    } else {
      console.error("❌ Error: No 'main' function found");
      process.exit(1);
    }
  } catch (error) {
    if (error instanceof SecurityError) {
      console.error(`❌ ${error.name}: ${error.message}`);
    } else {
      console.error("❌ Runtime Error:", error.message);
    }
    process.exit(1);
  }
}

/**
 * Load WAT and convert to WASM
 */
async function runWat(watPath, capabilities = DEFAULT_CAPABILITIES) {
  try {
    const { execSync } = require("child_process");
    const wasmPath = watPath.replace(/\.wat$/, ".wasm");
    
    console.log("🔧 Converting WAT to WASM...");
    execSync(`wat2wasm ${watPath} -o ${wasmPath}`, { stdio: "inherit" });
    
    await runWasm(wasmPath, capabilities);
  } catch (error) {
    console.error("❌ Error converting WAT to WASM:", error.message);
    console.error("\n💡 Tip: Install WABT toolkit: npm install -g wabt");
    process.exit(1);
  }
}

// ============================================================
// MAIN ENTRY POINT
// ============================================================

async function main() {
  const args = process.argv.slice(2);

  if (args.length === 0) {
    console.log("ASTRIXA WASM Runtime - Secure Execution Environment");
    console.log("\nUsage:");
    console.log("  node run.js <file.wasm>    Run compiled WASM");
    console.log("  node run.js <file.wat>     Run WAT (converts first)");
    console.log("\nEnvironment Variables:");
    console.log("  ASTRIXA_CAPABILITIES      Comma-separated list of allowed capabilities");
    console.log("  ASTRIXA_ALLOW_FS_READ     Enable file read access");
    console.log("  ASTRIXA_ALLOW_FS_WRITE    Enable file write access");
    console.log("  ASTRIXA_ALLOW_PROCESS_EXIT Enable process.exit()");
    console.log("  ASTRIXA_ALLOW_WEB3        Enable web3 mocked ops (wallet/sign/send)");
    console.log("\nExample:");
    console.log("  node run.js program.wasm");
    console.log("  ASTRIXA_ALLOW_FS_READ=1 node run.js program.wasm");
    process.exit(0);
  }

  // Parse capabilities from environment
  let capabilities = new Set(DEFAULT_CAPABILITIES);
  
  if (process.env.ASTRIXA_ALLOW_FS_READ) {
    capabilities.add(Capabilities.FS_READ);
  }
  if (process.env.ASTRIXA_ALLOW_FS_WRITE) {
    capabilities.add(Capabilities.FS_WRITE);
  }
  if (process.env.ASTRIXA_ALLOW_FS_DELETE) {
    capabilities.add(Capabilities.FS_DELETE);
  }
  if (process.env.ASTRIXA_ALLOW_PROCESS_EXIT) {
    capabilities.add(Capabilities.PROCESS_EXIT);
  }
  if (process.env.ASTRIXA_ALLOW_WEB3) {
    capabilities.add(Capabilities.WEB3);
  }
  if (process.env.ASTRIXA_CAPABILITIES) {
    const caps = process.env.ASTRIXA_CAPABILITIES.split(",");
    caps.forEach(cap => capabilities.add(cap.trim()));
  }

  const filePath = args[0];

  if (!fs.existsSync(filePath)) {
    console.error(`❌ Error: File not found: ${filePath}`);
    process.exit(1);
  }

  const ext = path.extname(filePath);

  if (ext === ".wasm") {
    await runWasm(filePath, capabilities);
  } else if (ext === ".wat") {
    await runWat(filePath, capabilities);
  } else {
    console.error(`❌ Error: Unsupported file type: ${ext}`);
    process.exit(1);
  }
}

main().catch((error) => {
  console.error("❌ Fatal error:", error);
  process.exit(1);
});
