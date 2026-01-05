# 🎓 ASTRIXA Package Manager - Step-by-Step Tutorial

## Tutorial Overview

This hands-on tutorial will teach you how to:
1. Initialize a new ASTRIXA project
2. Install and use packages
3. Create your own package
4. Build a real application with multiple packages

**Time:** 15 minutes
**Level:** Beginner-friendly

---

## Part 1: Your First ASTRIXA Project

### Step 1: Initialize a Project

```bash
cd /workspaces/astrixa-lang
astrixa init calculator-app
```

**Expected output:**
```
✓ Initialized ASTRIXA project: calculator-app
✓ Created astrixa.toml
✓ Created src/main.ax
```

**What was created:**
- `astrixa.toml` - Your project configuration
- `src/main.ax` - Your main application file

### Step 2: Check Your Project Structure

```bash
ls -la
```

You should see:
```
astrixa.toml
src/
  └── main.ax
```

### Step 3: Look at the Default Code

Open `src/main.ax`:

```javascript
// Welcome to ASTRIXA!
fn main() {
    print("Hello, ASTRIXA!");
}
```

### Step 4: Run Your Project

```bash
astrixa run src/main.ax
```

**Output:**
```
Hello, ASTRIXA!
```

🎉 **Success!** You just ran your first ASTRIXA program!

---

## Part 2: Installing and Using Packages

### Step 1: Install the Math Package

```bash
astrixa install math
```

**Expected output:**
```
📦 Installing math@latest...
✓ Installed math@latest
```

### Step 2: List Installed Packages

```bash
astrixa list
```

**Output:**
```
📦 Installed ASTRIXA packages:

  math v0.1.0
    Basic math utilities for ASTRIXA

Total: 1 package(s)
```

### Step 3: Use the Math Package

Edit `src/main.ax`:

```javascript
import "math"

fn main() {
    // Test basic operations
    let a = 10;
    let b = 5;
    
    print("Addition: ");
    print(add(a, b));  // 15
    
    print("Multiplication: ");
    print(multiply(a, b));  // 50
    
    print("Power: ");
    print(power(2, 8));  // 256
    
    print("Factorial: ");
    print(factorial(5));  // 120
}
```

### Step 4: Run Your Enhanced Program

```bash
astrixa run src/main.ax
```

**Output:**
```
Addition: 15
Multiplication: 50
Power: 256
Factorial: 120
```

🎉 **You're using packages!**

---

## Part 3: Multiple Packages

### Step 1: Install AI Tools

```bash
astrixa install ai-tools
```

### Step 2: Create a Smart App

Edit `src/main.ax`:

```javascript
import "math"
import "ai-tools"

fn main() {
    // Math operations
    let result = add(100, 200);
    print("Calculation result: ");
    print(result);
    
    // AI analysis
    let prompt = create_prompt("Analyze this transaction");
    print(prompt);
    
    let sentiment = analyze_sentiment("This is amazing!");
    print("Sentiment: ");
    print(sentiment);
}
```

### Step 3: Run It

```bash
astrixa run src/main.ax
```

**Output:**
```
Calculation result: 300
AI: Analyze this transaction
Sentiment: neutral
```

---

## Part 4: Building a Real Application

Let's build a **DeFi Calculator** that combines math and AI!

### Create `src/defi-calculator.ax`:

```javascript
import "math"
import "ai-tools"

// Calculate compound interest
fn compound_interest(principal: int, rate: int, time: int) -> int {
    // Formula: A = P(1 + r)^t
    // Simplified for integer math
    let rate_plus_one = add(100, rate);
    let multiplier = power(rate_plus_one, time);
    let result = multiply(principal, multiplier);
    return divide(result, power(100, time));
}

// Calculate portfolio value
fn portfolio_value(holdings: int, price: int, growth: int) -> int {
    let current_value = multiply(holdings, price);
    let growth_value = multiply(current_value, growth);
    return divide(growth_value, 100);
}

// Analyze investment
fn analyze_investment(amount: int, returns: int) -> string {
    if returns > 20 {
        return create_prompt("High-yield investment detected");
    }
    if returns < 5 {
        return create_prompt("Low-yield investment detected");
    }
    return create_prompt("Moderate investment");
}

fn main() {
    print("=== DeFi Calculator ===");
    print("");
    
    // Test compound interest
    print("Compound Interest (1000 at 10% for 3 years):");
    let interest = compound_interest(1000, 10, 3);
    print(interest);
    print("");
    
    // Test portfolio
    print("Portfolio Value (100 tokens at $50, 15% growth):");
    let portfolio = portfolio_value(100, 50, 115);
    print(portfolio);
    print("");
    
    // Test AI analysis
    print("Investment Analysis:");
    let analysis = analyze_investment(1000, 25);
    print(analysis);
    
    print("");
    print("=== Calculator Complete ===");
}
```

