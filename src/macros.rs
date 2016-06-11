// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! item {
    ($item:item) => ($item);
}

macro_rules! expr {
    ($expr:expr) => ($expr);
}

macro_rules! define_struct_impls {
    // Entry point.
    ($name:ident $body:tt) => (
        define_struct_impls!($name $body {});
    );
    // Exit point, all fields have been accumulated.
    ($name:ident {} { $($field:ident)* }) => (
        impl ::serde::Deserialize for $name {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer
            {
                $(let $field = try!(::serde::Deserialize::deserialize(deserializer));)+
                Ok($name {
                    $($field: $field,)+
                })
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: ::serde::Serializer
            {
                $(try!(self.$field.serialize(serializer));)+
                Ok(())
            }
        }
    );
    // Skip attribute.
    ($name:ident { #[$attr:meta] $($rest:tt)* } $acc:tt) => (
        define_struct_impls!($name { $($rest)* } $acc);
    );
    // Skip 'pub'.
    ($name:ident { pub $($rest:tt)* } $acc:tt) => (
        define_struct_impls!($name { $($rest)* } $acc);
    );
    // Accumulate field name.
    ($name:ident { $field:ident: $ty:ty, $($rest:tt)* } { $($acc:tt)* }) => (
        define_struct_impls!($name { $($rest)* } { $($acc)* $field });
    );
}

macro_rules! define_struct_tuple_impls {
    // Entry point. Each field of the tuple gets a stock name from { a b },
    // this works because the largest struct tuples we use are pairs.
    ($name:ident $body:tt) => (
        define_struct_tuple_impls!($name $body { a b } {});
    );
    // Exit point, all fields have been accumulated.
    ($name:ident {} $vars:tt { $($field:ident)+ }) => (
        impl ::serde::Deserialize for $name {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer
            {
                $(let $field = try!(::serde::Deserialize::deserialize(deserializer));)+
                Ok($name($($field),+))
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: ::serde::Serializer
            {
                let $name($(ref $field),+) = *self;
                $(try!($field.serialize(serializer));)+
                Ok(())
            }
        }
    );
    // Skip 'pub'.
    ($name:ident { pub $($rest:tt)* } $vars:tt $acc:tt) => (
        define_struct_tuple_impls!($name { $($rest)* } $vars $acc);
    );
    // Accumulate a stock variable name for the field.
    (
        $name:ident { $ty:ty, $($rest:tt)* } { $next_var:ident $($var:ident)* }
        { $($acc:ident)* }
    ) => (
        define_struct_tuple_impls!(
            $name { $($rest)* } { $($var)* } { $($acc)* $next_var });
    );
}

macro_rules! variant_de_impl {
    ($deserializer:ident $name:ident $variant:ident {}) => (
        Ok($name::$variant)
    );
    ($deserializer:ident $name:ident $variant:ident { $($field:ident)+ }) => ({
        $(let $field = try!(::serde::Deserialize::deserialize($deserializer));)+
        Ok($name::$variant($($field),+))
    });
}

macro_rules! variant_ser_pat {
    ($name:ident $variant:ident {}) => ($name::$variant);
    ($name:ident $variant:ident { $($field:ident)+ }) => (
        $name::$variant($(ref $field)+)
    );
}

macro_rules! variant_ser_impl {
    ($serializer:ident $name:ident $variant:ident {}) => ();
    ($serializer:ident $name:ident $variant:ident { $($field:ident)+ }) => (
        $(try!(::serde::Serialize::serialize($field, $serializer));)+
    );
}

macro_rules! define_enum_impls {
    // Exit point, all variants have been accumulated.
    (
        {} $name:ident $count:tt {
            $({ $variant:ident $index:tt $fields:tt })+
        }
    ) => (
        #[allow(non_upper_case_globals)] 
        impl ::serde::Deserialize for $name {
            fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer
            {
                $(const $variant: u8 = expr!($index);)+
                match try!(u8::deserialize(deserializer)) {
                    $(
                        $variant => {
                            variant_de_impl!(deserializer $name $variant $fields)
                        },
                    )+
                    _ => {
                        Err(<D::Error as ::serde::Error>::unknown_variant(
                            stringify!("couldn't decode ", $name)))
                    }
                }
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: ::serde::Serializer
            {
                match *self {
                    $(
                        variant_ser_pat!($name $variant $fields) => {
                            try!(u8::serialize(&$index, serializer));
                            variant_ser_impl!(serializer $name $variant $fields);
                            Ok(())
                        },
                    )+
                }
            }
        }
    );
    // Skip attribute.
    ({ #[$attr:meta] $($rest:tt)* } $name:ident $count:tt $acc:tt) => (
        define_enum_impls!({ $($rest)* } $name $count $acc);
    );
    // Accumulate a unit variant.
    ({ $variant:ident, $($rest:tt)* } $name:ident $count:tt { $($acc:tt)* }) => (
        define_enum_impls!(
            { $($rest)* } $name (1 + $count) { $($acc)* { $variant $count {} } });
    );
    // Entry point for variant with fields.
    ({ $variant:ident($($ty:ty),+), $($rest:tt)* } $name:ident $count:tt $acc:tt) => (
        define_enum_impls!(@VARIANT
            { $($rest)* } $name $count $acc
            $variant($($ty,)+) { a b c d e f g h } {});
    );
    // Accumulate a variant with fields.
    (@VARIANT
        $rest:tt $name:ident $count:tt { $($acc:tt)* }
        $variant:ident() $vars:tt $fields:tt
    ) => (
        define_enum_impls!(
            $rest $name (1 + $count) { $($acc)* { $variant $count $fields } });
    );
    // Accumulate a field of a variant.
    (@VARIANT
        $rest:tt $name:ident $count:tt $acc:tt
        $variant:ident($ty:ty, $($ty_rest:tt)*) { $var:ident $($vars:ident)* }
        { $($fields:ident)* }
    ) => (
        define_enum_impls!(@VARIANT
            $rest $name $count $acc
            $variant($($ty_rest)*) { $($vars)* }
            { $($fields)* $var });
    );
}

macro_rules! define_type {
    ($(#[$attr:meta])* pub struct $name:ident { $($inner:tt)* }) => (
        item!($(#[$attr])* pub struct $name { $($inner)* });
        define_struct_impls!($name { $($inner)* });
    );
    ($(#[$attr:meta])* pub struct $name:ident($($inner:tt)*);) => (
        item!($(#[$attr])* pub struct $name($($inner)*););
        define_struct_tuple_impls!($name { $($inner)*, });
    );
    ($(#[$attr:meta])* pub enum $name:ident { $($inner:tt)* }) => (
        item!($(#[$attr])* pub enum $name { $($inner)* });
        define_enum_impls!({ $($inner)* } $name 0 {});
    )
}
