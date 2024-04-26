use chrono::Datelike;
use leptos::{logging::log, *};
use leptos_router::*;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

use crate::{
    api::{
        posts::{create_article, delete_article, get_all_articles, update_article, PostModel},
        users::{get_user, user_logged_in, LoginUser, UserModel},
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
pub fn Office() -> impl IntoView {
    let i18n = use_i18n();

    let articles_resource = create_resource(|| (), |_| async move { get_all_articles().await });

    let toggle_writer = create_rw_signal(false);
    let editable = create_rw_signal(-1);

    let title = create_rw_signal(String::new());
    let slug = create_rw_signal(String::new());
    let short_description = create_rw_signal(String::new());
    let content = create_rw_signal(String::new());

    view! {
        <div class="flex flex-col">
        {move || match toggle_writer.get() {
            true => view! {
                <h2 class="text-3xl font-bold">"Write a article"</h2>
                <form>
                    <p class="text-left font-bold">"Title"</p>
                        <input type="text" on:input=move |ev| {
                            title.set(event_target_value(&ev));
                        } prop:value=title class="bg-orange-400 border-4 border-t-orange-900 border-l-orange-800 border-r-orange-300 border-b-orange-300 w-full flex-grow p-2 focus:outline-none"/>
                    <p class="text-left font-bold">"Slug"</p>
                    <input type="text" on:input=move |ev| {
                        slug.set(event_target_value(&ev));
                    } prop:value=slug class="bg-orange-400 border-4 border-t-orange-900 border-l-orange-800 border-r-orange-300 border-b-orange-300 w-full flex-grow p-2 focus:outline-none"/>
                    <textarea on:input=move |ev| {
                        short_description.set(event_target_value(&ev));
                    } class="bg-orange-400 border-4 border-t-orange-900 border-l-orange-800 border-r-orange-300 border-b-orange-300 w-full flex-grow p-2 focus:outline-none" placeholder="A short description..."></textarea>
                    <textarea on:input=move |ev| {
                        content.set(event_target_value(&ev));
                    } class="bg-orange-400 border-4 border-t-orange-900 border-l-orange-800 border-r-orange-300 border-b-orange-300 w-full flex-grow p-2 focus:outline-none" placeholder="Write your article..."></textarea>
                    <div class="flex my-2">
                    <button
                        on:click=move |ev| {
                            ev.prevent_default();
                            toggle_writer.set(false);
                        }
                        class="bg-orange-400 mr-2 border-4 border-b-orange-900 active:border-t-orange-900 border-r-orange-800 active:border-l-orange-900 border-l-orange-300 active:border-r-orange-300 border-t-orange-300 active:border-b-orange-300 w-full cursor-pointer py-2"
                    >
                        "Cancelar"
                    </button>
                    <button
                        on:click=move |ev| {
                            ev.prevent_default();
                            let mut uppost = PostModel {
                                title: title.get(),
                                slug: slug.get(),
                                short_desc: Some(short_description.get()),
                                content: content.get(),
                                ..Default::default()
                            };
                            spawn_local(async move {
                                let user_id = user_logged_in().await.unwrap().unwrap();
                                uppost.writer = get_user(user_id).await.unwrap().unwrap();
                                if editable.get() != -1 {
                                    uppost.id = editable.get();
                                    update_article(uppost).await;
                                } else {
                                    create_article(uppost).await;
                                }
                            });
                            articles_resource.refetch();
                            toggle_writer.set(false);
                        }
                        class="bg-orange-400 border-4 border-b-orange-900 active:border-t-orange-900 border-r-orange-800 active:border-l-orange-900 border-l-orange-300 active:border-r-orange-300 border-t-orange-300 active:border-b-orange-300 w-full cursor-pointer py-2"
                    >
                        "Create"
                    </button>
                    </div>
                </form>
            }.into_view(),
            false => view! {
                <h2 class="text-3xl font-bold">"Articles available"</h2>
                <ul class="flex-grow overflow-auto">
                <Suspense>
                {move || {
                    articles_resource.get()
                        .map(|articles| match articles {
                            Ok(articles) => articles.into_iter()
                                .map(|article| {
                                    let date = article.updated_at.clone();
                                    let day = date.day();
                                    let month = date.month();
                                    let year = date.year();
                                    let article_title = article.title.clone();
                                    let short_desc = article.short_desc.clone();
                                    view! {
                                        <li class="py-3 px-4">
                                            <RetroContainer>
                                                <div class="flex">
                                                    <A href=format!("/articles/{}", article.slug) class="hover:underline flex-grow decoration-2 underline-offset-2 decoration-white">
                                                        <h2 class="text-2xl text-white font-bold my-1">{article_title}</h2>
                                                    </A>
                                                    <button on:click=move |_| {
                                                        editable.set(article.id);
                                                        slug.set(article.slug.clone());
                                                        short_description.set(article.short_desc.clone().unwrap_or_default());
                                                        title.set(article.title.clone());
                                                        toggle_writer.set(true);
                                                    } class="p-2">"Edit"</button>
                                                    <button on:click=move |_| {
                                                        spawn_local(async move {
                                                            delete_article(article.id).await;
                                                        });
                                                        articles_resource.refetch()
                                                    } class="p-2">"Delete"</button>
                                                </div>
                                                <hr class="border-t-2"/>

                                                <p class="text-[#630000] mb-2">{t!(i18n, posts.written)}" "{t!(i18n, common.date, day = day, count = move || month as i32, year = year)}</p>
                                                <p class="mb-3">{short_desc}</p>
                                            </RetroContainer>
                                        </li>
                                    }
                                }).collect_view(),
                            Err(msg) => view! {
                                <RetroContainer>
                                    {t!(i18n, home.post_error, msg = msg.to_string())}
                                </RetroContainer>
                            }
                        })
                }}
                </Suspense>
                </ul>
                <button
                    on:click=move |_| {
                        toggle_writer.set(true)
                    }
                    class="bg-orange-400 border-4 border-b-orange-900 active:border-t-orange-900 border-r-orange-800 active:border-l-orange-900 border-l-orange-300 active:border-r-orange-300 border-t-orange-300 active:border-b-orange-300 w-full cursor-pointer py-2"
                >
                    "Create new article"
                </button>
            }.into_view()
        }}
        </div>

    }
}
