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

// ASTRIXA Standard Library
// Each function here is callable from ASTRIXA code
const astrixaStdlib = {
  env: {
    // Print a value to console (no newline)
    print: (value) => {
      process.stdout.write(value.toString());
    },

    // Print a value with newline
    println: (value) => {
      console.log(value);
    },

    // Additional stdlib functions can be added here
    // Example: read, write, malloc, free, etc.
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
