# Standard Library: Web3

Native blockchain operations in ASTRIXA - no SDK complexity.

## Quick Start

```astrixa
use std::web3

// Connect to Ethereum
let provider = web3.connect("https://mainnet.infura.io/v3/YOUR-KEY")

// Get balance
let balance = await provider.getBalance(0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb)
print("Balance: " + balance + " wei")
```

## Wallet Management

### Create/Import Wallet

```astrixa
// Generate new wallet
let wallet = web3.wallet.generate()
print("Address: " + wallet.address)
print("Private Key: " + wallet.privateKey)

// From private key
let imported = web3.wallet.fromPrivateKey("0x...")

// From mnemonic
let mnemonic = "word1 word2 word3 ..."
let hdWallet = web3.wallet.fromMnemonic(mnemonic, 0)
```

### Sign Messages

```astrixa
let wallet = web3.wallet.generate()
let message = "Hello, ASTRIXA!"
let signature = wallet.sign(message)

print("Signature: " + signature)
```

## Smart Contract Interaction

### Deploy Contract

```astrixa
contract Token {
    state totalSupply: U256
    state balances: map<Address, U256>
    
    fn constructor(supply: U256) {
        totalSupply = supply
        balances[tx.sender] = supply
    }
    
    fn transfer(to: Address, amount: U256) {
        require(balances[tx.sender] >= amount, "Insufficient balance")
        balances[tx.sender] -= amount
        balances[to] += amount
    }
    
    fn balanceOf(account: Address) -> U256 {
        return balances[account]
    }
}

// Deploy
let provider = web3.connect("http://localhost:8545")
let wallet = web3.wallet.fromPrivateKey(privateKey)
let contract = await provider.deploy(Token, [1000000], wallet)

print("Contract deployed at: " + contract.address)
```

### Call Contract Methods

```astrixa
// Read (view function - no gas)
let balance = await contract.balanceOf(userAddress)
print("Balance: " + balance)

// Write (transaction - costs gas)
let tx = await contract.transfer(recipientAddress, 100, {
    from: wallet,
    gasLimit: 100000
})

print("Transaction hash: " + tx.hash)

// Wait for confirmation
let receipt = await tx.wait()
print("Confirmed in block: " + receipt.blockNumber)
```

## Transactions

### Send ETH

```astrixa
let provider = web3.connect("http://localhost:8545")
let wallet = web3.wallet.fromPrivateKey(privateKey)

let tx = await provider.sendTransaction({
    from: wallet.address,
    to: recipientAddress,
    value: web3.toWei(1, "ether"),  // 1 ETH
    gasLimit: 21000,
    gasPrice: web3.toWei(20, "gwei")
})

print("TX Hash: " + tx.hash)
```

### Get Transaction Details

```astrixa
let txHash = "0x..."
let tx = await provider.getTransaction(txHash)

print("From: " + tx.from)
print("To: " + tx.to)
print("Value: " + tx.value)
print("Gas: " + tx.gasLimit)

let receipt = await provider.getTransactionReceipt(txHash)
print("Status: " + receipt.status)  // 1 = success, 0 = failed
print("Block: " + receipt.blockNumber)
```

## Blockchain Queries

### Get Block Information

```astrixa
// Latest block
let block = await provider.getBlock("latest")
print("Block number: " + block.number)
print("Timestamp: " + block.timestamp)
print("Transactions: " + block.transactions.length)

// Specific block
let oldBlock = await provider.getBlock(15000000)
```

### Get Balance

```astrixa
let address = 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb
let balance = await provider.getBalance(address)

print("Balance: " + web3.fromWei(balance, "ether") + " ETH")
```

### Gas Price

```astrixa
let gasPrice = await provider.getGasPrice()
print("Current gas price: " + web3.fromWei(gasPrice, "gwei") + " gwei")
```

## Events and Logs

### Listen to Contract Events

```astrixa
contract Token {
    event Transfer(from: Address, to: Address, amount: U256)
    
    fn transfer(to: Address, amount: U256) {
        balances[tx.sender] -= amount
        balances[to] += amount
        emit Transfer(tx.sender, to, amount)
    }
}

// Listen to events
contract.on("Transfer", fn(from, to, amount) {
    print("Transfer: " + from + " → " + to + " (" + amount + ")")
})
```

### Query Past Events

```astrixa
// Get all Transfer events
let events = await contract.queryFilter("Transfer", {
    fromBlock: 15000000,
    toBlock: "latest"
})

for event in events {
    print("From: " + event.args.from)
    print("To: " + event.args.to)
    print("Amount: " + event.args.amount)
}
```

## Unit Conversions

```astrixa
// Wei ↔ Ether
let wei = web3.toWei(1, "ether")          // 1000000000000000000
let ether = web3.fromWei(wei, "ether")    // 1

// Wei ↔ Gwei
let gwei = web3.toWei(20, "gwei")         // 20000000000
let gweiValue = web3.fromWei(gwei, "gwei") // 20
```

## Complete Example: Token Transfer

```astrixa
use std::web3

async fn transferTokens() {
    // Connect to network
    let provider = web3.connect("http://localhost:8545")
    
    // Load wallet
    let wallet = web3.wallet.fromPrivateKey(process.env.PRIVATE_KEY)
    
    // Load contract
    let tokenAddress = 0x1234567890123456789012345678901234567890
    let contract = provider.getContract(tokenAddress, TokenABI)
    
    // Check balance
    let balance = await contract.balanceOf(wallet.address)
    print("Your balance: " + balance)
    
    if balance < 100 {
        print("Insufficient balance")
        return
    }
    
    // Transfer tokens
    let recipient = 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb
    let amount = 100
    
    print("Sending " + amount + " tokens to " + recipient)
    
    let tx = await contract.transfer(recipient, amount, {
        from: wallet,
        gasLimit: 100000
    })
    
    print("Transaction sent: " + tx.hash)
    
    // Wait for confirmation
    let receipt = await tx.wait()
    
    if receipt.status == 1 {
        print("✅ Transfer successful!")
    } else {
        print("❌ Transfer failed")
    }
    
    // Check new balance
    let newBalance = await contract.balanceOf(wallet.address)
    print("New balance: " + newBalance)
}

await transferTokens()
```

## Best Practices

### ✅ Do: Always Validate Addresses

```astrixa
fn isValidAddress(addr: string) -> bool {
    return addr.matches("^0x[0-9a-fA-F]{40}$")
}

if !isValidAddress(recipient) {
    panic("Invalid address")
}
```

### ✅ Do: Handle Transaction Failures

```astrixa
try {
    let tx = await contract.transfer(to, amount)
    let receipt = await tx.wait()
    
    if receipt.status != 1 {
        print("Transaction reverted")
    }
} catch error {
    print("Error: " + error.message)
}
```

### ✅ Do: Estimate Gas Before Sending

```astrixa
let gasEstimate = await contract.estimateGas.transfer(to, amount)
print("Estimated gas: " + gasEstimate)

let tx = await contract.transfer(to, amount, {
    gasLimit: gasEstimate * 1.2  // Add 20% buffer
})
```

## Next Steps

- [AI Module →](ai.md)
- [Web Module →](web.md)
- [Smart Contract Examples →](../../examples/smart_contract_token.ax)

---

**Learn more:** See [WEB3_COMPLETE_GUIDE.md](../../WEB3_COMPLETE_GUIDE.md) for advanced Web3 patterns.
