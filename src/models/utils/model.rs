use crate::graph::context::Context;

pub struct DeserBuilder<'a, T> {
    builder: postgrest::Builder<'a>,
    _phantom: std::marker::PhantomData<T>
}

impl<'a, T> core::ops::Deref for DeserBuilder<'a, T> {
    type Target = postgrest::Builder<'a>;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

pub trait Model: serde::de::DeserializeOwned + serde::Serialize {
    fn collection_name() -> String;
    fn builder<'a>(context: &'a Context) -> DeserBuilder<'a, Self> {
        DeserBuilder {
            builder: context.postgrest_client
                .from(Self::collection_name())
                .auth(context.postgrest_jwt.as_str()),
            _phantom: std::marker::PhantomData
        }
    }
}