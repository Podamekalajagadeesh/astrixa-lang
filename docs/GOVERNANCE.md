# ASTRIXA GOVERNANCE

**Language stability > new features**

## Core Principles

1. **Stability First** - No breaking changes without compelling reason
2. **Community-Driven** - Users shape the language through RFCs
3. **Transparent Process** - All decisions documented and public
4. **Predictable Evolution** - Clear roadmap, no surprises
5. **Abuse-Resistant** - Protection against hostile takeovers

---

## Governance Structure

### Phase 1: Benevolent Dictator (Current - 2024-2025)

**BDFL:** Core Team Lead

**Responsibilities:**
- Final decisions on language design
- RFC approval/rejection
- Release management
- Security response
- Community leadership

**Why BDFL First:**
- Fast iteration in early stages
- Clear vision and direction
- Consistent design philosophy
- Quick decision-making

**Transition Trigger:** When ASTRIXA reaches 1,000+ active developers

---

### Phase 2: Core Team Governance (2025-2026)

**Core Team:** 5-7 trusted developers

**Selection Criteria:**
- Significant contributions to ASTRIXA
- Deep understanding of language design
- Community respect and trust
- Available time commitment
- Diverse perspectives

**Decision Process:**
1. RFC submitted
2. Core team review (2 weeks)
3. Public comment period (2 weeks)
4. Core team vote (majority wins)
5. BDFL veto power (rarely used)

**Core Team Roles:**
- Language Design Lead
- Compiler Lead
- Standard Library Lead
- Tooling Lead
- Documentation Lead
- Community Lead
- Security Lead

---

### Phase 3: Foundation Governance (2026+)

**ASTRIXA Foundation**

**Structure:**
- Non-profit organization
- Neutral ownership of IP
- Board of directors (7-9 members)
- Technical steering committee

**Responsibilities:**
- Trademark protection
- Legal defense
- Funding management
- Conference organization
- Marketing and outreach

**Optional:** DAO for certain decisions (package registry, grants, etc.)

---

## Decision-Making Process

### Language Changes (Requires RFC)

**Must go through RFC:**
- Syntax changes
- New keywords
- Behavioral changes
- Standard library additions
- Breaking changes
- Deprecations

**Does NOT require RFC:**
- Bug fixes
- Documentation improvements
- Performance optimizations (non-breaking)
- Tooling improvements
- Website updates

### RFC Lifecycle

```
Draft → Review → Comment → Decision → Implementation
```

**Timeline:**
- Review: 2 weeks (Core Team)
- Comment: 2 weeks (Community)
- Decision: 1 week (Vote)
- Implementation: Variable

### Voting

**Core Team Vote:**
- Simple majority (>50%)
- Quorum: 4 of 7 members
- BDFL veto (Phase 2 only)

**Types of Votes:**
- **Accept** - RFC approved, proceed to implementation
- **Reject** - RFC declined with reasoning
- **Defer** - Good idea, wrong time
- **Revise** - Needs changes, resubmit

---

## Release Process

### Version Numbers

**Semantic Versioning:** `MAJOR.MINOR.PATCH`

- **MAJOR** - Breaking changes (rare)
- **MINOR** - New features (backward compatible)
- **PATCH** - Bug fixes

**Example:**
- v0.1.0 → v0.2.0 (new features)
- v0.2.0 → v0.2.1 (bug fix)
- v0.9.0 → v1.0.0 (stable release)

### Release Schedule

**Minor Releases:** Every 6 weeks
**Patch Releases:** As needed (critical bugs)
**Major Releases:** Every 12-18 months

### Stability Guarantees

**v0.x (Pre-1.0):**
- Breaking changes allowed with notice
- Deprecation warnings 1 release before removal
- Migration guides provided

**v1.0+:**
- No breaking changes without MAJOR version bump
- At least 6 months deprecation period
- Automated migration tools when possible

---

## RFC System

See [RFC_PROCESS.md](./rfcs/RFC_PROCESS.md) for complete details.

**Quick Overview:**
1. Create RFC from template
2. Submit as pull request
3. Community discussion (GitHub + Discord)
4. Core team review
5. Decision made
6. Implementation tracked

---

## Community Roles

### Contributors

**Anyone who:**
- Submits pull requests
- Reports bugs
- Writes documentation
- Helps in discussions

**Recognition:**
- CONTRIBUTORS.md listing
- Release notes credit
- Swag (stickers, t-shirts)

### Committers

**Trusted contributors with:**
- Commit access to repository
- Can merge PRs
- Active for 6+ months
- High-quality contributions

**How to become:**
- Nominated by Core Team member
- Vote by Core Team
- Background check (security)

### Core Team

**See Phase 2 above**

**Requirements:**
- 2+ years significant contributions
- Domain expertise
- Available 10+ hours/week
- Community trust

---

## Code of Conduct

### Our Standards

**We are committed to providing a welcoming and inspiring community for all.**

**Expected Behavior:**
- Be respectful and inclusive
- Accept constructive criticism
- Focus on what's best for the community
- Show empathy towards others

**Unacceptable Behavior:**
- Harassment or discrimination
- Trolling or insulting comments
- Personal or political attacks
- Publishing private information
- Spamming or self-promotion

