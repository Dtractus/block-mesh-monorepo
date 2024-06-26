use leptos::*;
use leptos_router::A;

#[component]
pub fn NotificationComponent(summary: String, detailed: String, go_to: String) -> impl IntoView {
    view! {
        <div class="bg-gray-700 flex justify-center items-center h-screen">
            <div class="bg-gray-800 border-white border-solid border-2 p-8 rounded-lg shadow-md m-2">
                <div class="text-center">
                    <h1 class="mt-4 text-3xl font-bold tracking-tight text-white sm:text-5xl">
                        {{ summary }}
                    </h1>
                    <p class="mt-6 text-base leading-7 text-white">{{ detailed }}</p>
                    <div class="mt-10 flex items-center justify-center gap-x-6">
                        <A
                            href=go_to
                            class="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                        >
                            Go
                        </A>
                    </div>
                </div>
            </div>
        </div>
    }
}
