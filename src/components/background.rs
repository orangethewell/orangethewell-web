use leptos::*;

/// A Brickwall styled main tag.
/// TODO: Add theme support
#[component]
pub fn Brickwall(children: Children) -> impl IntoView {
    view! {

        <main style="image-rendering: pixelated;" class="bg-[url('/brickwall.png')] bg-repeat bg-[length:96px] min-h-screen">
            {children()}
        </main>
    }
}
