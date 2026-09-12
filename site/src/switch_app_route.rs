use crate::app_route::AppRoute;
use crate::home_page::HomePage;
use crate::submit_page::SubmitPage;
use crate::test_detail_page::TestDetailPage;
use crate::tests_page::TestsPage;
use crate::vocabulary_page::VocabularyPage;
use yew::{Html, html};

pub fn switch_app_route(target: AppRoute) -> Html {
  match target {
    AppRoute::Home => html! { <HomePage /> },
    AppRoute::Vocabulary => html! { <VocabularyPage /> },
    AppRoute::Tests => html! { <TestsPage /> },
    AppRoute::TestDetail { slug } => html! { <TestDetailPage slug={slug} /> },
    AppRoute::Submit => html! { <SubmitPage /> },
  }
}
