# ds-odrl-compliance-rdf

A shared, implementation-neutral corpus of RDF/Turtle-serialized test
cases for ODRL policy engines. Companion project to
[`ds-odrl-engine-rs`](https://github.com/ds-labs-org/ds-odrl-engine-rs), a
WASM ODRL policy engine — but the vocabulary and test cases here are meant
to be usable against any conformant ODRL engine, not only that one.

Each test case pairs a request (target asset, requested action, profile,
policies under test, and claims) with its expected
[FORCE `report:`](https://w3id.org/force/compliance-report#) compliance
outcome, built on [ODRL 2.2](http://www.w3.org/ns/odrl/2/), the
[W3C Verifiable Credentials Data Model 2.0](https://www.w3.org/TR/vc-data-model-2.0/),
and [W3C DPV](https://w3id.org/dpv#), plus a small residual `dsc:`
extension vocabulary for the parts none of those cover.

**Browse the corpus:** https://ds-labs-org.github.io/ds-odrl-compliance-rdf/

## Repository layout

| Path | What it is |
|---|---|
| `ns.ttl` / `ns` | The `dsc:` vocabulary ontology, Turtle. Byte-identical copies — see "Known limitations" below. |
| `cases/*.ttl` | One RDF/Turtle file per test case. |
| `profiles/` | By-reference `odrl:Profile` documents (each rooted at a `#profile` fragment), shared across multiple test cases. Empty for now — the one worked case uses an inline profile. |
| `docs/vocabulary-spec.md` | The full vocabulary specification and file-shape conventions — read this before writing a test case. |
| `site/` | The Yew + Trunk app that renders this repo as a browsable GitHub Pages site. |
| `.github/workflows/pages.yml` | Builds and deploys `site/` to GitHub Pages on every push to `main` that touches `site/`, `cases/`, or `ns.ttl`. |

## How to browse

The deployed site (https://ds-labs-org.github.io/ds-odrl-compliance-rdf/)
has three real pages plus a placeholder:

- **Home** — what this repo is, with links to the vocabulary and the spec doc.
- **Vocabulary** — `ns.ttl` fetched live, plus a human-readable term table.
- **Browse Tests** — every case in `cases/`, linking to its raw file on GitHub.
- **Submit** — a disclosed placeholder; see "Status" below.

## How to contribute a test case

Automated submission via GitHub sign-in is planned but not built yet (see
"Status"). For now:

1. Fork this repository.
2. Add one new `.ttl` file under `cases/`, named `<slug>.ttl`.
3. Follow `docs/vocabulary-spec.md` section 4 for the required file shape:
   the `@base`/`@prefix : <#>` pattern, the `dsc:TestCase`/`dsc:Request`
   split, and the claim shapes.
4. Open a pull request against `main`.

## Known limitations (v1)

- **`/ns` has no real content negotiation.** The vocabulary's namespace
  IRI is `https://ds-labs-org.github.io/ds-odrl-compliance-rdf/ns#` — a
  correct implementation would Accept-header-negotiate between HTML and
  Turtle at `/ns`. Plain GitHub Pages is a static host with no reverse
  proxy, so it can't do that (this is different from ds42.org's own
  nginx-based content negotiation, which doesn't apply to a GitHub
  Pages-only repo like this one). Instead, `/ns` and `/ns.ttl` are shipped
  as byte-identical raw Turtle files: a naive dereference of the
  namespace IRI at least gets valid Turtle bytes back, just without a
  correct `Content-Type`. A future `w3id.org` redirect could add real
  content negotiation; not needed for v1.
- **No automated submission form.** The Submit page explains the planned
  GitHub-OAuth-based PR flow (reusing the PKCE pattern from the
  `dataspace` repo's `authority_registration.rs`) and gives the manual
  fork-and-PR path in the meantime.

## Status

This is v1: one worked test case (`cases/inherited-agreement-duty-chain-01.ttl`),
a finished vocabulary, and a working but minimal Pages site.
Submission-via-GitHub-OAuth is planned but not yet built.

## License

[Apache-2.0](LICENSE).
