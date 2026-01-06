# ASTRIXA WEB3 COMPLETE GUIDE

## Overview

ASTRIXA is the **FIRST** programming language with **native Web3 support**. No frameworks, no libraries - Web3 is built into the language itself.

### Key Features

1. **Native Blockchain Context** - `msg`, `tx`, `chain` available everywhere
2. **Smart Contract Mode** - Compile directly to EVM bytecode
3. **AI + Web3** - Deterministic AI operations on-chain
4. **Multi-Chain Support** - Ethereum, Polygon, Arbitrum, Optimism (more coming)
5. **Type-Safe Web3** - No RPC errors, no ABI parsing hell

---

## Quick Start

### 1. Connect to Blockchain

```astrixa
import std::web3;

// Connect to Ethereum
let provider = web3.connect("https://eth-mainnet.alchemyapi.io/v2/YOUR_KEY");

// Or use environment variable
let provider = web3.connect(env("ETH_RPC_URL"));

// Multi-chain support
let eth = web3.connect("mainnet");
let polygon = web3.connect("polygon");
let arbitrum = web3.connect("arbitrum");
```

### 2. Read from Blockchain

```astrixa
// Get ETH balance
let balance = web3.balance("vitalik.eth");
println("Vitalik has: " + web3.fromWei(balance, "ether") + " ETH");

// Get block number
let block_num = web3.blockNumber();
println("Current block: " + block_num);

// Get gas price
let gas = web3.gasPrice();
println("Gas price: " + web3.fromWei(gas, "gwei") + " gwei");
```

### 3. Send Transactions

```astrixa
// Create wallet from private key
let wallet = web3.wallet.fromPrivateKey(env("PRIVATE_KEY"));

// Send ETH
let tx = web3.send({
    from: wallet.address,
    to: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    value: web3.toWei(0.1, "ether"),
    gasLimit: 21000
});

// Wait for confirmation
let receipt = web3.wait(tx.hash);
if receipt.status == 1 {
    println("✅ Transaction successful!");
} else {
    println("❌ Transaction failed");
}
```

### 4. Smart Contract Interaction

```astrixa
// Load contract ABI
let usdc = web3.contract({
    address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    abi: import("./usdc_abi.json")
});

// Read contract state (no gas)
let balance = usdc.call("balanceOf", [wallet.address]);
println("USDC balance: " + balance / 1e6);

let symbol = usdc.call("symbol", []);
println("Token: " + symbol);

// Write to contract (requires gas)
let tx = usdc.send("transfer", [
    "0x...",  // recipient
    1000000   // 1 USDC (6 decimals)
], {
    from: wallet.address,
    gasLimit: 100000
});

let receipt = web3.wait(tx.hash);
```

---

## Smart Contracts in ASTRIXA

### Writing Contracts

```astrixa
// token.ax
contract ERC20Token {
    state: [
        "name",
        "symbol",
        "decimals",
        "total_supply",
        "balances",
        "allowances"
    ]
    
    fn initialize(name: string, symbol: string, supply: U256) {
        state["name"] = name;
        state["symbol"] = symbol;
        state["decimals"] = 18;
        state["total_supply"] = supply;
        state["balances"][msg.sender] = supply;
        
        emit("Transfer", {
            from: "0x0",
            to: msg.sender,
            value: supply
        });
    }
    
    fn balance_of(account: Address) -> U256 {
        return state["balances"][account] or 0;
    }
    
    fn transfer(to: Address, amount: U256) -> bool {
        require(to != "0x0", "Invalid address");
        require(amount > 0, "Invalid amount");
        
        let sender_balance = state["balances"][msg.sender] or 0;
        require(sender_balance >= amount, "Insufficient balance");
        
        state["balances"][msg.sender] = sender_balance - amount;
        
        let recipient_balance = state["balances"][to] or 0;
        state["balances"][to] = recipient_balance + amount;
        
        emit("Transfer", {
            from: msg.sender,
            to: to,
            value: amount
        });
        
        return true;
    }
}
```

### Compiling Contracts

```bash
# Compile to EVM bytecode
astrixa build token.ax --target=contract --output=token.bin

# Deploy to testnet
astrixa deploy token.ax --network=sepolia --wallet=wallet.json

# Verify on Etherscan
astrixa verify token.ax --address=0x... --network=mainnet
```

### Contract Restrictions

Smart contracts have restrictions to ensure determinism:

❌ **NOT ALLOWED:**
- File system access (`read`, `write`)
- Network requests (`http_get`, `http_post`)
- Random numbers (`random`, `rand`)
- Dynamic memory allocation
- Floating point (use U256 fixed-point)
- Unbounded loops

