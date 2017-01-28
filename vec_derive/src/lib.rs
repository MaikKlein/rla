#![allow(dead_code)]
#![recursion_limit="2048"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate rustfmt;
extern crate itertools;

use rustfmt::*;
use syn::{Ty, PathSegment, Field, Ident, Body, Variant, VariantData, MacroInput};
use quote::ToTokens;
use proc_macro::TokenStream;

#[proc_macro_derive(Matrix, attributes(mat))]
pub fn mat_derive(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = gen_mat_derive(&ast);
    //let mut out = String::new();
    //let config =
    //    config::Config { write_mode: config::WriteMode::Plain, ..config::Config::default() };
    //let fmt_r = format_input::<Vec<u8>>(Input::Text(gen.to_string()),
    //                                    &config,
    //                                    Some(unsafe { out.as_mut_vec() }));
    //if let Ok(fmt) = fmt_r {
    //    println!("{}", out);
    //}
    gen.parse().unwrap()
}

fn gen_mat_derive(input: &MacroInput) -> quote::Tokens {
    let ident = &input.ident;
    let fields = match input.body {
        Body::Struct(ref data) => {
            match data {
                &VariantData::Struct(ref fields) => fields.clone(),
                _ => panic!("Only supports structs."),
            }
        }
        _ => panic!("Only supports structs."),
    };

    let field_idents_: Vec<Ident> = fields.iter().map(|f| f.ident.clone().unwrap()).collect();
    let first_field = field_idents_[0].clone();
    let field_idents_1 = &field_idents_;

    let field_idents_2 = &field_idents_;
    let ty = &fields[0].ty;
    let field_params = field_idents_.iter().fold(quote::Tokens::new(), |mut token, field| {
        token.append(&format!("{}: {},", field.as_ref(), quote!(#ty).as_str()));
        token
    });
    let mut unrolled_mul = quote::Tokens::new();
    let dim = fields.len();
    for i in 0..dim {
        for j in 0..dim {
            for k in 0..dim {
                unrolled_mul.append(
                    &format!(
                        "*r.get_unchecked_mut({i}).get_unchecked_mut({j}) +=
                            *self.get_unchecked({i}).get_unchecked({j}) * *other.get_unchecked({k}).get_unchecked({j});", i=i, j=j, k=k)
                );
            }
        }
    }

    let unrolled_mul = unrolled_mul;
    let mut unrolled_transpose = quote::Tokens::new();
    for i in 0..dim {
        for j in 0..dim {
            unrolled_transpose.append(&format!("r[{i}][{j}] = self[{j}][{i}];", i=i, j=j));
        }
    }

    let mut unrolled_mul_vec = quote::Tokens::new();
    for i in 0..dim {
        unrolled_mul_vec.append(&format!(
                    "*r.get_unchecked_mut({i}) =
                        self.get_unchecked({i}).dot(other);", i=i));
    }

    let mut unrolled_identity = quote::Tokens::new();
    for i in 0..dim {
        for j in 0..dim {
            if i == j {
                unrolled_identity.append(&format!(
                            "*r.get_unchecked_mut({i}).get_unchecked_mut({j}) = T::one();", i=i, j=j));
            }
            else{
                unrolled_identity.append(&format!(
                            "*r.get_unchecked_mut({i}).get_unchecked_mut({j}) = T::zero();", i=i, j=j));
            }
        }
    }
    quote!{
        impl<T> #ident<T> {
            pub fn from_rows(#field_params) -> Self{
                #ident{
                    #(
                        #field_idents_1: #field_idents_2,
                    )*
                }
            }

            pub unsafe fn get_unchecked(&self, idx: usize) -> &#ty{
                let ptr = &self.#first_field as *const #ty;
                &*ptr.offset(idx as isize)
            }

            pub unsafe fn get_unchecked_mut(&mut self, idx: usize) -> &mut #ty{
                let ptr = &mut self.#first_field as *mut #ty;
                &mut *ptr.offset(idx as isize)
            }
       }

       impl<T> #ident<T>
           where T: Copy{
        pub fn transpose(&self) -> Self{
                let mut r: Self = unsafe{ ::std::mem::uninitialized()};
                #unrolled_transpose
                r
            }
       }
       impl<T> #ident<T>
            where T: Float {
            pub fn identity() -> Self {
                let mut r: Self = unsafe{ ::std::mem::uninitialized()};
                unsafe{
                    #unrolled_identity
                }
                r
            }
       }

       impl<T> ::std::ops::Index<usize> for #ident<T> {
           type Output = #ty;
           fn index(&self, idx: usize) -> &Self::Output {
               assert!(idx < #dim, format!("Index: {} is out of bounds.", idx));
               unsafe{ self.get_unchecked(idx) }
           }
       }

       impl<T> ::std::ops::IndexMut<usize> for #ident<T> {
           fn index_mut(&mut self, idx: usize) -> &mut #ty {
               assert!(idx < #dim, format!("Index: {} is out of bounds.", idx));
               unsafe{ self.get_unchecked_mut(idx) }
           }
       }

       impl<'a, T> ::std::ops::Mul<#ty> for #ident<T>
           where T: Float + ::std::ops::AddAssign{
           type Output = #ty;
           fn mul(self, other: #ty) -> Self::Output {
               let mut r: #ty = unsafe{ ::std::mem::uninitialized()};
               unsafe{
                    #unrolled_mul_vec
               }
               r
           }
       }

       impl<'a, T> ::std::ops::Mul<&'a #ident<T>> for #ident<T>
           where T: Float + ::std::ops::AddAssign{
           type Output = #ident<T>;
           fn mul(self, other: &#ident<T>) -> Self::Output {
               let mut r: Self = unsafe{ ::std::mem::zeroed()};
               unsafe{
                 #unrolled_mul
               }
               r
           }
       }
    }
}



