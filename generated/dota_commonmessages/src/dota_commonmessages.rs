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
pub struct CDOTAMsg_LocationPing {
    // message fields
    x: ::std::option::Option<i32>,
    y: ::std::option::Option<i32>,
    target: ::std::option::Option<i32>,
    direct_ping: ::std::option::Option<bool>,
    field_type: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMsg_LocationPing {}

impl CDOTAMsg_LocationPing {
    pub fn new() -> CDOTAMsg_LocationPing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMsg_LocationPing {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMsg_LocationPing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMsg_LocationPing,
        };
        unsafe {
            instance.get(CDOTAMsg_LocationPing::new)
        }
    }

    // optional int32 x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: i32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> i32 {
        self.x.unwrap_or(0)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.x
    }

    // optional int32 y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: i32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> i32 {
        self.y.unwrap_or(0)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.y
    }

    // optional int32 target = 3;

    pub fn clear_target(&mut self) {
        self.target = ::std::option::Option::None;
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: i32) {
        self.target = ::std::option::Option::Some(v);
    }

    pub fn get_target(&self) -> i32 {
        self.target.unwrap_or(0)
    }

    fn get_target_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.target
    }

    fn mut_target_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.target
    }

    // optional bool direct_ping = 4;

    pub fn clear_direct_ping(&mut self) {
        self.direct_ping = ::std::option::Option::None;
    }

    pub fn has_direct_ping(&self) -> bool {
        self.direct_ping.is_some()
    }

    // Param is passed by value, moved
    pub fn set_direct_ping(&mut self, v: bool) {
        self.direct_ping = ::std::option::Option::Some(v);
    }

    pub fn get_direct_ping(&self) -> bool {
        self.direct_ping.unwrap_or(false)
    }

    fn get_direct_ping_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.direct_ping
    }

    fn mut_direct_ping_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.direct_ping
    }

    // optional int32 type = 5;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: i32) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> i32 {
        self.field_type.unwrap_or(0)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.field_type
    }
}

