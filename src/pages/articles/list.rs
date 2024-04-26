use chrono::Datelike;
use leptos::*;
use leptos_router::*;
use web_sys::{FormData, HtmlFormElement};

use crate::{
    api::posts::get_all_articles,
    components::{background::Brickwall, container::RetroContainer, navbar::Navbar},
};

use crate::i18n::*;

/// The home page component, renders on / default path
#[component]
pub fn ArticleList() -> impl IntoView {
    let i18n = use_i18n();

    let articles = create_resource(|| (), |_| async move { get_all_articles().await });

    view! {
        <ul class="pt-3">
        <Suspense>
        {move || {
            articles.get()
                .map(|articles| match articles {
                    Ok(articles) => articles.into_iter()
                        .map(|article| {
                            let date = article.updated_at.clone();
                            let day = date.day();
                            let month = date.month();
                            let year = date.year();
                            view! {
                                <li class="py-3">
                                    <RetroContainer>
                                        <A href=format!("/articles/{}", article.slug) class="hover:underline decoration-2 underline-offset-2 decoration-white"><h2 class="text-2xl text-white font-bold my-1">{article.title}</h2></A>
                                        <hr class="border-t-2"/>

                                        <p class="text-[#630000] mb-2">{t!(i18n, posts.written)}" "{t!(i18n, common.date, day = day, count = move || month as i32, year = year)}</p>
                                        <p class="mb-3">{article.short_desc}</p>
                                        <A href=format!("/articles/{}", article.slug) class="hover:underline decoration-2 underline-offset-2 decoration-[#630000]"><p class="text-center text-[#630000]">{t!(i18n, posts.read_more)}</p></A>
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
    }
}
