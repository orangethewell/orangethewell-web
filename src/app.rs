use crate::api::users::get_user;
use crate::components::navbar::Navbar;
use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::admin::dashboard::Dashboard;
use crate::pages::admin::gallery::Gallery;
use crate::pages::admin::identity::Identity;
use crate::pages::admin::office::Office;
use crate::pages::articles::handler::ArticleLoader;
use crate::pages::articles::Articles;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::i18n::provide_i18n_context;
use crate::pages::{admin::login::Login, AboutMe, Index};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_i18n_context();

    let formatter = |text| format!("{text} — Orange Museum");

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/orangethewell-web.css"/>
        <Stylesheet id="minecraft" href="/webfont-kit/stylesheet.css"/>
        <Stylesheet id="dotgothic16" href="https://fonts.googleapis.com/css2?family=DotGothic16&display=swap"/>

        // sets the document title
        <Title formatter/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="/" view=move || {
                        view! {
                            <Navbar/>
                            <Outlet/>
                        }
                    }>
                        <Route path="/" view=Index/>
                        <Route path="/articles" view=Articles/>
                        <Route path="/articles/:slug" view=ArticleLoader/>
                    </Route>
                    <Route path="/admin" view=move || {
                        view! {
                            <Outlet/>
                        }
                    }>
                        <Route path="/" view=Login/>
                        <Route path="/dashboard" view=Dashboard>
                            <Route path="/" view=move || {
                                view! {
                                    <p>"Selecione um menu para começar"</p>
                                }
                            }/>
                            <Route path="/office" view=Office/>
                            <Route path="/gallery" view=Gallery/>
                            <Route path="/identity" view=Identity/>
                        </Route>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}