✅ **ALLOWED:**
- Pure deterministic logic
- State variables
- `msg.sender`, `tx.timestamp`, `chain.id`
- `require()` for validation
- `emit()` for events
- AI operations (deterministic)
- Crypto operations
- Math operations
- U256 arithmetic

---

## Blockchain Context

ASTRIXA provides native blockchain context everywhere:

### Message Context (`msg`)

```astrixa
// Current transaction sender
let sender = msg.sender;  // Address

// Amount of ETH sent
let value = msg.value;    // U256 (wei)

// Transaction data
let data = msg.data;      // Bytes
```

### Transaction Context (`tx`)

```astrixa
// Gas price
let gas_price = tx.gasprice;  // U256

// Transaction origin
let origin = tx.origin;       // Address

// Block timestamp
let time = tx.timestamp;      // U256
```

### Chain Context (`chain`)

```astrixa
// Chain ID (1 = Ethereum, 137 = Polygon)
let id = chain.id;

// Network name
let network = chain.network;  // "mainnet", "sepolia", etc.
```

---

## Advanced Features

### 1. ENS Support

```astrixa
// Resolve ENS name to address
let addr = web3.ens.resolve("vitalik.eth");

// Reverse lookup
let name = web3.ens.reverse("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");

// Get ENS record
let email = web3.ens.text("vitalik.eth", "email");
let avatar = web3.ens.text("vitalik.eth", "avatar");
```

### 2. Event Listening

```astrixa
// Listen for Transfer events
let subscription = usdc.on("Transfer", fn(event) {
    println("Transfer detected:");
    println("  From: " + event.from);
    println("  To: " + event.to);
    println("  Amount: " + event.value);
});

// Listen for new blocks
web3.onBlock(fn(block) {
    println("New block: " + block.number);
    println("Transactions: " + len(block.transactions));
});

// Listen for pending transactions
web3.onTransaction(fn(tx) {
    if tx.to == wallet.address {
        println("Incoming transaction: " + tx.hash);
    }
});

// Clean up
subscription.unsubscribe();
```

### 3. Multi-Call (Batch Requests)

```astrixa
// Execute multiple reads in one RPC call
let results = web3.multicall([
    usdc.call("balanceOf", [addr1]),
    usdc.call("balanceOf", [addr2]),
    usdc.call("balanceOf", [addr3]),
    usdc.call("totalSupply", [])
]);

let balance1 = results[0];
let balance2 = results[1];
let balance3 = results[2];
let total_supply = results[3];
```

### 4. Transaction Building

```astrixa
// Build complex transaction
let tx = web3.transaction()
    .from(wallet.address)
    .to("0x...")
    .value(web3.toWei(1, "ether"))
    .gas(100000)
    .gasPrice(web3.toWei(50, "gwei"))
    .data("0x...")
    .nonce(web3.getTransactionCount(wallet.address))
    .build();

// Sign transaction
let signed = wallet.sign(tx);

// Send raw transaction
let hash = web3.sendRawTransaction(signed);
```

### 5. Gas Estimation

```astrixa
// Estimate gas for transaction
let estimated = web3.estimateGas({
    from: wallet.address,
    to: usdc.address,
    data: usdc.encode("transfer", ["0x...", 1000000])
});

println("Estimated gas: " + estimated);

// Add 20% buffer
let safe_gas = estimated * 120 / 100;
```

---

## Web3 + AI: Unique to ASTRIXA

ASTRIXA is the ONLY language with **native AI + Web3 integration**:

### 1. AI-Powered Trading Bot

```astrixa
import std::web3;
import std::ai;

let provider = web3.connect("mainnet");
let wallet = web3.wallet.fromPrivateKey(env("PRIVATE_KEY"));

// Load DEX contract
let uniswap = web3.contract({
    address: "0x...",
    abi: import("./uniswap_abi.json")
});

// Monitor price and use AI to decide when to trade
web3.onBlock(fn(block) {
    // Get current price
    let price = uniswap.call("getPrice", ["WETH", "USDC"]);
    
    // Get historical prices
    let history = get_price_history();
    
    // Use AI to predict if we should buy/sell
    let prediction = ai.infer(
        ai.model("trading_model"),
        { price: price, history: history, block: block.number }
    );
    
    if prediction.action == "buy" && prediction.confidence > 0.8 {
        // Execute swap
        let tx = uniswap.send("swap", [
            "USDC",
            "WETH",
            web3.toWei(100, "mwei"),  // 100 USDC
            0  // min output (use proper slippage in prod)
        ], {
            from: wallet.address,
            gasLimit: 300000
        });
        
        println("🤖 AI triggered buy: " + tx.hash);
    }
});
```

### 2. Content Moderation DAO

