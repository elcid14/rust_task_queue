use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemFn, Meta, Expr, ExprLit, Lit};

#[proc_macro_attribute]
pub fn task(attr: TokenStream, item: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(attr as Meta);
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    let task_name = match meta {
        Meta::NameValue(mnv) if mnv.path.is_ident("name") => {
            match mnv.value {
                Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => s.value(),
                _ => panic!("Expected string literal for task name"),
            }
        },
        _ => panic!("Expected #[task(name = \"...\")]"),
    };

    let register_fn = format_ident!("register_{}", fn_name);

    let expanded = quote! {
        #input_fn

        #[ctor::ctor]
        fn #register_fn() {
            use crate::task_registry::register_task_handler;
            use crate::task::Task;
            use lapin::{Channel, message::Delivery, options::*};
            use std::sync::Arc;
            use std::pin::Pin;
            use futures::Future;
            use serde_json::Value;

            register_task_handler(#task_name, Arc::new(|delivery: Delivery, channel: Channel| {
                Box::pin(async move {
                    match Task::new(delivery.data) {
                        Ok(mut task) => {
                            let payload = task.payload.get("payload").cloned().unwrap_or(Value::Null);
                            match #fn_name(payload, task.clone()).await {
                                Ok(_) => {
                                    task.complete();
                                    let _ = channel.basic_ack(delivery.delivery_tag, Default::default()).await;
                                }
                                Err(_) => {
                                    task.retry();
                                    let _ = channel.basic_nack(delivery.delivery_tag, Default::default()).await;
                                }
                            }
                        }
                        Err(_) => {
                            let _ = channel.basic_nack(delivery.delivery_tag, Default::default()).await;
                        }
                    }
                })
            }));
        }
    };

    TokenStream::from(expanded)
}
