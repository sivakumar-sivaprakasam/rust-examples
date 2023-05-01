fn main() {
    println!("{}", sum_it![1, 2, 3, 4, 5]);
}

#[macro_export]
macro_rules! sum_it {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_result: usize = 0;
            $(
                temp_result += $x;
            )*
            temp_result
        }
    };
}


extern crate proc_macro;
use proc_macro::TokenStream;

//#[proc_macro_derive(AnswerFn)]
//pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
//    "fn answer() -> u32 { 42 }".parse().unwrap()
//}

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}