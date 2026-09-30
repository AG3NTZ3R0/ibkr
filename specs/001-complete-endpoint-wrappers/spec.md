# Feature Specification: Complete the Endpoint Wrappers

**Feature Branch**: `001-complete-endpoint-wrappers`

**Created**: 2026-09-29

**Status**: Draft

**Input**: User description: "https://github.com/AG3NTZ3R0/ibkr/issues/9" (Pitch: Complete the endpoint wrappers)

## User Scenarios & Testing *(mandatory)*

The users of this feature are developers who call the IBKR Web API through the crate. Today each
endpoint is wrapped only as far as its first caller needed, so a documented capability the crate
omits is invisible: nothing errors, and callers design around it. The motivating case is
`direction` on the market-data history endpoint: a caller fetching a symbol's full daily history
must page backward and probe past the first bar, which for TSLA fails with
`500 Chart data unavailable`. IBKR documents forward paging, but no caller can request it.

### User Story 1 - Page history forward (Priority: P1)

A developer fetching a symbol's full daily history sets the documented paging direction on the
history request and pages forward from a start time to now, instead of probing backward for the
first bar.

**Why this priority**: It is the concrete failure that prompted the pitch, and the first missing
parameter the pitch names.

**Independent Test**: Build a history request with forward direction set and confirm the request
sent to IBKR carries the documented `direction` value; build one without it and confirm the
parameter is absent.

**Acceptance Scenarios**:

1. **Given** a history request with the direction set to forward, **When** it is sent, **Then**
   the request carries IBKR's documented value for forward paging.
2. **Given** a history request with no direction set, **When** it is sent, **Then** the request
   carries no `direction` parameter, exactly as before this feature.
3. **Given** a developer reading the history request's definition, **When** they look for a way
   to page forward, **Then** they find it without consulting IBKR's reference.

---

### User Story 2 - Every documented parameter and field is reachable (Priority: P2)

A developer using any endpoint the crate wraps — market-data history, contract search, session
initialization, tickle, and live session token — can set every request parameter and read every
response field that IBKR's Web API reference documents for it.

**Why this priority**: It generalizes Story 1 to the whole crate, so whether a capability exists
is answered by reading the crate rather than by discovering its absence late.

**Independent Test**: For each wrapped endpoint, compare its request and response against the
endpoint's entry in IBKR's reference and confirm no documented parameter or field is missing;
decode the reference's example response and confirm each documented field is populated.

**Acceptance Scenarios**:

1. **Given** the audit list of what each endpoint lacked, **When** the feature is complete,
   **Then** every listed parameter and field is present in the crate.
2. **Given** a documented parameter with a fixed set of allowed values, **When** a developer sets
   it, **Then** they choose from a typed set of those values rather than writing free text, as they
   already do for bar size and source.
3. **Given** a documented optional parameter left unset, **When** the request is sent, **Then**
   the parameter is omitted from the request.
4. **Given** a response that omits a documented field, **When** it is decoded, **Then** decoding
   succeeds and the field reads as absent.

---

### User Story 3 - Full coverage is the stated rule (Priority: P3)

A contributor adding or changing an endpoint wrapper learns from the README that endpoints are
wrapped in full, and finds on each endpoint module a link to that endpoint's reference entry.

**Why this priority**: It keeps the crate complete after this pass; without it, the next endpoint
repeats the problem.

**Independent Test**: Read the README and confirm it states the full-coverage rule; open each
endpoint module and confirm it links its reference entry.

**Acceptance Scenarios**:

1. **Given** the README, **When** a contributor reads it, **Then** it states that endpoints are
   wrapped in full, every request parameter and response field.
2. **Given** any wrapped endpoint module, **When** a contributor opens it, **Then** it links the
   endpoint's entry in IBKR's reference.

---

### Edge Cases

- **Live gateway returns a field the reference omits** (e.g. `chartAnnotations` on history): the
  reference is the floor; a field the crate already decodes is not removed.
- **Reference prose and reference example disagree**: decoding tolerates the difference; a field
  documented in either is optional and a missing one never breaks the decode.
