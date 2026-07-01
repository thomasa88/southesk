#[macro_use]
extern crate southesk_macros;
fn main() {
    use rust_decimal::Decimal;
    use types::*;
    /// # Low-level API
    ///
    /// The following methods provides a direct mapping to the API
    /// provided by the MCP. They are less ergonomic than the high-level
    /// methods. Each method maps directly to a Montrose MCP tool of the
    /// same name.
    impl Client<Connected> {
        ///Low-level API. A simple tool.
        async fn low_simple_tool_long(
            &self,
            args: SimpleToolArgs<'_>,
        ) -> Result<Decimal, ClientCallError> {
            let json_args = serde_json::to_value(args)
                .map_err(|e| ClientCallError::InvalidArguments(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("Failed to serialize arguments: {0}", e),
                        )
                    }),
                ))?
                .as_object()
                .ok_or_else(|| ClientCallError::InvalidArguments(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("JSON argument is not an object"),
                        )
                    }),
                ))?
                .to_owned();
            #[allow(clippy::needless_question_mark)]
            Ok(
                self
                    .api_call::<SimpleToolReturn>("simple_tool", Some(json_args))
                    .await?
                    .output,
            )
        }
        /**Low-level API. A simple tool.

`input`: The input string.

*/
        pub async fn low_simple_tool<'arg>(
            &self,
            input: &'arg str,
        ) -> Result<Decimal, ClientCallError> {
            let args = SimpleToolArgs { input };
            self.low_simple_tool_long(args).await
        }
        ///Low-level API. Another tool.
        async fn low_other_tool_long(
            &self,
            args: OtherToolArgs,
        ) -> Result<Decimal, ClientCallError> {
            let json_args = serde_json::to_value(args)
                .map_err(|e| ClientCallError::InvalidArguments(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("Failed to serialize arguments: {0}", e),
                        )
                    }),
                ))?
                .as_object()
                .ok_or_else(|| ClientCallError::InvalidArguments(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("JSON argument is not an object"),
                        )
                    }),
                ))?
                .to_owned();
            #[allow(clippy::needless_question_mark)]
            Ok(
                self
                    .api_call::<OtherToolReturn>("other_tool", Some(json_args))
                    .await?
                    .output,
            )
        }
        /**Low-level API. Another tool.

`input`: The input value.

*/
        pub async fn low_other_tool(
            &self,
            input: OtherToolArgsInput,
        ) -> Result<Decimal, ClientCallError> {
            let args = OtherToolArgs { input };
            self.low_other_tool_long(args).await
        }
    }
    /// Montrose Low-level API types
    pub mod types {
        use rust_decimal::Decimal;
        ///Arguments for [`low_simple_tool`](crate::Client::low_simple_tool)
        #[serde(rename_all = "camelCase")]
        pub struct SimpleToolArgs<'arg> {
            ///The input string.
            pub input: &'arg str,
        }
        #[automatically_derived]
        impl<'arg> ::core::fmt::Debug for SimpleToolArgs<'arg> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "SimpleToolArgs",
                    "input",
                    &&self.input,
                )
            }
        }
        #[automatically_derived]
        impl<'arg> ::core::clone::Clone for SimpleToolArgs<'arg> {
            #[inline]
            fn clone(&self) -> SimpleToolArgs<'arg> {
                SimpleToolArgs {
                    input: ::core::clone::Clone::clone(&self.input),
                }
            }
        }
        #[automatically_derived]
        impl<'arg> ::core::marker::StructuralPartialEq for SimpleToolArgs<'arg> {}
        #[automatically_derived]
        impl<'arg> ::core::cmp::PartialEq for SimpleToolArgs<'arg> {
            #[inline]
            fn eq(&self, other: &SimpleToolArgs<'arg>) -> bool {
                self.input == other.input
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'arg> _serde::Serialize for SimpleToolArgs<'arg> {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "SimpleToolArgs",
                        false as usize + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "input",
                        &self.input,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        ///Return value for [`low_simple_tool`](crate::Client::low_simple_tool).
        #[serde(rename_all = "camelCase")]
        pub struct SimpleToolReturn {
            ///The output number.
            pub output: Decimal,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SimpleToolReturn {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "SimpleToolReturn",
                    "output",
                    &&self.output,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SimpleToolReturn {
            #[inline]
            fn clone(&self) -> SimpleToolReturn {
                SimpleToolReturn {
                    output: ::core::clone::Clone::clone(&self.output),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for SimpleToolReturn {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for SimpleToolReturn {
            #[inline]
            fn eq(&self, other: &SimpleToolReturn) -> bool {
                self.output == other.output
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for SimpleToolReturn {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "SimpleToolReturn",
                        false as usize + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "output",
                        &self.output,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for SimpleToolReturn {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private228::Ok(__Field::__field0),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "output" => _serde::__private228::Ok(__Field::__field0),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"output" => _serde::__private228::Ok(__Field::__field0),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private228::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private228::PhantomData<SimpleToolReturn>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = SimpleToolReturn;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct SimpleToolReturn",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                Decimal,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct SimpleToolReturn with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private228::Ok(SimpleToolReturn {
                                output: __field0,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private228::Option<Decimal> = _serde::__private228::None;
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private228::Option::is_some(&__field0) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("output"),
                                            );
                                        }
                                        __field0 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<Decimal>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private228::Some(__field0) => __field0,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("output")?
                                }
                            };
                            _serde::__private228::Ok(SimpleToolReturn {
                                output: __field0,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["output"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "SimpleToolReturn",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<
                                SimpleToolReturn,
                            >,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        ///The input value.
        pub enum OtherToolArgsInput {
            Value1,
            Value2,
            Value3,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for OtherToolArgsInput {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        OtherToolArgsInput::Value1 => "Value1",
                        OtherToolArgsInput::Value2 => "Value2",
                        OtherToolArgsInput::Value3 => "Value3",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for OtherToolArgsInput {
            #[inline]
            fn clone(&self) -> OtherToolArgsInput {
                match self {
                    OtherToolArgsInput::Value1 => OtherToolArgsInput::Value1,
                    OtherToolArgsInput::Value2 => OtherToolArgsInput::Value2,
                    OtherToolArgsInput::Value3 => OtherToolArgsInput::Value3,
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for OtherToolArgsInput {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for OtherToolArgsInput {
            #[inline]
            fn eq(&self, other: &OtherToolArgsInput) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for OtherToolArgsInput {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        OtherToolArgsInput::Value1 => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "OtherToolArgsInput",
                                0u32,
                                "Value1",
                            )
                        }
                        OtherToolArgsInput::Value2 => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "OtherToolArgsInput",
                                1u32,
                                "Value2",
                            )
                        }
                        OtherToolArgsInput::Value3 => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "OtherToolArgsInput",
                                2u32,
                                "Value3",
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for OtherToolArgsInput {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private228::Ok(__Field::__field0),
                                1u64 => _serde::__private228::Ok(__Field::__field1),
                                2u64 => _serde::__private228::Ok(__Field::__field2),
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 3",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Value1" => _serde::__private228::Ok(__Field::__field0),
                                "Value2" => _serde::__private228::Ok(__Field::__field1),
                                "Value3" => _serde::__private228::Ok(__Field::__field2),
                                _ => {
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Value1" => _serde::__private228::Ok(__Field::__field0),
                                b"Value2" => _serde::__private228::Ok(__Field::__field1),
                                b"Value3" => _serde::__private228::Ok(__Field::__field2),
                                _ => {
                                    let __value = &_serde::__private228::from_utf8_lossy(
                                        __value,
                                    );
                                    _serde::__private228::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private228::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private228::PhantomData<OtherToolArgsInput>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = OtherToolArgsInput;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "enum OtherToolArgsInput",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private228::Ok(OtherToolArgsInput::Value1)
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private228::Ok(OtherToolArgsInput::Value2)
                                }
                                (__Field::__field2, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private228::Ok(OtherToolArgsInput::Value3)
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &[
                        "Value1",
                        "Value2",
                        "Value3",
                    ];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "OtherToolArgsInput",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<
                                OtherToolArgsInput,
                            >,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
        ///Arguments for [`low_other_tool`](crate::Client::low_other_tool)
        #[serde(rename_all = "camelCase")]
        pub struct OtherToolArgs {
            ///The input value.
            pub input: OtherToolArgsInput,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for OtherToolArgs {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "OtherToolArgs",
                    "input",
                    &&self.input,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for OtherToolArgs {
            #[inline]
            fn clone(&self) -> OtherToolArgs {
                OtherToolArgs {
                    input: ::core::clone::Clone::clone(&self.input),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for OtherToolArgs {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for OtherToolArgs {
            #[inline]
            fn eq(&self, other: &OtherToolArgs) -> bool {
                self.input == other.input
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for OtherToolArgs {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "OtherToolArgs",
                        false as usize + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "input",
                        &self.input,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        ///Return value for [`low_other_tool`](crate::Client::low_other_tool).
        #[serde(rename_all = "camelCase")]
        pub struct OtherToolReturn {
            ///The output number.
            pub output: Decimal,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for OtherToolReturn {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "OtherToolReturn",
                    "output",
                    &&self.output,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for OtherToolReturn {
            #[inline]
            fn clone(&self) -> OtherToolReturn {
                OtherToolReturn {
                    output: ::core::clone::Clone::clone(&self.output),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for OtherToolReturn {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for OtherToolReturn {
            #[inline]
            fn eq(&self, other: &OtherToolReturn) -> bool {
                self.output == other.output
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for OtherToolReturn {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private228::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "OtherToolReturn",
                        false as usize + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "output",
                        &self.output,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for OtherToolReturn {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private228::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private228::Ok(__Field::__field0),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "output" => _serde::__private228::Ok(__Field::__field0),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private228::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"output" => _serde::__private228::Ok(__Field::__field0),
                                _ => _serde::__private228::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private228::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private228::PhantomData<OtherToolReturn>,
                        lifetime: _serde::__private228::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = OtherToolReturn;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private228::Formatter,
                        ) -> _serde::__private228::fmt::Result {
                            _serde::__private228::Formatter::write_str(
                                __formatter,
                                "struct OtherToolReturn",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                Decimal,
                            >(&mut __seq)? {
                                _serde::__private228::Some(__value) => __value,
                                _serde::__private228::None => {
                                    return _serde::__private228::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct OtherToolReturn with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private228::Ok(OtherToolReturn {
                                output: __field0,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private228::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private228::Option<Decimal> = _serde::__private228::None;
                            while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private228::Option::is_some(&__field0) {
                                            return _serde::__private228::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("output"),
                                            );
                                        }
                                        __field0 = _serde::__private228::Some(
                                            _serde::de::MapAccess::next_value::<Decimal>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private228::Some(__field0) => __field0,
                                _serde::__private228::None => {
                                    _serde::__private228::de::missing_field("output")?
                                }
                            };
                            _serde::__private228::Ok(OtherToolReturn {
                                output: __field0,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["output"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "OtherToolReturn",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private228::PhantomData::<OtherToolReturn>,
                            lifetime: _serde::__private228::PhantomData,
                        },
                    )
                }
            }
        };
    }
}
