use leptos::{*, html::Input};


#[component]
pub fn TypeArea(cx: Scope, send: Action<String, Result<String, ServerFnError>>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    
    view! {cx, 
        <div class="h-24 w-full fixed bottom-0 flex justify-center items-center p-5 border-t bg-zinc-900 border-zinc-700">
            <form class="w-full flex justify-center items-center gap-4" on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                send.dispatch(input.value());
                input.set_value("");
            }>
                <input class="w-2/3 p-4 border rounded-full input-fieldbg-zinc-700 border-zinc-700 text-white" type="text"/>
                <input class="h-full p-4 rounded-full cursor-pointer bg-green-700 text-white" type="submit"/>
            </form>
        </div>
    }
}