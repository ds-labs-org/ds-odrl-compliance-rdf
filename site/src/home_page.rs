use crate::app_route::AppRoute;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::components::Link;

const REPO_URL: &str = "https://github.com/ds-labs-org/ds-odrl-compliance-rdf";

#[component]
pub fn HomePage() -> Html {
  html! {
    <Content>
      <Title level={Level::H1}>{ "ds-odrl-compliance-rdf" }</Title>
      <p>
        { "A shared, implementation-neutral corpus of RDF/Turtle-serialized test cases for " }
        <strong>{ "ODRL" }</strong>
        { " policy engines. Companion project to " }
        <a href="https://github.com/ds-labs-org/ds-odrl-engine-rs" target="_blank">{ "ds-odrl-engine-rs" }</a>
        { ", a WASM ODRL policy engine, but the vocabulary and test cases here are meant to be usable against "
          }{ "any" }{ " conformant ODRL engine, not only that one." }
      </p>
      <p>
        { "Each test case pairs a request (target asset, action, profile, policies under test, and claims) "
          }{ "with its expected compliance-report outcome, using the " }
        <a href="http://www.w3.org/ns/odrl/2/">{ "ODRL 2.2" }</a>
        { ", " }
        <a href="https://www.w3.org/TR/vc-data-model-2.0/">{ "W3C Verifiable Credentials 2.0" }</a>
        { ", " }
        <a href="https://w3id.org/dpv#">{ "W3C DPV" }</a>
        { ", and " }
        <a href="https://w3id.org/force/compliance-report#">{ "FORCE compliance-report" }</a>
        { " vocabularies, plus a small residual " }<code>{ "dsc:" }</code>{ " extension vocabulary for the parts "
          }{ "none of those cover -- see the Vocabulary page." }
      </p>
      <Card>
        <CardTitle><Title level={Level::H2}>{ "Get started" }</Title></CardTitle>
        <CardBody>
          <ul>
            <li>
              <Link<AppRoute> to={AppRoute::Vocabulary}>{ "Browse the vocabulary" }</Link<AppRoute>>
              { " -- the live " }<code>{ "ns.ttl" }</code>{ " plus a human-readable term table." }
            </li>
            <li>
              <Link<AppRoute> to={AppRoute::Tests}>{ "Browse test cases" }</Link<AppRoute>>
              { " -- every case currently in " }<code>{ "cases/" }</code>{ "." }
            </li>
            <li>
              <a href={format!("{REPO_URL}/blob/main/docs/vocabulary-spec.md")} target="_blank">
                { "Read the full vocabulary specification" }
              </a>
              { " -- the durable design-rationale document for contributors writing a new case." }
            </li>
            <li>
              <a href={format!("{REPO_URL}/blob/main/ns.ttl")} target="_blank">{ "ns.ttl on GitHub" }</a>
              { " / " }
              <a href="ns.ttl">{ "raw ns.ttl (this site)" }</a>
            </li>
          </ul>
        </CardBody>
      </Card>
      <Alert r#type={AlertType::Info} title="Early v1" inline=true>
        { "This is an early v1: a small, growing corpus of worked test cases, no automated submission flow yet. See the "
          }<Link<AppRoute> to={AppRoute::Submit}>{ "Submit" }</Link<AppRoute>>{ " page for the manual PR path, "
          }{ "and " }
        <a href={format!("{REPO_URL}#readme")} target="_blank">{ "the README" }</a>
        { " for what's deferred." }
      </Alert>
    </Content>
  }
}
