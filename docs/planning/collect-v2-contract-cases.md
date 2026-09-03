# collect/v2 Contract Case Matrix

Planning input filed 2026-09-03 alongside D020 so the post-stable
implementation of batch (multi-target) collect starts with its
acceptance set already enumerated. Cases are planning input, not the
fixtures themselves; the exact bundle identity and case identifiers are
fixed when the fixtures land (D020: the locked 82-case A1 bundle is not
mutated — these ship under a new contract-bundle identity). Taxonomy
follows `tests/contract/` conventions: black-box, executed against the
compiled binary in isolated roots, with asserted streams, protocols,
output data, and exit codes.

## Request and response shape

- C1 — Repeatable `--target`: `collect --root . --target a --target b`
  returns one `collect/v2` response with two scopes in request order.
- C2 — A single `--target` on the v2 path yields one scope and a
  `collect/v2` response (never a v1 response).
- C3 — The response carries the collect/v2 protocol identifier and the
  root marker; per-scope fields are the v2 scope shape.

## Sections and ordering

- C4 — Sections appear in request order even when it differs from
  lexical or depth order.
- C5 — Most-specific-first ordering holds within every section.
- C6 — A repeated target collapses into the earlier section (one
  section, not two).
- C7 — A file target and its parent directory collapse into one
  section: normalization precedes every identity decision.

## Shared conventions (dedupe by reference)

- C8 — A `.norm` file collected by several scopes renders with full
  content exactly once, in the first section in request order.
- C9 — Later scopes carry a reference entry at that file's exact chain
  position (path plus content digest; the wire shape is fixed with this
  bundle).
- C10 — Each scope's chain listing includes reference positions, so
  per-scope most-specific-first order stays reconstructable.

## Overlap

- C11 — Ancestor-overlapping targets (`a` with `a/b`) remain two
  scopes; shared bytes appear once at full fidelity and as references
  elsewhere.

## Failure — whole-request, no partial success

- C12 — An escaping target (`../outside`) fails the whole request with
  one `norm-spec/error/v1` error; no partial response is written.
- C13 — An absolute target inside the root is accepted (canonicalize
  plus strip), matching the rc.1 single-target behavior.
- C14 — A target that canonicalizes outside the root fails the whole
  request.
- C15 — A missing or non-directory project root fails the request
  before any collection.
- C16 — Boundary cap: exactly 8 targets are accepted; 9 fail as
  request validation before any collection.
- C17 — An empty target list fails as request validation.

## Empty scopes and bounded output

- C18 — A scope whose directory declares no `.norm` yields an empty
  section; the request succeeds with exit code 0.
- C19 — All scopes empty yields the typed empty success with no
  synthetic content.
- C20 — Output exceeding the single shared size budget fails with the
  budget error; content is never truncated.

## Stability and compatibility

- C21 — Identical requests produce byte-identical responses.
- C22 — The compatibility handshake lists collect/v2 so consumers
  discover batch support before use.
- C23 — The locked 82-case A1 bundle passes unchanged against the same
  binary that ships collect/v2.
- C24 — v1 single-target responses remain byte-identical to rc.1 for
  identical inputs.

## Implementation-order note

D020 fixes the order: this matrix informs the contract fixtures step,
which precedes engine, facade, and CLI work. The dsh-norm-spec adapter
mirrors these semantics at its projection layer (its target-context
plan WS2 alignment constraint), so its WS3 adoption after a `0.2.0`
line ships is a mechanical migration plus a deliberate pin bump.
