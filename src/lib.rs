use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

pub fn app(cx: Scope<()>) -> Element {
    let chain_flag = use_state(cx, || true);
    cx.render(rsx!{
        section { class: "todoapp",
            style { include_str!("../src/style.css") }
            div { class: "payment-container",
                header { class: "",
                    h2 {"Payment"}
                    p {"Choose your payment method and enter details."}
                    div {class: "toggle-group",
                        button {
                            onclick: move |_| chain_flag.set(true),
                            class: if *chain_flag.get() {"toggle-btn active"} else {"toggle-btn inactive"}, id: "onchain", "Onchain",
                        }
                        button {
                            onclick: move |_| chain_flag.set(false),
                            class: if *chain_flag.get() {"toggle-btn inactive"} else {"toggle-btn active"}, id: "internal", "Internal"
                        }
                    }
                }
                if *chain_flag.get() {
                    rsx!(
                        div { class: "form-group",
                            label { r#for: "chain", "Chain" }
                            select {
                                id: "chain",
                                class: "chain-select",
                                aria_placeholder: "Select chain",
                                option { value: "",disabled:"true", selected: "true", "Select chain" }
                                option { value: "ethereum", "Ethereum" }
                                option { value: "polygon", "Polygon" }
                                option { value: "bnb chain", "BNB Chain" }
                                // Add more options if needed
                            }
                        }
                        //Amount Input
                        div { class: "form-group",
                            label { r#for: "address", "Address" }
                            div { class: "address-input-group",
                                input {
                                    r#type: "text",
                                    id: "address",
                                    class: "address-input",
                                    placeholder: "Enter address"
                                }
                                button { class: "qr-button", "📷" }
                            }
                        }

                        // Amount Input
                        div { class: "form-group",
                            label { r#for: "amount", "Amount" }
                            input {
                                r#type: "text",
                                id: "amount",
                                class: "amount-input",
                                placeholder: "Enter amount"
                            }
                        }
                    )
                }
                else{
                    rsx!(
                        p {class:"internal-text", "Internal payment options will be available soon."}
                    )
                }
                // Submit Button
                div { class: "form-group",
                    button { class: "submit-button", "Submit Payment" }
                }
            }
        }
    })
}