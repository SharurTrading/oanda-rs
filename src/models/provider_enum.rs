//! Declaration macro for provider string enums with an unknown-spelling arm.

// Each spelling is declared once, alongside its public variant documentation.
macro_rules! provider_enum {
    (
        $(#[$meta:meta])* $visibility:vis enum $name:ident {
            $($(#[$variant_meta:meta])* $variant:ident => $wire:literal,)*
            ;
            $(#[$unknown_meta:meta])* Unknown(String),
        }
    ) => {
        $(#[$meta])*
        $visibility enum $name {
            $($(#[$variant_meta])* $variant,)*
            $(#[$unknown_meta])* Unknown(String),
        }

        impl $name {
            /// Return the exact provider spelling.
            #[must_use]
            pub fn as_str(&self) -> &str {
                match self {
                    $(Self::$variant => $wire,)*
                    Self::Unknown(value) => value,
                }
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value = <String as serde::Deserialize>::deserialize(deserializer)?;
                Ok(match value.as_str() {
                    $($wire => Self::$variant,)*
                    _ => Self::Unknown(value),
                })
            }
        }
    };
}
pub(crate) use provider_enum;