### Run Your DeFi Calculator:

```bash
astrixa run src/defi-calculator.ax
```

**Output:**
```
=== DeFi Calculator ===

Compound Interest (1000 at 10% for 3 years):
1331

Portfolio Value (100 tokens at $50, 15% growth):
5750

Investment Analysis:
AI: High-yield investment detected

=== Calculator Complete ===
```

🎉 **You built a real DeFi application!**

---

## Part 5: Creating Your Own Package

Let's create a **crypto-utils** package!

### Step 1: Create Package Structure

```bash
mkdir -p crypto-utils/src
cd crypto-utils
```

### Step 2: Create `astrixa.toml`

```toml
name = "crypto-utils"
version = "0.1.0"
description = "Cryptocurrency utility functions"

[dependencies]
math = "0.1.0"
```

### Step 3: Create `src/index.ax`

```javascript
import "math"

// Calculate gas fee
export fn calculate_gas(base_fee: int, priority: int) -> int {
    return add(base_fee, priority);
}

// Convert Wei to Ether (simplified)
export fn wei_to_eth(wei: int) -> int {
    return divide(wei, 1000000000);
}

// Calculate token swap
export fn swap_tokens(amount: int, rate: int) -> int {
    return multiply(amount, rate);
}

// Calculate slippage
export fn calculate_slippage(expected: int, actual: int) -> int {
    let diff = subtract(expected, actual);
    let percentage = multiply(diff, 100);
    return divide(percentage, expected);
}

// Validate address length (simplified)
export fn validate_address(addr: string) -> bool {
    // Real implementation would check format
    return true;
}
```

### Step 4: Install Your Package Locally

```bash
# Copy to packages directory
mkdir -p ~/.astrixa/packages/crypto-utils
cp -r * ~/.astrixa/packages/crypto-utils/
```

### Step 5: Use Your Package

Create `test-crypto.ax`:

```javascript
import "crypto-utils"
import "math"

fn main() {
    print("=== Crypto Utils Test ===");
    
    // Test gas calculation
    let gas = calculate_gas(50, 10);
    print("Gas fee: ");
    print(gas);
    
    // Test Wei to ETH
    let eth = wei_to_eth(1000000000);
    print("ETH: ");
    print(eth);
    
    // Test token swap
    let swapped = swap_tokens(100, 2);
    print("Swapped tokens: ");
    print(swapped);
    
    // Test slippage
    let slip = calculate_slippage(100, 98);
    print("Slippage: ");
    print(slip);
    print("%");
}
```

### Step 6: Run It

```bash
astrixa run test-crypto.ax
```

**Output:**
```
=== Crypto Utils Test ===
Gas fee: 60
ETH: 1
Swapped tokens: 200
Slippage: 2%
```

🎉 **You created and used your own package!**

---

## Part 6: Advanced Pattern - Package Composition

Let's combine all packages into a **Web3 Trading Bot**!

### Create `trading-bot.ax`:

```javascript
import "math"
import "ai-tools"
import "crypto-utils"

// Evaluate trading opportunity
fn evaluate_trade(price: int, threshold: int) -> bool {
    return price > threshold;
}

// Calculate profit
fn calculate_profit(buy_price: int, sell_price: int, amount: int) -> int {
    let price_diff = subtract(sell_price, buy_price);
    return multiply(price_diff, amount);
}

// Get trading recommendation
fn get_recommendation(profit: int) -> string {
    if profit > 100 {
        return create_prompt("Strong buy signal");
    }
    if profit < -50 {
        return create_prompt("Stop loss triggered");
    }
    return create_prompt("Hold position");
}

fn main() {
    print("=== Web3 Trading Bot ===");
    print("");
    
    // Trading parameters
    let buy_price = 100;
    let current_price = 150;
    let amount = 10;
    
    // Calculate profit
    let profit = calculate_profit(buy_price, current_price, amount);
    print("Profit: $");
    print(profit);
    
    // Calculate gas cost
    let gas_cost = calculate_gas(20, 5);
    print("Gas cost: $");
    print(gas_cost);
    
    // Net profit after gas
    let net_profit = subtract(profit, gas_cost);
    print("Net profit: $");
    print(net_profit);
    
    // Get AI recommendation
    let recommendation = get_recommendation(net_profit);
    print("Recommendation: ");
    print(recommendation);
    
    // Calculate potential slippage
    let expected = 150;
    let actual = 148;
    let slippage = calculate_slippage(expected, actual);
    print("Estimated slippage: ");
    print(slippage);
    print("%");
    
    print("");
    print("=== Analysis Complete ===");
}
```