```astrixa
contract ContentDAO {
    fn submit_content(content: string) -> U256 {
        // Use AI for initial moderation
        let sentiment = ai.infer(ai.model("sentiment"), content);
        
        let content_id = generate_id();
        
        state["submissions"][content_id] = {
            content: content,
            submitter: msg.sender,
            sentiment: sentiment,
            approved: false,
            timestamp: tx.timestamp
        };
        
        // Auto-approve if safe
        if sentiment.label == "positive" && sentiment.score > 0.8 {
            state["submissions"][content_id]["approved"] = true;
            emit("ContentApproved", { id: content_id, auto: true });
        } else {
            emit("ContentPendingReview", { id: content_id });
        }
        
        return content_id;
    }
}
```

### 3. NFT Metadata Generator

```astrixa
contract AIGeneratedNFT {
    fn mint(prompt: string) -> U256 {
        // Generate image with AI
        let image = ai.generate(ai.model("stable-diffusion"), prompt);
        
        // Generate description with AI
        let description = ai.infer(
            ai.model("gpt-4"),
            "Create an NFT description for: " + prompt
        );
        
        // Store metadata on-chain
        let token_id = generate_token_id();
        state["metadata"][token_id] = {
            name: "AI Generated #" + token_id,
            description: description,
            image: image,
            prompt: prompt,
            creator: msg.sender,
            timestamp: tx.timestamp
        };
        
        // Mint NFT
        state["owners"][token_id] = msg.sender;
        
        emit("Transfer", {
            from: "0x0",
            to: msg.sender,
            tokenId: token_id
        });
        
        return token_id;
    }
}
```

---

## Multi-Chain Support

ASTRIXA makes multi-chain development EASY:

```astrixa
// Connect to multiple chains
let eth = web3.connect("mainnet");
let polygon = web3.connect("polygon");
let arbitrum = web3.connect("arbitrum");

// Same contract on different chains
let usdc_eth = eth.contract({
    address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    abi: usdc_abi
});

let usdc_polygon = polygon.contract({
    address: "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
    abi: usdc_abi
});

// Get balances on both chains
let eth_balance = usdc_eth.call("balanceOf", [wallet.address]);
let polygon_balance = usdc_polygon.call("balanceOf", [wallet.address]);

println("ETH: " + eth_balance / 1e6 + " USDC");
println("Polygon: " + polygon_balance / 1e6 + " USDC");

// Bridge tokens (pseudo-code)
fn bridge_to_polygon(amount: U256) {
    // Approve on Ethereum
    let approve_tx = usdc_eth.send("approve", [bridge_address, amount], {
        from: wallet.address,
        gasLimit: 100000
    });
    eth.wait(approve_tx.hash);
    
    // Deposit to bridge
    let deposit_tx = bridge_eth.send("deposit", [amount], {
        from: wallet.address,
        gasLimit: 200000
    });
    
    println("Bridging in progress...");
    
    // Wait for L2 confirmation (simplified)
    sleep(60);  // In production, listen for events
    
    println("✅ Tokens bridged to Polygon!");
}
```

---

## Security Best Practices

### 1. Always Use `require()`

```astrixa
fn transfer(to: Address, amount: U256) -> bool {
    // ✅ GOOD: Validate inputs
    require(to != "0x0", "Invalid address");
    require(amount > 0, "Amount must be positive");
    require(balance_of(msg.sender) >= amount, "Insufficient balance");
    
    // ... rest of function
}

// ❌ BAD: No validation
fn transfer_bad(to: Address, amount: U256) -> bool {
    state["balances"][msg.sender] -= amount;
    state["balances"][to] += amount;
    return true;  // Can underflow, can send to 0x0, etc.
}
```

### 2. Check Return Values

```astrixa
// ✅ GOOD: Check if transaction succeeded
let tx = token.send("transfer", [to, amount], opts);
let receipt = web3.wait(tx.hash);
if receipt.status != 1 {
    println("❌ Transaction failed!");
    return;
}

// ❌ BAD: Ignore result
token.send("transfer", [to, amount], opts);
// Did it work? Who knows!
```

### 3. Use Events for Important State Changes

```astrixa
fn transfer(to: Address, amount: U256) -> bool {
    // ... logic ...
    
    // ✅ GOOD: Emit event
    emit("Transfer", {
        from: msg.sender,
        to: to,
        value: amount,
        timestamp: tx.timestamp
    });
    
    return true;
}
```

### 4. Reentrancy Protection

```astrixa
contract Vault {
    state: ["balances", "locked"]
    
    fn withdraw(amount: U256) {
        // ✅ GOOD: Checks-Effects-Interactions pattern
        require(!state["locked"], "Reentrancy detected");
        state["locked"] = true;
        
        let balance = state["balances"][msg.sender];
        require(balance >= amount, "Insufficient balance");
        
        // Update state BEFORE external call
        state["balances"][msg.sender] = balance - amount;
        
        // External call
        (bool success, ) = msg.sender.call{value: amount}("");
        require(success, "Transfer failed");
        
        state["locked"] = false;
    }
}
```

