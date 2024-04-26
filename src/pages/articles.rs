pub mod handler;
pub mod list;

use chrono::Datelike;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use web_sys::{FormData, HtmlFormElement};

use crate::components::navbar::NavbarReserved;
use crate::components::{background::Brickwall, container::RetroContainer, navbar::Navbar};
use crate::i18n::*;
use list::ArticleList;

/// The home page component, renders on / default path
#[component]
pub fn Articles() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <Title text="Articles"/>
        <Brickwall>
            <NavbarReserved/>
            <div class="p-4 px-8">
                <ArticleList/>
            </div>
        </Brickwall>
    }
}
