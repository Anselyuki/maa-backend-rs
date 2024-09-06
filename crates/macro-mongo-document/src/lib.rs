use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn mongo_document(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;
    let gen = quote::quote! {

        #input

        impl TryInto<bson::Document> for #name {
            type Error = bson::ser::Error;

            fn try_into(self) -> Result<bson::Document, Self::Error> {
                bson::ser::to_document(&self)
            }
        }

        impl TryFrom<bson::Document> for #name {
            type Error = bson::de::Error;

            fn try_from(doc: bson::Document) -> Result<Self, Self::Error> {
                bson::from_document(doc)
            }
        }
    };

    gen.into()
}
