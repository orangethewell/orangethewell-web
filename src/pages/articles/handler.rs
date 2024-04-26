use chrono::{DateTime, Datelike, FixedOffset};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use web_sys::{FormData, HtmlFormElement};

use crate::{
    api::posts::{get_article, PostModel},
    components::{
        background::Brickwall,
        container::RetroContainer,
        navbar::{Navbar, NavbarReserved},
    },
};

use crate::i18n::*;

#[derive(Params, PartialEq)]
pub struct ArticleParams {
    slug: String,
}

#[component]
pub fn ArticleReader(
    title: String,
    date: DateTime<FixedOffset>,
    writer: String,
    content: String,
) -> impl IntoView {
    let i18n = use_i18n();
    let day = date.day();
    let month = date.month();
    let year = date.year();

    let parser = pulldown_cmark::Parser::new(&content);
    let mut inner_html = String::new();
    pulldown_cmark::html::push_html(&mut inner_html, parser);

    view! {
        <Brickwall>
            <NavbarReserved/>
            <div class="p-4 px-8">
            <RetroContainer>
                <h1 class="text-4xl text-white font-bold my-1">{title.clone()}</h1>
                <Title text={title}/>
                <hr class="border-t-2"/>

                <p class="text-[#630000] mb-2">{t!(i18n, posts.written)}" "{t!(i18n, common.date, day = day, count = move || month as i32, year = year)}</p>
                <div class="mb-3 md-content-area" inner_html=inner_html></div>
            </RetroContainer>
            </div>
        </Brickwall>
    }
}
/// The home page component, renders on / default path
#[component]
pub fn ArticleLoader() -> impl IntoView {
    let i18n = use_i18n();
    let params = use_params::<ArticleParams>();

    let slug = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| params.slug.clone())
                .unwrap_or_default()
        })
    };

    let article = create_resource(slug, get_article);

    view! {
        <Suspense>
        {move || {
            article.get()
                .map(|data| match data {
                    Ok(article_exists) => {
                        match article_exists {
                            Some(article) => view!{
                                <ArticleReader title=article.title.clone() date=article.updated_at.clone() writer=article.writer.username.clone() content=article.content.clone()/>
                            },
                            None => view! {<p>"ops"</p>}.into_view()
                        }
                    },
                    Err(_) => view! {
                        <p>"ops"</p>
                    }.into_view()
                } )
        }}

        </Suspense>
    }
}
