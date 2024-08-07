use crate::errors::AppError;
use crate::pages::error_template::ErrorTemplate;

use leptos::prelude::*;
use leptos_meta::Title;

#[server(CauseInternalServerError, "/api")]
pub async fn cause_internal_server_error() -> Result<(), ServerFnError> {
    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    Err(ServerFnError::ServerError(
        "Generic Server Error".to_string(),
    ))
}

#[component]
pub fn ExampleErrors() -> impl IntoView {
    let generate_internal_error = ServerAction::<CauseInternalServerError>::new();

    view! {
        <Title text="Error example | Leptos"/>
        <div>
            <p>
                "These links will load 404 pages since they do not exist. Verify with browser development tools: " <br/>
                <a href="/404">"This links to a page that does not exist"</a> <br/> <a href="/404" target="_blank">
                    "Same link, but in a new tab"
                </a>
            </p>
            <p>"After pressing this button check browser network tools. Can be used even when WASM is blocked:"</p>
            <ActionForm action=generate_internal_error>
                <input name="error1" type="submit" value="Generate Internal Server Error"/>
            </ActionForm>
            <p>
                "The following <div> will always contain an error and cause this page to produce status 500. Check browser dev tools. "
            </p>
            <div>
                // note that the error boundaries could be placed above in the Router or lower down
                // in a particular route. The generated errors on the entire page contribute to the
                // final status code sent by the server when producing ssr pages.
                <ErrorBoundary fallback=|errors| view! { <ErrorTemplate errors/> }>
                    <ReturnsError/>
                </ErrorBoundary>
            </div>
        </div>
    }
}

#[component]
pub fn ReturnsError() -> impl IntoView {
    Err::<String, AppError>(AppError::InternalServerError)
}
