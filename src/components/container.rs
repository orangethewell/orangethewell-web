use leptos::*;

#[component]
pub fn RetroContainer(
    children: Children,
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class=&format!("shadow-[#000000aa_8px_8px] rounded-[18px] border-solid border-transparent border-[12px] [border-image:url('/border.png')_8_stretch] bg-[#ff9b37] {}", class.unwrap_or_default())>{children()}</div>
    }
}
