// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct CGCStorePurchaseInit_LineItem {
    // message fields
    item_def_id: ::std::option::Option<u32>,
    quantity: ::std::option::Option<u32>,
    cost_in_local_currency: ::std::option::Option<u32>,
    purchase_type: ::std::option::Option<u32>,
    source_reference_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCStorePurchaseInit_LineItem {}

impl CGCStorePurchaseInit_LineItem {
    pub fn new() -> CGCStorePurchaseInit_LineItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCStorePurchaseInit_LineItem {
        static mut instance: ::protobuf::lazy::Lazy<CGCStorePurchaseInit_LineItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCStorePurchaseInit_LineItem,
        };
        unsafe {
            instance.get(CGCStorePurchaseInit_LineItem::new)
        }
    }

    // optional uint32 item_def_id = 1;

    pub fn clear_item_def_id(&mut self) {
        self.item_def_id = ::std::option::Option::None;
    }

    pub fn has_item_def_id(&self) -> bool {
        self.item_def_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_def_id(&mut self, v: u32) {
        self.item_def_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_def_id(&self) -> u32 {
        self.item_def_id.unwrap_or(0)
    }

    fn get_item_def_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_def_id
    }

    fn mut_item_def_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_def_id
    }

    // optional uint32 quantity = 2;

    pub fn clear_quantity(&mut self) {
        self.quantity = ::std::option::Option::None;
    }

    pub fn has_quantity(&self) -> bool {
        self.quantity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quantity(&mut self, v: u32) {
        self.quantity = ::std::option::Option::Some(v);
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity.unwrap_or(0)
    }

    fn get_quantity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quantity
    }

    fn mut_quantity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quantity
    }

    // optional uint32 cost_in_local_currency = 3;

    pub fn clear_cost_in_local_currency(&mut self) {
        self.cost_in_local_currency = ::std::option::Option::None;
    }

    pub fn has_cost_in_local_currency(&self) -> bool {
        self.cost_in_local_currency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cost_in_local_currency(&mut self, v: u32) {
        self.cost_in_local_currency = ::std::option::Option::Some(v);
    }

    pub fn get_cost_in_local_currency(&self) -> u32 {
        self.cost_in_local_currency.unwrap_or(0)
    }

    fn get_cost_in_local_currency_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.cost_in_local_currency
    }

    fn mut_cost_in_local_currency_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.cost_in_local_currency
    }

    // optional uint32 purchase_type = 4;

    pub fn clear_purchase_type(&mut self) {
        self.purchase_type = ::std::option::Option::None;
    }

    pub fn has_purchase_type(&self) -> bool {
        self.purchase_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_purchase_type(&mut self, v: u32) {
        self.purchase_type = ::std::option::Option::Some(v);
    }

    pub fn get_purchase_type(&self) -> u32 {
        self.purchase_type.unwrap_or(0)
    }

    fn get_purchase_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.purchase_type
    }

    fn mut_purchase_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.purchase_type
    }

    // optional uint64 source_reference_id = 5;

    pub fn clear_source_reference_id(&mut self) {
        self.source_reference_id = ::std::option::Option::None;
    }

    pub fn has_source_reference_id(&self) -> bool {
        self.source_reference_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_reference_id(&mut self, v: u64) {
        self.source_reference_id = ::std::option::Option::Some(v);
    }

    pub fn get_source_reference_id(&self) -> u64 {
        self.source_reference_id.unwrap_or(0)
    }

    fn get_source_reference_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.source_reference_id
    }

    fn mut_source_reference_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.source_reference_id
    }
}

