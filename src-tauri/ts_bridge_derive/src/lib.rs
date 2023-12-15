extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::DeriveInput;

#[proc_macro_derive(TsBridge)]
pub fn ts_bridge(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST 语法树
    let ast: DeriveInput = syn::parse(input).unwrap();

    // 构建特征实现代码
    impl_ts(&ast)
}

fn impl_ts(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl TsBridge for #name {

            fn get_ts_name(&self) -> String {
                let format_k = capitalize_first_letter(self.key.clone().as_str());
                let ts_name = self.interface_name.clone() + format_k.as_str();
                ts_name
            }

        }
    };
    gen.into()
}
