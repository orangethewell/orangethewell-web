use leptos::{logging::log, *};
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};

use crate::{
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
pub fn Index() -> impl IntoView {
    let i18n = use_i18n();

    let img_src = create_rw_signal("/spr_player_down.png");

    view! {
        <Title text="Home"/>
        <Brickwall>
            <NavbarReserved/>
            <div class="flex md:flex-row flex-col py-2 px-9">
                <section class="flex-grow md:mr-8">
                    <RetroContainer class="flex items-center">
                        <hr class="border-t-2 mx-2 border-t-black flex-grow"/>
                        <p class="">{t!(i18n, home.recent_post)}</p>
                        <hr class="border-t-2 mx-2 border-t-black flex-grow"/>
                    </RetroContainer>
                    <ArticleList/>
                </section>
                <div class="my-8">
                    <hr class="md:hidden border-t-white border-t-4 border-dashed"/>
                    <hr class="md:hidden mt-[2px] ml-[2px] border-t-[#000000aa] border-t-4 border-dashed"/>
                </div>
                <aside>
                    <RetroContainer class="min-w-60 max-sm:flex flex-col justify-center items-center">
                        <p class="font-extrabold text-2xl max-sm:mb-3">{t!(i18n, home.contact)}</p>
                        <ul class="mx-4 my-2">
                            <li><a class="hover:invert text-black hover:underline decoration-2 underline-offset-2 decoration-black" href="https://www.instagram.com/orangethewell/"><img class="h-8 w-auto m-1 inline-block" src="/instagram.svg"/>"Instagram"</a></li>
                            <li><a class="hover:invert inline-block text-black hover:underline decoration-2 underline-offset-2 decoration-black" href="mailto:orangethewell@gmail.com"><img class="h-8 w-auto m-1 inline-block" src="/email.svg"/>"E-mail"</a></li>
                        </ul>
                    </RetroContainer>
                </aside>
            </div>
            <a class="fixed max-sm:hidden right-[4%] -bottom-10" target="_blank" href="/Stellarbonds.html"><img style="image-rendering: pixelated;" class="h-24 w-auto" on:mouseover=move |_| {
                img_src.set("/jadepixel_walking.gif");
            } on:mouseout=move |_| {
                img_src.set("/spr_player_down.png");
            } src={img_src}/></a>
        </Brickwall>
    }
}
