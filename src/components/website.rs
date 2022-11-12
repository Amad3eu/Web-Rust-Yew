use yew::{html, Html};

use crate::Model;

// consider test with Self and Component method later?
// pub fn steadylarner_blog() -> Html<Model> {
//     let class = "width-two-and-a-half theme-white border-round margin-right-half";
//     html! {
//         <a
//             class=("flex", "no-text-decoration", "hover", "cursor-pointer", "transition-half", "right-auto"),
//             href="",
//             title="Click it to learn how to code this.",
//             target="_blank",
//             rel="noopener noreferrer",
//         >
//             <span 
//                 class=("white", "bold", "flex", "center"), 
//             >
//                 <img 
//                     src="",
//                     class=class,
//                 />
//                 { "Full Stack Rust Chat App" }
//             </span>
//         </a>
//     }
// }

// Compared to React, it does work without dependency
// pub fn social(_title: &str, _description: &str, _image: &str) -> Html<Model> {
pub fn social() -> Html<Model> {
    html! {
        <>
            <title>{ "Full Stack Rust Chat App" }</title>
            <meta name="description", content="Rust Full Stack Website", />
            <meta name="thumbnail", content="", />
            <meta property="og:title", content="Full Stack Rust Chat App", />
            <meta property="og:description", content="Rust Full Stack Website", />
            <meta property="og:image", content="", />
            <meta property="og:locale", content="en_US", />
            <meta property="og:type", content="website", />
            <meta property="og:site_name", content="Full Stack Rust Chat App", />
            <meta name="twitter:title", content="Full Stack Rust Chat App", />
            <meta name="twitter:description", content="Rust Full Stack Website", />
            <meta name="twitter:image", content="", />
        </>
    }
}