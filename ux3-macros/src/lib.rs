use std::ops::RangeInclusive;
use std::str::FromStr;
use quote::{format_ident, quote};
use proc_macro2::{Literal, TokenStream};

#[cfg(target_pointer_width = "16")]
const TARGET_POINTER_WIDTH: u8 = 16;
#[cfg(target_pointer_width = "32")]
const TARGET_POINTER_WIDTH: u8 = 32;
#[cfg(target_pointer_width = "64")]
const TARGET_POINTER_WIDTH: u8 = 64;


fn get_enum_variant(number: u128, negative: bool) -> proc_macro2::Ident{
    format_ident!("Num{}{}", if negative {"Neg"} else {""}, number)
}
fn enum_variants_neg(range: RangeInclusive<u128>) -> impl Iterator<Item=(proc_macro2::Ident, (bool, u128))> {
    range.rev().map(move |item|
        (
            get_enum_variant(item, true),
            (true, item)
        )
    )
}
fn enum_variants(range: RangeInclusive<u128>) -> impl Iterator<Item=(proc_macro2::Ident, (bool, u128))> {
    range.map(move |item|
        (
            get_enum_variant(item, false),
            (false, item)
        )
    )
}

fn variants_to_definitions<'a>(variants: impl Iterator<Item=&'a(proc_macro2::Ident, (bool, u128))>) -> TokenStream {
    variants.map(|(variant, _)| variant)
        .map(|str| quote!(#str))
        .reduce(|t1, t2|quote!(#t1, #t2))
        .unwrap()
}

fn match_from_std_statments<'a>(variants: impl Iterator<Item=&'a(proc_macro2::Ident, (bool, u128))>) -> TokenStream {
    variants.map(|(variant, (negative, number))| {
        // let ident = format!("Self::{variant}");
        let negative = if *negative { "-" } else {""};
        // let number = format!("{negative}{number}");
        let statement = format!("{negative}{number} => Some(Self::{variant}),");
        statement.parse::<TokenStream>().unwrap()
    })
    .reduce(|v1, v2| {
        quote!(
            #v1
            #v2
        )
    }).unwrap()
}
fn match_to_std_statments<'a>(variants: impl Iterator<Item=&'a(proc_macro2::Ident, (bool, u128))>) -> TokenStream {
    variants.map(|(variant, (negative, number))| {
        let negative = if *negative { "-" } else {""};
        let statement = format!("Self::{variant} => {negative}{number},");
        statement.parse::<TokenStream>().unwrap()
    })
    .reduce(|v1, v2| {
        quote!(
            #v1
            #v2
        )
    }).unwrap()
}

fn generate_enum(item: u32, intotype:u32, sized: bool) -> TokenStream {
    let prefix = if sized {"i"} else {"u"};
    let enum_name = format_ident!("{prefix}{item}");
    let enum_name_path_self = format!("self::{enum_name}").parse::<TokenStream>().unwrap();
    let enum_name_path_super = format!("super::{enum_name}").parse::<TokenStream>().unwrap();
    let test_enum_name = format_ident!("test{prefix}{item}");
    let intotype_name = format!("core::primitive::{prefix}{intotype}").parse::<TokenStream>().unwrap();
    let min;
    let max;
    let default = (0, false);
    let variants:Vec<_> = if sized {
        min = (2u128.pow(item-1), true);
        max = (2u128.pow(item-1)-1, false);
        enum_variants_neg(1..=(min.0)).chain(
            enum_variants(0..=(max.0))
        ).collect()
    } else {
        min = (0, false);
        max = (2u128.pow(item)-1, false);
        enum_variants((min.0)..=(max.0)).collect()
    };
    let variant_number_str:Vec<_> = variants.iter().cloned().map(|(name, (neg, val))|{
        let negative = if neg { "-" } else {""};
        let number = format!("{negative}{val}").parse::<TokenStream>().unwrap();
        quote!{
            ( #name, #number )
        }
    }).collect();
    let min_ident:TokenStream = format!("Self::{}", get_enum_variant(min.0, min.1)).parse().unwrap();
    let min:TokenStream = format!("{}{}", if min.1 {"-"} else {""}, min.0).parse().unwrap();
    let max_ident:TokenStream = format!("Self::{}", get_enum_variant(max.0, max.1)).parse().unwrap();
    let max:TokenStream = format!("{}{}", if max.1 {"-"} else {""}, max.0).parse().unwrap();
    let default_ident:TokenStream = format!("Self::{}", get_enum_variant(default.0, default.1)).parse().unwrap();
    let default:TokenStream = format!("{}{}", if default.1 {"-"} else {""}, default.0).parse().unwrap();


    let definitions = variants_to_definitions(variants.iter());
    let match_to_std = match_to_std_statments(variants.iter());
    let match_from_std = match_from_std_statments(variants.iter());
    let item = item.to_string().parse::<TokenStream>().unwrap();
    let min_doctest = format!(" assert_eq!(<ux3::{enum_name}>::MIN, <ux3::{enum_name}>::from_std( {min} ).unwrap());");
    let default_const_doctest = format!(" assert_eq!(<ux3::{enum_name}>::DEFAULT, <ux3::{enum_name}>::from_std( {default} ).unwrap());");
    let max_doctest = format!("assert_eq!(<ux3::{enum_name}>::MAX, <ux3::{enum_name}>::from_std( {max} ).unwrap());");
    let bits_doctest = format!("assert_eq!(<ux3::{enum_name}>::BITS, {item} );");
    let default_doctest = format!(" assert_eq!(<ux3::{enum_name}>::DEFAULT, <ux3::{enum_name}>::default());");
    let to_std_doctest = format!("\
assert_eq!(<ux3::{enum_name}>::MIN.to_std(), {min} );
assert_eq!(<ux3::{enum_name}>::MAX.to_std(), {max} );
");
    let from_std_doctest = format!("\
assert_eq!(Some( <ux3::{enum_name}>::MIN ), <ux3::{enum_name}>::from_std( {min} ) );
assert_eq!(Some( <ux3::{enum_name}>::MAX ), <ux3::{enum_name}>::from_std( {max} ) );
");
    quote!(
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub enum #enum_name {
    #definitions
}
impl core::default::Default for #enum_name_path_self{
    fn default()->Self{
        #default_ident
    }
}
impl core::cmp::PartialOrd<Self> for #enum_name_path_self {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.to_std().partial_cmp(&other.to_std())
    }
    fn lt(&self, other: &Self) -> bool {
        self.to_std().lt(&other.to_std())
    }
    fn le(&self, other: &Self) -> bool {
        self.to_std().le(&other.to_std())
    }
    fn gt(&self, other: &Self) -> bool {
        self.to_std().gt(&other.to_std())
    }
    fn ge(&self, other: &Self) -> bool {
        self.to_std().ge(&other.to_std())
    }
}
impl core::cmp::Ord for #enum_name_path_self {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.to_std().cmp(&other.to_std())
    }
}
impl core::fmt::Debug for #enum_name_path_self {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.to_std(), f)
    }
}
impl core::fmt::Display for #enum_name_path_self {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.to_std(), f)
    }
}
impl core::fmt::Binary for #enum_name_path_self {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Binary::fmt(&self.to_std(), f)
    }
}
impl core::fmt::LowerHex for #enum_name_path_self {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.to_std(), f)
    }
}
impl core::fmt::UpperHex for #enum_name_path_self {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::UpperHex::fmt(&self.to_std(), f)
    }
}
impl core::fmt::Octal for #enum_name_path_self {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Octal::fmt(&self.to_std(), f)
    }
}
impl #enum_name_path_self {

    /// The smallest value that can be represented by this integer type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    #[doc = #min_doctest]
    /// ```
    pub const MIN: #enum_name_path_self = #min_ident;
    /// The default value of this integer type.
    /// This is usually 0.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    #[doc = #default_const_doctest]
    #[doc = #default_doctest]
    /// ```
    pub const DEFAULT: #enum_name_path_self = #default_ident;
    /// The largest value that can be represented by this integer type.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    #[doc = #max_doctest]
    /// ```
    pub const MAX: #enum_name_path_self = #max_ident;

    /// The size of this integer type in bits.
    ///
    /// # Examples
    ///
    /// ```
    #[doc = #bits_doctest]
    /// ```
    pub const BITS: core::primitive::u32 = #item;
    /// Converts this integer type into the next best fitting
    /// normal rust integer type.
    /// ```
    #[doc = #to_std_doctest]
    /// ```
    pub const fn to_std(&self) -> #intotype_name {
        match self {
            #match_to_std
        }
    }
    /// Converts this integer type into the next best fitting
    /// normal rust integer type.
    /// ```
    #[doc = #from_std_doctest]
    /// ```
    pub const fn from_std(value: #intotype_name) -> Option<Self> {
        match value{
            #match_from_std
            _ => None,
        }
    }
}
impl core::convert::From<#enum_name_path_self> for #intotype_name {
    fn from(value: #enum_name_path_self) -> Self{
        value.to_std()
    }
}

