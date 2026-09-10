# dsc: vocabulary specification

**Author:** Claude Sonnet 5 (Anthropic), on behalf of shared-claude@unxwares.com
**Document type:** vocabulary specification
**AI-assisted authorship note:** this document was drafted end-to-end by
an AI coding assistant (Claude Sonnet 5) against a vocabulary design that
had already been produced and verified against the `ds-odrl-engine-rs`
source and the upstream RDF vocabularies it reuses. A human reviewed and
accepted the result.

This is the authoritative reference for the `dsc:` extension vocabulary
and for the file-shape conventions every test case under `cases/` follows.
Read it before writing a new test case.

## 1. Namespace

- **IRI:** `https://ds-labs-org.github.io/ds-odrl-compliance-rdf/ns#`
- **Prefix:** `dsc:`
- **Shape:** hash namespace (`#`), not slash — every term is a fragment
  of one document, so a single GET of the namespace IRI's document part
  (`.../ns`, or the explicit `.../ns.ttl`) returns the whole vocabulary in
  one request. There is no per-term dereferencing to support, so a hash
  namespace costs nothing here and avoids the slash-namespace tax of one
  request per term.
- **Path is unversioned.** `https://ds-labs-org.github.io/ds-odrl-compliance-rdf/ns#`
  never gets a `/v1/`, `/v2/`, etc. segment. Governance is **additive-only**:
  new terms may be added, existing terms may gain clarifying `rdfs:comment`
  text, but a term already published here is never removed or repurposed.
  If a term turns out to be wrong, it is deprecated in place (documented as
  such) and a replacement term is added alongside it — the same discipline
  this repo's parent project (`dataspace`, ds42.org) applies to accepted
  ADRs: never edit the decision after the fact, supersede it.
- **Point releases** are tracked with `owl:versionInfo` on the ontology
  resource itself (currently `"0.2.0"`), not in the IRI. This lets tooling
  and changelogs cite a precise version without the namespace IRI itself
  ever changing, which would break every existing test case's `@prefix`.
- The ontology resource also carries `vann:preferredNamespacePrefix "dsc"`
  and `vann:preferredNamespaceUri`, the standard VANN terms consumers use
  to recover the canonical prefix for tooling that doesn't hardcode it.

## 2. The 23 new terms

Six classes, four named individuals (values of two enumerations), and
thirteen properties. Definitions below are the same ones carried as
`rdfs:comment` in `ns.ttl` — this table exists so a reader doesn't have to
open the Turtle to get an overview.

### 2.1 Classes (6)

