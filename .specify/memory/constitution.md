# ibkr Constitution

## Core Principles

### I. Official IBKR Documentation Is the Source of Truth

Every endpoint's path, parameters, request body, and response shape MUST come from the official
IBKR Web API documentation. Third-party gists, blogs, SDKs, and unofficial specs MUST NOT be used
as a source. Where the live gateway contradicts the documentation, the live gateway settles only
what the crate must do to decode the response.

Rationale: a community source mislabelled the `marketdata/history` response; building on it would
have broken every decode.

### II. Transparent, Docs-Faithful Types

- Each endpoint MUST be a declarative `Endpoint` implementation executed by `Client`.
- Types MUST model the documented shape and nothing more; undocumented fields MUST NOT be added
  because the live gateway returns them. The one exception is a live divergence that breaks
  decoding, which MUST be fixed minimally.
- Every response that is not the expected result MUST reach the caller verbatim: a non-2xx
  response as `Error::Api` with its status and body, and a 2xx response that fails to decode as
  `Error::Decode` with its body.

Rationale: the crate is a transparent layer; callers diagnose from what IBKR actually sent.

### III. Lean and Portable

- Every dependency MUST be justified. Cryptographic, encoding, and HTTP primitives MUST be
  delegated to maintained crates rather than reimplemented.
- The crate MUST NOT couple to any cloud vendor, secrets manager, or storage location; inputs such
  as credentials MUST be accepted as plain data.
- Changes MUST stay within the scope requested.
- Comments MUST carry only non-obvious information (gotchas, wire-format quirks); they MUST NOT
  restate the code or the documentation, and SHOULD link the documentation instead.

Rationale: every line and dependency is maintenance and supply-chain surface for a small library.

### IV. Tested Against Documented Examples

- Each endpoint MUST have unit tests covering its request construction and decoding of the
  official documentation's example response.
- `cargo test --all-features` MUST pass before merge; the release workflow runs it before
  publishing.
- Checks against a live IBKR account MUST be limited to one confirming run. Intermittent
  server-side failures MUST NOT be chased with repeated live runs; they MUST be reported as
  unconfirmed.

Rationale: each live run mints a session token and competes for a real brokerage session.

### V. Semantic Versioning and Breaking Changes

- The crate MUST follow Semantic Versioning, with the version set in `Cargo.toml`.
- A change that breaks the public API MUST be marked as breaking in its commit (`feat!:`) and MUST
  bump the version accordingly.
- Merging to `main` publishes a `Cargo.toml` version that is not yet on crates.io, so the version
  MUST be correct before merge.

Rationale: every merge to `main` can become a published release.

## Security and Supply Chain

- Key material and credentials MUST NOT be committed; `*.pem` stays ignored.
- GitHub Actions workflows MUST use only first-party actions (`actions/*`, `rust-lang/*`); anything
  else MUST be a plain `run:` step.
- Publishing MUST use short-lived OIDC tokens, not stored registry tokens.

## Development Workflow

- Work MUST start on a new branch from a freshly fetched `origin/main` and reach `main` only
  through a pull request.
- Commits MUST use Conventional Commit prefixes (`feat`, `fix`, `chore`, `docs`, and `!` for
  breaking changes).
- Spec Kit artifacts for a feature (spec, plan, tasks) MUST be checked against this constitution
  before implementation.

## Governance

This constitution supersedes other project practices. Amendments MUST be made by pull request that
edits this file, states the reason, and updates the version and Last Amended date. Versioning of
this document follows Semantic Versioning: MAJOR for removing or redefining a principle, MINOR for
adding a principle or section or materially expanding guidance, PATCH for clarifications. Every
pull request review MUST verify compliance; any deviation MUST be justified in the pull request.

**Version**: 1.0.0 | **Ratified**: 2026-09-29 | **Last Amended**: 2026-09-29
