use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use quote::format_ident;
use syn::Data;

pub fn macro_impl(input: &syn::DeriveInput) -> TokenStream {
    let mut register_accessors = quote! { };
    let data = &input.data;
    match data {
        Data::Struct(_struct) => {
            let struct_name = &input.ident;
            // 遍历成员
            for field in &_struct.fields {
                let field_ident = &field.ident.as_ref().unwrap();
                let field_type = &field.ty.to_token_stream().to_string();
                if !String::from("u16").eq(field_type) {
                    continue;
                }
                let field_name = field_ident.to_token_stream();
                let get = format_ident!("get_{}", field_name.to_string());
                let set = format_ident!("set_{}", field_name.to_string());
                let _incr_by = format_ident!("{}_incr_by", field_name.to_string());
                let _incr_by_i16 = format_ident!("{}_incr_by_i16", field_name.to_string());
                let _incr_by_i8 = format_ident!("{}_incr_by_i8", field_name.to_string());
                let _get_and_incr_by = format_ident!("{}_get_and_incr_by", field_name.to_string());
                let _incr = format_ident!("{}_incr", field_name.to_string());
                let _get_and_incr = format_ident!("{}_get_and_incr", field_name.to_string());
                let _incr_by_and_get = format_ident!("{}_incr_by_and_get", field_name.to_string());
                let _incr_and_get = format_ident!("{}_incr_and_get", field_name.to_string());
                let _decr_by = format_ident!("{}_decr_by", field_name.to_string());
                let _get_and_decr_by = format_ident!("{}_get_and_decr_by", field_name.to_string());
                let _decr = format_ident!("{}_decr", field_name.to_string());
                let _get_and_decr = format_ident!("{}_get_and_decr", field_name.to_string());
                let _decr_by_and_get = format_ident!("{}_decr_by_and_get", field_name.to_string());
                let _decr_and_get = format_ident!("{}_decr_and_get", field_name.to_string());

                let get_lo = format_ident!("get_{}_lo", field_name.to_string());
                let set_lo = format_ident!("set_{}_lo", field_name.to_string());
                let get_hi = format_ident!("get_{}_hi", field_name.to_string());
                let set_hi = format_ident!("set_{}_hi", field_name.to_string());

                let register_accessor = quote! {
impl #struct_name {

    #[inline]
    pub fn #get(&self) -> u16 {
        self.#field_name
    }

    #[inline]
    pub fn #set(&mut self, val: u16) {
        self.#field_name = val;
    }

    #[inline]
    pub fn #_incr_by(&mut self, by: u16) {
        self.#field_name += by;
    }

    #[inline]
    pub fn #_incr_by_i16(&mut self, by: i16){
        self.#field_name = (self.#field_name as i32 + by as i32) as u16 ;
    }

    #[inline]
    pub fn #_incr_by_i8(&mut self, by: i8){
        self.#_incr_by_i16(by as i16);
    }

    #[inline]
    pub fn #_get_and_incr_by(&mut self, by: u16) -> u16 {
        let before = self.#field_name;
        self.#_incr_by(by);
        before
    }

    #[inline]
    pub fn #_incr(&mut self) {
         self.#_incr_by(0x0001u16);
    }

    #[inline]
    pub fn #_get_and_incr(&mut self) -> u16 {
        let before = self.#field_name;
        self.#_incr();
        before
    }

    #[inline]
    pub fn #_incr_by_and_get(&mut self, by: u16) -> u16 {
        self.#_incr_by(by);
        self.#field_name
    }

    #[inline]
    pub fn #_incr_and_get(&mut self) -> u16 {
        self.#_incr_by_and_get(0x0001u16);
        self.#field_name
    }

    #[inline]
    pub fn #_decr_by(&mut self, by: u16) {
        self.#field_name -= by;
    }

    #[inline]
    pub fn #_get_and_decr_by(&mut self, by: u16) -> u16 {
        let before = self.#field_name;
        self.#_decr_by(by);
        before
    }

    #[inline]
    pub fn #_decr(&mut self) {
         self.#_decr_by(0x0001u16);
    }

    #[inline]
    pub fn #_get_and_decr(&mut self) -> u16 {
        let before = self.#field_name;
        self.#_decr();
        before
    }

    #[inline]
    pub fn #_decr_by_and_get(&mut self, by: u16) -> u16 {
        self.#_decr_by(by);
        self.#field_name
    }

    #[inline]
    pub fn #_decr_and_get(&mut self) -> u16 {
        self.#_decr_by_and_get(0x0001u16);
        self.#field_name
    }

    #[inline]
    pub fn #get_hi(&self) -> u8 {
        (self.#field_name >> 8) as u8
    }

    #[inline]
    pub fn #set_hi(&mut self, val: u8) {
       self.#field_name = (self.#field_name & 0x00FF) | ((val as u16) << 8);
    }

    #[inline]
    pub fn #get_lo(&self) -> u8 {
        self.#field_name as u8
    }

    #[inline]
    pub fn #set_lo(&mut self, val: u8) {
       self.#field_name = (self.#field_name & 0xFF00) | (val as u16);
    }

}
                };
                register_accessors = quote! {
                    #register_accessors

                    #register_accessor
                }
            }
        }
        _ => ()
    }
    register_accessors.into()
}