### Enforcement

**Process:**
1. Warning (first offense)
2. Temporary ban (repeated offense)
3. Permanent ban (severe or continued violations)

**Contact:** conduct@astrixa.dev

---

## Security Policy

### Reporting Vulnerabilities

**DO NOT** create public issues for security vulnerabilities.

**Instead:**
1. Use [GitHub Security Advisories](https://github.com/Podamekalajagadeesh/astrixa-lang/security/advisories/new)
2. Include: Description, reproduction steps, impact
3. Response time: 48 hours
4. Fix timeline: 30 days for critical, 90 days for others

**Disclosure:**
- Coordinated disclosure (30-90 days)
- Security advisory published
- CVE assigned if applicable
- Credit to reporter (if desired)

**Bug Bounty:** Coming in v1.0

---

## Conflict Resolution

### Disagreements

**When Core Team members disagree:**
1. Open discussion (GitHub issue)
2. Present arguments and evidence
3. Seek compromise
4. Vote if no consensus
5. BDFL decides if tied (Phase 2)

### Community Conflicts

**When community members conflict:**
1. Private discussion (moderators)
2. Mediation if needed
3. Code of Conduct enforcement
4. Escalate to Core Team

---

## Intellectual Property

### Copyright

**All contributions:**
- Licensed under MIT
- Copyright retained by contributors
- Contributor License Agreement (CLA) required

### Trademarks

**"ASTRIXA" and logo:**
- Owned by ASTRIXA Foundation (Phase 3)
- Usage guidelines provided
- Protection against misuse

### Patents

**Defensive patent pledge:**
- Will not sue for patent infringement
- Only if implementing ASTRIXA spec
- Community protection

---

## Financial Sustainability

### Phase 1-2: Open Source Only

**No monetization required**

**Funding sources:**
- Personal time
- Sponsorships (GitHub Sponsors)
- Grants (Protocol Labs, Ethereum Foundation)

### Phase 3: Foundation

**Revenue streams:**
- Conference tickets
- Enterprise support (optional)
- Training and certification
- Donations

**Principles:**
- Language remains free forever
- No "enterprise edition"
- No feature paywalls
- Community first

---

## Communication Channels

### Official Channels

- **GitHub:** Code, issues, RFCs, discussions
- **GitHub Discussions:** Community Q&A and proposals
- **GitHub Issues:** Bug reports and feature requests
- **GitHub Pull Requests:** Code contributions

### Core Team Meetings

**Frequency:** Bi-weekly (2 weeks)
**Format:** Video call
**Duration:** 1-2 hours
**Minutes:** Published publicly (GitHub)

**Agenda:**
1. RFC reviews
2. Release planning
3. Community feedback
4. Security issues
5. Strategic decisions

---

## Transparency

### Public Information

**Always public:**
- RFCs and discussions
- Meeting minutes
- Roadmap and priorities
- Financial reports (Phase 3)
- Security advisories (after fix)

**Private information:**
- Security reports (until fixed)
- Personal information
- Legal matters
- HR issues (Phase 3)

---

## Amendment Process

**This governance document can be amended through:**
1. RFC proposing changes
2. Core Team vote (2/3 majority)
3. 30-day notice period
4. Community feedback considered

---

## Transition Timeline

**Q1 2025:** Phase 1 (BDFL)
**Q3 2025:** Establish Core Team
**Q4 2025:** Transition to Phase 2
**Q2 2026:** Establish Foundation (Phase 3)
**Q4 2026:** Full Foundation governance

---

## Success Metrics

**Language is stable when:**
- 95% of code survives major version upgrade
- Breaking changes < 1 per year
- Security issues resolved < 30 days
- RFC process consistently followed
- Community trust high

**Community is healthy when:**
- >1000 active developers
- >100 contributors
- >10 Core Team members
- >5000 Discord members
- Multiple independent projects

---

## Why This Matters

**Languages die when:**
- One person controls everything (Bus factor = 1)
- Breaking changes alienate users
- Toxic community drives people away
- No clear direction or roadmap
- Corporate takeover ruins trust

**ASTRIXA will survive because:**
- Clear governance from day 1
- Community-driven through RFCs
- Stability guarantees
- Multiple maintainers
- Independent foundation

---

## Learn From Others

### Good Examples

**Rust:**
- Strong RFC process
- Core team governance
- Stability guarantees
- Edition system for changes

**Python:**
- PEP system
- Python Software Foundation
- Long-term stability

**TypeScript:**
- Clear leadership (Microsoft)
- Regular releases
- Backward compatibility

### Bad Examples

**Avoid:**
- Fragmentation (Python 2 vs 3)
- Corporate control (Oracle/Java)
- Toxic leadership (various)
- No stability (JavaScript pre-ES6)

---

## Questions?

**Open an issue:** github.com/astrixa-lang/astrixa
**Join Discord:** discord.gg/astrixa
**Email:** governance@astrixa.dev

---

**ASTRIXA: Built by the community, for the community.**

**Last Updated:** January 2025  
**Version:** 1.0  
**Status:** Draft (will be finalized at v0.5)
