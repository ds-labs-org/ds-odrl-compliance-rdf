use yew_nested_router::Target;

/// This site's routing shell for v1: four flat top-level routes. Kept
/// flat like the sibling ds-odrl-engine-rs site's own `AppRoute` -- no
/// nested doc-tree routes, since this v1 site has no embedded docs corpus
/// to browse (the spec doc and vocabulary.spec.md link out to GitHub
/// instead).
///
/// No route name here may also be the name of a directory Trunk creates
/// under `dist/` -- see `index.html`'s comment on why the generated test
/// index lands under `dist/data/` rather than `dist/tests/`, which would
/// collide with the `Tests` route below.
#[derive(Debug, Clone, PartialEq, Target, Eq)]
pub enum AppRoute {
  #[target(rename = "")]
  Home,
  #[target(rename = "vocabulary")]
  Vocabulary,
  #[target(rename = "tests")]
  Tests,
  #[target(rename = "submit")]
  Submit,
}
