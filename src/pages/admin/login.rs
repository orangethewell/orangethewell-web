use leptos::{logging::log, *};
use leptos_meta::*;
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
pub fn Login() -> impl IntoView {
    let i18n = use_i18n();
    let login = create_server_action::<LoginUser>();
    let on_submit = move |ev: SubmitEvent| {};

    view! {
        <Title text="Login"/>
        <Brickwall>
            <div class="flex justify-center min-h-screen items-center">
                <RetroContainer>
                    <ActionForm on:submit=on_submit action=login>
                        <h1 class="text-3xl font-bold">"Login"</h1>
                        <hr class="border-t-2 my-4"/>
                        <div class="mb-4">
                            <p class="text-left font-bold">"Email"</p>
                            <input
                                id="email"
                                name="email"
                                class="bg-orange-400 border-4 border-t-orange-900 border-l-orange-800 border-r-orange-300 border-b-orange-300 w-full flex-grow p-2 focus:outline-none "
                                type="email"
                            />
                        </div>
                        <div class="mb-4">
                            <p class="text-left font-bold">"Senha"</p>
                            <input
                                id="password"
                                name="password"
                                class="bg-orange-400 border-4 border-t-orange-900 border-l-orange-800 border-r-orange-300 border-b-orange-300 w-full flex-grow p-2 focus:outline-none"
                                type="password"
                            />
                        </div>
                        <button
                            class="bg-orange-400 border-4 border-b-orange-900 active:border-t-orange-900 border-r-orange-800 active:border-l-orange-900 border-l-orange-300 active:border-r-orange-300 border-t-orange-300 active:border-b-orange-300 w-full cursor-pointer py-2"
                        >
                            "Login"
                        </button>
                        <p>
                            "Esqueceu sua senha de acesso? "
                            <A class="text-blue-600" href="/login/recuperar-senha">
                                "Clique aqui."
                            </A>
                        </p>
                    </ActionForm>
                </RetroContainer>
            </div>
        </Brickwall>
    }
}
