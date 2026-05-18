use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="flex w-screen h-screen" style="background-color: #f5f5f5;">
            <div class="container mx-auto flex flex-col justify-center items-center h-full gap-4">
                <div style="border: 3px solid black; border-radius: 20px; padding: 40px 60px; background: white; box-shadow: 6px 6px 0px black;">
                    <h1 style="font-size: 2.5rem; font-weight: 900; color: black; text-align: center; margin-bottom: 8px;">{"YewChat!"}</h1>
                    <p style="color: #888; text-align: center; margin-bottom: 24px; font-size: 0.9rem;">{"Real-time chat experience"}</p>
                    <form class="flex">
                        <input {oninput}
                            style="border: 3px solid black; border-radius: 12px 0 0 12px; padding: 12px 20px; font-size: 1rem; outline: none; background: #f5f5f5;"
                            placeholder="Enter your username..." />
                        <Link<Route> to={Route::Chat}>
                            <button {onclick} disabled={username.len()<1}
                                style="border: 3px solid black; border-left: none; border-radius: 0 12px 12px 0; padding: 12px 24px; font-weight: 900; font-size: 1rem; background: #FF6B35; color: white; cursor: pointer; text-transform: uppercase;">
                                {"Join"}
                            </button>
                        </Link<Route>>
                    </form>
                </div>
            </div>
        </div>
    }
}
