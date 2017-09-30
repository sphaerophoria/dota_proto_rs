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
pub struct CPublishedFile_Subscribe_Request {
    // message fields
    publishedfileid: ::std::option::Option<u64>,
    list_type: ::std::option::Option<u32>,
    appid: ::std::option::Option<i32>,
    notify_client: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPublishedFile_Subscribe_Request {}

impl CPublishedFile_Subscribe_Request {
    pub fn new() -> CPublishedFile_Subscribe_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPublishedFile_Subscribe_Request {
        static mut instance: ::protobuf::lazy::Lazy<CPublishedFile_Subscribe_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPublishedFile_Subscribe_Request,
        };
        unsafe {
            instance.get(CPublishedFile_Subscribe_Request::new)
        }
    }

    // optional uint64 publishedfileid = 1;

    pub fn clear_publishedfileid(&mut self) {
        self.publishedfileid = ::std::option::Option::None;
    }

    pub fn has_publishedfileid(&self) -> bool {
        self.publishedfileid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publishedfileid(&mut self, v: u64) {
        self.publishedfileid = ::std::option::Option::Some(v);
    }

    pub fn get_publishedfileid(&self) -> u64 {
        self.publishedfileid.unwrap_or(0)
    }

    fn get_publishedfileid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.publishedfileid
    }

    fn mut_publishedfileid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.publishedfileid
    }

    // optional uint32 list_type = 2;

    pub fn clear_list_type(&mut self) {
        self.list_type = ::std::option::Option::None;
    }

    pub fn has_list_type(&self) -> bool {
        self.list_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_list_type(&mut self, v: u32) {
        self.list_type = ::std::option::Option::Some(v);
    }

    pub fn get_list_type(&self) -> u32 {
        self.list_type.unwrap_or(0)
    }

    fn get_list_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.list_type
    }

    fn mut_list_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.list_type
    }

    // optional int32 appid = 3;

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: i32) {
        self.appid = ::std::option::Option::Some(v);
    }

    pub fn get_appid(&self) -> i32 {
        self.appid.unwrap_or(0)
    }

    fn get_appid_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.appid
    }

    fn mut_appid_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.appid
    }

    // optional bool notify_client = 4;

    pub fn clear_notify_client(&mut self) {
        self.notify_client = ::std::option::Option::None;
    }

    pub fn has_notify_client(&self) -> bool {
        self.notify_client.is_some()
    }

    // Param is passed by value, moved
    pub fn set_notify_client(&mut self, v: bool) {
        self.notify_client = ::std::option::Option::Some(v);
    }

    pub fn get_notify_client(&self) -> bool {
        self.notify_client.unwrap_or(false)
    }

    fn get_notify_client_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.notify_client
    }

    fn mut_notify_client_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.notify_client
    }
}