#[cfg(test)]
mod #test_enum_name {
  #[test]
  fn test_ord(){
    use super::*;
    let test = [<#enum_name_path_super>::MIN, <#enum_name_path_super>::DEFAULT, <#enum_name_path_super>::MAX];
    for i in test{
        for j in test {
             assert_eq!(i.partial_cmp(&j).unwrap(), i.cmp(&j))
        }
    }
    assert!(<#enum_name_path_super>::MIN <= <#enum_name_path_super>::DEFAULT);
    assert!(<#enum_name_path_super>::MAX > <#enum_name_path_super>::DEFAULT);
  }
  #[test]
  fn test_eq(){
    use super::*;
    assert_eq!(<#enum_name_path_super>::MIN, <#enum_name_path_super>::MIN);
    assert_eq!(<#enum_name_path_super>::DEFAULT, <#enum_name_path_super>::DEFAULT);
    assert_ne!(<#enum_name_path_super>::MAX, <#enum_name_path_super>::DEFAULT);
    assert_eq!(<#enum_name_path_super>::MAX, <#enum_name_path_super>::MAX);
  }
  #[test]
  fn test_debug(){
    use super::*;
    let test = [<#enum_name_path_super>::MIN, <#enum_name_path_super>::DEFAULT, <#enum_name_path_super>::MAX];
    for i in test{
      assert_eq!(format!("{:?}", i), format!("{:?}", i.to_std()));
    }
  }
  #[test]
  fn test_display(){
    use super::*;
    let test = [<#enum_name_path_super>::MIN, <#enum_name_path_super>::DEFAULT, <#enum_name_path_super>::MAX];
    for i in test{
      assert_eq!(format!("{}", i), format!("{}", i.to_std()));
    }
  }
  #[test]
  fn test_binary(){
    use super::*;
    let test = [<#enum_name_path_super>::MIN, <#enum_name_path_super>::DEFAULT, <#enum_name_path_super>::MAX];
    for i in test{
      assert_eq!(format!("{:b}", i), format!("{:b}", i.to_std()));
    }
  }
  #[test]
  fn test_lower_hex(){
    use super::*;
    let test = [<#enum_name_path_super>::MIN, <#enum_name_path_super>::DEFAULT, <#enum_name_path_super>::MAX];
    for i in test{
      assert_eq!(format!("{:x}", i), format!("{:x}", i.to_std()));
    }
  }
  #[test]
  fn test_upper_hex(){
    use super::*;
    let test = [<#enum_name_path_super>::MIN, <#enum_name_path_super>::DEFAULT, <#enum_name_path_super>::MAX];
    for i in test{
      assert_eq!(format!("{:X}", i), format!("{:X}", i.to_std()));
    }
  }
  #[test]
  fn test_octal_hex(){
    use super::*;
    let test = [<#enum_name_path_super>::MIN, <#enum_name_path_super>::DEFAULT, <#enum_name_path_super>::MAX];
    for i in test{
      assert_eq!(format!("{:o}", i), format!("{:o}", i.to_std()));
    }
  }
  #[test]
  fn test_to_std(){
    use #enum_name_path_super::*;
    #(
        {
            let val = #variant_number_str;
            assert_eq!(val.0.to_std(), val.1);
        }
    )*
  }
  #[test]
  fn test_from_std(){
    use #enum_name_path_super::*;
    #(
        {
            let val = #variant_number_str;
            assert_eq!(Some(val.0), <#enum_name_path_super>::from_std(val.1));
        }
    )*
    if <#enum_name_path_super>::MAX.to_std() < <#intotype_name>::MAX {
        for i in (<#enum_name_path_super>::MAX.to_std()+1)..=<#intotype_name>::MAX {
            assert_eq!(None, <#enum_name_path_super>::from_std(i))
        }
    }
    for i in <#intotype_name>::MIN..<#enum_name_path_super>::MIN.to_std() {
        assert_eq!(None, <#enum_name_path_super>::from_std(i))
    }
  }
}

)}

#[proc_macro]
pub fn define_enum(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item.into_iter().filter_map(|item|{
        match item {
            proc_macro::TokenTree::Literal(literal) => {
                Some(u32::from_str(&literal.to_string()).unwrap())
            },
            proc_macro::TokenTree::Punct(_) => {None},
            x => panic!("Unexpexted token: {x:?}"),
        }
    }).map(|item|{
        let next_pow = {
            let mut next_pow = item.next_power_of_two();
            while next_pow < 8 {
                next_pow = (next_pow+1).next_power_of_two();
            }
            next_pow
        };
        let uenum = generate_enum(item, next_pow, false);
        let ienum = generate_enum(item, next_pow, true);
        quote!(
            #uenum
            #ienum
        )
    }).collect::<TokenStream>().into()
}