| Term | Definition |
|---|---|
| `dsc:TestCase` | One self-contained ODRL compliance test case: a Request under test plus its expected `report:PolicyReport` outcome(s). |
| `dsc:Request` | The exact wire-contract Request object: target asset, requested action, profile, policies under test, and claims. |
| `dsc:ClaimAssertion` | One entry of the wire contract's flat claims map, for a fact not already carried by `odrl:Party`, a Verifiable Credential's own envelope, or DPV. May carry more than one `rdf:value` for a multi-valued (array) claim. |
| `dsc:ClaimKey` | A constraint left-operand (subclass of `odrl:LeftOperand`) that resolves, at evaluation time, to a named entry of the request's claims map. |
| `dsc:DutyMode` | Enumeration class: `advise` \| `deny` (wire: `config.dutyMode`). |
| `dsc:Behaviour` | Enumeration class: `open` \| `closed` (wire: `config.behaviour`; the ODRL Formal Semantics CG's own parameter). |

### 2.2 Named individuals (4)

| Term | Definition |
|---|---|
| `dsc:Advise` | An unfulfilled duty is reported but does not deny the permission it governs. |
| `dsc:Deny` | An unfulfilled duty denies the permission it governs, unconditionally, and excludes that permission from `odrl:conflict` resolution while outstanding. |
| `dsc:Open` | An action neither permitted nor prohibited is allowed. |
| `dsc:Closed` | An action neither permitted nor prohibited is denied. |

### 2.3 Properties (13)

| Term | Domain -> Range | Definition |
|---|---|---|
| `dsc:request` | `dsc:TestCase` -> `dsc:Request` | The Request this test case evaluates. |
| `dsc:expectedOutcome` | `dsc:TestCase` -> `report:PolicyReport` | An expected outcome tree, one per policy under test. |
| `dsc:profile` | `dsc:Request` -> `odrl:Profile` | The engine profile governing this Request, inline or by reference (by-reference: at the referenced document's fixed `#profile` fragment). |
| `dsc:policy` | `dsc:Request` -> `odrl:Policy` | One member of the Request's policies array. |
| `dsc:claim` | `dsc:Request` -> `dsc:ClaimAssertion` | One entry of the Request's flat claims map. |
| `dsc:inheritsFrom` | `odrl:Policy` -> `rdf:List` | Ordered list of parent policies (same Request) whose rules and unset assigner/assignee this policy replicates before evaluation. Wire: `policy.inheritFrom`. Precedence: first-listed non-empty parent wins for assigner/assignee; permission/prohibition/obligation sets are unioned across all ancestors regardless of order. |
| `dsc:dutyMode` | `odrl:Profile` -> `dsc:DutyMode` | Which of `dsc:Advise`/`dsc:Deny` this profile uses. |
| `dsc:behaviour` | `odrl:Profile` -> `dsc:Behaviour` | Which of `dsc:Open`/`dsc:Closed` this profile uses. |
| `dsc:partyIdentityClaim` | `odrl:Profile` -> `xsd:string` | Claims-map key checked against every policy's `odrl:assignee`. |
| `dsc:agreementAssigneeClaim` | `odrl:Profile` -> `xsd:string` | Claims-map key checked against `odrl:assignee`, Agreement-kind policies only. |
| `dsc:key` | (unrestricted) -> `xsd:string` | The claims-map key a `ClaimAssertion` or `ClaimKey` concerns. Domain deliberately unrestricted — both classes use it. |
| `dsc:aboutParty` | `dsc:ClaimAssertion` -> `odrl:Party` | Which party a claim concerns, when the claim's value is not itself that party. Documentary, not part of the wire translation. Applies to a multi-valued claim as a whole. |
| `dsc:policyKind` | `odrl:Policy` -> `xsd:string` | The literal `policy.kind` wire string, used only when it names none of ODRL 2.2's seven native `odrl:Policy` subclasses. Never asserted alongside a native `rdf:type` for one of those seven. |

## 3. What's deliberately reused, not reinvented

The `dsc:` vocabulary is intentionally small. Everything that an existing,
widely-implemented vocabulary already says well is reused verbatim rather
than shadowed with a competing term:

- **ODRL 2.2 itself** carries the policy model: policy kind is expressed
  through ODRL's own seven native `odrl:Policy` subclasses (`odrl:Set`,
  `odrl:Offer`, `odrl:Agreement`, `odrl:Privacy`, `odrl:Request`,
  `odrl:Ticket`, `odrl:Assertion`) wherever the wire's `policy.kind` names
  one of them; `dsc:policyKind` exists only as an escape hatch for a
  `policy.kind` string that names none of the seven, and is never
  asserted alongside a native `rdf:type` for one of them. `odrl:conflict`,
  the full rule/constraint/logical-constraint machinery
  (`odrl:and`/`odrl:or`/`odrl:xone`, `odrl:LeftOperand`,
  `odrl:operator`, `odrl:rightOperand`, refinements, duties,
  consequences, remedies) are all native ODRL terms, unmodified. Even the
  wire contract's own `config` envelope is carried on a native
  `odrl:Profile` node (via `dsc:dutyMode`/`dsc:behaviour`/the two claim-key
  properties) rather than a `dsc:`-minted class competing with `odrl:Profile`.
- **Party identity** is `odrl:Party` plus whatever combination of FOAF
  (`foaf:Person`, `foaf:mbox`), vCard (`vcard:role`), or schema.org terms
  a test case needs to describe a party — `dsc:` adds nothing here.
- **The credential envelope** for a verified claim is the W3C Verifiable
  Credentials Data Model 2.0 (`cred:VerifiableCredential`, `cred:issuer`,
  `cred:validFrom`, `cred:credentialSubject`) verbatim; the VC's own
  `cred:credentialSubject` is the link into a `dsc:ClaimAssertion`, so
  `dsc:` doesn't need its own "this claim came from a credential" property.
- **Purpose and recipient values** (and any other DPV-shaped fact) are W3C
  DPV terms (e.g. `dpv:AcademicResearch`, `dpv:DataController`,
  `dpv:ThirdParty`) used directly as `odrl:rightOperand` values or claim
  values — `dsc:` mints no parallel purpose/recipient taxonomy.
- **Expected outcomes** are the FORCE `report:` compliance-report
  vocabulary, used verbatim (`report:PolicyReport`, `report:ruleReport`,
  `report:PermissionReport`/`ProhibitionReport`/`DutyReport`, the
  activation/attempt/performance/deontic state terms, `report:premiseReport`,
  `report:conditionReport`). Two upstream limitations are worth flagging
  explicitly, since they shape how a test case is written rather than being
  bugs in this vocabulary:
  - `report:policy`'s declared range excludes Ticket-, Privacy-, Request-,
    and Assertion-kind policies — only `odrl:Set`/`odrl:Offer`/`odrl:Agreement`
    are in range. A test case exercising one of the excluded kinds still
    asserts `report:policy` against it in practice (as every real compliance
    report for such a policy must), just knowingly outside the vocabulary's
    own stated range.
  - `report:attemptState` excludes `report:DutyReport` from its domain — a
    duty's report never carries `report:attemptState`, only
    `report:activationState`/`performanceState`/`deonticState`. The worked
    example's duty rule-reports (`:rr-a-duty-notify` etc.) follow this
    deliberately: no `report:attemptState` triple on any `report:DutyReport`.

## 4. File-shape conventions

### 4.1 One file per test case

Each test case is one self-contained Turtle file at `cases/<slug>.ttl`. The
slug is a short, hyphenated, descriptive name (e.g.
`inherited-agreement-duty-chain-01`); a trailing `-NN` counter
disambiguates multiple cases that share a topic slug.

### 4.2 The `@base` / `@prefix : <#>` pattern

Every case file opens with:

```turtle
@base <https://ds-labs-org.github.io/ds-odrl-compliance-rdf/cases/<slug>> .
@prefix :      <#> .
```

So every locally-defined resource in the file is a `#fragment` of that
file's own canonical URL, and can be written throughout the rest of the
file as a short `:name`. This keeps every case file globally dereferenceable
(`https://ds-labs-org.github.io/ds-odrl-compliance-rdf/cases/<slug>#request`
is a real, resolvable IRI once the corpus is published) without the file
needing to hardcode its own slug more than once.

### 4.3 `dsc:TestCase` / `dsc:Request` split

A case's root resource (`:testcase`) is a `dsc:TestCase` carrying `dct:title`,
provenance (`dct:created`, `dct:creator`), exactly one `dsc:request`, and one
or more `dsc:expectedOutcome` trees (one `report:PolicyReport` per policy
under test). The `dsc:Request` (`:request`) is kept as a distinct resource
from the `dsc:TestCase` deliberately: it mirrors the wire contract's own
`Request` object one-to-one (target, action, profile, policies array,
claims map), so a tool that only needs to reconstruct the wire JSON can
walk `:request` alone without needing to understand the test-case wrapper
around it.

### 4.4 Claims block: five shapes

A request's `dsc:claim` set typically mixes several claim shapes, all seen
in the worked example:

1. **Identity claim** — the claim's `rdf:value` *is* the party resource
   itself (e.g. `:claim-identity`'s value is `:alice`).
2. **VC-backed claim** — a `cred:VerifiableCredential`'s own
   `cred:credentialSubject` points at the `dsc:ClaimAssertion`; the
   assertion itself carries no separate "verified" flag, since the
   presence of a credential pointing at it *is* the verification evidence.
3. **Bare environment fact** — a claim with no credential and no party
   link, just a `dsc:key` and an `rdf:value` (e.g. session time).
4. **Per-party fact** — a claim whose value is not itself a party (a
   number, a string) but that concerns a specific party; `dsc:aboutParty`
   makes that link explicit and is load-bearing precisely because the
   value alone doesn't carry it.
5. **Array-valued (multi-valued) claim** — one `dsc:ClaimAssertion` node
   carrying more than one `rdf:value` triple. This is a *set*, not an
   `rdf:List`: order is not significant, matching the wire contract's own
   JSON array semantics for a multi-valued claim (`isAnyOf` and similar
   operators treat it as a membership test, not a sequence).

**Comparison convention:** when a claim value or an `odrl:rightOperand` is
compared, a literal compares by its lexical form and a resource (IRI)
compares by that IRI — never by, e.g., a resource's `rdfs:label`. This is
why `:claim-purpose`'s `rdf:value dpv:AcademicResearch` (a resource) can be
`eq`-compared directly against a constraint's `odrl:rightOperand
dpv:AcademicResearch`: both sides stringify to the identical IRI.

### 4.5 Left-operand resolution rule

A constraint's `odrl:leftOperand` resolves against the same flat claims map
two ways:

- **Explicitly**, via `dsc:ClaimKey` — a blank node
  `[ a dsc:ClaimKey ; dsc:key "riskScore" ]` used directly as the
  `odrl:leftOperand`, for a fact that has no native ODRL left-operand term.
- **Implicitly**, for any *native* ODRL contextual left-operand term
  (`odrl:purpose`, `odrl:recipient`, `odrl:spatial`, etc.): such a term
  resolves against the claims map by its own **local name** — `odrl:purpose`
  looks up the claims-map key `"purpose"` — exactly as if it had been
  written as `[ a dsc:ClaimKey ; dsc:key "purpose" ]`. `dsc:ClaimKey` is
  the general mechanism; using a native ODRL term directly is the
  preferred shorthand whenever ODRL already names the concept.

### 4.6 Profile: inline vs. by-reference

A `dsc:profile` value may be:

- **Inline** — a local `odrl:Profile` node defined in the same file (as in
  the worked example's `:profile`), the right choice when a profile is
  specific to one test case.
- **By-reference** — an IRI pointing into a separate document under
  `profiles/`. A by-reference profile document **must** root its profile
  node at a fixed `#profile` fragment of that document's own canonical
  URL (e.g. `.../profiles/strict-deny.ttl#profile`), never at an
  arbitrarily-named fragment. This lets a consumer resolve a by-reference
  profile by a direct subject match against the known `#profile` IRI,
  rather than having to parse the whole document and scan for whichever
  resource happens to have `rdf:type odrl:Profile` — a fixed, predictable
  anchor instead of a type-scan.

## 5. The worked example

`cases/inherited-agreement-duty-chain-01.ttl` is the canonical worked
example and the one case currently published. It exercises, in a single
request:

- A duty-free permission and a duty-gated permission for the **same**
  action and target, kept as **separate** rules — because the real engine
  excludes a duty-gated permission from `odrl:conflict` resolution
  entirely, the two must be reported (and reasoned about) independently
  rather than as one merged rule.
- A genuine conflict between that duty-free permission and a colliding
  prohibition, resolved by `odrl:conflict odrl:perm` in the permission's
  favor — which means the prohibition's own `odrl:remedy` never fires.
- The duty-gated permission being independently denied via
  `dsc:dutyMode dsc:Deny`, driven by an **outstanding duty -> consequence
  chain** (`:duty-notify` has no constraint, so it is always outstanding by
  design, and its `odrl:consequence` `:duty-escalate` is reported as a
  sibling `report:ruleReport`, not nested inside the duty's own report).
- Policy inheritance (`dsc:inheritsFrom`) from an `odrl:Set` into an
  `odrl:Agreement`: both permissions are replicated into the child policy,
  the child leaves `odrl:assigner` unset (so it inherits the parent's), and
  sets its own `odrl:assignee` (so that one is authoritative, not
  inherited) — which is exactly the scenario where the inherited
  duty-free permission now collides with a prohibition that only exists on
  the child policy, and the conflict resolution must be evaluated fresh
  for that policy rather than reused from the parent's report.
- A nested `odrl:and`/`odrl:or` constraint mixing a native `odrl:purpose`
  left-operand with two `dsc:ClaimKey` left-operands (`riskScore`, `roles`),
  including the `roles` `isAnyOf` test against the array-valued claim.
- All five claim shapes from section 4.4 above, including the
  `dpv:AcademicResearch`-valued purpose claim exercising the
  resource-comparison convention from section 4.4.

A new test case does not need to exercise everything this one does — most
should be much smaller and focused on one behavior. This one is dense on
purpose, to double as worked documentation of nearly every corner of the
vocabulary at once.
