use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use leptos::{logging::log, *};
use leptos_router::*;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

use crate::{
    api::{
        images::{get_image, get_image_len, upload_image_to_gallery, GetImage},
        users::{LoginUser, UserModel},
    },
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
pub fn Gallery() -> impl IntoView {
    let i18n = use_i18n();

    let image_count = create_resource(|| (), |_| async move { get_image_len().await });
    let upload_action = create_action(|data: &FormData| {
        let data = data.clone();
        upload_image_to_gallery(data.into())
    });

    view! {
        <div class="flex flex-col">
        <h2 class="text-3xl text-center py-2 font-bold">"Gallery"</h2>
        <div class="grid flex-grow grid-cols-2 md:grid-cols-3 overflow-auto gap-4">
            <Suspense>
            {
                match image_count.get() {
                    Some(Ok(count)) => (1..count + 1).into_iter()
                        .map(|id| view!{<img class="hover:border-4 w-full object-cover aspect-square duration-300 transition" src={format!("/gallery/{}", id)}/>})
                        .collect_view(),

                    _ => view! {
                        <>
                        <pre>"A error occured."</pre>
                        </>
                    }.into()
                }
            }
            </Suspense>
        </div>
        <form on:submit=move |ev: SubmitEvent| {
            ev.prevent_default();
            let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
            let form_data = FormData::new_with_form(&target).unwrap();
            upload_action.dispatch(form_data);
        }>
        <input name="file_to_upload" type="file"/>
        <button>"Enviar"</button>
        </form>
        </div>
    }
}
