use leptos::{*, html::Div};
use crate::model::conversation::Conversation;

const USER_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 bg-blue-500 text-white rounded-lg self-end";

const MODEL_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 bg-green-200 text-black rounded-lg self-start";

#[component]
pub fn ChatArea(cx: Scope, conversation: ReadSignal<Conversation>) -> impl IntoView {
    let chat_div_ref = create_node_ref::<Div>(cx);

    create_effect(cx, move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! {cx, 
        <div>
            {move || conversation.get().messages.iter().map(move |message| {
                let class_str = if message.user {USER_MESSAGE_CLASS} else {MODEL_MESSAGE_CLASS};
                view! {cx, 
                    <div class={class_str}>
                        {message.text.clone()}
                    </div>
                }
            }).collect::<Vec<_>>()

            }
        </div>
        }
}