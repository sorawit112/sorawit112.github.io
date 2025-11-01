use crate::components::*;
use leptos::prelude::*;
// use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <link data-trunk href="./style/output.css" rel="css" />
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="font-sans antialiased bg-lightgray min-h-screen text-text">
            <header::Header/>
            <main>
                <about::About/>
                <experience::Experience/>
                <education::Education/>
                <skills::Skills/>
                <projects::Projects/>
            </main>
            <footer::Footer/>
        </div>
    }
}