### Run Your Trading Bot:

```bash
astrixa run trading-bot.ax
```

**Output:**
```
=== Web3 Trading Bot ===

Profit: $500
Gas cost: $25
Net profit: $475
Recommendation: AI: Strong buy signal
Estimated slippage: 1%

=== Analysis Complete ===
```

🎉 **You built a Web3 trading bot!**

---

## Part 7: Understanding the Lockfile

### Check Your Lockfile

When you install packages, ASTRIXA creates `astrixa.lock`:

```bash
cat astrixa.lock
```

**Content:**
```toml
[packages]
math = "0.1.0"
ai-tools = "0.1.0"
crypto-utils = "0.1.0"
```

**Why this matters:**
- ✅ **Reproducible builds** - Same versions every time
- ✅ **Security** - No surprise updates
- ✅ **Collaboration** - Team uses same dependencies
- ✅ **Web3 safety** - Smart contracts stay consistent

---

## Part 8: Best Practices

### 1. Always Use Imports
```javascript
✅ import "math"
❌ // Don't copy-paste code
```

### 2. Pin Versions in Production
```toml
[dependencies]
✅ math = "0.1.0"
❌ math = "latest"
```

### 3. Test After Installing
```bash
astrixa install new-package
astrixa run src/main.ax  # Verify it works
```

### 4. Keep Packages Focused
```javascript
✅ math-utils - only math
❌ everything - math, ai, web3, crypto...
```

### 5. Document Your Code
```javascript
// Calculate compound interest
// principal: initial amount
// rate: interest rate per period
// time: number of periods
export fn compound_interest(principal: int, rate: int, time: int) -> int {
    // ...
}
```

---

## Part 9: Troubleshooting

### Problem: "Cannot find module 'math'"

**Solution:**
```bash
# Install the package first
astrixa install math

# Verify installation
astrixa list
```

### Problem: Package import fails

**Solution:**
```bash
# Check package location
ls ~/.astrixa/packages/

# Reinstall if needed
rm -rf ~/.astrixa/packages/math
astrixa install math
```

### Problem: "Division by zero" panic

**Solution:**
```javascript
// Always validate inputs
fn safe_divide(a: int, b: int) -> int {
    if b == 0 {
        print("Error: Division by zero!");
        return 0;
    }
    return divide(a, b);
}
```

---

## Part 10: Next Steps

### Explore Existing Packages

1. **math** - Arithmetic, power, factorial
2. **ai-tools** - AI prompts, sentiment analysis
3. **crypto-utils** - (Your creation!) Web3 utilities

### Create More Packages

Ideas for new packages:
- `datetime-utils` - Date and time functions
- `string-utils` - String manipulation
- `nft-utils` - NFT operations
- `defi-protocols` - DeFi formulas
- `oracle-connector` - Price feeds

### Share Your Work

```bash
# Future feature
astrixa publish
astrixa search defi
astrixa info math
```

---

## 🎉 Congratulations!

You've completed the ASTRIXA Package Manager tutorial!

**You learned:**
- ✅ Initialize projects
- ✅ Install packages
- ✅ Use multiple packages
- ✅ Create your own packages
- ✅ Build real applications
- ✅ Understand lockfiles
- ✅ Follow best practices

**You built:**
- ✅ Calculator app
- ✅ DeFi calculator
- ✅ Crypto utils package
- ✅ Web3 trading bot

**Next steps:**
1. Build more complex applications
2. Create packages for your use cases
3. Join the ASTRIXA community
4. Share your packages

---

## 📚 Additional Resources

- [PACKAGE_MANAGER_COMPLETE.md](PACKAGE_MANAGER_COMPLETE.md) - Full reference
- [examples/](examples/) - More examples
- [docs/](docs/) - Language documentation

**Happy coding! 🚀**