impl ::protobuf::Message for CPublishedFile_Subscribe_Request {
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
                    self.publishedfileid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.list_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.appid = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.notify_client = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.publishedfileid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.list_type {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.notify_client {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.publishedfileid {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.list_type {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.appid {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.notify_client {
            os.write_bool(4, v)?;
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

impl ::protobuf::MessageStatic for CPublishedFile_Subscribe_Request {
    fn new() -> CPublishedFile_Subscribe_Request {
        CPublishedFile_Subscribe_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPublishedFile_Subscribe_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "publishedfileid",
                    CPublishedFile_Subscribe_Request::get_publishedfileid_for_reflect,
                    CPublishedFile_Subscribe_Request::mut_publishedfileid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "list_type",
                    CPublishedFile_Subscribe_Request::get_list_type_for_reflect,
                    CPublishedFile_Subscribe_Request::mut_list_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "appid",
                    CPublishedFile_Subscribe_Request::get_appid_for_reflect,
                    CPublishedFile_Subscribe_Request::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "notify_client",
                    CPublishedFile_Subscribe_Request::get_notify_client_for_reflect,
                    CPublishedFile_Subscribe_Request::mut_notify_client_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CPublishedFile_Subscribe_Request>(
                    "CPublishedFile_Subscribe_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPublishedFile_Subscribe_Request {
    fn clear(&mut self) {
        self.clear_publishedfileid();
        self.clear_list_type();
        self.clear_appid();
        self.clear_notify_client();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPublishedFile_Subscribe_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPublishedFile_Subscribe_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CPublishedFile_Subscribe_Response {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPublishedFile_Subscribe_Response {}

impl CPublishedFile_Subscribe_Response {
    pub fn new() -> CPublishedFile_Subscribe_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPublishedFile_Subscribe_Response {
        static mut instance: ::protobuf::lazy::Lazy<CPublishedFile_Subscribe_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPublishedFile_Subscribe_Response,
        };
        unsafe {
            instance.get(CPublishedFile_Subscribe_Response::new)
        }
    }
}

impl ::protobuf::Message for CPublishedFile_Subscribe_Response {
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

impl ::protobuf::MessageStatic for CPublishedFile_Subscribe_Response {
    fn new() -> CPublishedFile_Subscribe_Response {
        CPublishedFile_Subscribe_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPublishedFile_Subscribe_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CPublishedFile_Subscribe_Response>(
                    "CPublishedFile_Subscribe_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPublishedFile_Subscribe_Response {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPublishedFile_Subscribe_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPublishedFile_Subscribe_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CPublishedFile_Unsubscribe_Request {
    // message fields
    publishedfileid: ::std::option::Option<u64>,
    list_type: ::std::option::Option<u32>,
    appid: ::std::option::Option<i32>,
    notify_client: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPublishedFile_Unsubscribe_Request {}

impl CPublishedFile_Unsubscribe_Request {
    pub fn new() -> CPublishedFile_Unsubscribe_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPublishedFile_Unsubscribe_Request {
        static mut instance: ::protobuf::lazy::Lazy<CPublishedFile_Unsubscribe_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPublishedFile_Unsubscribe_Request,
        };
        unsafe {
            instance.get(CPublishedFile_Unsubscribe_Request::new)
        }
    }

    // optional uint64 publishedfileid = 1;

    pub fn clear_publishedfileid(&mut self) {
        self.publishedfileid = ::std::option::Option::None;
    }

    pub fn has_publishedfileid(&self) -> bool {
        self.publishedfileid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publishedfileid(&mut self, v: u64) {
        self.publishedfileid = ::std::option::Option::Some(v);
    }

    pub fn get_publishedfileid(&self) -> u64 {
        self.publishedfileid.unwrap_or(0)
    }

    fn get_publishedfileid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.publishedfileid
    }

    fn mut_publishedfileid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.publishedfileid
    }

    // optional uint32 list_type = 2;

    pub fn clear_list_type(&mut self) {
        self.list_type = ::std::option::Option::None;
    }

    pub fn has_list_type(&self) -> bool {
        self.list_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_list_type(&mut self, v: u32) {
        self.list_type = ::std::option::Option::Some(v);
    }

    pub fn get_list_type(&self) -> u32 {
        self.list_type.unwrap_or(0)
    }

    fn get_list_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.list_type
    }

    fn mut_list_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.list_type
    }

    // optional int32 appid = 3;

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: i32) {
        self.appid = ::std::option::Option::Some(v);
    }

    pub fn get_appid(&self) -> i32 {
        self.appid.unwrap_or(0)
    }

    fn get_appid_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.appid
    }

    fn mut_appid_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.appid
    }

    // optional bool notify_client = 4;

    pub fn clear_notify_client(&mut self) {
        self.notify_client = ::std::option::Option::None;
    }

    pub fn has_notify_client(&self) -> bool {
        self.notify_client.is_some()
    }

    // Param is passed by value, moved
    pub fn set_notify_client(&mut self, v: bool) {
        self.notify_client = ::std::option::Option::Some(v);
    }

    pub fn get_notify_client(&self) -> bool {
        self.notify_client.unwrap_or(false)
    }

    fn get_notify_client_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.notify_client
    }

    fn mut_notify_client_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.notify_client
    }
}

impl ::protobuf::Message for CPublishedFile_Unsubscribe_Request {
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
                    self.publishedfileid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.list_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.appid = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.notify_client = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.publishedfileid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.list_type {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.notify_client {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.publishedfileid {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.list_type {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.appid {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.notify_client {
            os.write_bool(4, v)?;
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

impl ::protobuf::MessageStatic for CPublishedFile_Unsubscribe_Request {
    fn new() -> CPublishedFile_Unsubscribe_Request {
        CPublishedFile_Unsubscribe_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPublishedFile_Unsubscribe_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "publishedfileid",
                    CPublishedFile_Unsubscribe_Request::get_publishedfileid_for_reflect,
                    CPublishedFile_Unsubscribe_Request::mut_publishedfileid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "list_type",
                    CPublishedFile_Unsubscribe_Request::get_list_type_for_reflect,
                    CPublishedFile_Unsubscribe_Request::mut_list_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "appid",
                    CPublishedFile_Unsubscribe_Request::get_appid_for_reflect,
                    CPublishedFile_Unsubscribe_Request::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "notify_client",
                    CPublishedFile_Unsubscribe_Request::get_notify_client_for_reflect,
                    CPublishedFile_Unsubscribe_Request::mut_notify_client_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CPublishedFile_Unsubscribe_Request>(
                    "CPublishedFile_Unsubscribe_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPublishedFile_Unsubscribe_Request {
    fn clear(&mut self) {
        self.clear_publishedfileid();
        self.clear_list_type();
        self.clear_appid();
        self.clear_notify_client();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPublishedFile_Unsubscribe_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPublishedFile_Unsubscribe_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CPublishedFile_Unsubscribe_Response {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPublishedFile_Unsubscribe_Response {}

impl CPublishedFile_Unsubscribe_Response {
    pub fn new() -> CPublishedFile_Unsubscribe_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPublishedFile_Unsubscribe_Response {
        static mut instance: ::protobuf::lazy::Lazy<CPublishedFile_Unsubscribe_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPublishedFile_Unsubscribe_Response,
        };
        unsafe {
            instance.get(CPublishedFile_Unsubscribe_Response::new)
        }
    }
}

impl ::protobuf::Message for CPublishedFile_Unsubscribe_Response {
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

impl ::protobuf::MessageStatic for CPublishedFile_Unsubscribe_Response {
    fn new() -> CPublishedFile_Unsubscribe_Response {
        CPublishedFile_Unsubscribe_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPublishedFile_Unsubscribe_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CPublishedFile_Unsubscribe_Response>(
                    "CPublishedFile_Unsubscribe_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPublishedFile_Unsubscribe_Response {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPublishedFile_Unsubscribe_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPublishedFile_Unsubscribe_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CPublishedFile_Publish_Request {
    // message fields
    appid: ::std::option::Option<u32>,
    consumer_appid: ::std::option::Option<u32>,
    cloudfilename: ::protobuf::SingularField<::std::string::String>,
    preview_cloudfilename: ::protobuf::SingularField<::std::string::String>,
    title: ::protobuf::SingularField<::std::string::String>,
    file_description: ::protobuf::SingularField<::std::string::String>,
    file_type: ::std::option::Option<u32>,
    consumer_shortcut_name: ::protobuf::SingularField<::std::string::String>,
    youtube_username: ::protobuf::SingularField<::std::string::String>,
    youtube_videoid: ::protobuf::SingularField<::std::string::String>,
    visibility: ::std::option::Option<u32>,
    redirect_uri: ::protobuf::SingularField<::std::string::String>,
    tags: ::protobuf::RepeatedField<::std::string::String>,
    collection_type: ::protobuf::SingularField<::std::string::String>,
    game_type: ::protobuf::SingularField<::std::string::String>,
    url: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPublishedFile_Publish_Request {}

impl CPublishedFile_Publish_Request {
    pub fn new() -> CPublishedFile_Publish_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPublishedFile_Publish_Request {
        static mut instance: ::protobuf::lazy::Lazy<CPublishedFile_Publish_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPublishedFile_Publish_Request,
        };
        unsafe {
            instance.get(CPublishedFile_Publish_Request::new)
        }
    }

    // optional uint32 appid = 1;

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: u32) {
        self.appid = ::std::option::Option::Some(v);
    }

    pub fn get_appid(&self) -> u32 {
        self.appid.unwrap_or(0)
    }

    fn get_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.appid
    }

    fn mut_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.appid
    }

    // optional uint32 consumer_appid = 2;

    pub fn clear_consumer_appid(&mut self) {
        self.consumer_appid = ::std::option::Option::None;
    }

    pub fn has_consumer_appid(&self) -> bool {
        self.consumer_appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_consumer_appid(&mut self, v: u32) {
        self.consumer_appid = ::std::option::Option::Some(v);
    }

    pub fn get_consumer_appid(&self) -> u32 {
        self.consumer_appid.unwrap_or(0)
    }

    fn get_consumer_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.consumer_appid
    }

    fn mut_consumer_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.consumer_appid
    }

    // optional string cloudfilename = 3;

    pub fn clear_cloudfilename(&mut self) {
        self.cloudfilename.clear();
    }

    pub fn has_cloudfilename(&self) -> bool {
        self.cloudfilename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cloudfilename(&mut self, v: ::std::string::String) {
        self.cloudfilename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cloudfilename(&mut self) -> &mut ::std::string::String {
        if self.cloudfilename.is_none() {
            self.cloudfilename.set_default();
        }
        self.cloudfilename.as_mut().unwrap()
    }

    // Take field
    pub fn take_cloudfilename(&mut self) -> ::std::string::String {
        self.cloudfilename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cloudfilename(&self) -> &str {
        match self.cloudfilename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_cloudfilename_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.cloudfilename
    }

    fn mut_cloudfilename_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.cloudfilename
    }

    // optional string preview_cloudfilename = 4;

    pub fn clear_preview_cloudfilename(&mut self) {
        self.preview_cloudfilename.clear();
    }

    pub fn has_preview_cloudfilename(&self) -> bool {
        self.preview_cloudfilename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_preview_cloudfilename(&mut self, v: ::std::string::String) {
        self.preview_cloudfilename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_preview_cloudfilename(&mut self) -> &mut ::std::string::String {
        if self.preview_cloudfilename.is_none() {
            self.preview_cloudfilename.set_default();
        }
        self.preview_cloudfilename.as_mut().unwrap()
    }

    // Take field
    pub fn take_preview_cloudfilename(&mut self) -> ::std::string::String {
        self.preview_cloudfilename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_preview_cloudfilename(&self) -> &str {
        match self.preview_cloudfilename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_preview_cloudfilename_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.preview_cloudfilename
    }

    fn mut_preview_cloudfilename_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.preview_cloudfilename
    }

    // optional string title = 5;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    pub fn has_title(&self) -> bool {
        self.title.is_some()
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        if self.title.is_none() {
            self.title.set_default();
        }
        self.title.as_mut().unwrap()
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        self.title.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        match self.title.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_title_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.title
    }

    // optional string file_description = 6;

    pub fn clear_file_description(&mut self) {
        self.file_description.clear();
    }

    pub fn has_file_description(&self) -> bool {
        self.file_description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_description(&mut self, v: ::std::string::String) {
        self.file_description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_description(&mut self) -> &mut ::std::string::String {
        if self.file_description.is_none() {
            self.file_description.set_default();
        }
        self.file_description.as_mut().unwrap()
    }

    // Take field
    pub fn take_file_description(&mut self) -> ::std::string::String {
        self.file_description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_file_description(&self) -> &str {
        match self.file_description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_file_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.file_description
    }

    fn mut_file_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.file_description
    }

    // optional uint32 file_type = 7;

    pub fn clear_file_type(&mut self) {
        self.file_type = ::std::option::Option::None;
    }

    pub fn has_file_type(&self) -> bool {
        self.file_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_type(&mut self, v: u32) {
        self.file_type = ::std::option::Option::Some(v);
    }

    pub fn get_file_type(&self) -> u32 {
        self.file_type.unwrap_or(0)
    }

    fn get_file_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.file_type
    }

    fn mut_file_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.file_type
    }

    // optional string consumer_shortcut_name = 8;

    pub fn clear_consumer_shortcut_name(&mut self) {
        self.consumer_shortcut_name.clear();
    }

    pub fn has_consumer_shortcut_name(&self) -> bool {
        self.consumer_shortcut_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_consumer_shortcut_name(&mut self, v: ::std::string::String) {
        self.consumer_shortcut_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_consumer_shortcut_name(&mut self) -> &mut ::std::string::String {
        if self.consumer_shortcut_name.is_none() {
            self.consumer_shortcut_name.set_default();
        }
        self.consumer_shortcut_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_consumer_shortcut_name(&mut self) -> ::std::string::String {
        self.consumer_shortcut_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_consumer_shortcut_name(&self) -> &str {
        match self.consumer_shortcut_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_consumer_shortcut_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.consumer_shortcut_name
    }

    fn mut_consumer_shortcut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.consumer_shortcut_name
    }

    // optional string youtube_username = 9;

    pub fn clear_youtube_username(&mut self) {
        self.youtube_username.clear();
    }

    pub fn has_youtube_username(&self) -> bool {
        self.youtube_username.is_some()
    }

    // Param is passed by value, moved
    pub fn set_youtube_username(&mut self, v: ::std::string::String) {
        self.youtube_username = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_youtube_username(&mut self) -> &mut ::std::string::String {
        if self.youtube_username.is_none() {
            self.youtube_username.set_default();
        }
        self.youtube_username.as_mut().unwrap()
    }

    // Take field
    pub fn take_youtube_username(&mut self) -> ::std::string::String {
        self.youtube_username.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_youtube_username(&self) -> &str {
        match self.youtube_username.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_youtube_username_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.youtube_username
    }

    fn mut_youtube_username_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.youtube_username
    }

    // optional string youtube_videoid = 10;

    pub fn clear_youtube_videoid(&mut self) {
        self.youtube_videoid.clear();
    }

    pub fn has_youtube_videoid(&self) -> bool {
        self.youtube_videoid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_youtube_videoid(&mut self, v: ::std::string::String) {
        self.youtube_videoid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_youtube_videoid(&mut self) -> &mut ::std::string::String {
        if self.youtube_videoid.is_none() {
            self.youtube_videoid.set_default();
        }
        self.youtube_videoid.as_mut().unwrap()
    }

    // Take field
    pub fn take_youtube_videoid(&mut self) -> ::std::string::String {
        self.youtube_videoid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_youtube_videoid(&self) -> &str {
        match self.youtube_videoid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_youtube_videoid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.youtube_videoid
    }

    fn mut_youtube_videoid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.youtube_videoid
    }

    // optional uint32 visibility = 11;

    pub fn clear_visibility(&mut self) {
        self.visibility = ::std::option::Option::None;
    }

    pub fn has_visibility(&self) -> bool {
        self.visibility.is_some()
    }

    // Param is passed by value, moved
    pub fn set_visibility(&mut self, v: u32) {
        self.visibility = ::std::option::Option::Some(v);
    }

    pub fn get_visibility(&self) -> u32 {
        self.visibility.unwrap_or(0)
    }

    fn get_visibility_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.visibility
    }

    fn mut_visibility_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.visibility
    }

    // optional string redirect_uri = 12;

    pub fn clear_redirect_uri(&mut self) {
        self.redirect_uri.clear();
    }

    pub fn has_redirect_uri(&self) -> bool {
        self.redirect_uri.is_some()
    }

    // Param is passed by value, moved
    pub fn set_redirect_uri(&mut self, v: ::std::string::String) {
        self.redirect_uri = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_redirect_uri(&mut self) -> &mut ::std::string::String {
        if self.redirect_uri.is_none() {
            self.redirect_uri.set_default();
        }
        self.redirect_uri.as_mut().unwrap()
    }

    // Take field
    pub fn take_redirect_uri(&mut self) -> ::std::string::String {
        self.redirect_uri.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_redirect_uri(&self) -> &str {
        match self.redirect_uri.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_redirect_uri_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.redirect_uri
    }

    fn mut_redirect_uri_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.redirect_uri
    }

    // repeated string tags = 13;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags(&self) -> &[::std::string::String] {
        &self.tags
    }

    fn get_tags_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.tags
    }

    fn mut_tags_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tags
    }

    // optional string collection_type = 14;

    pub fn clear_collection_type(&mut self) {
        self.collection_type.clear();
    }

    pub fn has_collection_type(&self) -> bool {
        self.collection_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collection_type(&mut self, v: ::std::string::String) {
        self.collection_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_collection_type(&mut self) -> &mut ::std::string::String {
        if self.collection_type.is_none() {
            self.collection_type.set_default();
        }
        self.collection_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_collection_type(&mut self) -> ::std::string::String {
        self.collection_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_collection_type(&self) -> &str {
        match self.collection_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_collection_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.collection_type
    }

    fn mut_collection_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.collection_type
    }

    // optional string game_type = 15;

    pub fn clear_game_type(&mut self) {
        self.game_type.clear();
    }

    pub fn has_game_type(&self) -> bool {
        self.game_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_type(&mut self, v: ::std::string::String) {
        self.game_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_type(&mut self) -> &mut ::std::string::String {
        if self.game_type.is_none() {
            self.game_type.set_default();
        }
        self.game_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_type(&mut self) -> ::std::string::String {
        self.game_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_game_type(&self) -> &str {
        match self.game_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_game_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.game_type
    }

    fn mut_game_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.game_type
    }

    // optional string url = 16;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        if self.url.is_none() {
            self.url.set_default();
        }
        self.url.as_mut().unwrap()
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        self.url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        match self.url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.url
    }

    fn mut_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.url
    }
}

impl ::protobuf::Message for CPublishedFile_Publish_Request {
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
                    self.appid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.consumer_appid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cloudfilename)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.preview_cloudfilename)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.title)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.file_description)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.file_type = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.consumer_shortcut_name)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.youtube_username)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.youtube_videoid)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.visibility = ::std::option::Option::Some(tmp);
                },
                12 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.redirect_uri)?;
                },
                13 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.tags)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.collection_type)?;
                },
                15 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.game_type)?;
                },
                16 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url)?;
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
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.consumer_appid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.cloudfilename.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.preview_cloudfilename.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.title.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.file_description.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(v) = self.file_type {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.consumer_shortcut_name.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(ref v) = self.youtube_username.as_ref() {
            my_size += ::protobuf::rt::string_size(9, &v);
        }
        if let Some(ref v) = self.youtube_videoid.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        if let Some(v) = self.visibility {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.redirect_uri.as_ref() {
            my_size += ::protobuf::rt::string_size(12, &v);
        }
        for value in &self.tags {
            my_size += ::protobuf::rt::string_size(13, &value);
        };
        if let Some(ref v) = self.collection_type.as_ref() {
            my_size += ::protobuf::rt::string_size(14, &v);
        }
        if let Some(ref v) = self.game_type.as_ref() {
            my_size += ::protobuf::rt::string_size(15, &v);
        }
        if let Some(ref v) = self.url.as_ref() {
            my_size += ::protobuf::rt::string_size(16, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.appid {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.consumer_appid {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.cloudfilename.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.preview_cloudfilename.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.title.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.file_description.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(v) = self.file_type {
            os.write_uint32(7, v)?;
        }
        if let Some(ref v) = self.consumer_shortcut_name.as_ref() {
            os.write_string(8, &v)?;
        }
        if let Some(ref v) = self.youtube_username.as_ref() {
            os.write_string(9, &v)?;
        }
        if let Some(ref v) = self.youtube_videoid.as_ref() {
            os.write_string(10, &v)?;
        }
        if let Some(v) = self.visibility {
            os.write_uint32(11, v)?;
        }
        if let Some(ref v) = self.redirect_uri.as_ref() {
            os.write_string(12, &v)?;
        }
        for v in &self.tags {
            os.write_string(13, &v)?;
        };
        if let Some(ref v) = self.collection_type.as_ref() {
            os.write_string(14, &v)?;
        }
        if let Some(ref v) = self.game_type.as_ref() {
            os.write_string(15, &v)?;
        }
        if let Some(ref v) = self.url.as_ref() {
            os.write_string(16, &v)?;
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

impl ::protobuf::MessageStatic for CPublishedFile_Publish_Request {
    fn new() -> CPublishedFile_Publish_Request {
        CPublishedFile_Publish_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPublishedFile_Publish_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CPublishedFile_Publish_Request::get_appid_for_reflect,
                    CPublishedFile_Publish_Request::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "consumer_appid",
                    CPublishedFile_Publish_Request::get_consumer_appid_for_reflect,
                    CPublishedFile_Publish_Request::mut_consumer_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cloudfilename",
                    CPublishedFile_Publish_Request::get_cloudfilename_for_reflect,
                    CPublishedFile_Publish_Request::mut_cloudfilename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "preview_cloudfilename",
                    CPublishedFile_Publish_Request::get_preview_cloudfilename_for_reflect,
                    CPublishedFile_Publish_Request::mut_preview_cloudfilename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    CPublishedFile_Publish_Request::get_title_for_reflect,
                    CPublishedFile_Publish_Request::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "file_description",
                    CPublishedFile_Publish_Request::get_file_description_for_reflect,
                    CPublishedFile_Publish_Request::mut_file_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "file_type",
                    CPublishedFile_Publish_Request::get_file_type_for_reflect,
                    CPublishedFile_Publish_Request::mut_file_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "consumer_shortcut_name",
                    CPublishedFile_Publish_Request::get_consumer_shortcut_name_for_reflect,
                    CPublishedFile_Publish_Request::mut_consumer_shortcut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "youtube_username",
                    CPublishedFile_Publish_Request::get_youtube_username_for_reflect,
                    CPublishedFile_Publish_Request::mut_youtube_username_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "youtube_videoid",
                    CPublishedFile_Publish_Request::get_youtube_videoid_for_reflect,
                    CPublishedFile_Publish_Request::mut_youtube_videoid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "visibility",
                    CPublishedFile_Publish_Request::get_visibility_for_reflect,
                    CPublishedFile_Publish_Request::mut_visibility_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "redirect_uri",
                    CPublishedFile_Publish_Request::get_redirect_uri_for_reflect,
                    CPublishedFile_Publish_Request::mut_redirect_uri_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tags",
                    CPublishedFile_Publish_Request::get_tags_for_reflect,
                    CPublishedFile_Publish_Request::mut_tags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "collection_type",
                    CPublishedFile_Publish_Request::get_collection_type_for_reflect,
                    CPublishedFile_Publish_Request::mut_collection_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "game_type",
                    CPublishedFile_Publish_Request::get_game_type_for_reflect,
                    CPublishedFile_Publish_Request::mut_game_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url",
                    CPublishedFile_Publish_Request::get_url_for_reflect,
                    CPublishedFile_Publish_Request::mut_url_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CPublishedFile_Publish_Request>(
                    "CPublishedFile_Publish_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPublishedFile_Publish_Request {
    fn clear(&mut self) {
        self.clear_appid();
        self.clear_consumer_appid();
        self.clear_cloudfilename();
        self.clear_preview_cloudfilename();
        self.clear_title();
        self.clear_file_description();
        self.clear_file_type();
        self.clear_consumer_shortcut_name();
        self.clear_youtube_username();
        self.clear_youtube_videoid();
        self.clear_visibility();
        self.clear_redirect_uri();
        self.clear_tags();
        self.clear_collection_type();
        self.clear_game_type();
        self.clear_url();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPublishedFile_Publish_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPublishedFile_Publish_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CPublishedFile_Publish_Response {
    // message fields
    publishedfileid: ::std::option::Option<u64>,
    redirect_uri: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPublishedFile_Publish_Response {}

impl CPublishedFile_Publish_Response {
    pub fn new() -> CPublishedFile_Publish_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPublishedFile_Publish_Response {
        static mut instance: ::protobuf::lazy::Lazy<CPublishedFile_Publish_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPublishedFile_Publish_Response,
        };
        unsafe {
            instance.get(CPublishedFile_Publish_Response::new)
        }
    }

    // optional uint64 publishedfileid = 1;

    pub fn clear_publishedfileid(&mut self) {
        self.publishedfileid = ::std::option::Option::None;
    }

    pub fn has_publishedfileid(&self) -> bool {
        self.publishedfileid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publishedfileid(&mut self, v: u64) {
        self.publishedfileid = ::std::option::Option::Some(v);
    }

    pub fn get_publishedfileid(&self) -> u64 {
        self.publishedfileid.unwrap_or(0)
    }

    fn get_publishedfileid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.publishedfileid
    }

    fn mut_publishedfileid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.publishedfileid
    }

    // optional string redirect_uri = 2;

    pub fn clear_redirect_uri(&mut self) {
        self.redirect_uri.clear();
    }

    pub fn has_redirect_uri(&self) -> bool {
        self.redirect_uri.is_some()
    }

    // Param is passed by value, moved
    pub fn set_redirect_uri(&mut self, v: ::std::string::String) {
        self.redirect_uri = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_redirect_uri(&mut self) -> &mut ::std::string::String {
        if self.redirect_uri.is_none() {
            self.redirect_uri.set_default();
        }
        self.redirect_uri.as_mut().unwrap()
    }

    // Take field
    pub fn take_redirect_uri(&mut self) -> ::std::string::String {
        self.redirect_uri.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_redirect_uri(&self) -> &str {
        match self.redirect_uri.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_redirect_uri_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.redirect_uri
    }

    fn mut_redirect_uri_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.redirect_uri
    }
}

impl ::protobuf::Message for CPublishedFile_Publish_Response {
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
                    self.publishedfileid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.redirect_uri)?;
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
        if let Some(v) = self.publishedfileid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.redirect_uri.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.publishedfileid {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.redirect_uri.as_ref() {
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

impl ::protobuf::MessageStatic for CPublishedFile_Publish_Response {
    fn new() -> CPublishedFile_Publish_Response {
        CPublishedFile_Publish_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPublishedFile_Publish_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "publishedfileid",
                    CPublishedFile_Publish_Response::get_publishedfileid_for_reflect,
                    CPublishedFile_Publish_Response::mut_publishedfileid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "redirect_uri",
                    CPublishedFile_Publish_Response::get_redirect_uri_for_reflect,
                    CPublishedFile_Publish_Response::mut_redirect_uri_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CPublishedFile_Publish_Response>(
                    "CPublishedFile_Publish_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPublishedFile_Publish_Response {
    fn clear(&mut self) {
        self.clear_publishedfileid();
        self.clear_redirect_uri();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPublishedFile_Publish_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPublishedFile_Publish_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CPublishedFile_GetDetails_Request {
    // message fields
    publishedfileids: ::std::vec::Vec<u64>,
    includetags: ::std::option::Option<bool>,
    includeadditionalpreviews: ::std::option::Option<bool>,
    includechildren: ::std::option::Option<bool>,
    includekvtags: ::std::option::Option<bool>,
    includevotes: ::std::option::Option<bool>,
    short_description: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPublishedFile_GetDetails_Request {}

impl CPublishedFile_GetDetails_Request {
    pub fn new() -> CPublishedFile_GetDetails_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPublishedFile_GetDetails_Request {
        static mut instance: ::protobuf::lazy::Lazy<CPublishedFile_GetDetails_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPublishedFile_GetDetails_Request,
        };
        unsafe {
            instance.get(CPublishedFile_GetDetails_Request::new)
        }
    }

    // repeated fixed64 publishedfileids = 1;

    pub fn clear_publishedfileids(&mut self) {
        self.publishedfileids.clear();
    }

    // Param is passed by value, moved
    pub fn set_publishedfileids(&mut self, v: ::std::vec::Vec<u64>) {
        self.publishedfileids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_publishedfileids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.publishedfileids
    }

    // Take field
    pub fn take_publishedfileids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.publishedfileids, ::std::vec::Vec::new())
    }

    pub fn get_publishedfileids(&self) -> &[u64] {
        &self.publishedfileids
    }

    fn get_publishedfileids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.publishedfileids
    }

    fn mut_publishedfileids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.publishedfileids
    }

    // optional bool includetags = 2;

    pub fn clear_includetags(&mut self) {
        self.includetags = ::std::option::Option::None;
    }

    pub fn has_includetags(&self) -> bool {
        self.includetags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_includetags(&mut self, v: bool) {
        self.includetags = ::std::option::Option::Some(v);
    }

    pub fn get_includetags(&self) -> bool {
        self.includetags.unwrap_or(false)
    }

    fn get_includetags_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.includetags
    }

    fn mut_includetags_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.includetags
    }

    // optional bool includeadditionalpreviews = 3;

    pub fn clear_includeadditionalpreviews(&mut self) {
        self.includeadditionalpreviews = ::std::option::Option::None;
    }

    pub fn has_includeadditionalpreviews(&self) -> bool {
        self.includeadditionalpreviews.is_some()
    }

    // Param is passed by value, moved
    pub fn set_includeadditionalpreviews(&mut self, v: bool) {
        self.includeadditionalpreviews = ::std::option::Option::Some(v);
    }

    pub fn get_includeadditionalpreviews(&self) -> bool {
        self.includeadditionalpreviews.unwrap_or(false)
    }

    fn get_includeadditionalpreviews_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.includeadditionalpreviews
    }

    fn mut_includeadditionalpreviews_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.includeadditionalpreviews
    }

    // optional bool includechildren = 4;

    pub fn clear_includechildren(&mut self) {
        self.includechildren = ::std::option::Option::None;
    }

    pub fn has_includechildren(&self) -> bool {
        self.includechildren.is_some()
    }

    // Param is passed by value, moved
    pub fn set_includechildren(&mut self, v: bool) {
        self.includechildren = ::std::option::Option::Some(v);
    }

    pub fn get_includechildren(&self) -> bool {
        self.includechildren.unwrap_or(false)
    }

    fn get_includechildren_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.includechildren
    }

    fn mut_includechildren_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.includechildren
    }

    // optional bool includekvtags = 5;

    pub fn clear_includekvtags(&mut self) {
        self.includekvtags = ::std::option::Option::None;
    }

    pub fn has_includekvtags(&self) -> bool {
        self.includekvtags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_includekvtags(&mut self, v: bool) {
        self.includekvtags = ::std::option::Option::Some(v);
    }

    pub fn get_includekvtags(&self) -> bool {
        self.includekvtags.unwrap_or(false)
    }

    fn get_includekvtags_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.includekvtags
    }

    fn mut_includekvtags_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.includekvtags
    }

    // optional bool includevotes = 6;

    pub fn clear_includevotes(&mut self) {
        self.includevotes = ::std::option::Option::None;
    }

    pub fn has_includevotes(&self) -> bool {
        self.includevotes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_includevotes(&mut self, v: bool) {
        self.includevotes = ::std::option::Option::Some(v);
    }

    pub fn get_includevotes(&self) -> bool {
        self.includevotes.unwrap_or(false)
    }

    fn get_includevotes_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.includevotes
    }

    fn mut_includevotes_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.includevotes
    }

    // optional bool short_description = 8;

    pub fn clear_short_description(&mut self) {
        self.short_description = ::std::option::Option::None;
    }

    pub fn has_short_description(&self) -> bool {
        self.short_description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_short_description(&mut self, v: bool) {
        self.short_description = ::std::option::Option::Some(v);
    }

    pub fn get_short_description(&self) -> bool {
        self.short_description.unwrap_or(false)
    }

    fn get_short_description_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.short_description
    }

    fn mut_short_description_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.short_description
    }
}

impl ::protobuf::Message for CPublishedFile_GetDetails_Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.publishedfileids)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.includetags = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.includeadditionalpreviews = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.includechildren = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.includekvtags = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.includevotes = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.short_description = ::std::option::Option::Some(tmp);
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
        my_size += 9 * self.publishedfileids.len() as u32;
        if let Some(v) = self.includetags {
            my_size += 2;
        }
        if let Some(v) = self.includeadditionalpreviews {
            my_size += 2;
        }
        if let Some(v) = self.includechildren {
            my_size += 2;
        }
        if let Some(v) = self.includekvtags {
            my_size += 2;
        }
        if let Some(v) = self.includevotes {
            my_size += 2;
        }
        if let Some(v) = self.short_description {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.publishedfileids {
            os.write_fixed64(1, *v)?;
        };
        if let Some(v) = self.includetags {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.includeadditionalpreviews {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.includechildren {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.includekvtags {
            os.write_bool(5, v)?;
        }
        if let Some(v) = self.includevotes {
            os.write_bool(6, v)?;
        }
        if let Some(v) = self.short_description {
            os.write_bool(8, v)?;
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

impl ::protobuf::MessageStatic for CPublishedFile_GetDetails_Request {
    fn new() -> CPublishedFile_GetDetails_Request {
        CPublishedFile_GetDetails_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPublishedFile_GetDetails_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "publishedfileids",
                    CPublishedFile_GetDetails_Request::get_publishedfileids_for_reflect,
                    CPublishedFile_GetDetails_Request::mut_publishedfileids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "includetags",
                    CPublishedFile_GetDetails_Request::get_includetags_for_reflect,
                    CPublishedFile_GetDetails_Request::mut_includetags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "includeadditionalpreviews",
                    CPublishedFile_GetDetails_Request::get_includeadditionalpreviews_for_reflect,
                    CPublishedFile_GetDetails_Request::mut_includeadditionalpreviews_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "includechildren",
                    CPublishedFile_GetDetails_Request::get_includechildren_for_reflect,
                    CPublishedFile_GetDetails_Request::mut_includechildren_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "includekvtags",
                    CPublishedFile_GetDetails_Request::get_includekvtags_for_reflect,
                    CPublishedFile_GetDetails_Request::mut_includekvtags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "includevotes",
                    CPublishedFile_GetDetails_Request::get_includevotes_for_reflect,
                    CPublishedFile_GetDetails_Request::mut_includevotes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "short_description",
                    CPublishedFile_GetDetails_Request::get_short_description_for_reflect,
                    CPublishedFile_GetDetails_Request::mut_short_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CPublishedFile_GetDetails_Request>(
                    "CPublishedFile_GetDetails_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPublishedFile_GetDetails_Request {
    fn clear(&mut self) {
        self.clear_publishedfileids();
        self.clear_includetags();
        self.clear_includeadditionalpreviews();
        self.clear_includechildren();
        self.clear_includekvtags();
        self.clear_includevotes();
        self.clear_short_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPublishedFile_GetDetails_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPublishedFile_GetDetails_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PublishedFileDetails {
    // message fields
    result: ::std::option::Option<u32>,
    publishedfileid: ::std::option::Option<u64>,
    creator: ::std::option::Option<u64>,
    creator_appid: ::std::option::Option<u32>,
    consumer_appid: ::std::option::Option<u32>,
    consumer_shortcutid: ::std::option::Option<u32>,
    filename: ::protobuf::SingularField<::std::string::String>,
    file_size: ::std::option::Option<u64>,
    preview_file_size: ::std::option::Option<u64>,
    file_url: ::protobuf::SingularField<::std::string::String>,
    preview_url: ::protobuf::SingularField<::std::string::String>,
    youtubevideoid: ::protobuf::SingularField<::std::string::String>,
    url: ::protobuf::SingularField<::std::string::String>,
    hcontent_file: ::std::option::Option<u64>,
    hcontent_preview: ::std::option::Option<u64>,
    title: ::protobuf::SingularField<::std::string::String>,
    file_description: ::protobuf::SingularField<::std::string::String>,
    short_description: ::protobuf::SingularField<::std::string::String>,
    time_created: ::std::option::Option<u32>,
    time_updated: ::std::option::Option<u32>,
    visibility: ::std::option::Option<u32>,
    flags: ::std::option::Option<u32>,
    workshop_file: ::std::option::Option<bool>,
    workshop_accepted: ::std::option::Option<bool>,
    show_subscribe_all: ::std::option::Option<bool>,
    num_comments_developer: ::std::option::Option<i32>,
    num_comments_public: ::std::option::Option<i32>,
    banned: ::std::option::Option<bool>,
    ban_reason: ::protobuf::SingularField<::std::string::String>,
    banner: ::std::option::Option<u64>,
    can_be_deleted: ::std::option::Option<bool>,
    incompatible: ::std::option::Option<bool>,
    app_name: ::protobuf::SingularField<::std::string::String>,
    file_type: ::std::option::Option<u32>,
    can_subscribe: ::std::option::Option<bool>,
    subscriptions: ::std::option::Option<u32>,
    favorited: ::std::option::Option<u32>,
    followers: ::std::option::Option<u32>,
    lifetime_subscriptions: ::std::option::Option<u32>,
    lifetime_favorited: ::std::option::Option<u32>,
    lifetime_followers: ::std::option::Option<u32>,
    views: ::std::option::Option<u32>,
    image_width: ::std::option::Option<u32>,
    image_height: ::std::option::Option<u32>,
    image_url: ::protobuf::SingularField<::std::string::String>,
    spoiler_tag: ::std::option::Option<bool>,
    shortcutid: ::std::option::Option<u32>,
    shortcutname: ::protobuf::SingularField<::std::string::String>,
    num_children: ::std::option::Option<u32>,
    num_reports: ::std::option::Option<u32>,
    previews: ::protobuf::RepeatedField<PublishedFileDetails_Preview>,
    tags: ::protobuf::RepeatedField<PublishedFileDetails_Tag>,
    children: ::protobuf::RepeatedField<PublishedFileDetails_Child>,
    kvtags: ::protobuf::RepeatedField<PublishedFileDetails_KVTag>,
    vote_data: ::protobuf::SingularPtrField<PublishedFileDetails_VoteData>,
    time_subscribed: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PublishedFileDetails {}

impl PublishedFileDetails {
    pub fn new() -> PublishedFileDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublishedFileDetails {
        static mut instance: ::protobuf::lazy::Lazy<PublishedFileDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublishedFileDetails,
        };
        unsafe {
            instance.get(PublishedFileDetails::new)
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

    // optional uint64 publishedfileid = 2;

    pub fn clear_publishedfileid(&mut self) {
        self.publishedfileid = ::std::option::Option::None;
    }

    pub fn has_publishedfileid(&self) -> bool {
        self.publishedfileid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publishedfileid(&mut self, v: u64) {
        self.publishedfileid = ::std::option::Option::Some(v);
    }

    pub fn get_publishedfileid(&self) -> u64 {
        self.publishedfileid.unwrap_or(0)
    }

    fn get_publishedfileid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.publishedfileid
    }

    fn mut_publishedfileid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.publishedfileid
    }

    // optional fixed64 creator = 3;

    pub fn clear_creator(&mut self) {
        self.creator = ::std::option::Option::None;
    }

    pub fn has_creator(&self) -> bool {
        self.creator.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creator(&mut self, v: u64) {
        self.creator = ::std::option::Option::Some(v);
    }

    pub fn get_creator(&self) -> u64 {
        self.creator.unwrap_or(0)
    }

    fn get_creator_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.creator
    }

    fn mut_creator_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.creator
    }

    // optional uint32 creator_appid = 4;

    pub fn clear_creator_appid(&mut self) {
        self.creator_appid = ::std::option::Option::None;
    }

    pub fn has_creator_appid(&self) -> bool {
        self.creator_appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creator_appid(&mut self, v: u32) {
        self.creator_appid = ::std::option::Option::Some(v);
    }

    pub fn get_creator_appid(&self) -> u32 {
        self.creator_appid.unwrap_or(0)
    }

    fn get_creator_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.creator_appid
    }

    fn mut_creator_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.creator_appid
    }

    // optional uint32 consumer_appid = 5;

    pub fn clear_consumer_appid(&mut self) {
        self.consumer_appid = ::std::option::Option::None;
    }

    pub fn has_consumer_appid(&self) -> bool {
        self.consumer_appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_consumer_appid(&mut self, v: u32) {
        self.consumer_appid = ::std::option::Option::Some(v);
    }

    pub fn get_consumer_appid(&self) -> u32 {
        self.consumer_appid.unwrap_or(0)
    }

    fn get_consumer_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.consumer_appid
    }

    fn mut_consumer_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.consumer_appid
    }

    // optional uint32 consumer_shortcutid = 6;

    pub fn clear_consumer_shortcutid(&mut self) {
        self.consumer_shortcutid = ::std::option::Option::None;
    }

    pub fn has_consumer_shortcutid(&self) -> bool {
        self.consumer_shortcutid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_consumer_shortcutid(&mut self, v: u32) {
        self.consumer_shortcutid = ::std::option::Option::Some(v);
    }

    pub fn get_consumer_shortcutid(&self) -> u32 {
        self.consumer_shortcutid.unwrap_or(0)
    }

    fn get_consumer_shortcutid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.consumer_shortcutid
    }

    fn mut_consumer_shortcutid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.consumer_shortcutid
    }

    // optional string filename = 7;

    pub fn clear_filename(&mut self) {
        self.filename.clear();
    }

    pub fn has_filename(&self) -> bool {
        self.filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filename(&mut self, v: ::std::string::String) {
        self.filename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filename(&mut self) -> &mut ::std::string::String {
        if self.filename.is_none() {
            self.filename.set_default();
        }
        self.filename.as_mut().unwrap()
    }

    // Take field
    pub fn take_filename(&mut self) -> ::std::string::String {
        self.filename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_filename(&self) -> &str {
        match self.filename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_filename_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.filename
    }

    fn mut_filename_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.filename
    }

    // optional uint64 file_size = 8;

    pub fn clear_file_size(&mut self) {
        self.file_size = ::std::option::Option::None;
    }

    pub fn has_file_size(&self) -> bool {
        self.file_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_size(&mut self, v: u64) {
        self.file_size = ::std::option::Option::Some(v);
    }

    pub fn get_file_size(&self) -> u64 {
        self.file_size.unwrap_or(0)
    }

    fn get_file_size_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.file_size
    }

    fn mut_file_size_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.file_size
    }

    // optional uint64 preview_file_size = 9;

    pub fn clear_preview_file_size(&mut self) {
        self.preview_file_size = ::std::option::Option::None;
    }

    pub fn has_preview_file_size(&self) -> bool {
        self.preview_file_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_preview_file_size(&mut self, v: u64) {
        self.preview_file_size = ::std::option::Option::Some(v);
    }

    pub fn get_preview_file_size(&self) -> u64 {
        self.preview_file_size.unwrap_or(0)
    }

    fn get_preview_file_size_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.preview_file_size
    }

    fn mut_preview_file_size_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.preview_file_size
    }

    // optional string file_url = 10;

    pub fn clear_file_url(&mut self) {
        self.file_url.clear();
    }

    pub fn has_file_url(&self) -> bool {
        self.file_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_url(&mut self, v: ::std::string::String) {
        self.file_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_url(&mut self) -> &mut ::std::string::String {
        if self.file_url.is_none() {
            self.file_url.set_default();
        }
        self.file_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_file_url(&mut self) -> ::std::string::String {
        self.file_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_file_url(&self) -> &str {
        match self.file_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_file_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.file_url
    }

    fn mut_file_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.file_url
    }

    // optional string preview_url = 11;

    pub fn clear_preview_url(&mut self) {
        self.preview_url.clear();
    }

    pub fn has_preview_url(&self) -> bool {
        self.preview_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_preview_url(&mut self, v: ::std::string::String) {
        self.preview_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_preview_url(&mut self) -> &mut ::std::string::String {
        if self.preview_url.is_none() {
            self.preview_url.set_default();
        }
        self.preview_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_preview_url(&mut self) -> ::std::string::String {
        self.preview_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_preview_url(&self) -> &str {
        match self.preview_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_preview_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.preview_url
    }

    fn mut_preview_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.preview_url
    }

    // optional string youtubevideoid = 12;

    pub fn clear_youtubevideoid(&mut self) {
        self.youtubevideoid.clear();
    }

    pub fn has_youtubevideoid(&self) -> bool {
        self.youtubevideoid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_youtubevideoid(&mut self, v: ::std::string::String) {
        self.youtubevideoid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_youtubevideoid(&mut self) -> &mut ::std::string::String {
        if self.youtubevideoid.is_none() {
            self.youtubevideoid.set_default();
        }
        self.youtubevideoid.as_mut().unwrap()
    }

    // Take field
    pub fn take_youtubevideoid(&mut self) -> ::std::string::String {
        self.youtubevideoid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_youtubevideoid(&self) -> &str {
        match self.youtubevideoid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_youtubevideoid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.youtubevideoid
    }

    fn mut_youtubevideoid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.youtubevideoid
    }

    // optional string url = 13;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        if self.url.is_none() {
            self.url.set_default();
        }
        self.url.as_mut().unwrap()
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        self.url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        match self.url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.url
    }

    fn mut_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.url
    }

    // optional fixed64 hcontent_file = 14;

    pub fn clear_hcontent_file(&mut self) {
        self.hcontent_file = ::std::option::Option::None;
    }

    pub fn has_hcontent_file(&self) -> bool {
        self.hcontent_file.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hcontent_file(&mut self, v: u64) {
        self.hcontent_file = ::std::option::Option::Some(v);
    }

    pub fn get_hcontent_file(&self) -> u64 {
        self.hcontent_file.unwrap_or(0)
    }

    fn get_hcontent_file_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.hcontent_file
    }

    fn mut_hcontent_file_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.hcontent_file
    }

    // optional fixed64 hcontent_preview = 15;

    pub fn clear_hcontent_preview(&mut self) {
        self.hcontent_preview = ::std::option::Option::None;
    }

    pub fn has_hcontent_preview(&self) -> bool {
        self.hcontent_preview.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hcontent_preview(&mut self, v: u64) {
        self.hcontent_preview = ::std::option::Option::Some(v);
    }

    pub fn get_hcontent_preview(&self) -> u64 {
        self.hcontent_preview.unwrap_or(0)
    }

    fn get_hcontent_preview_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.hcontent_preview
    }

    fn mut_hcontent_preview_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.hcontent_preview
    }

    // optional string title = 16;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    pub fn has_title(&self) -> bool {
        self.title.is_some()
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        if self.title.is_none() {
            self.title.set_default();
        }
        self.title.as_mut().unwrap()
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        self.title.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        match self.title.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_title_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.title
    }

    // optional string file_description = 17;

    pub fn clear_file_description(&mut self) {
        self.file_description.clear();
    }

    pub fn has_file_description(&self) -> bool {
        self.file_description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_description(&mut self, v: ::std::string::String) {
        self.file_description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_description(&mut self) -> &mut ::std::string::String {
        if self.file_description.is_none() {
            self.file_description.set_default();
        }
        self.file_description.as_mut().unwrap()
    }

    // Take field
    pub fn take_file_description(&mut self) -> ::std::string::String {
        self.file_description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_file_description(&self) -> &str {
        match self.file_description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_file_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.file_description
    }

    fn mut_file_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.file_description
    }

    // optional string short_description = 18;

    pub fn clear_short_description(&mut self) {
        self.short_description.clear();
    }

    pub fn has_short_description(&self) -> bool {
        self.short_description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_short_description(&mut self, v: ::std::string::String) {
        self.short_description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_short_description(&mut self) -> &mut ::std::string::String {
        if self.short_description.is_none() {
            self.short_description.set_default();
        }
        self.short_description.as_mut().unwrap()
    }

    // Take field
    pub fn take_short_description(&mut self) -> ::std::string::String {
        self.short_description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_short_description(&self) -> &str {
        match self.short_description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_short_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.short_description
    }

    fn mut_short_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.short_description
    }

    // optional uint32 time_created = 19;

    pub fn clear_time_created(&mut self) {
        self.time_created = ::std::option::Option::None;
    }

    pub fn has_time_created(&self) -> bool {
        self.time_created.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_created(&mut self, v: u32) {
        self.time_created = ::std::option::Option::Some(v);
    }

    pub fn get_time_created(&self) -> u32 {
        self.time_created.unwrap_or(0)
    }

    fn get_time_created_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_created
    }

    fn mut_time_created_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_created
    }

    // optional uint32 time_updated = 20;

    pub fn clear_time_updated(&mut self) {
        self.time_updated = ::std::option::Option::None;
    }

    pub fn has_time_updated(&self) -> bool {
        self.time_updated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_updated(&mut self, v: u32) {
        self.time_updated = ::std::option::Option::Some(v);
    }

    pub fn get_time_updated(&self) -> u32 {
        self.time_updated.unwrap_or(0)
    }

    fn get_time_updated_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_updated
    }

    fn mut_time_updated_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_updated
    }

    // optional uint32 visibility = 21;

    pub fn clear_visibility(&mut self) {
        self.visibility = ::std::option::Option::None;
    }

    pub fn has_visibility(&self) -> bool {
        self.visibility.is_some()
    }

    // Param is passed by value, moved
    pub fn set_visibility(&mut self, v: u32) {
        self.visibility = ::std::option::Option::Some(v);
    }

    pub fn get_visibility(&self) -> u32 {
        self.visibility.unwrap_or(0)
    }

    fn get_visibility_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.visibility
    }

    fn mut_visibility_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.visibility
    }

    // optional uint32 flags = 22;

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
        self.flags.unwrap_or(0)
    }

    fn get_flags_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.flags
    }

    fn mut_flags_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.flags
    }

    // optional bool workshop_file = 23;

    pub fn clear_workshop_file(&mut self) {
        self.workshop_file = ::std::option::Option::None;
    }

    pub fn has_workshop_file(&self) -> bool {
        self.workshop_file.is_some()
    }

    // Param is passed by value, moved
    pub fn set_workshop_file(&mut self, v: bool) {
        self.workshop_file = ::std::option::Option::Some(v);
    }

    pub fn get_workshop_file(&self) -> bool {
        self.workshop_file.unwrap_or(false)
    }

    fn get_workshop_file_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.workshop_file
    }

    fn mut_workshop_file_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.workshop_file
    }

    // optional bool workshop_accepted = 24;

    pub fn clear_workshop_accepted(&mut self) {
        self.workshop_accepted = ::std::option::Option::None;
    }

    pub fn has_workshop_accepted(&self) -> bool {
        self.workshop_accepted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_workshop_accepted(&mut self, v: bool) {
        self.workshop_accepted = ::std::option::Option::Some(v);
    }

    pub fn get_workshop_accepted(&self) -> bool {
        self.workshop_accepted.unwrap_or(false)
    }

    fn get_workshop_accepted_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.workshop_accepted
    }

    fn mut_workshop_accepted_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.workshop_accepted
    }

    // optional bool show_subscribe_all = 25;

    pub fn clear_show_subscribe_all(&mut self) {
        self.show_subscribe_all = ::std::option::Option::None;
    }

    pub fn has_show_subscribe_all(&self) -> bool {
        self.show_subscribe_all.is_some()
    }

    // Param is passed by value, moved
    pub fn set_show_subscribe_all(&mut self, v: bool) {
        self.show_subscribe_all = ::std::option::Option::Some(v);
    }

    pub fn get_show_subscribe_all(&self) -> bool {
        self.show_subscribe_all.unwrap_or(false)
    }

    fn get_show_subscribe_all_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.show_subscribe_all
    }

    fn mut_show_subscribe_all_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.show_subscribe_all
    }

    // optional int32 num_comments_developer = 26;

    pub fn clear_num_comments_developer(&mut self) {
        self.num_comments_developer = ::std::option::Option::None;
    }

    pub fn has_num_comments_developer(&self) -> bool {
        self.num_comments_developer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_comments_developer(&mut self, v: i32) {
        self.num_comments_developer = ::std::option::Option::Some(v);
    }

    pub fn get_num_comments_developer(&self) -> i32 {
        self.num_comments_developer.unwrap_or(0)
    }

    fn get_num_comments_developer_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_comments_developer
    }

    fn mut_num_comments_developer_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_comments_developer
    }

    // optional int32 num_comments_public = 27;

    pub fn clear_num_comments_public(&mut self) {
        self.num_comments_public = ::std::option::Option::None;
    }

    pub fn has_num_comments_public(&self) -> bool {
        self.num_comments_public.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_comments_public(&mut self, v: i32) {
        self.num_comments_public = ::std::option::Option::Some(v);
    }

    pub fn get_num_comments_public(&self) -> i32 {
        self.num_comments_public.unwrap_or(0)
    }

    fn get_num_comments_public_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_comments_public
    }

    fn mut_num_comments_public_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_comments_public
    }

    // optional bool banned = 28;

    pub fn clear_banned(&mut self) {
        self.banned = ::std::option::Option::None;
    }

    pub fn has_banned(&self) -> bool {
        self.banned.is_some()
    }

    // Param is passed by value, moved
    pub fn set_banned(&mut self, v: bool) {
        self.banned = ::std::option::Option::Some(v);
    }

    pub fn get_banned(&self) -> bool {
        self.banned.unwrap_or(false)
    }

    fn get_banned_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.banned
    }

    fn mut_banned_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.banned
    }

    // optional string ban_reason = 29;

    pub fn clear_ban_reason(&mut self) {
        self.ban_reason.clear();
    }

    pub fn has_ban_reason(&self) -> bool {
        self.ban_reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ban_reason(&mut self, v: ::std::string::String) {
        self.ban_reason = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ban_reason(&mut self) -> &mut ::std::string::String {
        if self.ban_reason.is_none() {
            self.ban_reason.set_default();
        }
        self.ban_reason.as_mut().unwrap()
    }

    // Take field
    pub fn take_ban_reason(&mut self) -> ::std::string::String {
        self.ban_reason.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ban_reason(&self) -> &str {
        match self.ban_reason.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ban_reason_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ban_reason
    }

    fn mut_ban_reason_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ban_reason
    }

    // optional fixed64 banner = 30;

    pub fn clear_banner(&mut self) {
        self.banner = ::std::option::Option::None;
    }

    pub fn has_banner(&self) -> bool {
        self.banner.is_some()
    }

    // Param is passed by value, moved
    pub fn set_banner(&mut self, v: u64) {
        self.banner = ::std::option::Option::Some(v);
    }

    pub fn get_banner(&self) -> u64 {
        self.banner.unwrap_or(0)
    }

    fn get_banner_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.banner
    }

    fn mut_banner_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.banner
    }

    // optional bool can_be_deleted = 31;

    pub fn clear_can_be_deleted(&mut self) {
        self.can_be_deleted = ::std::option::Option::None;
    }

    pub fn has_can_be_deleted(&self) -> bool {
        self.can_be_deleted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_can_be_deleted(&mut self, v: bool) {
        self.can_be_deleted = ::std::option::Option::Some(v);
    }

    pub fn get_can_be_deleted(&self) -> bool {
        self.can_be_deleted.unwrap_or(false)
    }

    fn get_can_be_deleted_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.can_be_deleted
    }

    fn mut_can_be_deleted_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.can_be_deleted
    }

    // optional bool incompatible = 32;

    pub fn clear_incompatible(&mut self) {
        self.incompatible = ::std::option::Option::None;
    }

    pub fn has_incompatible(&self) -> bool {
        self.incompatible.is_some()
    }

    // Param is passed by value, moved
    pub fn set_incompatible(&mut self, v: bool) {
        self.incompatible = ::std::option::Option::Some(v);
    }

    pub fn get_incompatible(&self) -> bool {
        self.incompatible.unwrap_or(false)
    }

    fn get_incompatible_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.incompatible
    }

    fn mut_incompatible_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.incompatible
    }

    // optional string app_name = 33;

    pub fn clear_app_name(&mut self) {
        self.app_name.clear();
    }

    pub fn has_app_name(&self) -> bool {
        self.app_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_name(&mut self, v: ::std::string::String) {
        self.app_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_app_name(&mut self) -> &mut ::std::string::String {
        if self.app_name.is_none() {
            self.app_name.set_default();
        }
        self.app_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_app_name(&mut self) -> ::std::string::String {
        self.app_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_app_name(&self) -> &str {
        match self.app_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_app_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.app_name
    }

    fn mut_app_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.app_name
    }

    // optional uint32 file_type = 34;

    pub fn clear_file_type(&mut self) {
        self.file_type = ::std::option::Option::None;
    }

    pub fn has_file_type(&self) -> bool {
        self.file_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_type(&mut self, v: u32) {
        self.file_type = ::std::option::Option::Some(v);
    }

    pub fn get_file_type(&self) -> u32 {
        self.file_type.unwrap_or(0)
    }

    fn get_file_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.file_type
    }

    fn mut_file_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.file_type
    }

    // optional bool can_subscribe = 35;

    pub fn clear_can_subscribe(&mut self) {
        self.can_subscribe = ::std::option::Option::None;
    }

    pub fn has_can_subscribe(&self) -> bool {
        self.can_subscribe.is_some()
    }

    // Param is passed by value, moved
    pub fn set_can_subscribe(&mut self, v: bool) {
        self.can_subscribe = ::std::option::Option::Some(v);
    }

    pub fn get_can_subscribe(&self) -> bool {
        self.can_subscribe.unwrap_or(false)
    }

    fn get_can_subscribe_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.can_subscribe
    }

    fn mut_can_subscribe_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.can_subscribe
    }

    // optional uint32 subscriptions = 36;

    pub fn clear_subscriptions(&mut self) {
        self.subscriptions = ::std::option::Option::None;
    }

    pub fn has_subscriptions(&self) -> bool {
        self.subscriptions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscriptions(&mut self, v: u32) {
        self.subscriptions = ::std::option::Option::Some(v);
    }

    pub fn get_subscriptions(&self) -> u32 {
        self.subscriptions.unwrap_or(0)
    }

    fn get_subscriptions_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.subscriptions
    }

    fn mut_subscriptions_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.subscriptions
    }

    // optional uint32 favorited = 37;

    pub fn clear_favorited(&mut self) {
        self.favorited = ::std::option::Option::None;
    }

    pub fn has_favorited(&self) -> bool {
        self.favorited.is_some()
    }

    // Param is passed by value, moved
    pub fn set_favorited(&mut self, v: u32) {
        self.favorited = ::std::option::Option::Some(v);
    }

    pub fn get_favorited(&self) -> u32 {
        self.favorited.unwrap_or(0)
    }

    fn get_favorited_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.favorited
    }

    fn mut_favorited_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.favorited
    }

    // optional uint32 followers = 38;

    pub fn clear_followers(&mut self) {
        self.followers = ::std::option::Option::None;
    }

    pub fn has_followers(&self) -> bool {
        self.followers.is_some()
    }

    // Param is passed by value, moved
    pub fn set_followers(&mut self, v: u32) {
        self.followers = ::std::option::Option::Some(v);
    }

    pub fn get_followers(&self) -> u32 {
        self.followers.unwrap_or(0)
    }

    fn get_followers_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.followers
    }

    fn mut_followers_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.followers
    }

    // optional uint32 lifetime_subscriptions = 39;

    pub fn clear_lifetime_subscriptions(&mut self) {
        self.lifetime_subscriptions = ::std::option::Option::None;
    }

    pub fn has_lifetime_subscriptions(&self) -> bool {
        self.lifetime_subscriptions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lifetime_subscriptions(&mut self, v: u32) {
        self.lifetime_subscriptions = ::std::option::Option::Some(v);
    }

    pub fn get_lifetime_subscriptions(&self) -> u32 {
        self.lifetime_subscriptions.unwrap_or(0)
    }

    fn get_lifetime_subscriptions_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.lifetime_subscriptions
    }

    fn mut_lifetime_subscriptions_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.lifetime_subscriptions
    }

    // optional uint32 lifetime_favorited = 40;

    pub fn clear_lifetime_favorited(&mut self) {
        self.lifetime_favorited = ::std::option::Option::None;
    }

    pub fn has_lifetime_favorited(&self) -> bool {
        self.lifetime_favorited.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lifetime_favorited(&mut self, v: u32) {
        self.lifetime_favorited = ::std::option::Option::Some(v);
    }

    pub fn get_lifetime_favorited(&self) -> u32 {
        self.lifetime_favorited.unwrap_or(0)
    }

    fn get_lifetime_favorited_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.lifetime_favorited
    }

    fn mut_lifetime_favorited_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.lifetime_favorited
    }

    // optional uint32 lifetime_followers = 41;

    pub fn clear_lifetime_followers(&mut self) {
        self.lifetime_followers = ::std::option::Option::None;
    }

    pub fn has_lifetime_followers(&self) -> bool {
        self.lifetime_followers.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lifetime_followers(&mut self, v: u32) {
        self.lifetime_followers = ::std::option::Option::Some(v);
    }

    pub fn get_lifetime_followers(&self) -> u32 {
        self.lifetime_followers.unwrap_or(0)
    }

    fn get_lifetime_followers_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.lifetime_followers
    }

    fn mut_lifetime_followers_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.lifetime_followers
    }

