# Security Policy

## Reporting a Vulnerability

**IMPORTANT: Do NOT create public GitHub issues for security vulnerabilities.**

The ASTRIXA team takes security seriously. We appreciate your efforts to responsibly disclose your findings.

---

## How to Report

### GitHub Security Advisories (Recommended)

**Report via:** [GitHub Security Advisories](https://github.com/Podamekalajagadeesh/astrixa-lang/security/advisories/new)

This is the preferred method as it provides:
- Private discussion with maintainers
- Coordinated disclosure timeline
- Automatic CVE assignment
- Credit in security advisories

**Include:**
1. **Description** - What is the vulnerability?
2. **Impact** - What can an attacker do?
3. **Reproduction** - Step-by-step instructions
4. **Proof of Concept** - Code sample if applicable
5. **Suggested Fix** - If you have ideas
6. **Disclosure Timeline** - Your expectations

---

## What to Expect

### Response Timeline

1. **Acknowledgment:** Within 48 hours
2. **Initial Assessment:** Within 1 week
3. **Status Update:** Every week until resolved
4. **Fix Timeline:** 
   - Critical: 7-14 days
   - High: 30 days
   - Medium: 60 days
   - Low: 90 days

### Our Process

1. **Triage** - Assess severity and impact
2. **Fix** - Develop and test patch
3. **Coordinate** - Discuss disclosure timeline with you
4. **Release** - Deploy fix to all affected versions
5. **Disclose** - Publish security advisory
6. **Credit** - Acknowledge reporter (if desired)

---

## Scope

### In Scope

**ASTRIXA Core:**
- Compiler (Rust)
- Standard library (ASTRIXA)
- LSP server (Rust)
- VS Code extension (TypeScript)
- Package manager
- Build tools

**Smart Contracts:**
- Contract compilation
- EVM bytecode generation
- Gas metering
- Security restrictions

**Web3:**
- Wallet management
- Transaction signing
- RPC communication

### Out of Scope

**NOT covered:**
- Third-party packages
- User applications
- Deployment infrastructure
- Documentation website
- Social media accounts

---

## Vulnerability Types

### Critical Severity

**Examples:**
- Remote code execution (RCE)
- Arbitrary code execution in compiler
- Private key leakage
- Contract funds theft
- Consensus failure

**Response:** Fix within 7-14 days

### High Severity

**Examples:**
- Denial of service (DoS)
- Authentication bypass
- Privilege escalation
- Memory corruption
- Type system escape

**Response:** Fix within 30 days

### Medium Severity

**Examples:**
- Information disclosure
- Gas estimation errors
- LSP crashes
- Predictable randomness

**Response:** Fix within 60 days

### Low Severity

**Examples:**
- Minor information leaks
- Non-exploitable bugs
- UI issues
- Documentation errors

**Response:** Fix within 90 days

---

## Security Best Practices

### For ASTRIXA Developers

**When writing smart contracts:**

✅ **DO:**
```astrixa
// Validate inputs
fn transfer(to: Address, amount: U256) {
    require(to != "0x0", "Invalid address");
    require(amount > 0, "Invalid amount");
    require(balance >= amount, "Insufficient balance");
}

// Check return values
let receipt = web3.wait(tx.hash);
if receipt.status != 1 {
    panic("Transaction failed");
}

// Use require() for critical checks
require(msg.sender == owner, "Not authorized");

// Emit events for state changes
emit("Transfer", { from: sender, to: to, amount: amount });
```

❌ **DON'T:**
```astrixa
// Skip validation
fn bad_transfer(to: Address, amount: U256) {
    state["balance"][to] += amount;  // No checks!
}

// Ignore errors
contract.send("transfer", [to, amount]);  // Did it work?

// Use forbidden operations
let rand = random();  // Non-deterministic in contracts
let data = read("file.txt");  // External state
```

### For Users

**Protect your private keys:**
- Never share private keys
- Use environment variables
- Use hardware wallets
- Enable 2FA where possible

**Verify contracts:**
- Check source code on Etherscan
- Audit critical contracts
- Use battle-tested libraries

---

## Known Vulnerabilities

### Current Issues

**None publicly disclosed at this time.**

### Past Issues

**CVE-XXXX-XXXXX (if any will be listed here)**

---

## Security Audits

### Status

- **v0.1-0.3:** No formal audit (pre-release)
- **v0.4:** Internal security review (planned Q2 2025)
- **v1.0:** Full security audit (planned Q1 2026)

### Audit Reports

Published audits will be linked here as they become available.
- Current version (v0.1.0) has not been formally audited

---

## Bug Bounty Program

### Status

**Coming in v1.0 (Q1 2026)**

### Rewards (Planned)

| Severity | Reward |
|----------|--------|
| Critical | $5,000 - $10,000 |
| High | $2,000 - $5,000 |
| Medium | $500 - $2,000 |
| Low | $100 - $500 |

### Eligibility

**Requirements:**
- First to report vulnerability
- Clear reproduction steps
- Reasonable disclosure timeline
- No public disclosure before fix
- Responsible testing (no real funds)

**Disqualifications:**
- Already known issues
- Theoretical attacks without PoC
- DoS on third-party services
- Social engineering
- Physical attacks

---

## Disclosure Policy

### Coordinated Disclosure

**Our commitment:**
- Work with reporters to understand impact
- Provide regular updates on fix progress
- Credit reporters in advisory (if desired)
- Aim for 30-90 day disclosure timeline

**Typical timeline:**
1. **Day 0:** Vulnerability reported
2. **Day 1-7:** Initial assessment and fix development
3. **Day 7-30:** Fix testing and verification
4. **Day 30-60:** Coordinate disclosure date with reporter
5. **Day 60-90:** Public disclosure and security advisory

### Security Advisory

**When published, advisory includes:**
- CVE identifier
- Affected versions
- Impact description
- Mitigation steps
- Fixed version
- Credit to reporter(s)

**Published on:**
- GitHub Security Advisories
- ASTRIXA website
- Discord announcement
- Twitter
- Email to mailing list

---

## Security Features

### Compiler

**Built-in protections:**
- Memory safety (Rust)
- Type safety checks
- Contract restrictions enforced
- Gas limit validation
- Integer overflow checks

### Smart Contracts

**Security enforced:**
- No file I/O
- No network access
- No random numbers
- No floating point
- Bounded loops
- Deterministic execution

See [RFC 0003](rfcs/0003-smart-contract-subset.md) for details.

---

## Threat Model

### Assumptions

**We assume:**
- Attacker has access to source code
- Attacker can submit malicious programs
- Network may be hostile
- Users may make mistakes

**We do NOT assume:**
- Attacker has access to private keys
- Attacker controls infrastructure
- All users are adversarial

### Attack Vectors

**Compiler:**
- Malicious ASTRIXA code
- Supply chain attacks
- Malicious dependencies

**Smart Contracts:**
- Reentrancy attacks
- Integer overflow
- Access control bypass
- Gas griefing

**Web3:**
- Private key theft
- Transaction manipulation
- RPC endpoint attacks

---

## Security Updates

### Distribution

**How you'll be notified:**
1. GitHub Security Advisories (subscribe)
2. GitHub Releases page
3. GitHub Discussions

### Updating

**Critical updates:**
```bash
# Update ASTRIXA immediately
curl -fsSL https://astrixa.dev/install.sh | sh

# Or with package manager
cargo install astrixa-cli --force
```

**Verify checksum:**
```bash
sha256sum astrixa-binary
# Compare with published hash
```

---

## Contact

### Security Team

**GitHub Security:** [Report via Security Advisories](https://github.com/Podamekalajagadeesh/astrixa-lang/security/advisories/new)  
**Response time:** Within 48 hours

### Questions

**General security questions:**
- GitHub Discussions: Security category
- GitHub Issues: For non-sensitive questions

**Report vulnerabilities:**
- GitHub Security Advisories (private, recommended)
- NOT: Public GitHub issues

---

## Legal

### Safe Harbor

**We commit to:**
- Not pursue legal action for good faith security research
- Work with you to understand and fix issues
- Credit you publicly (if desired)

**Requirements:**
- Don't access/modify user data
- Don't harm service availability
- Don't use for illegal purposes
- Follow responsible disclosure

### Liability

ASTRIXA is provided "as is" under MIT license. See [LICENSE](LICENSE) for details.

---

## References

- CVE Database: https://cve.mitre.org/
- Ethereum Smart Contract Security: https://consensys.github.io/smart-contract-best-practices/
- OWASP: https://owasp.org/

---

**Thank you for helping keep ASTRIXA and its users safe! 🔒**

---

**Last Updated:** January 2025  
**Version:** 1.0
