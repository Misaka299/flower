use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemStruct};

#[proc_macro_attribute]
pub fn control(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item: ItemStruct = parse_macro_input!(item as ItemStruct);

    // 检查宏是否只应用于结构体
    if !item.fields.iter().any(|f| f.ident.is_some()) {
        panic!("Control macro can only be used on a struct with named fields");
    }

    item.fields.push(quote! {
        /// 组件id
        pub(crate) id: i32,
        pub(crate) name: String,
        /// 父级组件id
        pub(crate) parent_id: i32,
    });

    // 为新字段生成getter和setter方法
    let get_id = parse_quote! {
        pub fn get_id(&self) -> u32 {
            self.id
        }
    };
    item.push(parse_quote! { impl #item { #get_id } });

    let set_id = parse_quote! {
        pub fn set_id(&mut self, id: u32) {
            self.id = id;
        }
    };
    item
        .attrs
        .push(parse_quote! { impl #item { #set_id } });

    // 返回修改后的结构体定义
    TokenStream::from(quote! { #item })
}