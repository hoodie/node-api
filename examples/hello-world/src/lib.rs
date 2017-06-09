#![feature(link_args)]
#[macro_use]
extern crate node_api;

use node_api::{NapiEnv, NapiValue, FromNapiValues, IntoNapiValue};
use node_api::{create_function, set_named_property, create_object};
use node_api::error::*;

register!{
helloworld
    export add;
    export hello;
}

fn add(_: NapiEnv, _: NapiValue, a: u64) -> u64 {
    a + a
}

fn hello(_: NapiEnv, _: NapiValue, args: HelloArgs) -> HelloReturn {
    HelloReturn {
        foo: "HELLO".to_string(),
        bar: 23,
    }
}


struct HelloArgs {}
impl FromNapiValues for HelloArgs {
    fn from_napi_values(_: NapiEnv, _: NapiValue, _: &[NapiValue]) -> Result<Self> {
        Ok(HelloArgs {})
    }
}

struct HelloReturn {
    pub foo: String,
    pub bar: u64,
}

impl IntoNapiValue for HelloReturn {
    fn into_napi_value(self, env: NapiEnv) -> Result<NapiValue> {
        let object = create_object(env)?;
        let foo = self.foo.into_napi_value(env)?;
        let bar = self.bar.into_napi_value(env)?;
        set_named_property(env, object, "foo", foo)?;
        set_named_property(env, object, "bar", bar)?;
        Ok(object)
    }
}
