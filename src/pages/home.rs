use std::collections::BTreeMap;
use crate::components::counter_btn::Button;
use leptos::*;
use crate::{Vocabulary, Word};

/// Default Home Page
#[component]
pub fn Home2() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                <picture>
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <h1>"Welcome to Leptos"</h1>

                <div class="buttons">
                    <Button/>
                    <Button increment=5/>
                </div>

            </div>
        </ErrorBoundary>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    use leptos::html::*;

    /*
    let text = match create_resource(|| (), |_| async move {
        reqwest::get("../public/dict.toml").await.unwrap().text().await.unwrap()
    }).get() {
        None => return view!(<p>"Loading..."</p>).into_view(),
        Some(s) => s,
    };*/
    let text = include_str!("../../public/dict.min.toml");

    let data = toml::from_str::<Vocabulary>(&text).map_err(|e| panic!("{}", e)).unwrap();

    data.dictionary
        .iter()
        .filter(|(key, word)| !word.disabled.unwrap_or(false))
        .map(|(key, word)| {
            view! {
                <div class="word">
                    <h2>{key}</h2>
                    {word.to_view()}
                </div>
            }.into_view()
        }).collect_view()
}

impl Word {
    fn to_view(&self) -> View {
        let Word {
            english,
            category,
            antonym,
            infinitive,
            singular,
            plural,
            shortened,
            unshortened,
            kana,
            japanese,
            description,
            ..
        } = self;
        std::iter::empty::<Option<View>>()
            .chain(std::iter::once(
                english.as_ref().map(|en| {
                    view!(<p>"　　英語: "{en}</p>).into_view()
                })
            ))
            .chain(std::iter::once(
                category.as_ref().map(|ca| {
                    let cas = ca.join(", ").to_string();
                    view!(<p>"カテゴリ: "{cas}</p>).into_view()
                })
            ))
            .chain(std::iter::once(
                antonym.as_ref().map(|an| {
                    let ans = an.join(", ").to_string();
                    view!(<p>"　対義語: "{ans}</p>).into_view()
                })
            ))
            .chain(std::iter::once(
                infinitive.as_ref().map(|inf| {
                    view!(<p>"　不定詞: "{inf}</p>).into_view()
                })
            ))
            .chain(std::iter::once(
                singular.as_ref().map(|si| {
                    view!(<p>"　単数形: "{si}</p>).into_view()
                })
            ))
            .chain(std::iter::once(
                plural.as_ref().map(|pl| {
                    view!(<p>"　複数形: "{pl}</p>).into_view()
                })
            ))
            .chain(std::iter::once(
                shortened.as_ref().map(|sh| {
                    let shs = sh.join(", ").to_string();
                    view!(<p>"　省略形: "{shs}</p>).into_view()
                })
            ))
            .chain(std::iter::once(
                unshortened.as_ref().map(|un| {
                    let uns = un.join(", ").to_string();
                    view!(<p>"非省略形: "{uns}</p>).into_view()
                })
            ))
            .chain(std::iter::once(
                kana.as_ref().map(|ka| {
                    let kas = ka.join(", ").to_string();
                    view!(<p>"　　仮名: "{kas}</p>).into_view()
                })
            ))
            .chain(std::iter::once(
                japanese.as_ref().map(|ja| {
                    let jas = ja.join(", ").to_string();
                    view!(<p>"　　対訳: "{jas}</p>).into_view()
                })
            ))
            .chain(std::iter::once(
                description.as_ref().map(|de| {
                    view!(<p>{de}</p>).into_view()
                })
            ))
            .filter_map(|v| v)
            .collect_view()
    }
}
