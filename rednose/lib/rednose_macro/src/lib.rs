// Copyright 2025 North Pole Security, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use parse::Table;
use proc_macro::TokenStream;
use quote::quote;

mod generate;
mod parse;

/// This macro enables #[arrow_table]. See rednose::schema for more
/// information and the Trait definition.
#[proc_macro_attribute]
pub fn arrow_table(_: TokenStream, input: TokenStream) -> TokenStream {
    let table = Table::parse(input.into()).unwrap();

    let struct_table = generate::structs::table(&table);
    let impl_table = generate::impls::table(&table);
    let impl_arrow_table_trait = generate::impls::arrow_table_trait(&table);

    let struct_table_builder = generate::structs::table_builder(&table);
    let impl_table_builder = generate::impls::table_builder(&table);
    let impl_table_builder_trait = generate::impls::table_builder_trait(&table);

    let code = quote! {
        #[allow(unused_imports)]
        use arrow::datatypes::{DataType, Field, Schema, TimeUnit};
        #[allow(unused_imports)]
        use arrow::array::{
            Array, ArrayBuilder, BooleanBuilder, GenericStringArray, Int64Array,
            Int64Builder, StringArray, StringBuilder, StructArray, StructBuilder,
            Float64Array, Float64Builder, TimestampMicrosecondArray, TimestampMicrosecondBuilder,
            ListBuilder, RecordBatch
        };
        #[allow(unused_imports)]
        use arrow::record_batch::RecordBatch;
        #[allow(unused_imports)]
        use arrow::error::ArrowError;

        #struct_table

        #impl_table

        #impl_arrow_table_trait

        #struct_table_builder

        #impl_table_builder

        #impl_table_builder_trait
    };
    code.into()
}
