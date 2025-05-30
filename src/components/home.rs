use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <img
            src="images/Profile_Medium.jpeg"
            width="500px"
            alt="Christian Payne Profile Picture"
            class="rounded-lg shadow-lg"
        />

        <div>
            <p>I am a developer who works with code to solve problems.</p>
            <p>
                I pride myself on attention to detail and creating strong foundations for others to build off of.
            </p>
            <p>
                Based in Southern California, I am currently working as a Developer at Model Match.
            </p>
        </div>
    }
}
