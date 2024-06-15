#[macro_export]
macro_rules! impl_model_readable {
    ($name:ident) => {
        use std::{
            ops::{Deref, DerefMut},
        };

        use crate::models::save_slot::face_data::models::Model;
        use crate::impl_binary_readable;

        #[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
        pub struct $name {
            pub model: Model,
        }

        impl Default for $name {
            fn default() -> Self {
              $name {
                    model: Model::default(),
                }
            }
        }

        impl Deref for $name {
            type Target = Model;

            fn deref(&self) -> &Self::Target {
                &self.model
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.model
            }
        }
        impl_binary_readable!($name, model, Model);
    };
}