- **An endpoint absent from the published reference file** (the live session token endpoint is
  documented under authentication): the audit uses that endpoint's official documentation page
  instead and records which source it used.
- **An endpoint with no documented parameters** (tickle): the audit records that nothing is
  missing rather than inventing fields.
- **A documented enumerated value the gateway later adds**: out of scope; the typed set reflects
  the reference at audit time.
- **Existing callers that build requests field by field**: they stop compiling when fields are
  added; the release is marked as breaking.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The crate MUST expose the documented paging direction on the market-data history
  request.
- **FR-002**: Each of the five wrapped endpoints (`/iserver/marketdata/history`,
  `/iserver/secdef/search`, `/iserver/auth/ssodh/init`, `/tickle`, `/oauth/live_session_token`)
  MUST be audited against its entry in IBKR's official Web API reference, and the audit MUST list
  every request parameter and response field the crate lacks.
- **FR-003**: The audit MUST read each endpoint from IBKR's published OpenAPI reference
  (`https://ibkrcampus.com/docs/openapi/api-reference.json`); an endpoint absent from it MUST be
  audited against its official IBKR documentation page. No third-party source MAY be used.
- **FR-004**: Every request parameter the audit lists MUST be added as an optional field that is
  omitted from the request when unset.
- **FR-005**: Every request parameter whose reference entry enumerates its allowed values MUST be
  typed as a closed set of those values, consistent with the existing bar size and source types.
- **FR-006**: Every response field the audit lists MUST be added as optional and defaulted, so a
  response missing it still decodes.
- **FR-007**: Response fields the crate already decodes MUST NOT be removed, including ones the
  reference omits.
- **FR-008**: Each wrapped endpoint MUST have tests covering construction of every added request
  parameter and decoding of the reference's example response with the added fields populated.
- **FR-009**: The README MUST state that endpoints are wrapped in full, every documented request
  parameter and response field.
- **FR-010**: Each endpoint module MUST link its endpoint's entry in IBKR's reference.
- **FR-011**: The release MUST be versioned and marked as breaking, since added request fields
  break callers that construct requests field by field.
- **FR-012**: No endpoint the crate does not wrap today MAY be added, and no existing caller MAY
  be changed to use a new parameter.

### Key Entities

- **Endpoint wrapper**: the crate's typed request and response for one IBKR endpoint; gains
  missing parameters and fields.
- **Reference entry**: IBKR's documented definition of one endpoint — its parameters, allowed
  values, response fields, and example response; the floor the wrapper must meet.
- **Audit**: a one-time list, per endpoint, of the parameters and fields the wrapper lacks
  relative to its reference entry, and the source consulted.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A developer can request forward-paged daily history for a symbol without probing
  for its first bar.
- **SC-002**: 100% of the request parameters and response fields documented for the five wrapped
  endpoints are reachable through the crate, verified against the audit list.
- **SC-003**: 100% of documented parameters with enumerated values are set by choosing from a
  typed set rather than free text.
- **SC-004**: Each of the five endpoints decodes its reference example response with every
  documented field populated, and every test in the suite passes.
- **SC-005**: A contributor can find the full-coverage rule in the README and each endpoint's
  reference link in its module in under a minute.
- **SC-006**: The work fits the pitch's appetite of one week.

## Assumptions

- The pitch's no-gos hold: no new endpoints, no caller changes, and no automated drift detection;
  the audit is a single pass.
- "Reference" means IBKR's official Web API reference, per the constitution's first principle;
  pitch #10 (caching the reference) is not a dependency.
- Retaining `chartAnnotations`, which the reference omits, is consistent with the constitution: it
  forbids adding undocumented fields, not keeping ones the crate already decodes.
- Naming of enumerated types, the form of reference links, and the release's version number are
  left to planning, as the pitch leaves them open; under the crate's pre-1.0 versioning a breaking
  change is expected to bump the minor version.
- Confirming against a live IBKR account is limited to one run, per the constitution; unit tests
  against documented examples are the primary evidence.
