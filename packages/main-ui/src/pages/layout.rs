#![allow(non_snake_case)]
use crate::pages::i18n::RootLayoutTranslate;
use crate::route::Route;
use by_components::loaders::cube_loader::CubeLoader;
use by_components::meta::MetaSeoTemplate;
use by_components::theme::ColorTheme;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    let theme: ColorTheme = use_context();
    let path: Route = use_route();
    let tr: RootLayoutTranslate = translate(&lang);

    rsx! {
        MetaSeoTemplate {
            title: "{tr.title}",
            canonical: "https://dunning.im",
            keywords: "Dunning",
            url: "{path}",
        }

        div {
            class: "flex flex-col w-full items-center justify-start min-h-[100vh] text-white",
            background: "{theme.background}",
            color: "{theme.text.primary}",
            header { class: "w-full min-h-[70px] flex flex-row items-center justify-start max-[500px]:justify-center max-w-[1440px] max-[1440px]:px-[20px]",
                Link {
                    class: "flex items-center justify-center h-[70px]",
                    to: Route::HomePage { lang },
                    p { class: "logo text-[40px] font-black", "{tr.title}" }
                }
            }
            div {
                class: "w-full max-[1440px]:px-[20px]",
                min_height: "calc(100vh - 190px)",
                SuspenseBoundary {
                    fallback: |_| rsx! {
                        div { class: "absolute left-0 top-0 w-full h-full flex items-center justify-center",
                            CubeLoader {}
                        }
                    },

                    Outlet::<Route> {}
                }
            }
            div { class: "min-h-[70px] h-[70px]" }
        }
    }
}
