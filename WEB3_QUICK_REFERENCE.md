# ASTRIXA WEB3 QUICK REFERENCE

## Installation
```bash
curl -fsSL https://astrixa.dev/install.sh | sh
```

## Quick Start

### 1. Connect to Blockchain
```astrixa
import std::web3;
let provider = web3.connect("mainnet");
```

### 2. Create Wallet
```astrixa
let wallet = web3.wallet.fromPrivateKey(env("PRIVATE_KEY"));
// or
let wallet = web3.wallet.generate();
```

### 3. Get Balance
```astrixa
let balance = web3.balance("vitalik.eth");
println(web3.fromWei(balance, "ether") + " ETH");
```

### 4. Send Transaction
```astrixa
let tx = web3.send({
    from: wallet.address,
    to: "0x...",
    value: web3.toWei(1, "ether")
});
let receipt = web3.wait(tx.hash);
```

### 5. Load Contract
```astrixa
let usdc = web3.contract({
    address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    abi: import("./usdc_abi.json")
});
```

### 6. Read from Contract
```astrixa
let balance = usdc.call("balanceOf", [wallet.address]);
let symbol = usdc.call("symbol", []);
```

### 7. Write to Contract
```astrixa
let tx = usdc.send("transfer", [to, amount], {
    from: wallet.address,
    gasLimit: 100000
});
web3.wait(tx.hash);
```

### 8. Listen for Events
```astrixa
usdc.on("Transfer", fn(event) {
    println("Transfer from " + event.from + " to " + event.to);
});
```

## Smart Contracts

### Basic Contract
```astrixa
contract MyToken {
    state: ["name", "symbol", "balances"]
    
    fn initialize(name: string, symbol: string) {
        state["name"] = name;
        state["symbol"] = symbol;
    }
    
    fn balance_of(account: Address) -> U256 {
        return state["balances"][account] or 0;
    }
    
    fn transfer(to: Address, amount: U256) -> bool {
        require(to != "0x0", "Invalid address");
        
        let sender_balance = state["balances"][msg.sender] or 0;
        require(sender_balance >= amount, "Insufficient balance");
        
        state["balances"][msg.sender] = sender_balance - amount;
        state["balances"][to] = (state["balances"][to] or 0) + amount;
        
        emit("Transfer", {
            from: msg.sender,
            to: to,
            value: amount
        });
        
        return true;
    }
}
```

### Compile & Deploy
```bash
# Compile to bytecode
astrixa build token.ax --target=contract --output=token.bin

# Deploy to testnet
astrixa deploy token.ax --network=sepolia --wallet=wallet.json

# Deploy to mainnet
astrixa deploy token.ax --network=mainnet --wallet=wallet.json --verify
```

## Blockchain Context

### Available in Contracts
```astrixa
msg.sender     // Address - current transaction sender
msg.value      // U256 - ETH sent with transaction
msg.data       // Bytes - transaction data

tx.timestamp   // U256 - block timestamp
tx.gasprice    // U256 - gas price
tx.origin      // Address - original sender

chain.id       // U256 - chain ID (1 = Ethereum, 137 = Polygon)
chain.network  // string - "mainnet", "sepolia", etc.
```

### Built-in Functions
```astrixa
require(condition, "Error message")  // Assert condition
emit("EventName", { data })          // Emit event
```

## Multi-Chain Support

```astrixa
let eth = web3.connect("mainnet");
let polygon = web3.connect("polygon");
let arbitrum = web3.connect("arbitrum");

let eth_balance = eth.balance(address);
let polygon_balance = polygon.balance(address);
```

## ENS Support

```astrixa
// Resolve name to address
let addr = web3.ens.resolve("vitalik.eth");

// Reverse lookup
let name = web3.ens.reverse("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");

// Get text record
let email = web3.ens.text("vitalik.eth", "email");
```

## Utilities

```astrixa
// Wei conversion
let wei = web3.toWei(1, "ether");       // 1000000000000000000
let eth = web3.fromWei(wei, "ether");   // "1.0"
let gwei = web3.fromWei(wei, "gwei");   // "1000000000"

// Address validation
if web3.isValidAddress(addr) {
    // Valid Ethereum address
}

// Keccak hash
let hash = web3.keccak256("Hello World");

// ABI encoding
let encoded = web3.encodeABI("transfer", [to, amount]);
```

