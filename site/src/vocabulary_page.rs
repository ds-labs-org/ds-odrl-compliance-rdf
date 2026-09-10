use gloo_net::http::Request;
use patternfly_yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

/// Fetched at runtime (never `include_str!`-embedded) so this page always
/// shows whatever `ns.ttl` currently is at this site's own root, without
/// needing a rebuild when the vocabulary changes -- see README.md and
/// `docs/vocabulary-spec.md`.
async fn fetch_ns_ttl() -> Result<String, String> {
  let resp = Request::get("ns.ttl").send().await.map_err(|e| e.to_string())?;
  if !resp.ok() {
    return Err(format!("GET ns.ttl failed (HTTP {})", resp.status()));
  }
  resp.text().await.map_err(|e| e.to_string())
}

/// One row of the hardcoded term table below.
struct TermRow {
  term: &'static str,
  kind: &'static str,
  summary: &'static str,
}

/// The 23 terms `ns.ttl` currently defines (6 classes, 4 named
/// individuals, 13 properties), hardcoded here rather than parsed from the
/// fetched Turtle: the term *list* is stable and this is a v1 site with no
/// Turtle parser dependency. This table is NOT regenerated from `ns.ttl` --
/// only the raw content above is fetched live. If a term is ever added,
/// removed, or redefined in `ns.ttl`, THIS TABLE MUST BE UPDATED BY HAND to
/// match, or it will silently drift from the real vocabulary. See
/// `docs/vocabulary-spec.md` section 2 for the same table with full
/// definitions.
const TERMS: &[TermRow] = &[
  TermRow { term: "dsc:TestCase", kind: "Class", summary: "One self-contained ODRL compliance test case." },
  TermRow { term: "dsc:Request", kind: "Class", summary: "The exact wire-contract Request object." },
  TermRow { term: "dsc:ClaimAssertion", kind: "Class", summary: "One entry of the flat claims map." },
  TermRow { term: "dsc:ClaimKey", kind: "Class", summary: "A left-operand resolving to a claims-map entry." },
  TermRow { term: "dsc:DutyMode", kind: "Class", summary: "Enumeration: advise | deny." },
  TermRow { term: "dsc:Behaviour", kind: "Class", summary: "Enumeration: open | closed." },
  TermRow { term: "dsc:Advise", kind: "Individual", summary: "An unfulfilled duty does not deny its permission." },
  TermRow { term: "dsc:Deny", kind: "Individual", summary: "An unfulfilled duty denies its permission." },
  TermRow { term: "dsc:Open", kind: "Individual", summary: "An unaddressed action is allowed." },
  TermRow { term: "dsc:Closed", kind: "Individual", summary: "An unaddressed action is denied." },
  TermRow { term: "dsc:request", kind: "Property", summary: "TestCase -> Request." },
  TermRow { term: "dsc:expectedOutcome", kind: "Property", summary: "TestCase -> report:PolicyReport." },
  TermRow { term: "dsc:profile", kind: "Property", summary: "Request -> odrl:Profile, inline or by-reference." },
  TermRow { term: "dsc:policy", kind: "Property", summary: "Request -> odrl:Policy (one per policy under test)." },
  TermRow { term: "dsc:claim", kind: "Property", summary: "Request -> ClaimAssertion." },
  TermRow { term: "dsc:inheritsFrom", kind: "Property", summary: "Policy -> rdf:List of parent policies." },
  TermRow { term: "dsc:dutyMode", kind: "Property", summary: "Profile -> DutyMode." },
  TermRow { term: "dsc:behaviour", kind: "Property", summary: "Profile -> Behaviour." },
  TermRow { term: "dsc:partyIdentityClaim", kind: "Property", summary: "Profile -> claims-map key, checked against odrl:assignee." },
  TermRow { term: "dsc:agreementAssigneeClaim", kind: "Property", summary: "Profile -> claims-map key, Agreement policies only." },
  TermRow { term: "dsc:key", kind: "Property", summary: "The claims-map key a ClaimAssertion/ClaimKey concerns." },
  TermRow { term: "dsc:aboutParty", kind: "Property", summary: "ClaimAssertion -> odrl:Party the claim concerns." },
  TermRow { term: "dsc:policyKind", kind: "Property", summary: "The literal policy.kind string, escape hatch only." },
];

#[component]
pub fn VocabularyPage() -> Html {
  let ttl = use_state(|| None::<Result<String, String>>);

  {
    let ttl = ttl.clone();
    use_effect_with((), move |_| {
      spawn_local(async move {
        ttl.set(Some(fetch_ns_ttl().await));
      });
      || ()
    });
  }

  let ttl_block = match &*ttl {
    None => html! { <Spinner /> },
    Some(Ok(text)) => html! { <pre class="dsc-ttl"><code>{ text.clone() }</code></pre> },
    Some(Err(err)) => html! {
      <Alert r#type={AlertType::Danger} title="Could not load ns.ttl" inline=true><p>{ err.clone() }</p></Alert>
    },
  };

  html! {
    <Content>
      <Title level={Level::H1}>{ "Vocabulary" }</Title>
      <p>
        { "Namespace " }<code>{ "https://ds-labs-org.github.io/ds-odrl-compliance-rdf/ns#" }</code>
        { ", prefix " }<code>{ "dsc:" }</code>{ ". See " }
        <a href="https://github.com/ds-labs-org/ds-odrl-compliance-rdf/blob/main/docs/vocabulary-spec.md" target="_blank">
          { "docs/vocabulary-spec.md" }
        </a>
        { " for the full specification." }
      </p>

      <Title level={Level::H2}>{ "The 23 terms" }</Title>
      <table class="dsc-term-table">
        <thead>
          <tr><th>{ "Term" }</th><th>{ "Kind" }</th><th>{ "Summary" }</th></tr>
        </thead>
        <tbody>
          { for TERMS.iter().map(|row| html! {
            <tr>
              <td><code>{ row.term }</code></td>
              <td>{ row.kind }</td>
              <td>{ row.summary }</td>
            </tr>
          }) }
        </tbody>
      </table>

      <Title level={Level::H2}>{ "Raw ns.ttl (fetched live)" }</Title>
      { ttl_block }
    </Content>
  }
}
