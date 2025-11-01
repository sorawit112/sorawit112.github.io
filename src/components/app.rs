use crate::components::*;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
            </Routes>
        </Router>
    }
}

fn Home() -> impl IntoView {
    let (value, set_value) = signal(0);

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
