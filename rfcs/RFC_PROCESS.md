# RFC Process (Request for Comments)

**Every language change must go through RFC.**

## What is an RFC?

An RFC (Request for Comments) is a design document that:
- Proposes a new feature or change
- Explains the motivation
- Details the implementation
- Discusses alternatives
- Considers impact

**Why RFCs?**
- Prevents rushed decisions
- Enables community input
- Documents reasoning
- Creates paper trail
- Ensures stability

---

## When to Write an RFC

### ✅ REQUIRES RFC

- New syntax or keywords
- Language semantics changes
- Standard library additions
- Breaking changes
- Deprecations
- Type system changes
- Compiler behavior changes
- ABI/bytecode changes

### ❌ NO RFC NEEDED

- Bug fixes (no behavior change)
- Documentation improvements
- Performance optimizations (same behavior)
- Tooling improvements (LSP, build tools)
- Website updates
- Internal refactoring

**When in doubt, ask in Discord #rfc-discussion**

---

## RFC Template

Use this template for all RFCs:

```markdown
# RFC XXXX: [Title]

- **Start Date:** YYYY-MM-DD
- **Author:** Your Name (@github-handle)
- **Status:** Draft | Review | Accepted | Rejected | Implemented
- **Tracking Issue:** #XXX

## Summary

One paragraph explanation of the feature or change.

## Motivation

Why are we doing this? What use cases does it support? What problems does it solve?

**Examples of problems:**
- Current syntax is verbose
- Missing critical functionality
- Performance bottleneck
- Security issue
- Compatibility with Web3 standards

## Guide-level Explanation

Explain the proposal as if teaching it to an ASTRIXA user.
Use examples and show before/after code.

**Before:**
\```astrixa
// Current approach
let x = old_way();
\```

**After:**
\```astrixa
// New approach
let x = new_way();
\```

## Reference-level Explanation

Technical details. How does it work internally?

**Syntax:**
\```
grammar rule
\```

**Semantics:**
- What happens at compile time
- What happens at runtime
- Edge cases and corner cases

**Type System:**
- Type inference rules
- Type checking rules
- Constraints

## Drawbacks

Why should we NOT do this?

- Complexity added
- Learning curve
- Implementation cost
- Maintenance burden

## Alternatives

What other designs were considered? Why was this chosen?

**Alternative 1:**
- Description
- Pros
- Cons

**Alternative 2:**
- Description
- Pros
- Cons

## Prior Art

What similar features exist in other languages?

- **Rust:** [Example and link]
- **Python:** [Example and link]
- **TypeScript:** [Example and link]
- **Solidity:** [Example and link]

## Unresolved Questions

What parts are unclear or need more design?

- Question 1
- Question 2

## Future Possibilities

What future work does this enable?

- Extension 1
- Extension 2

## Implementation Plan

**Phase 1: Specification**
- Write formal spec
- Update grammar

**Phase 2: Implementation**
- Lexer changes
- Parser changes
- Type checker changes
- Compiler changes

**Phase 3: Testing**
- Unit tests
- Integration tests
- Example programs

**Phase 4: Documentation**
- Language reference
- Tutorial
- Examples

**Estimated Effort:** [Small | Medium | Large]
**Target Release:** v0.X.0

## Backwards Compatibility

**Is this a breaking change?** Yes | No

If yes:
- What code will break?
- How can users migrate?
- Can we provide automated migration?
- Deprecation timeline?

## Security Considerations

Does this affect security?

- Smart contract safety
- Memory safety
- Type safety
- Gas dos vectors

## Performance Considerations

Does this affect performance?

- Compile time impact
- Runtime impact
- Gas cost impact
- Memory usage

## Tooling Impact

What tools need updates?

- LSP (autocomplete, diagnostics, etc.)
- VS Code extension
- Package manager
- Build tools

## Teaching

How do we teach this?

- Update tutorial
- Add examples
- Write blog post
- Record video

## References

Links to:
- Related issues
- Prior discussions
- External resources
- Research papers
```

