use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
    pub header: String,
    pub message: Option<String>,
}

#[function_component(DDSModal)]
pub fn dds_modal(props: &Props) -> Html {
    let header = props.header.clone();
    let message = props.message.clone();
    let modal_open = props.message.is_some();

    html! {
        <dialog open={modal_open}>
            <article>
                <h2>{header}</h2>
                <p>
                {
                    { message.map(|m| html! { { m } }) }
                }
                </p>
                <footer>
                <button>{"Confirm"}</button>
                </footer>
            </article>
        </dialog>
    }
}