#[proc_macro_derive(Vector, attributes(vec))]
pub fn vec_derive(input: TokenStream) -> TokenStream {
    use rustfmt::*;
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = gen_vec_derive(&ast);
    let mut out = String::new();
    // let config =
    //    config::Config { write_mode: config::WriteMode::Plain, ..config::Config::default() };
    // let fmt_r = format_input::<Vec<u8>>(Input::Text(gen.to_string()),
    //                                    &config,
    //                                    Some(unsafe { out.as_mut_vec() }));
    // if let Ok(fmt) = fmt_r {
    //    println!("{}", out);
    // }
    gen.parse().unwrap()
}
struct OpTrait {
    name: Ident,
    fn_name: Ident,
    op: Ident,
}

impl OpTrait {
    pub fn new<T: Into<Ident>>(name: T, fn_name: T, op: T) -> Self {
        OpTrait {
            name: name.into(),
            fn_name: fn_name.into(),
            op: op.into(),
        }
    }
}


fn impl_op_vec(ident: &Ident,
               field_idents: &Vec<Ident>,
               name: Ident,
               fn_name: Ident,
               op: Ident)
               -> quote::Tokens {
    let s: &Vec<_> = &field_idents.iter()
        .map(|ident| {
            quote::Ident::from(format!("{}: self.{} {} other.{}", ident, ident, op, ident))
        })
        .collect();
    quote!{
        impl<T: Num> ::std::ops::#name for #ident<T>{
            type Output = Self;
            fn #fn_name(self, other: Self) -> Self::Output{
                #ident{
                    #(
                        #s,
                    )*
                }
            }
        }

        impl<'a, 'b, T: Num + Copy> ::std::ops::#name<&'b #ident<T>> for &'a #ident<T>{
            type Output = #ident<T>;
            fn #fn_name(self, other: &'b #ident<T>) -> Self::Output{
                #ident{
                    #(
                        #s,
                    )*
                }
            }
        }

        impl<'a, T: Num + Copy> ::std::ops::#name<#ident<T>> for &'a #ident<T>{
            type Output = #ident<T>;
            fn #fn_name(self, other: #ident<T>) -> Self::Output{
                #ident{
                    #(
                        #s,
                    )*
                }
            }
        }

        impl<'a, T: Num + Copy> ::std::ops::#name<&'a #ident<T>> for #ident<T>{
            type Output = #ident<T>;
            fn #fn_name(self, other: &'a #ident<T>) -> Self::Output{
                #ident{
                    #(
                        #s,
                    )*
                }
            }
        }
    }
}

