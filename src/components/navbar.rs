use leptos::*;
use leptos_router::*;
use web_sys::{FormData, HtmlFormElement};

use crate::{
    api::images::upload_image_to_gallery,
    components::{background::Brickwall, container::RetroContainer},
};

#[component]
pub fn NavButton<H: ToHref + 'static>(children: Children, href: H) -> impl IntoView {
    view! {
        <li class="inline mx-4"><A class="hover:underline decoration-2 underline-offset-2 decoration-[#790000] hover:text-[#630000] hover:font-bold" href=href>{children()}</A></li>
    }
}

#[component]
pub fn NavButtonUnavailable<H: ToHref + 'static>(children: Children, href: H) -> impl IntoView {
    view! {
        <li class="inline mx-4"><span class="decoration-2 decoration-[#790000] hover:text-[#630000] line-through hover:font-bold">{children()}</span></li>
    }
}

#[component]
pub fn NavbarReserved() -> impl IntoView {
    view! {
        <div class="h-16"></div>
    }
}

/// The home page component, renders on / default path
#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="fixed w-full flex justify-center items-center">
            <div class="w-11/12 top-1">
                <RetroContainer class="flex">
                    <p class="flex-grow">"Orangethewell"</p>
                    <ul class="inline">
                        <NavButton href="/">"Home"</NavButton>
                        <NavButton href="/articles">"Posts"</NavButton>
                        <NavButtonUnavailable href="/projects">"Projects"</NavButtonUnavailable>
                        <NavButtonUnavailable href="/about-me">"About me"</NavButtonUnavailable>
                    </ul>
                </RetroContainer>
            </div>
        </nav>
    }
}
