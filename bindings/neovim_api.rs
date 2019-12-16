// Auto generated {{date}}
use crate::runtime::AsyncWrite;
use crate::{Buffer, Window, Tabpage};

use crate::neovim::*;
use crate::rpc::*;
use crate::callerror::{map_generic_error, CallError2};

fn map_result<T: FromVal<Value>>(val: Value) -> T {
    T::from_val(val)
}

{% for etype in exttypes %}

impl<W> {{ etype.name }}<W>
  where W: AsyncWrite + Send + Sync + Unpin + 'static
  {
    pub fn new(code_data: Value, requester: Requester<W>) -> {{ etype.name }}<W>
    {
        {{ etype.name }} {
            code_data,
            requester
        }
    }

    /// Internal value, that represent type
    pub fn get_value(&self) -> &Value {
        &self.code_data
    }

    {% for f in functions if f.ext and f.name.startswith(etype.prefix) %}
    /// since: {{f.since}}
    pub async fn {{f.name|replace(etype.prefix, '')}}(&self, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, Box<CallError2>>
    {
        match self.requester.call("{{f.name}}",
                          call_args![self.code_data.clone()
                          {% if f.parameters|count > 0 %}
                          , {{ f.parameters|map(attribute = "name")|join(", ") }}
                          {% endif %}
                          ])
                    .await?
                  {
                    Ok(val) => Ok(map_result(val)),
                    Err(val) => Err(map_generic_error(val))?,
                  }
    }
    {% endfor %}
}

{% endfor %}


impl<W> Requester<W>
where
      W: AsyncWrite + Send + Sync + Unpin + 'static,
{
    {% for f in functions if not f.ext %}
    pub async fn {{f.name|replace('nvim_', '')}}(&self, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, Box<CallError2>> {
        match self.call("{{f.name}}",
                          call_args![{{ f.parameters|map(attribute = "name")|join(", ") }}])
                    .await?
                  {
                    Ok(val) => Ok(map_result(val)),
                    Err(val) => Err(map_generic_error(val))?,
                  }
    }

    {% endfor %}
}

impl<W> Neovim<W>
where
      W: AsyncWrite + Send + Sync + Unpin + 'static,
{
    {% for f in functions if not f.ext %}
    pub async fn {{f.name|replace('nvim_', '')}}(&self, {{f.argstring}}) -> Result<{{f.return_type.native_type_ret}}, Box<CallError2>> {
      // TODO: This will clone always, make it a ref
        self.requester().{{f.name|replace('nvim_', '')}}({{f.callstring}}).await
    }

    {% endfor %}
}