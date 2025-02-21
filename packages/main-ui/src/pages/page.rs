use super::i18n::*;
use by_components::meta::MetaPage;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn HomePage(lang: Language) -> Element {
    let tr: HomeTranslate = translate(&lang);

    rsx! {
        MetaPage { title: "{tr.title}", description: "{tr.description}" }
        div { id: "home-page", class: "flex flex-col items-center gap-[45px]",
            div { class: "w-full uppercase text-[192px] font-black h-[40vh] flex flex-col items-center justify-center max-[1200px]:text-[96px] max-[700px]:text-[64px]",
                div { class: "w-full flex flex-row items-center justify-center slogan",
                    "{tr.slogan_run}"
                }
                div { class: "w-full flex flex-row items-center justify-center slogan",
                    "{tr.slogan_earn}"
                }
                div { class: "w-full flex flex-row items-center justify-center text-center slogan",
                    "{tr.slogan_give}"
                }
            }

            div { class: "flex flex-col gap-[10px] text-[24px] max-[700px]:text-[16px] items-center justify-center h-[40vh] text-center p-[20px]",
                "{tr.description}"

            }
            ImageSection { class: "w-full center", lang }
        }
    }
}

#[component]
pub fn ImageSection(
    lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let tr: ImageSectionTranslate = translate(&lang);
    let mut mobile = use_signal(|| false);

    rsx! {
        div {..attributes,
            div {
                class: "w-full flex flex-col gap-[200px] items-center justify-center",
                onresize: move |e| {
                    match e.get_border_box_size() {
                        Ok(size) => {
                            let box_size = size.width;
                            if box_size < 600.0 {
                                mobile.set(true);
                            } else {
                                mobile.set(false);
                            }
                        }
                        Err(e) => {
                            tracing::error!("error: {:?}", e);
                        }
                    };
                },
                TextContainer {
                    image: asset!("/public/images/health.jpg"),
                    title: "{tr.health_title}",
                    desc: "{tr.health_desc}",
                }
                if mobile() {
                    TextContainer {
                        image: asset!("/public/images/social.jpg"),
                        title: "{tr.social_title}",
                        desc: "{tr.social_desc}",
                    }
                } else {
                    RightImageContainer {
                        image: asset!("/public/images/social.jpg"),
                        title: "{tr.social_title}",
                        desc: "{tr.social_desc}",
                    }
                }
                TextContainer {
                    image: asset!("/public/images/community.jpg"),
                    title: "{tr.community_title}",
                    desc: "{tr.community_desc}",
                }

                if mobile() {
                    TextContainer {
                        image: asset!("/public/images/ai.jpg"),
                        title: "{tr.ai_title}",
                        desc: "{tr.ai_desc}",
                    }
                } else {
                    RightImageContainer {
                        image: asset!("/public/images/ai.jpg"),
                        title: "{tr.ai_title}",
                        desc: "{tr.ai_desc}",
                    }
                }
            }
        }
    }
}

#[component]
pub fn TextContainer(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    image: Asset,
    title: String,
    desc: String,
) -> Element {
    rsx! {
        div { class: "w-full grid grid-cols-3 max-[600px]:grid-cols-1 max-[600px]:text-center gap-[40px] max-[600px]:gap-x-0",
            div { class: "w-full col-span-1 flex flex-col items-center justify-center items-end gap-[10px]",
                img {
                    class: "w-full max-w-[250px] rounded-[20px]",
                    src: "{image}",
                }
            }
            div { class: "w-full col-span-2 max-[600px]:grid-cols-1 flex flex-col justify-center gap-[10px]",
                h2 { class: "text-[42px] max-[600px]:text-[32px] font-bold", "{title}" }
                p { class: "text-[20px] max-[600px]:text-[14px]", "{desc}" }
            }
        }
    }
}

#[component]
pub fn RightImageContainer(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    image: Asset,
    title: String,
    desc: String,
) -> Element {
    rsx! {
        div { class: "w-full grid grid-cols-3 gap-[40px]",
            div { class: "w-full col-span-2 flex flex-col justify-center items-end gap-[10px]",
                h2 { class: "text-[42px] font-bold text-right", "{title}" }
                p { class: "text-[20px] text-right", "{desc}" }
            }
            div { class: "w-full col-span-1 flex flex-col justify-center items-start gap-[10px]",
                img {
                    class: "w-full max-w-[250px] rounded-[20px]",
                    src: "{image}",
                }
            }

        }
    }
}