fn impl_op_scalar(ident: &Ident,
                  field_idents: &Vec<Ident>,
                  name: Ident,
                  fn_name: Ident,
                  op: Ident)
                  -> quote::Tokens {
    let s: &Vec<_> = &field_idents.iter()
        .map(|ident| quote::Ident::from(format!("{}: self.{} {} other", ident, ident, op)))
        .collect();
    quote!{
        impl<'a, T: Num + Copy> ::std::ops::#name<T> for &'a #ident<T>{
            type Output = #ident<T>;
            fn #fn_name(self, other: T) -> Self::Output{
                #ident{
                    #(
                        #s,
                    )*
                }
            }
        }

        impl<T: Num + Copy> ::std::ops::#name<T> for #ident<T>{
            type Output = Self;
            fn #fn_name(self, other: T) -> Self::Output{
                #ident{
                    #(
                        #s,
                    )*
                }
            }
        }
    }
}

fn change_dimension(ident: &Ident, new_dim: usize) -> Ident {
    use std::str::FromStr;
    let mut new_ident = String::from_str(ident.as_ref()).unwrap();
    let new_len = new_ident.len() - 1;
    new_ident.truncate(new_len);
    new_ident.push_str(&format!("{}", new_dim));
    Ident::new(new_ident)
}

fn gen_vec_derive(input: &MacroInput) -> quote::Tokens {
    let ident = &input.ident;
    let fields = match input.body {
        Body::Struct(ref data) => {
            match data {
                &VariantData::Struct(ref fields) => fields.clone(),
                _ => panic!("Only supports structs."),
            }
        }
        _ => panic!("Only supports structs."),
    };
    let type_ident = &input.generics.ty_params[0].ident;
    let dim = fields.len();
    let ndim = dim.pow(dim as u32);
    let field_idents_: Vec<Ident> = fields.iter().map(|f| f.ident.clone().unwrap()).collect();
    let field_idents1 = &field_idents_;
    let field_idents2 = &field_idents_;
    let extend = if dim < 4 && dim >= 2 {
        let ident_upper_dim = change_dimension(ident, dim + 1);
        quote!{
            pub fn extend(&self, val: T) -> #ident_upper_dim<T>{
                #ident_upper_dim::new(#(self.#field_idents1),*, val)
            }
        }
    } else {
        quote!{}
    };
    let s_iter: Vec<_> = (0..field_idents_.len())
        .map(|i| {
            let iter: Vec<_> = field_idents_.iter()
                .map(|ident| ::std::iter::repeat(ident.clone()).take(dim.pow(i as u32)))
                .collect();
            let iter2 = iter.into_iter().fold(Vec::new(), |acc, iter| {
                acc.into_iter().chain(iter).collect::<Vec<_>>()
            });

            iter2.into_iter().cycle().take(ndim).collect::<Vec<_>>()
        })
        .collect();

    let swizzle_ident: Vec<_> = (0..ndim)
        .map(|idx| {
            let mut name = String::new();
            for i in 0..dim {
                name.push_str(&s_iter[i][idx].as_ref());
            }
            name
        })
        .collect();

    let swizzle: Vec<_> = swizzle_ident.into_iter()
        .map(|name| {
            let swizzle_field = name.chars().map(|c| quote::Ident::new(format!("{}", c)));
            let fn_name = quote::Ident::new(name.clone());
            quote!{
            pub fn #fn_name(&self) -> Self{
                #ident{
                    #(
                        #field_idents1: self.#swizzle_field,
                    )*
                }
            }
        }
        })
        .collect();

    let optraits = vec![OpTrait::new("Add", "add", "+"),
                        OpTrait::new("Mul", "mul", "*"),
                        OpTrait::new("Div", "div", "/"),
                        OpTrait::new("Sub", "sub", "-")];

    let impl_op_scalar = optraits.iter()
        .map(|ot| {
            impl_op_scalar(ident,
                           &field_idents_,
                           ot.name.clone(),
                           ot.fn_name.clone(),
                           ot.op.clone())
        });

    let impl_op_vec = optraits.iter()
        .map(|ot| {
            impl_op_vec(ident,
                        &field_idents_,
                        ot.name.clone(),
                        ot.fn_name.clone(),
                        ot.op.clone())
        });

    let first_field = field_idents_[0].clone();
    let number_of_fields = field_idents_.len();
    quote!{
        impl<T> #ident<T> {

            // 'match' is too slow here, see: https://godbolt.org/g/Bgqyx2
            #[inline]
            pub unsafe fn get_unchecked(&self, idx: usize) -> &T{
                let ptr = &self.#first_field as *const T;
                &*ptr.offset(idx as isize)
            }

            #[inline]
            pub unsafe fn get_unchecked_mut(&mut self, idx: usize) -> &mut T{
                let ptr = &mut self.#first_field as *mut T;
                &mut *ptr.offset(idx as isize)
            }

            pub fn new(#(#field_idents1: T, )*) -> Self{
                #ident{
                    #(
                        #field_idents1: #field_idents2,
                    )*
                }
            }

            #[inline]
            pub fn as_raw_slice(&self) -> &[T]{
                let ptr = &self.#first_field as *const T;
                unsafe {
                    ::std::slice::from_raw_parts(ptr, #number_of_fields)
                }
            }

            #[inline]
            pub fn map<F, B>(self, f: F) -> #ident<B>
                where F: Fn(T) -> B {
                #ident{
                    #(
                        #field_idents1: f(self.#field_idents2),
                    )*
                }
            }

        }

        impl<T> #ident<T>
            where T: Num + Copy + Zero {

            #extend

            #[inline]
            pub fn zero() -> Self{
                #ident{
                    #(
                        #field_idents1: T::zero(),
                    )*
                }
            }

            #[inline]
            pub fn dot(self, other: Self) -> T{
                #(
                    self.#field_idents1 * other.#field_idents2
                )+ *
            }

            #[inline]
            pub fn length_sq(self) -> T {
                self.dot(self)
            }

            #[inline]
            pub fn distance_sq(self, other: Self) -> T{
                (self - other).length_sq()
            }

            #(#swizzle)*
        }

        impl<T> #ident<T>
            where T: Float + Copy {

            #[inline]
            pub fn max(self) -> T{
                variadic2!(T::max, #(self.#field_idents1),*)
            }

            #[inline]
            pub fn min(self) -> T{
                variadic2!(T::min, #(self.#field_idents1),*)
            }

            #[inline]
            pub fn length(self) -> T {
                self.length_sq().sqrt()
            }

            #[inline]
            pub fn project(self, other: Self) -> Self {
                other * (self.dot(other) / other.length_sq())
            }

            #[inline]
            pub fn normalize(self) -> Option<Self> {
                 let len_sq = self.length_sq();
                 if len_sq == T::one() {
                     Some(self)
                 } else if len_sq == T::zero() {
                     None
                 } else {
                     Some(self / len_sq.sqrt())
                 }
            }

            #[inline]
            pub fn reflect_normal(self, normal: Self) -> Self{
                let two: T = NumCast::from(2).unwrap();
                let r = self - normal;
                r
            }

            #[inline]
            pub fn distance(self, other: Self) -> T{
                self.distance_sq(other).sqrt()
            }

            #[inline]
            pub fn lerp(self, target: Self, scale: T) -> Self{
                self + (target - self) * scale
            }
        }

        impl<T> ::std::ops::Index<usize> for #ident<T> {
            type Output = T;
            fn index(&self, idx: usize) -> &T{
                assert!(idx < #dim, format!("{} is out of bounds", idx));
                unsafe {
                    self.get_unchecked(idx)
                }
            }
        }

        impl<T> ::std::ops::IndexMut<usize> for #ident<T> {
            fn index_mut(&mut self, idx: usize) -> &mut T{
                assert!(idx < #dim, format!("{} is out of bounds", idx));
                unsafe{
                    self.get_unchecked_mut(idx)
                }
            }
        }

        #(#impl_op_vec)*
        #(#impl_op_scalar)*
    }
}