---

## RFC Lifecycle

### 1. Draft

**Author writes RFC:**
- Create file: `rfcs/XXXX-title.md`
- Fill in template
- Create PR to `rfcs/` folder

**Filename format:** `XXXX-short-title.md`
- XXXX = RFC number (next available)
- short-title = kebab-case summary

**Examples:**
- `0001-async-await.md`
- `0002-pattern-matching.md`
- `0003-null-safety.md`

### 2. Review (2 weeks)

**Core Team reviews:**
- Is it well-written?
- Is motivation clear?
- Are alternatives considered?
- Is implementation feasible?

**Possible outcomes:**
- **Ready for Comment** - Move to next phase
- **Needs Revision** - Author updates RFC
- **Reject Early** - Not aligned with goals

### 3. Public Comment (2 weeks)

**Community provides feedback:**
- Comment on PR
- Discuss on Discord #rfc-discussion
- Suggest alternatives
- Point out issues

**Author responds:**
- Answer questions
- Revise RFC based on feedback
- Update alternatives section

### 4. Final Comment Period (1 week)

**Last call for objections**

Core Team member announces:
> "RFC #XXXX is entering Final Comment Period. Last chance for feedback before decision."

### 5. Decision (1 week)

**Core Team votes:**
- **Accept** - RFC approved, ready for implementation
- **Reject** - Declined with written reasoning
- **Defer** - Good idea, wrong time (revisit later)
- **Revise** - Major changes needed, restart process

**Decision documented in:**
- RFC status updated
- Comment explaining reasoning
- PR merged (if accepted) or closed (if rejected)

### 6. Implementation

**Tracking Issue Created:**
- Links to RFC
- Implementation checklist
- Assigned to implementer

**Pull Requests:**
- Reference RFC number
- Implement one piece at a time
- Tests and docs included

**Status Updates:**
- Update RFC status: "Implementation in progress"
- Comment on tracking issue
- Announce in Discord

### 7. Shipped

**When merged to main:**
- Update RFC status: "Implemented"
- Note target release version
- Add to CHANGELOG
- Announce in release notes

---

## RFC Status Values

- **Draft** - Being written
- **Review** - Core Team reviewing
- **Comment** - Public comment period
- **FCP** - Final Comment Period
- **Accepted** - Approved, awaiting implementation
- **Rejected** - Declined
- **Deferred** - Good idea, wrong time
- **Implemented** - Shipped in release
- **Superseded** - Replaced by newer RFC

---

## RFC Numbering

**Sequential numbers starting at 0001**

**How to get next number:**
1. Check latest RFC in `/rfcs/`
2. Add 1
3. Use 4-digit format (0042, 0123, etc.)

**Gaps are OK** - Rejected RFCs keep their numbers

---

## Types of RFCs

### Feature RFCs

**New language features:**
- Syntax additions
- Standard library functions
- Compiler features

**Examples:**
- Async/await
- Pattern matching
- Generics

### Process RFCs

**Changes to development process:**
- Governance updates
- Release process
- Contribution guidelines

**Examples:**
- Edition system
- LTS releases
- Security policy

### Deprecation RFCs

**Removing features:**
- Why deprecate
- Migration path
- Timeline
- Replacement

**Examples:**
- Remove old syntax
- Change default behavior
- Update standard library

---

## Tips for Writing Good RFCs

### ✅ DO

- **Start small** - Narrow, focused proposals
- **Use examples** - Show, don't just tell
- **Be specific** - Concrete syntax and semantics
- **Consider impact** - Breaking changes, migration
- **Acknowledge tradeoffs** - Every design has downsides
- **Reference prior art** - Learn from other languages
- **Be patient** - Good designs take time

### ❌ DON'T

- **Kitchen sink** - Proposing too much at once
- **Vague descriptions** - "Make it better" isn't enough
- **Ignoring alternatives** - Always consider other options
- **Breaking changes lightly** - Needs strong justification
- **Rushing** - Give community time to respond
- **Being defensive** - Accept criticism gracefully

