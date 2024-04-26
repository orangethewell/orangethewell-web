use leptos::{logging::log, *};
use leptos_router::*;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

use crate::{
    api::users::{get_user, update_user, user_logged_in, LoginUser, UserModel},
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
pub fn Identity() -> impl IntoView {
    let i18n = use_i18n();

    let id = create_rw_signal(0);
    let name = create_rw_signal(String::new());
    let mail = create_rw_signal(String::new());
    let pass = create_rw_signal(String::new());

    // user_logged_in().await.unwrap().unwrap()
    let user = create_resource(
        || (),
        move |_| async move {
            let user_id = user_logged_in().await.unwrap().unwrap();
            let user = get_user(user_id).await.unwrap().unwrap();
            let id = id.clone();
            let name = name.clone();
            let mail = mail.clone();
            id.set(user_id);
            name.set(user.username);
            mail.set(user.email);
        },
    );

    view! {
        <div class="flex">
            <Suspense>
                <form autocomplete="off">
                    <p class="text-left font-bold">"Nome"</p>
                    <input type="text" on:input=move |ev| {
                        name.set(event_target_value(&ev));
                    } prop:value=name class="bg-orange-400 border-4 border-t-orange-900 border-l-orange-800 border-r-orange-300 border-b-orange-300 w-full flex-grow p-2 focus:outline-none" type="text"/>
                    <p class="text-left font-bold">"Email"</p>
                    <input type="email" on:input=move |ev| {
                        mail.set(event_target_value(&ev));
                    } prop:value=mail class="bg-orange-400 border-4 border-t-orange-900 border-l-orange-800 border-r-orange-300 border-b-orange-300 w-full flex-grow p-2 focus:outline-none" type="email"/>
                    <p class="text-left font-bold">"Senha"</p>
                    <input type="password" on:input=move |ev| {
                        pass.set(event_target_value(&ev));
                    } prop:value="" class="bg-orange-400 border-4 border-t-orange-900 border-l-orange-800 border-r-orange-300 border-b-orange-300 w-full flex-grow p-2 focus:outline-none" type="password"/>
                    <div class="flex my-2">
                    <button
                        on:click=move |ev| {
                            ev.prevent_default();
                            user.refetch()
                        }
                        class="bg-orange-400 mr-2 border-4 border-b-orange-900 active:border-t-orange-900 border-r-orange-800 active:border-l-orange-900 border-l-orange-300 active:border-r-orange-300 border-t-orange-300 active:border-b-orange-300 w-full cursor-pointer py-2"
                    >
                        "Cancelar"
                    </button>
                    <button
                        on:click=move |ev| {
                            ev.prevent_default();
                            let upuser = UserModel {
                                id: id.get(),
                                username: name.get(),
                                email: mail.get(),
                                password: pass.get(),
                                ..Default::default()
                            };
                            spawn_local(async move {
                                update_user(upuser).await;
                            });
                        }
                        class="bg-orange-400 border-4 border-b-orange-900 active:border-t-orange-900 border-r-orange-800 active:border-l-orange-900 border-l-orange-300 active:border-r-orange-300 border-t-orange-300 active:border-b-orange-300 w-full cursor-pointer py-2"
                    >
                        "Atualizar"
                    </button>
                    </div>
                </form>
                {user.refetch()}
            </Suspense>
        </div>
    }
}