## Gas Estimation

```astrixa
let estimated = web3.estimateGas({
    from: wallet.address,
    to: contract.address,
    data: contract.encode("transfer", [to, amount])
});

// Add safety buffer
let safe_gas = estimated * 120 / 100;
```

## Event Listening

```astrixa
// Contract events
let subscription = usdc.on("Transfer", fn(event) {
    println("Transfer: " + event.value);
});

// New blocks
web3.onBlock(fn(block) {
    println("Block: " + block.number);
});

// Pending transactions
web3.onTransaction(fn(tx) {
    println("TX: " + tx.hash);
});

// Clean up
subscription.unsubscribe();
```

## Batch Requests (Multi-call)

```astrixa
let results = web3.multicall([
    usdc.call("balanceOf", [addr1]),
    usdc.call("balanceOf", [addr2]),
    usdc.call("totalSupply", [])
]);
```

## Transaction Building

```astrixa
let tx = web3.transaction()
    .from(wallet.address)
    .to("0x...")
    .value(web3.toWei(1, "ether"))
    .gas(100000)
    .gasPrice(web3.toWei(50, "gwei"))
    .nonce(web3.getTransactionCount(wallet.address))
    .build();

let signed = wallet.sign(tx);
let hash = web3.sendRawTransaction(signed);
```

## AI + Web3 (Unique to ASTRIXA)

```astrixa
contract AITrader {
    fn analyze_and_trade() {
        let price = dex.call("getPrice", ["WETH", "USDC"]);
        
        // Run AI on-chain
        let prediction = ai.infer(ai.model("trading_model"), {
            price: price,
            timestamp: tx.timestamp
        });
        
        if prediction.action == "buy" && prediction.confidence > 0.85 {
            dex.send("swap", ["USDC", "WETH", amount]);
        }
    }
}
```

## Security Best Practices

### ✅ DO
```astrixa
// Always validate inputs
require(to != "0x0", "Invalid address");
require(amount > 0, "Amount must be positive");
require(balance >= amount, "Insufficient balance");

// Check return values
let receipt = web3.wait(tx.hash);
if receipt.status != 1 {
    println("Transaction failed!");
}

// Emit events
emit("Transfer", { from: msg.sender, to: to, value: amount });

// Use Checks-Effects-Interactions pattern
require(condition);          // 1. Checks
state["balance"] -= amount;  // 2. Effects
external_call();             // 3. Interactions
```

### ❌ DON'T
```astrixa
// Don't skip validation
fn transfer(to: Address, amount: U256) {
    state["balances"][to] += amount;  // ❌ No checks!
}

// Don't ignore return values
contract.send("transfer", [to, amount]);  // ❌ Did it succeed?

// Don't use forbidden operations in contracts
let data = read("file.txt");     // ❌ Not allowed
let price = http_get("api.com"); // ❌ Not allowed
let r = random();                // ❌ Not allowed
```

## Contract Restrictions

### ❌ NOT ALLOWED in Contracts
- File I/O (`read`, `write`)
- Network (`http_get`, `http_post`)
- Random (`random`, `rand`)
- Dynamic allocation
- Floating point
- Unbounded loops

### ✅ ALLOWED in Contracts
- Deterministic logic
- State variables
- Blockchain context (`msg`, `tx`, `chain`)
- Events (`emit`)
- Validation (`require`)
- AI operations (deterministic)
- Crypto (`sha256`, `keccak256`)
- Math operations
- U256 arithmetic

## Testing

```astrixa
import std::test;

test "should transfer tokens" {
    let token = deploy("token.ax", ["MyToken", "MTK", 1000000]);
    
    let alice = test.account("alice");
    let bob = test.account("bob");
    
    test.as(alice, fn() {
        let result = token.call("transfer", [bob.address, 100]);
        assert(result == true);
    });
}

test "should reject invalid transfer" {
    let token = deploy("token.ax", ["MyToken", "MTK", 1000]);
    
    test.expect_revert(fn() {
        token.send("transfer", ["0x0", 100]);
    }, "Invalid address");
}
```

Run tests:
```bash
astrixa test test_token.ax
```

