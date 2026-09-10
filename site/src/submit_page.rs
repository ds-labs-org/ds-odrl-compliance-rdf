use patternfly_yew::prelude::*;
use yew::prelude::*;

/// A deliberate, disclosed placeholder -- no OAuth/PKCE/GitHub API code
/// here. Automated PR submission via GitHub sign-in is planned (reusing
/// the Authorization Code + PKCE pattern the `dataspace` repo's
/// `site/src/authority_registration.rs` already implements for its own
/// registry-submission flow) but not yet built, pending a decision on
/// OAuth App registration for this repo specifically. See that file for
/// the pattern this page's future form would follow: PKCE (S256) plus a
/// same-origin relay for the token exchange, then a direct
/// `api.github.com` call to fork/branch/commit/open-PR.
#[component]
pub fn SubmitPage() -> Html {
  html! {
    <Content>
      <Title level={Level::H1}>{ "Submit a test case" }</Title>
      <Alert r#type={AlertType::Info} title="Not built yet" inline=true>
        { "A form that signs you in with GitHub and opens the PR for you is planned, but does not exist yet -- \
           use the manual path below in the meantime." }
      </Alert>

      <Title level={Level::H2}>{ "Manual path (works today)" }</Title>
      <ol>
        <li>{ "Fork " }<code>{ "ds-labs-org/ds-odrl-compliance-rdf" }</code>{ "." }</li>
        <li>
          { "Add one new " }<code>{ ".ttl" }</code>{ " file under " }<code>{ "cases/" }</code>
          { ", named " }<code>{ "<slug>.ttl" }</code>{ " (see " }
          <a href="https://github.com/ds-labs-org/ds-odrl-compliance-rdf/blob/main/docs/vocabulary-spec.md" target="_blank">
            { "docs/vocabulary-spec.md" }
          </a>
          { " section 4 for the required file shape -- the " }<code>{ "@base" }</code>{ "/" }
          <code>{ "@prefix : <#>" }</code>{ " pattern, the " }<code>{ "dsc:TestCase" }</code>{ "/" }
          <code>{ "dsc:Request" }</code>{ " split, and the claim shapes)." }
        </li>
        <li>{ "Open a pull request against " }<code>{ "main" }</code>{ "." }</li>
      </ol>

      <Title level={Level::H2}>{ "Planned: sign-in-and-submit" }</Title>
      <p>
        { "The plan is a browser-side form -- fill in a request, its policies, and the expected outcome -- that \
           signs the contributor in with GitHub (Authorization Code + PKCE, the same pattern " }
        <code>{ "dataspace" }</code>{ "'s " }<code>{ "site/src/authority_registration.rs" }</code>
        { " already uses for its own registry-submission flow), then forks this repo, commits the new file, and \
           opens the pull request via the GitHub API on the contributor's behalf. It is not built yet: it is \
           blocked on registering a GitHub OAuth App for this repo and deciding where the token-exchange relay \
           lives (that flow's own token exchange needs a small server-side relay to inject the OAuth app's client \
           secret, exactly as the linked file's own header comment explains for its case)." }
      </p>
    </Content>
  }
}
