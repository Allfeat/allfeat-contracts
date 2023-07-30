// Copyright (c) 2022-2023 Allfeat labs
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use crate::{
    aft34,
    aft34::extensions::metadata,
    traits::aft34::{extensions::metadata::*, *},
};
pub use aft34::{
    AFT34Impl, BalancesManager as _, Internal as _, InternalImpl as _, Operator, Owner,
};
pub use metadata::Internal as _;
pub use openbrush::traits::String;
use openbrush::{
    storage::{Mapping, TypeGuard},
    traits::Storage,
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub attributes: Mapping<(Id, String), String, AttributesKey>,
}

pub struct AttributesKey;

impl<'a> TypeGuard<'a> for AttributesKey {
    type Type = &'a (&'a Id, &'a String);
}

pub trait AFT34MetadataImpl: Storage<Data> {
    fn get_attribute(&self, id: Id, key: String) -> Option<String> {
        self.data().attributes.get(&(&id, &key))
    }
}

pub trait Internal {
    /// Event is emitted when an attribute is set for a token.
    fn _emit_attribute_set_event(&self, id: Id, key: String, data: String);

    fn _set_attribute(&mut self, id: Id, key: String, value: String);
}

pub trait InternalImpl: Internal + Storage<Data> {
    fn _emit_attribute_set_event(&self, _id: Id, _key: String, _data: String) {}

    fn _set_attribute(&mut self, id: Id, key: String, value: String) {
        self.data().attributes.insert(&(&id, &key), &value);
        Internal::_emit_attribute_set_event(self, id, key, value);
    }
}
