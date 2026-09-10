use gloo_net::http::Request;
use patternfly_yew::prelude::*;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const REPO_BLOB_BASE: &str = "https://github.com/ds-labs-org/ds-odrl-compliance-rdf/blob/main";

/// Mirrors one entry of `site/build.rs`'s generated `test-index.json`
/// (slug, title, path) -- see that file's header for how it's produced.
#[derive(Debug, Clone, Deserialize)]
struct CaseEntry {
  slug: String,
  title: String,
  path: String,
}

/// Fetched at runtime from `data/test-index.json`, the file
/// `site/build.rs` generates from `../cases/*.ttl` and `Trunk.toml`'s
/// `pre_build` hook forces to exist before Trunk copies it into `dist/`.
async fn fetch_test_index() -> Result<Vec<CaseEntry>, String> {
  let resp = Request::get("data/test-index.json").send().await.map_err(|e| e.to_string())?;
  if !resp.ok() {
    return Err(format!("GET data/test-index.json failed (HTTP {})", resp.status()));
  }
  resp.json::<Vec<CaseEntry>>().await.map_err(|e| e.to_string())
}

#[component]
pub fn TestsPage() -> Html {
  let cases = use_state(|| None::<Result<Vec<CaseEntry>, String>>);

  {
    let cases = cases.clone();
    use_effect_with((), move |_| {
      spawn_local(async move {
        cases.set(Some(fetch_test_index().await));
      });
      || ()
    });
  }

  let body = match &*cases {
    None => html! { <Spinner /> },
    Some(Err(err)) => html! {
      <Alert r#type={AlertType::Danger} title="Could not load the test-case index" inline=true><p>{ err.clone() }</p></Alert>
    },
    Some(Ok(entries)) if entries.is_empty() => html! {
      <Alert r#type={AlertType::Info} title="No test cases yet" inline=true>
        { "cases/ is currently empty." }
      </Alert>
    },
    Some(Ok(entries)) => html! {
      <ul class="dsc-case-list">
        { for entries.iter().map(|entry| {
          let href = format!("{REPO_BLOB_BASE}/{}", entry.path);
          html! {
            <li key={entry.slug.clone()}>
              <a href={href} target="_blank">{ entry.title.clone() }</a>
              { " " }
              <code>{ entry.slug.clone() }</code>
            </li>
          }
        }) }
      </ul>
    },
  };

  html! {
    <Content>
      <Title level={Level::H1}>{ "Browse Tests" }</Title>
      <p>
        { "Every RDF/Turtle test case currently checked into " }<code>{ "cases/" }</code>
        { ", linking to its raw file on GitHub." }
      </p>
      { body }
    </Content>
  }
}
