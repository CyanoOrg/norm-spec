# Proposal: Batch (Multi-Target) Collect

Planning input filed by the dsh-norm-spec adapter track on 2026-09-03, for
the maintainer's scheduling consideration alongside stable `v0.1.0`
promotion planning. Revised 2026-09-03 after a joint review against the
adapter-side target-context plan: each open question now carries an
explicit recommended position. Adopted 2026-09-03 as decision D020 in
`docs/decisions.md`; this document remains the design rationale and is
not itself a decision record.

## Problem

Downstream host adapters page conventions for the directories an agent is
actively working in. A single agent step can touch several directories, and
the natural consumer behavior is one collection per step covering a small
bounded set of active directories.

`collect` today accepts exactly one target
(`norm-core/src/collector.rs::collect_candidate_paths`). A consumer that
needs N directories must either:

- issue N collect calls — for CLI-based adapters this means N upstream
  process spawns per agent step — and compose the chains itself, or
- restrict itself to one directory and lose the others' conventions.

The first path fragments cross-chain merge semantics across consumers; the
second weakens convention delivery. Both argue for the merge semantics
living here, once, in `norm-core` — consistent with the repository rule
that collection semantics are declared here and not forked downstream.

## Proposal

Extend collection to accept a bounded set of targets in one operation —
repeatable `--target` on the CLI and a corresponding multi-target machine
request — with norm-spec owning the authoritative cross-chain semantics.

## Recommended semantics

The filed open questions, each with a recommended position for review.
The decision record fixes the final form.

1. **Output shape — per-scope sections.** One ordered group per target.
   Scopes are peers, not a hierarchy; a single flat ordering would have to
   invent a cross-scope precedence rule with no semantic basis. Sections
   preserve the canonical most-specific-first ordering within each chain
   and make scope attribution explicit. The reference consumer (the
   adapter's bounded multi-scope reminder) renders exactly this shape.

2. **Shared conventions — deduplicate with attribution by reference.** A
   `.norm` file present in several target chains (typically a root file)
   is collected once per chain today. In a batch, its bytes appear once:
   the first section in request order that sees it carries the full
   entry, and every later section that also sees it carries a reference
   entry (path and content digest; exact shape fixed with the v2 contract
   fixtures) at that file's exact chain position. Per-scope
   most-specific-first ordering stays exact, no bytes repeat, and the
   scopes that see a file are exactly the sections holding its full or
   reference entry.

3. **Cross-scope ordering — request order.** Sections appear in request
   order. This is deterministic and is the only cross-scope order with
   consumer meaning (the reference adapter's target set is recency
   ordered). Stated as contract: most-specific-first is canonical within
   a section; request order is canonical across sections.

4. **Failure — validate the whole request up front; no partial success.**
   All targets are validated before any collection: a malformed target
   (absolute or escaping) and a missing project root fail the whole
   request with a single `norm-spec/error/v1` error. After validation,
   collection cannot fail per target: a scope with no `.norm` files
   yields an empty section, which is success. The filed "unavailable"
   per-target state therefore does not exist in v2 — a mid-session
   invalid target is consumer state drift and should be seen loudly, not
   papered over with a per-entry status.

5. **Bounded output — one shared budget plus a target cap.** One size
   budget across the whole batch with the existing fail-not-truncate
   posture. In addition, the request form declares a maximum target count
   (recommended: 8; the reference adapter's working set is 4), enforced
   as request validation, so the budget's meaning does not degrade as N
   grows. The exact number is fixed in the decision record.

6. **Versioning — a new `collect/v2` request and response.** Multi-target
   changes the response shape; an optional multi-target field inside
   `collect/v1` would make one version emit two shapes. `collect/v1`
   stays frozen with the 82-case contract untouched. `collect/v2` is
   listed in the compatibility handshake so consumers discover batch
   support before use, and the downstream compatibility pin is bumped
   deliberately at adoption.

7. **Target validation — carried over unchanged.** Per-target
   contained-relative normalization and the file-target-begins-at-parent
   rule apply to each target exactly as in `collect/v1`.

## Chain identity and overlap

Normalization precedes every batch-level identity decision. Each target is
normalized first (file targets begin at their parent directory), so a
request mixing a file and its parent collapses naturally, and a repeated
target collapses into the earlier section. Ancestor-overlapping sets
(`a/` together with `a/b/`) remain distinct scopes; their overlap is
handled by the reference-entry deduplication above — shared bytes appear
once at full fidelity and as references elsewhere.

## Stability

Identical requests must produce byte-identical responses. Single-target
collect already behaves this way; batch collect states it as an explicit
contract clause, because the reference consumer's digest and single-slot
replacement mechanics depend on it (same directory set, same bytes, no
churn).

## Consumer impact

- dsh-norm-spec currently fans out over `collect` v1 and merges at the
  projection layer (union, dedupe, labeling — no inheritance semantics).
  Once batch collect exists, that merge migrates upstream and the adapter
  consumes one process call per step. Its plan and boundary notes are in
  `dsh-norm-spec/docs/planning/target-context-plan.md`.
- The adapter's prompt-context payload gains its own version identifier in
  the same adoption window (target-context plan WS2). The two version
  bumps — `collect/v2` and the prompt-context payload — should be
  discoverable in one compatibility-handshake pass so consumers can gate
  both together.
- The compatibility pin (`=0.1.0-rc.1` downstream) is bumped deliberately
  at adoption, not silently.
- pi-norm-spec can mirror the same consumption when the Host Adapter SDK
  converges.

## Boundary note

Host-side paging, injection timing, and enforcement stay downstream per this
repository's positioning. This proposal concerns collection semantics only:
what it means to collect for a set of directories in one authoritative
operation.

## Scheduling

Candidate for the post-stable backlog. It is additive, does not disturb the
frozen 82-case behavior contract or the rc.1 single-target response shape,
and can ride a minor-version line after stable promotion.
