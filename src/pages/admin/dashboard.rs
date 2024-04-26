use leptos::{logging::log, *};
use leptos_router::*;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

use crate::{
    api::users::{LoginUser, UserModel},
    components::{
        background::Brickwall,
        container::RetroContainer,
        navbar::{Navbar, NavbarReserved},
    },
    pages::articles::list::ArticleList,
};

use crate::i18n::*;

/// The home page component, renders on / default path
#[component]
pub fn Dashboard() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <Brickwall>
            <div class="flex md:p-4 min-h-screen">
                <RetroContainer class="px-4 mr-4">
                    <ul>
                        <li><A href="/admin/dashboard/office"><img class="h-16 w-auto m-1 inline-block" src="/writer.svg"/>"Escritório"</A></li>
                        <li><A href="/admin/dashboard/gallery"><img class="h-16 w-auto m-1 inline-block" src="/gallery.svg"/>"Galeria"</A></li>
                        <li><A href="/admin/dashboard/identity"><img class="h-16 w-auto m-1 inline-block" src="/identity.svg"/>"Identidade"</A></li>
                    </ul>
                </RetroContainer>
                <RetroContainer class="flex-1 flex">
                    <Outlet/>
                </RetroContainer>
            </div>
        </Brickwall>
    }
}
