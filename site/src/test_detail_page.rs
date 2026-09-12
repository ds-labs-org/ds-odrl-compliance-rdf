use gloo_net::http::Request;
use patternfly_yew::prelude::*;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_nested_router::components::Link;

use crate::app_route::AppRoute;

const REPO_BLOB_BASE: &str = "https://github.com/ds-labs-org/ds-odrl-compliance-rdf/blob/main";

/// Mirrors one entry of `site/build.rs`'s generated `test-index.json` --
/// see `src/tests_page.rs` for the same shape.
#[derive(Debug, Clone, Deserialize)]
struct CaseEntry {
  slug: String,
  title: String,
  path: String,
}

async fn fetch_test_index() -> Result<Vec<CaseEntry>, String> {
  let resp = Request::get("data/test-index.json").send().await.map_err(|e| e.to_string())?;
  if !resp.ok() {
    return Err(format!("GET data/test-index.json failed (HTTP {})", resp.status()));
  }
  resp.json::<Vec<CaseEntry>>().await.map_err(|e| e.to_string())
}

/// Fetched at runtime from `cases/<slug>.ttl` -- landed under `dist/cases/`
/// by `index.html`'s `copy-dir` directive for `../cases`, mirroring the
/// `copy-file` directives already used for `ns.ttl` and `test-index.json`.
async fn fetch_case_ttl(path: &str) -> Result<String, String> {
  let resp = Request::get(path).send().await.map_err(|e| e.to_string())?;
  if !resp.ok() {
    return Err(format!("GET {path} failed (HTTP {})", resp.status()));
  }
  resp.text().await.map_err(|e| e.to_string())
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TestDetailPageProps {
  pub slug: String,
}

#[component]
pub fn TestDetailPage(props: &TestDetailPageProps) -> Html {
  let slug = props.slug.clone();

  // (title, path) for this slug, once the index has loaded -- None while
  // loading, Some(Err) if the index failed or the slug isn't in it.
  let entry = use_state(|| None::<Result<CaseEntry, String>>);
  {
    let entry = entry.clone();
    let slug = slug.clone();
    use_effect_with(slug.clone(), move |slug| {
      let slug = slug.clone();
      spawn_local(async move {
        let result = match fetch_test_index().await {
          Ok(entries) => entries
            .into_iter()
            .find(|e| e.slug == slug)
            .ok_or_else(|| format!("No test case with slug \"{slug}\" in the index")),
          Err(err) => Err(err),
        };
        entry.set(Some(result));
      });
      || ()
    });
  }

  let ttl = use_state(|| None::<Result<String, String>>);
  {
    let ttl = ttl.clone();
    let path = match &*entry {
      Some(Ok(e)) => Some(e.path.clone()),
      _ => None,
    };
    use_effect_with(path, move |path| {
      if let Some(path) = path.clone() {
        spawn_local(async move {
          ttl.set(Some(fetch_case_ttl(&path).await));
        });
      }
      || ()
    });
  }

  let body = match &*entry {
    None => html! { <Spinner /> },
    Some(Err(err)) => html! {
      <Alert r#type={AlertType::Danger} title="Could not find this test case" inline=true><p>{ err.clone() }</p></Alert>
    },
    Some(Ok(case)) => {
      let raw_href = format!("{REPO_BLOB_BASE}/{}", case.path);
      let ttl_block = match &*ttl {
        None => html! { <Spinner /> },
        Some(Ok(text)) => html! { <pre class="dsc-ttl"><code>{ text.clone() }</code></pre> },
        Some(Err(err)) => html! {
          <Alert r#type={AlertType::Danger} title="Could not load the raw .ttl file" inline=true><p>{ err.clone() }</p></Alert>
        },
      };
      html! {
        <>
          <Title level={Level::H1}>{ case.title.clone() }</Title>
          <p>
            <code>{ case.slug.clone() }</code>
            { " -- " }
            <a href={raw_href} target="_blank">{ "View raw on GitHub" }</a>
          </p>
          { ttl_block }
        </>
      }
    }
  };

  html! {
    <Content>
      <p>
        <Link<AppRoute> to={AppRoute::Tests}>{ "<- Back to Browse Tests" }</Link<AppRoute>>
      </p>
      { body }
    </Content>
  }
}
