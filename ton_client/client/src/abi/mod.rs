/*
* Copyright 2018-2020 TON DEV SOLUTIONS LTD.
*
* Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
* this file except in compliance with the License.
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific TON DEV software governing permissions and
* limitations under the License.
*/

use crate::dispatch::DispatchTable;

#[cfg(test)]
mod tests;

mod internal;
mod abi;
mod decode;
mod defaults;
mod encode;
mod signing;
mod errors;

pub use abi::{Abi, AbiHandle};
pub use decode::{
    decode_message, MessageContentType, ParamsOfDecodeMessage, ResultOfDecodeMessage,
};
pub use encode::{
    attach_signature, encode_message, ParamsOfAttachSignature, ParamsOfEncodeMessage,
    ResultOfAttachSignature, ResultOfEncodeMessage,
};
pub use signing::MessageSigning;
pub use errors::{ErrorCode, Error};

pub(crate) fn register(handlers: &mut DispatchTable) {
    handlers.call("abi.encode_message", encode::encode_message);
    handlers.call("abi.attach_signature", encode::attach_signature);
    handlers.call("abi.decode_message", decode::decode_message);
}