    // optional uint32 views = 42;

    pub fn clear_views(&mut self) {
        self.views = ::std::option::Option::None;
    }

    pub fn has_views(&self) -> bool {
        self.views.is_some()
    }

    // Param is passed by value, moved
    pub fn set_views(&mut self, v: u32) {
        self.views = ::std::option::Option::Some(v);
    }

    pub fn get_views(&self) -> u32 {
        self.views.unwrap_or(0)
    }

    fn get_views_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.views
    }

    fn mut_views_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.views
    }

    // optional uint32 image_width = 43;

    pub fn clear_image_width(&mut self) {
        self.image_width = ::std::option::Option::None;
    }

    pub fn has_image_width(&self) -> bool {
        self.image_width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image_width(&mut self, v: u32) {
        self.image_width = ::std::option::Option::Some(v);
    }

    pub fn get_image_width(&self) -> u32 {
        self.image_width.unwrap_or(0)
    }

    fn get_image_width_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.image_width
    }

    fn mut_image_width_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.image_width
    }

    // optional uint32 image_height = 44;

    pub fn clear_image_height(&mut self) {
        self.image_height = ::std::option::Option::None;
    }

    pub fn has_image_height(&self) -> bool {
        self.image_height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image_height(&mut self, v: u32) {
        self.image_height = ::std::option::Option::Some(v);
    }

    pub fn get_image_height(&self) -> u32 {
        self.image_height.unwrap_or(0)
    }

    fn get_image_height_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.image_height
    }

    fn mut_image_height_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.image_height
    }

    // optional string image_url = 45;

    pub fn clear_image_url(&mut self) {
        self.image_url.clear();
    }

    pub fn has_image_url(&self) -> bool {
        self.image_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image_url(&mut self, v: ::std::string::String) {
        self.image_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image_url(&mut self) -> &mut ::std::string::String {
        if self.image_url.is_none() {
            self.image_url.set_default();
        }
        self.image_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_image_url(&mut self) -> ::std::string::String {
        self.image_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_image_url(&self) -> &str {
        match self.image_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_image_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.image_url
    }

    fn mut_image_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.image_url
    }

    // optional bool spoiler_tag = 46;

    pub fn clear_spoiler_tag(&mut self) {
        self.spoiler_tag = ::std::option::Option::None;
    }

    pub fn has_spoiler_tag(&self) -> bool {
        self.spoiler_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spoiler_tag(&mut self, v: bool) {
        self.spoiler_tag = ::std::option::Option::Some(v);
    }

    pub fn get_spoiler_tag(&self) -> bool {
        self.spoiler_tag.unwrap_or(false)
    }

    fn get_spoiler_tag_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.spoiler_tag
    }

    fn mut_spoiler_tag_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.spoiler_tag
    }

    // optional uint32 shortcutid = 47;

    pub fn clear_shortcutid(&mut self) {
        self.shortcutid = ::std::option::Option::None;
    }

    pub fn has_shortcutid(&self) -> bool {
        self.shortcutid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shortcutid(&mut self, v: u32) {
        self.shortcutid = ::std::option::Option::Some(v);
    }

    pub fn get_shortcutid(&self) -> u32 {
        self.shortcutid.unwrap_or(0)
    }

    fn get_shortcutid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.shortcutid
    }

    fn mut_shortcutid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.shortcutid
    }

    // optional string shortcutname = 48;

    pub fn clear_shortcutname(&mut self) {
        self.shortcutname.clear();
    }

    pub fn has_shortcutname(&self) -> bool {
        self.shortcutname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shortcutname(&mut self, v: ::std::string::String) {
        self.shortcutname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shortcutname(&mut self) -> &mut ::std::string::String {
        if self.shortcutname.is_none() {
            self.shortcutname.set_default();
        }
        self.shortcutname.as_mut().unwrap()
    }

    // Take field
    pub fn take_shortcutname(&mut self) -> ::std::string::String {
        self.shortcutname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_shortcutname(&self) -> &str {
        match self.shortcutname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_shortcutname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.shortcutname
    }

    fn mut_shortcutname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.shortcutname
    }

    // optional uint32 num_children = 49;

    pub fn clear_num_children(&mut self) {
        self.num_children = ::std::option::Option::None;
    }

    pub fn has_num_children(&self) -> bool {
        self.num_children.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_children(&mut self, v: u32) {
        self.num_children = ::std::option::Option::Some(v);
    }

    pub fn get_num_children(&self) -> u32 {
        self.num_children.unwrap_or(0)
    }

    fn get_num_children_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.num_children
    }

    fn mut_num_children_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.num_children
    }

    // optional uint32 num_reports = 50;

    pub fn clear_num_reports(&mut self) {
        self.num_reports = ::std::option::Option::None;
    }

    pub fn has_num_reports(&self) -> bool {
        self.num_reports.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_reports(&mut self, v: u32) {
        self.num_reports = ::std::option::Option::Some(v);
    }

    pub fn get_num_reports(&self) -> u32 {
        self.num_reports.unwrap_or(0)
    }

    fn get_num_reports_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.num_reports
    }

    fn mut_num_reports_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.num_reports
    }

    // repeated .PublishedFileDetails.Preview previews = 51;

    pub fn clear_previews(&mut self) {
        self.previews.clear();
    }

    // Param is passed by value, moved
    pub fn set_previews(&mut self, v: ::protobuf::RepeatedField<PublishedFileDetails_Preview>) {
        self.previews = v;
    }

    // Mutable pointer to the field.
    pub fn mut_previews(&mut self) -> &mut ::protobuf::RepeatedField<PublishedFileDetails_Preview> {
        &mut self.previews
    }

    // Take field
    pub fn take_previews(&mut self) -> ::protobuf::RepeatedField<PublishedFileDetails_Preview> {
        ::std::mem::replace(&mut self.previews, ::protobuf::RepeatedField::new())
    }

    pub fn get_previews(&self) -> &[PublishedFileDetails_Preview] {
        &self.previews
    }

    fn get_previews_for_reflect(&self) -> &::protobuf::RepeatedField<PublishedFileDetails_Preview> {
        &self.previews
    }

    fn mut_previews_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PublishedFileDetails_Preview> {
        &mut self.previews
    }

    // repeated .PublishedFileDetails.Tag tags = 52;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<PublishedFileDetails_Tag>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags(&mut self) -> &mut ::protobuf::RepeatedField<PublishedFileDetails_Tag> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<PublishedFileDetails_Tag> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags(&self) -> &[PublishedFileDetails_Tag] {
        &self.tags
    }

    fn get_tags_for_reflect(&self) -> &::protobuf::RepeatedField<PublishedFileDetails_Tag> {
        &self.tags
    }

    fn mut_tags_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PublishedFileDetails_Tag> {
        &mut self.tags
    }

    // repeated .PublishedFileDetails.Child children = 53;

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    // Param is passed by value, moved
    pub fn set_children(&mut self, v: ::protobuf::RepeatedField<PublishedFileDetails_Child>) {
        self.children = v;
    }

    // Mutable pointer to the field.
    pub fn mut_children(&mut self) -> &mut ::protobuf::RepeatedField<PublishedFileDetails_Child> {
        &mut self.children
    }

    // Take field
    pub fn take_children(&mut self) -> ::protobuf::RepeatedField<PublishedFileDetails_Child> {
        ::std::mem::replace(&mut self.children, ::protobuf::RepeatedField::new())
    }

    pub fn get_children(&self) -> &[PublishedFileDetails_Child] {
        &self.children
    }

    fn get_children_for_reflect(&self) -> &::protobuf::RepeatedField<PublishedFileDetails_Child> {
        &self.children
    }

    fn mut_children_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PublishedFileDetails_Child> {
        &mut self.children
    }

    // repeated .PublishedFileDetails.KVTag kvtags = 54;

    pub fn clear_kvtags(&mut self) {
        self.kvtags.clear();
    }

    // Param is passed by value, moved
    pub fn set_kvtags(&mut self, v: ::protobuf::RepeatedField<PublishedFileDetails_KVTag>) {
        self.kvtags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_kvtags(&mut self) -> &mut ::protobuf::RepeatedField<PublishedFileDetails_KVTag> {
        &mut self.kvtags
    }

    // Take field
    pub fn take_kvtags(&mut self) -> ::protobuf::RepeatedField<PublishedFileDetails_KVTag> {
        ::std::mem::replace(&mut self.kvtags, ::protobuf::RepeatedField::new())
    }

    pub fn get_kvtags(&self) -> &[PublishedFileDetails_KVTag] {
        &self.kvtags
    }

    fn get_kvtags_for_reflect(&self) -> &::protobuf::RepeatedField<PublishedFileDetails_KVTag> {
        &self.kvtags
    }

    fn mut_kvtags_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PublishedFileDetails_KVTag> {
        &mut self.kvtags
    }

    // optional .PublishedFileDetails.VoteData vote_data = 55;

    pub fn clear_vote_data(&mut self) {
        self.vote_data.clear();
    }

    pub fn has_vote_data(&self) -> bool {
        self.vote_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vote_data(&mut self, v: PublishedFileDetails_VoteData) {
        self.vote_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vote_data(&mut self) -> &mut PublishedFileDetails_VoteData {
        if self.vote_data.is_none() {
            self.vote_data.set_default();
        }
        self.vote_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_vote_data(&mut self) -> PublishedFileDetails_VoteData {
        self.vote_data.take().unwrap_or_else(|| PublishedFileDetails_VoteData::new())
    }

    pub fn get_vote_data(&self) -> &PublishedFileDetails_VoteData {
        self.vote_data.as_ref().unwrap_or_else(|| PublishedFileDetails_VoteData::default_instance())
    }

    fn get_vote_data_for_reflect(&self) -> &::protobuf::SingularPtrField<PublishedFileDetails_VoteData> {
        &self.vote_data
    }

    fn mut_vote_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PublishedFileDetails_VoteData> {
        &mut self.vote_data
    }

    // optional uint32 time_subscribed = 56;

    pub fn clear_time_subscribed(&mut self) {
        self.time_subscribed = ::std::option::Option::None;
    }

    pub fn has_time_subscribed(&self) -> bool {
        self.time_subscribed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_subscribed(&mut self, v: u32) {
        self.time_subscribed = ::std::option::Option::Some(v);
    }

    pub fn get_time_subscribed(&self) -> u32 {
        self.time_subscribed.unwrap_or(0)
    }

    fn get_time_subscribed_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_subscribed
    }

    fn mut_time_subscribed_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_subscribed
    }
}

impl ::protobuf::Message for PublishedFileDetails {
    fn is_initialized(&self) -> bool {
        for v in &self.previews {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tags {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.children {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.kvtags {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.vote_data {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.publishedfileid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.creator = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.creator_appid = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.consumer_appid = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.consumer_shortcutid = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.filename)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.file_size = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.preview_file_size = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.file_url)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.preview_url)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.youtubevideoid)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url)?;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.hcontent_file = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.hcontent_preview = ::std::option::Option::Some(tmp);
                },
                16 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.title)?;
                },
                17 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.file_description)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.short_description)?;
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.time_created = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.time_updated = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.visibility = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.workshop_file = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.workshop_accepted = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.show_subscribe_all = ::std::option::Option::Some(tmp);
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.num_comments_developer = ::std::option::Option::Some(tmp);
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.num_comments_public = ::std::option::Option::Some(tmp);
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.banned = ::std::option::Option::Some(tmp);
                },
                29 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ban_reason)?;
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.banner = ::std::option::Option::Some(tmp);
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.can_be_deleted = ::std::option::Option::Some(tmp);
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.incompatible = ::std::option::Option::Some(tmp);
                },
                33 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.app_name)?;
                },
                34 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.file_type = ::std::option::Option::Some(tmp);
                },
                35 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.can_subscribe = ::std::option::Option::Some(tmp);
                },
                36 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.subscriptions = ::std::option::Option::Some(tmp);
                },
                37 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.favorited = ::std::option::Option::Some(tmp);
                },
                38 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.followers = ::std::option::Option::Some(tmp);
                },
                39 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.lifetime_subscriptions = ::std::option::Option::Some(tmp);
                },
                40 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.lifetime_favorited = ::std::option::Option::Some(tmp);
                },
                41 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.lifetime_followers = ::std::option::Option::Some(tmp);
                },
                42 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.views = ::std::option::Option::Some(tmp);
                },
                43 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.image_width = ::std::option::Option::Some(tmp);
                },
                44 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.image_height = ::std::option::Option::Some(tmp);
                },
                45 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.image_url)?;
                },
                46 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.spoiler_tag = ::std::option::Option::Some(tmp);
                },
                47 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.shortcutid = ::std::option::Option::Some(tmp);
                },
                48 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.shortcutname)?;
                },
                49 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.num_children = ::std::option::Option::Some(tmp);
                },
                50 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.num_reports = ::std::option::Option::Some(tmp);
                },
                51 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.previews)?;
                },
                52 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tags)?;
                },
                53 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.children)?;
                },
                54 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.kvtags)?;
                },
                55 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.vote_data)?;
                },
                56 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.time_subscribed = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.publishedfileid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.creator {
            my_size += 9;
        }
        if let Some(v) = self.creator_appid {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.consumer_appid {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.consumer_shortcutid {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.filename.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        if let Some(v) = self.file_size {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.preview_file_size {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.file_url.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        if let Some(ref v) = self.preview_url.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        }
        if let Some(ref v) = self.youtubevideoid.as_ref() {
            my_size += ::protobuf::rt::string_size(12, &v);
        }
        if let Some(ref v) = self.url.as_ref() {
            my_size += ::protobuf::rt::string_size(13, &v);
        }
        if let Some(v) = self.hcontent_file {
            my_size += 9;
        }
        if let Some(v) = self.hcontent_preview {
            my_size += 9;
        }
        if let Some(ref v) = self.title.as_ref() {
            my_size += ::protobuf::rt::string_size(16, &v);
        }
        if let Some(ref v) = self.file_description.as_ref() {
            my_size += ::protobuf::rt::string_size(17, &v);
        }
        if let Some(ref v) = self.short_description.as_ref() {
            my_size += ::protobuf::rt::string_size(18, &v);
        }
        if let Some(v) = self.time_created {
            my_size += ::protobuf::rt::value_size(19, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.time_updated {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.visibility {
            my_size += ::protobuf::rt::value_size(21, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(22, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.workshop_file {
            my_size += 3;
        }
        if let Some(v) = self.workshop_accepted {
            my_size += 3;
        }
        if let Some(v) = self.show_subscribe_all {
            my_size += 3;
        }
        if let Some(v) = self.num_comments_developer {
            my_size += ::protobuf::rt::value_size(26, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_comments_public {
            my_size += ::protobuf::rt::value_size(27, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.banned {
            my_size += 3;
        }
        if let Some(ref v) = self.ban_reason.as_ref() {
            my_size += ::protobuf::rt::string_size(29, &v);
        }
        if let Some(v) = self.banner {
            my_size += 10;
        }
        if let Some(v) = self.can_be_deleted {
            my_size += 3;
        }
        if let Some(v) = self.incompatible {
            my_size += 3;
        }
        if let Some(ref v) = self.app_name.as_ref() {
            my_size += ::protobuf::rt::string_size(33, &v);
        }
        if let Some(v) = self.file_type {
            my_size += ::protobuf::rt::value_size(34, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.can_subscribe {
            my_size += 3;
        }
        if let Some(v) = self.subscriptions {
            my_size += ::protobuf::rt::value_size(36, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.favorited {
            my_size += ::protobuf::rt::value_size(37, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.followers {
            my_size += ::protobuf::rt::value_size(38, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lifetime_subscriptions {
            my_size += ::protobuf::rt::value_size(39, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lifetime_favorited {
            my_size += ::protobuf::rt::value_size(40, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lifetime_followers {
            my_size += ::protobuf::rt::value_size(41, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.views {
            my_size += ::protobuf::rt::value_size(42, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.image_width {
            my_size += ::protobuf::rt::value_size(43, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.image_height {
            my_size += ::protobuf::rt::value_size(44, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.image_url.as_ref() {
            my_size += ::protobuf::rt::string_size(45, &v);
        }
        if let Some(v) = self.spoiler_tag {
            my_size += 3;
        }
        if let Some(v) = self.shortcutid {
            my_size += ::protobuf::rt::value_size(47, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.shortcutname.as_ref() {
            my_size += ::protobuf::rt::string_size(48, &v);
        }
        if let Some(v) = self.num_children {
            my_size += ::protobuf::rt::value_size(49, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_reports {
            my_size += ::protobuf::rt::value_size(50, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.previews {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.tags {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.children {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.kvtags {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.vote_data.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.time_subscribed {
            my_size += ::protobuf::rt::value_size(56, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.publishedfileid {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.creator {
            os.write_fixed64(3, v)?;
        }
        if let Some(v) = self.creator_appid {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.consumer_appid {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.consumer_shortcutid {
            os.write_uint32(6, v)?;
        }
        if let Some(ref v) = self.filename.as_ref() {
            os.write_string(7, &v)?;
        }
        if let Some(v) = self.file_size {
            os.write_uint64(8, v)?;
        }
        if let Some(v) = self.preview_file_size {
            os.write_uint64(9, v)?;
        }
        if let Some(ref v) = self.file_url.as_ref() {
            os.write_string(10, &v)?;
        }
        if let Some(ref v) = self.preview_url.as_ref() {
            os.write_string(11, &v)?;
        }
        if let Some(ref v) = self.youtubevideoid.as_ref() {
            os.write_string(12, &v)?;
        }
        if let Some(ref v) = self.url.as_ref() {
            os.write_string(13, &v)?;
        }
        if let Some(v) = self.hcontent_file {
            os.write_fixed64(14, v)?;
        }
        if let Some(v) = self.hcontent_preview {
            os.write_fixed64(15, v)?;
        }
        if let Some(ref v) = self.title.as_ref() {
            os.write_string(16, &v)?;
        }
        if let Some(ref v) = self.file_description.as_ref() {
            os.write_string(17, &v)?;
        }
        if let Some(ref v) = self.short_description.as_ref() {
            os.write_string(18, &v)?;
        }
        if let Some(v) = self.time_created {
            os.write_uint32(19, v)?;
        }
        if let Some(v) = self.time_updated {
            os.write_uint32(20, v)?;
        }
        if let Some(v) = self.visibility {
            os.write_uint32(21, v)?;
        }
        if let Some(v) = self.flags {
            os.write_uint32(22, v)?;
        }
        if let Some(v) = self.workshop_file {
            os.write_bool(23, v)?;
        }
        if let Some(v) = self.workshop_accepted {
            os.write_bool(24, v)?;
        }
        if let Some(v) = self.show_subscribe_all {
            os.write_bool(25, v)?;
        }
        if let Some(v) = self.num_comments_developer {
            os.write_int32(26, v)?;
        }
        if let Some(v) = self.num_comments_public {
            os.write_int32(27, v)?;
        }
        if let Some(v) = self.banned {
            os.write_bool(28, v)?;
        }
        if let Some(ref v) = self.ban_reason.as_ref() {
            os.write_string(29, &v)?;
        }
        if let Some(v) = self.banner {
            os.write_fixed64(30, v)?;
        }
        if let Some(v) = self.can_be_deleted {
            os.write_bool(31, v)?;
        }
        if let Some(v) = self.incompatible {
            os.write_bool(32, v)?;
        }
        if let Some(ref v) = self.app_name.as_ref() {
            os.write_string(33, &v)?;
        }
        if let Some(v) = self.file_type {
            os.write_uint32(34, v)?;
        }
        if let Some(v) = self.can_subscribe {
            os.write_bool(35, v)?;
        }
        if let Some(v) = self.subscriptions {
            os.write_uint32(36, v)?;
        }
        if let Some(v) = self.favorited {
            os.write_uint32(37, v)?;
        }
        if let Some(v) = self.followers {
            os.write_uint32(38, v)?;
        }
        if let Some(v) = self.lifetime_subscriptions {
            os.write_uint32(39, v)?;
        }
        if let Some(v) = self.lifetime_favorited {
            os.write_uint32(40, v)?;
        }
        if let Some(v) = self.lifetime_followers {
            os.write_uint32(41, v)?;
        }
        if let Some(v) = self.views {
            os.write_uint32(42, v)?;
        }
        if let Some(v) = self.image_width {
            os.write_uint32(43, v)?;
        }
        if let Some(v) = self.image_height {
            os.write_uint32(44, v)?;
        }
        if let Some(ref v) = self.image_url.as_ref() {
            os.write_string(45, &v)?;
        }
        if let Some(v) = self.spoiler_tag {
            os.write_bool(46, v)?;
        }
        if let Some(v) = self.shortcutid {
            os.write_uint32(47, v)?;
        }
        if let Some(ref v) = self.shortcutname.as_ref() {
            os.write_string(48, &v)?;
        }
        if let Some(v) = self.num_children {
            os.write_uint32(49, v)?;
        }
        if let Some(v) = self.num_reports {
            os.write_uint32(50, v)?;
        }
        for v in &self.previews {
            os.write_tag(51, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.tags {
            os.write_tag(52, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.children {
            os.write_tag(53, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.kvtags {
            os.write_tag(54, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.vote_data.as_ref() {
            os.write_tag(55, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.time_subscribed {
            os.write_uint32(56, v)?;
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

impl ::protobuf::MessageStatic for PublishedFileDetails {
    fn new() -> PublishedFileDetails {
        PublishedFileDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<PublishedFileDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "result",
                    PublishedFileDetails::get_result_for_reflect,
                    PublishedFileDetails::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "publishedfileid",
                    PublishedFileDetails::get_publishedfileid_for_reflect,
                    PublishedFileDetails::mut_publishedfileid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "creator",
                    PublishedFileDetails::get_creator_for_reflect,
                    PublishedFileDetails::mut_creator_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "creator_appid",
                    PublishedFileDetails::get_creator_appid_for_reflect,
                    PublishedFileDetails::mut_creator_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "consumer_appid",
                    PublishedFileDetails::get_consumer_appid_for_reflect,
                    PublishedFileDetails::mut_consumer_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "consumer_shortcutid",
                    PublishedFileDetails::get_consumer_shortcutid_for_reflect,
                    PublishedFileDetails::mut_consumer_shortcutid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filename",
                    PublishedFileDetails::get_filename_for_reflect,
                    PublishedFileDetails::mut_filename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "file_size",
                    PublishedFileDetails::get_file_size_for_reflect,
                    PublishedFileDetails::mut_file_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "preview_file_size",
                    PublishedFileDetails::get_preview_file_size_for_reflect,
                    PublishedFileDetails::mut_preview_file_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "file_url",
                    PublishedFileDetails::get_file_url_for_reflect,
                    PublishedFileDetails::mut_file_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "preview_url",
                    PublishedFileDetails::get_preview_url_for_reflect,
                    PublishedFileDetails::mut_preview_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "youtubevideoid",
                    PublishedFileDetails::get_youtubevideoid_for_reflect,
                    PublishedFileDetails::mut_youtubevideoid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url",
                    PublishedFileDetails::get_url_for_reflect,
                    PublishedFileDetails::mut_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "hcontent_file",
                    PublishedFileDetails::get_hcontent_file_for_reflect,
                    PublishedFileDetails::mut_hcontent_file_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "hcontent_preview",
                    PublishedFileDetails::get_hcontent_preview_for_reflect,
                    PublishedFileDetails::mut_hcontent_preview_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    PublishedFileDetails::get_title_for_reflect,
                    PublishedFileDetails::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "file_description",
                    PublishedFileDetails::get_file_description_for_reflect,
                    PublishedFileDetails::mut_file_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "short_description",
                    PublishedFileDetails::get_short_description_for_reflect,
                    PublishedFileDetails::mut_short_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_created",
                    PublishedFileDetails::get_time_created_for_reflect,
                    PublishedFileDetails::mut_time_created_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_updated",
                    PublishedFileDetails::get_time_updated_for_reflect,
                    PublishedFileDetails::mut_time_updated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "visibility",
                    PublishedFileDetails::get_visibility_for_reflect,
                    PublishedFileDetails::mut_visibility_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    PublishedFileDetails::get_flags_for_reflect,
                    PublishedFileDetails::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "workshop_file",
                    PublishedFileDetails::get_workshop_file_for_reflect,
                    PublishedFileDetails::mut_workshop_file_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "workshop_accepted",
                    PublishedFileDetails::get_workshop_accepted_for_reflect,
                    PublishedFileDetails::mut_workshop_accepted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "show_subscribe_all",
                    PublishedFileDetails::get_show_subscribe_all_for_reflect,
                    PublishedFileDetails::mut_show_subscribe_all_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_comments_developer",
                    PublishedFileDetails::get_num_comments_developer_for_reflect,
                    PublishedFileDetails::mut_num_comments_developer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_comments_public",
                    PublishedFileDetails::get_num_comments_public_for_reflect,
                    PublishedFileDetails::mut_num_comments_public_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "banned",
                    PublishedFileDetails::get_banned_for_reflect,
                    PublishedFileDetails::mut_banned_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ban_reason",
                    PublishedFileDetails::get_ban_reason_for_reflect,
                    PublishedFileDetails::mut_ban_reason_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "banner",
                    PublishedFileDetails::get_banner_for_reflect,
                    PublishedFileDetails::mut_banner_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "can_be_deleted",
                    PublishedFileDetails::get_can_be_deleted_for_reflect,
                    PublishedFileDetails::mut_can_be_deleted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "incompatible",
                    PublishedFileDetails::get_incompatible_for_reflect,
                    PublishedFileDetails::mut_incompatible_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "app_name",
                    PublishedFileDetails::get_app_name_for_reflect,
                    PublishedFileDetails::mut_app_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "file_type",
                    PublishedFileDetails::get_file_type_for_reflect,
                    PublishedFileDetails::mut_file_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "can_subscribe",
                    PublishedFileDetails::get_can_subscribe_for_reflect,
                    PublishedFileDetails::mut_can_subscribe_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "subscriptions",
                    PublishedFileDetails::get_subscriptions_for_reflect,
                    PublishedFileDetails::mut_subscriptions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "favorited",
                    PublishedFileDetails::get_favorited_for_reflect,
                    PublishedFileDetails::mut_favorited_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "followers",
                    PublishedFileDetails::get_followers_for_reflect,
                    PublishedFileDetails::mut_followers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "lifetime_subscriptions",
                    PublishedFileDetails::get_lifetime_subscriptions_for_reflect,
                    PublishedFileDetails::mut_lifetime_subscriptions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "lifetime_favorited",
                    PublishedFileDetails::get_lifetime_favorited_for_reflect,
                    PublishedFileDetails::mut_lifetime_favorited_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "lifetime_followers",
                    PublishedFileDetails::get_lifetime_followers_for_reflect,
                    PublishedFileDetails::mut_lifetime_followers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "views",
                    PublishedFileDetails::get_views_for_reflect,
                    PublishedFileDetails::mut_views_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "image_width",
                    PublishedFileDetails::get_image_width_for_reflect,
                    PublishedFileDetails::mut_image_width_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "image_height",
                    PublishedFileDetails::get_image_height_for_reflect,
                    PublishedFileDetails::mut_image_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "image_url",
                    PublishedFileDetails::get_image_url_for_reflect,
                    PublishedFileDetails::mut_image_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "spoiler_tag",
                    PublishedFileDetails::get_spoiler_tag_for_reflect,
                    PublishedFileDetails::mut_spoiler_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "shortcutid",
                    PublishedFileDetails::get_shortcutid_for_reflect,
                    PublishedFileDetails::mut_shortcutid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "shortcutname",
                    PublishedFileDetails::get_shortcutname_for_reflect,
                    PublishedFileDetails::mut_shortcutname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_children",
                    PublishedFileDetails::get_num_children_for_reflect,
                    PublishedFileDetails::mut_num_children_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_reports",
                    PublishedFileDetails::get_num_reports_for_reflect,
                    PublishedFileDetails::mut_num_reports_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PublishedFileDetails_Preview>>(
                    "previews",
                    PublishedFileDetails::get_previews_for_reflect,
                    PublishedFileDetails::mut_previews_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PublishedFileDetails_Tag>>(
                    "tags",
                    PublishedFileDetails::get_tags_for_reflect,
                    PublishedFileDetails::mut_tags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PublishedFileDetails_Child>>(
                    "children",
                    PublishedFileDetails::get_children_for_reflect,
                    PublishedFileDetails::mut_children_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PublishedFileDetails_KVTag>>(
                    "kvtags",
                    PublishedFileDetails::get_kvtags_for_reflect,
                    PublishedFileDetails::mut_kvtags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PublishedFileDetails_VoteData>>(
                    "vote_data",
                    PublishedFileDetails::get_vote_data_for_reflect,
                    PublishedFileDetails::mut_vote_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_subscribed",
                    PublishedFileDetails::get_time_subscribed_for_reflect,
                    PublishedFileDetails::mut_time_subscribed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublishedFileDetails>(
                    "PublishedFileDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublishedFileDetails {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_publishedfileid();
        self.clear_creator();
        self.clear_creator_appid();
        self.clear_consumer_appid();
        self.clear_consumer_shortcutid();
        self.clear_filename();
        self.clear_file_size();
        self.clear_preview_file_size();
        self.clear_file_url();
        self.clear_preview_url();
        self.clear_youtubevideoid();
        self.clear_url();
        self.clear_hcontent_file();
        self.clear_hcontent_preview();
        self.clear_title();
        self.clear_file_description();
        self.clear_short_description();
        self.clear_time_created();
        self.clear_time_updated();
        self.clear_visibility();
        self.clear_flags();
        self.clear_workshop_file();
        self.clear_workshop_accepted();
        self.clear_show_subscribe_all();
        self.clear_num_comments_developer();
        self.clear_num_comments_public();
        self.clear_banned();
        self.clear_ban_reason();
        self.clear_banner();
        self.clear_can_be_deleted();
        self.clear_incompatible();
        self.clear_app_name();
        self.clear_file_type();
        self.clear_can_subscribe();
        self.clear_subscriptions();
        self.clear_favorited();
        self.clear_followers();
        self.clear_lifetime_subscriptions();
        self.clear_lifetime_favorited();
        self.clear_lifetime_followers();
        self.clear_views();
        self.clear_image_width();
        self.clear_image_height();
        self.clear_image_url();
        self.clear_spoiler_tag();
        self.clear_shortcutid();
        self.clear_shortcutname();
        self.clear_num_children();
        self.clear_num_reports();
        self.clear_previews();
        self.clear_tags();
        self.clear_children();
        self.clear_kvtags();
        self.clear_vote_data();
        self.clear_time_subscribed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PublishedFileDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PublishedFileDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PublishedFileDetails_Tag {
    // message fields
    tag: ::protobuf::SingularField<::std::string::String>,
    adminonly: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PublishedFileDetails_Tag {}

impl PublishedFileDetails_Tag {
    pub fn new() -> PublishedFileDetails_Tag {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublishedFileDetails_Tag {
        static mut instance: ::protobuf::lazy::Lazy<PublishedFileDetails_Tag> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublishedFileDetails_Tag,
        };
        unsafe {
            instance.get(PublishedFileDetails_Tag::new)
        }
    }

    // optional string tag = 1;

    pub fn clear_tag(&mut self) {
        self.tag.clear();
    }

    pub fn has_tag(&self) -> bool {
        self.tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: ::std::string::String) {
        self.tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tag(&mut self) -> &mut ::std::string::String {
        if self.tag.is_none() {
            self.tag.set_default();
        }
        self.tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_tag(&mut self) -> ::std::string::String {
        self.tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_tag(&self) -> &str {
        match self.tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.tag
    }

    fn mut_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.tag
    }

    // optional bool adminonly = 2;

    pub fn clear_adminonly(&mut self) {
        self.adminonly = ::std::option::Option::None;
    }

    pub fn has_adminonly(&self) -> bool {
        self.adminonly.is_some()
    }

    // Param is passed by value, moved
    pub fn set_adminonly(&mut self, v: bool) {
        self.adminonly = ::std::option::Option::Some(v);
    }

    pub fn get_adminonly(&self) -> bool {
        self.adminonly.unwrap_or(false)
    }

    fn get_adminonly_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.adminonly
    }

    fn mut_adminonly_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.adminonly
    }
}

impl ::protobuf::Message for PublishedFileDetails_Tag {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.tag)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.adminonly = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.tag.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.adminonly {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.tag.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.adminonly {
            os.write_bool(2, v)?;
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

impl ::protobuf::MessageStatic for PublishedFileDetails_Tag {
    fn new() -> PublishedFileDetails_Tag {
        PublishedFileDetails_Tag::new()
    }

    fn descriptor_static(_: ::std::option::Option<PublishedFileDetails_Tag>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tag",
                    PublishedFileDetails_Tag::get_tag_for_reflect,
                    PublishedFileDetails_Tag::mut_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "adminonly",
                    PublishedFileDetails_Tag::get_adminonly_for_reflect,
                    PublishedFileDetails_Tag::mut_adminonly_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublishedFileDetails_Tag>(
                    "PublishedFileDetails_Tag",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublishedFileDetails_Tag {
    fn clear(&mut self) {
        self.clear_tag();
        self.clear_adminonly();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PublishedFileDetails_Tag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PublishedFileDetails_Tag {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PublishedFileDetails_Preview {
    // message fields
    previewid: ::std::option::Option<u64>,
    sortorder: ::std::option::Option<u32>,
    url: ::protobuf::SingularField<::std::string::String>,
    size: ::std::option::Option<u32>,
    filename: ::protobuf::SingularField<::std::string::String>,
    youtubevideoid: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PublishedFileDetails_Preview {}

impl PublishedFileDetails_Preview {
    pub fn new() -> PublishedFileDetails_Preview {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublishedFileDetails_Preview {
        static mut instance: ::protobuf::lazy::Lazy<PublishedFileDetails_Preview> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublishedFileDetails_Preview,
        };
        unsafe {
            instance.get(PublishedFileDetails_Preview::new)
        }
    }

    // optional uint64 previewid = 1;

    pub fn clear_previewid(&mut self) {
        self.previewid = ::std::option::Option::None;
    }

    pub fn has_previewid(&self) -> bool {
        self.previewid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_previewid(&mut self, v: u64) {
        self.previewid = ::std::option::Option::Some(v);
    }

    pub fn get_previewid(&self) -> u64 {
        self.previewid.unwrap_or(0)
    }

    fn get_previewid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.previewid
    }

    fn mut_previewid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.previewid
    }

    // optional uint32 sortorder = 2;

    pub fn clear_sortorder(&mut self) {
        self.sortorder = ::std::option::Option::None;
    }

    pub fn has_sortorder(&self) -> bool {
        self.sortorder.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sortorder(&mut self, v: u32) {
        self.sortorder = ::std::option::Option::Some(v);
    }

    pub fn get_sortorder(&self) -> u32 {
        self.sortorder.unwrap_or(0)
    }

    fn get_sortorder_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sortorder
    }

    fn mut_sortorder_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sortorder
    }

    // optional string url = 3;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        if self.url.is_none() {
            self.url.set_default();
        }
        self.url.as_mut().unwrap()
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        self.url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        match self.url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.url
    }

    fn mut_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.url
    }

    // optional uint32 size = 4;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: u32) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> u32 {
        self.size.unwrap_or(0)
    }

    fn get_size_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.size
    }

    // optional string filename = 5;

    pub fn clear_filename(&mut self) {
        self.filename.clear();
    }

    pub fn has_filename(&self) -> bool {
        self.filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filename(&mut self, v: ::std::string::String) {
        self.filename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filename(&mut self) -> &mut ::std::string::String {
        if self.filename.is_none() {
            self.filename.set_default();
        }
        self.filename.as_mut().unwrap()
    }

    // Take field
    pub fn take_filename(&mut self) -> ::std::string::String {
        self.filename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_filename(&self) -> &str {
        match self.filename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_filename_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.filename
    }

    fn mut_filename_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.filename
    }

    // optional string youtubevideoid = 6;

    pub fn clear_youtubevideoid(&mut self) {
        self.youtubevideoid.clear();
    }

    pub fn has_youtubevideoid(&self) -> bool {
        self.youtubevideoid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_youtubevideoid(&mut self, v: ::std::string::String) {
        self.youtubevideoid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_youtubevideoid(&mut self) -> &mut ::std::string::String {
        if self.youtubevideoid.is_none() {
            self.youtubevideoid.set_default();
        }
        self.youtubevideoid.as_mut().unwrap()
    }

    // Take field
    pub fn take_youtubevideoid(&mut self) -> ::std::string::String {
        self.youtubevideoid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_youtubevideoid(&self) -> &str {
        match self.youtubevideoid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_youtubevideoid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.youtubevideoid
    }

    fn mut_youtubevideoid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.youtubevideoid
    }
}

impl ::protobuf::Message for PublishedFileDetails_Preview {
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
                    self.previewid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.sortorder = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.size = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.filename)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.youtubevideoid)?;
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
        if let Some(v) = self.previewid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.sortorder {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.url.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.size {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.filename.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.youtubevideoid.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.previewid {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.sortorder {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.url.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.size {
            os.write_uint32(4, v)?;
        }
        if let Some(ref v) = self.filename.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.youtubevideoid.as_ref() {
            os.write_string(6, &v)?;
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

impl ::protobuf::MessageStatic for PublishedFileDetails_Preview {
    fn new() -> PublishedFileDetails_Preview {
        PublishedFileDetails_Preview::new()
    }

    fn descriptor_static(_: ::std::option::Option<PublishedFileDetails_Preview>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "previewid",
                    PublishedFileDetails_Preview::get_previewid_for_reflect,
                    PublishedFileDetails_Preview::mut_previewid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sortorder",
                    PublishedFileDetails_Preview::get_sortorder_for_reflect,
                    PublishedFileDetails_Preview::mut_sortorder_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url",
                    PublishedFileDetails_Preview::get_url_for_reflect,
                    PublishedFileDetails_Preview::mut_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "size",
                    PublishedFileDetails_Preview::get_size_for_reflect,
                    PublishedFileDetails_Preview::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filename",
                    PublishedFileDetails_Preview::get_filename_for_reflect,
                    PublishedFileDetails_Preview::mut_filename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "youtubevideoid",
                    PublishedFileDetails_Preview::get_youtubevideoid_for_reflect,
                    PublishedFileDetails_Preview::mut_youtubevideoid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublishedFileDetails_Preview>(
                    "PublishedFileDetails_Preview",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublishedFileDetails_Preview {
    fn clear(&mut self) {
        self.clear_previewid();
        self.clear_sortorder();
        self.clear_url();
        self.clear_size();
        self.clear_filename();
        self.clear_youtubevideoid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PublishedFileDetails_Preview {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PublishedFileDetails_Preview {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PublishedFileDetails_Child {
    // message fields
    publishedfileid: ::std::option::Option<u64>,
    sortorder: ::std::option::Option<u32>,
    file_type: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PublishedFileDetails_Child {}

impl PublishedFileDetails_Child {
    pub fn new() -> PublishedFileDetails_Child {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublishedFileDetails_Child {
        static mut instance: ::protobuf::lazy::Lazy<PublishedFileDetails_Child> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublishedFileDetails_Child,
        };
        unsafe {
            instance.get(PublishedFileDetails_Child::new)
        }
    }

    // optional uint64 publishedfileid = 1;

    pub fn clear_publishedfileid(&mut self) {
        self.publishedfileid = ::std::option::Option::None;
    }

    pub fn has_publishedfileid(&self) -> bool {
        self.publishedfileid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publishedfileid(&mut self, v: u64) {
        self.publishedfileid = ::std::option::Option::Some(v);
    }

    pub fn get_publishedfileid(&self) -> u64 {
        self.publishedfileid.unwrap_or(0)
    }

    fn get_publishedfileid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.publishedfileid
    }

    fn mut_publishedfileid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.publishedfileid
    }

    // optional uint32 sortorder = 2;

    pub fn clear_sortorder(&mut self) {
        self.sortorder = ::std::option::Option::None;
    }

    pub fn has_sortorder(&self) -> bool {
        self.sortorder.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sortorder(&mut self, v: u32) {
        self.sortorder = ::std::option::Option::Some(v);
    }

    pub fn get_sortorder(&self) -> u32 {
        self.sortorder.unwrap_or(0)
    }

    fn get_sortorder_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sortorder
    }

    fn mut_sortorder_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sortorder
    }

    // optional uint32 file_type = 3;

    pub fn clear_file_type(&mut self) {
        self.file_type = ::std::option::Option::None;
    }

    pub fn has_file_type(&self) -> bool {
        self.file_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_type(&mut self, v: u32) {
        self.file_type = ::std::option::Option::Some(v);
    }

    pub fn get_file_type(&self) -> u32 {
        self.file_type.unwrap_or(0)
    }

    fn get_file_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.file_type
    }

    fn mut_file_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.file_type
    }
}

impl ::protobuf::Message for PublishedFileDetails_Child {
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
                    self.publishedfileid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.sortorder = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.file_type = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.publishedfileid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.sortorder {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.file_type {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.publishedfileid {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.sortorder {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.file_type {
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

impl ::protobuf::MessageStatic for PublishedFileDetails_Child {
    fn new() -> PublishedFileDetails_Child {
        PublishedFileDetails_Child::new()
    }

    fn descriptor_static(_: ::std::option::Option<PublishedFileDetails_Child>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "publishedfileid",
                    PublishedFileDetails_Child::get_publishedfileid_for_reflect,
                    PublishedFileDetails_Child::mut_publishedfileid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sortorder",
                    PublishedFileDetails_Child::get_sortorder_for_reflect,
                    PublishedFileDetails_Child::mut_sortorder_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "file_type",
                    PublishedFileDetails_Child::get_file_type_for_reflect,
                    PublishedFileDetails_Child::mut_file_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublishedFileDetails_Child>(
                    "PublishedFileDetails_Child",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublishedFileDetails_Child {
    fn clear(&mut self) {
        self.clear_publishedfileid();
        self.clear_sortorder();
        self.clear_file_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PublishedFileDetails_Child {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PublishedFileDetails_Child {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PublishedFileDetails_KVTag {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PublishedFileDetails_KVTag {}

impl PublishedFileDetails_KVTag {
    pub fn new() -> PublishedFileDetails_KVTag {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublishedFileDetails_KVTag {
        static mut instance: ::protobuf::lazy::Lazy<PublishedFileDetails_KVTag> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublishedFileDetails_KVTag,
        };
        unsafe {
            instance.get(PublishedFileDetails_KVTag::new)
        }
    }

    // optional string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.key
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

impl ::protobuf::Message for PublishedFileDetails_KVTag {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key)?;
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
        if let Some(ref v) = self.key.as_ref() {
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
        if let Some(ref v) = self.key.as_ref() {
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

impl ::protobuf::MessageStatic for PublishedFileDetails_KVTag {
    fn new() -> PublishedFileDetails_KVTag {
        PublishedFileDetails_KVTag::new()
    }

    fn descriptor_static(_: ::std::option::Option<PublishedFileDetails_KVTag>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    PublishedFileDetails_KVTag::get_key_for_reflect,
                    PublishedFileDetails_KVTag::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    PublishedFileDetails_KVTag::get_value_for_reflect,
                    PublishedFileDetails_KVTag::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublishedFileDetails_KVTag>(
                    "PublishedFileDetails_KVTag",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublishedFileDetails_KVTag {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PublishedFileDetails_KVTag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PublishedFileDetails_KVTag {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PublishedFileDetails_VoteData {
    // message fields
    score: ::std::option::Option<f32>,
    votes_up: ::std::option::Option<u32>,
    votes_down: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PublishedFileDetails_VoteData {}

impl PublishedFileDetails_VoteData {
    pub fn new() -> PublishedFileDetails_VoteData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublishedFileDetails_VoteData {
        static mut instance: ::protobuf::lazy::Lazy<PublishedFileDetails_VoteData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublishedFileDetails_VoteData,
        };
        unsafe {
            instance.get(PublishedFileDetails_VoteData::new)
        }
    }

    // optional float score = 1;

    pub fn clear_score(&mut self) {
        self.score = ::std::option::Option::None;
    }

    pub fn has_score(&self) -> bool {
        self.score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_score(&mut self, v: f32) {
        self.score = ::std::option::Option::Some(v);
    }

    pub fn get_score(&self) -> f32 {
        self.score.unwrap_or(0.)
    }

    fn get_score_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.score
    }

    fn mut_score_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.score
    }

    // optional uint32 votes_up = 2;

    pub fn clear_votes_up(&mut self) {
        self.votes_up = ::std::option::Option::None;
    }

    pub fn has_votes_up(&self) -> bool {
        self.votes_up.is_some()
    }

    // Param is passed by value, moved
    pub fn set_votes_up(&mut self, v: u32) {
        self.votes_up = ::std::option::Option::Some(v);
    }

    pub fn get_votes_up(&self) -> u32 {
        self.votes_up.unwrap_or(0)
    }

    fn get_votes_up_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.votes_up
    }

    fn mut_votes_up_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.votes_up
    }

    // optional uint32 votes_down = 3;

    pub fn clear_votes_down(&mut self) {
        self.votes_down = ::std::option::Option::None;
    }

    pub fn has_votes_down(&self) -> bool {
        self.votes_down.is_some()
    }

    // Param is passed by value, moved
    pub fn set_votes_down(&mut self, v: u32) {
        self.votes_down = ::std::option::Option::Some(v);
    }

    pub fn get_votes_down(&self) -> u32 {
        self.votes_down.unwrap_or(0)
    }

    fn get_votes_down_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.votes_down
    }

    fn mut_votes_down_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.votes_down
    }
}

impl ::protobuf::Message for PublishedFileDetails_VoteData {
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
                    self.score = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.votes_up = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.votes_down = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.score {
            my_size += 5;
        }
        if let Some(v) = self.votes_up {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.votes_down {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.score {
            os.write_float(1, v)?;
        }
        if let Some(v) = self.votes_up {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.votes_down {
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

impl ::protobuf::MessageStatic for PublishedFileDetails_VoteData {
    fn new() -> PublishedFileDetails_VoteData {
        PublishedFileDetails_VoteData::new()
    }

    fn descriptor_static(_: ::std::option::Option<PublishedFileDetails_VoteData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "score",
                    PublishedFileDetails_VoteData::get_score_for_reflect,
                    PublishedFileDetails_VoteData::mut_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "votes_up",
                    PublishedFileDetails_VoteData::get_votes_up_for_reflect,
                    PublishedFileDetails_VoteData::mut_votes_up_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "votes_down",
                    PublishedFileDetails_VoteData::get_votes_down_for_reflect,
                    PublishedFileDetails_VoteData::mut_votes_down_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublishedFileDetails_VoteData>(
                    "PublishedFileDetails_VoteData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublishedFileDetails_VoteData {
    fn clear(&mut self) {
        self.clear_score();
        self.clear_votes_up();
        self.clear_votes_down();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PublishedFileDetails_VoteData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PublishedFileDetails_VoteData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CPublishedFile_GetDetails_Response {
    // message fields
    publishedfiledetails: ::protobuf::RepeatedField<PublishedFileDetails>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPublishedFile_GetDetails_Response {}

impl CPublishedFile_GetDetails_Response {
    pub fn new() -> CPublishedFile_GetDetails_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPublishedFile_GetDetails_Response {
        static mut instance: ::protobuf::lazy::Lazy<CPublishedFile_GetDetails_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPublishedFile_GetDetails_Response,
        };
        unsafe {
            instance.get(CPublishedFile_GetDetails_Response::new)
        }
    }

    // repeated .PublishedFileDetails publishedfiledetails = 1;

    pub fn clear_publishedfiledetails(&mut self) {
        self.publishedfiledetails.clear();
    }

    // Param is passed by value, moved
    pub fn set_publishedfiledetails(&mut self, v: ::protobuf::RepeatedField<PublishedFileDetails>) {
        self.publishedfiledetails = v;
    }

    // Mutable pointer to the field.
    pub fn mut_publishedfiledetails(&mut self) -> &mut ::protobuf::RepeatedField<PublishedFileDetails> {
        &mut self.publishedfiledetails
    }

    // Take field
    pub fn take_publishedfiledetails(&mut self) -> ::protobuf::RepeatedField<PublishedFileDetails> {
        ::std::mem::replace(&mut self.publishedfiledetails, ::protobuf::RepeatedField::new())
    }

    pub fn get_publishedfiledetails(&self) -> &[PublishedFileDetails] {
        &self.publishedfiledetails
    }

    fn get_publishedfiledetails_for_reflect(&self) -> &::protobuf::RepeatedField<PublishedFileDetails> {
        &self.publishedfiledetails
    }

    fn mut_publishedfiledetails_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PublishedFileDetails> {
        &mut self.publishedfiledetails
    }
}

impl ::protobuf::Message for CPublishedFile_GetDetails_Response {
    fn is_initialized(&self) -> bool {
        for v in &self.publishedfiledetails {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.publishedfiledetails)?;
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
        for value in &self.publishedfiledetails {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.publishedfiledetails {
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

impl ::protobuf::MessageStatic for CPublishedFile_GetDetails_Response {
    fn new() -> CPublishedFile_GetDetails_Response {
        CPublishedFile_GetDetails_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPublishedFile_GetDetails_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PublishedFileDetails>>(
                    "publishedfiledetails",
                    CPublishedFile_GetDetails_Response::get_publishedfiledetails_for_reflect,
                    CPublishedFile_GetDetails_Response::mut_publishedfiledetails_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CPublishedFile_GetDetails_Response>(
                    "CPublishedFile_GetDetails_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPublishedFile_GetDetails_Response {
    fn clear(&mut self) {
        self.clear_publishedfiledetails();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPublishedFile_GetDetails_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPublishedFile_GetDetails_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CPublishedFile_GetUserFiles_Request {
    // message fields
    appid: ::std::option::Option<u32>,
    page: ::std::option::Option<u32>,
    numperpage: ::std::option::Option<u32>,
    sortmethod: ::protobuf::SingularField<::std::string::String>,
    totalonly: ::std::option::Option<bool>,
    privacy: ::std::option::Option<u32>,
    ids_only: ::std::option::Option<bool>,
    requiredtags: ::protobuf::RepeatedField<::std::string::String>,
    excludedtags: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPublishedFile_GetUserFiles_Request {}

impl CPublishedFile_GetUserFiles_Request {
    pub fn new() -> CPublishedFile_GetUserFiles_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPublishedFile_GetUserFiles_Request {
        static mut instance: ::protobuf::lazy::Lazy<CPublishedFile_GetUserFiles_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPublishedFile_GetUserFiles_Request,
        };
        unsafe {
            instance.get(CPublishedFile_GetUserFiles_Request::new)
        }
    }

    // optional uint32 appid = 1;

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: u32) {
        self.appid = ::std::option::Option::Some(v);
    }

    pub fn get_appid(&self) -> u32 {
        self.appid.unwrap_or(0)
    }

    fn get_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.appid
    }

    fn mut_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.appid
    }

    // optional uint32 page = 3;

    pub fn clear_page(&mut self) {
        self.page = ::std::option::Option::None;
    }

    pub fn has_page(&self) -> bool {
        self.page.is_some()
    }

    // Param is passed by value, moved
    pub fn set_page(&mut self, v: u32) {
        self.page = ::std::option::Option::Some(v);
    }

    pub fn get_page(&self) -> u32 {
        self.page.unwrap_or(1u32)
    }

    fn get_page_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.page
    }

    fn mut_page_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.page
    }

    // optional uint32 numperpage = 4;

    pub fn clear_numperpage(&mut self) {
        self.numperpage = ::std::option::Option::None;
    }

    pub fn has_numperpage(&self) -> bool {
        self.numperpage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numperpage(&mut self, v: u32) {
        self.numperpage = ::std::option::Option::Some(v);
    }

    pub fn get_numperpage(&self) -> u32 {
        self.numperpage.unwrap_or(1u32)
    }

    fn get_numperpage_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.numperpage
    }

    fn mut_numperpage_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.numperpage
    }

    // optional string sortmethod = 6;

    pub fn clear_sortmethod(&mut self) {
        self.sortmethod.clear();
    }

    pub fn has_sortmethod(&self) -> bool {
        self.sortmethod.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sortmethod(&mut self, v: ::std::string::String) {
        self.sortmethod = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sortmethod(&mut self) -> &mut ::std::string::String {
        if self.sortmethod.is_none() {
            self.sortmethod.set_default();
        }
        self.sortmethod.as_mut().unwrap()
    }

    // Take field
    pub fn take_sortmethod(&mut self) -> ::std::string::String {
        self.sortmethod.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sortmethod(&self) -> &str {
        match self.sortmethod.as_ref() {
            Some(v) => &v,
            None => "lastupdated",
        }
    }

    fn get_sortmethod_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.sortmethod
    }

    fn mut_sortmethod_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.sortmethod
    }

    // optional bool totalonly = 7;

    pub fn clear_totalonly(&mut self) {
        self.totalonly = ::std::option::Option::None;
    }

    pub fn has_totalonly(&self) -> bool {
        self.totalonly.is_some()
    }

    // Param is passed by value, moved
    pub fn set_totalonly(&mut self, v: bool) {
        self.totalonly = ::std::option::Option::Some(v);
    }

    pub fn get_totalonly(&self) -> bool {
        self.totalonly.unwrap_or(false)
    }

    fn get_totalonly_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.totalonly
    }

    fn mut_totalonly_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.totalonly
    }

    // optional uint32 privacy = 9;

    pub fn clear_privacy(&mut self) {
        self.privacy = ::std::option::Option::None;
    }

    pub fn has_privacy(&self) -> bool {
        self.privacy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_privacy(&mut self, v: u32) {
        self.privacy = ::std::option::Option::Some(v);
    }

    pub fn get_privacy(&self) -> u32 {
        self.privacy.unwrap_or(0)
    }

    fn get_privacy_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.privacy
    }

    fn mut_privacy_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.privacy
    }

    // optional bool ids_only = 10;

    pub fn clear_ids_only(&mut self) {
        self.ids_only = ::std::option::Option::None;
    }

    pub fn has_ids_only(&self) -> bool {
        self.ids_only.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ids_only(&mut self, v: bool) {
        self.ids_only = ::std::option::Option::Some(v);
    }

    pub fn get_ids_only(&self) -> bool {
        self.ids_only.unwrap_or(false)
    }

    fn get_ids_only_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.ids_only
    }

    fn mut_ids_only_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.ids_only
    }

    // repeated string requiredtags = 11;

    pub fn clear_requiredtags(&mut self) {
        self.requiredtags.clear();
    }

    // Param is passed by value, moved
    pub fn set_requiredtags(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.requiredtags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_requiredtags(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.requiredtags
    }

    // Take field
    pub fn take_requiredtags(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.requiredtags, ::protobuf::RepeatedField::new())
    }

    pub fn get_requiredtags(&self) -> &[::std::string::String] {
        &self.requiredtags
    }

    fn get_requiredtags_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.requiredtags
    }

    fn mut_requiredtags_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.requiredtags
    }

    // repeated string excludedtags = 12;

    pub fn clear_excludedtags(&mut self) {
        self.excludedtags.clear();
    }

    // Param is passed by value, moved
    pub fn set_excludedtags(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.excludedtags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_excludedtags(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.excludedtags
    }

    // Take field
    pub fn take_excludedtags(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.excludedtags, ::protobuf::RepeatedField::new())
    }

    pub fn get_excludedtags(&self) -> &[::std::string::String] {
        &self.excludedtags
    }

    fn get_excludedtags_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.excludedtags
    }

    fn mut_excludedtags_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.excludedtags
    }
}

impl ::protobuf::Message for CPublishedFile_GetUserFiles_Request {
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
                    self.appid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.page = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.numperpage = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sortmethod)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.totalonly = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.privacy = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.ids_only = ::std::option::Option::Some(tmp);
                },
                11 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.requiredtags)?;
                },
                12 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.excludedtags)?;
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
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.page {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numperpage {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.sortmethod.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(v) = self.totalonly {
            my_size += 2;
        }
        if let Some(v) = self.privacy {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ids_only {
            my_size += 2;
        }
        for value in &self.requiredtags {
            my_size += ::protobuf::rt::string_size(11, &value);
        };
        for value in &self.excludedtags {
            my_size += ::protobuf::rt::string_size(12, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.appid {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.page {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.numperpage {
            os.write_uint32(4, v)?;
        }
        if let Some(ref v) = self.sortmethod.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(v) = self.totalonly {
            os.write_bool(7, v)?;
        }
        if let Some(v) = self.privacy {
            os.write_uint32(9, v)?;
        }
        if let Some(v) = self.ids_only {
            os.write_bool(10, v)?;
        }
        for v in &self.requiredtags {
            os.write_string(11, &v)?;
        };
        for v in &self.excludedtags {
            os.write_string(12, &v)?;
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

impl ::protobuf::MessageStatic for CPublishedFile_GetUserFiles_Request {
    fn new() -> CPublishedFile_GetUserFiles_Request {
        CPublishedFile_GetUserFiles_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPublishedFile_GetUserFiles_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CPublishedFile_GetUserFiles_Request::get_appid_for_reflect,
                    CPublishedFile_GetUserFiles_Request::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "page",
                    CPublishedFile_GetUserFiles_Request::get_page_for_reflect,
                    CPublishedFile_GetUserFiles_Request::mut_page_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "numperpage",
                    CPublishedFile_GetUserFiles_Request::get_numperpage_for_reflect,
                    CPublishedFile_GetUserFiles_Request::mut_numperpage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sortmethod",
                    CPublishedFile_GetUserFiles_Request::get_sortmethod_for_reflect,
                    CPublishedFile_GetUserFiles_Request::mut_sortmethod_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "totalonly",
                    CPublishedFile_GetUserFiles_Request::get_totalonly_for_reflect,
                    CPublishedFile_GetUserFiles_Request::mut_totalonly_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "privacy",
                    CPublishedFile_GetUserFiles_Request::get_privacy_for_reflect,
                    CPublishedFile_GetUserFiles_Request::mut_privacy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ids_only",
                    CPublishedFile_GetUserFiles_Request::get_ids_only_for_reflect,
                    CPublishedFile_GetUserFiles_Request::mut_ids_only_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "requiredtags",
                    CPublishedFile_GetUserFiles_Request::get_requiredtags_for_reflect,
                    CPublishedFile_GetUserFiles_Request::mut_requiredtags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "excludedtags",
                    CPublishedFile_GetUserFiles_Request::get_excludedtags_for_reflect,
                    CPublishedFile_GetUserFiles_Request::mut_excludedtags_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CPublishedFile_GetUserFiles_Request>(
                    "CPublishedFile_GetUserFiles_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPublishedFile_GetUserFiles_Request {
    fn clear(&mut self) {
        self.clear_appid();
        self.clear_page();
        self.clear_numperpage();
        self.clear_sortmethod();
        self.clear_totalonly();
        self.clear_privacy();
        self.clear_ids_only();
        self.clear_requiredtags();
        self.clear_excludedtags();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPublishedFile_GetUserFiles_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPublishedFile_GetUserFiles_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CPublishedFile_GetUserFiles_Response {
    // message fields
    total: ::std::option::Option<u32>,
    startindex: ::std::option::Option<u32>,
    publishedfiledetails: ::protobuf::RepeatedField<PublishedFileDetails>,
    apps: ::protobuf::RepeatedField<CPublishedFile_GetUserFiles_Response_App>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPublishedFile_GetUserFiles_Response {}

impl CPublishedFile_GetUserFiles_Response {
    pub fn new() -> CPublishedFile_GetUserFiles_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPublishedFile_GetUserFiles_Response {
        static mut instance: ::protobuf::lazy::Lazy<CPublishedFile_GetUserFiles_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPublishedFile_GetUserFiles_Response,
        };
        unsafe {
            instance.get(CPublishedFile_GetUserFiles_Response::new)
        }
    }

    // optional uint32 total = 1;

    pub fn clear_total(&mut self) {
        self.total = ::std::option::Option::None;
    }

    pub fn has_total(&self) -> bool {
        self.total.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total(&mut self, v: u32) {
        self.total = ::std::option::Option::Some(v);
    }

    pub fn get_total(&self) -> u32 {
        self.total.unwrap_or(0)
    }

    fn get_total_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.total
    }

    fn mut_total_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.total
    }

    // optional uint32 startindex = 2;

    pub fn clear_startindex(&mut self) {
        self.startindex = ::std::option::Option::None;
    }

    pub fn has_startindex(&self) -> bool {
        self.startindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_startindex(&mut self, v: u32) {
        self.startindex = ::std::option::Option::Some(v);
    }

    pub fn get_startindex(&self) -> u32 {
        self.startindex.unwrap_or(0)
    }

    fn get_startindex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.startindex
    }

    fn mut_startindex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.startindex
    }

    // repeated .PublishedFileDetails publishedfiledetails = 3;

    pub fn clear_publishedfiledetails(&mut self) {
        self.publishedfiledetails.clear();
    }

    // Param is passed by value, moved
    pub fn set_publishedfiledetails(&mut self, v: ::protobuf::RepeatedField<PublishedFileDetails>) {
        self.publishedfiledetails = v;
    }

    // Mutable pointer to the field.
    pub fn mut_publishedfiledetails(&mut self) -> &mut ::protobuf::RepeatedField<PublishedFileDetails> {
        &mut self.publishedfiledetails
    }

    // Take field
    pub fn take_publishedfiledetails(&mut self) -> ::protobuf::RepeatedField<PublishedFileDetails> {
        ::std::mem::replace(&mut self.publishedfiledetails, ::protobuf::RepeatedField::new())
    }

    pub fn get_publishedfiledetails(&self) -> &[PublishedFileDetails] {
        &self.publishedfiledetails
    }

    fn get_publishedfiledetails_for_reflect(&self) -> &::protobuf::RepeatedField<PublishedFileDetails> {
        &self.publishedfiledetails
    }

    fn mut_publishedfiledetails_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PublishedFileDetails> {
        &mut self.publishedfiledetails
    }

    // repeated .CPublishedFile_GetUserFiles_Response.App apps = 4;

    pub fn clear_apps(&mut self) {
        self.apps.clear();
    }

    // Param is passed by value, moved
    pub fn set_apps(&mut self, v: ::protobuf::RepeatedField<CPublishedFile_GetUserFiles_Response_App>) {
        self.apps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_apps(&mut self) -> &mut ::protobuf::RepeatedField<CPublishedFile_GetUserFiles_Response_App> {
        &mut self.apps
    }

    // Take field
    pub fn take_apps(&mut self) -> ::protobuf::RepeatedField<CPublishedFile_GetUserFiles_Response_App> {
        ::std::mem::replace(&mut self.apps, ::protobuf::RepeatedField::new())
    }

    pub fn get_apps(&self) -> &[CPublishedFile_GetUserFiles_Response_App] {
        &self.apps
    }

    fn get_apps_for_reflect(&self) -> &::protobuf::RepeatedField<CPublishedFile_GetUserFiles_Response_App> {
        &self.apps
    }

    fn mut_apps_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CPublishedFile_GetUserFiles_Response_App> {
        &mut self.apps
    }
}

impl ::protobuf::Message for CPublishedFile_GetUserFiles_Response {
    fn is_initialized(&self) -> bool {
        for v in &self.publishedfiledetails {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.apps {
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
                    self.total = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.startindex = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.publishedfiledetails)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.apps)?;
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
        if let Some(v) = self.total {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.startindex {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.publishedfiledetails {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.apps {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.total {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.startindex {
            os.write_uint32(2, v)?;
        }
        for v in &self.publishedfiledetails {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.apps {
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

impl ::protobuf::MessageStatic for CPublishedFile_GetUserFiles_Response {
    fn new() -> CPublishedFile_GetUserFiles_Response {
        CPublishedFile_GetUserFiles_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPublishedFile_GetUserFiles_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "total",
                    CPublishedFile_GetUserFiles_Response::get_total_for_reflect,
                    CPublishedFile_GetUserFiles_Response::mut_total_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "startindex",
                    CPublishedFile_GetUserFiles_Response::get_startindex_for_reflect,
                    CPublishedFile_GetUserFiles_Response::mut_startindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PublishedFileDetails>>(
                    "publishedfiledetails",
                    CPublishedFile_GetUserFiles_Response::get_publishedfiledetails_for_reflect,
                    CPublishedFile_GetUserFiles_Response::mut_publishedfiledetails_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CPublishedFile_GetUserFiles_Response_App>>(
                    "apps",
                    CPublishedFile_GetUserFiles_Response::get_apps_for_reflect,
                    CPublishedFile_GetUserFiles_Response::mut_apps_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CPublishedFile_GetUserFiles_Response>(
                    "CPublishedFile_GetUserFiles_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPublishedFile_GetUserFiles_Response {
    fn clear(&mut self) {
        self.clear_total();
        self.clear_startindex();
        self.clear_publishedfiledetails();
        self.clear_apps();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPublishedFile_GetUserFiles_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPublishedFile_GetUserFiles_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CPublishedFile_GetUserFiles_Response_App {
    // message fields
    appid: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    shortcutid: ::std::option::Option<u32>,
    private: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPublishedFile_GetUserFiles_Response_App {}

impl CPublishedFile_GetUserFiles_Response_App {
    pub fn new() -> CPublishedFile_GetUserFiles_Response_App {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPublishedFile_GetUserFiles_Response_App {
        static mut instance: ::protobuf::lazy::Lazy<CPublishedFile_GetUserFiles_Response_App> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPublishedFile_GetUserFiles_Response_App,
        };
        unsafe {
            instance.get(CPublishedFile_GetUserFiles_Response_App::new)
        }
    }

    // optional uint32 appid = 1;

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: u32) {
        self.appid = ::std::option::Option::Some(v);
    }

    pub fn get_appid(&self) -> u32 {
        self.appid.unwrap_or(0)
    }

    fn get_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.appid
    }

    fn mut_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.appid
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

    // optional uint32 shortcutid = 3;

    pub fn clear_shortcutid(&mut self) {
        self.shortcutid = ::std::option::Option::None;
    }

    pub fn has_shortcutid(&self) -> bool {
        self.shortcutid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shortcutid(&mut self, v: u32) {
        self.shortcutid = ::std::option::Option::Some(v);
    }

    pub fn get_shortcutid(&self) -> u32 {
        self.shortcutid.unwrap_or(0)
    }

    fn get_shortcutid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.shortcutid
    }

    fn mut_shortcutid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.shortcutid
    }

    // optional bool private = 4;

    pub fn clear_private(&mut self) {
        self.private = ::std::option::Option::None;
    }

    pub fn has_private(&self) -> bool {
        self.private.is_some()
    }

    // Param is passed by value, moved
    pub fn set_private(&mut self, v: bool) {
        self.private = ::std::option::Option::Some(v);
    }

    pub fn get_private(&self) -> bool {
        self.private.unwrap_or(false)
    }

    fn get_private_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.private
    }

    fn mut_private_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.private
    }
}

impl ::protobuf::Message for CPublishedFile_GetUserFiles_Response_App {
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
                    self.appid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.shortcutid = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.private = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.shortcutid {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.private {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.appid {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.shortcutid {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.private {
            os.write_bool(4, v)?;
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

impl ::protobuf::MessageStatic for CPublishedFile_GetUserFiles_Response_App {
    fn new() -> CPublishedFile_GetUserFiles_Response_App {
        CPublishedFile_GetUserFiles_Response_App::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPublishedFile_GetUserFiles_Response_App>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CPublishedFile_GetUserFiles_Response_App::get_appid_for_reflect,
                    CPublishedFile_GetUserFiles_Response_App::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CPublishedFile_GetUserFiles_Response_App::get_name_for_reflect,
                    CPublishedFile_GetUserFiles_Response_App::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "shortcutid",
                    CPublishedFile_GetUserFiles_Response_App::get_shortcutid_for_reflect,
                    CPublishedFile_GetUserFiles_Response_App::mut_shortcutid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "private",
                    CPublishedFile_GetUserFiles_Response_App::get_private_for_reflect,
                    CPublishedFile_GetUserFiles_Response_App::mut_private_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CPublishedFile_GetUserFiles_Response_App>(
                    "CPublishedFile_GetUserFiles_Response_App",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPublishedFile_GetUserFiles_Response_App {
    fn clear(&mut self) {
        self.clear_appid();
        self.clear_name();
        self.clear_shortcutid();
        self.clear_private();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPublishedFile_GetUserFiles_Response_App {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPublishedFile_GetUserFiles_Response_App {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CPublishedFile_Update_Request {
    // message fields
    appid: ::std::option::Option<u32>,
    publishedfileid: ::std::option::Option<u64>,
    title: ::protobuf::SingularField<::std::string::String>,
    file_description: ::protobuf::SingularField<::std::string::String>,
    visibility: ::std::option::Option<u32>,
    tags: ::protobuf::RepeatedField<::std::string::String>,
    filename: ::protobuf::SingularField<::std::string::String>,
    preview_filename: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPublishedFile_Update_Request {}

impl CPublishedFile_Update_Request {
    pub fn new() -> CPublishedFile_Update_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPublishedFile_Update_Request {
        static mut instance: ::protobuf::lazy::Lazy<CPublishedFile_Update_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPublishedFile_Update_Request,
        };
        unsafe {
            instance.get(CPublishedFile_Update_Request::new)
        }
    }

    // optional uint32 appid = 1;

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: u32) {
        self.appid = ::std::option::Option::Some(v);
    }

    pub fn get_appid(&self) -> u32 {
        self.appid.unwrap_or(0)
    }

    fn get_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.appid
    }

    fn mut_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.appid
    }

    // optional fixed64 publishedfileid = 2;

    pub fn clear_publishedfileid(&mut self) {
        self.publishedfileid = ::std::option::Option::None;
    }

    pub fn has_publishedfileid(&self) -> bool {
        self.publishedfileid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publishedfileid(&mut self, v: u64) {
        self.publishedfileid = ::std::option::Option::Some(v);
    }

    pub fn get_publishedfileid(&self) -> u64 {
        self.publishedfileid.unwrap_or(0)
    }

    fn get_publishedfileid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.publishedfileid
    }

    fn mut_publishedfileid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.publishedfileid
    }

    // optional string title = 3;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    pub fn has_title(&self) -> bool {
        self.title.is_some()
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        if self.title.is_none() {
            self.title.set_default();
        }
        self.title.as_mut().unwrap()
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        self.title.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        match self.title.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_title_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.title
    }

    // optional string file_description = 4;

    pub fn clear_file_description(&mut self) {
        self.file_description.clear();
    }

    pub fn has_file_description(&self) -> bool {
        self.file_description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_description(&mut self, v: ::std::string::String) {
        self.file_description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_description(&mut self) -> &mut ::std::string::String {
        if self.file_description.is_none() {
            self.file_description.set_default();
        }
        self.file_description.as_mut().unwrap()
    }

    // Take field
    pub fn take_file_description(&mut self) -> ::std::string::String {
        self.file_description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_file_description(&self) -> &str {
        match self.file_description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_file_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.file_description
    }

    fn mut_file_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.file_description
    }

    // optional uint32 visibility = 5;

    pub fn clear_visibility(&mut self) {
        self.visibility = ::std::option::Option::None;
    }

    pub fn has_visibility(&self) -> bool {
        self.visibility.is_some()
    }

    // Param is passed by value, moved
    pub fn set_visibility(&mut self, v: u32) {
        self.visibility = ::std::option::Option::Some(v);
    }

    pub fn get_visibility(&self) -> u32 {
        self.visibility.unwrap_or(0)
    }

    fn get_visibility_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.visibility
    }

    fn mut_visibility_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.visibility
    }

    // repeated string tags = 6;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags(&self) -> &[::std::string::String] {
        &self.tags
    }

    fn get_tags_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.tags
    }

    fn mut_tags_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tags
    }

    // optional string filename = 7;

    pub fn clear_filename(&mut self) {
        self.filename.clear();
    }

    pub fn has_filename(&self) -> bool {
        self.filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filename(&mut self, v: ::std::string::String) {
        self.filename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filename(&mut self) -> &mut ::std::string::String {
        if self.filename.is_none() {
            self.filename.set_default();
        }
        self.filename.as_mut().unwrap()
    }

    // Take field
    pub fn take_filename(&mut self) -> ::std::string::String {
        self.filename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_filename(&self) -> &str {
        match self.filename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_filename_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.filename
    }

    fn mut_filename_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.filename
    }

    // optional string preview_filename = 8;

    pub fn clear_preview_filename(&mut self) {
        self.preview_filename.clear();
    }

    pub fn has_preview_filename(&self) -> bool {
        self.preview_filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_preview_filename(&mut self, v: ::std::string::String) {
        self.preview_filename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_preview_filename(&mut self) -> &mut ::std::string::String {
        if self.preview_filename.is_none() {
            self.preview_filename.set_default();
        }
        self.preview_filename.as_mut().unwrap()
    }

    // Take field
    pub fn take_preview_filename(&mut self) -> ::std::string::String {
        self.preview_filename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_preview_filename(&self) -> &str {
        match self.preview_filename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_preview_filename_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.preview_filename
    }

    fn mut_preview_filename_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.preview_filename
    }
}

impl ::protobuf::Message for CPublishedFile_Update_Request {
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
                    self.appid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.publishedfileid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.title)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.file_description)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.visibility = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.tags)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.filename)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.preview_filename)?;
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
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.publishedfileid {
            my_size += 9;
        }
        if let Some(ref v) = self.title.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.file_description.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(v) = self.visibility {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.tags {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        if let Some(ref v) = self.filename.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        if let Some(ref v) = self.preview_filename.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.appid {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.publishedfileid {
            os.write_fixed64(2, v)?;
        }
        if let Some(ref v) = self.title.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.file_description.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(v) = self.visibility {
            os.write_uint32(5, v)?;
        }
        for v in &self.tags {
            os.write_string(6, &v)?;
        };
        if let Some(ref v) = self.filename.as_ref() {
            os.write_string(7, &v)?;
        }
        if let Some(ref v) = self.preview_filename.as_ref() {
            os.write_string(8, &v)?;
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

impl ::protobuf::MessageStatic for CPublishedFile_Update_Request {
    fn new() -> CPublishedFile_Update_Request {
        CPublishedFile_Update_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPublishedFile_Update_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CPublishedFile_Update_Request::get_appid_for_reflect,
                    CPublishedFile_Update_Request::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "publishedfileid",
                    CPublishedFile_Update_Request::get_publishedfileid_for_reflect,
                    CPublishedFile_Update_Request::mut_publishedfileid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    CPublishedFile_Update_Request::get_title_for_reflect,
                    CPublishedFile_Update_Request::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "file_description",
                    CPublishedFile_Update_Request::get_file_description_for_reflect,
                    CPublishedFile_Update_Request::mut_file_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "visibility",
                    CPublishedFile_Update_Request::get_visibility_for_reflect,
                    CPublishedFile_Update_Request::mut_visibility_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tags",
                    CPublishedFile_Update_Request::get_tags_for_reflect,
                    CPublishedFile_Update_Request::mut_tags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filename",
                    CPublishedFile_Update_Request::get_filename_for_reflect,
                    CPublishedFile_Update_Request::mut_filename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "preview_filename",
                    CPublishedFile_Update_Request::get_preview_filename_for_reflect,
                    CPublishedFile_Update_Request::mut_preview_filename_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CPublishedFile_Update_Request>(
                    "CPublishedFile_Update_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPublishedFile_Update_Request {
    fn clear(&mut self) {
        self.clear_appid();
        self.clear_publishedfileid();
        self.clear_title();
        self.clear_file_description();
        self.clear_visibility();
        self.clear_tags();
        self.clear_filename();
        self.clear_preview_filename();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPublishedFile_Update_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPublishedFile_Update_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CPublishedFile_Update_Response {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPublishedFile_Update_Response {}

impl CPublishedFile_Update_Response {
    pub fn new() -> CPublishedFile_Update_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPublishedFile_Update_Response {
        static mut instance: ::protobuf::lazy::Lazy<CPublishedFile_Update_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPublishedFile_Update_Response,
        };
        unsafe {
            instance.get(CPublishedFile_Update_Response::new)
        }
    }
}

impl ::protobuf::Message for CPublishedFile_Update_Response {
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

impl ::protobuf::MessageStatic for CPublishedFile_Update_Response {
    fn new() -> CPublishedFile_Update_Response {
        CPublishedFile_Update_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPublishedFile_Update_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CPublishedFile_Update_Response>(
                    "CPublishedFile_Update_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPublishedFile_Update_Response {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPublishedFile_Update_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPublishedFile_Update_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CPublishedFile_RefreshVotingQueue_Request {
    // message fields
    appid: ::std::option::Option<u32>,
    matching_file_type: ::std::option::Option<u32>,
    tags: ::protobuf::RepeatedField<::std::string::String>,
    match_all_tags: ::std::option::Option<bool>,
    excluded_tags: ::protobuf::RepeatedField<::std::string::String>,
    desired_queue_size: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPublishedFile_RefreshVotingQueue_Request {}

impl CPublishedFile_RefreshVotingQueue_Request {
    pub fn new() -> CPublishedFile_RefreshVotingQueue_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPublishedFile_RefreshVotingQueue_Request {
        static mut instance: ::protobuf::lazy::Lazy<CPublishedFile_RefreshVotingQueue_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPublishedFile_RefreshVotingQueue_Request,
        };
        unsafe {
            instance.get(CPublishedFile_RefreshVotingQueue_Request::new)
        }
    }

    // optional uint32 appid = 1;

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: u32) {
        self.appid = ::std::option::Option::Some(v);
    }

    pub fn get_appid(&self) -> u32 {
        self.appid.unwrap_or(0)
    }

    fn get_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.appid
    }

    fn mut_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.appid
    }

    // optional uint32 matching_file_type = 2;

    pub fn clear_matching_file_type(&mut self) {
        self.matching_file_type = ::std::option::Option::None;
    }

    pub fn has_matching_file_type(&self) -> bool {
        self.matching_file_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_matching_file_type(&mut self, v: u32) {
        self.matching_file_type = ::std::option::Option::Some(v);
    }

    pub fn get_matching_file_type(&self) -> u32 {
        self.matching_file_type.unwrap_or(0)
    }

    fn get_matching_file_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.matching_file_type
    }

    fn mut_matching_file_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.matching_file_type
    }

    // repeated string tags = 3;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags(&self) -> &[::std::string::String] {
        &self.tags
    }

    fn get_tags_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.tags
    }

    fn mut_tags_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tags
    }

    // optional bool match_all_tags = 4;

    pub fn clear_match_all_tags(&mut self) {
        self.match_all_tags = ::std::option::Option::None;
    }

    pub fn has_match_all_tags(&self) -> bool {
        self.match_all_tags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_all_tags(&mut self, v: bool) {
        self.match_all_tags = ::std::option::Option::Some(v);
    }

    pub fn get_match_all_tags(&self) -> bool {
        self.match_all_tags.unwrap_or(true)
    }

    fn get_match_all_tags_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.match_all_tags
    }

    fn mut_match_all_tags_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.match_all_tags
    }

    // repeated string excluded_tags = 5;

    pub fn clear_excluded_tags(&mut self) {
        self.excluded_tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_excluded_tags(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.excluded_tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_excluded_tags(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.excluded_tags
    }

    // Take field
    pub fn take_excluded_tags(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.excluded_tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_excluded_tags(&self) -> &[::std::string::String] {
        &self.excluded_tags
    }

    fn get_excluded_tags_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.excluded_tags
    }

    fn mut_excluded_tags_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.excluded_tags
    }

    // optional uint32 desired_queue_size = 6;

    pub fn clear_desired_queue_size(&mut self) {
        self.desired_queue_size = ::std::option::Option::None;
    }

    pub fn has_desired_queue_size(&self) -> bool {
        self.desired_queue_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desired_queue_size(&mut self, v: u32) {
        self.desired_queue_size = ::std::option::Option::Some(v);
    }

    pub fn get_desired_queue_size(&self) -> u32 {
        self.desired_queue_size.unwrap_or(0)
    }

    fn get_desired_queue_size_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.desired_queue_size
    }

    fn mut_desired_queue_size_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.desired_queue_size
    }
}

impl ::protobuf::Message for CPublishedFile_RefreshVotingQueue_Request {
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
                    self.appid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.matching_file_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.tags)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.match_all_tags = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.excluded_tags)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.desired_queue_size = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.matching_file_type {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.tags {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if let Some(v) = self.match_all_tags {
            my_size += 2;
        }
        for value in &self.excluded_tags {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        if let Some(v) = self.desired_queue_size {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.appid {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.matching_file_type {
            os.write_uint32(2, v)?;
        }
        for v in &self.tags {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.match_all_tags {
            os.write_bool(4, v)?;
        }
        for v in &self.excluded_tags {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.desired_queue_size {
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

impl ::protobuf::MessageStatic for CPublishedFile_RefreshVotingQueue_Request {
    fn new() -> CPublishedFile_RefreshVotingQueue_Request {
        CPublishedFile_RefreshVotingQueue_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPublishedFile_RefreshVotingQueue_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CPublishedFile_RefreshVotingQueue_Request::get_appid_for_reflect,
                    CPublishedFile_RefreshVotingQueue_Request::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "matching_file_type",
                    CPublishedFile_RefreshVotingQueue_Request::get_matching_file_type_for_reflect,
                    CPublishedFile_RefreshVotingQueue_Request::mut_matching_file_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tags",
                    CPublishedFile_RefreshVotingQueue_Request::get_tags_for_reflect,
                    CPublishedFile_RefreshVotingQueue_Request::mut_tags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "match_all_tags",
                    CPublishedFile_RefreshVotingQueue_Request::get_match_all_tags_for_reflect,
                    CPublishedFile_RefreshVotingQueue_Request::mut_match_all_tags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "excluded_tags",
                    CPublishedFile_RefreshVotingQueue_Request::get_excluded_tags_for_reflect,
                    CPublishedFile_RefreshVotingQueue_Request::mut_excluded_tags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "desired_queue_size",
                    CPublishedFile_RefreshVotingQueue_Request::get_desired_queue_size_for_reflect,
                    CPublishedFile_RefreshVotingQueue_Request::mut_desired_queue_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CPublishedFile_RefreshVotingQueue_Request>(
                    "CPublishedFile_RefreshVotingQueue_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPublishedFile_RefreshVotingQueue_Request {
    fn clear(&mut self) {
        self.clear_appid();
        self.clear_matching_file_type();
        self.clear_tags();
        self.clear_match_all_tags();
        self.clear_excluded_tags();
        self.clear_desired_queue_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPublishedFile_RefreshVotingQueue_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPublishedFile_RefreshVotingQueue_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CPublishedFile_RefreshVotingQueue_Response {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CPublishedFile_RefreshVotingQueue_Response {}

impl CPublishedFile_RefreshVotingQueue_Response {
    pub fn new() -> CPublishedFile_RefreshVotingQueue_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CPublishedFile_RefreshVotingQueue_Response {
        static mut instance: ::protobuf::lazy::Lazy<CPublishedFile_RefreshVotingQueue_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CPublishedFile_RefreshVotingQueue_Response,
        };
        unsafe {
            instance.get(CPublishedFile_RefreshVotingQueue_Response::new)
        }
    }
}

impl ::protobuf::Message for CPublishedFile_RefreshVotingQueue_Response {
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

impl ::protobuf::MessageStatic for CPublishedFile_RefreshVotingQueue_Response {
    fn new() -> CPublishedFile_RefreshVotingQueue_Response {
        CPublishedFile_RefreshVotingQueue_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CPublishedFile_RefreshVotingQueue_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CPublishedFile_RefreshVotingQueue_Response>(
                    "CPublishedFile_RefreshVotingQueue_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CPublishedFile_RefreshVotingQueue_Response {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CPublishedFile_RefreshVotingQueue_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CPublishedFile_RefreshVotingQueue_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n/steammessages_publishedfile.steamworkssdk.proto\x1a.steammessages_uni\
    fied_base.steamworkssdk.proto\"\xa4\x01\n\x20CPublishedFile_Subscribe_Re\
    quest\x12(\n\x0fpublishedfileid\x18\x01\x20\x01(\x04R\x0fpublishedfileid\
    \x12\x1b\n\tlist_type\x18\x02\x20\x01(\rR\x08listType\x12\x14\n\x05appid\
    \x18\x03\x20\x01(\x05R\x05appid\x12#\n\rnotify_client\x18\x04\x20\x01(\
    \x08R\x0cnotifyClient\"#\n!CPublishedFile_Subscribe_Response\"\xa6\x01\n\
    \"CPublishedFile_Unsubscribe_Request\x12(\n\x0fpublishedfileid\x18\x01\
    \x20\x01(\x04R\x0fpublishedfileid\x12\x1b\n\tlist_type\x18\x02\x20\x01(\
    \rR\x08listType\x12\x14\n\x05appid\x18\x03\x20\x01(\x05R\x05appid\x12#\n\
    \rnotify_client\x18\x04\x20\x01(\x08R\x0cnotifyClient\"%\n#CPublishedFil\
    e_Unsubscribe_Response\"\x9e\x0c\n\x1eCPublishedFile_Publish_Request\x12\
    C\n\x05appid\x18\x01\x20\x01(\rR\x05appidB-\x82\xb5\x18)App\x20Id\x20thi\
    s\x20file\x20is\x20being\x20published\x20FROM.\x12R\n\x0econsumer_appid\
    \x18\x02\x20\x01(\rR\rconsumerAppidB+\x82\xb5\x18'App\x20Id\x20this\x20f\
    ile\x20is\x20being\x20published\x20TO.\x12Z\n\rcloudfilename\x18\x03\x20\
    \x01(\tR\rcloudfilenameB4\x82\xb5\x180Name\x20of\x20the\x20file\x20to\
    \x20publish\x20in\x20the\x20user's\x20cloud.\x12q\n\x15preview_cloudfile\
    name\x18\x04\x20\x01(\tR\x14previewCloudfilenameB<\x82\xb5\x188Name\x20o\
    f\x20the\x20file\x20to\x20use\x20as\x20the\x20published\x20file's\x20pre\
    view.\x12<\n\x05title\x18\x05\x20\x01(\tR\x05titleB&\x82\xb5\x18\"Text\
    \x20title\x20for\x20the\x20published\x20file.\x12W\n\x10file_description\
    \x18\x06\x20\x01(\tR\x0ffileDescriptionB,\x82\xb5\x18(Text\x20descriptio\
    n\x20for\x20the\x20published\x20file.\x12V\n\tfile_type\x18\x07\x20\x01(\
    \rR\x08fileTypeB9\x82\xb5\x185(EWorkshopFileType)\x20Type\x20of\x20Works\
    hop\x20file\x20to\x20publish.\x12_\n\x16consumer_shortcut_name\x18\x08\
    \x20\x01(\tR\x14consumerShortcutNameB)\x82\xb5\x18%Shortcut\x20name\x20f\
    or\x20the\x20published\x20file.\x12Z\n\x10youtube_username\x18\t\x20\x01\
    (\tR\x0fyoutubeUsernameB/\x82\xb5\x18+(Optional)\x20User's\x20YouTube\
    \x20account\x20username.\x12l\n\x0fyoutube_videoid\x18\n\x20\x01(\tR\x0e\
    youtubeVideoidBC\x82\xb5\x18?(Optional)\x20Video\x20Id\x20of\x20a\x20You\
    Tube\x20video\x20for\x20this\x20published\x20file.\x12\x8d\x01\n\nvisibi\
    lity\x18\x0b\x20\x01(\rR\nvisibilityBm\x82\xb5\x18i(ERemoteStoragePublis\
    hedFileVisibility)\x20Visibility\x20of\x20the\x20published\x20file\x20(p\
    rivate,\x20friends,\x20public,\x20etc.)\x12x\n\x0credirect_uri\x18\x0c\
    \x20\x01(\tR\x0bredirectUriBU\x82\xb5\x18Q(Optional)\x20If\x20supplied,\
    \x20the\x20resulting\x20published\x20file's\x20Id\x20is\x20appended\x20t\
    o\x20the\x20URI.\x12J\n\x04tags\x18\r\x20\x03(\tR\x04tagsB6\x82\xb5\x182\
    Array\x20of\x20text\x20tags\x20to\x20apply\x20to\x20the\x20published\x20\
    file.\x12i\n\x0fcollection_type\x18\x0e\x20\x01(\tR\x0ecollectionTypeB@\
    \x82\xb5\x18<(Optional)\x20Type\x20of\x20collection\x20the\x20published\
    \x20file\x20represents.\x12W\n\tgame_type\x18\x0f\x20\x01(\tR\x08gameTyp\
    eB:\x82\xb5\x186(Optional)\x20Type\x20of\x20game\x20the\x20published\x20\
    file\x20represents.\x12`\n\x03url\x18\x10\x20\x01(\tR\x03urlBN\x82\xb5\
    \x18J(Optional)\x20If\x20this\x20represents\x20a\x20game,\x20this\x20is\
    \x20the\x20URL\x20to\x20that\x20game's\x20page.\"n\n\x1fCPublishedFile_P\
    ublish_Response\x12(\n\x0fpublishedfileid\x18\x01\x20\x01(\x04R\x0fpubli\
    shedfileid\x12!\n\x0credirect_uri\x18\x02\x20\x01(\tR\x0bredirectUri\"\
    \xfe\x05\n!CPublishedFile_GetDetails_Request\x12b\n\x10publishedfileids\
    \x18\x01\x20\x03(\x06R\x10publishedfileidsB6\x82\xb5\x182Set\x20of\x20pu\
    blished\x20file\x20Ids\x20to\x20retrieve\x20details\x20for.\x12^\n\x0bin\
    cludetags\x18\x02\x20\x01(\x08R\x0bincludetagsB<\x82\xb5\x188If\x20true,\
    \x20return\x20tag\x20information\x20in\x20the\x20returned\x20details.\
    \x12~\n\x19includeadditionalpreviews\x18\x03\x20\x01(\x08R\x19includeadd\
    itionalpreviewsB@\x82\xb5\x18<If\x20true,\x20return\x20preview\x20inform\
    ation\x20in\x20the\x20returned\x20details.\x12_\n\x0fincludechildren\x18\
    \x04\x20\x01(\x08R\x0fincludechildrenB5\x82\xb5\x181If\x20true,\x20retur\
    n\x20children\x20in\x20the\x20returned\x20details.\x12a\n\rincludekvtags\
    \x18\x05\x20\x01(\x08R\rincludekvtagsB;\x82\xb5\x187If\x20true,\x20retur\
    n\x20key\x20value\x20tags\x20in\x20the\x20returned\x20details.\x12Z\n\
    \x0cincludevotes\x18\x06\x20\x01(\x08R\x0cincludevotesB6\x82\xb5\x182If\
    \x20true,\x20return\x20vote\x20data\x20in\x20the\x20returned\x20details.\
    \x12u\n\x11short_description\x18\x08\x20\x01(\x08R\x10shortDescriptionBH\
    \x82\xb5\x18DIf\x20true,\x20return\x20a\x20short\x20description\x20inste\
    ad\x20of\x20the\x20full\x20description.\"\xf5\x14\n\x14PublishedFileDeta\
    ils\x12\x16\n\x06result\x18\x01\x20\x01(\rR\x06result\x12(\n\x0fpublishe\
    dfileid\x18\x02\x20\x01(\x04R\x0fpublishedfileid\x12\x18\n\x07creator\
    \x18\x03\x20\x01(\x06R\x07creator\x12#\n\rcreator_appid\x18\x04\x20\x01(\
    \rR\x0ccreatorAppid\x12%\n\x0econsumer_appid\x18\x05\x20\x01(\rR\rconsum\
    erAppid\x12/\n\x13consumer_shortcutid\x18\x06\x20\x01(\rR\x12consumerSho\
    rtcutid\x12\x1a\n\x08filename\x18\x07\x20\x01(\tR\x08filename\x12\x1b\n\
    \tfile_size\x18\x08\x20\x01(\x04R\x08fileSize\x12*\n\x11preview_file_siz\
    e\x18\t\x20\x01(\x04R\x0fpreviewFileSize\x12\x19\n\x08file_url\x18\n\x20\
    \x01(\tR\x07fileUrl\x12\x1f\n\x0bpreview_url\x18\x0b\x20\x01(\tR\nprevie\
    wUrl\x12&\n\x0eyoutubevideoid\x18\x0c\x20\x01(\tR\x0eyoutubevideoid\x12\
    \x10\n\x03url\x18\r\x20\x01(\tR\x03url\x12#\n\rhcontent_file\x18\x0e\x20\
    \x01(\x06R\x0chcontentFile\x12)\n\x10hcontent_preview\x18\x0f\x20\x01(\
    \x06R\x0fhcontentPreview\x12\x14\n\x05title\x18\x10\x20\x01(\tR\x05title\
    \x12)\n\x10file_description\x18\x11\x20\x01(\tR\x0ffileDescription\x12+\
    \n\x11short_description\x18\x12\x20\x01(\tR\x10shortDescription\x12!\n\
    \x0ctime_created\x18\x13\x20\x01(\rR\x0btimeCreated\x12!\n\x0ctime_updat\
    ed\x18\x14\x20\x01(\rR\x0btimeUpdated\x12\x1e\n\nvisibility\x18\x15\x20\
    \x01(\rR\nvisibility\x12\x14\n\x05flags\x18\x16\x20\x01(\rR\x05flags\x12\
    #\n\rworkshop_file\x18\x17\x20\x01(\x08R\x0cworkshopFile\x12+\n\x11works\
    hop_accepted\x18\x18\x20\x01(\x08R\x10workshopAccepted\x12,\n\x12show_su\
    bscribe_all\x18\x19\x20\x01(\x08R\x10showSubscribeAll\x124\n\x16num_comm\
    ents_developer\x18\x1a\x20\x01(\x05R\x14numCommentsDeveloper\x12.\n\x13n\
    um_comments_public\x18\x1b\x20\x01(\x05R\x11numCommentsPublic\x12\x16\n\
    \x06banned\x18\x1c\x20\x01(\x08R\x06banned\x12\x1d\n\nban_reason\x18\x1d\
    \x20\x01(\tR\tbanReason\x12\x16\n\x06banner\x18\x1e\x20\x01(\x06R\x06ban\
    ner\x12$\n\x0ecan_be_deleted\x18\x1f\x20\x01(\x08R\x0ccanBeDeleted\x12\"\
    \n\x0cincompatible\x18\x20\x20\x01(\x08R\x0cincompatible\x12\x19\n\x08ap\
    p_name\x18!\x20\x01(\tR\x07appName\x12\x1b\n\tfile_type\x18\"\x20\x01(\r\
    R\x08fileType\x12#\n\rcan_subscribe\x18#\x20\x01(\x08R\x0ccanSubscribe\
    \x12$\n\rsubscriptions\x18$\x20\x01(\rR\rsubscriptions\x12\x1c\n\tfavori\
    ted\x18%\x20\x01(\rR\tfavorited\x12\x1c\n\tfollowers\x18&\x20\x01(\rR\tf\
    ollowers\x125\n\x16lifetime_subscriptions\x18'\x20\x01(\rR\x15lifetimeSu\
    bscriptions\x12-\n\x12lifetime_favorited\x18(\x20\x01(\rR\x11lifetimeFav\
    orited\x12-\n\x12lifetime_followers\x18)\x20\x01(\rR\x11lifetimeFollower\
    s\x12\x14\n\x05views\x18*\x20\x01(\rR\x05views\x12\x1f\n\x0bimage_width\
    \x18+\x20\x01(\rR\nimageWidth\x12!\n\x0cimage_height\x18,\x20\x01(\rR\
    \x0bimageHeight\x12\x1b\n\timage_url\x18-\x20\x01(\tR\x08imageUrl\x12\
    \x1f\n\x0bspoiler_tag\x18.\x20\x01(\x08R\nspoilerTag\x12\x1e\n\nshortcut\
    id\x18/\x20\x01(\rR\nshortcutid\x12\"\n\x0cshortcutname\x180\x20\x01(\tR\
    \x0cshortcutname\x12!\n\x0cnum_children\x181\x20\x01(\rR\x0bnumChildren\
    \x12\x1f\n\x0bnum_reports\x182\x20\x01(\rR\nnumReports\x129\n\x08preview\
    s\x183\x20\x03(\x0b2\x1d.PublishedFileDetails.PreviewR\x08previews\x12-\
    \n\x04tags\x184\x20\x03(\x0b2\x19.PublishedFileDetails.TagR\x04tags\x127\
    \n\x08children\x185\x20\x03(\x0b2\x1b.PublishedFileDetails.ChildR\x08chi\
    ldren\x123\n\x06kvtags\x186\x20\x03(\x0b2\x1b.PublishedFileDetails.KVTag\
    R\x06kvtags\x12;\n\tvote_data\x187\x20\x01(\x0b2\x1e.PublishedFileDetail\
    s.VoteDataR\x08voteData\x12\x82\x01\n\x0ftime_subscribed\x188\x20\x01(\r\
    R\x0etimeSubscribedBY\x82\xb5\x18UOnly\x20valid\x20in\x20PublishedFile.G\
    etUserFiles\x20and\x20not\x20normal\x20PublishedFile.GetDetail\x20calls\
    \x1a5\n\x03Tag\x12\x10\n\x03tag\x18\x01\x20\x01(\tR\x03tag\x12\x1c\n\tad\
    minonly\x18\x02\x20\x01(\x08R\tadminonly\x1a\xaf\x01\n\x07Preview\x12\
    \x1c\n\tpreviewid\x18\x01\x20\x01(\x04R\tpreviewid\x12\x1c\n\tsortorder\
    \x18\x02\x20\x01(\rR\tsortorder\x12\x10\n\x03url\x18\x03\x20\x01(\tR\x03\
    url\x12\x12\n\x04size\x18\x04\x20\x01(\rR\x04size\x12\x1a\n\x08filename\
    \x18\x05\x20\x01(\tR\x08filename\x12&\n\x0eyoutubevideoid\x18\x06\x20\
    \x01(\tR\x0eyoutubevideoid\x1al\n\x05Child\x12(\n\x0fpublishedfileid\x18\
    \x01\x20\x01(\x04R\x0fpublishedfileid\x12\x1c\n\tsortorder\x18\x02\x20\
    \x01(\rR\tsortorder\x12\x1b\n\tfile_type\x18\x03\x20\x01(\rR\x08fileType\
    \x1a/\n\x05KVTag\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\
    \x05value\x18\x02\x20\x01(\tR\x05value\x1aZ\n\x08VoteData\x12\x14\n\x05s\
    core\x18\x01\x20\x01(\x02R\x05score\x12\x19\n\x08votes_up\x18\x02\x20\
    \x01(\rR\x07votesUp\x12\x1d\n\nvotes_down\x18\x03\x20\x01(\rR\tvotesDown\
    \"o\n\"CPublishedFile_GetDetails_Response\x12I\n\x14publishedfiledetails\
    \x18\x01\x20\x03(\x0b2\x15.PublishedFileDetailsR\x14publishedfiledetails\
    \"\x99\x07\n#CPublishedFile_GetUserFiles_Request\x12B\n\x05appid\x18\x01\
    \x20\x01(\rR\x05appidB,\x82\xb5\x18(App\x20Id\x20to\x20retrieve\x20publi\
    shed\x20files\x20from.\x12@\n\x04page\x18\x03\x20\x01(\r:\x011R\x04pageB\
    )\x82\xb5\x18%(Optional)\x20Starting\x20page\x20for\x20results.\x12\\\n\
    \nnumperpage\x18\x04\x20\x01(\r:\x011R\nnumperpageB9\x82\xb5\x185(Option\
    al)\x20The\x20number\x20of\x20results,\x20per\x20page\x20to\x20return.\
    \x12e\n\nsortmethod\x18\x06\x20\x01(\t:\x0blastupdatedR\nsortmethodB8\
    \x82\xb5\x184(Optional)\x20Sorting\x20method\x20to\x20use\x20on\x20retur\
    ned\x20values.\x12t\n\ttotalonly\x18\x07\x20\x01(\x08R\ttotalonlyBV\x82\
    \xb5\x18R(Optional)\x20If\x20true,\x20only\x20return\x20the\x20total\x20\
    number\x20of\x20files\x20that\x20satisfy\x20this\x20query.\x12D\n\x07pri\
    vacy\x18\t\x20\x01(\rR\x07privacyB*\x82\xb5\x18&(optional)\x20Filter\x20\
    by\x20privacy\x20settings.\x12w\n\x08ids_only\x18\n\x20\x01(\x08R\x07ids\
    OnlyB\\\x82\xb5\x18X(Optional)\x20If\x20true,\x20only\x20return\x20the\
    \x20published\x20file\x20ids\x20of\x20files\x20that\x20satisfy\x20this\
    \x20query.\x12v\n\x0crequiredtags\x18\x0b\x20\x03(\tR\x0crequiredtagsBR\
    \x82\xb5\x18N(Optional)\x20Tags\x20that\x20must\x20be\x20present\x20on\
    \x20a\x20published\x20file\x20to\x20satisfy\x20the\x20query.\x12z\n\x0ce\
    xcludedtags\x18\x0c\x20\x03(\tR\x0cexcludedtagsBV\x82\xb5\x18R(Optional)\
    \x20Tags\x20that\x20must\x20NOT\x20be\x20present\x20on\x20a\x20published\
    \x20file\x20to\x20satisfy\x20the\x20query.\"\xd1\x02\n$CPublishedFile_Ge\
    tUserFiles_Response\x12\x14\n\x05total\x18\x01\x20\x01(\rR\x05total\x12\
    \x1e\n\nstartindex\x18\x02\x20\x01(\rR\nstartindex\x12I\n\x14publishedfi\
    ledetails\x18\x03\x20\x03(\x0b2\x15.PublishedFileDetailsR\x14publishedfi\
    ledetails\x12=\n\x04apps\x18\x04\x20\x03(\x0b2).CPublishedFile_GetUserFi\
    les_Response.AppR\x04apps\x1ai\n\x03App\x12\x14\n\x05appid\x18\x01\x20\
    \x01(\rR\x05appid\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\x1e\
    \n\nshortcutid\x18\x03\x20\x01(\rR\nshortcutid\x12\x18\n\x07private\x18\
    \x04\x20\x01(\x08R\x07private\"\xac\x05\n\x1dCPublishedFile_Update_Reque\
    st\x12@\n\x05appid\x18\x01\x20\x01(\rR\x05appidB*\x82\xb5\x18&App\x20Id\
    \x20this\x20published\x20file\x20belongs\x20to.\x12]\n\x0fpublishedfilei\
    d\x18\x02\x20\x01(\x06R\x0fpublishedfileidB3\x82\xb5\x18/Published\x20fi\
    le\x20id\x20of\x20the\x20file\x20we'd\x20like\x20update.\x12A\n\x05title\
    \x18\x03\x20\x01(\tR\x05titleB+\x82\xb5\x18'(Optional)\x20Title\x20of\
    \x20the\x20published\x20file.\x12\\\n\x10file_description\x18\x04\x20\
    \x01(\tR\x0ffileDescriptionB1\x82\xb5\x18-(Optional)\x20Description\x20o\
    f\x20the\x20published\x20file.\x12P\n\nvisibility\x18\x05\x20\x01(\rR\nv\
    isibilityB0\x82\xb5\x18,(Optional)\x20Visibility\x20of\x20the\x20publish\
    ed\x20file.\x12F\n\x04tags\x18\x06\x20\x03(\tR\x04tagsB2\x82\xb5\x18.(Op\
    tional)\x20Set\x20of\x20tags\x20for\x20the\x20published\x20file.\x12K\n\
    \x08filename\x18\x07\x20\x01(\tR\x08filenameB/\x82\xb5\x18+(Optional)\
    \x20Filename\x20for\x20the\x20published\x20file.\x12b\n\x10preview_filen\
    ame\x18\x08\x20\x01(\tR\x0fpreviewFilenameB7\x82\xb5\x183(Optional)\x20P\
    review\x20filename\x20for\x20the\x20published\x20file.\"\x20\n\x1eCPubli\
    shedFile_Update_Response\"\x88\x05\n)CPublishedFile_RefreshVotingQueue_R\
    equest\x12\x14\n\x05appid\x18\x01\x20\x01(\rR\x05appid\x12T\n\x12matchin\
    g_file_type\x18\x02\x20\x01(\rR\x10matchingFileTypeB&\x82\xb5\x18\"EPubl\
    ishedFileInfoMatchingFileType\x12r\n\x04tags\x18\x03\x20\x03(\tR\x04tags\
    B^\x82\xb5\x18ZInclude\x20files\x20that\x20have\x20all\x20the\x20tags\
    \x20or\x20any\x20of\x20the\x20tags\x20if\x20match_all_tags\x20is\x20set\
    \x20to\x20false.\x12\xa3\x01\n\x0ematch_all_tags\x18\x04\x20\x01(\x08:\
    \x04trueR\x0cmatchAllTagsBw\x82\xb5\x18sIf\x20true,\x20then\x20files\x20\
    must\x20have\x20all\x20the\x20tags\x20specified.\x20\x20If\x20false,\x20\
    then\x20must\x20have\x20at\x20least\x20one\x20of\x20the\x20tags\x20speci\
    fied.\x12W\n\rexcluded_tags\x18\x05\x20\x03(\tR\x0cexcludedTagsB2\x82\
    \xb5\x18.Exclude\x20any\x20files\x20that\x20have\x20any\x20of\x20these\
    \x20tags.\x12|\n\x12desired_queue_size\x18\x06\x20\x01(\rR\x10desiredQue\
    ueSizeBN\x82\xb5\x18JDesired\x20number\x20of\x20items\x20in\x20the\x20vo\
    ting\x20queue.\x20\x20May\x20be\x20clamped\x20by\x20the\x20server\",\n*C\
    PublishedFile_RefreshVotingQueue_Response2\x83\x08\n\rPublishedFile\x12\
    \x81\x01\n\tSubscribe\x12!.CPublishedFile_Subscribe_Request\x1a\".CPubli\
    shedFile_Subscribe_Response\"-\x82\xb5\x18)Subscribes\x20the\x20user\x20\
    to\x20the\x20published\x20file\x12\x8b\x01\n\x0bUnsubscribe\x12#.CPublis\
    hedFile_Unsubscribe_Request\x1a$.CPublishedFile_Unsubscribe_Response\"1\
    \x82\xb5\x18-Unsubscribes\x20the\x20user\x20from\x20the\x20published\x20\
    file\x12\x80\x01\n\x07Publish\x12\x1f.CPublishedFile_Publish_Request\x1a\
    \x20.CPublishedFile_Publish_Response\"2\x82\xb5\x18.Publishes\x20a\x20cl\
    ouded\x20user\x20file\x20to\x20the\x20Workshop.\x12\x90\x01\n\nGetDetail\
    s\x12\".CPublishedFile_GetDetails_Request\x1a#.CPublishedFile_GetDetails\
    _Response\"9\x82\xb5\x185Retrieves\x20information\x20about\x20a\x20set\
    \x20of\x20published\x20files.\x12\x85\x01\n\x0cGetUserFiles\x12$.CPublis\
    hedFile_GetUserFiles_Request\x1a%.CPublishedFile_GetUserFiles_Response\"\
    (\x82\xb5\x18$Retrieves\x20files\x20published\x20by\x20a\x20user.\x12z\n\
    \x06Update\x12\x1e.CPublishedFile_Update_Request\x1a\x1f.CPublishedFile_\
    Update_Response\"/\x82\xb5\x18+Updates\x20information\x20about\x20a\x20p\
    ublished\x20file.\x12\x98\x01\n\x12RefreshVotingQueue\x12*.CPublishedFil\
    e_RefreshVotingQueue_Request\x1a+.CPublishedFile_RefreshVotingQueue_Resp\
    onse\")\x82\xb5\x18%Refresh\x20the\x20voting\x20queue\x20for\x20the\x20u\
    ser\x1a+\x82\xb5\x18'A\x20service\x20to\x20access\x20published\x20file\
    \x20data\
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