impl ::protobuf::Message for CGCStorePurchaseInit_LineItem {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.item_def_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quantity = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.cost_in_local_currency = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.purchase_type = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.source_reference_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.item_def_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quantity {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.cost_in_local_currency {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.purchase_type {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.source_reference_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_def_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.quantity {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.cost_in_local_currency {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.purchase_type {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.source_reference_id {
            os.write_uint64(5, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCStorePurchaseInit_LineItem {
    fn new() -> CGCStorePurchaseInit_LineItem {
        CGCStorePurchaseInit_LineItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCStorePurchaseInit_LineItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_def_id",
                    CGCStorePurchaseInit_LineItem::get_item_def_id_for_reflect,
                    CGCStorePurchaseInit_LineItem::mut_item_def_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quantity",
                    CGCStorePurchaseInit_LineItem::get_quantity_for_reflect,
                    CGCStorePurchaseInit_LineItem::mut_quantity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cost_in_local_currency",
                    CGCStorePurchaseInit_LineItem::get_cost_in_local_currency_for_reflect,
                    CGCStorePurchaseInit_LineItem::mut_cost_in_local_currency_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "purchase_type",
                    CGCStorePurchaseInit_LineItem::get_purchase_type_for_reflect,
                    CGCStorePurchaseInit_LineItem::mut_purchase_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "source_reference_id",
                    CGCStorePurchaseInit_LineItem::get_source_reference_id_for_reflect,
                    CGCStorePurchaseInit_LineItem::mut_source_reference_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCStorePurchaseInit_LineItem>(
                    "CGCStorePurchaseInit_LineItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCStorePurchaseInit_LineItem {
    fn clear(&mut self) {
        self.clear_item_def_id();
        self.clear_quantity();
        self.clear_cost_in_local_currency();
        self.clear_purchase_type();
        self.clear_source_reference_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCStorePurchaseInit_LineItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCStorePurchaseInit_LineItem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCStorePurchaseInit {
    // message fields
    country: ::protobuf::SingularField<::std::string::String>,
    language: ::std::option::Option<i32>,
    currency: ::std::option::Option<i32>,
    line_items: ::protobuf::RepeatedField<CGCStorePurchaseInit_LineItem>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCStorePurchaseInit {}

impl CMsgGCStorePurchaseInit {
    pub fn new() -> CMsgGCStorePurchaseInit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCStorePurchaseInit {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCStorePurchaseInit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCStorePurchaseInit,
        };
        unsafe {
            instance.get(CMsgGCStorePurchaseInit::new)
        }
    }

    // optional string country = 1;

    pub fn clear_country(&mut self) {
        self.country.clear();
    }

    pub fn has_country(&self) -> bool {
        self.country.is_some()
    }

    // Param is passed by value, moved
    pub fn set_country(&mut self, v: ::std::string::String) {
        self.country = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_country(&mut self) -> &mut ::std::string::String {
        if self.country.is_none() {
            self.country.set_default();
        }
        self.country.as_mut().unwrap()
    }

    // Take field
    pub fn take_country(&mut self) -> ::std::string::String {
        self.country.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_country(&self) -> &str {
        match self.country.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_country_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.country
    }

    fn mut_country_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.country
    }

    // optional int32 language = 2;

    pub fn clear_language(&mut self) {
        self.language = ::std::option::Option::None;
    }

    pub fn has_language(&self) -> bool {
        self.language.is_some()
    }

    // Param is passed by value, moved
    pub fn set_language(&mut self, v: i32) {
        self.language = ::std::option::Option::Some(v);
    }

    pub fn get_language(&self) -> i32 {
        self.language.unwrap_or(0)
    }

    fn get_language_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.language
    }

    fn mut_language_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.language
    }

    // optional int32 currency = 3;

    pub fn clear_currency(&mut self) {
        self.currency = ::std::option::Option::None;
    }

    pub fn has_currency(&self) -> bool {
        self.currency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currency(&mut self, v: i32) {
        self.currency = ::std::option::Option::Some(v);
    }

    pub fn get_currency(&self) -> i32 {
        self.currency.unwrap_or(0)
    }

    fn get_currency_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.currency
    }

    fn mut_currency_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.currency
    }

    // repeated .CGCStorePurchaseInit_LineItem line_items = 4;

    pub fn clear_line_items(&mut self) {
        self.line_items.clear();
    }

    // Param is passed by value, moved
    pub fn set_line_items(&mut self, v: ::protobuf::RepeatedField<CGCStorePurchaseInit_LineItem>) {
        self.line_items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_line_items(&mut self) -> &mut ::protobuf::RepeatedField<CGCStorePurchaseInit_LineItem> {
        &mut self.line_items
    }

    // Take field
    pub fn take_line_items(&mut self) -> ::protobuf::RepeatedField<CGCStorePurchaseInit_LineItem> {
        ::std::mem::replace(&mut self.line_items, ::protobuf::RepeatedField::new())
    }

    pub fn get_line_items(&self) -> &[CGCStorePurchaseInit_LineItem] {
        &self.line_items
    }

    fn get_line_items_for_reflect(&self) -> &::protobuf::RepeatedField<CGCStorePurchaseInit_LineItem> {
        &self.line_items
    }

    fn mut_line_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CGCStorePurchaseInit_LineItem> {
        &mut self.line_items
    }
}

impl ::protobuf::Message for CMsgGCStorePurchaseInit {
    fn is_initialized(&self) -> bool {
        for v in &self.line_items {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.country)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.language = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.currency = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.line_items)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.country.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.language {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.currency {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.line_items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.country.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.language {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.currency {
            os.write_int32(3, v)?;
        }
        for v in &self.line_items {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCStorePurchaseInit {
    fn new() -> CMsgGCStorePurchaseInit {
        CMsgGCStorePurchaseInit::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCStorePurchaseInit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "country",
                    CMsgGCStorePurchaseInit::get_country_for_reflect,
                    CMsgGCStorePurchaseInit::mut_country_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "language",
                    CMsgGCStorePurchaseInit::get_language_for_reflect,
                    CMsgGCStorePurchaseInit::mut_language_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "currency",
                    CMsgGCStorePurchaseInit::get_currency_for_reflect,
                    CMsgGCStorePurchaseInit::mut_currency_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CGCStorePurchaseInit_LineItem>>(
                    "line_items",
                    CMsgGCStorePurchaseInit::get_line_items_for_reflect,
                    CMsgGCStorePurchaseInit::mut_line_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCStorePurchaseInit>(
                    "CMsgGCStorePurchaseInit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCStorePurchaseInit {
    fn clear(&mut self) {
        self.clear_country();
        self.clear_language();
        self.clear_currency();
        self.clear_line_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCStorePurchaseInit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCStorePurchaseInit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCStorePurchaseInitResponse {
    // message fields
    result: ::std::option::Option<i32>,
    txn_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCStorePurchaseInitResponse {}

impl CMsgGCStorePurchaseInitResponse {
    pub fn new() -> CMsgGCStorePurchaseInitResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCStorePurchaseInitResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCStorePurchaseInitResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCStorePurchaseInitResponse,
        };
        unsafe {
            instance.get(CMsgGCStorePurchaseInitResponse::new)
        }
    }

    // optional int32 result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: i32) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> i32 {
        self.result.unwrap_or(0)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.result
    }

    // optional uint64 txn_id = 2;

    pub fn clear_txn_id(&mut self) {
        self.txn_id = ::std::option::Option::None;
    }

    pub fn has_txn_id(&self) -> bool {
        self.txn_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txn_id(&mut self, v: u64) {
        self.txn_id = ::std::option::Option::Some(v);
    }

    pub fn get_txn_id(&self) -> u64 {
        self.txn_id.unwrap_or(0)
    }

    fn get_txn_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.txn_id
    }

    fn mut_txn_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.txn_id
    }
}

impl ::protobuf::Message for CMsgGCStorePurchaseInitResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.txn_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.txn_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.txn_id {
            os.write_uint64(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCStorePurchaseInitResponse {
    fn new() -> CMsgGCStorePurchaseInitResponse {
        CMsgGCStorePurchaseInitResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCStorePurchaseInitResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "result",
                    CMsgGCStorePurchaseInitResponse::get_result_for_reflect,
                    CMsgGCStorePurchaseInitResponse::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "txn_id",
                    CMsgGCStorePurchaseInitResponse::get_txn_id_for_reflect,
                    CMsgGCStorePurchaseInitResponse::mut_txn_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCStorePurchaseInitResponse>(
                    "CMsgGCStorePurchaseInitResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCStorePurchaseInitResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_txn_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCStorePurchaseInitResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCStorePurchaseInitResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSystemBroadcast {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSystemBroadcast {}

impl CMsgSystemBroadcast {
    pub fn new() -> CMsgSystemBroadcast {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSystemBroadcast {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSystemBroadcast> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSystemBroadcast,
        };
        unsafe {
            instance.get(CMsgSystemBroadcast::new)
        }
    }

    // optional string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message
    }
}

impl ::protobuf::Message for CMsgSystemBroadcast {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(1, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgSystemBroadcast {
    fn new() -> CMsgSystemBroadcast {
        CMsgSystemBroadcast::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSystemBroadcast>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    CMsgSystemBroadcast::get_message_for_reflect,
                    CMsgSystemBroadcast::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSystemBroadcast>(
                    "CMsgSystemBroadcast",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSystemBroadcast {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSystemBroadcast {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSystemBroadcast {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientPingData {
    // message fields
    relay_codes: ::std::vec::Vec<u32>,
    relay_pings: ::std::vec::Vec<u32>,
    region_codes: ::std::vec::Vec<u32>,
    region_pings: ::std::vec::Vec<u32>,
    region_ping_failed_bitmask: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientPingData {}

impl CMsgClientPingData {
    pub fn new() -> CMsgClientPingData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientPingData {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientPingData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientPingData,
        };
        unsafe {
            instance.get(CMsgClientPingData::new)
        }
    }

    // repeated fixed32 relay_codes = 4;

    pub fn clear_relay_codes(&mut self) {
        self.relay_codes.clear();
    }

    // Param is passed by value, moved
    pub fn set_relay_codes(&mut self, v: ::std::vec::Vec<u32>) {
        self.relay_codes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_relay_codes(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.relay_codes
    }

    // Take field
    pub fn take_relay_codes(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.relay_codes, ::std::vec::Vec::new())
    }

    pub fn get_relay_codes(&self) -> &[u32] {
        &self.relay_codes
    }

    fn get_relay_codes_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.relay_codes
    }

    fn mut_relay_codes_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.relay_codes
    }

    // repeated uint32 relay_pings = 5;

    pub fn clear_relay_pings(&mut self) {
        self.relay_pings.clear();
    }

    // Param is passed by value, moved
    pub fn set_relay_pings(&mut self, v: ::std::vec::Vec<u32>) {
        self.relay_pings = v;
    }

    // Mutable pointer to the field.
    pub fn mut_relay_pings(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.relay_pings
    }

    // Take field
    pub fn take_relay_pings(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.relay_pings, ::std::vec::Vec::new())
    }

    pub fn get_relay_pings(&self) -> &[u32] {
        &self.relay_pings
    }

    fn get_relay_pings_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.relay_pings
    }

    fn mut_relay_pings_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.relay_pings
    }

    // repeated uint32 region_codes = 8;

    pub fn clear_region_codes(&mut self) {
        self.region_codes.clear();
    }

    // Param is passed by value, moved
    pub fn set_region_codes(&mut self, v: ::std::vec::Vec<u32>) {
        self.region_codes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_region_codes(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.region_codes
    }

    // Take field
    pub fn take_region_codes(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.region_codes, ::std::vec::Vec::new())
    }

    pub fn get_region_codes(&self) -> &[u32] {
        &self.region_codes
    }

    fn get_region_codes_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.region_codes
    }

    fn mut_region_codes_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.region_codes
    }

    // repeated uint32 region_pings = 9;

    pub fn clear_region_pings(&mut self) {
        self.region_pings.clear();
    }

    // Param is passed by value, moved
    pub fn set_region_pings(&mut self, v: ::std::vec::Vec<u32>) {
        self.region_pings = v;
    }

    // Mutable pointer to the field.
    pub fn mut_region_pings(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.region_pings
    }

    // Take field
    pub fn take_region_pings(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.region_pings, ::std::vec::Vec::new())
    }

    pub fn get_region_pings(&self) -> &[u32] {
        &self.region_pings
    }

    fn get_region_pings_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.region_pings
    }

    fn mut_region_pings_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.region_pings
    }

    // optional uint32 region_ping_failed_bitmask = 10;

    pub fn clear_region_ping_failed_bitmask(&mut self) {
        self.region_ping_failed_bitmask = ::std::option::Option::None;
    }

    pub fn has_region_ping_failed_bitmask(&self) -> bool {
        self.region_ping_failed_bitmask.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_ping_failed_bitmask(&mut self, v: u32) {
        self.region_ping_failed_bitmask = ::std::option::Option::Some(v);
    }

    pub fn get_region_ping_failed_bitmask(&self) -> u32 {
        self.region_ping_failed_bitmask.unwrap_or(0)
    }

    fn get_region_ping_failed_bitmask_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.region_ping_failed_bitmask
    }

    fn mut_region_ping_failed_bitmask_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.region_ping_failed_bitmask
    }
}

impl ::protobuf::Message for CMsgClientPingData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                4 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.relay_codes)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.relay_pings)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.region_codes)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.region_pings)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.region_ping_failed_bitmask = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.relay_codes.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.relay_codes.len() as u32) + (self.relay_codes.len() * 4) as u32;
        }
        if !self.relay_pings.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(5, &self.relay_pings);
        }
        if !self.region_codes.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(8, &self.region_codes);
        }
        if !self.region_pings.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(9, &self.region_pings);
        }
        if let Some(v) = self.region_ping_failed_bitmask {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.relay_codes.is_empty() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.relay_codes.len() * 4) as u32)?;
            for v in &self.relay_codes {
                os.write_fixed32_no_tag(*v)?;
            };
        }
        if !self.relay_pings.is_empty() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.relay_pings))?;
            for v in &self.relay_pings {
                os.write_uint32_no_tag(*v)?;
            };
        }
        if !self.region_codes.is_empty() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.region_codes))?;
            for v in &self.region_codes {
                os.write_uint32_no_tag(*v)?;
            };
        }
        if !self.region_pings.is_empty() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.region_pings))?;
            for v in &self.region_pings {
                os.write_uint32_no_tag(*v)?;
            };
        }
        if let Some(v) = self.region_ping_failed_bitmask {
            os.write_uint32(10, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgClientPingData {
    fn new() -> CMsgClientPingData {
        CMsgClientPingData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientPingData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "relay_codes",
                    CMsgClientPingData::get_relay_codes_for_reflect,
                    CMsgClientPingData::mut_relay_codes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "relay_pings",
                    CMsgClientPingData::get_relay_pings_for_reflect,
                    CMsgClientPingData::mut_relay_pings_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "region_codes",
                    CMsgClientPingData::get_region_codes_for_reflect,
                    CMsgClientPingData::mut_region_codes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "region_pings",
                    CMsgClientPingData::get_region_pings_for_reflect,
                    CMsgClientPingData::mut_region_pings_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "region_ping_failed_bitmask",
                    CMsgClientPingData::get_region_ping_failed_bitmask_for_reflect,
                    CMsgClientPingData::mut_region_ping_failed_bitmask_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientPingData>(
                    "CMsgClientPingData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientPingData {
    fn clear(&mut self) {
        self.clear_relay_codes();
        self.clear_relay_pings();
        self.clear_region_codes();
        self.clear_region_pings();
        self.clear_region_ping_failed_bitmask();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientPingData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientPingData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgInviteToParty {
    // message fields
    steam_id: ::std::option::Option<u64>,
    client_version: ::std::option::Option<u32>,
    team_id: ::std::option::Option<u32>,
    as_coach: ::std::option::Option<bool>,
    ping_data: ::protobuf::SingularPtrField<CMsgClientPingData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgInviteToParty {}

impl CMsgInviteToParty {
    pub fn new() -> CMsgInviteToParty {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgInviteToParty {
        static mut instance: ::protobuf::lazy::Lazy<CMsgInviteToParty> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgInviteToParty,
        };
        unsafe {
            instance.get(CMsgInviteToParty::new)
        }
    }

    // optional fixed64 steam_id = 1;

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    fn get_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steam_id
    }

    // optional uint32 client_version = 2;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }

    // optional uint32 team_id = 3;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
    }

    // optional bool as_coach = 4;

    pub fn clear_as_coach(&mut self) {
        self.as_coach = ::std::option::Option::None;
    }

    pub fn has_as_coach(&self) -> bool {
        self.as_coach.is_some()
    }

    // Param is passed by value, moved
    pub fn set_as_coach(&mut self, v: bool) {
        self.as_coach = ::std::option::Option::Some(v);
    }

    pub fn get_as_coach(&self) -> bool {
        self.as_coach.unwrap_or(false)
    }

    fn get_as_coach_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.as_coach
    }

    fn mut_as_coach_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.as_coach
    }

    // optional .CMsgClientPingData ping_data = 5;

    pub fn clear_ping_data(&mut self) {
        self.ping_data.clear();
    }

    pub fn has_ping_data(&self) -> bool {
        self.ping_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_data(&mut self, v: CMsgClientPingData) {
        self.ping_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping_data(&mut self) -> &mut CMsgClientPingData {
        if self.ping_data.is_none() {
            self.ping_data.set_default();
        }
        self.ping_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_ping_data(&mut self) -> CMsgClientPingData {
        self.ping_data.take().unwrap_or_else(|| CMsgClientPingData::new())
    }

    pub fn get_ping_data(&self) -> &CMsgClientPingData {
        self.ping_data.as_ref().unwrap_or_else(|| CMsgClientPingData::default_instance())
    }

    fn get_ping_data_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgClientPingData> {
        &self.ping_data
    }

    fn mut_ping_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgClientPingData> {
        &mut self.ping_data
    }
}

impl ::protobuf::Message for CMsgInviteToParty {
    fn is_initialized(&self) -> bool {
        for v in &self.ping_data {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.as_coach = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ping_data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steam_id {
            my_size += 9;
        }
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.as_coach {
            my_size += 2;
        }
        if let Some(ref v) = self.ping_data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steam_id {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.client_version {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.team_id {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.as_coach {
            os.write_bool(4, v)?;
        }
        if let Some(ref v) = self.ping_data.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgInviteToParty {
    fn new() -> CMsgInviteToParty {
        CMsgInviteToParty::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgInviteToParty>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgInviteToParty::get_steam_id_for_reflect,
                    CMsgInviteToParty::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgInviteToParty::get_client_version_for_reflect,
                    CMsgInviteToParty::mut_client_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgInviteToParty::get_team_id_for_reflect,
                    CMsgInviteToParty::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "as_coach",
                    CMsgInviteToParty::get_as_coach_for_reflect,
                    CMsgInviteToParty::mut_as_coach_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgClientPingData>>(
                    "ping_data",
                    CMsgInviteToParty::get_ping_data_for_reflect,
                    CMsgInviteToParty::mut_ping_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgInviteToParty>(
                    "CMsgInviteToParty",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgInviteToParty {
    fn clear(&mut self) {
        self.clear_steam_id();
        self.clear_client_version();
        self.clear_team_id();
        self.clear_as_coach();
        self.clear_ping_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgInviteToParty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgInviteToParty {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgInviteToLobby {
    // message fields
    steam_id: ::std::option::Option<u64>,
    client_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgInviteToLobby {}

impl CMsgInviteToLobby {
    pub fn new() -> CMsgInviteToLobby {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgInviteToLobby {
        static mut instance: ::protobuf::lazy::Lazy<CMsgInviteToLobby> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgInviteToLobby,
        };
        unsafe {
            instance.get(CMsgInviteToLobby::new)
        }
    }

    // optional fixed64 steam_id = 1;

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    fn get_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steam_id
    }

    // optional uint32 client_version = 2;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }
}

impl ::protobuf::Message for CMsgInviteToLobby {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steam_id {
            my_size += 9;
        }
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steam_id {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.client_version {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgInviteToLobby {
    fn new() -> CMsgInviteToLobby {
        CMsgInviteToLobby::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgInviteToLobby>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgInviteToLobby::get_steam_id_for_reflect,
                    CMsgInviteToLobby::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgInviteToLobby::get_client_version_for_reflect,
                    CMsgInviteToLobby::mut_client_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgInviteToLobby>(
                    "CMsgInviteToLobby",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgInviteToLobby {
    fn clear(&mut self) {
        self.clear_steam_id();
        self.clear_client_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgInviteToLobby {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgInviteToLobby {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgInvitationCreated {
    // message fields
    group_id: ::std::option::Option<u64>,
    steam_id: ::std::option::Option<u64>,
    user_offline: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgInvitationCreated {}

impl CMsgInvitationCreated {
    pub fn new() -> CMsgInvitationCreated {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgInvitationCreated {
        static mut instance: ::protobuf::lazy::Lazy<CMsgInvitationCreated> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgInvitationCreated,
        };
        unsafe {
            instance.get(CMsgInvitationCreated::new)
        }
    }

    // optional uint64 group_id = 1;

    pub fn clear_group_id(&mut self) {
        self.group_id = ::std::option::Option::None;
    }

    pub fn has_group_id(&self) -> bool {
        self.group_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group_id(&mut self, v: u64) {
        self.group_id = ::std::option::Option::Some(v);
    }

    pub fn get_group_id(&self) -> u64 {
        self.group_id.unwrap_or(0)
    }

    fn get_group_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.group_id
    }

    fn mut_group_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.group_id
    }

    // optional fixed64 steam_id = 2;

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    fn get_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steam_id
    }

    // optional bool user_offline = 3;

    pub fn clear_user_offline(&mut self) {
        self.user_offline = ::std::option::Option::None;
    }

    pub fn has_user_offline(&self) -> bool {
        self.user_offline.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_offline(&mut self, v: bool) {
        self.user_offline = ::std::option::Option::Some(v);
    }

    pub fn get_user_offline(&self) -> bool {
        self.user_offline.unwrap_or(false)
    }

    fn get_user_offline_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.user_offline
    }

    fn mut_user_offline_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.user_offline
    }
}

impl ::protobuf::Message for CMsgInvitationCreated {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.group_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.user_offline = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.group_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.steam_id {
            my_size += 9;
        }
        if let Some(v) = self.user_offline {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.group_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.steam_id {
            os.write_fixed64(2, v)?;
        }
        if let Some(v) = self.user_offline {
            os.write_bool(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgInvitationCreated {
    fn new() -> CMsgInvitationCreated {
        CMsgInvitationCreated::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgInvitationCreated>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "group_id",
                    CMsgInvitationCreated::get_group_id_for_reflect,
                    CMsgInvitationCreated::mut_group_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgInvitationCreated::get_steam_id_for_reflect,
                    CMsgInvitationCreated::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "user_offline",
                    CMsgInvitationCreated::get_user_offline_for_reflect,
                    CMsgInvitationCreated::mut_user_offline_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgInvitationCreated>(
                    "CMsgInvitationCreated",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgInvitationCreated {
    fn clear(&mut self) {
        self.clear_group_id();
        self.clear_steam_id();
        self.clear_user_offline();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgInvitationCreated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgInvitationCreated {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPartyInviteResponse {
    // message fields
    party_id: ::std::option::Option<u64>,
    accept: ::std::option::Option<bool>,
    client_version: ::std::option::Option<u32>,
    ping_data: ::protobuf::SingularPtrField<CMsgClientPingData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPartyInviteResponse {}

impl CMsgPartyInviteResponse {
    pub fn new() -> CMsgPartyInviteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPartyInviteResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPartyInviteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPartyInviteResponse,
        };
        unsafe {
            instance.get(CMsgPartyInviteResponse::new)
        }
    }

    // optional uint64 party_id = 1;

    pub fn clear_party_id(&mut self) {
        self.party_id = ::std::option::Option::None;
    }

    pub fn has_party_id(&self) -> bool {
        self.party_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_party_id(&mut self, v: u64) {
        self.party_id = ::std::option::Option::Some(v);
    }

    pub fn get_party_id(&self) -> u64 {
        self.party_id.unwrap_or(0)
    }

    fn get_party_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.party_id
    }

    fn mut_party_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.party_id
    }

    // optional bool accept = 2;

    pub fn clear_accept(&mut self) {
        self.accept = ::std::option::Option::None;
    }

    pub fn has_accept(&self) -> bool {
        self.accept.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accept(&mut self, v: bool) {
        self.accept = ::std::option::Option::Some(v);
    }

    pub fn get_accept(&self) -> bool {
        self.accept.unwrap_or(false)
    }

    fn get_accept_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.accept
    }

    fn mut_accept_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.accept
    }

    // optional uint32 client_version = 3;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }

    // optional .CMsgClientPingData ping_data = 8;

    pub fn clear_ping_data(&mut self) {
        self.ping_data.clear();
    }

    pub fn has_ping_data(&self) -> bool {
        self.ping_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_data(&mut self, v: CMsgClientPingData) {
        self.ping_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping_data(&mut self) -> &mut CMsgClientPingData {
        if self.ping_data.is_none() {
            self.ping_data.set_default();
        }
        self.ping_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_ping_data(&mut self) -> CMsgClientPingData {
        self.ping_data.take().unwrap_or_else(|| CMsgClientPingData::new())
    }

    pub fn get_ping_data(&self) -> &CMsgClientPingData {
        self.ping_data.as_ref().unwrap_or_else(|| CMsgClientPingData::default_instance())
    }

    fn get_ping_data_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgClientPingData> {
        &self.ping_data
    }

    fn mut_ping_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgClientPingData> {
        &mut self.ping_data
    }
}

impl ::protobuf::Message for CMsgPartyInviteResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.ping_data {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.party_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.accept = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ping_data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.party_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.accept {
            my_size += 2;
        }
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.ping_data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.party_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.accept {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.client_version {
            os.write_uint32(3, v)?;
        }
        if let Some(ref v) = self.ping_data.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgPartyInviteResponse {
    fn new() -> CMsgPartyInviteResponse {
        CMsgPartyInviteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPartyInviteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "party_id",
                    CMsgPartyInviteResponse::get_party_id_for_reflect,
                    CMsgPartyInviteResponse::mut_party_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "accept",
                    CMsgPartyInviteResponse::get_accept_for_reflect,
                    CMsgPartyInviteResponse::mut_accept_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgPartyInviteResponse::get_client_version_for_reflect,
                    CMsgPartyInviteResponse::mut_client_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgClientPingData>>(
                    "ping_data",
                    CMsgPartyInviteResponse::get_ping_data_for_reflect,
                    CMsgPartyInviteResponse::mut_ping_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPartyInviteResponse>(
                    "CMsgPartyInviteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPartyInviteResponse {
    fn clear(&mut self) {
        self.clear_party_id();
        self.clear_accept();
        self.clear_client_version();
        self.clear_ping_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPartyInviteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPartyInviteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgLobbyInviteResponse {
    // message fields
    lobby_id: ::std::option::Option<u64>,
    accept: ::std::option::Option<bool>,
    client_version: ::std::option::Option<u32>,
    custom_game_crc: ::std::option::Option<u64>,
    custom_game_timestamp: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgLobbyInviteResponse {}

impl CMsgLobbyInviteResponse {
    pub fn new() -> CMsgLobbyInviteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgLobbyInviteResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgLobbyInviteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgLobbyInviteResponse,
        };
        unsafe {
            instance.get(CMsgLobbyInviteResponse::new)
        }
    }

    // optional fixed64 lobby_id = 1;

    pub fn clear_lobby_id(&mut self) {
        self.lobby_id = ::std::option::Option::None;
    }

    pub fn has_lobby_id(&self) -> bool {
        self.lobby_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_id(&mut self, v: u64) {
        self.lobby_id = ::std::option::Option::Some(v);
    }

    pub fn get_lobby_id(&self) -> u64 {
        self.lobby_id.unwrap_or(0)
    }

    fn get_lobby_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lobby_id
    }

    fn mut_lobby_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lobby_id
    }

    // optional bool accept = 2;

    pub fn clear_accept(&mut self) {
        self.accept = ::std::option::Option::None;
    }

    pub fn has_accept(&self) -> bool {
        self.accept.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accept(&mut self, v: bool) {
        self.accept = ::std::option::Option::Some(v);
    }

    pub fn get_accept(&self) -> bool {
        self.accept.unwrap_or(false)
    }

    fn get_accept_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.accept
    }

    fn mut_accept_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.accept
    }

    // optional uint32 client_version = 3;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }

    // optional fixed64 custom_game_crc = 6;

    pub fn clear_custom_game_crc(&mut self) {
        self.custom_game_crc = ::std::option::Option::None;
    }

    pub fn has_custom_game_crc(&self) -> bool {
        self.custom_game_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_crc(&mut self, v: u64) {
        self.custom_game_crc = ::std::option::Option::Some(v);
    }

    pub fn get_custom_game_crc(&self) -> u64 {
        self.custom_game_crc.unwrap_or(0)
    }

    fn get_custom_game_crc_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.custom_game_crc
    }

    fn mut_custom_game_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.custom_game_crc
    }

    // optional fixed32 custom_game_timestamp = 7;

    pub fn clear_custom_game_timestamp(&mut self) {
        self.custom_game_timestamp = ::std::option::Option::None;
    }

    pub fn has_custom_game_timestamp(&self) -> bool {
        self.custom_game_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_timestamp(&mut self, v: u32) {
        self.custom_game_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_custom_game_timestamp(&self) -> u32 {
        self.custom_game_timestamp.unwrap_or(0)
    }

    fn get_custom_game_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.custom_game_timestamp
    }

    fn mut_custom_game_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.custom_game_timestamp
    }
}

impl ::protobuf::Message for CMsgLobbyInviteResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.lobby_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.accept = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.custom_game_crc = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.custom_game_timestamp = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.lobby_id {
            my_size += 9;
        }
        if let Some(v) = self.accept {
            my_size += 2;
        }
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.custom_game_crc {
            my_size += 9;
        }
        if let Some(v) = self.custom_game_timestamp {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lobby_id {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.accept {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.client_version {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.custom_game_crc {
            os.write_fixed64(6, v)?;
        }
        if let Some(v) = self.custom_game_timestamp {
            os.write_fixed32(7, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgLobbyInviteResponse {
    fn new() -> CMsgLobbyInviteResponse {
        CMsgLobbyInviteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgLobbyInviteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "lobby_id",
                    CMsgLobbyInviteResponse::get_lobby_id_for_reflect,
                    CMsgLobbyInviteResponse::mut_lobby_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "accept",
                    CMsgLobbyInviteResponse::get_accept_for_reflect,
                    CMsgLobbyInviteResponse::mut_accept_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgLobbyInviteResponse::get_client_version_for_reflect,
                    CMsgLobbyInviteResponse::mut_client_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "custom_game_crc",
                    CMsgLobbyInviteResponse::get_custom_game_crc_for_reflect,
                    CMsgLobbyInviteResponse::mut_custom_game_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "custom_game_timestamp",
                    CMsgLobbyInviteResponse::get_custom_game_timestamp_for_reflect,
                    CMsgLobbyInviteResponse::mut_custom_game_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgLobbyInviteResponse>(
                    "CMsgLobbyInviteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgLobbyInviteResponse {
    fn clear(&mut self) {
        self.clear_lobby_id();
        self.clear_accept();
        self.clear_client_version();
        self.clear_custom_game_crc();
        self.clear_custom_game_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgLobbyInviteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgLobbyInviteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgKickFromParty {
    // message fields
    steam_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgKickFromParty {}

impl CMsgKickFromParty {
    pub fn new() -> CMsgKickFromParty {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgKickFromParty {
        static mut instance: ::protobuf::lazy::Lazy<CMsgKickFromParty> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgKickFromParty,
        };
        unsafe {
            instance.get(CMsgKickFromParty::new)
        }
    }

    // optional fixed64 steam_id = 1;

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    fn get_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steam_id
    }
}

impl ::protobuf::Message for CMsgKickFromParty {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steam_id {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steam_id {
            os.write_fixed64(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgKickFromParty {
    fn new() -> CMsgKickFromParty {
        CMsgKickFromParty::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgKickFromParty>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgKickFromParty::get_steam_id_for_reflect,
                    CMsgKickFromParty::mut_steam_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgKickFromParty>(
                    "CMsgKickFromParty",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgKickFromParty {
    fn clear(&mut self) {
        self.clear_steam_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgKickFromParty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgKickFromParty {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgLeaveParty {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgLeaveParty {}

impl CMsgLeaveParty {
    pub fn new() -> CMsgLeaveParty {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgLeaveParty {
        static mut instance: ::protobuf::lazy::Lazy<CMsgLeaveParty> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgLeaveParty,
        };
        unsafe {
            instance.get(CMsgLeaveParty::new)
        }
    }
}

impl ::protobuf::Message for CMsgLeaveParty {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgLeaveParty {
    fn new() -> CMsgLeaveParty {
        CMsgLeaveParty::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgLeaveParty>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgLeaveParty>(
                    "CMsgLeaveParty",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgLeaveParty {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgLeaveParty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgLeaveParty {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgCustomGameInstallStatus {
    // message fields
    status: ::std::option::Option<ECustomGameInstallStatus>,
    message: ::protobuf::SingularField<::std::string::String>,
    latest_timestamp_from_steam: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgCustomGameInstallStatus {}

impl CMsgCustomGameInstallStatus {
    pub fn new() -> CMsgCustomGameInstallStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgCustomGameInstallStatus {
        static mut instance: ::protobuf::lazy::Lazy<CMsgCustomGameInstallStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgCustomGameInstallStatus,
        };
        unsafe {
            instance.get(CMsgCustomGameInstallStatus::new)
        }
    }

    // optional .ECustomGameInstallStatus status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: ECustomGameInstallStatus) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> ECustomGameInstallStatus {
        self.status.unwrap_or(ECustomGameInstallStatus::k_ECustomGameInstallStatus_Unknown)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<ECustomGameInstallStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<ECustomGameInstallStatus> {
        &mut self.status
    }

    // optional string message = 2;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message
    }

    // optional fixed32 latest_timestamp_from_steam = 3;

    pub fn clear_latest_timestamp_from_steam(&mut self) {
        self.latest_timestamp_from_steam = ::std::option::Option::None;
    }

    pub fn has_latest_timestamp_from_steam(&self) -> bool {
        self.latest_timestamp_from_steam.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latest_timestamp_from_steam(&mut self, v: u32) {
        self.latest_timestamp_from_steam = ::std::option::Option::Some(v);
    }

    pub fn get_latest_timestamp_from_steam(&self) -> u32 {
        self.latest_timestamp_from_steam.unwrap_or(0)
    }

    fn get_latest_timestamp_from_steam_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.latest_timestamp_from_steam
    }

    fn mut_latest_timestamp_from_steam_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.latest_timestamp_from_steam
    }
}

impl ::protobuf::Message for CMsgCustomGameInstallStatus {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.latest_timestamp_from_steam = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.latest_timestamp_from_steam {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.latest_timestamp_from_steam {
            os.write_fixed32(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgCustomGameInstallStatus {
    fn new() -> CMsgCustomGameInstallStatus {
        CMsgCustomGameInstallStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgCustomGameInstallStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ECustomGameInstallStatus>>(
                    "status",
                    CMsgCustomGameInstallStatus::get_status_for_reflect,
                    CMsgCustomGameInstallStatus::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    CMsgCustomGameInstallStatus::get_message_for_reflect,
                    CMsgCustomGameInstallStatus::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "latest_timestamp_from_steam",
                    CMsgCustomGameInstallStatus::get_latest_timestamp_from_steam_for_reflect,
                    CMsgCustomGameInstallStatus::mut_latest_timestamp_from_steam_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgCustomGameInstallStatus>(
                    "CMsgCustomGameInstallStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgCustomGameInstallStatus {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_message();
        self.clear_latest_timestamp_from_steam();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgCustomGameInstallStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgCustomGameInstallStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgServerAvailable {
    // message fields
    custom_game_install_status: ::protobuf::SingularPtrField<CMsgCustomGameInstallStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgServerAvailable {}

impl CMsgServerAvailable {
    pub fn new() -> CMsgServerAvailable {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgServerAvailable {
        static mut instance: ::protobuf::lazy::Lazy<CMsgServerAvailable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgServerAvailable,
        };
        unsafe {
            instance.get(CMsgServerAvailable::new)
        }
    }

    // optional .CMsgCustomGameInstallStatus custom_game_install_status = 1;

    pub fn clear_custom_game_install_status(&mut self) {
        self.custom_game_install_status.clear();
    }

    pub fn has_custom_game_install_status(&self) -> bool {
        self.custom_game_install_status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_install_status(&mut self, v: CMsgCustomGameInstallStatus) {
        self.custom_game_install_status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_custom_game_install_status(&mut self) -> &mut CMsgCustomGameInstallStatus {
        if self.custom_game_install_status.is_none() {
            self.custom_game_install_status.set_default();
        }
        self.custom_game_install_status.as_mut().unwrap()
    }

    // Take field
    pub fn take_custom_game_install_status(&mut self) -> CMsgCustomGameInstallStatus {
        self.custom_game_install_status.take().unwrap_or_else(|| CMsgCustomGameInstallStatus::new())
    }

    pub fn get_custom_game_install_status(&self) -> &CMsgCustomGameInstallStatus {
        self.custom_game_install_status.as_ref().unwrap_or_else(|| CMsgCustomGameInstallStatus::default_instance())
    }

    fn get_custom_game_install_status_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgCustomGameInstallStatus> {
        &self.custom_game_install_status
    }

    fn mut_custom_game_install_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgCustomGameInstallStatus> {
        &mut self.custom_game_install_status
    }
}

impl ::protobuf::Message for CMsgServerAvailable {
    fn is_initialized(&self) -> bool {
        for v in &self.custom_game_install_status {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.custom_game_install_status)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.custom_game_install_status.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.custom_game_install_status.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgServerAvailable {
    fn new() -> CMsgServerAvailable {
        CMsgServerAvailable::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgServerAvailable>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgCustomGameInstallStatus>>(
                    "custom_game_install_status",
                    CMsgServerAvailable::get_custom_game_install_status_for_reflect,
                    CMsgServerAvailable::mut_custom_game_install_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgServerAvailable>(
                    "CMsgServerAvailable",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgServerAvailable {
    fn clear(&mut self) {
        self.clear_custom_game_install_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgServerAvailable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgServerAvailable {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgLANServerAvailable {
    // message fields
    lobby_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgLANServerAvailable {}

impl CMsgLANServerAvailable {
    pub fn new() -> CMsgLANServerAvailable {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgLANServerAvailable {
        static mut instance: ::protobuf::lazy::Lazy<CMsgLANServerAvailable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgLANServerAvailable,
        };
        unsafe {
            instance.get(CMsgLANServerAvailable::new)
        }
    }

    // optional fixed64 lobby_id = 1;

    pub fn clear_lobby_id(&mut self) {
        self.lobby_id = ::std::option::Option::None;
    }

    pub fn has_lobby_id(&self) -> bool {
        self.lobby_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_id(&mut self, v: u64) {
        self.lobby_id = ::std::option::Option::Some(v);
    }

    pub fn get_lobby_id(&self) -> u64 {
        self.lobby_id.unwrap_or(0)
    }

    fn get_lobby_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lobby_id
    }

    fn mut_lobby_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lobby_id
    }
}

impl ::protobuf::Message for CMsgLANServerAvailable {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.lobby_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.lobby_id {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lobby_id {
            os.write_fixed64(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgLANServerAvailable {
    fn new() -> CMsgLANServerAvailable {
        CMsgLANServerAvailable::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgLANServerAvailable>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "lobby_id",
                    CMsgLANServerAvailable::get_lobby_id_for_reflect,
                    CMsgLANServerAvailable::mut_lobby_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgLANServerAvailable>(
                    "CMsgLANServerAvailable",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgLANServerAvailable {
    fn clear(&mut self) {
        self.clear_lobby_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgLANServerAvailable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgLANServerAvailable {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSOEconGameAccountClient {
    // message fields
    additional_backpack_slots: ::std::option::Option<u32>,
    trial_account: ::std::option::Option<bool>,
    eligible_for_online_play: ::std::option::Option<bool>,
    need_to_choose_most_helpful_friend: ::std::option::Option<bool>,
    in_coaches_list: ::std::option::Option<bool>,
    trade_ban_expiration: ::std::option::Option<u32>,
    duel_ban_expiration: ::std::option::Option<u32>,
    made_first_purchase: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSOEconGameAccountClient {}

impl CSOEconGameAccountClient {
    pub fn new() -> CSOEconGameAccountClient {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSOEconGameAccountClient {
        static mut instance: ::protobuf::lazy::Lazy<CSOEconGameAccountClient> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSOEconGameAccountClient,
        };
        unsafe {
            instance.get(CSOEconGameAccountClient::new)
        }
    }

    // optional uint32 additional_backpack_slots = 1;

    pub fn clear_additional_backpack_slots(&mut self) {
        self.additional_backpack_slots = ::std::option::Option::None;
    }

    pub fn has_additional_backpack_slots(&self) -> bool {
        self.additional_backpack_slots.is_some()
    }

    // Param is passed by value, moved
    pub fn set_additional_backpack_slots(&mut self, v: u32) {
        self.additional_backpack_slots = ::std::option::Option::Some(v);
    }

    pub fn get_additional_backpack_slots(&self) -> u32 {
        self.additional_backpack_slots.unwrap_or(0u32)
    }

    fn get_additional_backpack_slots_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.additional_backpack_slots
    }

    fn mut_additional_backpack_slots_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.additional_backpack_slots
    }

    // optional bool trial_account = 2;

    pub fn clear_trial_account(&mut self) {
        self.trial_account = ::std::option::Option::None;
    }

    pub fn has_trial_account(&self) -> bool {
        self.trial_account.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trial_account(&mut self, v: bool) {
        self.trial_account = ::std::option::Option::Some(v);
    }

    pub fn get_trial_account(&self) -> bool {
        self.trial_account.unwrap_or(false)
    }

    fn get_trial_account_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.trial_account
    }

    fn mut_trial_account_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.trial_account
    }

    // optional bool eligible_for_online_play = 3;

    pub fn clear_eligible_for_online_play(&mut self) {
        self.eligible_for_online_play = ::std::option::Option::None;
    }

    pub fn has_eligible_for_online_play(&self) -> bool {
        self.eligible_for_online_play.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eligible_for_online_play(&mut self, v: bool) {
        self.eligible_for_online_play = ::std::option::Option::Some(v);
    }

    pub fn get_eligible_for_online_play(&self) -> bool {
        self.eligible_for_online_play.unwrap_or(true)
    }

    fn get_eligible_for_online_play_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.eligible_for_online_play
    }

    fn mut_eligible_for_online_play_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.eligible_for_online_play
    }

    // optional bool need_to_choose_most_helpful_friend = 4;

    pub fn clear_need_to_choose_most_helpful_friend(&mut self) {
        self.need_to_choose_most_helpful_friend = ::std::option::Option::None;
    }

    pub fn has_need_to_choose_most_helpful_friend(&self) -> bool {
        self.need_to_choose_most_helpful_friend.is_some()
    }

    // Param is passed by value, moved
    pub fn set_need_to_choose_most_helpful_friend(&mut self, v: bool) {
        self.need_to_choose_most_helpful_friend = ::std::option::Option::Some(v);
    }

    pub fn get_need_to_choose_most_helpful_friend(&self) -> bool {
        self.need_to_choose_most_helpful_friend.unwrap_or(false)
    }

    fn get_need_to_choose_most_helpful_friend_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.need_to_choose_most_helpful_friend
    }

    fn mut_need_to_choose_most_helpful_friend_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.need_to_choose_most_helpful_friend
    }

    // optional bool in_coaches_list = 5;

    pub fn clear_in_coaches_list(&mut self) {
        self.in_coaches_list = ::std::option::Option::None;
    }

    pub fn has_in_coaches_list(&self) -> bool {
        self.in_coaches_list.is_some()
    }

    // Param is passed by value, moved
    pub fn set_in_coaches_list(&mut self, v: bool) {
        self.in_coaches_list = ::std::option::Option::Some(v);
    }

    pub fn get_in_coaches_list(&self) -> bool {
        self.in_coaches_list.unwrap_or(false)
    }

    fn get_in_coaches_list_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.in_coaches_list
    }

    fn mut_in_coaches_list_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.in_coaches_list
    }

    // optional fixed32 trade_ban_expiration = 6;

    pub fn clear_trade_ban_expiration(&mut self) {
        self.trade_ban_expiration = ::std::option::Option::None;
    }

    pub fn has_trade_ban_expiration(&self) -> bool {
        self.trade_ban_expiration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trade_ban_expiration(&mut self, v: u32) {
        self.trade_ban_expiration = ::std::option::Option::Some(v);
    }

    pub fn get_trade_ban_expiration(&self) -> u32 {
        self.trade_ban_expiration.unwrap_or(0)
    }

    fn get_trade_ban_expiration_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.trade_ban_expiration
    }

    fn mut_trade_ban_expiration_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.trade_ban_expiration
    }

    // optional fixed32 duel_ban_expiration = 7;

    pub fn clear_duel_ban_expiration(&mut self) {
        self.duel_ban_expiration = ::std::option::Option::None;
    }

    pub fn has_duel_ban_expiration(&self) -> bool {
        self.duel_ban_expiration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duel_ban_expiration(&mut self, v: u32) {
        self.duel_ban_expiration = ::std::option::Option::Some(v);
    }

    pub fn get_duel_ban_expiration(&self) -> u32 {
        self.duel_ban_expiration.unwrap_or(0)
    }

    fn get_duel_ban_expiration_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.duel_ban_expiration
    }

    fn mut_duel_ban_expiration_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.duel_ban_expiration
    }

    // optional bool made_first_purchase = 9;

    pub fn clear_made_first_purchase(&mut self) {
        self.made_first_purchase = ::std::option::Option::None;
    }

    pub fn has_made_first_purchase(&self) -> bool {
        self.made_first_purchase.is_some()
    }

    // Param is passed by value, moved
    pub fn set_made_first_purchase(&mut self, v: bool) {
        self.made_first_purchase = ::std::option::Option::Some(v);
    }

    pub fn get_made_first_purchase(&self) -> bool {
        self.made_first_purchase.unwrap_or(false)
    }

    fn get_made_first_purchase_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.made_first_purchase
    }

    fn mut_made_first_purchase_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.made_first_purchase
    }
}

impl ::protobuf::Message for CSOEconGameAccountClient {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.additional_backpack_slots = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.trial_account = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.eligible_for_online_play = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.need_to_choose_most_helpful_friend = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.in_coaches_list = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.trade_ban_expiration = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.duel_ban_expiration = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.made_first_purchase = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.additional_backpack_slots {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.trial_account {
            my_size += 2;
        }
        if let Some(v) = self.eligible_for_online_play {
            my_size += 2;
        }
        if let Some(v) = self.need_to_choose_most_helpful_friend {
            my_size += 2;
        }
        if let Some(v) = self.in_coaches_list {
            my_size += 2;
        }
        if let Some(v) = self.trade_ban_expiration {
            my_size += 5;
        }
        if let Some(v) = self.duel_ban_expiration {
            my_size += 5;
        }
        if let Some(v) = self.made_first_purchase {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.additional_backpack_slots {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.trial_account {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.eligible_for_online_play {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.need_to_choose_most_helpful_friend {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.in_coaches_list {
            os.write_bool(5, v)?;
        }
        if let Some(v) = self.trade_ban_expiration {
            os.write_fixed32(6, v)?;
        }
        if let Some(v) = self.duel_ban_expiration {
            os.write_fixed32(7, v)?;
        }
        if let Some(v) = self.made_first_purchase {
            os.write_bool(9, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSOEconGameAccountClient {
    fn new() -> CSOEconGameAccountClient {
        CSOEconGameAccountClient::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSOEconGameAccountClient>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "additional_backpack_slots",
                    CSOEconGameAccountClient::get_additional_backpack_slots_for_reflect,
                    CSOEconGameAccountClient::mut_additional_backpack_slots_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "trial_account",
                    CSOEconGameAccountClient::get_trial_account_for_reflect,
                    CSOEconGameAccountClient::mut_trial_account_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "eligible_for_online_play",
                    CSOEconGameAccountClient::get_eligible_for_online_play_for_reflect,
                    CSOEconGameAccountClient::mut_eligible_for_online_play_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "need_to_choose_most_helpful_friend",
                    CSOEconGameAccountClient::get_need_to_choose_most_helpful_friend_for_reflect,
                    CSOEconGameAccountClient::mut_need_to_choose_most_helpful_friend_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "in_coaches_list",
                    CSOEconGameAccountClient::get_in_coaches_list_for_reflect,
                    CSOEconGameAccountClient::mut_in_coaches_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "trade_ban_expiration",
                    CSOEconGameAccountClient::get_trade_ban_expiration_for_reflect,
                    CSOEconGameAccountClient::mut_trade_ban_expiration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "duel_ban_expiration",
                    CSOEconGameAccountClient::get_duel_ban_expiration_for_reflect,
                    CSOEconGameAccountClient::mut_duel_ban_expiration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "made_first_purchase",
                    CSOEconGameAccountClient::get_made_first_purchase_for_reflect,
                    CSOEconGameAccountClient::mut_made_first_purchase_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSOEconGameAccountClient>(
                    "CSOEconGameAccountClient",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSOEconGameAccountClient {
    fn clear(&mut self) {
        self.clear_additional_backpack_slots();
        self.clear_trial_account();
        self.clear_eligible_for_online_play();
        self.clear_need_to_choose_most_helpful_friend();
        self.clear_in_coaches_list();
        self.clear_trade_ban_expiration();
        self.clear_duel_ban_expiration();
        self.clear_made_first_purchase();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSOEconGameAccountClient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSOEconGameAccountClient {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSOItemCriteriaCondition {
    // message fields
    op: ::std::option::Option<i32>,
    field: ::protobuf::SingularField<::std::string::String>,
    required: ::std::option::Option<bool>,
    float_value: ::std::option::Option<f32>,
    string_value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSOItemCriteriaCondition {}

impl CSOItemCriteriaCondition {
    pub fn new() -> CSOItemCriteriaCondition {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSOItemCriteriaCondition {
        static mut instance: ::protobuf::lazy::Lazy<CSOItemCriteriaCondition> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSOItemCriteriaCondition,
        };
        unsafe {
            instance.get(CSOItemCriteriaCondition::new)
        }
    }

    // optional int32 op = 1;

    pub fn clear_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: i32) {
        self.op = ::std::option::Option::Some(v);
    }

    pub fn get_op(&self) -> i32 {
        self.op.unwrap_or(0)
    }

    fn get_op_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.op
    }

    fn mut_op_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.op
    }

    // optional string field = 2;

    pub fn clear_field(&mut self) {
        self.field.clear();
    }

    pub fn has_field(&self) -> bool {
        self.field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field(&mut self, v: ::std::string::String) {
        self.field = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field(&mut self) -> &mut ::std::string::String {
        if self.field.is_none() {
            self.field.set_default();
        }
        self.field.as_mut().unwrap()
    }

    // Take field
    pub fn take_field(&mut self) -> ::std::string::String {
        self.field.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_field(&self) -> &str {
        match self.field.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_field_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.field
    }

    fn mut_field_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.field
    }

    // optional bool required = 3;

    pub fn clear_required(&mut self) {
        self.required = ::std::option::Option::None;
    }

    pub fn has_required(&self) -> bool {
        self.required.is_some()
    }

    // Param is passed by value, moved
    pub fn set_required(&mut self, v: bool) {
        self.required = ::std::option::Option::Some(v);
    }

    pub fn get_required(&self) -> bool {
        self.required.unwrap_or(false)
    }

    fn get_required_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.required
    }

    fn mut_required_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.required
    }

    // optional float float_value = 4;

    pub fn clear_float_value(&mut self) {
        self.float_value = ::std::option::Option::None;
    }

    pub fn has_float_value(&self) -> bool {
        self.float_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_float_value(&mut self, v: f32) {
        self.float_value = ::std::option::Option::Some(v);
    }

    pub fn get_float_value(&self) -> f32 {
        self.float_value.unwrap_or(0.)
    }

    fn get_float_value_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.float_value
    }

    fn mut_float_value_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.float_value
    }

    // optional string string_value = 5;

    pub fn clear_string_value(&mut self) {
        self.string_value.clear();
    }

    pub fn has_string_value(&self) -> bool {
        self.string_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_value(&mut self, v: ::std::string::String) {
        self.string_value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_value(&mut self) -> &mut ::std::string::String {
        if self.string_value.is_none() {
            self.string_value.set_default();
        }
        self.string_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_string_value(&mut self) -> ::std::string::String {
        self.string_value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_string_value(&self) -> &str {
        match self.string_value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_string_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.string_value
    }

    fn mut_string_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.string_value
    }
}

impl ::protobuf::Message for CSOItemCriteriaCondition {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.op = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.field)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.required = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.float_value = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.string_value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.op {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.field.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.required {
            my_size += 2;
        }
        if let Some(v) = self.float_value {
            my_size += 5;
        }
        if let Some(ref v) = self.string_value.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.op {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.field.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.required {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.float_value {
            os.write_float(4, v)?;
        }
        if let Some(ref v) = self.string_value.as_ref() {
            os.write_string(5, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSOItemCriteriaCondition {
    fn new() -> CSOItemCriteriaCondition {
        CSOItemCriteriaCondition::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSOItemCriteriaCondition>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "op",
                    CSOItemCriteriaCondition::get_op_for_reflect,
                    CSOItemCriteriaCondition::mut_op_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "field",
                    CSOItemCriteriaCondition::get_field_for_reflect,
                    CSOItemCriteriaCondition::mut_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "required",
                    CSOItemCriteriaCondition::get_required_for_reflect,
                    CSOItemCriteriaCondition::mut_required_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "float_value",
                    CSOItemCriteriaCondition::get_float_value_for_reflect,
                    CSOItemCriteriaCondition::mut_float_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "string_value",
                    CSOItemCriteriaCondition::get_string_value_for_reflect,
                    CSOItemCriteriaCondition::mut_string_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSOItemCriteriaCondition>(
                    "CSOItemCriteriaCondition",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSOItemCriteriaCondition {
    fn clear(&mut self) {
        self.clear_op();
        self.clear_field();
        self.clear_required();
        self.clear_float_value();
        self.clear_string_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSOItemCriteriaCondition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSOItemCriteriaCondition {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSOItemCriteria {
    // message fields
    item_level: ::std::option::Option<u32>,
    item_quality: ::std::option::Option<i32>,
    item_level_set: ::std::option::Option<bool>,
    item_quality_set: ::std::option::Option<bool>,
    initial_inventory: ::std::option::Option<u32>,
    initial_quantity: ::std::option::Option<u32>,
    ignore_enabled_flag: ::std::option::Option<bool>,
    conditions: ::protobuf::RepeatedField<CSOItemCriteriaCondition>,
    recent_only: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSOItemCriteria {}

impl CSOItemCriteria {
    pub fn new() -> CSOItemCriteria {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSOItemCriteria {
        static mut instance: ::protobuf::lazy::Lazy<CSOItemCriteria> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSOItemCriteria,
        };
        unsafe {
            instance.get(CSOItemCriteria::new)
        }
    }

    // optional uint32 item_level = 1;

    pub fn clear_item_level(&mut self) {
        self.item_level = ::std::option::Option::None;
    }

    pub fn has_item_level(&self) -> bool {
        self.item_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_level(&mut self, v: u32) {
        self.item_level = ::std::option::Option::Some(v);
    }

    pub fn get_item_level(&self) -> u32 {
        self.item_level.unwrap_or(0)
    }

    fn get_item_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_level
    }

    fn mut_item_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_level
    }

    // optional int32 item_quality = 2;

    pub fn clear_item_quality(&mut self) {
        self.item_quality = ::std::option::Option::None;
    }

    pub fn has_item_quality(&self) -> bool {
        self.item_quality.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_quality(&mut self, v: i32) {
        self.item_quality = ::std::option::Option::Some(v);
    }

    pub fn get_item_quality(&self) -> i32 {
        self.item_quality.unwrap_or(0)
    }

    fn get_item_quality_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.item_quality
    }

    fn mut_item_quality_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.item_quality
    }

    // optional bool item_level_set = 3;

    pub fn clear_item_level_set(&mut self) {
        self.item_level_set = ::std::option::Option::None;
    }

    pub fn has_item_level_set(&self) -> bool {
        self.item_level_set.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_level_set(&mut self, v: bool) {
        self.item_level_set = ::std::option::Option::Some(v);
    }

    pub fn get_item_level_set(&self) -> bool {
        self.item_level_set.unwrap_or(false)
    }

    fn get_item_level_set_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.item_level_set
    }

    fn mut_item_level_set_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.item_level_set
    }

    // optional bool item_quality_set = 4;

    pub fn clear_item_quality_set(&mut self) {
        self.item_quality_set = ::std::option::Option::None;
    }

    pub fn has_item_quality_set(&self) -> bool {
        self.item_quality_set.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_quality_set(&mut self, v: bool) {
        self.item_quality_set = ::std::option::Option::Some(v);
    }

    pub fn get_item_quality_set(&self) -> bool {
        self.item_quality_set.unwrap_or(false)
    }

    fn get_item_quality_set_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.item_quality_set
    }

    fn mut_item_quality_set_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.item_quality_set
    }

    // optional uint32 initial_inventory = 5;

    pub fn clear_initial_inventory(&mut self) {
        self.initial_inventory = ::std::option::Option::None;
    }

    pub fn has_initial_inventory(&self) -> bool {
        self.initial_inventory.is_some()
    }

    // Param is passed by value, moved
    pub fn set_initial_inventory(&mut self, v: u32) {
        self.initial_inventory = ::std::option::Option::Some(v);
    }

    pub fn get_initial_inventory(&self) -> u32 {
        self.initial_inventory.unwrap_or(0)
    }

    fn get_initial_inventory_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.initial_inventory
    }

    fn mut_initial_inventory_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.initial_inventory
    }

    // optional uint32 initial_quantity = 6;

    pub fn clear_initial_quantity(&mut self) {
        self.initial_quantity = ::std::option::Option::None;
    }

    pub fn has_initial_quantity(&self) -> bool {
        self.initial_quantity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_initial_quantity(&mut self, v: u32) {
        self.initial_quantity = ::std::option::Option::Some(v);
    }

    pub fn get_initial_quantity(&self) -> u32 {
        self.initial_quantity.unwrap_or(0)
    }

    fn get_initial_quantity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.initial_quantity
    }

    fn mut_initial_quantity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.initial_quantity
    }

    // optional bool ignore_enabled_flag = 8;

    pub fn clear_ignore_enabled_flag(&mut self) {
        self.ignore_enabled_flag = ::std::option::Option::None;
    }

    pub fn has_ignore_enabled_flag(&self) -> bool {
        self.ignore_enabled_flag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ignore_enabled_flag(&mut self, v: bool) {
        self.ignore_enabled_flag = ::std::option::Option::Some(v);
    }

    pub fn get_ignore_enabled_flag(&self) -> bool {
        self.ignore_enabled_flag.unwrap_or(false)
    }

    fn get_ignore_enabled_flag_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.ignore_enabled_flag
    }

    fn mut_ignore_enabled_flag_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.ignore_enabled_flag
    }

    // repeated .CSOItemCriteriaCondition conditions = 9;

    pub fn clear_conditions(&mut self) {
        self.conditions.clear();
    }

    // Param is passed by value, moved
    pub fn set_conditions(&mut self, v: ::protobuf::RepeatedField<CSOItemCriteriaCondition>) {
        self.conditions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_conditions(&mut self) -> &mut ::protobuf::RepeatedField<CSOItemCriteriaCondition> {
        &mut self.conditions
    }

    // Take field
    pub fn take_conditions(&mut self) -> ::protobuf::RepeatedField<CSOItemCriteriaCondition> {
        ::std::mem::replace(&mut self.conditions, ::protobuf::RepeatedField::new())
    }

    pub fn get_conditions(&self) -> &[CSOItemCriteriaCondition] {
        &self.conditions
    }

    fn get_conditions_for_reflect(&self) -> &::protobuf::RepeatedField<CSOItemCriteriaCondition> {
        &self.conditions
    }

    fn mut_conditions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSOItemCriteriaCondition> {
        &mut self.conditions
    }

    // optional bool recent_only = 10;

    pub fn clear_recent_only(&mut self) {
        self.recent_only = ::std::option::Option::None;
    }

    pub fn has_recent_only(&self) -> bool {
        self.recent_only.is_some()
    }

    // Param is passed by value, moved
    pub fn set_recent_only(&mut self, v: bool) {
        self.recent_only = ::std::option::Option::Some(v);
    }

    pub fn get_recent_only(&self) -> bool {
        self.recent_only.unwrap_or(false)
    }

    fn get_recent_only_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.recent_only
    }

    fn mut_recent_only_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.recent_only
    }
}

impl ::protobuf::Message for CSOItemCriteria {
    fn is_initialized(&self) -> bool {
        for v in &self.conditions {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.item_level = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.item_quality = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.item_level_set = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.item_quality_set = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.initial_inventory = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.initial_quantity = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.ignore_enabled_flag = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.conditions)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.recent_only = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.item_level {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.item_quality {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.item_level_set {
            my_size += 2;
        }
        if let Some(v) = self.item_quality_set {
            my_size += 2;
        }
        if let Some(v) = self.initial_inventory {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.initial_quantity {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ignore_enabled_flag {
            my_size += 2;
        }
        for value in &self.conditions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.recent_only {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_level {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.item_quality {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.item_level_set {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.item_quality_set {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.initial_inventory {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.initial_quantity {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.ignore_enabled_flag {
            os.write_bool(8, v)?;
        }
        for v in &self.conditions {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.recent_only {
            os.write_bool(10, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSOItemCriteria {
    fn new() -> CSOItemCriteria {
        CSOItemCriteria::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSOItemCriteria>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_level",
                    CSOItemCriteria::get_item_level_for_reflect,
                    CSOItemCriteria::mut_item_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "item_quality",
                    CSOItemCriteria::get_item_quality_for_reflect,
                    CSOItemCriteria::mut_item_quality_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "item_level_set",
                    CSOItemCriteria::get_item_level_set_for_reflect,
                    CSOItemCriteria::mut_item_level_set_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "item_quality_set",
                    CSOItemCriteria::get_item_quality_set_for_reflect,
                    CSOItemCriteria::mut_item_quality_set_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "initial_inventory",
                    CSOItemCriteria::get_initial_inventory_for_reflect,
                    CSOItemCriteria::mut_initial_inventory_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "initial_quantity",
                    CSOItemCriteria::get_initial_quantity_for_reflect,
                    CSOItemCriteria::mut_initial_quantity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ignore_enabled_flag",
                    CSOItemCriteria::get_ignore_enabled_flag_for_reflect,
                    CSOItemCriteria::mut_ignore_enabled_flag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSOItemCriteriaCondition>>(
                    "conditions",
                    CSOItemCriteria::get_conditions_for_reflect,
                    CSOItemCriteria::mut_conditions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "recent_only",
                    CSOItemCriteria::get_recent_only_for_reflect,
                    CSOItemCriteria::mut_recent_only_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSOItemCriteria>(
                    "CSOItemCriteria",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSOItemCriteria {
    fn clear(&mut self) {
        self.clear_item_level();
        self.clear_item_quality();
        self.clear_item_level_set();
        self.clear_item_quality_set();
        self.clear_initial_inventory();
        self.clear_initial_quantity();
        self.clear_ignore_enabled_flag();
        self.clear_conditions();
        self.clear_recent_only();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSOItemCriteria {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSOItemCriteria {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSOItemRecipe {
    // message fields
    def_index: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    n_a: ::protobuf::SingularField<::std::string::String>,
    desc_inputs: ::protobuf::SingularField<::std::string::String>,
    desc_outputs: ::protobuf::SingularField<::std::string::String>,
    di_a: ::protobuf::SingularField<::std::string::String>,
    di_b: ::protobuf::SingularField<::std::string::String>,
    di_c: ::protobuf::SingularField<::std::string::String>,
    do_a: ::protobuf::SingularField<::std::string::String>,
    do_b: ::protobuf::SingularField<::std::string::String>,
    do_c: ::protobuf::SingularField<::std::string::String>,
    requires_all_same_class: ::std::option::Option<bool>,
    requires_all_same_slot: ::std::option::Option<bool>,
    class_usage_for_output: ::std::option::Option<i32>,
    slot_usage_for_output: ::std::option::Option<i32>,
    set_for_output: ::std::option::Option<i32>,
    input_items_criteria: ::protobuf::RepeatedField<CSOItemCriteria>,
    output_items_criteria: ::protobuf::RepeatedField<CSOItemCriteria>,
    input_item_dupe_counts: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSOItemRecipe {}

impl CSOItemRecipe {
    pub fn new() -> CSOItemRecipe {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSOItemRecipe {
        static mut instance: ::protobuf::lazy::Lazy<CSOItemRecipe> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSOItemRecipe,
        };
        unsafe {
            instance.get(CSOItemRecipe::new)
        }
    }

    // optional uint32 def_index = 1;

    pub fn clear_def_index(&mut self) {
        self.def_index = ::std::option::Option::None;
    }

    pub fn has_def_index(&self) -> bool {
        self.def_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_def_index(&mut self, v: u32) {
        self.def_index = ::std::option::Option::Some(v);
    }

    pub fn get_def_index(&self) -> u32 {
        self.def_index.unwrap_or(0)
    }

    fn get_def_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.def_index
    }

    fn mut_def_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.def_index
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string n_a = 3;

    pub fn clear_n_a(&mut self) {
        self.n_a.clear();
    }

    pub fn has_n_a(&self) -> bool {
        self.n_a.is_some()
    }

    // Param is passed by value, moved
    pub fn set_n_a(&mut self, v: ::std::string::String) {
        self.n_a = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_n_a(&mut self) -> &mut ::std::string::String {
        if self.n_a.is_none() {
            self.n_a.set_default();
        }
        self.n_a.as_mut().unwrap()
    }

    // Take field
    pub fn take_n_a(&mut self) -> ::std::string::String {
        self.n_a.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_n_a(&self) -> &str {
        match self.n_a.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_n_a_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.n_a
    }

    fn mut_n_a_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.n_a
    }

    // optional string desc_inputs = 4;

    pub fn clear_desc_inputs(&mut self) {
        self.desc_inputs.clear();
    }

    pub fn has_desc_inputs(&self) -> bool {
        self.desc_inputs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desc_inputs(&mut self, v: ::std::string::String) {
        self.desc_inputs = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_desc_inputs(&mut self) -> &mut ::std::string::String {
        if self.desc_inputs.is_none() {
            self.desc_inputs.set_default();
        }
        self.desc_inputs.as_mut().unwrap()
    }

    // Take field
    pub fn take_desc_inputs(&mut self) -> ::std::string::String {
        self.desc_inputs.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_desc_inputs(&self) -> &str {
        match self.desc_inputs.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_desc_inputs_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.desc_inputs
    }

    fn mut_desc_inputs_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.desc_inputs
    }

    // optional string desc_outputs = 5;

    pub fn clear_desc_outputs(&mut self) {
        self.desc_outputs.clear();
    }

    pub fn has_desc_outputs(&self) -> bool {
        self.desc_outputs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desc_outputs(&mut self, v: ::std::string::String) {
        self.desc_outputs = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_desc_outputs(&mut self) -> &mut ::std::string::String {
        if self.desc_outputs.is_none() {
            self.desc_outputs.set_default();
        }
        self.desc_outputs.as_mut().unwrap()
    }

    // Take field
    pub fn take_desc_outputs(&mut self) -> ::std::string::String {
        self.desc_outputs.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_desc_outputs(&self) -> &str {
        match self.desc_outputs.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_desc_outputs_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.desc_outputs
    }

    fn mut_desc_outputs_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.desc_outputs
    }

    // optional string di_a = 6;

    pub fn clear_di_a(&mut self) {
        self.di_a.clear();
    }

    pub fn has_di_a(&self) -> bool {
        self.di_a.is_some()
    }

    // Param is passed by value, moved
    pub fn set_di_a(&mut self, v: ::std::string::String) {
        self.di_a = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_di_a(&mut self) -> &mut ::std::string::String {
        if self.di_a.is_none() {
            self.di_a.set_default();
        }
        self.di_a.as_mut().unwrap()
    }

    // Take field
    pub fn take_di_a(&mut self) -> ::std::string::String {
        self.di_a.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_di_a(&self) -> &str {
        match self.di_a.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_di_a_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.di_a
    }

    fn mut_di_a_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.di_a
    }

    // optional string di_b = 7;

    pub fn clear_di_b(&mut self) {
        self.di_b.clear();
    }

    pub fn has_di_b(&self) -> bool {
        self.di_b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_di_b(&mut self, v: ::std::string::String) {
        self.di_b = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_di_b(&mut self) -> &mut ::std::string::String {
        if self.di_b.is_none() {
            self.di_b.set_default();
        }
        self.di_b.as_mut().unwrap()
    }

    // Take field
    pub fn take_di_b(&mut self) -> ::std::string::String {
        self.di_b.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_di_b(&self) -> &str {
        match self.di_b.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_di_b_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.di_b
    }

    fn mut_di_b_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.di_b
    }

    // optional string di_c = 8;

    pub fn clear_di_c(&mut self) {
        self.di_c.clear();
    }

    pub fn has_di_c(&self) -> bool {
        self.di_c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_di_c(&mut self, v: ::std::string::String) {
        self.di_c = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_di_c(&mut self) -> &mut ::std::string::String {
        if self.di_c.is_none() {
            self.di_c.set_default();
        }
        self.di_c.as_mut().unwrap()
    }

    // Take field
    pub fn take_di_c(&mut self) -> ::std::string::String {
        self.di_c.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_di_c(&self) -> &str {
        match self.di_c.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_di_c_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.di_c
    }

    fn mut_di_c_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.di_c
    }

    // optional string do_a = 9;

    pub fn clear_do_a(&mut self) {
        self.do_a.clear();
    }

    pub fn has_do_a(&self) -> bool {
        self.do_a.is_some()
    }

    // Param is passed by value, moved
    pub fn set_do_a(&mut self, v: ::std::string::String) {
        self.do_a = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_do_a(&mut self) -> &mut ::std::string::String {
        if self.do_a.is_none() {
            self.do_a.set_default();
        }
        self.do_a.as_mut().unwrap()
    }

    // Take field
    pub fn take_do_a(&mut self) -> ::std::string::String {
        self.do_a.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_do_a(&self) -> &str {
        match self.do_a.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_do_a_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.do_a
    }

    fn mut_do_a_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.do_a
    }

    // optional string do_b = 10;

    pub fn clear_do_b(&mut self) {
        self.do_b.clear();
    }

    pub fn has_do_b(&self) -> bool {
        self.do_b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_do_b(&mut self, v: ::std::string::String) {
        self.do_b = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_do_b(&mut self) -> &mut ::std::string::String {
        if self.do_b.is_none() {
            self.do_b.set_default();
        }
        self.do_b.as_mut().unwrap()
    }

    // Take field
    pub fn take_do_b(&mut self) -> ::std::string::String {
        self.do_b.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_do_b(&self) -> &str {
        match self.do_b.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_do_b_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.do_b
    }

    fn mut_do_b_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.do_b
    }

    // optional string do_c = 11;

    pub fn clear_do_c(&mut self) {
        self.do_c.clear();
    }

    pub fn has_do_c(&self) -> bool {
        self.do_c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_do_c(&mut self, v: ::std::string::String) {
        self.do_c = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_do_c(&mut self) -> &mut ::std::string::String {
        if self.do_c.is_none() {
            self.do_c.set_default();
        }
        self.do_c.as_mut().unwrap()
    }

    // Take field
    pub fn take_do_c(&mut self) -> ::std::string::String {
        self.do_c.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_do_c(&self) -> &str {
        match self.do_c.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_do_c_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.do_c
    }

    fn mut_do_c_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.do_c
    }

    // optional bool requires_all_same_class = 12;

    pub fn clear_requires_all_same_class(&mut self) {
        self.requires_all_same_class = ::std::option::Option::None;
    }

    pub fn has_requires_all_same_class(&self) -> bool {
        self.requires_all_same_class.is_some()
    }

    // Param is passed by value, moved
    pub fn set_requires_all_same_class(&mut self, v: bool) {
        self.requires_all_same_class = ::std::option::Option::Some(v);
    }

    pub fn get_requires_all_same_class(&self) -> bool {
        self.requires_all_same_class.unwrap_or(false)
    }

    fn get_requires_all_same_class_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.requires_all_same_class
    }

    fn mut_requires_all_same_class_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.requires_all_same_class
    }

    // optional bool requires_all_same_slot = 13;

    pub fn clear_requires_all_same_slot(&mut self) {
        self.requires_all_same_slot = ::std::option::Option::None;
    }

    pub fn has_requires_all_same_slot(&self) -> bool {
        self.requires_all_same_slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_requires_all_same_slot(&mut self, v: bool) {
        self.requires_all_same_slot = ::std::option::Option::Some(v);
    }

    pub fn get_requires_all_same_slot(&self) -> bool {
        self.requires_all_same_slot.unwrap_or(false)
    }

    fn get_requires_all_same_slot_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.requires_all_same_slot
    }

    fn mut_requires_all_same_slot_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.requires_all_same_slot
    }

    // optional int32 class_usage_for_output = 14;

    pub fn clear_class_usage_for_output(&mut self) {
        self.class_usage_for_output = ::std::option::Option::None;
    }

    pub fn has_class_usage_for_output(&self) -> bool {
        self.class_usage_for_output.is_some()
    }

    // Param is passed by value, moved
    pub fn set_class_usage_for_output(&mut self, v: i32) {
        self.class_usage_for_output = ::std::option::Option::Some(v);
    }

    pub fn get_class_usage_for_output(&self) -> i32 {
        self.class_usage_for_output.unwrap_or(0)
    }

    fn get_class_usage_for_output_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.class_usage_for_output
    }

    fn mut_class_usage_for_output_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.class_usage_for_output
    }

    // optional int32 slot_usage_for_output = 15;

    pub fn clear_slot_usage_for_output(&mut self) {
        self.slot_usage_for_output = ::std::option::Option::None;
    }

    pub fn has_slot_usage_for_output(&self) -> bool {
        self.slot_usage_for_output.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slot_usage_for_output(&mut self, v: i32) {
        self.slot_usage_for_output = ::std::option::Option::Some(v);
    }

    pub fn get_slot_usage_for_output(&self) -> i32 {
        self.slot_usage_for_output.unwrap_or(0)
    }

    fn get_slot_usage_for_output_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.slot_usage_for_output
    }

    fn mut_slot_usage_for_output_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.slot_usage_for_output
    }

    // optional int32 set_for_output = 16;

    pub fn clear_set_for_output(&mut self) {
        self.set_for_output = ::std::option::Option::None;
    }

    pub fn has_set_for_output(&self) -> bool {
        self.set_for_output.is_some()
    }

    // Param is passed by value, moved
    pub fn set_set_for_output(&mut self, v: i32) {
        self.set_for_output = ::std::option::Option::Some(v);
    }

    pub fn get_set_for_output(&self) -> i32 {
        self.set_for_output.unwrap_or(0)
    }

    fn get_set_for_output_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.set_for_output
    }

    fn mut_set_for_output_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.set_for_output
    }

    // repeated .CSOItemCriteria input_items_criteria = 20;

    pub fn clear_input_items_criteria(&mut self) {
        self.input_items_criteria.clear();
    }

    // Param is passed by value, moved
    pub fn set_input_items_criteria(&mut self, v: ::protobuf::RepeatedField<CSOItemCriteria>) {
        self.input_items_criteria = v;
    }

    // Mutable pointer to the field.
    pub fn mut_input_items_criteria(&mut self) -> &mut ::protobuf::RepeatedField<CSOItemCriteria> {
        &mut self.input_items_criteria
    }

    // Take field
    pub fn take_input_items_criteria(&mut self) -> ::protobuf::RepeatedField<CSOItemCriteria> {
        ::std::mem::replace(&mut self.input_items_criteria, ::protobuf::RepeatedField::new())
    }

    pub fn get_input_items_criteria(&self) -> &[CSOItemCriteria] {
        &self.input_items_criteria
    }

    fn get_input_items_criteria_for_reflect(&self) -> &::protobuf::RepeatedField<CSOItemCriteria> {
        &self.input_items_criteria
    }

    fn mut_input_items_criteria_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSOItemCriteria> {
        &mut self.input_items_criteria
    }

    // repeated .CSOItemCriteria output_items_criteria = 21;

    pub fn clear_output_items_criteria(&mut self) {
        self.output_items_criteria.clear();
    }

    // Param is passed by value, moved
    pub fn set_output_items_criteria(&mut self, v: ::protobuf::RepeatedField<CSOItemCriteria>) {
        self.output_items_criteria = v;
    }

    // Mutable pointer to the field.
    pub fn mut_output_items_criteria(&mut self) -> &mut ::protobuf::RepeatedField<CSOItemCriteria> {
        &mut self.output_items_criteria
    }

    // Take field
    pub fn take_output_items_criteria(&mut self) -> ::protobuf::RepeatedField<CSOItemCriteria> {
        ::std::mem::replace(&mut self.output_items_criteria, ::protobuf::RepeatedField::new())
    }

    pub fn get_output_items_criteria(&self) -> &[CSOItemCriteria] {
        &self.output_items_criteria
    }

    fn get_output_items_criteria_for_reflect(&self) -> &::protobuf::RepeatedField<CSOItemCriteria> {
        &self.output_items_criteria
    }

    fn mut_output_items_criteria_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSOItemCriteria> {
        &mut self.output_items_criteria
    }

    // repeated uint32 input_item_dupe_counts = 22;

    pub fn clear_input_item_dupe_counts(&mut self) {
        self.input_item_dupe_counts.clear();
    }

    // Param is passed by value, moved
    pub fn set_input_item_dupe_counts(&mut self, v: ::std::vec::Vec<u32>) {
        self.input_item_dupe_counts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_input_item_dupe_counts(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.input_item_dupe_counts
    }

    // Take field
    pub fn take_input_item_dupe_counts(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.input_item_dupe_counts, ::std::vec::Vec::new())
    }

    pub fn get_input_item_dupe_counts(&self) -> &[u32] {
        &self.input_item_dupe_counts
    }

    fn get_input_item_dupe_counts_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.input_item_dupe_counts
    }

    fn mut_input_item_dupe_counts_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.input_item_dupe_counts
    }
}

impl ::protobuf::Message for CSOItemRecipe {
    fn is_initialized(&self) -> bool {
        for v in &self.input_items_criteria {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.output_items_criteria {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.def_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.n_a)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.desc_inputs)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.desc_outputs)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.di_a)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.di_b)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.di_c)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.do_a)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.do_b)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.do_c)?;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.requires_all_same_class = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.requires_all_same_slot = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.class_usage_for_output = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.slot_usage_for_output = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.set_for_output = ::std::option::Option::Some(tmp);
                },
                20 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.input_items_criteria)?;
                },
                21 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.output_items_criteria)?;
                },
                22 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.input_item_dupe_counts)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.def_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.n_a.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.desc_inputs.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.desc_outputs.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.di_a.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(ref v) = self.di_b.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        if let Some(ref v) = self.di_c.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(ref v) = self.do_a.as_ref() {
            my_size += ::protobuf::rt::string_size(9, &v);
        }
        if let Some(ref v) = self.do_b.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        if let Some(ref v) = self.do_c.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        }
        if let Some(v) = self.requires_all_same_class {
            my_size += 2;
        }
        if let Some(v) = self.requires_all_same_slot {
            my_size += 2;
        }
        if let Some(v) = self.class_usage_for_output {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.slot_usage_for_output {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.set_for_output {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.input_items_criteria {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.output_items_criteria {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.input_item_dupe_counts {
            my_size += ::protobuf::rt::value_size(22, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.def_index {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.n_a.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.desc_inputs.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.desc_outputs.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.di_a.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(ref v) = self.di_b.as_ref() {
            os.write_string(7, &v)?;
        }
        if let Some(ref v) = self.di_c.as_ref() {
            os.write_string(8, &v)?;
        }
        if let Some(ref v) = self.do_a.as_ref() {
            os.write_string(9, &v)?;
        }
        if let Some(ref v) = self.do_b.as_ref() {
            os.write_string(10, &v)?;
        }
        if let Some(ref v) = self.do_c.as_ref() {
            os.write_string(11, &v)?;
        }
        if let Some(v) = self.requires_all_same_class {
            os.write_bool(12, v)?;
        }
        if let Some(v) = self.requires_all_same_slot {
            os.write_bool(13, v)?;
        }
        if let Some(v) = self.class_usage_for_output {
            os.write_int32(14, v)?;
        }
        if let Some(v) = self.slot_usage_for_output {
            os.write_int32(15, v)?;
        }
        if let Some(v) = self.set_for_output {
            os.write_int32(16, v)?;
        }
        for v in &self.input_items_criteria {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.output_items_criteria {
            os.write_tag(21, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.input_item_dupe_counts {
            os.write_uint32(22, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSOItemRecipe {
    fn new() -> CSOItemRecipe {
        CSOItemRecipe::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSOItemRecipe>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "def_index",
                    CSOItemRecipe::get_def_index_for_reflect,
                    CSOItemRecipe::mut_def_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CSOItemRecipe::get_name_for_reflect,
                    CSOItemRecipe::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "n_a",
                    CSOItemRecipe::get_n_a_for_reflect,
                    CSOItemRecipe::mut_n_a_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "desc_inputs",
                    CSOItemRecipe::get_desc_inputs_for_reflect,
                    CSOItemRecipe::mut_desc_inputs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "desc_outputs",
                    CSOItemRecipe::get_desc_outputs_for_reflect,
                    CSOItemRecipe::mut_desc_outputs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "di_a",
                    CSOItemRecipe::get_di_a_for_reflect,
                    CSOItemRecipe::mut_di_a_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "di_b",
                    CSOItemRecipe::get_di_b_for_reflect,
                    CSOItemRecipe::mut_di_b_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "di_c",
                    CSOItemRecipe::get_di_c_for_reflect,
                    CSOItemRecipe::mut_di_c_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "do_a",
                    CSOItemRecipe::get_do_a_for_reflect,
                    CSOItemRecipe::mut_do_a_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "do_b",
                    CSOItemRecipe::get_do_b_for_reflect,
                    CSOItemRecipe::mut_do_b_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "do_c",
                    CSOItemRecipe::get_do_c_for_reflect,
                    CSOItemRecipe::mut_do_c_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "requires_all_same_class",
                    CSOItemRecipe::get_requires_all_same_class_for_reflect,
                    CSOItemRecipe::mut_requires_all_same_class_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "requires_all_same_slot",
                    CSOItemRecipe::get_requires_all_same_slot_for_reflect,
                    CSOItemRecipe::mut_requires_all_same_slot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "class_usage_for_output",
                    CSOItemRecipe::get_class_usage_for_output_for_reflect,
                    CSOItemRecipe::mut_class_usage_for_output_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "slot_usage_for_output",
                    CSOItemRecipe::get_slot_usage_for_output_for_reflect,
                    CSOItemRecipe::mut_slot_usage_for_output_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "set_for_output",
                    CSOItemRecipe::get_set_for_output_for_reflect,
                    CSOItemRecipe::mut_set_for_output_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSOItemCriteria>>(
                    "input_items_criteria",
                    CSOItemRecipe::get_input_items_criteria_for_reflect,
                    CSOItemRecipe::mut_input_items_criteria_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSOItemCriteria>>(
                    "output_items_criteria",
                    CSOItemRecipe::get_output_items_criteria_for_reflect,
                    CSOItemRecipe::mut_output_items_criteria_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "input_item_dupe_counts",
                    CSOItemRecipe::get_input_item_dupe_counts_for_reflect,
                    CSOItemRecipe::mut_input_item_dupe_counts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSOItemRecipe>(
                    "CSOItemRecipe",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSOItemRecipe {
    fn clear(&mut self) {
        self.clear_def_index();
        self.clear_name();
        self.clear_n_a();
        self.clear_desc_inputs();
        self.clear_desc_outputs();
        self.clear_di_a();
        self.clear_di_b();
        self.clear_di_c();
        self.clear_do_a();
        self.clear_do_b();
        self.clear_do_c();
        self.clear_requires_all_same_class();
        self.clear_requires_all_same_slot();
        self.clear_class_usage_for_output();
        self.clear_slot_usage_for_output();
        self.clear_set_for_output();
        self.clear_input_items_criteria();
        self.clear_output_items_criteria();
        self.clear_input_item_dupe_counts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSOItemRecipe {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSOItemRecipe {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgApplyStrangePart {
    // message fields
    strange_part_item_id: ::std::option::Option<u64>,
    item_item_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgApplyStrangePart {}

impl CMsgApplyStrangePart {
    pub fn new() -> CMsgApplyStrangePart {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgApplyStrangePart {
        static mut instance: ::protobuf::lazy::Lazy<CMsgApplyStrangePart> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgApplyStrangePart,
        };
        unsafe {
            instance.get(CMsgApplyStrangePart::new)
        }
    }

    // optional uint64 strange_part_item_id = 1;

    pub fn clear_strange_part_item_id(&mut self) {
        self.strange_part_item_id = ::std::option::Option::None;
    }

    pub fn has_strange_part_item_id(&self) -> bool {
        self.strange_part_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_strange_part_item_id(&mut self, v: u64) {
        self.strange_part_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_strange_part_item_id(&self) -> u64 {
        self.strange_part_item_id.unwrap_or(0)
    }

    fn get_strange_part_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.strange_part_item_id
    }

    fn mut_strange_part_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.strange_part_item_id
    }

    // optional uint64 item_item_id = 2;

    pub fn clear_item_item_id(&mut self) {
        self.item_item_id = ::std::option::Option::None;
    }

    pub fn has_item_item_id(&self) -> bool {
        self.item_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_item_id(&mut self, v: u64) {
        self.item_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_item_id(&self) -> u64 {
        self.item_item_id.unwrap_or(0)
    }

    fn get_item_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.item_item_id
    }

    fn mut_item_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.item_item_id
    }
}

impl ::protobuf::Message for CMsgApplyStrangePart {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.strange_part_item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.item_item_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.strange_part_item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.item_item_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.strange_part_item_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.item_item_id {
            os.write_uint64(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgApplyStrangePart {
    fn new() -> CMsgApplyStrangePart {
        CMsgApplyStrangePart::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgApplyStrangePart>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "strange_part_item_id",
                    CMsgApplyStrangePart::get_strange_part_item_id_for_reflect,
                    CMsgApplyStrangePart::mut_strange_part_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "item_item_id",
                    CMsgApplyStrangePart::get_item_item_id_for_reflect,
                    CMsgApplyStrangePart::mut_item_item_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgApplyStrangePart>(
                    "CMsgApplyStrangePart",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgApplyStrangePart {
    fn clear(&mut self) {
        self.clear_strange_part_item_id();
        self.clear_item_item_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgApplyStrangePart {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgApplyStrangePart {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgApplyPennantUpgrade {
    // message fields
    upgrade_item_id: ::std::option::Option<u64>,
    pennant_item_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgApplyPennantUpgrade {}

impl CMsgApplyPennantUpgrade {
    pub fn new() -> CMsgApplyPennantUpgrade {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgApplyPennantUpgrade {
        static mut instance: ::protobuf::lazy::Lazy<CMsgApplyPennantUpgrade> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgApplyPennantUpgrade,
        };
        unsafe {
            instance.get(CMsgApplyPennantUpgrade::new)
        }
    }

    // optional uint64 upgrade_item_id = 1;

    pub fn clear_upgrade_item_id(&mut self) {
        self.upgrade_item_id = ::std::option::Option::None;
    }

    pub fn has_upgrade_item_id(&self) -> bool {
        self.upgrade_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upgrade_item_id(&mut self, v: u64) {
        self.upgrade_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_upgrade_item_id(&self) -> u64 {
        self.upgrade_item_id.unwrap_or(0)
    }

    fn get_upgrade_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.upgrade_item_id
    }

    fn mut_upgrade_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.upgrade_item_id
    }

    // optional uint64 pennant_item_id = 2;

    pub fn clear_pennant_item_id(&mut self) {
        self.pennant_item_id = ::std::option::Option::None;
    }

    pub fn has_pennant_item_id(&self) -> bool {
        self.pennant_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pennant_item_id(&mut self, v: u64) {
        self.pennant_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_pennant_item_id(&self) -> u64 {
        self.pennant_item_id.unwrap_or(0)
    }

    fn get_pennant_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.pennant_item_id
    }

    fn mut_pennant_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.pennant_item_id
    }
}

impl ::protobuf::Message for CMsgApplyPennantUpgrade {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.upgrade_item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.pennant_item_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.upgrade_item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.pennant_item_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.upgrade_item_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.pennant_item_id {
            os.write_uint64(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgApplyPennantUpgrade {
    fn new() -> CMsgApplyPennantUpgrade {
        CMsgApplyPennantUpgrade::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgApplyPennantUpgrade>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "upgrade_item_id",
                    CMsgApplyPennantUpgrade::get_upgrade_item_id_for_reflect,
                    CMsgApplyPennantUpgrade::mut_upgrade_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "pennant_item_id",
                    CMsgApplyPennantUpgrade::get_pennant_item_id_for_reflect,
                    CMsgApplyPennantUpgrade::mut_pennant_item_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgApplyPennantUpgrade>(
                    "CMsgApplyPennantUpgrade",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgApplyPennantUpgrade {
    fn clear(&mut self) {
        self.clear_upgrade_item_id();
        self.clear_pennant_item_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgApplyPennantUpgrade {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgApplyPennantUpgrade {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgApplyEggEssence {
    // message fields
    essence_item_id: ::std::option::Option<u64>,
    egg_item_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgApplyEggEssence {}

impl CMsgApplyEggEssence {
    pub fn new() -> CMsgApplyEggEssence {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgApplyEggEssence {
        static mut instance: ::protobuf::lazy::Lazy<CMsgApplyEggEssence> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgApplyEggEssence,
        };
        unsafe {
            instance.get(CMsgApplyEggEssence::new)
        }
    }

    // optional uint64 essence_item_id = 1;

    pub fn clear_essence_item_id(&mut self) {
        self.essence_item_id = ::std::option::Option::None;
    }

    pub fn has_essence_item_id(&self) -> bool {
        self.essence_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_essence_item_id(&mut self, v: u64) {
        self.essence_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_essence_item_id(&self) -> u64 {
        self.essence_item_id.unwrap_or(0)
    }

    fn get_essence_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.essence_item_id
    }

    fn mut_essence_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.essence_item_id
    }

    // optional uint64 egg_item_id = 2;

    pub fn clear_egg_item_id(&mut self) {
        self.egg_item_id = ::std::option::Option::None;
    }

    pub fn has_egg_item_id(&self) -> bool {
        self.egg_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_egg_item_id(&mut self, v: u64) {
        self.egg_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_egg_item_id(&self) -> u64 {
        self.egg_item_id.unwrap_or(0)
    }

    fn get_egg_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.egg_item_id
    }

    fn mut_egg_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.egg_item_id
    }
}

impl ::protobuf::Message for CMsgApplyEggEssence {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.essence_item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.egg_item_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.essence_item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.egg_item_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.essence_item_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.egg_item_id {
            os.write_uint64(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgApplyEggEssence {
    fn new() -> CMsgApplyEggEssence {
        CMsgApplyEggEssence::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgApplyEggEssence>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "essence_item_id",
                    CMsgApplyEggEssence::get_essence_item_id_for_reflect,
                    CMsgApplyEggEssence::mut_essence_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "egg_item_id",
                    CMsgApplyEggEssence::get_egg_item_id_for_reflect,
                    CMsgApplyEggEssence::mut_egg_item_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgApplyEggEssence>(
                    "CMsgApplyEggEssence",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgApplyEggEssence {
    fn clear(&mut self) {
        self.clear_essence_item_id();
        self.clear_egg_item_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgApplyEggEssence {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgApplyEggEssence {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSOEconItemAttribute {
    // message fields
    def_index: ::std::option::Option<u32>,
    value: ::std::option::Option<u32>,
    value_bytes: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSOEconItemAttribute {}

impl CSOEconItemAttribute {
    pub fn new() -> CSOEconItemAttribute {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSOEconItemAttribute {
        static mut instance: ::protobuf::lazy::Lazy<CSOEconItemAttribute> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSOEconItemAttribute,
        };
        unsafe {
            instance.get(CSOEconItemAttribute::new)
        }
    }

    // optional uint32 def_index = 1;

    pub fn clear_def_index(&mut self) {
        self.def_index = ::std::option::Option::None;
    }

    pub fn has_def_index(&self) -> bool {
        self.def_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_def_index(&mut self, v: u32) {
        self.def_index = ::std::option::Option::Some(v);
    }

    pub fn get_def_index(&self) -> u32 {
        self.def_index.unwrap_or(0)
    }

    fn get_def_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.def_index
    }

    fn mut_def_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.def_index
    }

    // optional uint32 value = 2;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: u32) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> u32 {
        self.value.unwrap_or(0)
    }

    fn get_value_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.value
    }

    // optional bytes value_bytes = 3;

    pub fn clear_value_bytes(&mut self) {
        self.value_bytes.clear();
    }

    pub fn has_value_bytes(&self) -> bool {
        self.value_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value_bytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.value_bytes = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value_bytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value_bytes.is_none() {
            self.value_bytes.set_default();
        }
        self.value_bytes.as_mut().unwrap()
    }

    // Take field
    pub fn take_value_bytes(&mut self) -> ::std::vec::Vec<u8> {
        self.value_bytes.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value_bytes(&self) -> &[u8] {
        match self.value_bytes.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_bytes_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value_bytes
    }

    fn mut_value_bytes_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value_bytes
    }
}

impl ::protobuf::Message for CSOEconItemAttribute {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.def_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.value = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value_bytes)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.def_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.value {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.value_bytes.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.def_index {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.value {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.value_bytes.as_ref() {
            os.write_bytes(3, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSOEconItemAttribute {
    fn new() -> CSOEconItemAttribute {
        CSOEconItemAttribute::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSOEconItemAttribute>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "def_index",
                    CSOEconItemAttribute::get_def_index_for_reflect,
                    CSOEconItemAttribute::mut_def_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "value",
                    CSOEconItemAttribute::get_value_for_reflect,
                    CSOEconItemAttribute::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value_bytes",
                    CSOEconItemAttribute::get_value_bytes_for_reflect,
                    CSOEconItemAttribute::mut_value_bytes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSOEconItemAttribute>(
                    "CSOEconItemAttribute",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSOEconItemAttribute {
    fn clear(&mut self) {
        self.clear_def_index();
        self.clear_value();
        self.clear_value_bytes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSOEconItemAttribute {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSOEconItemAttribute {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSOEconItemEquipped {
    // message fields
    new_class: ::std::option::Option<u32>,
    new_slot: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSOEconItemEquipped {}

impl CSOEconItemEquipped {
    pub fn new() -> CSOEconItemEquipped {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSOEconItemEquipped {
        static mut instance: ::protobuf::lazy::Lazy<CSOEconItemEquipped> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSOEconItemEquipped,
        };
        unsafe {
            instance.get(CSOEconItemEquipped::new)
        }
    }

    // optional uint32 new_class = 1;

    pub fn clear_new_class(&mut self) {
        self.new_class = ::std::option::Option::None;
    }

    pub fn has_new_class(&self) -> bool {
        self.new_class.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_class(&mut self, v: u32) {
        self.new_class = ::std::option::Option::Some(v);
    }

    pub fn get_new_class(&self) -> u32 {
        self.new_class.unwrap_or(0)
    }

    fn get_new_class_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.new_class
    }

    fn mut_new_class_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.new_class
    }

    // optional uint32 new_slot = 2;

    pub fn clear_new_slot(&mut self) {
        self.new_slot = ::std::option::Option::None;
    }

    pub fn has_new_slot(&self) -> bool {
        self.new_slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_slot(&mut self, v: u32) {
        self.new_slot = ::std::option::Option::Some(v);
    }

    pub fn get_new_slot(&self) -> u32 {
        self.new_slot.unwrap_or(0)
    }

    fn get_new_slot_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.new_slot
    }

    fn mut_new_slot_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.new_slot
    }
}

impl ::protobuf::Message for CSOEconItemEquipped {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.new_class = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.new_slot = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.new_class {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.new_slot {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.new_class {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.new_slot {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSOEconItemEquipped {
    fn new() -> CSOEconItemEquipped {
        CSOEconItemEquipped::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSOEconItemEquipped>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "new_class",
                    CSOEconItemEquipped::get_new_class_for_reflect,
                    CSOEconItemEquipped::mut_new_class_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "new_slot",
                    CSOEconItemEquipped::get_new_slot_for_reflect,
                    CSOEconItemEquipped::mut_new_slot_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSOEconItemEquipped>(
                    "CSOEconItemEquipped",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSOEconItemEquipped {
    fn clear(&mut self) {
        self.clear_new_class();
        self.clear_new_slot();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSOEconItemEquipped {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSOEconItemEquipped {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSOEconItem {
    // message fields
    id: ::std::option::Option<u64>,
    account_id: ::std::option::Option<u32>,
    inventory: ::std::option::Option<u32>,
    def_index: ::std::option::Option<u32>,
    quantity: ::std::option::Option<u32>,
    level: ::std::option::Option<u32>,
    quality: ::std::option::Option<u32>,
    flags: ::std::option::Option<u32>,
    origin: ::std::option::Option<u32>,
    attribute: ::protobuf::RepeatedField<CSOEconItemAttribute>,
    interior_item: ::protobuf::SingularPtrField<CSOEconItem>,
    in_use: ::std::option::Option<bool>,
    style: ::std::option::Option<u32>,
    original_id: ::std::option::Option<u64>,
    equipped_state: ::protobuf::RepeatedField<CSOEconItemEquipped>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSOEconItem {}

impl CSOEconItem {
    pub fn new() -> CSOEconItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSOEconItem {
        static mut instance: ::protobuf::lazy::Lazy<CSOEconItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSOEconItem,
        };
        unsafe {
            instance.get(CSOEconItem::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional uint32 account_id = 2;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u32 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id
    }

    // optional uint32 inventory = 3;

    pub fn clear_inventory(&mut self) {
        self.inventory = ::std::option::Option::None;
    }

    pub fn has_inventory(&self) -> bool {
        self.inventory.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inventory(&mut self, v: u32) {
        self.inventory = ::std::option::Option::Some(v);
    }

    pub fn get_inventory(&self) -> u32 {
        self.inventory.unwrap_or(0)
    }

    fn get_inventory_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.inventory
    }

    fn mut_inventory_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.inventory
    }

    // optional uint32 def_index = 4;

    pub fn clear_def_index(&mut self) {
        self.def_index = ::std::option::Option::None;
    }

    pub fn has_def_index(&self) -> bool {
        self.def_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_def_index(&mut self, v: u32) {
        self.def_index = ::std::option::Option::Some(v);
    }

    pub fn get_def_index(&self) -> u32 {
        self.def_index.unwrap_or(0)
    }

    fn get_def_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.def_index
    }

    fn mut_def_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.def_index
    }

    // optional uint32 quantity = 5;

    pub fn clear_quantity(&mut self) {
        self.quantity = ::std::option::Option::None;
    }

    pub fn has_quantity(&self) -> bool {
        self.quantity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quantity(&mut self, v: u32) {
        self.quantity = ::std::option::Option::Some(v);
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity.unwrap_or(1u32)
    }

    fn get_quantity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quantity
    }

    fn mut_quantity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quantity
    }

    // optional uint32 level = 6;

    pub fn clear_level(&mut self) {
        self.level = ::std::option::Option::None;
    }

    pub fn has_level(&self) -> bool {
        self.level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_level(&mut self, v: u32) {
        self.level = ::std::option::Option::Some(v);
    }

    pub fn get_level(&self) -> u32 {
        self.level.unwrap_or(1u32)
    }

    fn get_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.level
    }

    fn mut_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.level
    }

    // optional uint32 quality = 7;

    pub fn clear_quality(&mut self) {
        self.quality = ::std::option::Option::None;
    }

    pub fn has_quality(&self) -> bool {
        self.quality.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality(&mut self, v: u32) {
        self.quality = ::std::option::Option::Some(v);
    }

    pub fn get_quality(&self) -> u32 {
        self.quality.unwrap_or(4u32)
    }

    fn get_quality_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality
    }

    fn mut_quality_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality
    }

    // optional uint32 flags = 8;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: u32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> u32 {
        self.flags.unwrap_or(0u32)
    }

    fn get_flags_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.flags
    }

    fn mut_flags_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.flags
    }

    // optional uint32 origin = 9;

    pub fn clear_origin(&mut self) {
        self.origin = ::std::option::Option::None;
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: u32) {
        self.origin = ::std::option::Option::Some(v);
    }

    pub fn get_origin(&self) -> u32 {
        self.origin.unwrap_or(0u32)
    }

    fn get_origin_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.origin
    }

    fn mut_origin_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.origin
    }

    // repeated .CSOEconItemAttribute attribute = 12;

    pub fn clear_attribute(&mut self) {
        self.attribute.clear();
    }

    // Param is passed by value, moved
    pub fn set_attribute(&mut self, v: ::protobuf::RepeatedField<CSOEconItemAttribute>) {
        self.attribute = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attribute(&mut self) -> &mut ::protobuf::RepeatedField<CSOEconItemAttribute> {
        &mut self.attribute
    }

    // Take field
    pub fn take_attribute(&mut self) -> ::protobuf::RepeatedField<CSOEconItemAttribute> {
        ::std::mem::replace(&mut self.attribute, ::protobuf::RepeatedField::new())
    }

    pub fn get_attribute(&self) -> &[CSOEconItemAttribute] {
        &self.attribute
    }

    fn get_attribute_for_reflect(&self) -> &::protobuf::RepeatedField<CSOEconItemAttribute> {
        &self.attribute
    }

    fn mut_attribute_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSOEconItemAttribute> {
        &mut self.attribute
    }

    // optional .CSOEconItem interior_item = 13;

    pub fn clear_interior_item(&mut self) {
        self.interior_item.clear();
    }

    pub fn has_interior_item(&self) -> bool {
        self.interior_item.is_some()
    }

    // Param is passed by value, moved
    pub fn set_interior_item(&mut self, v: CSOEconItem) {
        self.interior_item = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_interior_item(&mut self) -> &mut CSOEconItem {
        if self.interior_item.is_none() {
            self.interior_item.set_default();
        }
        self.interior_item.as_mut().unwrap()
    }

    // Take field
    pub fn take_interior_item(&mut self) -> CSOEconItem {
        self.interior_item.take().unwrap_or_else(|| CSOEconItem::new())
    }

    pub fn get_interior_item(&self) -> &CSOEconItem {
        self.interior_item.as_ref().unwrap_or_else(|| CSOEconItem::default_instance())
    }

    fn get_interior_item_for_reflect(&self) -> &::protobuf::SingularPtrField<CSOEconItem> {
        &self.interior_item
    }

    fn mut_interior_item_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CSOEconItem> {
        &mut self.interior_item
    }

    // optional bool in_use = 14;

    pub fn clear_in_use(&mut self) {
        self.in_use = ::std::option::Option::None;
    }

    pub fn has_in_use(&self) -> bool {
        self.in_use.is_some()
    }

    // Param is passed by value, moved
    pub fn set_in_use(&mut self, v: bool) {
        self.in_use = ::std::option::Option::Some(v);
    }

    pub fn get_in_use(&self) -> bool {
        self.in_use.unwrap_or(false)
    }

    fn get_in_use_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.in_use
    }

    fn mut_in_use_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.in_use
    }

    // optional uint32 style = 15;

    pub fn clear_style(&mut self) {
        self.style = ::std::option::Option::None;
    }

    pub fn has_style(&self) -> bool {
        self.style.is_some()
    }

    // Param is passed by value, moved
    pub fn set_style(&mut self, v: u32) {
        self.style = ::std::option::Option::Some(v);
    }

    pub fn get_style(&self) -> u32 {
        self.style.unwrap_or(0u32)
    }

    fn get_style_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.style
    }

    fn mut_style_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.style
    }

    // optional uint64 original_id = 16;

    pub fn clear_original_id(&mut self) {
        self.original_id = ::std::option::Option::None;
    }

    pub fn has_original_id(&self) -> bool {
        self.original_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_original_id(&mut self, v: u64) {
        self.original_id = ::std::option::Option::Some(v);
    }

    pub fn get_original_id(&self) -> u64 {
        self.original_id.unwrap_or(0u64)
    }

    fn get_original_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.original_id
    }

    fn mut_original_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.original_id
    }

    // repeated .CSOEconItemEquipped equipped_state = 18;

    pub fn clear_equipped_state(&mut self) {
        self.equipped_state.clear();
    }

    // Param is passed by value, moved
    pub fn set_equipped_state(&mut self, v: ::protobuf::RepeatedField<CSOEconItemEquipped>) {
        self.equipped_state = v;
    }

    // Mutable pointer to the field.
    pub fn mut_equipped_state(&mut self) -> &mut ::protobuf::RepeatedField<CSOEconItemEquipped> {
        &mut self.equipped_state
    }

    // Take field
    pub fn take_equipped_state(&mut self) -> ::protobuf::RepeatedField<CSOEconItemEquipped> {
        ::std::mem::replace(&mut self.equipped_state, ::protobuf::RepeatedField::new())
    }

    pub fn get_equipped_state(&self) -> &[CSOEconItemEquipped] {
        &self.equipped_state
    }

    fn get_equipped_state_for_reflect(&self) -> &::protobuf::RepeatedField<CSOEconItemEquipped> {
        &self.equipped_state
    }

    fn mut_equipped_state_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSOEconItemEquipped> {
        &mut self.equipped_state
    }
}

impl ::protobuf::Message for CSOEconItem {
    fn is_initialized(&self) -> bool {
        for v in &self.attribute {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.interior_item {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.equipped_state {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.inventory = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.def_index = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quantity = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.level = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quality = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.origin = ::std::option::Option::Some(tmp);
                },
                12 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attribute)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.interior_item)?;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.in_use = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.style = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.original_id = ::std::option::Option::Some(tmp);
                },
                18 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.equipped_state)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.inventory {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.def_index {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quantity {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.level {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quality {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.origin {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.attribute {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.interior_item.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.in_use {
            my_size += 2;
        }
        if let Some(v) = self.style {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.original_id {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.equipped_state {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.account_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.inventory {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.def_index {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.quantity {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.level {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.quality {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.flags {
            os.write_uint32(8, v)?;
        }
        if let Some(v) = self.origin {
            os.write_uint32(9, v)?;
        }
        for v in &self.attribute {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.interior_item.as_ref() {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.in_use {
            os.write_bool(14, v)?;
        }
        if let Some(v) = self.style {
            os.write_uint32(15, v)?;
        }
        if let Some(v) = self.original_id {
            os.write_uint64(16, v)?;
        }
        for v in &self.equipped_state {
            os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSOEconItem {
    fn new() -> CSOEconItem {
        CSOEconItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSOEconItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    CSOEconItem::get_id_for_reflect,
                    CSOEconItem::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CSOEconItem::get_account_id_for_reflect,
                    CSOEconItem::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "inventory",
                    CSOEconItem::get_inventory_for_reflect,
                    CSOEconItem::mut_inventory_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "def_index",
                    CSOEconItem::get_def_index_for_reflect,
                    CSOEconItem::mut_def_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quantity",
                    CSOEconItem::get_quantity_for_reflect,
                    CSOEconItem::mut_quantity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "level",
                    CSOEconItem::get_level_for_reflect,
                    CSOEconItem::mut_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality",
                    CSOEconItem::get_quality_for_reflect,
                    CSOEconItem::mut_quality_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CSOEconItem::get_flags_for_reflect,
                    CSOEconItem::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "origin",
                    CSOEconItem::get_origin_for_reflect,
                    CSOEconItem::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSOEconItemAttribute>>(
                    "attribute",
                    CSOEconItem::get_attribute_for_reflect,
                    CSOEconItem::mut_attribute_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSOEconItem>>(
                    "interior_item",
                    CSOEconItem::get_interior_item_for_reflect,
                    CSOEconItem::mut_interior_item_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "in_use",
                    CSOEconItem::get_in_use_for_reflect,
                    CSOEconItem::mut_in_use_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "style",
                    CSOEconItem::get_style_for_reflect,
                    CSOEconItem::mut_style_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "original_id",
                    CSOEconItem::get_original_id_for_reflect,
                    CSOEconItem::mut_original_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSOEconItemEquipped>>(
                    "equipped_state",
                    CSOEconItem::get_equipped_state_for_reflect,
                    CSOEconItem::mut_equipped_state_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSOEconItem>(
                    "CSOEconItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSOEconItem {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_account_id();
        self.clear_inventory();
        self.clear_def_index();
        self.clear_quantity();
        self.clear_level();
        self.clear_quality();
        self.clear_flags();
        self.clear_origin();
        self.clear_attribute();
        self.clear_interior_item();
        self.clear_in_use();
        self.clear_style();
        self.clear_original_id();
        self.clear_equipped_state();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSOEconItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSOEconItem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSortItems {
    // message fields
    sort_type: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSortItems {}

impl CMsgSortItems {
    pub fn new() -> CMsgSortItems {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSortItems {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSortItems> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSortItems,
        };
        unsafe {
            instance.get(CMsgSortItems::new)
        }
    }

    // optional uint32 sort_type = 1;

    pub fn clear_sort_type(&mut self) {
        self.sort_type = ::std::option::Option::None;
    }

    pub fn has_sort_type(&self) -> bool {
        self.sort_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sort_type(&mut self, v: u32) {
        self.sort_type = ::std::option::Option::Some(v);
    }

    pub fn get_sort_type(&self) -> u32 {
        self.sort_type.unwrap_or(0)
    }

    fn get_sort_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sort_type
    }

    fn mut_sort_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sort_type
    }
}

impl ::protobuf::Message for CMsgSortItems {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.sort_type = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.sort_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sort_type {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgSortItems {
    fn new() -> CMsgSortItems {
        CMsgSortItems::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSortItems>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sort_type",
                    CMsgSortItems::get_sort_type_for_reflect,
                    CMsgSortItems::mut_sort_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSortItems>(
                    "CMsgSortItems",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSortItems {
    fn clear(&mut self) {
        self.clear_sort_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSortItems {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSortItems {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSOEconClaimCode {
    // message fields
    account_id: ::std::option::Option<u32>,
    code_type: ::std::option::Option<u32>,
    time_acquired: ::std::option::Option<u32>,
    code: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSOEconClaimCode {}

impl CSOEconClaimCode {
    pub fn new() -> CSOEconClaimCode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSOEconClaimCode {
        static mut instance: ::protobuf::lazy::Lazy<CSOEconClaimCode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSOEconClaimCode,
        };
        unsafe {
            instance.get(CSOEconClaimCode::new)
        }
    }

    // optional uint32 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u32 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id
    }

    // optional uint32 code_type = 2;

    pub fn clear_code_type(&mut self) {
        self.code_type = ::std::option::Option::None;
    }

    pub fn has_code_type(&self) -> bool {
        self.code_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code_type(&mut self, v: u32) {
        self.code_type = ::std::option::Option::Some(v);
    }

    pub fn get_code_type(&self) -> u32 {
        self.code_type.unwrap_or(0)
    }

    fn get_code_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.code_type
    }

    fn mut_code_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.code_type
    }

    // optional uint32 time_acquired = 3;

    pub fn clear_time_acquired(&mut self) {
        self.time_acquired = ::std::option::Option::None;
    }

    pub fn has_time_acquired(&self) -> bool {
        self.time_acquired.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_acquired(&mut self, v: u32) {
        self.time_acquired = ::std::option::Option::Some(v);
    }

    pub fn get_time_acquired(&self) -> u32 {
        self.time_acquired.unwrap_or(0)
    }

    fn get_time_acquired_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_acquired
    }

    fn mut_time_acquired_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_acquired
    }

    // optional string code = 4;

    pub fn clear_code(&mut self) {
        self.code.clear();
    }

    pub fn has_code(&self) -> bool {
        self.code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: ::std::string::String) {
        self.code = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_code(&mut self) -> &mut ::std::string::String {
        if self.code.is_none() {
            self.code.set_default();
        }
        self.code.as_mut().unwrap()
    }

    // Take field
    pub fn take_code(&mut self) -> ::std::string::String {
        self.code.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_code(&self) -> &str {
        match self.code.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_code_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.code
    }
}

impl ::protobuf::Message for CSOEconClaimCode {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.code_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.time_acquired = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.code)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.code_type {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.time_acquired {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.code.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.code_type {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.time_acquired {
            os.write_uint32(3, v)?;
        }
        if let Some(ref v) = self.code.as_ref() {
            os.write_string(4, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSOEconClaimCode {
    fn new() -> CSOEconClaimCode {
        CSOEconClaimCode::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSOEconClaimCode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CSOEconClaimCode::get_account_id_for_reflect,
                    CSOEconClaimCode::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "code_type",
                    CSOEconClaimCode::get_code_type_for_reflect,
                    CSOEconClaimCode::mut_code_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_acquired",
                    CSOEconClaimCode::get_time_acquired_for_reflect,
                    CSOEconClaimCode::mut_time_acquired_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "code",
                    CSOEconClaimCode::get_code_for_reflect,
                    CSOEconClaimCode::mut_code_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSOEconClaimCode>(
                    "CSOEconClaimCode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSOEconClaimCode {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_code_type();
        self.clear_time_acquired();
        self.clear_code();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSOEconClaimCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSOEconClaimCode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgStoreGetUserData {
    // message fields
    price_sheet_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgStoreGetUserData {}

impl CMsgStoreGetUserData {
    pub fn new() -> CMsgStoreGetUserData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgStoreGetUserData {
        static mut instance: ::protobuf::lazy::Lazy<CMsgStoreGetUserData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgStoreGetUserData,
        };
        unsafe {
            instance.get(CMsgStoreGetUserData::new)
        }
    }

    // optional fixed32 price_sheet_version = 1;

    pub fn clear_price_sheet_version(&mut self) {
        self.price_sheet_version = ::std::option::Option::None;
    }

    pub fn has_price_sheet_version(&self) -> bool {
        self.price_sheet_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_price_sheet_version(&mut self, v: u32) {
        self.price_sheet_version = ::std::option::Option::Some(v);
    }

    pub fn get_price_sheet_version(&self) -> u32 {
        self.price_sheet_version.unwrap_or(0)
    }

    fn get_price_sheet_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.price_sheet_version
    }

    fn mut_price_sheet_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.price_sheet_version
    }
}

impl ::protobuf::Message for CMsgStoreGetUserData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.price_sheet_version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.price_sheet_version {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.price_sheet_version {
            os.write_fixed32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgStoreGetUserData {
    fn new() -> CMsgStoreGetUserData {
        CMsgStoreGetUserData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgStoreGetUserData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "price_sheet_version",
                    CMsgStoreGetUserData::get_price_sheet_version_for_reflect,
                    CMsgStoreGetUserData::mut_price_sheet_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgStoreGetUserData>(
                    "CMsgStoreGetUserData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgStoreGetUserData {
    fn clear(&mut self) {
        self.clear_price_sheet_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgStoreGetUserData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgStoreGetUserData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgStoreGetUserDataResponse {
    // message fields
    result: ::std::option::Option<i32>,
    currency: ::std::option::Option<i32>,
    country: ::protobuf::SingularField<::std::string::String>,
    price_sheet_version: ::std::option::Option<u32>,
    experiment_data: ::std::option::Option<u64>,
    featured_item_idx: ::std::option::Option<i32>,
    show_hat_descriptions: ::std::option::Option<bool>,
    price_sheet: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    default_item_sort: ::std::option::Option<i32>,
    popular_items: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgStoreGetUserDataResponse {}

impl CMsgStoreGetUserDataResponse {
    pub fn new() -> CMsgStoreGetUserDataResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgStoreGetUserDataResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgStoreGetUserDataResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgStoreGetUserDataResponse,
        };
        unsafe {
            instance.get(CMsgStoreGetUserDataResponse::new)
        }
    }

    // optional int32 result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: i32) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> i32 {
        self.result.unwrap_or(0)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.result
    }

    // optional int32 currency = 2;

    pub fn clear_currency(&mut self) {
        self.currency = ::std::option::Option::None;
    }

    pub fn has_currency(&self) -> bool {
        self.currency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currency(&mut self, v: i32) {
        self.currency = ::std::option::Option::Some(v);
    }

    pub fn get_currency(&self) -> i32 {
        self.currency.unwrap_or(0)
    }

    fn get_currency_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.currency
    }

    fn mut_currency_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.currency
    }

    // optional string country = 3;

    pub fn clear_country(&mut self) {
        self.country.clear();
    }

    pub fn has_country(&self) -> bool {
        self.country.is_some()
    }

    // Param is passed by value, moved
    pub fn set_country(&mut self, v: ::std::string::String) {
        self.country = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_country(&mut self) -> &mut ::std::string::String {
        if self.country.is_none() {
            self.country.set_default();
        }
        self.country.as_mut().unwrap()
    }

    // Take field
    pub fn take_country(&mut self) -> ::std::string::String {
        self.country.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_country(&self) -> &str {
        match self.country.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_country_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.country
    }

    fn mut_country_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.country
    }

    // optional fixed32 price_sheet_version = 4;

    pub fn clear_price_sheet_version(&mut self) {
        self.price_sheet_version = ::std::option::Option::None;
    }

    pub fn has_price_sheet_version(&self) -> bool {
        self.price_sheet_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_price_sheet_version(&mut self, v: u32) {
        self.price_sheet_version = ::std::option::Option::Some(v);
    }

    pub fn get_price_sheet_version(&self) -> u32 {
        self.price_sheet_version.unwrap_or(0)
    }

    fn get_price_sheet_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.price_sheet_version
    }

    fn mut_price_sheet_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.price_sheet_version
    }

    // optional uint64 experiment_data = 5;

    pub fn clear_experiment_data(&mut self) {
        self.experiment_data = ::std::option::Option::None;
    }

    pub fn has_experiment_data(&self) -> bool {
        self.experiment_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_experiment_data(&mut self, v: u64) {
        self.experiment_data = ::std::option::Option::Some(v);
    }

    pub fn get_experiment_data(&self) -> u64 {
        self.experiment_data.unwrap_or(0u64)
    }

    fn get_experiment_data_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.experiment_data
    }

    fn mut_experiment_data_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.experiment_data
    }

    // optional int32 featured_item_idx = 6;

    pub fn clear_featured_item_idx(&mut self) {
        self.featured_item_idx = ::std::option::Option::None;
    }

    pub fn has_featured_item_idx(&self) -> bool {
        self.featured_item_idx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_featured_item_idx(&mut self, v: i32) {
        self.featured_item_idx = ::std::option::Option::Some(v);
    }

    pub fn get_featured_item_idx(&self) -> i32 {
        self.featured_item_idx.unwrap_or(0)
    }

    fn get_featured_item_idx_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.featured_item_idx
    }

    fn mut_featured_item_idx_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.featured_item_idx
    }

    // optional bool show_hat_descriptions = 7;

    pub fn clear_show_hat_descriptions(&mut self) {
        self.show_hat_descriptions = ::std::option::Option::None;
    }

    pub fn has_show_hat_descriptions(&self) -> bool {
        self.show_hat_descriptions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_show_hat_descriptions(&mut self, v: bool) {
        self.show_hat_descriptions = ::std::option::Option::Some(v);
    }

    pub fn get_show_hat_descriptions(&self) -> bool {
        self.show_hat_descriptions.unwrap_or(true)
    }

    fn get_show_hat_descriptions_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.show_hat_descriptions
    }

    fn mut_show_hat_descriptions_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.show_hat_descriptions
    }

    // optional bytes price_sheet = 8;

    pub fn clear_price_sheet(&mut self) {
        self.price_sheet.clear();
    }

    pub fn has_price_sheet(&self) -> bool {
        self.price_sheet.is_some()
    }

    // Param is passed by value, moved
    pub fn set_price_sheet(&mut self, v: ::std::vec::Vec<u8>) {
        self.price_sheet = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_price_sheet(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.price_sheet.is_none() {
            self.price_sheet.set_default();
        }
        self.price_sheet.as_mut().unwrap()
    }

    // Take field
    pub fn take_price_sheet(&mut self) -> ::std::vec::Vec<u8> {
        self.price_sheet.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_price_sheet(&self) -> &[u8] {
        match self.price_sheet.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_price_sheet_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.price_sheet
    }

    fn mut_price_sheet_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.price_sheet
    }

    // optional int32 default_item_sort = 9;

    pub fn clear_default_item_sort(&mut self) {
        self.default_item_sort = ::std::option::Option::None;
    }

    pub fn has_default_item_sort(&self) -> bool {
        self.default_item_sort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_default_item_sort(&mut self, v: i32) {
        self.default_item_sort = ::std::option::Option::Some(v);
    }

    pub fn get_default_item_sort(&self) -> i32 {
        self.default_item_sort.unwrap_or(0i32)
    }

    fn get_default_item_sort_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.default_item_sort
    }

    fn mut_default_item_sort_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.default_item_sort
    }

    // repeated uint32 popular_items = 10;

    pub fn clear_popular_items(&mut self) {
        self.popular_items.clear();
    }

    // Param is passed by value, moved
    pub fn set_popular_items(&mut self, v: ::std::vec::Vec<u32>) {
        self.popular_items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_popular_items(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.popular_items
    }

    // Take field
    pub fn take_popular_items(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.popular_items, ::std::vec::Vec::new())
    }

    pub fn get_popular_items(&self) -> &[u32] {
        &self.popular_items
    }

    fn get_popular_items_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.popular_items
    }

    fn mut_popular_items_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.popular_items
    }
}

impl ::protobuf::Message for CMsgStoreGetUserDataResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.currency = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.country)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.price_sheet_version = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.experiment_data = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.featured_item_idx = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.show_hat_descriptions = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.price_sheet)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.default_item_sort = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.popular_items)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.currency {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.country.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.price_sheet_version {
            my_size += 5;
        }
        if let Some(v) = self.experiment_data {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.featured_item_idx {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.show_hat_descriptions {
            my_size += 2;
        }
        if let Some(ref v) = self.price_sheet.as_ref() {
            my_size += ::protobuf::rt::bytes_size(8, &v);
        }
        if let Some(v) = self.default_item_sort {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.popular_items {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.currency {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.country.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.price_sheet_version {
            os.write_fixed32(4, v)?;
        }
        if let Some(v) = self.experiment_data {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.featured_item_idx {
            os.write_int32(6, v)?;
        }
        if let Some(v) = self.show_hat_descriptions {
            os.write_bool(7, v)?;
        }
        if let Some(ref v) = self.price_sheet.as_ref() {
            os.write_bytes(8, &v)?;
        }
        if let Some(v) = self.default_item_sort {
            os.write_int32(9, v)?;
        }
        for v in &self.popular_items {
            os.write_uint32(10, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgStoreGetUserDataResponse {
    fn new() -> CMsgStoreGetUserDataResponse {
        CMsgStoreGetUserDataResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgStoreGetUserDataResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "result",
                    CMsgStoreGetUserDataResponse::get_result_for_reflect,
                    CMsgStoreGetUserDataResponse::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "currency",
                    CMsgStoreGetUserDataResponse::get_currency_for_reflect,
                    CMsgStoreGetUserDataResponse::mut_currency_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "country",
                    CMsgStoreGetUserDataResponse::get_country_for_reflect,
                    CMsgStoreGetUserDataResponse::mut_country_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "price_sheet_version",
                    CMsgStoreGetUserDataResponse::get_price_sheet_version_for_reflect,
                    CMsgStoreGetUserDataResponse::mut_price_sheet_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "experiment_data",
                    CMsgStoreGetUserDataResponse::get_experiment_data_for_reflect,
                    CMsgStoreGetUserDataResponse::mut_experiment_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "featured_item_idx",
                    CMsgStoreGetUserDataResponse::get_featured_item_idx_for_reflect,
                    CMsgStoreGetUserDataResponse::mut_featured_item_idx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "show_hat_descriptions",
                    CMsgStoreGetUserDataResponse::get_show_hat_descriptions_for_reflect,
                    CMsgStoreGetUserDataResponse::mut_show_hat_descriptions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "price_sheet",
                    CMsgStoreGetUserDataResponse::get_price_sheet_for_reflect,
                    CMsgStoreGetUserDataResponse::mut_price_sheet_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "default_item_sort",
                    CMsgStoreGetUserDataResponse::get_default_item_sort_for_reflect,
                    CMsgStoreGetUserDataResponse::mut_default_item_sort_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "popular_items",
                    CMsgStoreGetUserDataResponse::get_popular_items_for_reflect,
                    CMsgStoreGetUserDataResponse::mut_popular_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgStoreGetUserDataResponse>(
                    "CMsgStoreGetUserDataResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgStoreGetUserDataResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_currency();
        self.clear_country();
        self.clear_price_sheet_version();
        self.clear_experiment_data();
        self.clear_featured_item_idx();
        self.clear_show_hat_descriptions();
        self.clear_price_sheet();
        self.clear_default_item_sort();
        self.clear_popular_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgStoreGetUserDataResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgStoreGetUserDataResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgUpdateItemSchema {
    // message fields
    items_game: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    item_schema_version: ::std::option::Option<u32>,
    items_game_url: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgUpdateItemSchema {}

impl CMsgUpdateItemSchema {
    pub fn new() -> CMsgUpdateItemSchema {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgUpdateItemSchema {
        static mut instance: ::protobuf::lazy::Lazy<CMsgUpdateItemSchema> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgUpdateItemSchema,
        };
        unsafe {
            instance.get(CMsgUpdateItemSchema::new)
        }
    }

    // optional bytes items_game = 1;

    pub fn clear_items_game(&mut self) {
        self.items_game.clear();
    }

    pub fn has_items_game(&self) -> bool {
        self.items_game.is_some()
    }

    // Param is passed by value, moved
    pub fn set_items_game(&mut self, v: ::std::vec::Vec<u8>) {
        self.items_game = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_items_game(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.items_game.is_none() {
            self.items_game.set_default();
        }
        self.items_game.as_mut().unwrap()
    }

    // Take field
    pub fn take_items_game(&mut self) -> ::std::vec::Vec<u8> {
        self.items_game.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_items_game(&self) -> &[u8] {
        match self.items_game.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_items_game_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.items_game
    }

    fn mut_items_game_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.items_game
    }

    // optional fixed32 item_schema_version = 2;

    pub fn clear_item_schema_version(&mut self) {
        self.item_schema_version = ::std::option::Option::None;
    }

    pub fn has_item_schema_version(&self) -> bool {
        self.item_schema_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_schema_version(&mut self, v: u32) {
        self.item_schema_version = ::std::option::Option::Some(v);
    }

    pub fn get_item_schema_version(&self) -> u32 {
        self.item_schema_version.unwrap_or(0)
    }

    fn get_item_schema_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_schema_version
    }

    fn mut_item_schema_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_schema_version
    }

    // optional string items_game_url = 3;

    pub fn clear_items_game_url(&mut self) {
        self.items_game_url.clear();
    }

    pub fn has_items_game_url(&self) -> bool {
        self.items_game_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_items_game_url(&mut self, v: ::std::string::String) {
        self.items_game_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_items_game_url(&mut self) -> &mut ::std::string::String {
        if self.items_game_url.is_none() {
            self.items_game_url.set_default();
        }
        self.items_game_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_items_game_url(&mut self) -> ::std::string::String {
        self.items_game_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_items_game_url(&self) -> &str {
        match self.items_game_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_items_game_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.items_game_url
    }

    fn mut_items_game_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.items_game_url
    }
}

impl ::protobuf::Message for CMsgUpdateItemSchema {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.items_game)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.item_schema_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.items_game_url)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.items_game.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.item_schema_version {
            my_size += 5;
        }
        if let Some(ref v) = self.items_game_url.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.items_game.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(v) = self.item_schema_version {
            os.write_fixed32(2, v)?;
        }
        if let Some(ref v) = self.items_game_url.as_ref() {
            os.write_string(3, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgUpdateItemSchema {
    fn new() -> CMsgUpdateItemSchema {
        CMsgUpdateItemSchema::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgUpdateItemSchema>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "items_game",
                    CMsgUpdateItemSchema::get_items_game_for_reflect,
                    CMsgUpdateItemSchema::mut_items_game_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "item_schema_version",
                    CMsgUpdateItemSchema::get_item_schema_version_for_reflect,
                    CMsgUpdateItemSchema::mut_item_schema_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "items_game_url",
                    CMsgUpdateItemSchema::get_items_game_url_for_reflect,
                    CMsgUpdateItemSchema::mut_items_game_url_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgUpdateItemSchema>(
                    "CMsgUpdateItemSchema",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgUpdateItemSchema {
    fn clear(&mut self) {
        self.clear_items_game();
        self.clear_item_schema_version();
        self.clear_items_game_url();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgUpdateItemSchema {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgUpdateItemSchema {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCError {
    // message fields
    error_text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCError {}

impl CMsgGCError {
    pub fn new() -> CMsgGCError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCError {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCError,
        };
        unsafe {
            instance.get(CMsgGCError::new)
        }
    }

    // optional string error_text = 1;

    pub fn clear_error_text(&mut self) {
        self.error_text.clear();
    }

    pub fn has_error_text(&self) -> bool {
        self.error_text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_text(&mut self, v: ::std::string::String) {
        self.error_text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_text(&mut self) -> &mut ::std::string::String {
        if self.error_text.is_none() {
            self.error_text.set_default();
        }
        self.error_text.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_text(&mut self) -> ::std::string::String {
        self.error_text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_text(&self) -> &str {
        match self.error_text.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_text_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error_text
    }

    fn mut_error_text_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error_text
    }
}

impl ::protobuf::Message for CMsgGCError {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_text)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.error_text.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.error_text.as_ref() {
            os.write_string(1, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCError {
    fn new() -> CMsgGCError {
        CMsgGCError::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_text",
                    CMsgGCError::get_error_text_for_reflect,
                    CMsgGCError::mut_error_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCError>(
                    "CMsgGCError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCError {
    fn clear(&mut self) {
        self.clear_error_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCError {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRequestInventoryRefresh {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRequestInventoryRefresh {}

impl CMsgRequestInventoryRefresh {
    pub fn new() -> CMsgRequestInventoryRefresh {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRequestInventoryRefresh {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRequestInventoryRefresh> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRequestInventoryRefresh,
        };
        unsafe {
            instance.get(CMsgRequestInventoryRefresh::new)
        }
    }
}

impl ::protobuf::Message for CMsgRequestInventoryRefresh {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRequestInventoryRefresh {
    fn new() -> CMsgRequestInventoryRefresh {
        CMsgRequestInventoryRefresh::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRequestInventoryRefresh>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRequestInventoryRefresh>(
                    "CMsgRequestInventoryRefresh",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRequestInventoryRefresh {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRequestInventoryRefresh {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRequestInventoryRefresh {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgConVarValue {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgConVarValue {}

impl CMsgConVarValue {
    pub fn new() -> CMsgConVarValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgConVarValue {
        static mut instance: ::protobuf::lazy::Lazy<CMsgConVarValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgConVarValue,
        };
        unsafe {
            instance.get(CMsgConVarValue::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.value
    }
}

impl ::protobuf::Message for CMsgConVarValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgConVarValue {
    fn new() -> CMsgConVarValue {
        CMsgConVarValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgConVarValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgConVarValue::get_name_for_reflect,
                    CMsgConVarValue::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    CMsgConVarValue::get_value_for_reflect,
                    CMsgConVarValue::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgConVarValue>(
                    "CMsgConVarValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgConVarValue {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgConVarValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgConVarValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgReplicateConVars {
    // message fields
    convars: ::protobuf::RepeatedField<CMsgConVarValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgReplicateConVars {}

impl CMsgReplicateConVars {
    pub fn new() -> CMsgReplicateConVars {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgReplicateConVars {
        static mut instance: ::protobuf::lazy::Lazy<CMsgReplicateConVars> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgReplicateConVars,
        };
        unsafe {
            instance.get(CMsgReplicateConVars::new)
        }
    }

    // repeated .CMsgConVarValue convars = 1;

    pub fn clear_convars(&mut self) {
        self.convars.clear();
    }

    // Param is passed by value, moved
    pub fn set_convars(&mut self, v: ::protobuf::RepeatedField<CMsgConVarValue>) {
        self.convars = v;
    }

    // Mutable pointer to the field.
    pub fn mut_convars(&mut self) -> &mut ::protobuf::RepeatedField<CMsgConVarValue> {
        &mut self.convars
    }

    // Take field
    pub fn take_convars(&mut self) -> ::protobuf::RepeatedField<CMsgConVarValue> {
        ::std::mem::replace(&mut self.convars, ::protobuf::RepeatedField::new())
    }

    pub fn get_convars(&self) -> &[CMsgConVarValue] {
        &self.convars
    }

    fn get_convars_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgConVarValue> {
        &self.convars
    }

    fn mut_convars_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgConVarValue> {
        &mut self.convars
    }
}

impl ::protobuf::Message for CMsgReplicateConVars {
    fn is_initialized(&self) -> bool {
        for v in &self.convars {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.convars)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.convars {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.convars {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgReplicateConVars {
    fn new() -> CMsgReplicateConVars {
        CMsgReplicateConVars::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgReplicateConVars>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgConVarValue>>(
                    "convars",
                    CMsgReplicateConVars::get_convars_for_reflect,
                    CMsgReplicateConVars::mut_convars_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgReplicateConVars>(
                    "CMsgReplicateConVars",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgReplicateConVars {
    fn clear(&mut self) {
        self.clear_convars();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgReplicateConVars {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgReplicateConVars {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgConsumableExhausted {
    // message fields
    item_def_id: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgConsumableExhausted {}

impl CMsgConsumableExhausted {
    pub fn new() -> CMsgConsumableExhausted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgConsumableExhausted {
        static mut instance: ::protobuf::lazy::Lazy<CMsgConsumableExhausted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgConsumableExhausted,
        };
        unsafe {
            instance.get(CMsgConsumableExhausted::new)
        }
    }

    // optional int32 item_def_id = 1;

    pub fn clear_item_def_id(&mut self) {
        self.item_def_id = ::std::option::Option::None;
    }

    pub fn has_item_def_id(&self) -> bool {
        self.item_def_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_def_id(&mut self, v: i32) {
        self.item_def_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_def_id(&self) -> i32 {
        self.item_def_id.unwrap_or(0)
    }

    fn get_item_def_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.item_def_id
    }

    fn mut_item_def_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.item_def_id
    }
}

impl ::protobuf::Message for CMsgConsumableExhausted {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.item_def_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.item_def_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_def_id {
            os.write_int32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgConsumableExhausted {
    fn new() -> CMsgConsumableExhausted {
        CMsgConsumableExhausted::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgConsumableExhausted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "item_def_id",
                    CMsgConsumableExhausted::get_item_def_id_for_reflect,
                    CMsgConsumableExhausted::mut_item_def_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgConsumableExhausted>(
                    "CMsgConsumableExhausted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgConsumableExhausted {
    fn clear(&mut self) {
        self.clear_item_def_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgConsumableExhausted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgConsumableExhausted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgItemAcknowledged {
    // message fields
    account_id: ::std::option::Option<u32>,
    inventory: ::std::option::Option<u32>,
    def_index: ::std::option::Option<u32>,
    quality: ::std::option::Option<u32>,
    rarity: ::std::option::Option<u32>,
    origin: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgItemAcknowledged {}

impl CMsgItemAcknowledged {
    pub fn new() -> CMsgItemAcknowledged {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgItemAcknowledged {
        static mut instance: ::protobuf::lazy::Lazy<CMsgItemAcknowledged> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgItemAcknowledged,
        };
        unsafe {
            instance.get(CMsgItemAcknowledged::new)
        }
    }

    // optional uint32 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u32 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id
    }

    // optional uint32 inventory = 2;

    pub fn clear_inventory(&mut self) {
        self.inventory = ::std::option::Option::None;
    }

    pub fn has_inventory(&self) -> bool {
        self.inventory.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inventory(&mut self, v: u32) {
        self.inventory = ::std::option::Option::Some(v);
    }

    pub fn get_inventory(&self) -> u32 {
        self.inventory.unwrap_or(0)
    }

    fn get_inventory_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.inventory
    }

    fn mut_inventory_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.inventory
    }

    // optional uint32 def_index = 3;

    pub fn clear_def_index(&mut self) {
        self.def_index = ::std::option::Option::None;
    }

    pub fn has_def_index(&self) -> bool {
        self.def_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_def_index(&mut self, v: u32) {
        self.def_index = ::std::option::Option::Some(v);
    }

    pub fn get_def_index(&self) -> u32 {
        self.def_index.unwrap_or(0)
    }

    fn get_def_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.def_index
    }

    fn mut_def_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.def_index
    }

    // optional uint32 quality = 4;

    pub fn clear_quality(&mut self) {
        self.quality = ::std::option::Option::None;
    }

    pub fn has_quality(&self) -> bool {
        self.quality.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality(&mut self, v: u32) {
        self.quality = ::std::option::Option::Some(v);
    }

    pub fn get_quality(&self) -> u32 {
        self.quality.unwrap_or(0)
    }

    fn get_quality_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality
    }

    fn mut_quality_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality
    }

    // optional uint32 rarity = 5;

    pub fn clear_rarity(&mut self) {
        self.rarity = ::std::option::Option::None;
    }

    pub fn has_rarity(&self) -> bool {
        self.rarity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rarity(&mut self, v: u32) {
        self.rarity = ::std::option::Option::Some(v);
    }

    pub fn get_rarity(&self) -> u32 {
        self.rarity.unwrap_or(0)
    }

    fn get_rarity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.rarity
    }

    fn mut_rarity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.rarity
    }

    // optional uint32 origin = 6;

    pub fn clear_origin(&mut self) {
        self.origin = ::std::option::Option::None;
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: u32) {
        self.origin = ::std::option::Option::Some(v);
    }

    pub fn get_origin(&self) -> u32 {
        self.origin.unwrap_or(0)
    }

    fn get_origin_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.origin
    }

    fn mut_origin_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.origin
    }
}

impl ::protobuf::Message for CMsgItemAcknowledged {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.inventory = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.def_index = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quality = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.rarity = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.origin = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.inventory {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.def_index {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quality {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.rarity {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.origin {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.inventory {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.def_index {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.quality {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.rarity {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.origin {
            os.write_uint32(6, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgItemAcknowledged {
    fn new() -> CMsgItemAcknowledged {
        CMsgItemAcknowledged::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgItemAcknowledged>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgItemAcknowledged::get_account_id_for_reflect,
                    CMsgItemAcknowledged::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "inventory",
                    CMsgItemAcknowledged::get_inventory_for_reflect,
                    CMsgItemAcknowledged::mut_inventory_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "def_index",
                    CMsgItemAcknowledged::get_def_index_for_reflect,
                    CMsgItemAcknowledged::mut_def_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality",
                    CMsgItemAcknowledged::get_quality_for_reflect,
                    CMsgItemAcknowledged::mut_quality_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "rarity",
                    CMsgItemAcknowledged::get_rarity_for_reflect,
                    CMsgItemAcknowledged::mut_rarity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "origin",
                    CMsgItemAcknowledged::get_origin_for_reflect,
                    CMsgItemAcknowledged::mut_origin_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgItemAcknowledged>(
                    "CMsgItemAcknowledged",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgItemAcknowledged {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_inventory();
        self.clear_def_index();
        self.clear_quality();
        self.clear_rarity();
        self.clear_origin();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgItemAcknowledged {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgItemAcknowledged {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSetItemPositions {
    // message fields
    item_positions: ::protobuf::RepeatedField<CMsgSetItemPositions_ItemPosition>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSetItemPositions {}

impl CMsgSetItemPositions {
    pub fn new() -> CMsgSetItemPositions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSetItemPositions {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSetItemPositions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSetItemPositions,
        };
        unsafe {
            instance.get(CMsgSetItemPositions::new)
        }
    }

    // repeated .CMsgSetItemPositions.ItemPosition item_positions = 1;

    pub fn clear_item_positions(&mut self) {
        self.item_positions.clear();
    }

    // Param is passed by value, moved
    pub fn set_item_positions(&mut self, v: ::protobuf::RepeatedField<CMsgSetItemPositions_ItemPosition>) {
        self.item_positions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_item_positions(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSetItemPositions_ItemPosition> {
        &mut self.item_positions
    }

    // Take field
    pub fn take_item_positions(&mut self) -> ::protobuf::RepeatedField<CMsgSetItemPositions_ItemPosition> {
        ::std::mem::replace(&mut self.item_positions, ::protobuf::RepeatedField::new())
    }

    pub fn get_item_positions(&self) -> &[CMsgSetItemPositions_ItemPosition] {
        &self.item_positions
    }

    fn get_item_positions_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSetItemPositions_ItemPosition> {
        &self.item_positions
    }

    fn mut_item_positions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSetItemPositions_ItemPosition> {
        &mut self.item_positions
    }
}

impl ::protobuf::Message for CMsgSetItemPositions {
    fn is_initialized(&self) -> bool {
        for v in &self.item_positions {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.item_positions)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.item_positions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.item_positions {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgSetItemPositions {
    fn new() -> CMsgSetItemPositions {
        CMsgSetItemPositions::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSetItemPositions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSetItemPositions_ItemPosition>>(
                    "item_positions",
                    CMsgSetItemPositions::get_item_positions_for_reflect,
                    CMsgSetItemPositions::mut_item_positions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSetItemPositions>(
                    "CMsgSetItemPositions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSetItemPositions {
    fn clear(&mut self) {
        self.clear_item_positions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSetItemPositions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSetItemPositions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSetItemPositions_ItemPosition {
    // message fields
    item_id: ::std::option::Option<u64>,
    position: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSetItemPositions_ItemPosition {}

impl CMsgSetItemPositions_ItemPosition {
    pub fn new() -> CMsgSetItemPositions_ItemPosition {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSetItemPositions_ItemPosition {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSetItemPositions_ItemPosition> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSetItemPositions_ItemPosition,
        };
        unsafe {
            instance.get(CMsgSetItemPositions_ItemPosition::new)
        }
    }

    // optional uint64 item_id = 1;

    pub fn clear_item_id(&mut self) {
        self.item_id = ::std::option::Option::None;
    }

    pub fn has_item_id(&self) -> bool {
        self.item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_id(&mut self, v: u64) {
        self.item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_id(&self) -> u64 {
        self.item_id.unwrap_or(0)
    }

    fn get_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.item_id
    }

    fn mut_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.item_id
    }

    // optional uint32 position = 2;

    pub fn clear_position(&mut self) {
        self.position = ::std::option::Option::None;
    }

    pub fn has_position(&self) -> bool {
        self.position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position(&mut self, v: u32) {
        self.position = ::std::option::Option::Some(v);
    }

    pub fn get_position(&self) -> u32 {
        self.position.unwrap_or(0)
    }

    fn get_position_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.position
    }

    fn mut_position_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.position
    }
}

impl ::protobuf::Message for CMsgSetItemPositions_ItemPosition {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.position = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.position {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.position {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgSetItemPositions_ItemPosition {
    fn new() -> CMsgSetItemPositions_ItemPosition {
        CMsgSetItemPositions_ItemPosition::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSetItemPositions_ItemPosition>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "item_id",
                    CMsgSetItemPositions_ItemPosition::get_item_id_for_reflect,
                    CMsgSetItemPositions_ItemPosition::mut_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "position",
                    CMsgSetItemPositions_ItemPosition::get_position_for_reflect,
                    CMsgSetItemPositions_ItemPosition::mut_position_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSetItemPositions_ItemPosition>(
                    "CMsgSetItemPositions_ItemPosition",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSetItemPositions_ItemPosition {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_position();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSetItemPositions_ItemPosition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSetItemPositions_ItemPosition {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCNameItemNotification {
    // message fields
    player_steamid: ::std::option::Option<u64>,
    item_def_index: ::std::option::Option<u32>,
    item_name_custom: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCNameItemNotification {}

impl CMsgGCNameItemNotification {
    pub fn new() -> CMsgGCNameItemNotification {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCNameItemNotification {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCNameItemNotification> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCNameItemNotification,
        };
        unsafe {
            instance.get(CMsgGCNameItemNotification::new)
        }
    }

    // optional fixed64 player_steamid = 1;

    pub fn clear_player_steamid(&mut self) {
        self.player_steamid = ::std::option::Option::None;
    }

    pub fn has_player_steamid(&self) -> bool {
        self.player_steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_steamid(&mut self, v: u64) {
        self.player_steamid = ::std::option::Option::Some(v);
    }

    pub fn get_player_steamid(&self) -> u64 {
        self.player_steamid.unwrap_or(0)
    }

    fn get_player_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.player_steamid
    }

    fn mut_player_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.player_steamid
    }

    // optional uint32 item_def_index = 2;

    pub fn clear_item_def_index(&mut self) {
        self.item_def_index = ::std::option::Option::None;
    }

    pub fn has_item_def_index(&self) -> bool {
        self.item_def_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_def_index(&mut self, v: u32) {
        self.item_def_index = ::std::option::Option::Some(v);
    }

    pub fn get_item_def_index(&self) -> u32 {
        self.item_def_index.unwrap_or(0)
    }

    fn get_item_def_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_def_index
    }

    fn mut_item_def_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_def_index
    }

    // optional string item_name_custom = 3;

    pub fn clear_item_name_custom(&mut self) {
        self.item_name_custom.clear();
    }

    pub fn has_item_name_custom(&self) -> bool {
        self.item_name_custom.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_name_custom(&mut self, v: ::std::string::String) {
        self.item_name_custom = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_item_name_custom(&mut self) -> &mut ::std::string::String {
        if self.item_name_custom.is_none() {
            self.item_name_custom.set_default();
        }
        self.item_name_custom.as_mut().unwrap()
    }

    // Take field
    pub fn take_item_name_custom(&mut self) -> ::std::string::String {
        self.item_name_custom.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_item_name_custom(&self) -> &str {
        match self.item_name_custom.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_item_name_custom_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.item_name_custom
    }

    fn mut_item_name_custom_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.item_name_custom
    }
}

impl ::protobuf::Message for CMsgGCNameItemNotification {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.player_steamid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.item_def_index = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.item_name_custom)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.player_steamid {
            my_size += 9;
        }
        if let Some(v) = self.item_def_index {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.item_name_custom.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_steamid {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.item_def_index {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.item_name_custom.as_ref() {
            os.write_string(3, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCNameItemNotification {
    fn new() -> CMsgGCNameItemNotification {
        CMsgGCNameItemNotification::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCNameItemNotification>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "player_steamid",
                    CMsgGCNameItemNotification::get_player_steamid_for_reflect,
                    CMsgGCNameItemNotification::mut_player_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_def_index",
                    CMsgGCNameItemNotification::get_item_def_index_for_reflect,
                    CMsgGCNameItemNotification::mut_item_def_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "item_name_custom",
                    CMsgGCNameItemNotification::get_item_name_custom_for_reflect,
                    CMsgGCNameItemNotification::mut_item_name_custom_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCNameItemNotification>(
                    "CMsgGCNameItemNotification",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCNameItemNotification {
    fn clear(&mut self) {
        self.clear_player_steamid();
        self.clear_item_def_index();
        self.clear_item_name_custom();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCNameItemNotification {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCNameItemNotification {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCClientDisplayNotification {
    // message fields
    notification_title_localization_key: ::protobuf::SingularField<::std::string::String>,
    notification_body_localization_key: ::protobuf::SingularField<::std::string::String>,
    body_substring_keys: ::protobuf::RepeatedField<::std::string::String>,
    body_substring_values: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCClientDisplayNotification {}

impl CMsgGCClientDisplayNotification {
    pub fn new() -> CMsgGCClientDisplayNotification {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCClientDisplayNotification {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCClientDisplayNotification> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCClientDisplayNotification,
        };
        unsafe {
            instance.get(CMsgGCClientDisplayNotification::new)
        }
    }

    // optional string notification_title_localization_key = 1;

    pub fn clear_notification_title_localization_key(&mut self) {
        self.notification_title_localization_key.clear();
    }

    pub fn has_notification_title_localization_key(&self) -> bool {
        self.notification_title_localization_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_notification_title_localization_key(&mut self, v: ::std::string::String) {
        self.notification_title_localization_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_notification_title_localization_key(&mut self) -> &mut ::std::string::String {
        if self.notification_title_localization_key.is_none() {
            self.notification_title_localization_key.set_default();
        }
        self.notification_title_localization_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_notification_title_localization_key(&mut self) -> ::std::string::String {
        self.notification_title_localization_key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_notification_title_localization_key(&self) -> &str {
        match self.notification_title_localization_key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_notification_title_localization_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.notification_title_localization_key
    }

    fn mut_notification_title_localization_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.notification_title_localization_key
    }

    // optional string notification_body_localization_key = 2;

    pub fn clear_notification_body_localization_key(&mut self) {
        self.notification_body_localization_key.clear();
    }

    pub fn has_notification_body_localization_key(&self) -> bool {
        self.notification_body_localization_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_notification_body_localization_key(&mut self, v: ::std::string::String) {
        self.notification_body_localization_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_notification_body_localization_key(&mut self) -> &mut ::std::string::String {
        if self.notification_body_localization_key.is_none() {
            self.notification_body_localization_key.set_default();
        }
        self.notification_body_localization_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_notification_body_localization_key(&mut self) -> ::std::string::String {
        self.notification_body_localization_key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_notification_body_localization_key(&self) -> &str {
        match self.notification_body_localization_key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_notification_body_localization_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.notification_body_localization_key
    }

    fn mut_notification_body_localization_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.notification_body_localization_key
    }

    // repeated string body_substring_keys = 3;

    pub fn clear_body_substring_keys(&mut self) {
        self.body_substring_keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_body_substring_keys(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.body_substring_keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_body_substring_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.body_substring_keys
    }

    // Take field
    pub fn take_body_substring_keys(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.body_substring_keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_body_substring_keys(&self) -> &[::std::string::String] {
        &self.body_substring_keys
    }

    fn get_body_substring_keys_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.body_substring_keys
    }

    fn mut_body_substring_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.body_substring_keys
    }

    // repeated string body_substring_values = 4;

    pub fn clear_body_substring_values(&mut self) {
        self.body_substring_values.clear();
    }

    // Param is passed by value, moved
    pub fn set_body_substring_values(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.body_substring_values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_body_substring_values(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.body_substring_values
    }

    // Take field
    pub fn take_body_substring_values(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.body_substring_values, ::protobuf::RepeatedField::new())
    }

    pub fn get_body_substring_values(&self) -> &[::std::string::String] {
        &self.body_substring_values
    }

    fn get_body_substring_values_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.body_substring_values
    }

    fn mut_body_substring_values_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.body_substring_values
    }
}

impl ::protobuf::Message for CMsgGCClientDisplayNotification {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.notification_title_localization_key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.notification_body_localization_key)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.body_substring_keys)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.body_substring_values)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.notification_title_localization_key.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.notification_body_localization_key.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.body_substring_keys {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.body_substring_values {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.notification_title_localization_key.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.notification_body_localization_key.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.body_substring_keys {
            os.write_string(3, &v)?;
        };
        for v in &self.body_substring_values {
            os.write_string(4, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCClientDisplayNotification {
    fn new() -> CMsgGCClientDisplayNotification {
        CMsgGCClientDisplayNotification::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCClientDisplayNotification>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "notification_title_localization_key",
                    CMsgGCClientDisplayNotification::get_notification_title_localization_key_for_reflect,
                    CMsgGCClientDisplayNotification::mut_notification_title_localization_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "notification_body_localization_key",
                    CMsgGCClientDisplayNotification::get_notification_body_localization_key_for_reflect,
                    CMsgGCClientDisplayNotification::mut_notification_body_localization_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "body_substring_keys",
                    CMsgGCClientDisplayNotification::get_body_substring_keys_for_reflect,
                    CMsgGCClientDisplayNotification::mut_body_substring_keys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "body_substring_values",
                    CMsgGCClientDisplayNotification::get_body_substring_values_for_reflect,
                    CMsgGCClientDisplayNotification::mut_body_substring_values_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCClientDisplayNotification>(
                    "CMsgGCClientDisplayNotification",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCClientDisplayNotification {
    fn clear(&mut self) {
        self.clear_notification_title_localization_key();
        self.clear_notification_body_localization_key();
        self.clear_body_substring_keys();
        self.clear_body_substring_values();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCClientDisplayNotification {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCClientDisplayNotification {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCShowItemsPickedUp {
    // message fields
    player_steamid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCShowItemsPickedUp {}

impl CMsgGCShowItemsPickedUp {
    pub fn new() -> CMsgGCShowItemsPickedUp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCShowItemsPickedUp {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCShowItemsPickedUp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCShowItemsPickedUp,
        };
        unsafe {
            instance.get(CMsgGCShowItemsPickedUp::new)
        }
    }

    // optional fixed64 player_steamid = 1;

    pub fn clear_player_steamid(&mut self) {
        self.player_steamid = ::std::option::Option::None;
    }

    pub fn has_player_steamid(&self) -> bool {
        self.player_steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_steamid(&mut self, v: u64) {
        self.player_steamid = ::std::option::Option::Some(v);
    }

    pub fn get_player_steamid(&self) -> u64 {
        self.player_steamid.unwrap_or(0)
    }

    fn get_player_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.player_steamid
    }

    fn mut_player_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.player_steamid
    }
}

impl ::protobuf::Message for CMsgGCShowItemsPickedUp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.player_steamid = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.player_steamid {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_steamid {
            os.write_fixed64(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCShowItemsPickedUp {
    fn new() -> CMsgGCShowItemsPickedUp {
        CMsgGCShowItemsPickedUp::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCShowItemsPickedUp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "player_steamid",
                    CMsgGCShowItemsPickedUp::get_player_steamid_for_reflect,
                    CMsgGCShowItemsPickedUp::mut_player_steamid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCShowItemsPickedUp>(
                    "CMsgGCShowItemsPickedUp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCShowItemsPickedUp {
    fn clear(&mut self) {
        self.clear_player_steamid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCShowItemsPickedUp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCShowItemsPickedUp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCIncrementKillCountResponse {
    // message fields
    killer_account_id: ::std::option::Option<u32>,
    num_kills: ::std::option::Option<u32>,
    item_def: ::std::option::Option<u32>,
    level_type: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCIncrementKillCountResponse {}

impl CMsgGCIncrementKillCountResponse {
    pub fn new() -> CMsgGCIncrementKillCountResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCIncrementKillCountResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCIncrementKillCountResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCIncrementKillCountResponse,
        };
        unsafe {
            instance.get(CMsgGCIncrementKillCountResponse::new)
        }
    }

    // optional uint32 killer_account_id = 1;

    pub fn clear_killer_account_id(&mut self) {
        self.killer_account_id = ::std::option::Option::None;
    }

    pub fn has_killer_account_id(&self) -> bool {
        self.killer_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_killer_account_id(&mut self, v: u32) {
        self.killer_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_killer_account_id(&self) -> u32 {
        self.killer_account_id.unwrap_or(0)
    }

    fn get_killer_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.killer_account_id
    }

    fn mut_killer_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.killer_account_id
    }

    // optional uint32 num_kills = 2;

    pub fn clear_num_kills(&mut self) {
        self.num_kills = ::std::option::Option::None;
    }

    pub fn has_num_kills(&self) -> bool {
        self.num_kills.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_kills(&mut self, v: u32) {
        self.num_kills = ::std::option::Option::Some(v);
    }

    pub fn get_num_kills(&self) -> u32 {
        self.num_kills.unwrap_or(0)
    }

    fn get_num_kills_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.num_kills
    }

    fn mut_num_kills_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.num_kills
    }

    // optional uint32 item_def = 3;

    pub fn clear_item_def(&mut self) {
        self.item_def = ::std::option::Option::None;
    }

    pub fn has_item_def(&self) -> bool {
        self.item_def.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_def(&mut self, v: u32) {
        self.item_def = ::std::option::Option::Some(v);
    }

    pub fn get_item_def(&self) -> u32 {
        self.item_def.unwrap_or(0)
    }

    fn get_item_def_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_def
    }

    fn mut_item_def_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_def
    }

    // optional uint32 level_type = 4;

    pub fn clear_level_type(&mut self) {
        self.level_type = ::std::option::Option::None;
    }

    pub fn has_level_type(&self) -> bool {
        self.level_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_level_type(&mut self, v: u32) {
        self.level_type = ::std::option::Option::Some(v);
    }

    pub fn get_level_type(&self) -> u32 {
        self.level_type.unwrap_or(0)
    }

    fn get_level_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.level_type
    }

    fn mut_level_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.level_type
    }
}

impl ::protobuf::Message for CMsgGCIncrementKillCountResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.killer_account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.num_kills = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.item_def = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.level_type = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.killer_account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_kills {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.item_def {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.level_type {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.killer_account_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.num_kills {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.item_def {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.level_type {
            os.write_uint32(4, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCIncrementKillCountResponse {
    fn new() -> CMsgGCIncrementKillCountResponse {
        CMsgGCIncrementKillCountResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCIncrementKillCountResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "killer_account_id",
                    CMsgGCIncrementKillCountResponse::get_killer_account_id_for_reflect,
                    CMsgGCIncrementKillCountResponse::mut_killer_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_kills",
                    CMsgGCIncrementKillCountResponse::get_num_kills_for_reflect,
                    CMsgGCIncrementKillCountResponse::mut_num_kills_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_def",
                    CMsgGCIncrementKillCountResponse::get_item_def_for_reflect,
                    CMsgGCIncrementKillCountResponse::mut_item_def_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "level_type",
                    CMsgGCIncrementKillCountResponse::get_level_type_for_reflect,
                    CMsgGCIncrementKillCountResponse::mut_level_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCIncrementKillCountResponse>(
                    "CMsgGCIncrementKillCountResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCIncrementKillCountResponse {
    fn clear(&mut self) {
        self.clear_killer_account_id();
        self.clear_num_kills();
        self.clear_item_def();
        self.clear_level_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCIncrementKillCountResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCIncrementKillCountResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSOEconItemDropRateBonus {
    // message fields
    account_id: ::std::option::Option<u32>,
    expiration_date: ::std::option::Option<u32>,
    bonus: ::std::option::Option<f32>,
    bonus_count: ::std::option::Option<u32>,
    item_id: ::std::option::Option<u64>,
    def_index: ::std::option::Option<u32>,
    seconds_left: ::std::option::Option<u32>,
    booster_type: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSOEconItemDropRateBonus {}

impl CSOEconItemDropRateBonus {
    pub fn new() -> CSOEconItemDropRateBonus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSOEconItemDropRateBonus {
        static mut instance: ::protobuf::lazy::Lazy<CSOEconItemDropRateBonus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSOEconItemDropRateBonus,
        };
        unsafe {
            instance.get(CSOEconItemDropRateBonus::new)
        }
    }

    // optional uint32 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u32 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id
    }

    // optional fixed32 expiration_date = 2;

    pub fn clear_expiration_date(&mut self) {
        self.expiration_date = ::std::option::Option::None;
    }

    pub fn has_expiration_date(&self) -> bool {
        self.expiration_date.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expiration_date(&mut self, v: u32) {
        self.expiration_date = ::std::option::Option::Some(v);
    }

    pub fn get_expiration_date(&self) -> u32 {
        self.expiration_date.unwrap_or(0)
    }

    fn get_expiration_date_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.expiration_date
    }

    fn mut_expiration_date_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.expiration_date
    }

    // optional float bonus = 3;

    pub fn clear_bonus(&mut self) {
        self.bonus = ::std::option::Option::None;
    }

    pub fn has_bonus(&self) -> bool {
        self.bonus.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bonus(&mut self, v: f32) {
        self.bonus = ::std::option::Option::Some(v);
    }

    pub fn get_bonus(&self) -> f32 {
        self.bonus.unwrap_or(0.)
    }

    fn get_bonus_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.bonus
    }

    fn mut_bonus_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.bonus
    }

    // optional uint32 bonus_count = 4;

    pub fn clear_bonus_count(&mut self) {
        self.bonus_count = ::std::option::Option::None;
    }

    pub fn has_bonus_count(&self) -> bool {
        self.bonus_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bonus_count(&mut self, v: u32) {
        self.bonus_count = ::std::option::Option::Some(v);
    }

    pub fn get_bonus_count(&self) -> u32 {
        self.bonus_count.unwrap_or(0)
    }

    fn get_bonus_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.bonus_count
    }

    fn mut_bonus_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.bonus_count
    }

    // optional uint64 item_id = 5;

    pub fn clear_item_id(&mut self) {
        self.item_id = ::std::option::Option::None;
    }

    pub fn has_item_id(&self) -> bool {
        self.item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_id(&mut self, v: u64) {
        self.item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_id(&self) -> u64 {
        self.item_id.unwrap_or(0)
    }

    fn get_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.item_id
    }

    fn mut_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.item_id
    }

    // optional uint32 def_index = 6;

    pub fn clear_def_index(&mut self) {
        self.def_index = ::std::option::Option::None;
    }

    pub fn has_def_index(&self) -> bool {
        self.def_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_def_index(&mut self, v: u32) {
        self.def_index = ::std::option::Option::Some(v);
    }

    pub fn get_def_index(&self) -> u32 {
        self.def_index.unwrap_or(0)
    }

    fn get_def_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.def_index
    }

    fn mut_def_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.def_index
    }

    // optional uint32 seconds_left = 7;

    pub fn clear_seconds_left(&mut self) {
        self.seconds_left = ::std::option::Option::None;
    }

    pub fn has_seconds_left(&self) -> bool {
        self.seconds_left.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seconds_left(&mut self, v: u32) {
        self.seconds_left = ::std::option::Option::Some(v);
    }

    pub fn get_seconds_left(&self) -> u32 {
        self.seconds_left.unwrap_or(0)
    }

    fn get_seconds_left_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seconds_left
    }

    fn mut_seconds_left_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seconds_left
    }

    // optional uint32 booster_type = 8;

    pub fn clear_booster_type(&mut self) {
        self.booster_type = ::std::option::Option::None;
    }

    pub fn has_booster_type(&self) -> bool {
        self.booster_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_booster_type(&mut self, v: u32) {
        self.booster_type = ::std::option::Option::Some(v);
    }

    pub fn get_booster_type(&self) -> u32 {
        self.booster_type.unwrap_or(0)
    }

    fn get_booster_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.booster_type
    }

    fn mut_booster_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.booster_type
    }
}

impl ::protobuf::Message for CSOEconItemDropRateBonus {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.expiration_date = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.bonus = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.bonus_count = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.item_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.def_index = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seconds_left = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.booster_type = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.expiration_date {
            my_size += 5;
        }
        if let Some(v) = self.bonus {
            my_size += 5;
        }
        if let Some(v) = self.bonus_count {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.item_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.def_index {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.seconds_left {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.booster_type {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.expiration_date {
            os.write_fixed32(2, v)?;
        }
        if let Some(v) = self.bonus {
            os.write_float(3, v)?;
        }
        if let Some(v) = self.bonus_count {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.item_id {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.def_index {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.seconds_left {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.booster_type {
            os.write_uint32(8, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSOEconItemDropRateBonus {
    fn new() -> CSOEconItemDropRateBonus {
        CSOEconItemDropRateBonus::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSOEconItemDropRateBonus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CSOEconItemDropRateBonus::get_account_id_for_reflect,
                    CSOEconItemDropRateBonus::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "expiration_date",
                    CSOEconItemDropRateBonus::get_expiration_date_for_reflect,
                    CSOEconItemDropRateBonus::mut_expiration_date_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "bonus",
                    CSOEconItemDropRateBonus::get_bonus_for_reflect,
                    CSOEconItemDropRateBonus::mut_bonus_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bonus_count",
                    CSOEconItemDropRateBonus::get_bonus_count_for_reflect,
                    CSOEconItemDropRateBonus::mut_bonus_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "item_id",
                    CSOEconItemDropRateBonus::get_item_id_for_reflect,
                    CSOEconItemDropRateBonus::mut_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "def_index",
                    CSOEconItemDropRateBonus::get_def_index_for_reflect,
                    CSOEconItemDropRateBonus::mut_def_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seconds_left",
                    CSOEconItemDropRateBonus::get_seconds_left_for_reflect,
                    CSOEconItemDropRateBonus::mut_seconds_left_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "booster_type",
                    CSOEconItemDropRateBonus::get_booster_type_for_reflect,
                    CSOEconItemDropRateBonus::mut_booster_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSOEconItemDropRateBonus>(
                    "CSOEconItemDropRateBonus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSOEconItemDropRateBonus {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_expiration_date();
        self.clear_bonus();
        self.clear_bonus_count();
        self.clear_item_id();
        self.clear_def_index();
        self.clear_seconds_left();
        self.clear_booster_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSOEconItemDropRateBonus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSOEconItemDropRateBonus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSOEconItemLeagueViewPass {
    // message fields
    account_id: ::std::option::Option<u32>,
    league_id: ::std::option::Option<u32>,
    itemindex: ::std::option::Option<u32>,
    grant_reason: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSOEconItemLeagueViewPass {}

impl CSOEconItemLeagueViewPass {
    pub fn new() -> CSOEconItemLeagueViewPass {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSOEconItemLeagueViewPass {
        static mut instance: ::protobuf::lazy::Lazy<CSOEconItemLeagueViewPass> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSOEconItemLeagueViewPass,
        };
        unsafe {
            instance.get(CSOEconItemLeagueViewPass::new)
        }
    }

    // optional uint32 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u32 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id
    }

    // optional uint32 league_id = 2;

    pub fn clear_league_id(&mut self) {
        self.league_id = ::std::option::Option::None;
    }

    pub fn has_league_id(&self) -> bool {
        self.league_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_league_id(&mut self, v: u32) {
        self.league_id = ::std::option::Option::Some(v);
    }

    pub fn get_league_id(&self) -> u32 {
        self.league_id.unwrap_or(0)
    }

    fn get_league_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.league_id
    }

    fn mut_league_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.league_id
    }

    // optional uint32 itemindex = 4;

    pub fn clear_itemindex(&mut self) {
        self.itemindex = ::std::option::Option::None;
    }

    pub fn has_itemindex(&self) -> bool {
        self.itemindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_itemindex(&mut self, v: u32) {
        self.itemindex = ::std::option::Option::Some(v);
    }

    pub fn get_itemindex(&self) -> u32 {
        self.itemindex.unwrap_or(0)
    }

    fn get_itemindex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.itemindex
    }

    fn mut_itemindex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.itemindex
    }

    // optional uint32 grant_reason = 5;

    pub fn clear_grant_reason(&mut self) {
        self.grant_reason = ::std::option::Option::None;
    }

    pub fn has_grant_reason(&self) -> bool {
        self.grant_reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_grant_reason(&mut self, v: u32) {
        self.grant_reason = ::std::option::Option::Some(v);
    }

    pub fn get_grant_reason(&self) -> u32 {
        self.grant_reason.unwrap_or(0)
    }

    fn get_grant_reason_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.grant_reason
    }

    fn mut_grant_reason_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.grant_reason
    }
}

impl ::protobuf::Message for CSOEconItemLeagueViewPass {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.league_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.itemindex = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.grant_reason = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.league_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.itemindex {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.grant_reason {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.league_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.itemindex {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.grant_reason {
            os.write_uint32(5, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSOEconItemLeagueViewPass {
    fn new() -> CSOEconItemLeagueViewPass {
        CSOEconItemLeagueViewPass::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSOEconItemLeagueViewPass>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CSOEconItemLeagueViewPass::get_account_id_for_reflect,
                    CSOEconItemLeagueViewPass::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_id",
                    CSOEconItemLeagueViewPass::get_league_id_for_reflect,
                    CSOEconItemLeagueViewPass::mut_league_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "itemindex",
                    CSOEconItemLeagueViewPass::get_itemindex_for_reflect,
                    CSOEconItemLeagueViewPass::mut_itemindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "grant_reason",
                    CSOEconItemLeagueViewPass::get_grant_reason_for_reflect,
                    CSOEconItemLeagueViewPass::mut_grant_reason_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSOEconItemLeagueViewPass>(
                    "CSOEconItemLeagueViewPass",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSOEconItemLeagueViewPass {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_league_id();
        self.clear_itemindex();
        self.clear_grant_reason();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSOEconItemLeagueViewPass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSOEconItemLeagueViewPass {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSOEconItemEventTicket {
    // message fields
    account_id: ::std::option::Option<u32>,
    event_id: ::std::option::Option<u32>,
    item_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSOEconItemEventTicket {}

impl CSOEconItemEventTicket {
    pub fn new() -> CSOEconItemEventTicket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSOEconItemEventTicket {
        static mut instance: ::protobuf::lazy::Lazy<CSOEconItemEventTicket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSOEconItemEventTicket,
        };
        unsafe {
            instance.get(CSOEconItemEventTicket::new)
        }
    }

    // optional uint32 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u32 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id
    }

    // optional uint32 event_id = 2;

    pub fn clear_event_id(&mut self) {
        self.event_id = ::std::option::Option::None;
    }

    pub fn has_event_id(&self) -> bool {
        self.event_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_id(&mut self, v: u32) {
        self.event_id = ::std::option::Option::Some(v);
    }

    pub fn get_event_id(&self) -> u32 {
        self.event_id.unwrap_or(0)
    }

    fn get_event_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.event_id
    }

    fn mut_event_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.event_id
    }

    // optional uint64 item_id = 3;

    pub fn clear_item_id(&mut self) {
        self.item_id = ::std::option::Option::None;
    }

    pub fn has_item_id(&self) -> bool {
        self.item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_id(&mut self, v: u64) {
        self.item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_id(&self) -> u64 {
        self.item_id.unwrap_or(0)
    }

    fn get_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.item_id
    }

    fn mut_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.item_id
    }
}

impl ::protobuf::Message for CSOEconItemEventTicket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.event_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.item_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.event_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.item_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.event_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.item_id {
            os.write_uint64(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSOEconItemEventTicket {
    fn new() -> CSOEconItemEventTicket {
        CSOEconItemEventTicket::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSOEconItemEventTicket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CSOEconItemEventTicket::get_account_id_for_reflect,
                    CSOEconItemEventTicket::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "event_id",
                    CSOEconItemEventTicket::get_event_id_for_reflect,
                    CSOEconItemEventTicket::mut_event_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "item_id",
                    CSOEconItemEventTicket::get_item_id_for_reflect,
                    CSOEconItemEventTicket::mut_item_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSOEconItemEventTicket>(
                    "CSOEconItemEventTicket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSOEconItemEventTicket {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_event_id();
        self.clear_item_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSOEconItemEventTicket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSOEconItemEventTicket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSOEconItemTournamentPassport {
    // message fields
    account_id: ::std::option::Option<u32>,
    league_id: ::std::option::Option<u32>,
    item_id: ::std::option::Option<u64>,
    original_purchaser_id: ::std::option::Option<u32>,
    passports_bought: ::std::option::Option<u32>,
    version: ::std::option::Option<u32>,
    def_index: ::std::option::Option<u32>,
    reward_flags: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSOEconItemTournamentPassport {}

impl CSOEconItemTournamentPassport {
    pub fn new() -> CSOEconItemTournamentPassport {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSOEconItemTournamentPassport {
        static mut instance: ::protobuf::lazy::Lazy<CSOEconItemTournamentPassport> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSOEconItemTournamentPassport,
        };
        unsafe {
            instance.get(CSOEconItemTournamentPassport::new)
        }
    }

    // optional uint32 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u32 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id
    }

    // optional uint32 league_id = 2;

    pub fn clear_league_id(&mut self) {
        self.league_id = ::std::option::Option::None;
    }

    pub fn has_league_id(&self) -> bool {
        self.league_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_league_id(&mut self, v: u32) {
        self.league_id = ::std::option::Option::Some(v);
    }

    pub fn get_league_id(&self) -> u32 {
        self.league_id.unwrap_or(0)
    }

    fn get_league_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.league_id
    }

    fn mut_league_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.league_id
    }

    // optional uint64 item_id = 3;

    pub fn clear_item_id(&mut self) {
        self.item_id = ::std::option::Option::None;
    }

    pub fn has_item_id(&self) -> bool {
        self.item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_id(&mut self, v: u64) {
        self.item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_id(&self) -> u64 {
        self.item_id.unwrap_or(0)
    }

    fn get_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.item_id
    }

    fn mut_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.item_id
    }

    // optional uint32 original_purchaser_id = 4;

    pub fn clear_original_purchaser_id(&mut self) {
        self.original_purchaser_id = ::std::option::Option::None;
    }

    pub fn has_original_purchaser_id(&self) -> bool {
        self.original_purchaser_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_original_purchaser_id(&mut self, v: u32) {
        self.original_purchaser_id = ::std::option::Option::Some(v);
    }

    pub fn get_original_purchaser_id(&self) -> u32 {
        self.original_purchaser_id.unwrap_or(0)
    }

    fn get_original_purchaser_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.original_purchaser_id
    }

    fn mut_original_purchaser_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.original_purchaser_id
    }

    // optional uint32 passports_bought = 5;

    pub fn clear_passports_bought(&mut self) {
        self.passports_bought = ::std::option::Option::None;
    }

    pub fn has_passports_bought(&self) -> bool {
        self.passports_bought.is_some()
    }

    // Param is passed by value, moved
    pub fn set_passports_bought(&mut self, v: u32) {
        self.passports_bought = ::std::option::Option::Some(v);
    }

    pub fn get_passports_bought(&self) -> u32 {
        self.passports_bought.unwrap_or(0)
    }

    fn get_passports_bought_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.passports_bought
    }

    fn mut_passports_bought_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.passports_bought
    }

    // optional uint32 version = 6;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u32 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.version
    }

    // optional uint32 def_index = 7;

    pub fn clear_def_index(&mut self) {
        self.def_index = ::std::option::Option::None;
    }

    pub fn has_def_index(&self) -> bool {
        self.def_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_def_index(&mut self, v: u32) {
        self.def_index = ::std::option::Option::Some(v);
    }

    pub fn get_def_index(&self) -> u32 {
        self.def_index.unwrap_or(0)
    }

    fn get_def_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.def_index
    }

    fn mut_def_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.def_index
    }

    // optional uint32 reward_flags = 8;

    pub fn clear_reward_flags(&mut self) {
        self.reward_flags = ::std::option::Option::None;
    }

    pub fn has_reward_flags(&self) -> bool {
        self.reward_flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reward_flags(&mut self, v: u32) {
        self.reward_flags = ::std::option::Option::Some(v);
    }

    pub fn get_reward_flags(&self) -> u32 {
        self.reward_flags.unwrap_or(0)
    }

    fn get_reward_flags_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.reward_flags
    }

    fn mut_reward_flags_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.reward_flags
    }
}

impl ::protobuf::Message for CSOEconItemTournamentPassport {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.league_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.item_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.original_purchaser_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.passports_bought = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.def_index = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.reward_flags = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.league_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.item_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.original_purchaser_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.passports_bought {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.def_index {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.reward_flags {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.league_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.item_id {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.original_purchaser_id {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.passports_bought {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.version {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.def_index {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.reward_flags {
            os.write_uint32(8, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSOEconItemTournamentPassport {
    fn new() -> CSOEconItemTournamentPassport {
        CSOEconItemTournamentPassport::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSOEconItemTournamentPassport>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CSOEconItemTournamentPassport::get_account_id_for_reflect,
                    CSOEconItemTournamentPassport::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_id",
                    CSOEconItemTournamentPassport::get_league_id_for_reflect,
                    CSOEconItemTournamentPassport::mut_league_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "item_id",
                    CSOEconItemTournamentPassport::get_item_id_for_reflect,
                    CSOEconItemTournamentPassport::mut_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "original_purchaser_id",
                    CSOEconItemTournamentPassport::get_original_purchaser_id_for_reflect,
                    CSOEconItemTournamentPassport::mut_original_purchaser_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "passports_bought",
                    CSOEconItemTournamentPassport::get_passports_bought_for_reflect,
                    CSOEconItemTournamentPassport::mut_passports_bought_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    CSOEconItemTournamentPassport::get_version_for_reflect,
                    CSOEconItemTournamentPassport::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "def_index",
                    CSOEconItemTournamentPassport::get_def_index_for_reflect,
                    CSOEconItemTournamentPassport::mut_def_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "reward_flags",
                    CSOEconItemTournamentPassport::get_reward_flags_for_reflect,
                    CSOEconItemTournamentPassport::mut_reward_flags_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSOEconItemTournamentPassport>(
                    "CSOEconItemTournamentPassport",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSOEconItemTournamentPassport {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_league_id();
        self.clear_item_id();
        self.clear_original_purchaser_id();
        self.clear_passports_bought();
        self.clear_version();
        self.clear_def_index();
        self.clear_reward_flags();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSOEconItemTournamentPassport {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSOEconItemTournamentPassport {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCStorePurchaseCancel {
    // message fields
    txn_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCStorePurchaseCancel {}

impl CMsgGCStorePurchaseCancel {
    pub fn new() -> CMsgGCStorePurchaseCancel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCStorePurchaseCancel {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCStorePurchaseCancel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCStorePurchaseCancel,
        };
        unsafe {
            instance.get(CMsgGCStorePurchaseCancel::new)
        }
    }

    // optional uint64 txn_id = 1;

    pub fn clear_txn_id(&mut self) {
        self.txn_id = ::std::option::Option::None;
    }

    pub fn has_txn_id(&self) -> bool {
        self.txn_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txn_id(&mut self, v: u64) {
        self.txn_id = ::std::option::Option::Some(v);
    }

    pub fn get_txn_id(&self) -> u64 {
        self.txn_id.unwrap_or(0)
    }

    fn get_txn_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.txn_id
    }

    fn mut_txn_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.txn_id
    }
}

impl ::protobuf::Message for CMsgGCStorePurchaseCancel {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.txn_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.txn_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.txn_id {
            os.write_uint64(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCStorePurchaseCancel {
    fn new() -> CMsgGCStorePurchaseCancel {
        CMsgGCStorePurchaseCancel::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCStorePurchaseCancel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "txn_id",
                    CMsgGCStorePurchaseCancel::get_txn_id_for_reflect,
                    CMsgGCStorePurchaseCancel::mut_txn_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCStorePurchaseCancel>(
                    "CMsgGCStorePurchaseCancel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCStorePurchaseCancel {
    fn clear(&mut self) {
        self.clear_txn_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCStorePurchaseCancel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCStorePurchaseCancel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCStorePurchaseCancelResponse {
    // message fields
    result: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCStorePurchaseCancelResponse {}

impl CMsgGCStorePurchaseCancelResponse {
    pub fn new() -> CMsgGCStorePurchaseCancelResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCStorePurchaseCancelResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCStorePurchaseCancelResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCStorePurchaseCancelResponse,
        };
        unsafe {
            instance.get(CMsgGCStorePurchaseCancelResponse::new)
        }
    }

    // optional uint32 result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: u32) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> u32 {
        self.result.unwrap_or(0)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgGCStorePurchaseCancelResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.result = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCStorePurchaseCancelResponse {
    fn new() -> CMsgGCStorePurchaseCancelResponse {
        CMsgGCStorePurchaseCancelResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCStorePurchaseCancelResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "result",
                    CMsgGCStorePurchaseCancelResponse::get_result_for_reflect,
                    CMsgGCStorePurchaseCancelResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCStorePurchaseCancelResponse>(
                    "CMsgGCStorePurchaseCancelResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCStorePurchaseCancelResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCStorePurchaseCancelResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCStorePurchaseCancelResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCStorePurchaseFinalize {
    // message fields
    txn_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCStorePurchaseFinalize {}

impl CMsgGCStorePurchaseFinalize {
    pub fn new() -> CMsgGCStorePurchaseFinalize {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCStorePurchaseFinalize {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCStorePurchaseFinalize> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCStorePurchaseFinalize,
        };
        unsafe {
            instance.get(CMsgGCStorePurchaseFinalize::new)
        }
    }

    // optional uint64 txn_id = 1;

    pub fn clear_txn_id(&mut self) {
        self.txn_id = ::std::option::Option::None;
    }

    pub fn has_txn_id(&self) -> bool {
        self.txn_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txn_id(&mut self, v: u64) {
        self.txn_id = ::std::option::Option::Some(v);
    }

    pub fn get_txn_id(&self) -> u64 {
        self.txn_id.unwrap_or(0)
    }

    fn get_txn_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.txn_id
    }

    fn mut_txn_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.txn_id
    }
}

impl ::protobuf::Message for CMsgGCStorePurchaseFinalize {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.txn_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.txn_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.txn_id {
            os.write_uint64(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCStorePurchaseFinalize {
    fn new() -> CMsgGCStorePurchaseFinalize {
        CMsgGCStorePurchaseFinalize::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCStorePurchaseFinalize>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "txn_id",
                    CMsgGCStorePurchaseFinalize::get_txn_id_for_reflect,
                    CMsgGCStorePurchaseFinalize::mut_txn_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCStorePurchaseFinalize>(
                    "CMsgGCStorePurchaseFinalize",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCStorePurchaseFinalize {
    fn clear(&mut self) {
        self.clear_txn_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCStorePurchaseFinalize {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCStorePurchaseFinalize {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCStorePurchaseFinalizeResponse {
    // message fields
    result: ::std::option::Option<u32>,
    item_ids: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCStorePurchaseFinalizeResponse {}

impl CMsgGCStorePurchaseFinalizeResponse {
    pub fn new() -> CMsgGCStorePurchaseFinalizeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCStorePurchaseFinalizeResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCStorePurchaseFinalizeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCStorePurchaseFinalizeResponse,
        };
        unsafe {
            instance.get(CMsgGCStorePurchaseFinalizeResponse::new)
        }
    }

    // optional uint32 result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: u32) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> u32 {
        self.result.unwrap_or(0)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.result
    }

    // repeated uint64 item_ids = 2;

    pub fn clear_item_ids(&mut self) {
        self.item_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_item_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.item_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_item_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.item_ids
    }

    // Take field
    pub fn take_item_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.item_ids, ::std::vec::Vec::new())
    }

    pub fn get_item_ids(&self) -> &[u64] {
        &self.item_ids
    }

    fn get_item_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.item_ids
    }

    fn mut_item_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.item_ids
    }
}

impl ::protobuf::Message for CMsgGCStorePurchaseFinalizeResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.item_ids)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.item_ids {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_uint32(1, v)?;
        }
        for v in &self.item_ids {
            os.write_uint64(2, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCStorePurchaseFinalizeResponse {
    fn new() -> CMsgGCStorePurchaseFinalizeResponse {
        CMsgGCStorePurchaseFinalizeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCStorePurchaseFinalizeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "result",
                    CMsgGCStorePurchaseFinalizeResponse::get_result_for_reflect,
                    CMsgGCStorePurchaseFinalizeResponse::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "item_ids",
                    CMsgGCStorePurchaseFinalizeResponse::get_item_ids_for_reflect,
                    CMsgGCStorePurchaseFinalizeResponse::mut_item_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCStorePurchaseFinalizeResponse>(
                    "CMsgGCStorePurchaseFinalizeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCStorePurchaseFinalizeResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_item_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCStorePurchaseFinalizeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCStorePurchaseFinalizeResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCBannedWordListRequest {
    // message fields
    ban_list_group_id: ::std::option::Option<u32>,
    word_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCBannedWordListRequest {}

impl CMsgGCBannedWordListRequest {
    pub fn new() -> CMsgGCBannedWordListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCBannedWordListRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCBannedWordListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCBannedWordListRequest,
        };
        unsafe {
            instance.get(CMsgGCBannedWordListRequest::new)
        }
    }

    // optional uint32 ban_list_group_id = 1;

    pub fn clear_ban_list_group_id(&mut self) {
        self.ban_list_group_id = ::std::option::Option::None;
    }

    pub fn has_ban_list_group_id(&self) -> bool {
        self.ban_list_group_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ban_list_group_id(&mut self, v: u32) {
        self.ban_list_group_id = ::std::option::Option::Some(v);
    }

    pub fn get_ban_list_group_id(&self) -> u32 {
        self.ban_list_group_id.unwrap_or(0)
    }

    fn get_ban_list_group_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ban_list_group_id
    }

    fn mut_ban_list_group_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ban_list_group_id
    }

    // optional uint32 word_id = 2;

    pub fn clear_word_id(&mut self) {
        self.word_id = ::std::option::Option::None;
    }

    pub fn has_word_id(&self) -> bool {
        self.word_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_word_id(&mut self, v: u32) {
        self.word_id = ::std::option::Option::Some(v);
    }

    pub fn get_word_id(&self) -> u32 {
        self.word_id.unwrap_or(0)
    }

    fn get_word_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.word_id
    }

    fn mut_word_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.word_id
    }
}

impl ::protobuf::Message for CMsgGCBannedWordListRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ban_list_group_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.word_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.ban_list_group_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.word_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ban_list_group_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.word_id {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCBannedWordListRequest {
    fn new() -> CMsgGCBannedWordListRequest {
        CMsgGCBannedWordListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCBannedWordListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ban_list_group_id",
                    CMsgGCBannedWordListRequest::get_ban_list_group_id_for_reflect,
                    CMsgGCBannedWordListRequest::mut_ban_list_group_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "word_id",
                    CMsgGCBannedWordListRequest::get_word_id_for_reflect,
                    CMsgGCBannedWordListRequest::mut_word_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCBannedWordListRequest>(
                    "CMsgGCBannedWordListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCBannedWordListRequest {
    fn clear(&mut self) {
        self.clear_ban_list_group_id();
        self.clear_word_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCBannedWordListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCBannedWordListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCBannedWord {
    // message fields
    word_id: ::std::option::Option<u32>,
    word_type: ::std::option::Option<GC_BannedWordType>,
    word: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCBannedWord {}

impl CMsgGCBannedWord {
    pub fn new() -> CMsgGCBannedWord {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCBannedWord {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCBannedWord> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCBannedWord,
        };
        unsafe {
            instance.get(CMsgGCBannedWord::new)
        }
    }

    // optional uint32 word_id = 1;

    pub fn clear_word_id(&mut self) {
        self.word_id = ::std::option::Option::None;
    }

    pub fn has_word_id(&self) -> bool {
        self.word_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_word_id(&mut self, v: u32) {
        self.word_id = ::std::option::Option::Some(v);
    }

    pub fn get_word_id(&self) -> u32 {
        self.word_id.unwrap_or(0)
    }

    fn get_word_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.word_id
    }

    fn mut_word_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.word_id
    }

    // optional .GC_BannedWordType word_type = 2;

    pub fn clear_word_type(&mut self) {
        self.word_type = ::std::option::Option::None;
    }

    pub fn has_word_type(&self) -> bool {
        self.word_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_word_type(&mut self, v: GC_BannedWordType) {
        self.word_type = ::std::option::Option::Some(v);
    }

    pub fn get_word_type(&self) -> GC_BannedWordType {
        self.word_type.unwrap_or(GC_BannedWordType::GC_BANNED_WORD_DISABLE_WORD)
    }

    fn get_word_type_for_reflect(&self) -> &::std::option::Option<GC_BannedWordType> {
        &self.word_type
    }

    fn mut_word_type_for_reflect(&mut self) -> &mut ::std::option::Option<GC_BannedWordType> {
        &mut self.word_type
    }

    // optional string word = 3;

    pub fn clear_word(&mut self) {
        self.word.clear();
    }

    pub fn has_word(&self) -> bool {
        self.word.is_some()
    }

    // Param is passed by value, moved
    pub fn set_word(&mut self, v: ::std::string::String) {
        self.word = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_word(&mut self) -> &mut ::std::string::String {
        if self.word.is_none() {
            self.word.set_default();
        }
        self.word.as_mut().unwrap()
    }

    // Take field
    pub fn take_word(&mut self) -> ::std::string::String {
        self.word.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_word(&self) -> &str {
        match self.word.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_word_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.word
    }

    fn mut_word_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.word
    }
}

impl ::protobuf::Message for CMsgGCBannedWord {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.word_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.word_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.word)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.word_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.word_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.word.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.word_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.word_type {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.word.as_ref() {
            os.write_string(3, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCBannedWord {
    fn new() -> CMsgGCBannedWord {
        CMsgGCBannedWord::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCBannedWord>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "word_id",
                    CMsgGCBannedWord::get_word_id_for_reflect,
                    CMsgGCBannedWord::mut_word_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<GC_BannedWordType>>(
                    "word_type",
                    CMsgGCBannedWord::get_word_type_for_reflect,
                    CMsgGCBannedWord::mut_word_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "word",
                    CMsgGCBannedWord::get_word_for_reflect,
                    CMsgGCBannedWord::mut_word_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCBannedWord>(
                    "CMsgGCBannedWord",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCBannedWord {
    fn clear(&mut self) {
        self.clear_word_id();
        self.clear_word_type();
        self.clear_word();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCBannedWord {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCBannedWord {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCBannedWordListResponse {
    // message fields
    ban_list_group_id: ::std::option::Option<u32>,
    word_list: ::protobuf::RepeatedField<CMsgGCBannedWord>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCBannedWordListResponse {}

impl CMsgGCBannedWordListResponse {
    pub fn new() -> CMsgGCBannedWordListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCBannedWordListResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCBannedWordListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCBannedWordListResponse,
        };
        unsafe {
            instance.get(CMsgGCBannedWordListResponse::new)
        }
    }

    // optional uint32 ban_list_group_id = 1;

    pub fn clear_ban_list_group_id(&mut self) {
        self.ban_list_group_id = ::std::option::Option::None;
    }

    pub fn has_ban_list_group_id(&self) -> bool {
        self.ban_list_group_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ban_list_group_id(&mut self, v: u32) {
        self.ban_list_group_id = ::std::option::Option::Some(v);
    }

    pub fn get_ban_list_group_id(&self) -> u32 {
        self.ban_list_group_id.unwrap_or(0)
    }

    fn get_ban_list_group_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ban_list_group_id
    }

    fn mut_ban_list_group_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ban_list_group_id
    }

    // repeated .CMsgGCBannedWord word_list = 2;

    pub fn clear_word_list(&mut self) {
        self.word_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_word_list(&mut self, v: ::protobuf::RepeatedField<CMsgGCBannedWord>) {
        self.word_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_word_list(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCBannedWord> {
        &mut self.word_list
    }

    // Take field
    pub fn take_word_list(&mut self) -> ::protobuf::RepeatedField<CMsgGCBannedWord> {
        ::std::mem::replace(&mut self.word_list, ::protobuf::RepeatedField::new())
    }

    pub fn get_word_list(&self) -> &[CMsgGCBannedWord] {
        &self.word_list
    }

    fn get_word_list_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgGCBannedWord> {
        &self.word_list
    }

    fn mut_word_list_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCBannedWord> {
        &mut self.word_list
    }
}

impl ::protobuf::Message for CMsgGCBannedWordListResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.word_list {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ban_list_group_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.word_list)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.ban_list_group_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.word_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ban_list_group_id {
            os.write_uint32(1, v)?;
        }
        for v in &self.word_list {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCBannedWordListResponse {
    fn new() -> CMsgGCBannedWordListResponse {
        CMsgGCBannedWordListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCBannedWordListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ban_list_group_id",
                    CMsgGCBannedWordListResponse::get_ban_list_group_id_for_reflect,
                    CMsgGCBannedWordListResponse::mut_ban_list_group_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgGCBannedWord>>(
                    "word_list",
                    CMsgGCBannedWordListResponse::get_word_list_for_reflect,
                    CMsgGCBannedWordListResponse::mut_word_list_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCBannedWordListResponse>(
                    "CMsgGCBannedWordListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCBannedWordListResponse {
    fn clear(&mut self) {
        self.clear_ban_list_group_id();
        self.clear_word_list();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCBannedWordListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCBannedWordListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCBannedWordListBroadcast {
    // message fields
    broadcast: ::protobuf::SingularPtrField<CMsgGCBannedWordListResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCBannedWordListBroadcast {}

impl CMsgGCToGCBannedWordListBroadcast {
    pub fn new() -> CMsgGCToGCBannedWordListBroadcast {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCBannedWordListBroadcast {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCBannedWordListBroadcast> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCBannedWordListBroadcast,
        };
        unsafe {
            instance.get(CMsgGCToGCBannedWordListBroadcast::new)
        }
    }

    // optional .CMsgGCBannedWordListResponse broadcast = 1;

    pub fn clear_broadcast(&mut self) {
        self.broadcast.clear();
    }

    pub fn has_broadcast(&self) -> bool {
        self.broadcast.is_some()
    }

    // Param is passed by value, moved
    pub fn set_broadcast(&mut self, v: CMsgGCBannedWordListResponse) {
        self.broadcast = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_broadcast(&mut self) -> &mut CMsgGCBannedWordListResponse {
        if self.broadcast.is_none() {
            self.broadcast.set_default();
        }
        self.broadcast.as_mut().unwrap()
    }

    // Take field
    pub fn take_broadcast(&mut self) -> CMsgGCBannedWordListResponse {
        self.broadcast.take().unwrap_or_else(|| CMsgGCBannedWordListResponse::new())
    }

    pub fn get_broadcast(&self) -> &CMsgGCBannedWordListResponse {
        self.broadcast.as_ref().unwrap_or_else(|| CMsgGCBannedWordListResponse::default_instance())
    }

    fn get_broadcast_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgGCBannedWordListResponse> {
        &self.broadcast
    }

    fn mut_broadcast_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgGCBannedWordListResponse> {
        &mut self.broadcast
    }
}

impl ::protobuf::Message for CMsgGCToGCBannedWordListBroadcast {
    fn is_initialized(&self) -> bool {
        for v in &self.broadcast {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.broadcast)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.broadcast.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.broadcast.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCToGCBannedWordListBroadcast {
    fn new() -> CMsgGCToGCBannedWordListBroadcast {
        CMsgGCToGCBannedWordListBroadcast::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCBannedWordListBroadcast>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgGCBannedWordListResponse>>(
                    "broadcast",
                    CMsgGCToGCBannedWordListBroadcast::get_broadcast_for_reflect,
                    CMsgGCToGCBannedWordListBroadcast::mut_broadcast_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCBannedWordListBroadcast>(
                    "CMsgGCToGCBannedWordListBroadcast",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCBannedWordListBroadcast {
    fn clear(&mut self) {
        self.clear_broadcast();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCBannedWordListBroadcast {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCBannedWordListBroadcast {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCBannedWordListUpdated {
    // message fields
    group_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCBannedWordListUpdated {}

impl CMsgGCToGCBannedWordListUpdated {
    pub fn new() -> CMsgGCToGCBannedWordListUpdated {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCBannedWordListUpdated {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCBannedWordListUpdated> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCBannedWordListUpdated,
        };
        unsafe {
            instance.get(CMsgGCToGCBannedWordListUpdated::new)
        }
    }

    // optional uint32 group_id = 1;

    pub fn clear_group_id(&mut self) {
        self.group_id = ::std::option::Option::None;
    }

    pub fn has_group_id(&self) -> bool {
        self.group_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group_id(&mut self, v: u32) {
        self.group_id = ::std::option::Option::Some(v);
    }

    pub fn get_group_id(&self) -> u32 {
        self.group_id.unwrap_or(0)
    }

    fn get_group_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.group_id
    }

    fn mut_group_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.group_id
    }
}

impl ::protobuf::Message for CMsgGCToGCBannedWordListUpdated {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.group_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.group_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.group_id {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCToGCBannedWordListUpdated {
    fn new() -> CMsgGCToGCBannedWordListUpdated {
        CMsgGCToGCBannedWordListUpdated::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCBannedWordListUpdated>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "group_id",
                    CMsgGCToGCBannedWordListUpdated::get_group_id_for_reflect,
                    CMsgGCToGCBannedWordListUpdated::mut_group_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCBannedWordListUpdated>(
                    "CMsgGCToGCBannedWordListUpdated",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCBannedWordListUpdated {
    fn clear(&mut self) {
        self.clear_group_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCBannedWordListUpdated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCBannedWordListUpdated {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCDirtySDOCache {
    // message fields
    sdo_type: ::std::option::Option<u32>,
    key_uint64: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCDirtySDOCache {}

impl CMsgGCToGCDirtySDOCache {
    pub fn new() -> CMsgGCToGCDirtySDOCache {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCDirtySDOCache {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCDirtySDOCache> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCDirtySDOCache,
        };
        unsafe {
            instance.get(CMsgGCToGCDirtySDOCache::new)
        }
    }

    // optional uint32 sdo_type = 1;

    pub fn clear_sdo_type(&mut self) {
        self.sdo_type = ::std::option::Option::None;
    }

    pub fn has_sdo_type(&self) -> bool {
        self.sdo_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sdo_type(&mut self, v: u32) {
        self.sdo_type = ::std::option::Option::Some(v);
    }

    pub fn get_sdo_type(&self) -> u32 {
        self.sdo_type.unwrap_or(0)
    }

    fn get_sdo_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sdo_type
    }

    fn mut_sdo_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sdo_type
    }

    // optional uint64 key_uint64 = 2;

    pub fn clear_key_uint64(&mut self) {
        self.key_uint64 = ::std::option::Option::None;
    }

    pub fn has_key_uint64(&self) -> bool {
        self.key_uint64.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_uint64(&mut self, v: u64) {
        self.key_uint64 = ::std::option::Option::Some(v);
    }

    pub fn get_key_uint64(&self) -> u64 {
        self.key_uint64.unwrap_or(0)
    }

    fn get_key_uint64_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.key_uint64
    }

    fn mut_key_uint64_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.key_uint64
    }
}

impl ::protobuf::Message for CMsgGCToGCDirtySDOCache {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.sdo_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.key_uint64 = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.sdo_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.key_uint64 {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sdo_type {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.key_uint64 {
            os.write_uint64(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCToGCDirtySDOCache {
    fn new() -> CMsgGCToGCDirtySDOCache {
        CMsgGCToGCDirtySDOCache::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCDirtySDOCache>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sdo_type",
                    CMsgGCToGCDirtySDOCache::get_sdo_type_for_reflect,
                    CMsgGCToGCDirtySDOCache::mut_sdo_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "key_uint64",
                    CMsgGCToGCDirtySDOCache::get_key_uint64_for_reflect,
                    CMsgGCToGCDirtySDOCache::mut_key_uint64_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCDirtySDOCache>(
                    "CMsgGCToGCDirtySDOCache",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCDirtySDOCache {
    fn clear(&mut self) {
        self.clear_sdo_type();
        self.clear_key_uint64();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCDirtySDOCache {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCDirtySDOCache {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCDirtyMultipleSDOCache {
    // message fields
    sdo_type: ::std::option::Option<u32>,
    key_uint64: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCDirtyMultipleSDOCache {}

impl CMsgGCToGCDirtyMultipleSDOCache {
    pub fn new() -> CMsgGCToGCDirtyMultipleSDOCache {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCDirtyMultipleSDOCache {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCDirtyMultipleSDOCache> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCDirtyMultipleSDOCache,
        };
        unsafe {
            instance.get(CMsgGCToGCDirtyMultipleSDOCache::new)
        }
    }

    // optional uint32 sdo_type = 1;

    pub fn clear_sdo_type(&mut self) {
        self.sdo_type = ::std::option::Option::None;
    }

    pub fn has_sdo_type(&self) -> bool {
        self.sdo_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sdo_type(&mut self, v: u32) {
        self.sdo_type = ::std::option::Option::Some(v);
    }

    pub fn get_sdo_type(&self) -> u32 {
        self.sdo_type.unwrap_or(0)
    }

    fn get_sdo_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sdo_type
    }

    fn mut_sdo_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sdo_type
    }

    // repeated uint64 key_uint64 = 2;

    pub fn clear_key_uint64(&mut self) {
        self.key_uint64.clear();
    }

    // Param is passed by value, moved
    pub fn set_key_uint64(&mut self, v: ::std::vec::Vec<u64>) {
        self.key_uint64 = v;
    }

    // Mutable pointer to the field.
    pub fn mut_key_uint64(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.key_uint64
    }

    // Take field
    pub fn take_key_uint64(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.key_uint64, ::std::vec::Vec::new())
    }

    pub fn get_key_uint64(&self) -> &[u64] {
        &self.key_uint64
    }

    fn get_key_uint64_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.key_uint64
    }

    fn mut_key_uint64_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.key_uint64
    }
}

impl ::protobuf::Message for CMsgGCToGCDirtyMultipleSDOCache {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.sdo_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.key_uint64)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.sdo_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.key_uint64 {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sdo_type {
            os.write_uint32(1, v)?;
        }
        for v in &self.key_uint64 {
            os.write_uint64(2, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCToGCDirtyMultipleSDOCache {
    fn new() -> CMsgGCToGCDirtyMultipleSDOCache {
        CMsgGCToGCDirtyMultipleSDOCache::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCDirtyMultipleSDOCache>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sdo_type",
                    CMsgGCToGCDirtyMultipleSDOCache::get_sdo_type_for_reflect,
                    CMsgGCToGCDirtyMultipleSDOCache::mut_sdo_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "key_uint64",
                    CMsgGCToGCDirtyMultipleSDOCache::get_key_uint64_for_reflect,
                    CMsgGCToGCDirtyMultipleSDOCache::mut_key_uint64_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCDirtyMultipleSDOCache>(
                    "CMsgGCToGCDirtyMultipleSDOCache",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCDirtyMultipleSDOCache {
    fn clear(&mut self) {
        self.clear_sdo_type();
        self.clear_key_uint64();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCDirtyMultipleSDOCache {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCDirtyMultipleSDOCache {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCApplyLocalizationDiff {
    // message fields
    language: ::std::option::Option<u32>,
    packed_diff: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCApplyLocalizationDiff {}

impl CMsgGCToGCApplyLocalizationDiff {
    pub fn new() -> CMsgGCToGCApplyLocalizationDiff {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCApplyLocalizationDiff {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCApplyLocalizationDiff> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCApplyLocalizationDiff,
        };
        unsafe {
            instance.get(CMsgGCToGCApplyLocalizationDiff::new)
        }
    }

    // optional uint32 language = 1;

    pub fn clear_language(&mut self) {
        self.language = ::std::option::Option::None;
    }

    pub fn has_language(&self) -> bool {
        self.language.is_some()
    }

    // Param is passed by value, moved
    pub fn set_language(&mut self, v: u32) {
        self.language = ::std::option::Option::Some(v);
    }

    pub fn get_language(&self) -> u32 {
        self.language.unwrap_or(0)
    }

    fn get_language_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.language
    }

    fn mut_language_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.language
    }

    // optional string packed_diff = 2;

    pub fn clear_packed_diff(&mut self) {
        self.packed_diff.clear();
    }

    pub fn has_packed_diff(&self) -> bool {
        self.packed_diff.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packed_diff(&mut self, v: ::std::string::String) {
        self.packed_diff = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_packed_diff(&mut self) -> &mut ::std::string::String {
        if self.packed_diff.is_none() {
            self.packed_diff.set_default();
        }
        self.packed_diff.as_mut().unwrap()
    }

    // Take field
    pub fn take_packed_diff(&mut self) -> ::std::string::String {
        self.packed_diff.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_packed_diff(&self) -> &str {
        match self.packed_diff.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_packed_diff_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.packed_diff
    }

    fn mut_packed_diff_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.packed_diff
    }
}

impl ::protobuf::Message for CMsgGCToGCApplyLocalizationDiff {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.language = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.packed_diff)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.language {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.packed_diff.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.language {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.packed_diff.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCToGCApplyLocalizationDiff {
    fn new() -> CMsgGCToGCApplyLocalizationDiff {
        CMsgGCToGCApplyLocalizationDiff::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCApplyLocalizationDiff>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "language",
                    CMsgGCToGCApplyLocalizationDiff::get_language_for_reflect,
                    CMsgGCToGCApplyLocalizationDiff::mut_language_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "packed_diff",
                    CMsgGCToGCApplyLocalizationDiff::get_packed_diff_for_reflect,
                    CMsgGCToGCApplyLocalizationDiff::mut_packed_diff_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCApplyLocalizationDiff>(
                    "CMsgGCToGCApplyLocalizationDiff",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCApplyLocalizationDiff {
    fn clear(&mut self) {
        self.clear_language();
        self.clear_packed_diff();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCApplyLocalizationDiff {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCApplyLocalizationDiff {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCApplyLocalizationDiffResponse {
    // message fields
    success: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCApplyLocalizationDiffResponse {}

impl CMsgGCToGCApplyLocalizationDiffResponse {
    pub fn new() -> CMsgGCToGCApplyLocalizationDiffResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCApplyLocalizationDiffResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCApplyLocalizationDiffResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCApplyLocalizationDiffResponse,
        };
        unsafe {
            instance.get(CMsgGCToGCApplyLocalizationDiffResponse::new)
        }
    }

    // optional bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }

    fn get_success_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.success
    }

    fn mut_success_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.success
    }
}

impl ::protobuf::Message for CMsgGCToGCApplyLocalizationDiffResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.success = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.success {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            os.write_bool(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCToGCApplyLocalizationDiffResponse {
    fn new() -> CMsgGCToGCApplyLocalizationDiffResponse {
        CMsgGCToGCApplyLocalizationDiffResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCApplyLocalizationDiffResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "success",
                    CMsgGCToGCApplyLocalizationDiffResponse::get_success_for_reflect,
                    CMsgGCToGCApplyLocalizationDiffResponse::mut_success_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCApplyLocalizationDiffResponse>(
                    "CMsgGCToGCApplyLocalizationDiffResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCApplyLocalizationDiffResponse {
    fn clear(&mut self) {
        self.clear_success();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCApplyLocalizationDiffResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCApplyLocalizationDiffResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCCollectItem {
    // message fields
    collection_item_id: ::std::option::Option<u64>,
    subject_item_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCCollectItem {}

impl CMsgGCCollectItem {
    pub fn new() -> CMsgGCCollectItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCCollectItem {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCCollectItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCCollectItem,
        };
        unsafe {
            instance.get(CMsgGCCollectItem::new)
        }
    }

    // optional uint64 collection_item_id = 1;

    pub fn clear_collection_item_id(&mut self) {
        self.collection_item_id = ::std::option::Option::None;
    }

    pub fn has_collection_item_id(&self) -> bool {
        self.collection_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collection_item_id(&mut self, v: u64) {
        self.collection_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_collection_item_id(&self) -> u64 {
        self.collection_item_id.unwrap_or(0)
    }

    fn get_collection_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.collection_item_id
    }

    fn mut_collection_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.collection_item_id
    }

    // optional uint64 subject_item_id = 2;

    pub fn clear_subject_item_id(&mut self) {
        self.subject_item_id = ::std::option::Option::None;
    }

    pub fn has_subject_item_id(&self) -> bool {
        self.subject_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subject_item_id(&mut self, v: u64) {
        self.subject_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_subject_item_id(&self) -> u64 {
        self.subject_item_id.unwrap_or(0)
    }

    fn get_subject_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.subject_item_id
    }

    fn mut_subject_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.subject_item_id
    }
}

impl ::protobuf::Message for CMsgGCCollectItem {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.collection_item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.subject_item_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.collection_item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.subject_item_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.collection_item_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.subject_item_id {
            os.write_uint64(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCCollectItem {
    fn new() -> CMsgGCCollectItem {
        CMsgGCCollectItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCCollectItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "collection_item_id",
                    CMsgGCCollectItem::get_collection_item_id_for_reflect,
                    CMsgGCCollectItem::mut_collection_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "subject_item_id",
                    CMsgGCCollectItem::get_subject_item_id_for_reflect,
                    CMsgGCCollectItem::mut_subject_item_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCCollectItem>(
                    "CMsgGCCollectItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCCollectItem {
    fn clear(&mut self) {
        self.clear_collection_item_id();
        self.clear_subject_item_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCCollectItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCCollectItem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSDONoMemcached {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSDONoMemcached {}

impl CMsgSDONoMemcached {
    pub fn new() -> CMsgSDONoMemcached {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSDONoMemcached {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSDONoMemcached> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSDONoMemcached,
        };
        unsafe {
            instance.get(CMsgSDONoMemcached::new)
        }
    }
}

impl ::protobuf::Message for CMsgSDONoMemcached {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgSDONoMemcached {
    fn new() -> CMsgSDONoMemcached {
        CMsgSDONoMemcached::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSDONoMemcached>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSDONoMemcached>(
                    "CMsgSDONoMemcached",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSDONoMemcached {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSDONoMemcached {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSDONoMemcached {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCUpdateSQLKeyValue {
    // message fields
    key_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCUpdateSQLKeyValue {}

impl CMsgGCToGCUpdateSQLKeyValue {
    pub fn new() -> CMsgGCToGCUpdateSQLKeyValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCUpdateSQLKeyValue {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCUpdateSQLKeyValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCUpdateSQLKeyValue,
        };
        unsafe {
            instance.get(CMsgGCToGCUpdateSQLKeyValue::new)
        }
    }

    // optional string key_name = 1;

    pub fn clear_key_name(&mut self) {
        self.key_name.clear();
    }

    pub fn has_key_name(&self) -> bool {
        self.key_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_name(&mut self, v: ::std::string::String) {
        self.key_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key_name(&mut self) -> &mut ::std::string::String {
        if self.key_name.is_none() {
            self.key_name.set_default();
        }
        self.key_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_key_name(&mut self) -> ::std::string::String {
        self.key_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key_name(&self) -> &str {
        match self.key_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_key_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.key_name
    }

    fn mut_key_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.key_name
    }
}

impl ::protobuf::Message for CMsgGCToGCUpdateSQLKeyValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.key_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.key_name.as_ref() {
            os.write_string(1, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCToGCUpdateSQLKeyValue {
    fn new() -> CMsgGCToGCUpdateSQLKeyValue {
        CMsgGCToGCUpdateSQLKeyValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCUpdateSQLKeyValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key_name",
                    CMsgGCToGCUpdateSQLKeyValue::get_key_name_for_reflect,
                    CMsgGCToGCUpdateSQLKeyValue::mut_key_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCUpdateSQLKeyValue>(
                    "CMsgGCToGCUpdateSQLKeyValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCUpdateSQLKeyValue {
    fn clear(&mut self) {
        self.clear_key_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCUpdateSQLKeyValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCUpdateSQLKeyValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCServerVersionUpdated {
    // message fields
    server_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCServerVersionUpdated {}

impl CMsgGCServerVersionUpdated {
    pub fn new() -> CMsgGCServerVersionUpdated {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCServerVersionUpdated {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCServerVersionUpdated> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCServerVersionUpdated,
        };
        unsafe {
            instance.get(CMsgGCServerVersionUpdated::new)
        }
    }

    // optional uint32 server_version = 1;

    pub fn clear_server_version(&mut self) {
        self.server_version = ::std::option::Option::None;
    }

    pub fn has_server_version(&self) -> bool {
        self.server_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_version(&mut self, v: u32) {
        self.server_version = ::std::option::Option::Some(v);
    }

    pub fn get_server_version(&self) -> u32 {
        self.server_version.unwrap_or(0)
    }

    fn get_server_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_version
    }

    fn mut_server_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_version
    }
}

impl ::protobuf::Message for CMsgGCServerVersionUpdated {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.server_version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.server_version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.server_version {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCServerVersionUpdated {
    fn new() -> CMsgGCServerVersionUpdated {
        CMsgGCServerVersionUpdated::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCServerVersionUpdated>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_version",
                    CMsgGCServerVersionUpdated::get_server_version_for_reflect,
                    CMsgGCServerVersionUpdated::mut_server_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCServerVersionUpdated>(
                    "CMsgGCServerVersionUpdated",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCServerVersionUpdated {
    fn clear(&mut self) {
        self.clear_server_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCServerVersionUpdated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCServerVersionUpdated {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCClientVersionUpdated {
    // message fields
    client_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCClientVersionUpdated {}

impl CMsgGCClientVersionUpdated {
    pub fn new() -> CMsgGCClientVersionUpdated {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCClientVersionUpdated {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCClientVersionUpdated> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCClientVersionUpdated,
        };
        unsafe {
            instance.get(CMsgGCClientVersionUpdated::new)
        }
    }

    // optional uint32 client_version = 1;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }
}

impl ::protobuf::Message for CMsgGCClientVersionUpdated {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_version {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCClientVersionUpdated {
    fn new() -> CMsgGCClientVersionUpdated {
        CMsgGCClientVersionUpdated::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCClientVersionUpdated>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgGCClientVersionUpdated::get_client_version_for_reflect,
                    CMsgGCClientVersionUpdated::mut_client_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCClientVersionUpdated>(
                    "CMsgGCClientVersionUpdated",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCClientVersionUpdated {
    fn clear(&mut self) {
        self.clear_client_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCClientVersionUpdated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCClientVersionUpdated {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCWebAPIAccountChanged {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCWebAPIAccountChanged {}

impl CMsgGCToGCWebAPIAccountChanged {
    pub fn new() -> CMsgGCToGCWebAPIAccountChanged {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCWebAPIAccountChanged {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCWebAPIAccountChanged> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCWebAPIAccountChanged,
        };
        unsafe {
            instance.get(CMsgGCToGCWebAPIAccountChanged::new)
        }
    }
}

impl ::protobuf::Message for CMsgGCToGCWebAPIAccountChanged {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCToGCWebAPIAccountChanged {
    fn new() -> CMsgGCToGCWebAPIAccountChanged {
        CMsgGCToGCWebAPIAccountChanged::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCWebAPIAccountChanged>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCWebAPIAccountChanged>(
                    "CMsgGCToGCWebAPIAccountChanged",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCWebAPIAccountChanged {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCWebAPIAccountChanged {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCWebAPIAccountChanged {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRecipeComponent {
    // message fields
    subject_item_id: ::std::option::Option<u64>,
    attribute_index: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRecipeComponent {}

impl CMsgRecipeComponent {
    pub fn new() -> CMsgRecipeComponent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRecipeComponent {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRecipeComponent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRecipeComponent,
        };
        unsafe {
            instance.get(CMsgRecipeComponent::new)
        }
    }

    // optional uint64 subject_item_id = 1;

    pub fn clear_subject_item_id(&mut self) {
        self.subject_item_id = ::std::option::Option::None;
    }

    pub fn has_subject_item_id(&self) -> bool {
        self.subject_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subject_item_id(&mut self, v: u64) {
        self.subject_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_subject_item_id(&self) -> u64 {
        self.subject_item_id.unwrap_or(0)
    }

    fn get_subject_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.subject_item_id
    }

    fn mut_subject_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.subject_item_id
    }

    // optional uint64 attribute_index = 2;

    pub fn clear_attribute_index(&mut self) {
        self.attribute_index = ::std::option::Option::None;
    }

    pub fn has_attribute_index(&self) -> bool {
        self.attribute_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attribute_index(&mut self, v: u64) {
        self.attribute_index = ::std::option::Option::Some(v);
    }

    pub fn get_attribute_index(&self) -> u64 {
        self.attribute_index.unwrap_or(0)
    }

    fn get_attribute_index_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.attribute_index
    }

    fn mut_attribute_index_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.attribute_index
    }
}

impl ::protobuf::Message for CMsgRecipeComponent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.subject_item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.attribute_index = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.subject_item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.attribute_index {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.subject_item_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.attribute_index {
            os.write_uint64(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgRecipeComponent {
    fn new() -> CMsgRecipeComponent {
        CMsgRecipeComponent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRecipeComponent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "subject_item_id",
                    CMsgRecipeComponent::get_subject_item_id_for_reflect,
                    CMsgRecipeComponent::mut_subject_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "attribute_index",
                    CMsgRecipeComponent::get_attribute_index_for_reflect,
                    CMsgRecipeComponent::mut_attribute_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRecipeComponent>(
                    "CMsgRecipeComponent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRecipeComponent {
    fn clear(&mut self) {
        self.clear_subject_item_id();
        self.clear_attribute_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRecipeComponent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRecipeComponent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgFulfillDynamicRecipeComponent {
    // message fields
    tool_item_id: ::std::option::Option<u64>,
    consumption_components: ::protobuf::RepeatedField<CMsgRecipeComponent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgFulfillDynamicRecipeComponent {}

impl CMsgFulfillDynamicRecipeComponent {
    pub fn new() -> CMsgFulfillDynamicRecipeComponent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgFulfillDynamicRecipeComponent {
        static mut instance: ::protobuf::lazy::Lazy<CMsgFulfillDynamicRecipeComponent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgFulfillDynamicRecipeComponent,
        };
        unsafe {
            instance.get(CMsgFulfillDynamicRecipeComponent::new)
        }
    }

    // optional uint64 tool_item_id = 1;

    pub fn clear_tool_item_id(&mut self) {
        self.tool_item_id = ::std::option::Option::None;
    }

    pub fn has_tool_item_id(&self) -> bool {
        self.tool_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tool_item_id(&mut self, v: u64) {
        self.tool_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_tool_item_id(&self) -> u64 {
        self.tool_item_id.unwrap_or(0)
    }

    fn get_tool_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.tool_item_id
    }

    fn mut_tool_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.tool_item_id
    }

    // repeated .CMsgRecipeComponent consumption_components = 2;

    pub fn clear_consumption_components(&mut self) {
        self.consumption_components.clear();
    }

    // Param is passed by value, moved
    pub fn set_consumption_components(&mut self, v: ::protobuf::RepeatedField<CMsgRecipeComponent>) {
        self.consumption_components = v;
    }

    // Mutable pointer to the field.
    pub fn mut_consumption_components(&mut self) -> &mut ::protobuf::RepeatedField<CMsgRecipeComponent> {
        &mut self.consumption_components
    }

    // Take field
    pub fn take_consumption_components(&mut self) -> ::protobuf::RepeatedField<CMsgRecipeComponent> {
        ::std::mem::replace(&mut self.consumption_components, ::protobuf::RepeatedField::new())
    }

    pub fn get_consumption_components(&self) -> &[CMsgRecipeComponent] {
        &self.consumption_components
    }

    fn get_consumption_components_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgRecipeComponent> {
        &self.consumption_components
    }

    fn mut_consumption_components_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgRecipeComponent> {
        &mut self.consumption_components
    }
}

impl ::protobuf::Message for CMsgFulfillDynamicRecipeComponent {
    fn is_initialized(&self) -> bool {
        for v in &self.consumption_components {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.tool_item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.consumption_components)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.tool_item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.consumption_components {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tool_item_id {
            os.write_uint64(1, v)?;
        }
        for v in &self.consumption_components {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgFulfillDynamicRecipeComponent {
    fn new() -> CMsgFulfillDynamicRecipeComponent {
        CMsgFulfillDynamicRecipeComponent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgFulfillDynamicRecipeComponent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "tool_item_id",
                    CMsgFulfillDynamicRecipeComponent::get_tool_item_id_for_reflect,
                    CMsgFulfillDynamicRecipeComponent::mut_tool_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgRecipeComponent>>(
                    "consumption_components",
                    CMsgFulfillDynamicRecipeComponent::get_consumption_components_for_reflect,
                    CMsgFulfillDynamicRecipeComponent::mut_consumption_components_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgFulfillDynamicRecipeComponent>(
                    "CMsgFulfillDynamicRecipeComponent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgFulfillDynamicRecipeComponent {
    fn clear(&mut self) {
        self.clear_tool_item_id();
        self.clear_consumption_components();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgFulfillDynamicRecipeComponent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgFulfillDynamicRecipeComponent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCClientMarketDataRequest {
    // message fields
    user_currency: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCClientMarketDataRequest {}

impl CMsgGCClientMarketDataRequest {
    pub fn new() -> CMsgGCClientMarketDataRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCClientMarketDataRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCClientMarketDataRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCClientMarketDataRequest,
        };
        unsafe {
            instance.get(CMsgGCClientMarketDataRequest::new)
        }
    }

    // optional uint32 user_currency = 1;

    pub fn clear_user_currency(&mut self) {
        self.user_currency = ::std::option::Option::None;
    }

    pub fn has_user_currency(&self) -> bool {
        self.user_currency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_currency(&mut self, v: u32) {
        self.user_currency = ::std::option::Option::Some(v);
    }

    pub fn get_user_currency(&self) -> u32 {
        self.user_currency.unwrap_or(0)
    }

    fn get_user_currency_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.user_currency
    }

    fn mut_user_currency_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.user_currency
    }
}

impl ::protobuf::Message for CMsgGCClientMarketDataRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.user_currency = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.user_currency {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.user_currency {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCClientMarketDataRequest {
    fn new() -> CMsgGCClientMarketDataRequest {
        CMsgGCClientMarketDataRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCClientMarketDataRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "user_currency",
                    CMsgGCClientMarketDataRequest::get_user_currency_for_reflect,
                    CMsgGCClientMarketDataRequest::mut_user_currency_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCClientMarketDataRequest>(
                    "CMsgGCClientMarketDataRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCClientMarketDataRequest {
    fn clear(&mut self) {
        self.clear_user_currency();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCClientMarketDataRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCClientMarketDataRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCClientMarketDataEntry {
    // message fields
    item_def_index: ::std::option::Option<u32>,
    item_quality: ::std::option::Option<u32>,
    item_sell_listings: ::std::option::Option<u32>,
    price_in_local_currency: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCClientMarketDataEntry {}

impl CMsgGCClientMarketDataEntry {
    pub fn new() -> CMsgGCClientMarketDataEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCClientMarketDataEntry {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCClientMarketDataEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCClientMarketDataEntry,
        };
        unsafe {
            instance.get(CMsgGCClientMarketDataEntry::new)
        }
    }

    // optional uint32 item_def_index = 1;

    pub fn clear_item_def_index(&mut self) {
        self.item_def_index = ::std::option::Option::None;
    }

    pub fn has_item_def_index(&self) -> bool {
        self.item_def_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_def_index(&mut self, v: u32) {
        self.item_def_index = ::std::option::Option::Some(v);
    }

    pub fn get_item_def_index(&self) -> u32 {
        self.item_def_index.unwrap_or(0)
    }

    fn get_item_def_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_def_index
    }

    fn mut_item_def_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_def_index
    }

    // optional uint32 item_quality = 2;

    pub fn clear_item_quality(&mut self) {
        self.item_quality = ::std::option::Option::None;
    }

    pub fn has_item_quality(&self) -> bool {
        self.item_quality.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_quality(&mut self, v: u32) {
        self.item_quality = ::std::option::Option::Some(v);
    }

    pub fn get_item_quality(&self) -> u32 {
        self.item_quality.unwrap_or(0)
    }

    fn get_item_quality_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_quality
    }

    fn mut_item_quality_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_quality
    }

    // optional uint32 item_sell_listings = 3;

    pub fn clear_item_sell_listings(&mut self) {
        self.item_sell_listings = ::std::option::Option::None;
    }

    pub fn has_item_sell_listings(&self) -> bool {
        self.item_sell_listings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_sell_listings(&mut self, v: u32) {
        self.item_sell_listings = ::std::option::Option::Some(v);
    }

    pub fn get_item_sell_listings(&self) -> u32 {
        self.item_sell_listings.unwrap_or(0)
    }

    fn get_item_sell_listings_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_sell_listings
    }

    fn mut_item_sell_listings_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_sell_listings
    }

    // optional uint32 price_in_local_currency = 4;

    pub fn clear_price_in_local_currency(&mut self) {
        self.price_in_local_currency = ::std::option::Option::None;
    }

    pub fn has_price_in_local_currency(&self) -> bool {
        self.price_in_local_currency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_price_in_local_currency(&mut self, v: u32) {
        self.price_in_local_currency = ::std::option::Option::Some(v);
    }

    pub fn get_price_in_local_currency(&self) -> u32 {
        self.price_in_local_currency.unwrap_or(0)
    }

    fn get_price_in_local_currency_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.price_in_local_currency
    }

    fn mut_price_in_local_currency_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.price_in_local_currency
    }
}

impl ::protobuf::Message for CMsgGCClientMarketDataEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.item_def_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.item_quality = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.item_sell_listings = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.price_in_local_currency = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.item_def_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.item_quality {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.item_sell_listings {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.price_in_local_currency {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_def_index {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.item_quality {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.item_sell_listings {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.price_in_local_currency {
            os.write_uint32(4, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCClientMarketDataEntry {
    fn new() -> CMsgGCClientMarketDataEntry {
        CMsgGCClientMarketDataEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCClientMarketDataEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_def_index",
                    CMsgGCClientMarketDataEntry::get_item_def_index_for_reflect,
                    CMsgGCClientMarketDataEntry::mut_item_def_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_quality",
                    CMsgGCClientMarketDataEntry::get_item_quality_for_reflect,
                    CMsgGCClientMarketDataEntry::mut_item_quality_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_sell_listings",
                    CMsgGCClientMarketDataEntry::get_item_sell_listings_for_reflect,
                    CMsgGCClientMarketDataEntry::mut_item_sell_listings_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "price_in_local_currency",
                    CMsgGCClientMarketDataEntry::get_price_in_local_currency_for_reflect,
                    CMsgGCClientMarketDataEntry::mut_price_in_local_currency_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCClientMarketDataEntry>(
                    "CMsgGCClientMarketDataEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCClientMarketDataEntry {
    fn clear(&mut self) {
        self.clear_item_def_index();
        self.clear_item_quality();
        self.clear_item_sell_listings();
        self.clear_price_in_local_currency();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCClientMarketDataEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCClientMarketDataEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCClientMarketData {
    // message fields
    entries: ::protobuf::RepeatedField<CMsgGCClientMarketDataEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCClientMarketData {}

impl CMsgGCClientMarketData {
    pub fn new() -> CMsgGCClientMarketData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCClientMarketData {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCClientMarketData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCClientMarketData,
        };
        unsafe {
            instance.get(CMsgGCClientMarketData::new)
        }
    }

    // repeated .CMsgGCClientMarketDataEntry entries = 1;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<CMsgGCClientMarketDataEntry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCClientMarketDataEntry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<CMsgGCClientMarketDataEntry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[CMsgGCClientMarketDataEntry] {
        &self.entries
    }

    fn get_entries_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgGCClientMarketDataEntry> {
        &self.entries
    }

    fn mut_entries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCClientMarketDataEntry> {
        &mut self.entries
    }
}

impl ::protobuf::Message for CMsgGCClientMarketData {
    fn is_initialized(&self) -> bool {
        for v in &self.entries {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.entries {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCClientMarketData {
    fn new() -> CMsgGCClientMarketData {
        CMsgGCClientMarketData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCClientMarketData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgGCClientMarketDataEntry>>(
                    "entries",
                    CMsgGCClientMarketData::get_entries_for_reflect,
                    CMsgGCClientMarketData::mut_entries_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCClientMarketData>(
                    "CMsgGCClientMarketData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCClientMarketData {
    fn clear(&mut self) {
        self.clear_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCClientMarketData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCClientMarketData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgExtractGems {
    // message fields
    tool_item_id: ::std::option::Option<u64>,
    item_item_id: ::std::option::Option<u64>,
    item_socket_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgExtractGems {}

impl CMsgExtractGems {
    pub fn new() -> CMsgExtractGems {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgExtractGems {
        static mut instance: ::protobuf::lazy::Lazy<CMsgExtractGems> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgExtractGems,
        };
        unsafe {
            instance.get(CMsgExtractGems::new)
        }
    }

    // optional uint64 tool_item_id = 1;

    pub fn clear_tool_item_id(&mut self) {
        self.tool_item_id = ::std::option::Option::None;
    }

    pub fn has_tool_item_id(&self) -> bool {
        self.tool_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tool_item_id(&mut self, v: u64) {
        self.tool_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_tool_item_id(&self) -> u64 {
        self.tool_item_id.unwrap_or(0)
    }

    fn get_tool_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.tool_item_id
    }

    fn mut_tool_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.tool_item_id
    }

    // optional uint64 item_item_id = 2;

    pub fn clear_item_item_id(&mut self) {
        self.item_item_id = ::std::option::Option::None;
    }

    pub fn has_item_item_id(&self) -> bool {
        self.item_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_item_id(&mut self, v: u64) {
        self.item_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_item_id(&self) -> u64 {
        self.item_item_id.unwrap_or(0)
    }

    fn get_item_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.item_item_id
    }

    fn mut_item_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.item_item_id
    }

    // optional uint32 item_socket_id = 3;

    pub fn clear_item_socket_id(&mut self) {
        self.item_socket_id = ::std::option::Option::None;
    }

    pub fn has_item_socket_id(&self) -> bool {
        self.item_socket_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_socket_id(&mut self, v: u32) {
        self.item_socket_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_socket_id(&self) -> u32 {
        self.item_socket_id.unwrap_or(65535u32)
    }

    fn get_item_socket_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_socket_id
    }

    fn mut_item_socket_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_socket_id
    }
}

impl ::protobuf::Message for CMsgExtractGems {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.tool_item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.item_item_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.item_socket_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.tool_item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.item_item_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.item_socket_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tool_item_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.item_item_id {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.item_socket_id {
            os.write_uint32(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgExtractGems {
    fn new() -> CMsgExtractGems {
        CMsgExtractGems::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgExtractGems>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "tool_item_id",
                    CMsgExtractGems::get_tool_item_id_for_reflect,
                    CMsgExtractGems::mut_tool_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "item_item_id",
                    CMsgExtractGems::get_item_item_id_for_reflect,
                    CMsgExtractGems::mut_item_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_socket_id",
                    CMsgExtractGems::get_item_socket_id_for_reflect,
                    CMsgExtractGems::mut_item_socket_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgExtractGems>(
                    "CMsgExtractGems",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgExtractGems {
    fn clear(&mut self) {
        self.clear_tool_item_id();
        self.clear_item_item_id();
        self.clear_item_socket_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgExtractGems {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgExtractGems {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgExtractGemsResponse {
    // message fields
    item_id: ::std::option::Option<u64>,
    response: ::std::option::Option<CMsgExtractGemsResponse_EExtractGems>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgExtractGemsResponse {}

impl CMsgExtractGemsResponse {
    pub fn new() -> CMsgExtractGemsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgExtractGemsResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgExtractGemsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgExtractGemsResponse,
        };
        unsafe {
            instance.get(CMsgExtractGemsResponse::new)
        }
    }

    // optional uint64 item_id = 1;

    pub fn clear_item_id(&mut self) {
        self.item_id = ::std::option::Option::None;
    }

    pub fn has_item_id(&self) -> bool {
        self.item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_id(&mut self, v: u64) {
        self.item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_id(&self) -> u64 {
        self.item_id.unwrap_or(0)
    }

    fn get_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.item_id
    }

    fn mut_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.item_id
    }

    // optional .CMsgExtractGemsResponse.EExtractGems response = 2;

    pub fn clear_response(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: CMsgExtractGemsResponse_EExtractGems) {
        self.response = ::std::option::Option::Some(v);
    }

    pub fn get_response(&self) -> CMsgExtractGemsResponse_EExtractGems {
        self.response.unwrap_or(CMsgExtractGemsResponse_EExtractGems::k_ExtractGems_Succeeded)
    }

    fn get_response_for_reflect(&self) -> &::std::option::Option<CMsgExtractGemsResponse_EExtractGems> {
        &self.response
    }

    fn mut_response_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgExtractGemsResponse_EExtractGems> {
        &mut self.response
    }
}

impl ::protobuf::Message for CMsgExtractGemsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.response = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.response {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.response {
            os.write_enum(2, v.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgExtractGemsResponse {
    fn new() -> CMsgExtractGemsResponse {
        CMsgExtractGemsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgExtractGemsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "item_id",
                    CMsgExtractGemsResponse::get_item_id_for_reflect,
                    CMsgExtractGemsResponse::mut_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgExtractGemsResponse_EExtractGems>>(
                    "response",
                    CMsgExtractGemsResponse::get_response_for_reflect,
                    CMsgExtractGemsResponse::mut_response_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgExtractGemsResponse>(
                    "CMsgExtractGemsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgExtractGemsResponse {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_response();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgExtractGemsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgExtractGemsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgExtractGemsResponse_EExtractGems {
    k_ExtractGems_Succeeded = 0,
    k_ExtractGems_Failed_ToolIsInvalid = 1,
    k_ExtractGems_Failed_ItemIsInvalid = 2,
    k_ExtractGems_Failed_ToolCannotRemoveGem = 3,
    k_ExtractGems_Failed_FailedToRemoveGem = 4,
}

impl ::protobuf::ProtobufEnum for CMsgExtractGemsResponse_EExtractGems {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgExtractGemsResponse_EExtractGems> {
        match value {
            0 => ::std::option::Option::Some(CMsgExtractGemsResponse_EExtractGems::k_ExtractGems_Succeeded),
            1 => ::std::option::Option::Some(CMsgExtractGemsResponse_EExtractGems::k_ExtractGems_Failed_ToolIsInvalid),
            2 => ::std::option::Option::Some(CMsgExtractGemsResponse_EExtractGems::k_ExtractGems_Failed_ItemIsInvalid),
            3 => ::std::option::Option::Some(CMsgExtractGemsResponse_EExtractGems::k_ExtractGems_Failed_ToolCannotRemoveGem),
            4 => ::std::option::Option::Some(CMsgExtractGemsResponse_EExtractGems::k_ExtractGems_Failed_FailedToRemoveGem),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgExtractGemsResponse_EExtractGems] = &[
            CMsgExtractGemsResponse_EExtractGems::k_ExtractGems_Succeeded,
            CMsgExtractGemsResponse_EExtractGems::k_ExtractGems_Failed_ToolIsInvalid,
            CMsgExtractGemsResponse_EExtractGems::k_ExtractGems_Failed_ItemIsInvalid,
            CMsgExtractGemsResponse_EExtractGems::k_ExtractGems_Failed_ToolCannotRemoveGem,
            CMsgExtractGemsResponse_EExtractGems::k_ExtractGems_Failed_FailedToRemoveGem,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CMsgExtractGemsResponse_EExtractGems>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgExtractGemsResponse_EExtractGems", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgExtractGemsResponse_EExtractGems {
}

impl ::protobuf::reflect::ProtobufValue for CMsgExtractGemsResponse_EExtractGems {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAddSocket {
    // message fields
    tool_item_id: ::std::option::Option<u64>,
    item_item_id: ::std::option::Option<u64>,
    unusual: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAddSocket {}

impl CMsgAddSocket {
    pub fn new() -> CMsgAddSocket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAddSocket {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAddSocket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAddSocket,
        };
        unsafe {
            instance.get(CMsgAddSocket::new)
        }
    }

    // optional uint64 tool_item_id = 1;

    pub fn clear_tool_item_id(&mut self) {
        self.tool_item_id = ::std::option::Option::None;
    }

    pub fn has_tool_item_id(&self) -> bool {
        self.tool_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tool_item_id(&mut self, v: u64) {
        self.tool_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_tool_item_id(&self) -> u64 {
        self.tool_item_id.unwrap_or(0)
    }

    fn get_tool_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.tool_item_id
    }

    fn mut_tool_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.tool_item_id
    }

    // optional uint64 item_item_id = 2;

    pub fn clear_item_item_id(&mut self) {
        self.item_item_id = ::std::option::Option::None;
    }

    pub fn has_item_item_id(&self) -> bool {
        self.item_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_item_id(&mut self, v: u64) {
        self.item_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_item_id(&self) -> u64 {
        self.item_item_id.unwrap_or(0)
    }

    fn get_item_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.item_item_id
    }

    fn mut_item_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.item_item_id
    }

    // optional bool unusual = 3;

    pub fn clear_unusual(&mut self) {
        self.unusual = ::std::option::Option::None;
    }

    pub fn has_unusual(&self) -> bool {
        self.unusual.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unusual(&mut self, v: bool) {
        self.unusual = ::std::option::Option::Some(v);
    }

    pub fn get_unusual(&self) -> bool {
        self.unusual.unwrap_or(false)
    }

    fn get_unusual_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.unusual
    }

    fn mut_unusual_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.unusual
    }
}

impl ::protobuf::Message for CMsgAddSocket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.tool_item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.item_item_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.unusual = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.tool_item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.item_item_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.unusual {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tool_item_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.item_item_id {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.unusual {
            os.write_bool(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAddSocket {
    fn new() -> CMsgAddSocket {
        CMsgAddSocket::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAddSocket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "tool_item_id",
                    CMsgAddSocket::get_tool_item_id_for_reflect,
                    CMsgAddSocket::mut_tool_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "item_item_id",
                    CMsgAddSocket::get_item_item_id_for_reflect,
                    CMsgAddSocket::mut_item_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "unusual",
                    CMsgAddSocket::get_unusual_for_reflect,
                    CMsgAddSocket::mut_unusual_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAddSocket>(
                    "CMsgAddSocket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAddSocket {
    fn clear(&mut self) {
        self.clear_tool_item_id();
        self.clear_item_item_id();
        self.clear_unusual();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAddSocket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAddSocket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAddSocketResponse {
    // message fields
    item_id: ::std::option::Option<u64>,
    updated_socket_index: ::std::vec::Vec<u32>,
    response: ::std::option::Option<CMsgAddSocketResponse_EAddSocket>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAddSocketResponse {}

impl CMsgAddSocketResponse {
    pub fn new() -> CMsgAddSocketResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAddSocketResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAddSocketResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAddSocketResponse,
        };
        unsafe {
            instance.get(CMsgAddSocketResponse::new)
        }
    }

    // optional uint64 item_id = 1;

    pub fn clear_item_id(&mut self) {
        self.item_id = ::std::option::Option::None;
    }

    pub fn has_item_id(&self) -> bool {
        self.item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_id(&mut self, v: u64) {
        self.item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_id(&self) -> u64 {
        self.item_id.unwrap_or(0)
    }

    fn get_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.item_id
    }

    fn mut_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.item_id
    }

    // repeated uint32 updated_socket_index = 2;

    pub fn clear_updated_socket_index(&mut self) {
        self.updated_socket_index.clear();
    }

    // Param is passed by value, moved
    pub fn set_updated_socket_index(&mut self, v: ::std::vec::Vec<u32>) {
        self.updated_socket_index = v;
    }

    // Mutable pointer to the field.
    pub fn mut_updated_socket_index(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.updated_socket_index
    }

    // Take field
    pub fn take_updated_socket_index(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.updated_socket_index, ::std::vec::Vec::new())
    }

    pub fn get_updated_socket_index(&self) -> &[u32] {
        &self.updated_socket_index
    }

    fn get_updated_socket_index_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.updated_socket_index
    }

    fn mut_updated_socket_index_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.updated_socket_index
    }

    // optional .CMsgAddSocketResponse.EAddSocket response = 3;

    pub fn clear_response(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: CMsgAddSocketResponse_EAddSocket) {
        self.response = ::std::option::Option::Some(v);
    }

    pub fn get_response(&self) -> CMsgAddSocketResponse_EAddSocket {
        self.response.unwrap_or(CMsgAddSocketResponse_EAddSocket::k_AddSocket_Succeeded)
    }

    fn get_response_for_reflect(&self) -> &::std::option::Option<CMsgAddSocketResponse_EAddSocket> {
        &self.response
    }

    fn mut_response_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgAddSocketResponse_EAddSocket> {
        &mut self.response
    }
}

impl ::protobuf::Message for CMsgAddSocketResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.updated_socket_index)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.response = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.updated_socket_index {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.response {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id {
            os.write_uint64(1, v)?;
        }
        for v in &self.updated_socket_index {
            os.write_uint32(2, *v)?;
        };
        if let Some(v) = self.response {
            os.write_enum(3, v.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAddSocketResponse {
    fn new() -> CMsgAddSocketResponse {
        CMsgAddSocketResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAddSocketResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "item_id",
                    CMsgAddSocketResponse::get_item_id_for_reflect,
                    CMsgAddSocketResponse::mut_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "updated_socket_index",
                    CMsgAddSocketResponse::get_updated_socket_index_for_reflect,
                    CMsgAddSocketResponse::mut_updated_socket_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgAddSocketResponse_EAddSocket>>(
                    "response",
                    CMsgAddSocketResponse::get_response_for_reflect,
                    CMsgAddSocketResponse::mut_response_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAddSocketResponse>(
                    "CMsgAddSocketResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAddSocketResponse {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_updated_socket_index();
        self.clear_response();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAddSocketResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAddSocketResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgAddSocketResponse_EAddSocket {
    k_AddSocket_Succeeded = 0,
    k_AddSocket_Failed_ToolIsInvalid = 1,
    k_AddSocket_Failed_ItemCannotBeSocketed = 2,
    k_AddSocket_Failed_FailedToAddSocket = 3,
}

impl ::protobuf::ProtobufEnum for CMsgAddSocketResponse_EAddSocket {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgAddSocketResponse_EAddSocket> {
        match value {
            0 => ::std::option::Option::Some(CMsgAddSocketResponse_EAddSocket::k_AddSocket_Succeeded),
            1 => ::std::option::Option::Some(CMsgAddSocketResponse_EAddSocket::k_AddSocket_Failed_ToolIsInvalid),
            2 => ::std::option::Option::Some(CMsgAddSocketResponse_EAddSocket::k_AddSocket_Failed_ItemCannotBeSocketed),
            3 => ::std::option::Option::Some(CMsgAddSocketResponse_EAddSocket::k_AddSocket_Failed_FailedToAddSocket),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgAddSocketResponse_EAddSocket] = &[
            CMsgAddSocketResponse_EAddSocket::k_AddSocket_Succeeded,
            CMsgAddSocketResponse_EAddSocket::k_AddSocket_Failed_ToolIsInvalid,
            CMsgAddSocketResponse_EAddSocket::k_AddSocket_Failed_ItemCannotBeSocketed,
            CMsgAddSocketResponse_EAddSocket::k_AddSocket_Failed_FailedToAddSocket,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CMsgAddSocketResponse_EAddSocket>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgAddSocketResponse_EAddSocket", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgAddSocketResponse_EAddSocket {
}

impl ::protobuf::reflect::ProtobufValue for CMsgAddSocketResponse_EAddSocket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAddItemToSocketData {
    // message fields
    gem_item_id: ::std::option::Option<u64>,
    socket_index: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAddItemToSocketData {}

impl CMsgAddItemToSocketData {
    pub fn new() -> CMsgAddItemToSocketData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAddItemToSocketData {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAddItemToSocketData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAddItemToSocketData,
        };
        unsafe {
            instance.get(CMsgAddItemToSocketData::new)
        }
    }

    // optional uint64 gem_item_id = 1;

    pub fn clear_gem_item_id(&mut self) {
        self.gem_item_id = ::std::option::Option::None;
    }

    pub fn has_gem_item_id(&self) -> bool {
        self.gem_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gem_item_id(&mut self, v: u64) {
        self.gem_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_gem_item_id(&self) -> u64 {
        self.gem_item_id.unwrap_or(0)
    }

    fn get_gem_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.gem_item_id
    }

    fn mut_gem_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.gem_item_id
    }

    // optional uint32 socket_index = 2;

    pub fn clear_socket_index(&mut self) {
        self.socket_index = ::std::option::Option::None;
    }

    pub fn has_socket_index(&self) -> bool {
        self.socket_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_socket_index(&mut self, v: u32) {
        self.socket_index = ::std::option::Option::Some(v);
    }

    pub fn get_socket_index(&self) -> u32 {
        self.socket_index.unwrap_or(0)
    }

    fn get_socket_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.socket_index
    }

    fn mut_socket_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.socket_index
    }
}

impl ::protobuf::Message for CMsgAddItemToSocketData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.gem_item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.socket_index = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.gem_item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.socket_index {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gem_item_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.socket_index {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAddItemToSocketData {
    fn new() -> CMsgAddItemToSocketData {
        CMsgAddItemToSocketData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAddItemToSocketData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "gem_item_id",
                    CMsgAddItemToSocketData::get_gem_item_id_for_reflect,
                    CMsgAddItemToSocketData::mut_gem_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "socket_index",
                    CMsgAddItemToSocketData::get_socket_index_for_reflect,
                    CMsgAddItemToSocketData::mut_socket_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAddItemToSocketData>(
                    "CMsgAddItemToSocketData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAddItemToSocketData {
    fn clear(&mut self) {
        self.clear_gem_item_id();
        self.clear_socket_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAddItemToSocketData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAddItemToSocketData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAddItemToSocket {
    // message fields
    item_item_id: ::std::option::Option<u64>,
    gems_to_socket: ::protobuf::RepeatedField<CMsgAddItemToSocketData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAddItemToSocket {}

impl CMsgAddItemToSocket {
    pub fn new() -> CMsgAddItemToSocket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAddItemToSocket {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAddItemToSocket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAddItemToSocket,
        };
        unsafe {
            instance.get(CMsgAddItemToSocket::new)
        }
    }

    // optional uint64 item_item_id = 1;

    pub fn clear_item_item_id(&mut self) {
        self.item_item_id = ::std::option::Option::None;
    }

    pub fn has_item_item_id(&self) -> bool {
        self.item_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_item_id(&mut self, v: u64) {
        self.item_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_item_id(&self) -> u64 {
        self.item_item_id.unwrap_or(0)
    }

    fn get_item_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.item_item_id
    }

    fn mut_item_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.item_item_id
    }

    // repeated .CMsgAddItemToSocketData gems_to_socket = 2;

    pub fn clear_gems_to_socket(&mut self) {
        self.gems_to_socket.clear();
    }

    // Param is passed by value, moved
    pub fn set_gems_to_socket(&mut self, v: ::protobuf::RepeatedField<CMsgAddItemToSocketData>) {
        self.gems_to_socket = v;
    }

    // Mutable pointer to the field.
    pub fn mut_gems_to_socket(&mut self) -> &mut ::protobuf::RepeatedField<CMsgAddItemToSocketData> {
        &mut self.gems_to_socket
    }

    // Take field
    pub fn take_gems_to_socket(&mut self) -> ::protobuf::RepeatedField<CMsgAddItemToSocketData> {
        ::std::mem::replace(&mut self.gems_to_socket, ::protobuf::RepeatedField::new())
    }

    pub fn get_gems_to_socket(&self) -> &[CMsgAddItemToSocketData] {
        &self.gems_to_socket
    }

    fn get_gems_to_socket_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgAddItemToSocketData> {
        &self.gems_to_socket
    }

    fn mut_gems_to_socket_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgAddItemToSocketData> {
        &mut self.gems_to_socket
    }
}

impl ::protobuf::Message for CMsgAddItemToSocket {
    fn is_initialized(&self) -> bool {
        for v in &self.gems_to_socket {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.item_item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.gems_to_socket)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.item_item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.gems_to_socket {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_item_id {
            os.write_uint64(1, v)?;
        }
        for v in &self.gems_to_socket {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAddItemToSocket {
    fn new() -> CMsgAddItemToSocket {
        CMsgAddItemToSocket::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAddItemToSocket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "item_item_id",
                    CMsgAddItemToSocket::get_item_item_id_for_reflect,
                    CMsgAddItemToSocket::mut_item_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgAddItemToSocketData>>(
                    "gems_to_socket",
                    CMsgAddItemToSocket::get_gems_to_socket_for_reflect,
                    CMsgAddItemToSocket::mut_gems_to_socket_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAddItemToSocket>(
                    "CMsgAddItemToSocket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAddItemToSocket {
    fn clear(&mut self) {
        self.clear_item_item_id();
        self.clear_gems_to_socket();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAddItemToSocket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAddItemToSocket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAddItemToSocketResponse {
    // message fields
    item_item_id: ::std::option::Option<u64>,
    updated_socket_index: ::std::vec::Vec<u32>,
    response: ::std::option::Option<CMsgAddItemToSocketResponse_EAddGem>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAddItemToSocketResponse {}

impl CMsgAddItemToSocketResponse {
    pub fn new() -> CMsgAddItemToSocketResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAddItemToSocketResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAddItemToSocketResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAddItemToSocketResponse,
        };
        unsafe {
            instance.get(CMsgAddItemToSocketResponse::new)
        }
    }

    // optional uint64 item_item_id = 1;

    pub fn clear_item_item_id(&mut self) {
        self.item_item_id = ::std::option::Option::None;
    }

    pub fn has_item_item_id(&self) -> bool {
        self.item_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_item_id(&mut self, v: u64) {
        self.item_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_item_id(&self) -> u64 {
        self.item_item_id.unwrap_or(0)
    }

    fn get_item_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.item_item_id
    }

    fn mut_item_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.item_item_id
    }

    // repeated uint32 updated_socket_index = 2;

    pub fn clear_updated_socket_index(&mut self) {
        self.updated_socket_index.clear();
    }

    // Param is passed by value, moved
    pub fn set_updated_socket_index(&mut self, v: ::std::vec::Vec<u32>) {
        self.updated_socket_index = v;
    }

    // Mutable pointer to the field.
    pub fn mut_updated_socket_index(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.updated_socket_index
    }

    // Take field
    pub fn take_updated_socket_index(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.updated_socket_index, ::std::vec::Vec::new())
    }

    pub fn get_updated_socket_index(&self) -> &[u32] {
        &self.updated_socket_index
    }

    fn get_updated_socket_index_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.updated_socket_index
    }

    fn mut_updated_socket_index_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.updated_socket_index
    }

    // optional .CMsgAddItemToSocketResponse.EAddGem response = 3;

    pub fn clear_response(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: CMsgAddItemToSocketResponse_EAddGem) {
        self.response = ::std::option::Option::Some(v);
    }

    pub fn get_response(&self) -> CMsgAddItemToSocketResponse_EAddGem {
        self.response.unwrap_or(CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Succeeded)
    }

    fn get_response_for_reflect(&self) -> &::std::option::Option<CMsgAddItemToSocketResponse_EAddGem> {
        &self.response
    }

    fn mut_response_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgAddItemToSocketResponse_EAddGem> {
        &mut self.response
    }
}

impl ::protobuf::Message for CMsgAddItemToSocketResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.item_item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.updated_socket_index)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.response = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.item_item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.updated_socket_index {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.response {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_item_id {
            os.write_uint64(1, v)?;
        }
        for v in &self.updated_socket_index {
            os.write_uint32(2, *v)?;
        };
        if let Some(v) = self.response {
            os.write_enum(3, v.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAddItemToSocketResponse {
    fn new() -> CMsgAddItemToSocketResponse {
        CMsgAddItemToSocketResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAddItemToSocketResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "item_item_id",
                    CMsgAddItemToSocketResponse::get_item_item_id_for_reflect,
                    CMsgAddItemToSocketResponse::mut_item_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "updated_socket_index",
                    CMsgAddItemToSocketResponse::get_updated_socket_index_for_reflect,
                    CMsgAddItemToSocketResponse::mut_updated_socket_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgAddItemToSocketResponse_EAddGem>>(
                    "response",
                    CMsgAddItemToSocketResponse::get_response_for_reflect,
                    CMsgAddItemToSocketResponse::mut_response_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAddItemToSocketResponse>(
                    "CMsgAddItemToSocketResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAddItemToSocketResponse {
    fn clear(&mut self) {
        self.clear_item_item_id();
        self.clear_updated_socket_index();
        self.clear_response();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAddItemToSocketResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAddItemToSocketResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgAddItemToSocketResponse_EAddGem {
    k_AddGem_Succeeded = 0,
    k_AddGem_Failed_GemIsInvalid = 1,
    k_AddGem_Failed_ItemIsInvalid = 2,
    k_AddGem_Failed_FailedToAddGem = 3,
    k_AddGem_Failed_InvalidGemTypeForSocket = 4,
    k_AddGem_Failed_InvalidGemTypeForHero = 5,
    k_AddGem_Failed_InvalidGemTypeForSlot = 6,
    k_AddGem_Failed_SocketContainsUnremovableGem = 7,
}

impl ::protobuf::ProtobufEnum for CMsgAddItemToSocketResponse_EAddGem {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgAddItemToSocketResponse_EAddGem> {
        match value {
            0 => ::std::option::Option::Some(CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Succeeded),
            1 => ::std::option::Option::Some(CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Failed_GemIsInvalid),
            2 => ::std::option::Option::Some(CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Failed_ItemIsInvalid),
            3 => ::std::option::Option::Some(CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Failed_FailedToAddGem),
            4 => ::std::option::Option::Some(CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Failed_InvalidGemTypeForSocket),
            5 => ::std::option::Option::Some(CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Failed_InvalidGemTypeForHero),
            6 => ::std::option::Option::Some(CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Failed_InvalidGemTypeForSlot),
            7 => ::std::option::Option::Some(CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Failed_SocketContainsUnremovableGem),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgAddItemToSocketResponse_EAddGem] = &[
            CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Succeeded,
            CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Failed_GemIsInvalid,
            CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Failed_ItemIsInvalid,
            CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Failed_FailedToAddGem,
            CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Failed_InvalidGemTypeForSocket,
            CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Failed_InvalidGemTypeForHero,
            CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Failed_InvalidGemTypeForSlot,
            CMsgAddItemToSocketResponse_EAddGem::k_AddGem_Failed_SocketContainsUnremovableGem,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CMsgAddItemToSocketResponse_EAddGem>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgAddItemToSocketResponse_EAddGem", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgAddItemToSocketResponse_EAddGem {
}

impl ::protobuf::reflect::ProtobufValue for CMsgAddItemToSocketResponse_EAddGem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgResetStrangeGemCount {
    // message fields
    item_item_id: ::std::option::Option<u64>,
    socket_index: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgResetStrangeGemCount {}

impl CMsgResetStrangeGemCount {
    pub fn new() -> CMsgResetStrangeGemCount {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgResetStrangeGemCount {
        static mut instance: ::protobuf::lazy::Lazy<CMsgResetStrangeGemCount> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgResetStrangeGemCount,
        };
        unsafe {
            instance.get(CMsgResetStrangeGemCount::new)
        }
    }

    // optional uint64 item_item_id = 1;

    pub fn clear_item_item_id(&mut self) {
        self.item_item_id = ::std::option::Option::None;
    }

    pub fn has_item_item_id(&self) -> bool {
        self.item_item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_item_id(&mut self, v: u64) {
        self.item_item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_item_id(&self) -> u64 {
        self.item_item_id.unwrap_or(0)
    }

    fn get_item_item_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.item_item_id
    }

    fn mut_item_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.item_item_id
    }

    // optional uint32 socket_index = 2;

    pub fn clear_socket_index(&mut self) {
        self.socket_index = ::std::option::Option::None;
    }

    pub fn has_socket_index(&self) -> bool {
        self.socket_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_socket_index(&mut self, v: u32) {
        self.socket_index = ::std::option::Option::Some(v);
    }

    pub fn get_socket_index(&self) -> u32 {
        self.socket_index.unwrap_or(0)
    }

    fn get_socket_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.socket_index
    }

    fn mut_socket_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.socket_index
    }
}

impl ::protobuf::Message for CMsgResetStrangeGemCount {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.item_item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.socket_index = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.item_item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.socket_index {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_item_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.socket_index {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgResetStrangeGemCount {
    fn new() -> CMsgResetStrangeGemCount {
        CMsgResetStrangeGemCount::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgResetStrangeGemCount>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "item_item_id",
                    CMsgResetStrangeGemCount::get_item_item_id_for_reflect,
                    CMsgResetStrangeGemCount::mut_item_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "socket_index",
                    CMsgResetStrangeGemCount::get_socket_index_for_reflect,
                    CMsgResetStrangeGemCount::mut_socket_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgResetStrangeGemCount>(
                    "CMsgResetStrangeGemCount",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgResetStrangeGemCount {
    fn clear(&mut self) {
        self.clear_item_item_id();
        self.clear_socket_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgResetStrangeGemCount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgResetStrangeGemCount {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgResetStrangeGemCountResponse {
    // message fields
    response: ::std::option::Option<CMsgResetStrangeGemCountResponse_EResetGem>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgResetStrangeGemCountResponse {}

impl CMsgResetStrangeGemCountResponse {
    pub fn new() -> CMsgResetStrangeGemCountResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgResetStrangeGemCountResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgResetStrangeGemCountResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgResetStrangeGemCountResponse,
        };
        unsafe {
            instance.get(CMsgResetStrangeGemCountResponse::new)
        }
    }

    // optional .CMsgResetStrangeGemCountResponse.EResetGem response = 1;

    pub fn clear_response(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: CMsgResetStrangeGemCountResponse_EResetGem) {
        self.response = ::std::option::Option::Some(v);
    }

    pub fn get_response(&self) -> CMsgResetStrangeGemCountResponse_EResetGem {
        self.response.unwrap_or(CMsgResetStrangeGemCountResponse_EResetGem::k_ResetGem_Succeeded)
    }

    fn get_response_for_reflect(&self) -> &::std::option::Option<CMsgResetStrangeGemCountResponse_EResetGem> {
        &self.response
    }

    fn mut_response_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgResetStrangeGemCountResponse_EResetGem> {
        &mut self.response
    }
}

impl ::protobuf::Message for CMsgResetStrangeGemCountResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.response = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.response {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.response {
            os.write_enum(1, v.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgResetStrangeGemCountResponse {
    fn new() -> CMsgResetStrangeGemCountResponse {
        CMsgResetStrangeGemCountResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgResetStrangeGemCountResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgResetStrangeGemCountResponse_EResetGem>>(
                    "response",
                    CMsgResetStrangeGemCountResponse::get_response_for_reflect,
                    CMsgResetStrangeGemCountResponse::mut_response_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgResetStrangeGemCountResponse>(
                    "CMsgResetStrangeGemCountResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgResetStrangeGemCountResponse {
    fn clear(&mut self) {
        self.clear_response();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgResetStrangeGemCountResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgResetStrangeGemCountResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgResetStrangeGemCountResponse_EResetGem {
    k_ResetGem_Succeeded = 0,
    k_ResetGem_Failed_FailedToResetGem = 1,
    k_ResetGem_Failed_ItemIsInvalid = 2,
    k_ResetGem_Failed_InvalidSocketId = 3,
    k_ResetGem_Failed_SocketCannotBeReset = 4,
}

impl ::protobuf::ProtobufEnum for CMsgResetStrangeGemCountResponse_EResetGem {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgResetStrangeGemCountResponse_EResetGem> {
        match value {
            0 => ::std::option::Option::Some(CMsgResetStrangeGemCountResponse_EResetGem::k_ResetGem_Succeeded),
            1 => ::std::option::Option::Some(CMsgResetStrangeGemCountResponse_EResetGem::k_ResetGem_Failed_FailedToResetGem),
            2 => ::std::option::Option::Some(CMsgResetStrangeGemCountResponse_EResetGem::k_ResetGem_Failed_ItemIsInvalid),
            3 => ::std::option::Option::Some(CMsgResetStrangeGemCountResponse_EResetGem::k_ResetGem_Failed_InvalidSocketId),
            4 => ::std::option::Option::Some(CMsgResetStrangeGemCountResponse_EResetGem::k_ResetGem_Failed_SocketCannotBeReset),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgResetStrangeGemCountResponse_EResetGem] = &[
            CMsgResetStrangeGemCountResponse_EResetGem::k_ResetGem_Succeeded,
            CMsgResetStrangeGemCountResponse_EResetGem::k_ResetGem_Failed_FailedToResetGem,
            CMsgResetStrangeGemCountResponse_EResetGem::k_ResetGem_Failed_ItemIsInvalid,
            CMsgResetStrangeGemCountResponse_EResetGem::k_ResetGem_Failed_InvalidSocketId,
            CMsgResetStrangeGemCountResponse_EResetGem::k_ResetGem_Failed_SocketCannotBeReset,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CMsgResetStrangeGemCountResponse_EResetGem>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgResetStrangeGemCountResponse_EResetGem", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgResetStrangeGemCountResponse_EResetGem {
}

impl ::protobuf::reflect::ProtobufValue for CMsgResetStrangeGemCountResponse_EResetGem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToClientPollFileRequest {
    // message fields
    file_name: ::protobuf::SingularField<::std::string::String>,
    client_version: ::std::option::Option<u32>,
    poll_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToClientPollFileRequest {}

impl CMsgGCToClientPollFileRequest {
    pub fn new() -> CMsgGCToClientPollFileRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToClientPollFileRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToClientPollFileRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToClientPollFileRequest,
        };
        unsafe {
            instance.get(CMsgGCToClientPollFileRequest::new)
        }
    }

    // optional string file_name = 1;

    pub fn clear_file_name(&mut self) {
        self.file_name.clear();
    }

    pub fn has_file_name(&self) -> bool {
        self.file_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_name(&mut self, v: ::std::string::String) {
        self.file_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_name(&mut self) -> &mut ::std::string::String {
        if self.file_name.is_none() {
            self.file_name.set_default();
        }
        self.file_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_file_name(&mut self) -> ::std::string::String {
        self.file_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_file_name(&self) -> &str {
        match self.file_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_file_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.file_name
    }

    fn mut_file_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.file_name
    }

    // optional uint32 client_version = 2;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }

    // optional uint32 poll_id = 3;

    pub fn clear_poll_id(&mut self) {
        self.poll_id = ::std::option::Option::None;
    }

    pub fn has_poll_id(&self) -> bool {
        self.poll_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_poll_id(&mut self, v: u32) {
        self.poll_id = ::std::option::Option::Some(v);
    }

    pub fn get_poll_id(&self) -> u32 {
        self.poll_id.unwrap_or(0)
    }

    fn get_poll_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.poll_id
    }

    fn mut_poll_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.poll_id
    }
}

impl ::protobuf::Message for CMsgGCToClientPollFileRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.file_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.poll_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.file_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.poll_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.file_name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.client_version {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.poll_id {
            os.write_uint32(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCToClientPollFileRequest {
    fn new() -> CMsgGCToClientPollFileRequest {
        CMsgGCToClientPollFileRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToClientPollFileRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "file_name",
                    CMsgGCToClientPollFileRequest::get_file_name_for_reflect,
                    CMsgGCToClientPollFileRequest::mut_file_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgGCToClientPollFileRequest::get_client_version_for_reflect,
                    CMsgGCToClientPollFileRequest::mut_client_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "poll_id",
                    CMsgGCToClientPollFileRequest::get_poll_id_for_reflect,
                    CMsgGCToClientPollFileRequest::mut_poll_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToClientPollFileRequest>(
                    "CMsgGCToClientPollFileRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToClientPollFileRequest {
    fn clear(&mut self) {
        self.clear_file_name();
        self.clear_client_version();
        self.clear_poll_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToClientPollFileRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToClientPollFileRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToClientPollFileResponse {
    // message fields
    poll_id: ::std::option::Option<u32>,
    file_size: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToClientPollFileResponse {}

impl CMsgGCToClientPollFileResponse {
    pub fn new() -> CMsgGCToClientPollFileResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToClientPollFileResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToClientPollFileResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToClientPollFileResponse,
        };
        unsafe {
            instance.get(CMsgGCToClientPollFileResponse::new)
        }
    }

    // optional uint32 poll_id = 1;

    pub fn clear_poll_id(&mut self) {
        self.poll_id = ::std::option::Option::None;
    }

    pub fn has_poll_id(&self) -> bool {
        self.poll_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_poll_id(&mut self, v: u32) {
        self.poll_id = ::std::option::Option::Some(v);
    }

    pub fn get_poll_id(&self) -> u32 {
        self.poll_id.unwrap_or(0)
    }

    fn get_poll_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.poll_id
    }

    fn mut_poll_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.poll_id
    }

    // optional uint32 file_size = 2;

    pub fn clear_file_size(&mut self) {
        self.file_size = ::std::option::Option::None;
    }

    pub fn has_file_size(&self) -> bool {
        self.file_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_size(&mut self, v: u32) {
        self.file_size = ::std::option::Option::Some(v);
    }

    pub fn get_file_size(&self) -> u32 {
        self.file_size.unwrap_or(0)
    }

    fn get_file_size_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.file_size
    }

    fn mut_file_size_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.file_size
    }
}

impl ::protobuf::Message for CMsgGCToClientPollFileResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.poll_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.file_size = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.poll_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.file_size {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.poll_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.file_size {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCToClientPollFileResponse {
    fn new() -> CMsgGCToClientPollFileResponse {
        CMsgGCToClientPollFileResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToClientPollFileResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "poll_id",
                    CMsgGCToClientPollFileResponse::get_poll_id_for_reflect,
                    CMsgGCToClientPollFileResponse::mut_poll_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "file_size",
                    CMsgGCToClientPollFileResponse::get_file_size_for_reflect,
                    CMsgGCToClientPollFileResponse::mut_file_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToClientPollFileResponse>(
                    "CMsgGCToClientPollFileResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToClientPollFileResponse {
    fn clear(&mut self) {
        self.clear_poll_id();
        self.clear_file_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToClientPollFileResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToClientPollFileResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EGCBaseMsg {
    k_EMsgGCSystemMessage = 4001,
    k_EMsgGCReplicateConVars = 4002,
    k_EMsgGCConVarUpdated = 4003,
    k_EMsgGCInviteToParty = 4501,
    k_EMsgGCInvitationCreated = 4502,
    k_EMsgGCPartyInviteResponse = 4503,
    k_EMsgGCKickFromParty = 4504,
    k_EMsgGCLeaveParty = 4505,
    k_EMsgGCServerAvailable = 4506,
    k_EMsgGCClientConnectToServer = 4507,
    k_EMsgGCGameServerInfo = 4508,
    k_EMsgGCError = 4509,
    k_EMsgGCLANServerAvailable = 4511,
    k_EMsgGCInviteToLobby = 4512,
    k_EMsgGCLobbyInviteResponse = 4513,
    k_EMsgGCToClientPollFileRequest = 4514,
    k_EMsgGCToClientPollFileResponse = 4515,
}

impl ::protobuf::ProtobufEnum for EGCBaseMsg {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EGCBaseMsg> {
        match value {
            4001 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCSystemMessage),
            4002 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCReplicateConVars),
            4003 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCConVarUpdated),
            4501 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCInviteToParty),
            4502 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCInvitationCreated),
            4503 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCPartyInviteResponse),
            4504 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCKickFromParty),
            4505 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCLeaveParty),
            4506 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCServerAvailable),
            4507 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCClientConnectToServer),
            4508 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCGameServerInfo),
            4509 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCError),
            4511 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCLANServerAvailable),
            4512 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCInviteToLobby),
            4513 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCLobbyInviteResponse),
            4514 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCToClientPollFileRequest),
            4515 => ::std::option::Option::Some(EGCBaseMsg::k_EMsgGCToClientPollFileResponse),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EGCBaseMsg] = &[
            EGCBaseMsg::k_EMsgGCSystemMessage,
            EGCBaseMsg::k_EMsgGCReplicateConVars,
            EGCBaseMsg::k_EMsgGCConVarUpdated,
            EGCBaseMsg::k_EMsgGCInviteToParty,
            EGCBaseMsg::k_EMsgGCInvitationCreated,
            EGCBaseMsg::k_EMsgGCPartyInviteResponse,
            EGCBaseMsg::k_EMsgGCKickFromParty,
            EGCBaseMsg::k_EMsgGCLeaveParty,
            EGCBaseMsg::k_EMsgGCServerAvailable,
            EGCBaseMsg::k_EMsgGCClientConnectToServer,
            EGCBaseMsg::k_EMsgGCGameServerInfo,
            EGCBaseMsg::k_EMsgGCError,
            EGCBaseMsg::k_EMsgGCLANServerAvailable,
            EGCBaseMsg::k_EMsgGCInviteToLobby,
            EGCBaseMsg::k_EMsgGCLobbyInviteResponse,
            EGCBaseMsg::k_EMsgGCToClientPollFileRequest,
            EGCBaseMsg::k_EMsgGCToClientPollFileResponse,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EGCBaseMsg>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EGCBaseMsg", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EGCBaseMsg {
}

impl ::protobuf::reflect::ProtobufValue for EGCBaseMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EGCBaseProtoObjectTypes {
    k_EProtoObjectPartyInvite = 1001,
    k_EProtoObjectLobbyInvite = 1002,
}

impl ::protobuf::ProtobufEnum for EGCBaseProtoObjectTypes {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EGCBaseProtoObjectTypes> {
        match value {
            1001 => ::std::option::Option::Some(EGCBaseProtoObjectTypes::k_EProtoObjectPartyInvite),
            1002 => ::std::option::Option::Some(EGCBaseProtoObjectTypes::k_EProtoObjectLobbyInvite),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EGCBaseProtoObjectTypes] = &[
            EGCBaseProtoObjectTypes::k_EProtoObjectPartyInvite,
            EGCBaseProtoObjectTypes::k_EProtoObjectLobbyInvite,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EGCBaseProtoObjectTypes>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EGCBaseProtoObjectTypes", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EGCBaseProtoObjectTypes {
}

impl ::protobuf::reflect::ProtobufValue for EGCBaseProtoObjectTypes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ECustomGameInstallStatus {
    k_ECustomGameInstallStatus_Unknown = 0,
    k_ECustomGameInstallStatus_Ready = 1,
    k_ECustomGameInstallStatus_Busy = 2,
    k_ECustomGameInstallStatus_FailedGeneric = 101,
    k_ECustomGameInstallStatus_FailedInternalError = 102,
    k_ECustomGameInstallStatus_RequestedTimestampTooOld = 103,
    k_ECustomGameInstallStatus_RequestedTimestampTooNew = 104,
    k_ECustomGameInstallStatus_CRCMismatch = 105,
    k_ECustomGameInstallStatus_FailedSteam = 106,
    k_ECustomGameInstallStatus_FailedCanceled = 107,
}

impl ::protobuf::ProtobufEnum for ECustomGameInstallStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ECustomGameInstallStatus> {
        match value {
            0 => ::std::option::Option::Some(ECustomGameInstallStatus::k_ECustomGameInstallStatus_Unknown),
            1 => ::std::option::Option::Some(ECustomGameInstallStatus::k_ECustomGameInstallStatus_Ready),
            2 => ::std::option::Option::Some(ECustomGameInstallStatus::k_ECustomGameInstallStatus_Busy),
            101 => ::std::option::Option::Some(ECustomGameInstallStatus::k_ECustomGameInstallStatus_FailedGeneric),
            102 => ::std::option::Option::Some(ECustomGameInstallStatus::k_ECustomGameInstallStatus_FailedInternalError),
            103 => ::std::option::Option::Some(ECustomGameInstallStatus::k_ECustomGameInstallStatus_RequestedTimestampTooOld),
            104 => ::std::option::Option::Some(ECustomGameInstallStatus::k_ECustomGameInstallStatus_RequestedTimestampTooNew),
            105 => ::std::option::Option::Some(ECustomGameInstallStatus::k_ECustomGameInstallStatus_CRCMismatch),
            106 => ::std::option::Option::Some(ECustomGameInstallStatus::k_ECustomGameInstallStatus_FailedSteam),
            107 => ::std::option::Option::Some(ECustomGameInstallStatus::k_ECustomGameInstallStatus_FailedCanceled),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ECustomGameInstallStatus] = &[
            ECustomGameInstallStatus::k_ECustomGameInstallStatus_Unknown,
            ECustomGameInstallStatus::k_ECustomGameInstallStatus_Ready,
            ECustomGameInstallStatus::k_ECustomGameInstallStatus_Busy,
            ECustomGameInstallStatus::k_ECustomGameInstallStatus_FailedGeneric,
            ECustomGameInstallStatus::k_ECustomGameInstallStatus_FailedInternalError,
            ECustomGameInstallStatus::k_ECustomGameInstallStatus_RequestedTimestampTooOld,
            ECustomGameInstallStatus::k_ECustomGameInstallStatus_RequestedTimestampTooNew,
            ECustomGameInstallStatus::k_ECustomGameInstallStatus_CRCMismatch,
            ECustomGameInstallStatus::k_ECustomGameInstallStatus_FailedSteam,
            ECustomGameInstallStatus::k_ECustomGameInstallStatus_FailedCanceled,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ECustomGameInstallStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ECustomGameInstallStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ECustomGameInstallStatus {
}

impl ::protobuf::reflect::ProtobufValue for ECustomGameInstallStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum GC_BannedWordType {
    GC_BANNED_WORD_DISABLE_WORD = 0,
    GC_BANNED_WORD_ENABLE_WORD = 1,
}

impl ::protobuf::ProtobufEnum for GC_BannedWordType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GC_BannedWordType> {
        match value {
            0 => ::std::option::Option::Some(GC_BannedWordType::GC_BANNED_WORD_DISABLE_WORD),
            1 => ::std::option::Option::Some(GC_BannedWordType::GC_BANNED_WORD_ENABLE_WORD),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [GC_BannedWordType] = &[
            GC_BannedWordType::GC_BANNED_WORD_DISABLE_WORD,
            GC_BannedWordType::GC_BANNED_WORD_ENABLE_WORD,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<GC_BannedWordType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("GC_BannedWordType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for GC_BannedWordType {
}

impl ::protobuf::reflect::ProtobufValue for GC_BannedWordType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15base_gcmessages.proto\x1a\x13steammessages.proto\"\xe5\x01\n\x1dCG\
    CStorePurchaseInit_LineItem\x12\x1e\n\x0bitem_def_id\x18\x01\x20\x01(\rR\
    \titemDefId\x12\x1a\n\x08quantity\x18\x02\x20\x01(\rR\x08quantity\x123\n\
    \x16cost_in_local_currency\x18\x03\x20\x01(\rR\x13costInLocalCurrency\
    \x12#\n\rpurchase_type\x18\x04\x20\x01(\rR\x0cpurchaseType\x12.\n\x13sou\
    rce_reference_id\x18\x05\x20\x01(\x04R\x11sourceReferenceId\"\xaa\x01\n\
    \x17CMsgGCStorePurchaseInit\x12\x18\n\x07country\x18\x01\x20\x01(\tR\x07\
    country\x12\x1a\n\x08language\x18\x02\x20\x01(\x05R\x08language\x12\x1a\
    \n\x08currency\x18\x03\x20\x01(\x05R\x08currency\x12=\n\nline_items\x18\
    \x04\x20\x03(\x0b2\x1e.CGCStorePurchaseInit_LineItemR\tlineItems\"P\n\
    \x1fCMsgGCStorePurchaseInitResponse\x12\x16\n\x06result\x18\x01\x20\x01(\
    \x05R\x06result\x12\x15\n\x06txn_id\x18\x02\x20\x01(\x04R\x05txnId\"/\n\
    \x13CMsgSystemBroadcast\x12\x18\n\x07message\x18\x01\x20\x01(\tR\x07mess\
    age\"\xe9\x01\n\x12CMsgClientPingData\x12#\n\x0brelay_codes\x18\x04\x20\
    \x03(\x07R\nrelayCodesB\x02\x10\x01\x12#\n\x0brelay_pings\x18\x05\x20\
    \x03(\rR\nrelayPingsB\x02\x10\x01\x12%\n\x0cregion_codes\x18\x08\x20\x03\
    (\rR\x0bregionCodesB\x02\x10\x01\x12%\n\x0cregion_pings\x18\t\x20\x03(\r\
    R\x0bregionPingsB\x02\x10\x01\x12;\n\x1aregion_ping_failed_bitmask\x18\n\
    \x20\x01(\rR\x17regionPingFailedBitmask\"\xbb\x01\n\x11CMsgInviteToParty\
    \x12\x19\n\x08steam_id\x18\x01\x20\x01(\x06R\x07steamId\x12%\n\x0eclient\
    _version\x18\x02\x20\x01(\rR\rclientVersion\x12\x17\n\x07team_id\x18\x03\
    \x20\x01(\rR\x06teamId\x12\x19\n\x08as_coach\x18\x04\x20\x01(\x08R\x07as\
    Coach\x120\n\tping_data\x18\x05\x20\x01(\x0b2\x13.CMsgClientPingDataR\
    \x08pingData\"U\n\x11CMsgInviteToLobby\x12\x19\n\x08steam_id\x18\x01\x20\
    \x01(\x06R\x07steamId\x12%\n\x0eclient_version\x18\x02\x20\x01(\rR\rclie\
    ntVersion\"p\n\x15CMsgInvitationCreated\x12\x19\n\x08group_id\x18\x01\
    \x20\x01(\x04R\x07groupId\x12\x19\n\x08steam_id\x18\x02\x20\x01(\x06R\
    \x07steamId\x12!\n\x0cuser_offline\x18\x03\x20\x01(\x08R\x0buserOffline\
    \"\xa5\x01\n\x17CMsgPartyInviteResponse\x12\x19\n\x08party_id\x18\x01\
    \x20\x01(\x04R\x07partyId\x12\x16\n\x06accept\x18\x02\x20\x01(\x08R\x06a\
    ccept\x12%\n\x0eclient_version\x18\x03\x20\x01(\rR\rclientVersion\x120\n\
    \tping_data\x18\x08\x20\x01(\x0b2\x13.CMsgClientPingDataR\x08pingData\"\
    \xcf\x01\n\x17CMsgLobbyInviteResponse\x12\x19\n\x08lobby_id\x18\x01\x20\
    \x01(\x06R\x07lobbyId\x12\x16\n\x06accept\x18\x02\x20\x01(\x08R\x06accep\
    t\x12%\n\x0eclient_version\x18\x03\x20\x01(\rR\rclientVersion\x12&\n\x0f\
    custom_game_crc\x18\x06\x20\x01(\x06R\rcustomGameCrc\x122\n\x15custom_ga\
    me_timestamp\x18\x07\x20\x01(\x07R\x13customGameTimestamp\".\n\x11CMsgKi\
    ckFromParty\x12\x19\n\x08steam_id\x18\x01\x20\x01(\x06R\x07steamId\"\x10\
    \n\x0eCMsgLeaveParty\"\xcd\x01\n\x1bCMsgCustomGameInstallStatus\x12U\n\
    \x06status\x18\x01\x20\x01(\x0e2\x19.ECustomGameInstallStatus:\"k_ECusto\
    mGameInstallStatus_UnknownR\x06status\x12\x18\n\x07message\x18\x02\x20\
    \x01(\tR\x07message\x12=\n\x1blatest_timestamp_from_steam\x18\x03\x20\
    \x01(\x07R\x18latestTimestampFromSteam\"p\n\x13CMsgServerAvailable\x12Y\
    \n\x1acustom_game_install_status\x18\x01\x20\x01(\x0b2\x1c.CMsgCustomGam\
    eInstallStatusR\x17customGameInstallStatus\"3\n\x16CMsgLANServerAvailabl\
    e\x12\x19\n\x08lobby_id\x18\x01\x20\x01(\x06R\x07lobbyId\"\xd0\x03\n\x18\
    CSOEconGameAccountClient\x12=\n\x19additional_backpack_slots\x18\x01\x20\
    \x01(\r:\x010R\x17additionalBackpackSlots\x12*\n\rtrial_account\x18\x02\
    \x20\x01(\x08:\x05falseR\x0ctrialAccount\x12=\n\x18eligible_for_online_p\
    lay\x18\x03\x20\x01(\x08:\x04trueR\x15eligibleForOnlinePlay\x12I\n\"need\
    _to_choose_most_helpful_friend\x18\x04\x20\x01(\x08R\x1dneedToChooseMost\
    HelpfulFriend\x12&\n\x0fin_coaches_list\x18\x05\x20\x01(\x08R\rinCoaches\
    List\x120\n\x14trade_ban_expiration\x18\x06\x20\x01(\x07R\x12tradeBanExp\
    iration\x12.\n\x13duel_ban_expiration\x18\x07\x20\x01(\x07R\x11duelBanEx\
    piration\x125\n\x13made_first_purchase\x18\t\x20\x01(\x08:\x05falseR\x11\
    madeFirstPurchase\"\xa0\x01\n\x18CSOItemCriteriaCondition\x12\x0e\n\x02o\
    p\x18\x01\x20\x01(\x05R\x02op\x12\x14\n\x05field\x18\x02\x20\x01(\tR\x05\
    field\x12\x1a\n\x08required\x18\x03\x20\x01(\x08R\x08required\x12\x1f\n\
    \x0bfloat_value\x18\x04\x20\x01(\x02R\nfloatValue\x12!\n\x0cstring_value\
    \x18\x05\x20\x01(\tR\x0bstringValue\"\x87\x03\n\x0fCSOItemCriteria\x12\
    \x1d\n\nitem_level\x18\x01\x20\x01(\rR\titemLevel\x12!\n\x0citem_quality\
    \x18\x02\x20\x01(\x05R\x0bitemQuality\x12$\n\x0eitem_level_set\x18\x03\
    \x20\x01(\x08R\x0citemLevelSet\x12(\n\x10item_quality_set\x18\x04\x20\
    \x01(\x08R\x0eitemQualitySet\x12+\n\x11initial_inventory\x18\x05\x20\x01\
    (\rR\x10initialInventory\x12)\n\x10initial_quantity\x18\x06\x20\x01(\rR\
    \x0finitialQuantity\x12.\n\x13ignore_enabled_flag\x18\x08\x20\x01(\x08R\
    \x11ignoreEnabledFlag\x129\n\nconditions\x18\t\x20\x03(\x0b2\x19.CSOItem\
    CriteriaConditionR\nconditions\x12\x1f\n\x0brecent_only\x18\n\x20\x01(\
    \x08R\nrecentOnly\"\xc0\x05\n\rCSOItemRecipe\x12\x1b\n\tdef_index\x18\
    \x01\x20\x01(\rR\x08defIndex\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04na\
    me\x12\x0f\n\x03n_a\x18\x03\x20\x01(\tR\x02nA\x12\x1f\n\x0bdesc_inputs\
    \x18\x04\x20\x01(\tR\ndescInputs\x12!\n\x0cdesc_outputs\x18\x05\x20\x01(\
    \tR\x0bdescOutputs\x12\x11\n\x04di_a\x18\x06\x20\x01(\tR\x03diA\x12\x11\
    \n\x04di_b\x18\x07\x20\x01(\tR\x03diB\x12\x11\n\x04di_c\x18\x08\x20\x01(\
    \tR\x03diC\x12\x11\n\x04do_a\x18\t\x20\x01(\tR\x03doA\x12\x11\n\x04do_b\
    \x18\n\x20\x01(\tR\x03doB\x12\x11\n\x04do_c\x18\x0b\x20\x01(\tR\x03doC\
    \x125\n\x17requires_all_same_class\x18\x0c\x20\x01(\x08R\x14requiresAllS\
    ameClass\x123\n\x16requires_all_same_slot\x18\r\x20\x01(\x08R\x13require\
    sAllSameSlot\x123\n\x16class_usage_for_output\x18\x0e\x20\x01(\x05R\x13c\
    lassUsageForOutput\x121\n\x15slot_usage_for_output\x18\x0f\x20\x01(\x05R\
    \x12slotUsageForOutput\x12$\n\x0eset_for_output\x18\x10\x20\x01(\x05R\
    \x0csetForOutput\x12B\n\x14input_items_criteria\x18\x14\x20\x03(\x0b2\
    \x10.CSOItemCriteriaR\x12inputItemsCriteria\x12D\n\x15output_items_crite\
    ria\x18\x15\x20\x03(\x0b2\x10.CSOItemCriteriaR\x13outputItemsCriteria\
    \x123\n\x16input_item_dupe_counts\x18\x16\x20\x03(\rR\x13inputItemDupeCo\
    unts\"i\n\x14CMsgApplyStrangePart\x12/\n\x14strange_part_item_id\x18\x01\
    \x20\x01(\x04R\x11strangePartItemId\x12\x20\n\x0citem_item_id\x18\x02\
    \x20\x01(\x04R\nitemItemId\"i\n\x17CMsgApplyPennantUpgrade\x12&\n\x0fupg\
    rade_item_id\x18\x01\x20\x01(\x04R\rupgradeItemId\x12&\n\x0fpennant_item\
    _id\x18\x02\x20\x01(\x04R\rpennantItemId\"]\n\x13CMsgApplyEggEssence\x12\
    &\n\x0fessence_item_id\x18\x01\x20\x01(\x04R\ressenceItemId\x12\x1e\n\
    \x0begg_item_id\x18\x02\x20\x01(\x04R\teggItemId\"j\n\x14CSOEconItemAttr\
    ibute\x12\x1b\n\tdef_index\x18\x01\x20\x01(\rR\x08defIndex\x12\x14\n\x05\
    value\x18\x02\x20\x01(\rR\x05value\x12\x1f\n\x0bvalue_bytes\x18\x03\x20\
    \x01(\x0cR\nvalueBytes\"M\n\x13CSOEconItemEquipped\x12\x1b\n\tnew_class\
    \x18\x01\x20\x01(\rR\x08newClass\x12\x19\n\x08new_slot\x18\x02\x20\x01(\
    \rR\x07newSlot\"\x80\x04\n\x0bCSOEconItem\x12\x0e\n\x02id\x18\x01\x20\
    \x01(\x04R\x02id\x12\x1d\n\naccount_id\x18\x02\x20\x01(\rR\taccountId\
    \x12\x1c\n\tinventory\x18\x03\x20\x01(\rR\tinventory\x12\x1b\n\tdef_inde\
    x\x18\x04\x20\x01(\rR\x08defIndex\x12\x1d\n\x08quantity\x18\x05\x20\x01(\
    \r:\x011R\x08quantity\x12\x17\n\x05level\x18\x06\x20\x01(\r:\x011R\x05le\
    vel\x12\x1b\n\x07quality\x18\x07\x20\x01(\r:\x014R\x07quality\x12\x17\n\
    \x05flags\x18\x08\x20\x01(\r:\x010R\x05flags\x12\x19\n\x06origin\x18\t\
    \x20\x01(\r:\x010R\x06origin\x123\n\tattribute\x18\x0c\x20\x03(\x0b2\x15\
    .CSOEconItemAttributeR\tattribute\x121\n\rinterior_item\x18\r\x20\x01(\
    \x0b2\x0c.CSOEconItemR\x0cinteriorItem\x12\x1c\n\x06in_use\x18\x0e\x20\
    \x01(\x08:\x05falseR\x05inUse\x12\x17\n\x05style\x18\x0f\x20\x01(\r:\x01\
    0R\x05style\x12\"\n\x0boriginal_id\x18\x10\x20\x01(\x04:\x010R\noriginal\
    Id\x12;\n\x0eequipped_state\x18\x12\x20\x03(\x0b2\x14.CSOEconItemEquippe\
    dR\requippedState\",\n\rCMsgSortItems\x12\x1b\n\tsort_type\x18\x01\x20\
    \x01(\rR\x08sortType\"\x87\x01\n\x10CSOEconClaimCode\x12\x1d\n\naccount_\
    id\x18\x01\x20\x01(\rR\taccountId\x12\x1b\n\tcode_type\x18\x02\x20\x01(\
    \rR\x08codeType\x12#\n\rtime_acquired\x18\x03\x20\x01(\rR\x0ctimeAcquire\
    d\x12\x12\n\x04code\x18\x04\x20\x01(\tR\x04code\"F\n\x14CMsgStoreGetUser\
    Data\x12.\n\x13price_sheet_version\x18\x01\x20\x01(\x07R\x11priceSheetVe\
    rsion\"\xa3\x03\n\x1cCMsgStoreGetUserDataResponse\x12\x16\n\x06result\
    \x18\x01\x20\x01(\x05R\x06result\x12\x1a\n\x08currency\x18\x02\x20\x01(\
    \x05R\x08currency\x12\x18\n\x07country\x18\x03\x20\x01(\tR\x07country\
    \x12.\n\x13price_sheet_version\x18\x04\x20\x01(\x07R\x11priceSheetVersio\
    n\x12*\n\x0fexperiment_data\x18\x05\x20\x01(\x04:\x010R\x0eexperimentDat\
    a\x12*\n\x11featured_item_idx\x18\x06\x20\x01(\x05R\x0ffeaturedItemIdx\
    \x128\n\x15show_hat_descriptions\x18\x07\x20\x01(\x08:\x04trueR\x13showH\
    atDescriptions\x12\x1f\n\x0bprice_sheet\x18\x08\x20\x01(\x0cR\npriceShee\
    t\x12-\n\x11default_item_sort\x18\t\x20\x01(\x05:\x010R\x0fdefaultItemSo\
    rt\x12#\n\rpopular_items\x18\n\x20\x03(\rR\x0cpopularItems\"\x8b\x01\n\
    \x14CMsgUpdateItemSchema\x12\x1d\n\nitems_game\x18\x01\x20\x01(\x0cR\tit\
    emsGame\x12.\n\x13item_schema_version\x18\x02\x20\x01(\x07R\x11itemSchem\
    aVersion\x12$\n\x0eitems_game_url\x18\x03\x20\x01(\tR\x0citemsGameUrl\",\
    \n\x0bCMsgGCError\x12\x1d\n\nerror_text\x18\x01\x20\x01(\tR\terrorText\"\
    \x1d\n\x1bCMsgRequestInventoryRefresh\";\n\x0fCMsgConVarValue\x12\x12\n\
    \x04name\x18\x01\x20\x01(\tR\x04name\x12\x14\n\x05value\x18\x02\x20\x01(\
    \tR\x05value\"B\n\x14CMsgReplicateConVars\x12*\n\x07convars\x18\x01\x20\
    \x03(\x0b2\x10.CMsgConVarValueR\x07convars\"9\n\x17CMsgConsumableExhaust\
    ed\x12\x1e\n\x0bitem_def_id\x18\x01\x20\x01(\x05R\titemDefId\"\xba\x01\n\
    \x14CMsgItemAcknowledged\x12\x1d\n\naccount_id\x18\x01\x20\x01(\rR\tacco\
    untId\x12\x1c\n\tinventory\x18\x02\x20\x01(\rR\tinventory\x12\x1b\n\tdef\
    _index\x18\x03\x20\x01(\rR\x08defIndex\x12\x18\n\x07quality\x18\x04\x20\
    \x01(\rR\x07quality\x12\x16\n\x06rarity\x18\x05\x20\x01(\rR\x06rarity\
    \x12\x16\n\x06origin\x18\x06\x20\x01(\rR\x06origin\"\xa6\x01\n\x14CMsgSe\
    tItemPositions\x12I\n\x0eitem_positions\x18\x01\x20\x03(\x0b2\".CMsgSetI\
    temPositions.ItemPositionR\ritemPositions\x1aC\n\x0cItemPosition\x12\x17\
    \n\x07item_id\x18\x01\x20\x01(\x04R\x06itemId\x12\x1a\n\x08position\x18\
    \x02\x20\x01(\rR\x08position\"\x93\x01\n\x1aCMsgGCNameItemNotification\
    \x12%\n\x0eplayer_steamid\x18\x01\x20\x01(\x06R\rplayerSteamid\x12$\n\
    \x0eitem_def_index\x18\x02\x20\x01(\rR\x0citemDefIndex\x12(\n\x10item_na\
    me_custom\x18\x03\x20\x01(\tR\x0eitemNameCustom\"\xa1\x02\n\x1fCMsgGCCli\
    entDisplayNotification\x12M\n#notification_title_localization_key\x18\
    \x01\x20\x01(\tR\x20notificationTitleLocalizationKey\x12K\n\"notificatio\
    n_body_localization_key\x18\x02\x20\x01(\tR\x1fnotificationBodyLocalizat\
    ionKey\x12.\n\x13body_substring_keys\x18\x03\x20\x03(\tR\x11bodySubstrin\
    gKeys\x122\n\x15body_substring_values\x18\x04\x20\x03(\tR\x13bodySubstri\
    ngValues\"@\n\x17CMsgGCShowItemsPickedUp\x12%\n\x0eplayer_steamid\x18\
    \x01\x20\x01(\x06R\rplayerSteamid\"\xab\x01\n\x20CMsgGCIncrementKillCoun\
    tResponse\x120\n\x11killer_account_id\x18\x01\x20\x01(\rR\x0fkillerAccou\
    ntIdB\x04\x80\xa6\x1d\x01\x12\x1b\n\tnum_kills\x18\x02\x20\x01(\rR\x08nu\
    mKills\x12\x19\n\x08item_def\x18\x03\x20\x01(\rR\x07itemDef\x12\x1d\n\nl\
    evel_type\x18\x04\x20\x01(\rR\tlevelType\"\xa7\x02\n\x18CSOEconItemDropR\
    ateBonus\x12#\n\naccount_id\x18\x01\x20\x01(\rR\taccountIdB\x04\x80\xa6\
    \x1d\x01\x12'\n\x0fexpiration_date\x18\x02\x20\x01(\x07R\x0eexpirationDa\
    te\x12\x1a\n\x05bonus\x18\x03\x20\x01(\x02R\x05bonusB\x04\x80\xa6\x1d\
    \x01\x12\x1f\n\x0bbonus_count\x18\x04\x20\x01(\rR\nbonusCount\x12\x17\n\
    \x07item_id\x18\x05\x20\x01(\x04R\x06itemId\x12\x1b\n\tdef_index\x18\x06\
    \x20\x01(\rR\x08defIndex\x12!\n\x0cseconds_left\x18\x07\x20\x01(\rR\x0bs\
    econdsLeft\x12'\n\x0cbooster_type\x18\x08\x20\x01(\rR\x0bboosterTypeB\
    \x04\x80\xa6\x1d\x01\"\xa4\x01\n\x19CSOEconItemLeagueViewPass\x12#\n\nac\
    count_id\x18\x01\x20\x01(\rR\taccountIdB\x04\x80\xa6\x1d\x01\x12!\n\tlea\
    gue_id\x18\x02\x20\x01(\rR\x08leagueIdB\x04\x80\xa6\x1d\x01\x12\x1c\n\ti\
    temindex\x18\x04\x20\x01(\rR\titemindex\x12!\n\x0cgrant_reason\x18\x05\
    \x20\x01(\rR\x0bgrantReason\"k\n\x16CSOEconItemEventTicket\x12\x1d\n\nac\
    count_id\x18\x01\x20\x01(\rR\taccountId\x12\x19\n\x08event_id\x18\x02\
    \x20\x01(\rR\x07eventId\x12\x17\n\x07item_id\x18\x03\x20\x01(\x04R\x06it\
    emId\"\xad\x02\n\x1dCSOEconItemTournamentPassport\x12\x1d\n\naccount_id\
    \x18\x01\x20\x01(\rR\taccountId\x12\x1b\n\tleague_id\x18\x02\x20\x01(\rR\
    \x08leagueId\x12\x17\n\x07item_id\x18\x03\x20\x01(\x04R\x06itemId\x122\n\
    \x15original_purchaser_id\x18\x04\x20\x01(\rR\x13originalPurchaserId\x12\
    )\n\x10passports_bought\x18\x05\x20\x01(\rR\x0fpassportsBought\x12\x18\n\
    \x07version\x18\x06\x20\x01(\rR\x07version\x12\x1b\n\tdef_index\x18\x07\
    \x20\x01(\rR\x08defIndex\x12!\n\x0creward_flags\x18\x08\x20\x01(\rR\x0br\
    ewardFlags\"2\n\x19CMsgGCStorePurchaseCancel\x12\x15\n\x06txn_id\x18\x01\
    \x20\x01(\x04R\x05txnId\";\n!CMsgGCStorePurchaseCancelResponse\x12\x16\n\
    \x06result\x18\x01\x20\x01(\rR\x06result\"4\n\x1bCMsgGCStorePurchaseFina\
    lize\x12\x15\n\x06txn_id\x18\x01\x20\x01(\x04R\x05txnId\"X\n#CMsgGCStore\
    PurchaseFinalizeResponse\x12\x16\n\x06result\x18\x01\x20\x01(\rR\x06resu\
    lt\x12\x19\n\x08item_ids\x18\x02\x20\x03(\x04R\x07itemIds\"a\n\x1bCMsgGC\
    BannedWordListRequest\x12)\n\x11ban_list_group_id\x18\x01\x20\x01(\rR\
    \x0ebanListGroupId\x12\x17\n\x07word_id\x18\x02\x20\x01(\rR\x06wordId\"\
    \x8d\x01\n\x10CMsgGCBannedWord\x12\x17\n\x07word_id\x18\x01\x20\x01(\rR\
    \x06wordId\x12L\n\tword_type\x18\x02\x20\x01(\x0e2\x12.GC_BannedWordType\
    :\x1bGC_BANNED_WORD_DISABLE_WORDR\x08wordType\x12\x12\n\x04word\x18\x03\
    \x20\x01(\tR\x04word\"y\n\x1cCMsgGCBannedWordListResponse\x12)\n\x11ban_\
    list_group_id\x18\x01\x20\x01(\rR\x0ebanListGroupId\x12.\n\tword_list\
    \x18\x02\x20\x03(\x0b2\x11.CMsgGCBannedWordR\x08wordList\"`\n!CMsgGCToGC\
    BannedWordListBroadcast\x12;\n\tbroadcast\x18\x01\x20\x01(\x0b2\x1d.CMsg\
    GCBannedWordListResponseR\tbroadcast\"<\n\x1fCMsgGCToGCBannedWordListUpd\
    ated\x12\x19\n\x08group_id\x18\x01\x20\x01(\rR\x07groupId\"S\n\x17CMsgGC\
    ToGCDirtySDOCache\x12\x19\n\x08sdo_type\x18\x01\x20\x01(\rR\x07sdoType\
    \x12\x1d\n\nkey_uint64\x18\x02\x20\x01(\x04R\tkeyUint64\"[\n\x1fCMsgGCTo\
    GCDirtyMultipleSDOCache\x12\x19\n\x08sdo_type\x18\x01\x20\x01(\rR\x07sdo\
    Type\x12\x1d\n\nkey_uint64\x18\x02\x20\x03(\x04R\tkeyUint64\"^\n\x1fCMsg\
    GCToGCApplyLocalizationDiff\x12\x1a\n\x08language\x18\x01\x20\x01(\rR\
    \x08language\x12\x1f\n\x0bpacked_diff\x18\x02\x20\x01(\tR\npackedDiff\"C\
    \n'CMsgGCToGCApplyLocalizationDiffResponse\x12\x18\n\x07success\x18\x01\
    \x20\x01(\x08R\x07success\"i\n\x11CMsgGCCollectItem\x12,\n\x12collection\
    _item_id\x18\x01\x20\x01(\x04R\x10collectionItemId\x12&\n\x0fsubject_ite\
    m_id\x18\x02\x20\x01(\x04R\rsubjectItemId\"\x14\n\x12CMsgSDONoMemcached\
    \"8\n\x1bCMsgGCToGCUpdateSQLKeyValue\x12\x19\n\x08key_name\x18\x01\x20\
    \x01(\tR\x07keyName\"C\n\x1aCMsgGCServerVersionUpdated\x12%\n\x0eserver_\
    version\x18\x01\x20\x01(\rR\rserverVersion\"C\n\x1aCMsgGCClientVersionUp\
    dated\x12%\n\x0eclient_version\x18\x01\x20\x01(\rR\rclientVersion\"\x20\
    \n\x1eCMsgGCToGCWebAPIAccountChanged\"f\n\x13CMsgRecipeComponent\x12&\n\
    \x0fsubject_item_id\x18\x01\x20\x01(\x04R\rsubjectItemId\x12'\n\x0fattri\
    bute_index\x18\x02\x20\x01(\x04R\x0eattributeIndex\"\x92\x01\n!CMsgFulfi\
    llDynamicRecipeComponent\x12\x20\n\x0ctool_item_id\x18\x01\x20\x01(\x04R\
    \ntoolItemId\x12K\n\x16consumption_components\x18\x02\x20\x03(\x0b2\x14.\
    CMsgRecipeComponentR\x15consumptionComponents\"D\n\x1dCMsgGCClientMarket\
    DataRequest\x12#\n\ruser_currency\x18\x01\x20\x01(\rR\x0cuserCurrency\"\
    \xcb\x01\n\x1bCMsgGCClientMarketDataEntry\x12$\n\x0eitem_def_index\x18\
    \x01\x20\x01(\rR\x0citemDefIndex\x12!\n\x0citem_quality\x18\x02\x20\x01(\
    \rR\x0bitemQuality\x12,\n\x12item_sell_listings\x18\x03\x20\x01(\rR\x10i\
    temSellListings\x125\n\x17price_in_local_currency\x18\x04\x20\x01(\rR\
    \x14priceInLocalCurrency\"P\n\x16CMsgGCClientMarketData\x126\n\x07entrie\
    s\x18\x01\x20\x03(\x0b2\x1c.CMsgGCClientMarketDataEntryR\x07entries\"\
    \x82\x01\n\x0fCMsgExtractGems\x12\x20\n\x0ctool_item_id\x18\x01\x20\x01(\
    \x04R\ntoolItemId\x12\x20\n\x0citem_item_id\x18\x02\x20\x01(\x04R\nitemI\
    temId\x12+\n\x0eitem_socket_id\x18\x03\x20\x01(\r:\x0565535R\x0citemSock\
    etId\"\xe6\x02\n\x17CMsgExtractGemsResponse\x12\x17\n\x07item_id\x18\x01\
    \x20\x01(\x04R\x06itemId\x12Z\n\x08response\x18\x02\x20\x01(\x0e2%.CMsgE\
    xtractGemsResponse.EExtractGems:\x17k_ExtractGems_SucceededR\x08response\
    \"\xd5\x01\n\x0cEExtractGems\x12\x1b\n\x17k_ExtractGems_Succeeded\x10\0\
    \x12&\n\"k_ExtractGems_Failed_ToolIsInvalid\x10\x01\x12&\n\"k_ExtractGem\
    s_Failed_ItemIsInvalid\x10\x02\x12,\n(k_ExtractGems_Failed_ToolCannotRem\
    oveGem\x10\x03\x12*\n&k_ExtractGems_Failed_FailedToRemoveGem\x10\x04\"m\
    \n\rCMsgAddSocket\x12\x20\n\x0ctool_item_id\x18\x01\x20\x01(\x04R\ntoolI\
    temId\x12\x20\n\x0citem_item_id\x18\x02\x20\x01(\x04R\nitemItemId\x12\
    \x18\n\x07unusual\x18\x03\x20\x01(\x08R\x07unusual\"\xdf\x02\n\x15CMsgAd\
    dSocketResponse\x12\x17\n\x07item_id\x18\x01\x20\x01(\x04R\x06itemId\x12\
    0\n\x14updated_socket_index\x18\x02\x20\x03(\rR\x12updatedSocketIndex\
    \x12T\n\x08response\x18\x03\x20\x01(\x0e2!.CMsgAddSocketResponse.EAddSoc\
    ket:\x15k_AddSocket_SucceededR\x08response\"\xa4\x01\n\nEAddSocket\x12\
    \x19\n\x15k_AddSocket_Succeeded\x10\0\x12$\n\x20k_AddSocket_Failed_ToolI\
    sInvalid\x10\x01\x12+\n'k_AddSocket_Failed_ItemCannotBeSocketed\x10\x02\
    \x12(\n$k_AddSocket_Failed_FailedToAddSocket\x10\x03\"\\\n\x17CMsgAddIte\
    mToSocketData\x12\x1e\n\x0bgem_item_id\x18\x01\x20\x01(\x04R\tgemItemId\
    \x12!\n\x0csocket_index\x18\x02\x20\x01(\rR\x0bsocketIndex\"w\n\x13CMsgA\
    ddItemToSocket\x12\x20\n\x0citem_item_id\x18\x01\x20\x01(\x04R\nitemItem\
    Id\x12>\n\x0egems_to_socket\x18\x02\x20\x03(\x0b2\x18.CMsgAddItemToSocke\
    tDataR\x0cgemsToSocket\"\x89\x04\n\x1bCMsgAddItemToSocketResponse\x12\
    \x20\n\x0citem_item_id\x18\x01\x20\x01(\x04R\nitemItemId\x120\n\x14updat\
    ed_socket_index\x18\x02\x20\x03(\rR\x12updatedSocketIndex\x12T\n\x08resp\
    onse\x18\x03\x20\x01(\x0e2$.CMsgAddItemToSocketResponse.EAddGem:\x12k_Ad\
    dGem_SucceededR\x08response\"\xbf\x02\n\x07EAddGem\x12\x16\n\x12k_AddGem\
    _Succeeded\x10\0\x12\x20\n\x1ck_AddGem_Failed_GemIsInvalid\x10\x01\x12!\
    \n\x1dk_AddGem_Failed_ItemIsInvalid\x10\x02\x12\"\n\x1ek_AddGem_Failed_F\
    ailedToAddGem\x10\x03\x12+\n'k_AddGem_Failed_InvalidGemTypeForSocket\x10\
    \x04\x12)\n%k_AddGem_Failed_InvalidGemTypeForHero\x10\x05\x12)\n%k_AddGe\
    m_Failed_InvalidGemTypeForSlot\x10\x06\x120\n,k_AddGem_Failed_SocketCont\
    ainsUnremovableGem\x10\x07\"_\n\x18CMsgResetStrangeGemCount\x12\x20\n\
    \x0citem_item_id\x18\x01\x20\x01(\x04R\nitemItemId\x12!\n\x0csocket_inde\
    x\x18\x02\x20\x01(\rR\x0bsocketIndex\"\xc8\x02\n\x20CMsgResetStrangeGemC\
    ountResponse\x12]\n\x08response\x18\x01\x20\x01(\x0e2+.CMsgResetStrangeG\
    emCountResponse.EResetGem:\x14k_ResetGem_SucceededR\x08response\"\xc4\
    \x01\n\tEResetGem\x12\x18\n\x14k_ResetGem_Succeeded\x10\0\x12&\n\"k_Rese\
    tGem_Failed_FailedToResetGem\x10\x01\x12#\n\x1fk_ResetGem_Failed_ItemIsI\
    nvalid\x10\x02\x12%\n!k_ResetGem_Failed_InvalidSocketId\x10\x03\x12)\n%k\
    _ResetGem_Failed_SocketCannotBeReset\x10\x04\"|\n\x1dCMsgGCToClientPollF\
    ileRequest\x12\x1b\n\tfile_name\x18\x01\x20\x01(\tR\x08fileName\x12%\n\
    \x0eclient_version\x18\x02\x20\x01(\rR\rclientVersion\x12\x17\n\x07poll_\
    id\x18\x03\x20\x01(\rR\x06pollId\"V\n\x1eCMsgGCToClientPollFileResponse\
    \x12\x17\n\x07poll_id\x18\x01\x20\x01(\rR\x06pollId\x12\x1b\n\tfile_size\
    \x18\x02\x20\x01(\rR\x08fileSize*\x95\x04\n\nEGCBaseMsg\x12\x1a\n\x15k_E\
    MsgGCSystemMessage\x10\xa1\x1f\x12\x1d\n\x18k_EMsgGCReplicateConVars\x10\
    \xa2\x1f\x12\x1a\n\x15k_EMsgGCConVarUpdated\x10\xa3\x1f\x12\x1a\n\x15k_E\
    MsgGCInviteToParty\x10\x95#\x12\x1e\n\x19k_EMsgGCInvitationCreated\x10\
    \x96#\x12\x20\n\x1bk_EMsgGCPartyInviteResponse\x10\x97#\x12\x1a\n\x15k_E\
    MsgGCKickFromParty\x10\x98#\x12\x17\n\x12k_EMsgGCLeaveParty\x10\x99#\x12\
    \x1c\n\x17k_EMsgGCServerAvailable\x10\x9a#\x12\"\n\x1dk_EMsgGCClientConn\
    ectToServer\x10\x9b#\x12\x1b\n\x16k_EMsgGCGameServerInfo\x10\x9c#\x12\
    \x12\n\rk_EMsgGCError\x10\x9d#\x12\x1f\n\x1ak_EMsgGCLANServerAvailable\
    \x10\x9f#\x12\x1a\n\x15k_EMsgGCInviteToLobby\x10\xa0#\x12\x20\n\x1bk_EMs\
    gGCLobbyInviteResponse\x10\xa1#\x12$\n\x1fk_EMsgGCToClientPollFileReques\
    t\x10\xa2#\x12%\n\x20k_EMsgGCToClientPollFileResponse\x10\xa3#*Y\n\x17EG\
    CBaseProtoObjectTypes\x12\x1e\n\x19k_EProtoObjectPartyInvite\x10\xe9\x07\
    \x12\x1e\n\x19k_EProtoObjectLobbyInvite\x10\xea\x07*\xe8\x03\n\x18ECusto\
    mGameInstallStatus\x12&\n\"k_ECustomGameInstallStatus_Unknown\x10\0\x12$\
    \n\x20k_ECustomGameInstallStatus_Ready\x10\x01\x12#\n\x1fk_ECustomGameIn\
    stallStatus_Busy\x10\x02\x12,\n(k_ECustomGameInstallStatus_FailedGeneric\
    \x10e\x122\n.k_ECustomGameInstallStatus_FailedInternalError\x10f\x127\n3\
    k_ECustomGameInstallStatus_RequestedTimestampTooOld\x10g\x127\n3k_ECusto\
    mGameInstallStatus_RequestedTimestampTooNew\x10h\x12*\n&k_ECustomGameIns\
    tallStatus_CRCMismatch\x10i\x12*\n&k_ECustomGameInstallStatus_FailedStea\
    m\x10j\x12-\n)k_ECustomGameInstallStatus_FailedCanceled\x10k*T\n\x11GC_B\
    annedWordType\x12\x1f\n\x1bGC_BANNED_WORD_DISABLE_WORD\x10\0\x12\x1e\n\
    \x1aGC_BANNED_WORD_ENABLE_WORD\x10\x01B\x05H\x01\x80\x01\0\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
