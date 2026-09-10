use crate::app_route::AppRoute;
use crate::switch_app_route::switch_app_route;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::prelude::Switch as RouterSwitch;

/// The page shell: masthead brand and a sidebar nav linking the four v1
/// routes (Home/Vocabulary/Tests/Submit), plus a router `Switch` plugging
/// in whichever is current. Structurally mirrors the ds-odrl-engine-rs and
/// dataspace sites' own `MainView` (Masthead/Page/Nav via patternfly-yew),
/// not their page content -- this repo has its own small four-page shape.
#[component]
pub fn MainView() -> Html {
  let brand = html!(
    <Title level={Level::H3} size={Size::XLarge}>{ "ds-odrl-compliance-rdf" }</Title>
  );

  let sidebar = html_nested!(
    <PageSidebar>
      <Nav>
        <NavList>
          <NavRouterItem<AppRoute> to={AppRoute::Home}>{ "Home" }</NavRouterItem<AppRoute>>
          <NavRouterItem<AppRoute> to={AppRoute::Vocabulary}>{ "Vocabulary" }</NavRouterItem<AppRoute>>
          <NavRouterItem<AppRoute> to={AppRoute::Tests}>{ "Browse Tests" }</NavRouterItem<AppRoute>>
          <NavRouterItem<AppRoute> to={AppRoute::Submit}>{ "Submit" }</NavRouterItem<AppRoute>>
        </NavList>
      </Nav>
    </PageSidebar>
  );

  html!(
    <Page {brand} {sidebar} full_height=true>
      <PageSection>
        <RouterSwitch<AppRoute> render={switch_app_route} />
      </PageSection>
    </Page>
  )
}