---

## Community Participation

### Anyone Can

- **Write RFCs** - Open to everyone
- **Comment** - Feedback welcome
- **Suggest alternatives** - Help improve proposals
- **Vote (informal)** - Upvote/downvote on GitHub

### Only Core Team Can

- **Accept/Reject** - Final decision
- **Merge RFC PR** - Add to official RFCs
- **Close without decision** - If off-topic

---

## Examples of RFCs

See `/rfcs/` folder for:
- `0001-language-vision.md` - Language design principles
- `0002-async-model.md` - Async/await implementation
- `0003-smart-contract-subset.md` - Contract restrictions

Study these for good examples!

---

## RFC Amendments

**Can accepted RFCs be changed?**

**Yes, through amendments:**
1. Create new RFC (e.g., `0042-amend-0001.md`)
2. Reference original RFC
3. Explain why amendment needed
4. Go through full process

**Small fixes:**
- Typos, clarifications → Direct PR to RFC

---

## Tracking RFCs

**GitHub Labels:**
- `rfc:draft` - Being written
- `rfc:review` - Core Team reviewing
- `rfc:comment` - Public comment
- `rfc:fcp` - Final Comment Period
- `rfc:accepted` - Approved
- `rfc:rejected` - Declined
- `rfc:implemented` - Shipped

**GitHub Project Board:**
- Track implementation progress
- See all active RFCs
- Filter by status

---

## Historical Record

**All RFCs preserved forever:**
- Accepted RFCs → Explain why features exist
- Rejected RFCs → Explain why NOT to propose again
- Deferred RFCs → Ideas for future

**Learning from history:**
- Why was X designed this way?
- Was Y considered and rejected?
- When should we revisit Z?

---

## RFC Best Practices

### Before Writing

1. **Search existing RFCs** - Has this been proposed?
2. **Discuss on Discord** - Gauge interest
3. **Prototype if possible** - Test the idea
4. **Study other languages** - Learn from prior art

### While Writing

1. **Be thorough** - Cover all aspects
2. **Be clear** - Avoid jargon
3. **Be honest** - Acknowledge drawbacks
4. **Be respectful** - Consider all viewpoints

### After Submitting

1. **Be responsive** - Answer questions quickly
2. **Be flexible** - Willing to revise
3. **Be patient** - Process takes time
4. **Be gracious** - Accept outcome

---

## Common Rejection Reasons

**Why RFCs get rejected:**

1. **Not aligned with vision** - Doesn't fit ASTRIXA goals
2. **Too complex** - Cost > benefit
3. **Better alternative** - Different solution preferred
4. **Wrong time** - Good idea, wrong stage
5. **Insufficient motivation** - Problem not important enough
6. **Breaking change** - Stability cost too high
7. **Implementation burden** - Can't maintain long-term

**Rejection doesn't mean bad idea** - Often timing or fit issue

---

## Questions?

**Before submitting RFC:**
- Ask in Discord #rfc-discussion
- Review past RFCs for examples

**During RFC process:**
- Comment on GitHub PR
- Join Discord voice chat

**After decision:**
- Open issue if confused
- Email: rfcs@astrixa.dev

---

## Comparison to Other Languages

### Python PEPs

**Similar:**
- Formal proposal process
- Template-based
- Community feedback

**Different:**
- ASTRIXA RFCs are shorter
- Faster turnaround (4-5 weeks vs months)
- Smaller community (easier consensus)

### Rust RFCs

**Very similar:**
- Strong inspiration for ASTRIXA
- Same lifecycle stages
- Public comment period

**Different:**
- ASTRIXA has BDFL veto (Phase 2)
- Faster initial review
- Smaller scope per RFC

---

**The RFC process is what makes ASTRIXA stable, trustworthy, and community-driven.**

**Start your first RFC today!**

---

**Last Updated:** January 2025  
**Version:** 1.0