## CLI Commands

```bash
# Initialize project
astrixa init my-dapp

# Install package
astrixa install ai-tools

# List packages
astrixa list

# Run script
astrixa run script.ax

# Build for different targets
astrixa build app.ax --target=native
astrixa build contract.ax --target=contract
astrixa build ui.ax --target=wasm
astrixa build api.ax --target=web

# Deploy contract
astrixa deploy token.ax --network=sepolia --wallet=wallet.json

# Verify contract
astrixa verify token.ax --address=0x... --network=mainnet

# Run tests
astrixa test test.ax
```

## Common Patterns

### Check Balance Before Transfer
```astrixa
fn safe_transfer(to: Address, amount: U256) -> bool {
    let balance = state["balances"][msg.sender] or 0;
    require(balance >= amount, "Insufficient balance");
    
    state["balances"][msg.sender] = balance - amount;
    state["balances"][to] = (state["balances"][to] or 0) + amount;
    
    return true;
}
```

### Access Control
```astrixa
fn only_owner() {
    require(msg.sender == state["owner"], "Not authorized");
}

fn admin_function() {
    only_owner();
    // Admin-only logic
}
```

### Reentrancy Guard
```astrixa
fn withdraw(amount: U256) {
    require(!state["locked"], "Reentrancy");
    state["locked"] = true;
    
    // Update state before external call
    state["balances"][msg.sender] -= amount;
    
    // External call
    send_eth(msg.sender, amount);
    
    state["locked"] = false;
}
```

### Pausable
```astrixa
fn when_not_paused() {
    require(!state["paused"], "Contract is paused");
}

fn transfer(to: Address, amount: U256) -> bool {
    when_not_paused();
    // Transfer logic
}

fn pause() {
    only_owner();
    state["paused"] = true;
    emit("Paused", {});
}
```

## Types

### Core Types
- `Address` - Ethereum address (20 bytes)
- `U256` - Unsigned 256-bit integer
- `Bytes` - Dynamic byte array
- `string` - UTF-8 string
- `bool` - Boolean

### Web3 Types
- `Wallet` - Wallet with private key
- `Transaction` - Transaction object
- `TransactionReceipt` - Receipt after confirmation
- `Block` - Block data
- `Contract` - Contract instance
- `Event` - Event log

### Units
- `wei` - Base unit (1)
- `gwei` - 10^9 wei
- `ether` - 10^18 wei

## Network Shortcuts

```astrixa
// Mainnets
web3.connect("mainnet")    // Ethereum
web3.connect("polygon")    // Polygon
web3.connect("arbitrum")   // Arbitrum
web3.connect("optimism")   // Optimism
web3.connect("bsc")        // BNB Chain

// Testnets
web3.connect("sepolia")    // Ethereum Sepolia
web3.connect("goerli")     // Ethereum Goerli
web3.connect("mumbai")     // Polygon Mumbai

// Custom RPC
web3.connect("https://eth-mainnet.alchemyapi.io/v2/YOUR_KEY")
```

## Environment Variables

```bash
# .env file
PRIVATE_KEY=0x...
ETH_RPC_URL=https://...
WALLET_MNEMONIC="word1 word2 ..."
```

```astrixa
// Access in code
let key = env("PRIVATE_KEY");
let rpc = env("ETH_RPC_URL");
```

## Error Handling

```astrixa
// Contract validation
require(condition, "Error message");

// Transaction errors
let receipt = web3.wait(tx.hash);
if receipt.status != 1 {
    println("Transaction failed!");
    return;
}

// Try-catch (coming soon)
try {
    let result = risky_operation();
} catch (error) {
    println("Error: " + error);
}
```

## Resources

- 📖 Full Guide: [WEB3_COMPLETE_GUIDE.md](./WEB3_COMPLETE_GUIDE.md)
- 💻 Examples: [examples/](./examples/)
- 📚 API Docs: [stdlib/web3.ax](./stdlib/web3.ax)
- 🌐 Website: https://astrixa.dev
- 💬 Discord: https://discord.gg/astrixa
- 🐙 GitHub: https://github.com/astrixa-lang/astrixa

---

**ASTRIXA - The first language born for AI, Web3, and the decentralized web.**