---

## Deployment & Testing

### Deploy Contract

```bash
# Deploy to testnet
astrixa deploy token.ax \
    --network=sepolia \
    --wallet=wallet.json \
    --constructor="MyToken,MTK,1000000"

# Deploy to mainnet (use carefully!)
astrixa deploy token.ax \
    --network=mainnet \
    --wallet=wallet.json \
    --gas-price=30 \
    --verify

# Output:
# Deploying to Sepolia...
# Transaction: 0x...
# Contract deployed at: 0x...
# ✅ Verified on Etherscan
```

### Test Contract

```astrixa
// test_token.ax
import std::test;
import std::web3;

test "should transfer tokens" {
    // Deploy contract
    let token = deploy_contract("token.ax", ["TestToken", "TST", 1000000]);
    
    // Create test accounts
    let alice = test.account("alice");
    let bob = test.account("bob");
    
    // Test transfer
    test.as(alice, fn() {
        let result = token.call("transfer", [bob.address, 100]);
        assert(result == true, "Transfer should succeed");
        
        let balance = token.call("balance_of", [bob.address]);
        assert(balance == 100, "Bob should have 100 tokens");
    });
}

test "should reject invalid transfer" {
    let token = deploy_contract("token.ax", ["TestToken", "TST", 1000]);
    
    test.expect_revert(fn() {
        token.send("transfer", ["0x0", 100], { from: test.account("alice") });
    }, "Invalid address");
}
```

Run tests:

```bash
astrixa test test_token.ax
```

---

## Why ASTRIXA for Web3?

### 1. Native Support = Better DX

**Python/JavaScript:**
```python
# Need Web3.py or ethers.js
from web3 import Web3
w3 = Web3(Web3.HTTPProvider('http://localhost:8545'))
balance = w3.eth.get_balance('0x...')
```

**ASTRIXA:**
```astrixa
// Built into language
let balance = web3.balance("0x...");
```

### 2. Type Safety

**JavaScript:**
```javascript
// Runtime error - typo in method name
const balance = await contract.balaneOf(address);  // ❌
```

**ASTRIXA:**
```astrixa
// Compile-time error
let balance = contract.call("balaneOf", [address]);  // ❌ Compiler catches this
```

### 3. One Language for Everything

**Traditional:**
- Solidity for contracts
- JavaScript for backend
- TypeScript for frontend
- Python for AI/scripts

**ASTRIXA:**
- ASTRIXA for contracts
- ASTRIXA for backend
- ASTRIXA (WASM) for frontend
- ASTRIXA for AI/scripts

### 4. AI + Web3 Integration

No other language has native AI + Web3 support. ASTRIXA does.

---

## Roadmap

### Current (v0.1)
- ✅ Ethereum/EVM support
- ✅ Smart contract compilation
- ✅ Wallet management
- ✅ Transaction signing
- ✅ Contract interaction
- ✅ ENS support
- ✅ Event listening

### Coming Soon (v0.2)
- 🔄 Solana support
- 🔄 Aptos/Move support
- 🔄 Cosmos SDK support
- 🔄 Layer 2 optimizations
- 🔄 Account abstraction (EIP-4337)

### Future (v1.0)
- Cross-chain messaging
- Built-in indexer
- Graph queries
- IPFS integration
- Decentralized storage

---

## API Reference

See [stdlib/web3.ax](../stdlib/web3.ax) for complete API documentation.

### Core Functions

- `web3.connect(rpc_url)` - Connect to blockchain
- `web3.balance(address)` - Get ETH balance
- `web3.send(tx)` - Send transaction
- `web3.wait(hash)` - Wait for confirmation
- `web3.contract(opts)` - Load contract
- `web3.wallet.fromPrivateKey(key)` - Create wallet
- `web3.toWei(amount, unit)` - Convert to wei
- `web3.fromWei(amount, unit)` - Convert from wei

### Smart Contract Functions

- `require(condition, message)` - Assert condition
- `emit(event, data)` - Emit event
- `msg.sender` - Transaction sender
- `msg.value` - ETH sent
- `tx.timestamp` - Block timestamp
- `chain.id` - Chain ID

---

## Community & Support

- **Documentation:** https://astrixa.dev/docs/web3
- **Discord:** https://discord.gg/astrixa
- **GitHub:** https://github.com/astrixa-lang/astrixa
- **Twitter:** @astrixalang

---

**ASTRIXA: The first language born for AI, Web3, and the decentralized web.**
