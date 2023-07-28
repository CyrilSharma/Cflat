#[allow(unused_macros)]
macro_rules! polymorphic_enum {
    ($name:ident => $variants:tt, Attributes => [$($attr:ident: $type:path),*]) => {
        polymorphic_enum!(@call_enum, $name, $variants);
        impl $name {$(
            pub fn $attr(&self) -> $type {
                polymorphic_enum!(@call_match, $name, self,
                    $attr, $variants);
            }
        )*}
    };
    (@call_match, $name:ident, $self:ident, $attr:ident,
        [$($variant:ident($type:path)),*]) => {
        return match $self {$(
            $name::$variant(a) => a.$attr,
        )*};
    };
    (@call_enum, $name:ident, [$($variant:ident($type:path)),*]) => {
        pub enum $name { $($variant($type)),* }
    };
}
pub(crate) use polymorphic_enum;