impl ::protobuf::Message for CDOTAMsg_LocationPing {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.target = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.direct_ping = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.field_type = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.x {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.y {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.target {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.direct_ping {
            my_size += 2;
        }
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.target {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.direct_ping {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.field_type {
            os.write_int32(5, v)?;
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

impl ::protobuf::MessageStatic for CDOTAMsg_LocationPing {
    fn new() -> CDOTAMsg_LocationPing {
        CDOTAMsg_LocationPing::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMsg_LocationPing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "x",
                    CDOTAMsg_LocationPing::get_x_for_reflect,
                    CDOTAMsg_LocationPing::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "y",
                    CDOTAMsg_LocationPing::get_y_for_reflect,
                    CDOTAMsg_LocationPing::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "target",
                    CDOTAMsg_LocationPing::get_target_for_reflect,
                    CDOTAMsg_LocationPing::mut_target_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "direct_ping",
                    CDOTAMsg_LocationPing::get_direct_ping_for_reflect,
                    CDOTAMsg_LocationPing::mut_direct_ping_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "type",
                    CDOTAMsg_LocationPing::get_field_type_for_reflect,
                    CDOTAMsg_LocationPing::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMsg_LocationPing>(
                    "CDOTAMsg_LocationPing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMsg_LocationPing {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_target();
        self.clear_direct_ping();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMsg_LocationPing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMsg_LocationPing {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMsg_ItemAlert {
    // message fields
    x: ::std::option::Option<i32>,
    y: ::std::option::Option<i32>,
    itemid: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMsg_ItemAlert {}

impl CDOTAMsg_ItemAlert {
    pub fn new() -> CDOTAMsg_ItemAlert {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMsg_ItemAlert {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMsg_ItemAlert> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMsg_ItemAlert,
        };
        unsafe {
            instance.get(CDOTAMsg_ItemAlert::new)
        }
    }

    // optional int32 x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: i32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> i32 {
        self.x.unwrap_or(0)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.x
    }

    // optional int32 y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: i32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> i32 {
        self.y.unwrap_or(0)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.y
    }

    // optional int32 itemid = 3;

    pub fn clear_itemid(&mut self) {
        self.itemid = ::std::option::Option::None;
    }

    pub fn has_itemid(&self) -> bool {
        self.itemid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_itemid(&mut self, v: i32) {
        self.itemid = ::std::option::Option::Some(v);
    }

    pub fn get_itemid(&self) -> i32 {
        self.itemid.unwrap_or(0)
    }

    fn get_itemid_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.itemid
    }

    fn mut_itemid_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.itemid
    }
}

impl ::protobuf::Message for CDOTAMsg_ItemAlert {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.itemid = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.x {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.y {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.itemid {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.itemid {
            os.write_int32(3, v)?;
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

impl ::protobuf::MessageStatic for CDOTAMsg_ItemAlert {
    fn new() -> CDOTAMsg_ItemAlert {
        CDOTAMsg_ItemAlert::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMsg_ItemAlert>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "x",
                    CDOTAMsg_ItemAlert::get_x_for_reflect,
                    CDOTAMsg_ItemAlert::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "y",
                    CDOTAMsg_ItemAlert::get_y_for_reflect,
                    CDOTAMsg_ItemAlert::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "itemid",
                    CDOTAMsg_ItemAlert::get_itemid_for_reflect,
                    CDOTAMsg_ItemAlert::mut_itemid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMsg_ItemAlert>(
                    "CDOTAMsg_ItemAlert",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMsg_ItemAlert {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_itemid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMsg_ItemAlert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMsg_ItemAlert {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMsg_MapLine {
    // message fields
    x: ::std::option::Option<i32>,
    y: ::std::option::Option<i32>,
    initial: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMsg_MapLine {}

impl CDOTAMsg_MapLine {
    pub fn new() -> CDOTAMsg_MapLine {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMsg_MapLine {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMsg_MapLine> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMsg_MapLine,
        };
        unsafe {
            instance.get(CDOTAMsg_MapLine::new)
        }
    }

    // optional int32 x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: i32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> i32 {
        self.x.unwrap_or(0)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.x
    }

    // optional int32 y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: i32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> i32 {
        self.y.unwrap_or(0)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.y
    }

    // optional bool initial = 3;

    pub fn clear_initial(&mut self) {
        self.initial = ::std::option::Option::None;
    }

    pub fn has_initial(&self) -> bool {
        self.initial.is_some()
    }

    // Param is passed by value, moved
    pub fn set_initial(&mut self, v: bool) {
        self.initial = ::std::option::Option::Some(v);
    }

    pub fn get_initial(&self) -> bool {
        self.initial.unwrap_or(false)
    }

    fn get_initial_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.initial
    }

    fn mut_initial_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.initial
    }
}

impl ::protobuf::Message for CDOTAMsg_MapLine {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.initial = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.x {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.y {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.initial {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.initial {
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

impl ::protobuf::MessageStatic for CDOTAMsg_MapLine {
    fn new() -> CDOTAMsg_MapLine {
        CDOTAMsg_MapLine::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMsg_MapLine>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "x",
                    CDOTAMsg_MapLine::get_x_for_reflect,
                    CDOTAMsg_MapLine::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "y",
                    CDOTAMsg_MapLine::get_y_for_reflect,
                    CDOTAMsg_MapLine::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "initial",
                    CDOTAMsg_MapLine::get_initial_for_reflect,
                    CDOTAMsg_MapLine::mut_initial_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMsg_MapLine>(
                    "CDOTAMsg_MapLine",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMsg_MapLine {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_initial();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMsg_MapLine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMsg_MapLine {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMsg_WorldLine {
    // message fields
    x: ::std::option::Option<i32>,
    y: ::std::option::Option<i32>,
    z: ::std::option::Option<i32>,
    initial: ::std::option::Option<bool>,
    end: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMsg_WorldLine {}

impl CDOTAMsg_WorldLine {
    pub fn new() -> CDOTAMsg_WorldLine {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMsg_WorldLine {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMsg_WorldLine> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMsg_WorldLine,
        };
        unsafe {
            instance.get(CDOTAMsg_WorldLine::new)
        }
    }

    // optional int32 x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: i32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> i32 {
        self.x.unwrap_or(0)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.x
    }

    // optional int32 y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: i32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> i32 {
        self.y.unwrap_or(0)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.y
    }

    // optional int32 z = 3;

    pub fn clear_z(&mut self) {
        self.z = ::std::option::Option::None;
    }

    pub fn has_z(&self) -> bool {
        self.z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_z(&mut self, v: i32) {
        self.z = ::std::option::Option::Some(v);
    }

    pub fn get_z(&self) -> i32 {
        self.z.unwrap_or(0)
    }

    fn get_z_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.z
    }

    fn mut_z_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.z
    }

    // optional bool initial = 4;

    pub fn clear_initial(&mut self) {
        self.initial = ::std::option::Option::None;
    }

    pub fn has_initial(&self) -> bool {
        self.initial.is_some()
    }

    // Param is passed by value, moved
    pub fn set_initial(&mut self, v: bool) {
        self.initial = ::std::option::Option::Some(v);
    }

    pub fn get_initial(&self) -> bool {
        self.initial.unwrap_or(false)
    }

    fn get_initial_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.initial
    }

    fn mut_initial_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.initial
    }

    // optional bool end = 5;

    pub fn clear_end(&mut self) {
        self.end = ::std::option::Option::None;
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: bool) {
        self.end = ::std::option::Option::Some(v);
    }

    pub fn get_end(&self) -> bool {
        self.end.unwrap_or(false)
    }

    fn get_end_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.end
    }

    fn mut_end_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.end
    }
}

impl ::protobuf::Message for CDOTAMsg_WorldLine {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.z = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.initial = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.end = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.x {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.y {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.z {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.initial {
            my_size += 2;
        }
        if let Some(v) = self.end {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.z {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.initial {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.end {
            os.write_bool(5, v)?;
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

impl ::protobuf::MessageStatic for CDOTAMsg_WorldLine {
    fn new() -> CDOTAMsg_WorldLine {
        CDOTAMsg_WorldLine::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMsg_WorldLine>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "x",
                    CDOTAMsg_WorldLine::get_x_for_reflect,
                    CDOTAMsg_WorldLine::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "y",
                    CDOTAMsg_WorldLine::get_y_for_reflect,
                    CDOTAMsg_WorldLine::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "z",
                    CDOTAMsg_WorldLine::get_z_for_reflect,
                    CDOTAMsg_WorldLine::mut_z_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "initial",
                    CDOTAMsg_WorldLine::get_initial_for_reflect,
                    CDOTAMsg_WorldLine::mut_initial_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "end",
                    CDOTAMsg_WorldLine::get_end_for_reflect,
                    CDOTAMsg_WorldLine::mut_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMsg_WorldLine>(
                    "CDOTAMsg_WorldLine",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMsg_WorldLine {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_z();
        self.clear_initial();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMsg_WorldLine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMsg_WorldLine {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMsg_SendStatPopup {
    // message fields
    style: ::std::option::Option<EDOTAStatPopupTypes>,
    stat_strings: ::protobuf::RepeatedField<::std::string::String>,
    stat_images: ::std::vec::Vec<i32>,
    stat_image_types: ::std::vec::Vec<i32>,
    duration: ::std::option::Option<f32>,
    use_html: ::std::option::Option<bool>,
    movie_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMsg_SendStatPopup {}

impl CDOTAMsg_SendStatPopup {
    pub fn new() -> CDOTAMsg_SendStatPopup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMsg_SendStatPopup {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMsg_SendStatPopup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMsg_SendStatPopup,
        };
        unsafe {
            instance.get(CDOTAMsg_SendStatPopup::new)
        }
    }

    // optional .EDOTAStatPopupTypes style = 1;

    pub fn clear_style(&mut self) {
        self.style = ::std::option::Option::None;
    }

    pub fn has_style(&self) -> bool {
        self.style.is_some()
    }

    // Param is passed by value, moved
    pub fn set_style(&mut self, v: EDOTAStatPopupTypes) {
        self.style = ::std::option::Option::Some(v);
    }

    pub fn get_style(&self) -> EDOTAStatPopupTypes {
        self.style.unwrap_or(EDOTAStatPopupTypes::k_EDOTA_SPT_Textline)
    }

    fn get_style_for_reflect(&self) -> &::std::option::Option<EDOTAStatPopupTypes> {
        &self.style
    }

    fn mut_style_for_reflect(&mut self) -> &mut ::std::option::Option<EDOTAStatPopupTypes> {
        &mut self.style
    }

    // repeated string stat_strings = 2;

    pub fn clear_stat_strings(&mut self) {
        self.stat_strings.clear();
    }

    // Param is passed by value, moved
    pub fn set_stat_strings(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.stat_strings = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stat_strings(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.stat_strings
    }

    // Take field
    pub fn take_stat_strings(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.stat_strings, ::protobuf::RepeatedField::new())
    }

    pub fn get_stat_strings(&self) -> &[::std::string::String] {
        &self.stat_strings
    }

    fn get_stat_strings_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.stat_strings
    }

    fn mut_stat_strings_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.stat_strings
    }

    // repeated int32 stat_images = 3;

    pub fn clear_stat_images(&mut self) {
        self.stat_images.clear();
    }

    // Param is passed by value, moved
    pub fn set_stat_images(&mut self, v: ::std::vec::Vec<i32>) {
        self.stat_images = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stat_images(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.stat_images
    }

    // Take field
    pub fn take_stat_images(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.stat_images, ::std::vec::Vec::new())
    }

    pub fn get_stat_images(&self) -> &[i32] {
        &self.stat_images
    }

    fn get_stat_images_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.stat_images
    }

    fn mut_stat_images_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.stat_images
    }

    // repeated int32 stat_image_types = 4;

    pub fn clear_stat_image_types(&mut self) {
        self.stat_image_types.clear();
    }

    // Param is passed by value, moved
    pub fn set_stat_image_types(&mut self, v: ::std::vec::Vec<i32>) {
        self.stat_image_types = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stat_image_types(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.stat_image_types
    }

    // Take field
    pub fn take_stat_image_types(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.stat_image_types, ::std::vec::Vec::new())
    }

    pub fn get_stat_image_types(&self) -> &[i32] {
        &self.stat_image_types
    }

    fn get_stat_image_types_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.stat_image_types
    }

    fn mut_stat_image_types_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.stat_image_types
    }

    // optional float duration = 5;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: f32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> f32 {
        self.duration.unwrap_or(0.)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.duration
    }

    // optional bool use_html = 6;

    pub fn clear_use_html(&mut self) {
        self.use_html = ::std::option::Option::None;
    }

    pub fn has_use_html(&self) -> bool {
        self.use_html.is_some()
    }

    // Param is passed by value, moved
    pub fn set_use_html(&mut self, v: bool) {
        self.use_html = ::std::option::Option::Some(v);
    }

    pub fn get_use_html(&self) -> bool {
        self.use_html.unwrap_or(false)
    }

    fn get_use_html_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.use_html
    }

    fn mut_use_html_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.use_html
    }

    // optional string movie_name = 7;

    pub fn clear_movie_name(&mut self) {
        self.movie_name.clear();
    }

    pub fn has_movie_name(&self) -> bool {
        self.movie_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_movie_name(&mut self, v: ::std::string::String) {
        self.movie_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_movie_name(&mut self) -> &mut ::std::string::String {
        if self.movie_name.is_none() {
            self.movie_name.set_default();
        }
        self.movie_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_movie_name(&mut self) -> ::std::string::String {
        self.movie_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_movie_name(&self) -> &str {
        match self.movie_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_movie_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.movie_name
    }

    fn mut_movie_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.movie_name
    }
}

impl ::protobuf::Message for CDOTAMsg_SendStatPopup {
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
                    self.style = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.stat_strings)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.stat_images)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.stat_image_types)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.use_html = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.movie_name)?;
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
        if let Some(v) = self.style {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        for value in &self.stat_strings {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.stat_images {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.stat_image_types {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.duration {
            my_size += 5;
        }
        if let Some(v) = self.use_html {
            my_size += 2;
        }
        if let Some(ref v) = self.movie_name.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.style {
            os.write_enum(1, v.value())?;
        }
        for v in &self.stat_strings {
            os.write_string(2, &v)?;
        };
        for v in &self.stat_images {
            os.write_int32(3, *v)?;
        };
        for v in &self.stat_image_types {
            os.write_int32(4, *v)?;
        };
        if let Some(v) = self.duration {
            os.write_float(5, v)?;
        }
        if let Some(v) = self.use_html {
            os.write_bool(6, v)?;
        }
        if let Some(ref v) = self.movie_name.as_ref() {
            os.write_string(7, &v)?;
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

impl ::protobuf::MessageStatic for CDOTAMsg_SendStatPopup {
    fn new() -> CDOTAMsg_SendStatPopup {
        CDOTAMsg_SendStatPopup::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMsg_SendStatPopup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EDOTAStatPopupTypes>>(
                    "style",
                    CDOTAMsg_SendStatPopup::get_style_for_reflect,
                    CDOTAMsg_SendStatPopup::mut_style_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "stat_strings",
                    CDOTAMsg_SendStatPopup::get_stat_strings_for_reflect,
                    CDOTAMsg_SendStatPopup::mut_stat_strings_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "stat_images",
                    CDOTAMsg_SendStatPopup::get_stat_images_for_reflect,
                    CDOTAMsg_SendStatPopup::mut_stat_images_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "stat_image_types",
                    CDOTAMsg_SendStatPopup::get_stat_image_types_for_reflect,
                    CDOTAMsg_SendStatPopup::mut_stat_image_types_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "duration",
                    CDOTAMsg_SendStatPopup::get_duration_for_reflect,
                    CDOTAMsg_SendStatPopup::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "use_html",
                    CDOTAMsg_SendStatPopup::get_use_html_for_reflect,
                    CDOTAMsg_SendStatPopup::mut_use_html_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "movie_name",
                    CDOTAMsg_SendStatPopup::get_movie_name_for_reflect,
                    CDOTAMsg_SendStatPopup::mut_movie_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMsg_SendStatPopup>(
                    "CDOTAMsg_SendStatPopup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMsg_SendStatPopup {
    fn clear(&mut self) {
        self.clear_style();
        self.clear_stat_strings();
        self.clear_stat_images();
        self.clear_stat_image_types();
        self.clear_duration();
        self.clear_use_html();
        self.clear_movie_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMsg_SendStatPopup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMsg_SendStatPopup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMsg_DismissAllStatPopups {
    // message fields
    time_delay: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMsg_DismissAllStatPopups {}

impl CDOTAMsg_DismissAllStatPopups {
    pub fn new() -> CDOTAMsg_DismissAllStatPopups {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMsg_DismissAllStatPopups {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMsg_DismissAllStatPopups> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMsg_DismissAllStatPopups,
        };
        unsafe {
            instance.get(CDOTAMsg_DismissAllStatPopups::new)
        }
    }

    // optional float time_delay = 1;

    pub fn clear_time_delay(&mut self) {
        self.time_delay = ::std::option::Option::None;
    }

    pub fn has_time_delay(&self) -> bool {
        self.time_delay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_delay(&mut self, v: f32) {
        self.time_delay = ::std::option::Option::Some(v);
    }

    pub fn get_time_delay(&self) -> f32 {
        self.time_delay.unwrap_or(0.)
    }

    fn get_time_delay_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.time_delay
    }

    fn mut_time_delay_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.time_delay
    }
}

impl ::protobuf::Message for CDOTAMsg_DismissAllStatPopups {
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
                    let tmp = is.read_float()?;
                    self.time_delay = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.time_delay {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.time_delay {
            os.write_float(1, v)?;
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

impl ::protobuf::MessageStatic for CDOTAMsg_DismissAllStatPopups {
    fn new() -> CDOTAMsg_DismissAllStatPopups {
        CDOTAMsg_DismissAllStatPopups::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMsg_DismissAllStatPopups>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "time_delay",
                    CDOTAMsg_DismissAllStatPopups::get_time_delay_for_reflect,
                    CDOTAMsg_DismissAllStatPopups::mut_time_delay_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMsg_DismissAllStatPopups>(
                    "CDOTAMsg_DismissAllStatPopups",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMsg_DismissAllStatPopups {
    fn clear(&mut self) {
        self.clear_time_delay();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMsg_DismissAllStatPopups {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMsg_DismissAllStatPopups {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMsg_CoachHUDPing {
    // message fields
    x: ::std::option::Option<u32>,
    y: ::std::option::Option<u32>,
    tgtpath: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMsg_CoachHUDPing {}

impl CDOTAMsg_CoachHUDPing {
    pub fn new() -> CDOTAMsg_CoachHUDPing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMsg_CoachHUDPing {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMsg_CoachHUDPing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMsg_CoachHUDPing,
        };
        unsafe {
            instance.get(CDOTAMsg_CoachHUDPing::new)
        }
    }

    // optional uint32 x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: u32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> u32 {
        self.x.unwrap_or(0)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.x
    }

    // optional uint32 y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: u32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> u32 {
        self.y.unwrap_or(0)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.y
    }

    // optional string tgtpath = 3;

    pub fn clear_tgtpath(&mut self) {
        self.tgtpath.clear();
    }

    pub fn has_tgtpath(&self) -> bool {
        self.tgtpath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tgtpath(&mut self, v: ::std::string::String) {
        self.tgtpath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tgtpath(&mut self) -> &mut ::std::string::String {
        if self.tgtpath.is_none() {
            self.tgtpath.set_default();
        }
        self.tgtpath.as_mut().unwrap()
    }

    // Take field
    pub fn take_tgtpath(&mut self) -> ::std::string::String {
        self.tgtpath.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_tgtpath(&self) -> &str {
        match self.tgtpath.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_tgtpath_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.tgtpath
    }

    fn mut_tgtpath_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.tgtpath
    }
}

impl ::protobuf::Message for CDOTAMsg_CoachHUDPing {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.tgtpath)?;
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
        if let Some(v) = self.x {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.y {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.tgtpath.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.tgtpath.as_ref() {
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

impl ::protobuf::MessageStatic for CDOTAMsg_CoachHUDPing {
    fn new() -> CDOTAMsg_CoachHUDPing {
        CDOTAMsg_CoachHUDPing::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMsg_CoachHUDPing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "x",
                    CDOTAMsg_CoachHUDPing::get_x_for_reflect,
                    CDOTAMsg_CoachHUDPing::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "y",
                    CDOTAMsg_CoachHUDPing::get_y_for_reflect,
                    CDOTAMsg_CoachHUDPing::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tgtpath",
                    CDOTAMsg_CoachHUDPing::get_tgtpath_for_reflect,
                    CDOTAMsg_CoachHUDPing::mut_tgtpath_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMsg_CoachHUDPing>(
                    "CDOTAMsg_CoachHUDPing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMsg_CoachHUDPing {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_tgtpath();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMsg_CoachHUDPing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMsg_CoachHUDPing {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMsg_UnitOrder {
    // message fields
    issuer: ::std::option::Option<i32>,
    order_type: ::std::option::Option<dotaunitorder_t>,
    units: ::std::vec::Vec<i32>,
    target_index: ::std::option::Option<i32>,
    ability_index: ::std::option::Option<i32>,
    position: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    queue: ::std::option::Option<bool>,
    sequence_number: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMsg_UnitOrder {}

impl CDOTAMsg_UnitOrder {
    pub fn new() -> CDOTAMsg_UnitOrder {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMsg_UnitOrder {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMsg_UnitOrder> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMsg_UnitOrder,
        };
        unsafe {
            instance.get(CDOTAMsg_UnitOrder::new)
        }
    }

    // optional sint32 issuer = 1;

    pub fn clear_issuer(&mut self) {
        self.issuer = ::std::option::Option::None;
    }

    pub fn has_issuer(&self) -> bool {
        self.issuer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_issuer(&mut self, v: i32) {
        self.issuer = ::std::option::Option::Some(v);
    }

    pub fn get_issuer(&self) -> i32 {
        self.issuer.unwrap_or(-1i32)
    }

    fn get_issuer_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.issuer
    }

    fn mut_issuer_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.issuer
    }

    // optional .dotaunitorder_t order_type = 2;

    pub fn clear_order_type(&mut self) {
        self.order_type = ::std::option::Option::None;
    }

    pub fn has_order_type(&self) -> bool {
        self.order_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_order_type(&mut self, v: dotaunitorder_t) {
        self.order_type = ::std::option::Option::Some(v);
    }

    pub fn get_order_type(&self) -> dotaunitorder_t {
        self.order_type.unwrap_or(dotaunitorder_t::DOTA_UNIT_ORDER_NONE)
    }

    fn get_order_type_for_reflect(&self) -> &::std::option::Option<dotaunitorder_t> {
        &self.order_type
    }

    fn mut_order_type_for_reflect(&mut self) -> &mut ::std::option::Option<dotaunitorder_t> {
        &mut self.order_type
    }

    // repeated int32 units = 3;

    pub fn clear_units(&mut self) {
        self.units.clear();
    }

    // Param is passed by value, moved
    pub fn set_units(&mut self, v: ::std::vec::Vec<i32>) {
        self.units = v;
    }

    // Mutable pointer to the field.
    pub fn mut_units(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.units
    }

    // Take field
    pub fn take_units(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.units, ::std::vec::Vec::new())
    }

    pub fn get_units(&self) -> &[i32] {
        &self.units
    }

    fn get_units_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.units
    }

    fn mut_units_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.units
    }

    // optional int32 target_index = 4;

    pub fn clear_target_index(&mut self) {
        self.target_index = ::std::option::Option::None;
    }

    pub fn has_target_index(&self) -> bool {
        self.target_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_index(&mut self, v: i32) {
        self.target_index = ::std::option::Option::Some(v);
    }

    pub fn get_target_index(&self) -> i32 {
        self.target_index.unwrap_or(0)
    }

    fn get_target_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.target_index
    }

    fn mut_target_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.target_index
    }

    // optional int32 ability_index = 5;

    pub fn clear_ability_index(&mut self) {
        self.ability_index = ::std::option::Option::None;
    }

    pub fn has_ability_index(&self) -> bool {
        self.ability_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_index(&mut self, v: i32) {
        self.ability_index = ::std::option::Option::Some(v);
    }

    pub fn get_ability_index(&self) -> i32 {
        self.ability_index.unwrap_or(0)
    }

    fn get_ability_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ability_index
    }

    fn mut_ability_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ability_index
    }

    // optional .CMsgVector position = 6;

    pub fn clear_position(&mut self) {
        self.position.clear();
    }

    pub fn has_position(&self) -> bool {
        self.position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.position = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_position(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.position.is_none() {
            self.position.set_default();
        }
        self.position.as_mut().unwrap()
    }

    // Take field
    pub fn take_position(&mut self) -> super::networkbasetypes::CMsgVector {
        self.position.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_position(&self) -> &super::networkbasetypes::CMsgVector {
        self.position.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_position_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.position
    }

    fn mut_position_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.position
    }

    // optional bool queue = 7;

    pub fn clear_queue(&mut self) {
        self.queue = ::std::option::Option::None;
    }

    pub fn has_queue(&self) -> bool {
        self.queue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_queue(&mut self, v: bool) {
        self.queue = ::std::option::Option::Some(v);
    }

    pub fn get_queue(&self) -> bool {
        self.queue.unwrap_or(false)
    }

    fn get_queue_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.queue
    }

    fn mut_queue_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.queue
    }

    // optional int32 sequence_number = 8;

    pub fn clear_sequence_number(&mut self) {
        self.sequence_number = ::std::option::Option::None;
    }

    pub fn has_sequence_number(&self) -> bool {
        self.sequence_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence_number(&mut self, v: i32) {
        self.sequence_number = ::std::option::Option::Some(v);
    }

    pub fn get_sequence_number(&self) -> i32 {
        self.sequence_number.unwrap_or(0)
    }

    fn get_sequence_number_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.sequence_number
    }

    fn mut_sequence_number_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.sequence_number
    }
}

impl ::protobuf::Message for CDOTAMsg_UnitOrder {
    fn is_initialized(&self) -> bool {
        for v in &self.position {
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
                    let tmp = is.read_sint32()?;
                    self.issuer = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.order_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.units)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.target_index = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.ability_index = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.position)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.queue = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.sequence_number = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.issuer {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        }
        if let Some(v) = self.order_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        for value in &self.units {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.target_index {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ability_index {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.position.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.queue {
            my_size += 2;
        }
        if let Some(v) = self.sequence_number {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.issuer {
            os.write_sint32(1, v)?;
        }
        if let Some(v) = self.order_type {
            os.write_enum(2, v.value())?;
        }
        for v in &self.units {
            os.write_int32(3, *v)?;
        };
        if let Some(v) = self.target_index {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.ability_index {
            os.write_int32(5, v)?;
        }
        if let Some(ref v) = self.position.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.queue {
            os.write_bool(7, v)?;
        }
        if let Some(v) = self.sequence_number {
            os.write_int32(8, v)?;
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

impl ::protobuf::MessageStatic for CDOTAMsg_UnitOrder {
    fn new() -> CDOTAMsg_UnitOrder {
        CDOTAMsg_UnitOrder::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMsg_UnitOrder>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "issuer",
                    CDOTAMsg_UnitOrder::get_issuer_for_reflect,
                    CDOTAMsg_UnitOrder::mut_issuer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<dotaunitorder_t>>(
                    "order_type",
                    CDOTAMsg_UnitOrder::get_order_type_for_reflect,
                    CDOTAMsg_UnitOrder::mut_order_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "units",
                    CDOTAMsg_UnitOrder::get_units_for_reflect,
                    CDOTAMsg_UnitOrder::mut_units_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "target_index",
                    CDOTAMsg_UnitOrder::get_target_index_for_reflect,
                    CDOTAMsg_UnitOrder::mut_target_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ability_index",
                    CDOTAMsg_UnitOrder::get_ability_index_for_reflect,
                    CDOTAMsg_UnitOrder::mut_ability_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "position",
                    CDOTAMsg_UnitOrder::get_position_for_reflect,
                    CDOTAMsg_UnitOrder::mut_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "queue",
                    CDOTAMsg_UnitOrder::get_queue_for_reflect,
                    CDOTAMsg_UnitOrder::mut_queue_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "sequence_number",
                    CDOTAMsg_UnitOrder::get_sequence_number_for_reflect,
                    CDOTAMsg_UnitOrder::mut_sequence_number_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMsg_UnitOrder>(
                    "CDOTAMsg_UnitOrder",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMsg_UnitOrder {
    fn clear(&mut self) {
        self.clear_issuer();
        self.clear_order_type();
        self.clear_units();
        self.clear_target_index();
        self.clear_ability_index();
        self.clear_position();
        self.clear_queue();
        self.clear_sequence_number();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMsg_UnitOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMsg_UnitOrder {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EDOTAStatPopupTypes {
    k_EDOTA_SPT_Textline = 0,
    k_EDOTA_SPT_Basic = 1,
    k_EDOTA_SPT_Poll = 2,
    k_EDOTA_SPT_Grid = 3,
    k_EDOTA_SPT_DualImage = 4,
    k_EDOTA_SPT_Movie = 5,
}

impl ::protobuf::ProtobufEnum for EDOTAStatPopupTypes {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EDOTAStatPopupTypes> {
        match value {
            0 => ::std::option::Option::Some(EDOTAStatPopupTypes::k_EDOTA_SPT_Textline),
            1 => ::std::option::Option::Some(EDOTAStatPopupTypes::k_EDOTA_SPT_Basic),
            2 => ::std::option::Option::Some(EDOTAStatPopupTypes::k_EDOTA_SPT_Poll),
            3 => ::std::option::Option::Some(EDOTAStatPopupTypes::k_EDOTA_SPT_Grid),
            4 => ::std::option::Option::Some(EDOTAStatPopupTypes::k_EDOTA_SPT_DualImage),
            5 => ::std::option::Option::Some(EDOTAStatPopupTypes::k_EDOTA_SPT_Movie),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EDOTAStatPopupTypes] = &[
            EDOTAStatPopupTypes::k_EDOTA_SPT_Textline,
            EDOTAStatPopupTypes::k_EDOTA_SPT_Basic,
            EDOTAStatPopupTypes::k_EDOTA_SPT_Poll,
            EDOTAStatPopupTypes::k_EDOTA_SPT_Grid,
            EDOTAStatPopupTypes::k_EDOTA_SPT_DualImage,
            EDOTAStatPopupTypes::k_EDOTA_SPT_Movie,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EDOTAStatPopupTypes>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EDOTAStatPopupTypes", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EDOTAStatPopupTypes {
}

impl ::protobuf::reflect::ProtobufValue for EDOTAStatPopupTypes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum dotaunitorder_t {
    DOTA_UNIT_ORDER_NONE = 0,
    DOTA_UNIT_ORDER_MOVE_TO_POSITION = 1,
    DOTA_UNIT_ORDER_MOVE_TO_TARGET = 2,
    DOTA_UNIT_ORDER_ATTACK_MOVE = 3,
    DOTA_UNIT_ORDER_ATTACK_TARGET = 4,
    DOTA_UNIT_ORDER_CAST_POSITION = 5,
    DOTA_UNIT_ORDER_CAST_TARGET = 6,
    DOTA_UNIT_ORDER_CAST_TARGET_TREE = 7,
    DOTA_UNIT_ORDER_CAST_NO_TARGET = 8,
    DOTA_UNIT_ORDER_CAST_TOGGLE = 9,
    DOTA_UNIT_ORDER_HOLD_POSITION = 10,
    DOTA_UNIT_ORDER_TRAIN_ABILITY = 11,
    DOTA_UNIT_ORDER_DROP_ITEM = 12,
    DOTA_UNIT_ORDER_GIVE_ITEM = 13,
    DOTA_UNIT_ORDER_PICKUP_ITEM = 14,
    DOTA_UNIT_ORDER_PICKUP_RUNE = 15,
    DOTA_UNIT_ORDER_PURCHASE_ITEM = 16,
    DOTA_UNIT_ORDER_SELL_ITEM = 17,
    DOTA_UNIT_ORDER_DISASSEMBLE_ITEM = 18,
    DOTA_UNIT_ORDER_MOVE_ITEM = 19,
    DOTA_UNIT_ORDER_CAST_TOGGLE_AUTO = 20,
    DOTA_UNIT_ORDER_STOP = 21,
    DOTA_UNIT_ORDER_TAUNT = 22,
    DOTA_UNIT_ORDER_BUYBACK = 23,
    DOTA_UNIT_ORDER_GLYPH = 24,
    DOTA_UNIT_ORDER_EJECT_ITEM_FROM_STASH = 25,
    DOTA_UNIT_ORDER_CAST_RUNE = 26,
    DOTA_UNIT_ORDER_PING_ABILITY = 27,
    DOTA_UNIT_ORDER_MOVE_TO_DIRECTION = 28,
    DOTA_UNIT_ORDER_PATROL = 29,
    DOTA_UNIT_ORDER_VECTOR_TARGET_POSITION = 30,
    DOTA_UNIT_ORDER_RADAR = 31,
    DOTA_UNIT_ORDER_SET_ITEM_COMBINE_LOCK = 32,
    DOTA_UNIT_ORDER_CONTINUE = 33,
    DOTA_UNIT_ORDER_VECTOR_TARGET_CANCELED = 34,
    DOTA_UNIT_ORDER_CAST_RIVER_PAINT = 35,
}

impl ::protobuf::ProtobufEnum for dotaunitorder_t {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<dotaunitorder_t> {
        match value {
            0 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_NONE),
            1 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_MOVE_TO_POSITION),
            2 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_MOVE_TO_TARGET),
            3 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_ATTACK_MOVE),
            4 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_ATTACK_TARGET),
            5 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_CAST_POSITION),
            6 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_CAST_TARGET),
            7 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_CAST_TARGET_TREE),
            8 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_CAST_NO_TARGET),
            9 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_CAST_TOGGLE),
            10 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_HOLD_POSITION),
            11 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_TRAIN_ABILITY),
            12 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_DROP_ITEM),
            13 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_GIVE_ITEM),
            14 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_PICKUP_ITEM),
            15 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_PICKUP_RUNE),
            16 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_PURCHASE_ITEM),
            17 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_SELL_ITEM),
            18 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_DISASSEMBLE_ITEM),
            19 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_MOVE_ITEM),
            20 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_CAST_TOGGLE_AUTO),
            21 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_STOP),
            22 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_TAUNT),
            23 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_BUYBACK),
            24 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_GLYPH),
            25 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_EJECT_ITEM_FROM_STASH),
            26 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_CAST_RUNE),
            27 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_PING_ABILITY),
            28 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_MOVE_TO_DIRECTION),
            29 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_PATROL),
            30 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_VECTOR_TARGET_POSITION),
            31 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_RADAR),
            32 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_SET_ITEM_COMBINE_LOCK),
            33 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_CONTINUE),
            34 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_VECTOR_TARGET_CANCELED),
            35 => ::std::option::Option::Some(dotaunitorder_t::DOTA_UNIT_ORDER_CAST_RIVER_PAINT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [dotaunitorder_t] = &[
            dotaunitorder_t::DOTA_UNIT_ORDER_NONE,
            dotaunitorder_t::DOTA_UNIT_ORDER_MOVE_TO_POSITION,
            dotaunitorder_t::DOTA_UNIT_ORDER_MOVE_TO_TARGET,
            dotaunitorder_t::DOTA_UNIT_ORDER_ATTACK_MOVE,
            dotaunitorder_t::DOTA_UNIT_ORDER_ATTACK_TARGET,
            dotaunitorder_t::DOTA_UNIT_ORDER_CAST_POSITION,
            dotaunitorder_t::DOTA_UNIT_ORDER_CAST_TARGET,
            dotaunitorder_t::DOTA_UNIT_ORDER_CAST_TARGET_TREE,
            dotaunitorder_t::DOTA_UNIT_ORDER_CAST_NO_TARGET,
            dotaunitorder_t::DOTA_UNIT_ORDER_CAST_TOGGLE,
            dotaunitorder_t::DOTA_UNIT_ORDER_HOLD_POSITION,
            dotaunitorder_t::DOTA_UNIT_ORDER_TRAIN_ABILITY,
            dotaunitorder_t::DOTA_UNIT_ORDER_DROP_ITEM,
            dotaunitorder_t::DOTA_UNIT_ORDER_GIVE_ITEM,
            dotaunitorder_t::DOTA_UNIT_ORDER_PICKUP_ITEM,
            dotaunitorder_t::DOTA_UNIT_ORDER_PICKUP_RUNE,
            dotaunitorder_t::DOTA_UNIT_ORDER_PURCHASE_ITEM,
            dotaunitorder_t::DOTA_UNIT_ORDER_SELL_ITEM,
            dotaunitorder_t::DOTA_UNIT_ORDER_DISASSEMBLE_ITEM,
            dotaunitorder_t::DOTA_UNIT_ORDER_MOVE_ITEM,
            dotaunitorder_t::DOTA_UNIT_ORDER_CAST_TOGGLE_AUTO,
            dotaunitorder_t::DOTA_UNIT_ORDER_STOP,
            dotaunitorder_t::DOTA_UNIT_ORDER_TAUNT,
            dotaunitorder_t::DOTA_UNIT_ORDER_BUYBACK,
            dotaunitorder_t::DOTA_UNIT_ORDER_GLYPH,
            dotaunitorder_t::DOTA_UNIT_ORDER_EJECT_ITEM_FROM_STASH,
            dotaunitorder_t::DOTA_UNIT_ORDER_CAST_RUNE,
            dotaunitorder_t::DOTA_UNIT_ORDER_PING_ABILITY,
            dotaunitorder_t::DOTA_UNIT_ORDER_MOVE_TO_DIRECTION,
            dotaunitorder_t::DOTA_UNIT_ORDER_PATROL,
            dotaunitorder_t::DOTA_UNIT_ORDER_VECTOR_TARGET_POSITION,
            dotaunitorder_t::DOTA_UNIT_ORDER_RADAR,
            dotaunitorder_t::DOTA_UNIT_ORDER_SET_ITEM_COMBINE_LOCK,
            dotaunitorder_t::DOTA_UNIT_ORDER_CONTINUE,
            dotaunitorder_t::DOTA_UNIT_ORDER_VECTOR_TARGET_CANCELED,
            dotaunitorder_t::DOTA_UNIT_ORDER_CAST_RIVER_PAINT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<dotaunitorder_t>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("dotaunitorder_t", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for dotaunitorder_t {
}

impl ::protobuf::reflect::ProtobufValue for dotaunitorder_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19dota_commonmessages.proto\x1a\x16networkbasetypes.proto\"\x80\x01\
    \n\x15CDOTAMsg_LocationPing\x12\x0c\n\x01x\x18\x01\x20\x01(\x05R\x01x\
    \x12\x0c\n\x01y\x18\x02\x20\x01(\x05R\x01y\x12\x16\n\x06target\x18\x03\
    \x20\x01(\x05R\x06target\x12\x1f\n\x0bdirect_ping\x18\x04\x20\x01(\x08R\
    \ndirectPing\x12\x12\n\x04type\x18\x05\x20\x01(\x05R\x04type\"H\n\x12CDO\
    TAMsg_ItemAlert\x12\x0c\n\x01x\x18\x01\x20\x01(\x05R\x01x\x12\x0c\n\x01y\
    \x18\x02\x20\x01(\x05R\x01y\x12\x16\n\x06itemid\x18\x03\x20\x01(\x05R\
    \x06itemid\"H\n\x10CDOTAMsg_MapLine\x12\x0c\n\x01x\x18\x01\x20\x01(\x05R\
    \x01x\x12\x0c\n\x01y\x18\x02\x20\x01(\x05R\x01y\x12\x18\n\x07initial\x18\
    \x03\x20\x01(\x08R\x07initial\"j\n\x12CDOTAMsg_WorldLine\x12\x0c\n\x01x\
    \x18\x01\x20\x01(\x05R\x01x\x12\x0c\n\x01y\x18\x02\x20\x01(\x05R\x01y\
    \x12\x0c\n\x01z\x18\x03\x20\x01(\x05R\x01z\x12\x18\n\x07initial\x18\x04\
    \x20\x01(\x08R\x07initial\x12\x10\n\x03end\x18\x05\x20\x01(\x08R\x03end\
    \"\x9e\x02\n\x16CDOTAMsg_SendStatPopup\x12@\n\x05style\x18\x01\x20\x01(\
    \x0e2\x14.EDOTAStatPopupTypes:\x14k_EDOTA_SPT_TextlineR\x05style\x12!\n\
    \x0cstat_strings\x18\x02\x20\x03(\tR\x0bstatStrings\x12\x1f\n\x0bstat_im\
    ages\x18\x03\x20\x03(\x05R\nstatImages\x12(\n\x10stat_image_types\x18\
    \x04\x20\x03(\x05R\x0estatImageTypes\x12\x1a\n\x08duration\x18\x05\x20\
    \x01(\x02R\x08duration\x12\x19\n\x08use_html\x18\x06\x20\x01(\x08R\x07us\
    eHtml\x12\x1d\n\nmovie_name\x18\x07\x20\x01(\tR\tmovieName\">\n\x1dCDOTA\
    Msg_DismissAllStatPopups\x12\x1d\n\ntime_delay\x18\x01\x20\x01(\x02R\tti\
    meDelay\"M\n\x15CDOTAMsg_CoachHUDPing\x12\x0c\n\x01x\x18\x01\x20\x01(\rR\
    \x01x\x12\x0c\n\x01y\x18\x02\x20\x01(\rR\x01y\x12\x18\n\x07tgtpath\x18\
    \x03\x20\x01(\tR\x07tgtpath\"\xbd\x02\n\x12CDOTAMsg_UnitOrder\x12\x1a\n\
    \x06issuer\x18\x01\x20\x01(\x11:\x02-1R\x06issuer\x12E\n\norder_type\x18\
    \x02\x20\x01(\x0e2\x10.dotaunitorder_t:\x14DOTA_UNIT_ORDER_NONER\torderT\
    ype\x12\x14\n\x05units\x18\x03\x20\x03(\x05R\x05units\x12!\n\x0ctarget_i\
    ndex\x18\x04\x20\x01(\x05R\x0btargetIndex\x12#\n\rability_index\x18\x05\
    \x20\x01(\x05R\x0cabilityIndex\x12'\n\x08position\x18\x06\x20\x01(\x0b2\
    \x0b.CMsgVectorR\x08position\x12\x14\n\x05queue\x18\x07\x20\x01(\x08R\
    \x05queue\x12'\n\x0fsequence_number\x18\x08\x20\x01(\x05R\x0esequenceNum\
    ber*\xa4\x01\n\x13EDOTAStatPopupTypes\x12\x18\n\x14k_EDOTA_SPT_Textline\
    \x10\0\x12\x15\n\x11k_EDOTA_SPT_Basic\x10\x01\x12\x14\n\x10k_EDOTA_SPT_P\
    oll\x10\x02\x12\x14\n\x10k_EDOTA_SPT_Grid\x10\x03\x12\x19\n\x15k_EDOTA_S\
    PT_DualImage\x10\x04\x12\x15\n\x11k_EDOTA_SPT_Movie\x10\x05*\xd9\t\n\x0f\
    dotaunitorder_t\x12\x18\n\x14DOTA_UNIT_ORDER_NONE\x10\0\x12$\n\x20DOTA_U\
    NIT_ORDER_MOVE_TO_POSITION\x10\x01\x12\"\n\x1eDOTA_UNIT_ORDER_MOVE_TO_TA\
    RGET\x10\x02\x12\x1f\n\x1bDOTA_UNIT_ORDER_ATTACK_MOVE\x10\x03\x12!\n\x1d\
    DOTA_UNIT_ORDER_ATTACK_TARGET\x10\x04\x12!\n\x1dDOTA_UNIT_ORDER_CAST_POS\
    ITION\x10\x05\x12\x1f\n\x1bDOTA_UNIT_ORDER_CAST_TARGET\x10\x06\x12$\n\
    \x20DOTA_UNIT_ORDER_CAST_TARGET_TREE\x10\x07\x12\"\n\x1eDOTA_UNIT_ORDER_\
    CAST_NO_TARGET\x10\x08\x12\x1f\n\x1bDOTA_UNIT_ORDER_CAST_TOGGLE\x10\t\
    \x12!\n\x1dDOTA_UNIT_ORDER_HOLD_POSITION\x10\n\x12!\n\x1dDOTA_UNIT_ORDER\
    _TRAIN_ABILITY\x10\x0b\x12\x1d\n\x19DOTA_UNIT_ORDER_DROP_ITEM\x10\x0c\
    \x12\x1d\n\x19DOTA_UNIT_ORDER_GIVE_ITEM\x10\r\x12\x1f\n\x1bDOTA_UNIT_ORD\
    ER_PICKUP_ITEM\x10\x0e\x12\x1f\n\x1bDOTA_UNIT_ORDER_PICKUP_RUNE\x10\x0f\
    \x12!\n\x1dDOTA_UNIT_ORDER_PURCHASE_ITEM\x10\x10\x12\x1d\n\x19DOTA_UNIT_\
    ORDER_SELL_ITEM\x10\x11\x12$\n\x20DOTA_UNIT_ORDER_DISASSEMBLE_ITEM\x10\
    \x12\x12\x1d\n\x19DOTA_UNIT_ORDER_MOVE_ITEM\x10\x13\x12$\n\x20DOTA_UNIT_\
    ORDER_CAST_TOGGLE_AUTO\x10\x14\x12\x18\n\x14DOTA_UNIT_ORDER_STOP\x10\x15\
    \x12\x19\n\x15DOTA_UNIT_ORDER_TAUNT\x10\x16\x12\x1b\n\x17DOTA_UNIT_ORDER\
    _BUYBACK\x10\x17\x12\x19\n\x15DOTA_UNIT_ORDER_GLYPH\x10\x18\x12)\n%DOTA_\
    UNIT_ORDER_EJECT_ITEM_FROM_STASH\x10\x19\x12\x1d\n\x19DOTA_UNIT_ORDER_CA\
    ST_RUNE\x10\x1a\x12\x20\n\x1cDOTA_UNIT_ORDER_PING_ABILITY\x10\x1b\x12%\n\
    !DOTA_UNIT_ORDER_MOVE_TO_DIRECTION\x10\x1c\x12\x1a\n\x16DOTA_UNIT_ORDER_\
    PATROL\x10\x1d\x12*\n&DOTA_UNIT_ORDER_VECTOR_TARGET_POSITION\x10\x1e\x12\
    \x19\n\x15DOTA_UNIT_ORDER_RADAR\x10\x1f\x12)\n%DOTA_UNIT_ORDER_SET_ITEM_\
    COMBINE_LOCK\x10\x20\x12\x1c\n\x18DOTA_UNIT_ORDER_CONTINUE\x10!\x12*\n&D\
    OTA_UNIT_ORDER_VECTOR_TARGET_CANCELED\x10\"\x12$\n\x20DOTA_UNIT_ORDER_CA\
    ST_RIVER_PAINT\x10#B\x05H\x01\x80\x01\0\
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
