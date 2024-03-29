use std::collections::BTreeMap;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="プログラミング用語辞典"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <Routes>
                // <Route path="/" view=Home/>
                <Route path="/*" view=Home/>
            </Routes>
        </Router>
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Vocabulary {
    pub dictionary: BTreeMap<String, Word>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Word {
    pub disabled: Option<bool>,
    pub english: Option<String>,
    pub category: Option<Vec<String>>,
    pub see_also: Option<Vec<String>>,
    pub synonym: Option<Vec<String>>,
    pub antonym: Option<Vec<String>>,
    pub infinitive: Option<String>,
    pub singular: Option<String>,
    pub plural: Option<String>,
    pub shortened: Option<Vec<String>>,
    pub unshortened: Option<Vec<String>>,
    pub kana: Option<Vec<String>>,
    pub japanese: Option<Vec<String>>,
    pub description: Option<String>,
}

impl Word {
    pub fn disabled(&self) -> bool {
        self.disabled.unwrap_or_default()
    }

    pub fn enabled(&self) -> bool {
        !self.disabled()
    }
}