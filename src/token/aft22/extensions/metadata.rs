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
    aft22,
    aft22::extensions::metadata,
    traits::aft22::{extensions::metadata::*, *},
};
pub use aft22::{AFT22Impl, Internal as _, InternalImpl as _};
use openbrush::traits::Storage;
pub use openbrush::traits::String;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub name: Option<String>,
    #[lazy]
    pub symbol: Option<String>,
    #[lazy]
    pub decimals: u8,
}

pub trait AFT22MetadataImpl: Storage<Data> {
    fn token_name(&self) -> Option<String> {
        self.data().name.get_or_default()
    }

    fn token_symbol(&self) -> Option<String> {
        self.data().symbol.get_or_default()
    }

    fn token_decimals(&self) -> u8 {
        self.data().decimals.get_or_default()
    }
}
