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
pub struct CCloud_GetUploadServerInfo_Request {
    // message fields
    appid: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCloud_GetUploadServerInfo_Request {}

impl CCloud_GetUploadServerInfo_Request {
    pub fn new() -> CCloud_GetUploadServerInfo_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCloud_GetUploadServerInfo_Request {
        static mut instance: ::protobuf::lazy::Lazy<CCloud_GetUploadServerInfo_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCloud_GetUploadServerInfo_Request,
        };
        unsafe {
            instance.get(CCloud_GetUploadServerInfo_Request::new)
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
}

impl ::protobuf::Message for CCloud_GetUploadServerInfo_Request {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.appid {
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

impl ::protobuf::MessageStatic for CCloud_GetUploadServerInfo_Request {
    fn new() -> CCloud_GetUploadServerInfo_Request {
        CCloud_GetUploadServerInfo_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCloud_GetUploadServerInfo_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CCloud_GetUploadServerInfo_Request::get_appid_for_reflect,
                    CCloud_GetUploadServerInfo_Request::mut_appid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCloud_GetUploadServerInfo_Request>(
                    "CCloud_GetUploadServerInfo_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCloud_GetUploadServerInfo_Request {
    fn clear(&mut self) {
        self.clear_appid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCloud_GetUploadServerInfo_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCloud_GetUploadServerInfo_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCloud_GetUploadServerInfo_Response {
    // message fields
    server_url: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCloud_GetUploadServerInfo_Response {}

impl CCloud_GetUploadServerInfo_Response {
    pub fn new() -> CCloud_GetUploadServerInfo_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCloud_GetUploadServerInfo_Response {
        static mut instance: ::protobuf::lazy::Lazy<CCloud_GetUploadServerInfo_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCloud_GetUploadServerInfo_Response,
        };
        unsafe {
            instance.get(CCloud_GetUploadServerInfo_Response::new)
        }
    }

    // optional string server_url = 1;

    pub fn clear_server_url(&mut self) {
        self.server_url.clear();
    }

    pub fn has_server_url(&self) -> bool {
        self.server_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_url(&mut self, v: ::std::string::String) {
        self.server_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_url(&mut self) -> &mut ::std::string::String {
        if self.server_url.is_none() {
            self.server_url.set_default();
        }
        self.server_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_server_url(&mut self) -> ::std::string::String {
        self.server_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_server_url(&self) -> &str {
        match self.server_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_server_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.server_url
    }

    fn mut_server_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.server_url
    }
}

impl ::protobuf::Message for CCloud_GetUploadServerInfo_Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.server_url)?;
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
        if let Some(ref v) = self.server_url.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.server_url.as_ref() {
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

impl ::protobuf::MessageStatic for CCloud_GetUploadServerInfo_Response {
    fn new() -> CCloud_GetUploadServerInfo_Response {
        CCloud_GetUploadServerInfo_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCloud_GetUploadServerInfo_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "server_url",
                    CCloud_GetUploadServerInfo_Response::get_server_url_for_reflect,
                    CCloud_GetUploadServerInfo_Response::mut_server_url_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCloud_GetUploadServerInfo_Response>(
                    "CCloud_GetUploadServerInfo_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCloud_GetUploadServerInfo_Response {
    fn clear(&mut self) {
        self.clear_server_url();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCloud_GetUploadServerInfo_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCloud_GetUploadServerInfo_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCloud_GetFileDetails_Request {
    // message fields
    ugcid: ::std::option::Option<u64>,
    appid: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCloud_GetFileDetails_Request {}

impl CCloud_GetFileDetails_Request {
    pub fn new() -> CCloud_GetFileDetails_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCloud_GetFileDetails_Request {
        static mut instance: ::protobuf::lazy::Lazy<CCloud_GetFileDetails_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCloud_GetFileDetails_Request,
        };
        unsafe {
            instance.get(CCloud_GetFileDetails_Request::new)
        }
    }

    // optional uint64 ugcid = 1;

    pub fn clear_ugcid(&mut self) {
        self.ugcid = ::std::option::Option::None;
    }

    pub fn has_ugcid(&self) -> bool {
        self.ugcid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ugcid(&mut self, v: u64) {
        self.ugcid = ::std::option::Option::Some(v);
    }

    pub fn get_ugcid(&self) -> u64 {
        self.ugcid.unwrap_or(0)
    }

    fn get_ugcid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ugcid
    }

    fn mut_ugcid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ugcid
    }

    // optional uint32 appid = 2;

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
}

impl ::protobuf::Message for CCloud_GetFileDetails_Request {
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
                    self.ugcid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.appid = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.ugcid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ugcid {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.appid {
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

impl ::protobuf::MessageStatic for CCloud_GetFileDetails_Request {
    fn new() -> CCloud_GetFileDetails_Request {
        CCloud_GetFileDetails_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCloud_GetFileDetails_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ugcid",
                    CCloud_GetFileDetails_Request::get_ugcid_for_reflect,
                    CCloud_GetFileDetails_Request::mut_ugcid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CCloud_GetFileDetails_Request::get_appid_for_reflect,
                    CCloud_GetFileDetails_Request::mut_appid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCloud_GetFileDetails_Request>(
                    "CCloud_GetFileDetails_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCloud_GetFileDetails_Request {
    fn clear(&mut self) {
        self.clear_ugcid();
        self.clear_appid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCloud_GetFileDetails_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCloud_GetFileDetails_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCloud_UserFile {
    // message fields
    appid: ::std::option::Option<u32>,
    ugcid: ::std::option::Option<u64>,
    filename: ::protobuf::SingularField<::std::string::String>,
    timestamp: ::std::option::Option<u64>,
    file_size: ::std::option::Option<u32>,
    url: ::protobuf::SingularField<::std::string::String>,
    steamid_creator: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCloud_UserFile {}

impl CCloud_UserFile {
    pub fn new() -> CCloud_UserFile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCloud_UserFile {
        static mut instance: ::protobuf::lazy::Lazy<CCloud_UserFile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCloud_UserFile,
        };
        unsafe {
            instance.get(CCloud_UserFile::new)
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

    // optional uint64 ugcid = 2;

    pub fn clear_ugcid(&mut self) {
        self.ugcid = ::std::option::Option::None;
    }

    pub fn has_ugcid(&self) -> bool {
        self.ugcid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ugcid(&mut self, v: u64) {
        self.ugcid = ::std::option::Option::Some(v);
    }

    pub fn get_ugcid(&self) -> u64 {
        self.ugcid.unwrap_or(0)
    }

    fn get_ugcid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ugcid
    }

    fn mut_ugcid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ugcid
    }

    // optional string filename = 3;

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

    // optional uint64 timestamp = 4;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.timestamp
    }

    // optional uint32 file_size = 5;

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

    // optional string url = 6;

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

    // optional fixed64 steamid_creator = 7;

    pub fn clear_steamid_creator(&mut self) {
        self.steamid_creator = ::std::option::Option::None;
    }

    pub fn has_steamid_creator(&self) -> bool {
        self.steamid_creator.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid_creator(&mut self, v: u64) {
        self.steamid_creator = ::std::option::Option::Some(v);
    }

    pub fn get_steamid_creator(&self) -> u64 {
        self.steamid_creator.unwrap_or(0)
    }

    fn get_steamid_creator_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid_creator
    }

    fn mut_steamid_creator_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid_creator
    }
}

impl ::protobuf::Message for CCloud_UserFile {
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
                    let tmp = is.read_uint64()?;
                    self.ugcid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.filename)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.file_size = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steamid_creator = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.ugcid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.filename.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.file_size {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.url.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(v) = self.steamid_creator {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.appid {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.ugcid {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.filename.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.timestamp {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.file_size {
            os.write_uint32(5, v)?;
        }
        if let Some(ref v) = self.url.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(v) = self.steamid_creator {
            os.write_fixed64(7, v)?;
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

impl ::protobuf::MessageStatic for CCloud_UserFile {
    fn new() -> CCloud_UserFile {
        CCloud_UserFile::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCloud_UserFile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CCloud_UserFile::get_appid_for_reflect,
                    CCloud_UserFile::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ugcid",
                    CCloud_UserFile::get_ugcid_for_reflect,
                    CCloud_UserFile::mut_ugcid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filename",
                    CCloud_UserFile::get_filename_for_reflect,
                    CCloud_UserFile::mut_filename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    CCloud_UserFile::get_timestamp_for_reflect,
                    CCloud_UserFile::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "file_size",
                    CCloud_UserFile::get_file_size_for_reflect,
                    CCloud_UserFile::mut_file_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url",
                    CCloud_UserFile::get_url_for_reflect,
                    CCloud_UserFile::mut_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid_creator",
                    CCloud_UserFile::get_steamid_creator_for_reflect,
                    CCloud_UserFile::mut_steamid_creator_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCloud_UserFile>(
                    "CCloud_UserFile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCloud_UserFile {
    fn clear(&mut self) {
        self.clear_appid();
        self.clear_ugcid();
        self.clear_filename();
        self.clear_timestamp();
        self.clear_file_size();
        self.clear_url();
        self.clear_steamid_creator();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCloud_UserFile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCloud_UserFile {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCloud_GetFileDetails_Response {
    // message fields
    details: ::protobuf::SingularPtrField<CCloud_UserFile>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCloud_GetFileDetails_Response {}

impl CCloud_GetFileDetails_Response {
    pub fn new() -> CCloud_GetFileDetails_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCloud_GetFileDetails_Response {
        static mut instance: ::protobuf::lazy::Lazy<CCloud_GetFileDetails_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCloud_GetFileDetails_Response,
        };
        unsafe {
            instance.get(CCloud_GetFileDetails_Response::new)
        }
    }

    // optional .CCloud_UserFile details = 1;

    pub fn clear_details(&mut self) {
        self.details.clear();
    }

    pub fn has_details(&self) -> bool {
        self.details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_details(&mut self, v: CCloud_UserFile) {
        self.details = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_details(&mut self) -> &mut CCloud_UserFile {
        if self.details.is_none() {
            self.details.set_default();
        }
        self.details.as_mut().unwrap()
    }

    // Take field
    pub fn take_details(&mut self) -> CCloud_UserFile {
        self.details.take().unwrap_or_else(|| CCloud_UserFile::new())
    }

    pub fn get_details(&self) -> &CCloud_UserFile {
        self.details.as_ref().unwrap_or_else(|| CCloud_UserFile::default_instance())
    }

    fn get_details_for_reflect(&self) -> &::protobuf::SingularPtrField<CCloud_UserFile> {
        &self.details
    }

    fn mut_details_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CCloud_UserFile> {
        &mut self.details
    }
}

impl ::protobuf::Message for CCloud_GetFileDetails_Response {
    fn is_initialized(&self) -> bool {
        for v in &self.details {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.details)?;
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
        if let Some(ref v) = self.details.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.details.as_ref() {
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

impl ::protobuf::MessageStatic for CCloud_GetFileDetails_Response {
    fn new() -> CCloud_GetFileDetails_Response {
        CCloud_GetFileDetails_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCloud_GetFileDetails_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CCloud_UserFile>>(
                    "details",
                    CCloud_GetFileDetails_Response::get_details_for_reflect,
                    CCloud_GetFileDetails_Response::mut_details_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCloud_GetFileDetails_Response>(
                    "CCloud_GetFileDetails_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCloud_GetFileDetails_Response {
    fn clear(&mut self) {
        self.clear_details();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCloud_GetFileDetails_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCloud_GetFileDetails_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCloud_EnumerateUserFiles_Request {
    // message fields
    appid: ::std::option::Option<u32>,
    extended_details: ::std::option::Option<bool>,
    count: ::std::option::Option<u32>,
    start_index: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCloud_EnumerateUserFiles_Request {}

impl CCloud_EnumerateUserFiles_Request {
    pub fn new() -> CCloud_EnumerateUserFiles_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCloud_EnumerateUserFiles_Request {
        static mut instance: ::protobuf::lazy::Lazy<CCloud_EnumerateUserFiles_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCloud_EnumerateUserFiles_Request,
        };
        unsafe {
            instance.get(CCloud_EnumerateUserFiles_Request::new)
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

    // optional bool extended_details = 2;

    pub fn clear_extended_details(&mut self) {
        self.extended_details = ::std::option::Option::None;
    }

    pub fn has_extended_details(&self) -> bool {
        self.extended_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_extended_details(&mut self, v: bool) {
        self.extended_details = ::std::option::Option::Some(v);
    }

    pub fn get_extended_details(&self) -> bool {
        self.extended_details.unwrap_or(false)
    }

    fn get_extended_details_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.extended_details
    }

    fn mut_extended_details_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.extended_details
    }

    // optional uint32 count = 3;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u32) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count(&self) -> u32 {
        self.count.unwrap_or(0)
    }

    fn get_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.count
    }

    // optional uint32 start_index = 4;

    pub fn clear_start_index(&mut self) {
        self.start_index = ::std::option::Option::None;
    }

    pub fn has_start_index(&self) -> bool {
        self.start_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_index(&mut self, v: u32) {
        self.start_index = ::std::option::Option::Some(v);
    }

    pub fn get_start_index(&self) -> u32 {
        self.start_index.unwrap_or(0)
    }

    fn get_start_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.start_index
    }

    fn mut_start_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.start_index
    }
}

impl ::protobuf::Message for CCloud_EnumerateUserFiles_Request {
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
                    let tmp = is.read_bool()?;
                    self.extended_details = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.count = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.start_index = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.extended_details {
            my_size += 2;
        }
        if let Some(v) = self.count {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.start_index {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.appid {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.extended_details {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.count {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.start_index {
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

impl ::protobuf::MessageStatic for CCloud_EnumerateUserFiles_Request {
    fn new() -> CCloud_EnumerateUserFiles_Request {
        CCloud_EnumerateUserFiles_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCloud_EnumerateUserFiles_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CCloud_EnumerateUserFiles_Request::get_appid_for_reflect,
                    CCloud_EnumerateUserFiles_Request::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "extended_details",
                    CCloud_EnumerateUserFiles_Request::get_extended_details_for_reflect,
                    CCloud_EnumerateUserFiles_Request::mut_extended_details_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "count",
                    CCloud_EnumerateUserFiles_Request::get_count_for_reflect,
                    CCloud_EnumerateUserFiles_Request::mut_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "start_index",
                    CCloud_EnumerateUserFiles_Request::get_start_index_for_reflect,
                    CCloud_EnumerateUserFiles_Request::mut_start_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCloud_EnumerateUserFiles_Request>(
                    "CCloud_EnumerateUserFiles_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCloud_EnumerateUserFiles_Request {
    fn clear(&mut self) {
        self.clear_appid();
        self.clear_extended_details();
        self.clear_count();
        self.clear_start_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCloud_EnumerateUserFiles_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCloud_EnumerateUserFiles_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCloud_EnumerateUserFiles_Response {
    // message fields
    files: ::protobuf::RepeatedField<CCloud_UserFile>,
    total_files: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCloud_EnumerateUserFiles_Response {}

impl CCloud_EnumerateUserFiles_Response {
    pub fn new() -> CCloud_EnumerateUserFiles_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCloud_EnumerateUserFiles_Response {
        static mut instance: ::protobuf::lazy::Lazy<CCloud_EnumerateUserFiles_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCloud_EnumerateUserFiles_Response,
        };
        unsafe {
            instance.get(CCloud_EnumerateUserFiles_Response::new)
        }
    }

    // repeated .CCloud_UserFile files = 1;

    pub fn clear_files(&mut self) {
        self.files.clear();
    }

    // Param is passed by value, moved
    pub fn set_files(&mut self, v: ::protobuf::RepeatedField<CCloud_UserFile>) {
        self.files = v;
    }

    // Mutable pointer to the field.
    pub fn mut_files(&mut self) -> &mut ::protobuf::RepeatedField<CCloud_UserFile> {
        &mut self.files
    }

    // Take field
    pub fn take_files(&mut self) -> ::protobuf::RepeatedField<CCloud_UserFile> {
        ::std::mem::replace(&mut self.files, ::protobuf::RepeatedField::new())
    }

    pub fn get_files(&self) -> &[CCloud_UserFile] {
        &self.files
    }

    fn get_files_for_reflect(&self) -> &::protobuf::RepeatedField<CCloud_UserFile> {
        &self.files
    }

    fn mut_files_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CCloud_UserFile> {
        &mut self.files
    }

    // optional uint32 total_files = 2;

    pub fn clear_total_files(&mut self) {
        self.total_files = ::std::option::Option::None;
    }

    pub fn has_total_files(&self) -> bool {
        self.total_files.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_files(&mut self, v: u32) {
        self.total_files = ::std::option::Option::Some(v);
    }

    pub fn get_total_files(&self) -> u32 {
        self.total_files.unwrap_or(0)
    }

    fn get_total_files_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.total_files
    }

    fn mut_total_files_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.total_files
    }
}

impl ::protobuf::Message for CCloud_EnumerateUserFiles_Response {
    fn is_initialized(&self) -> bool {
        for v in &self.files {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.files)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.total_files = ::std::option::Option::Some(tmp);
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
        for value in &self.files {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.total_files {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.files {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.total_files {
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

impl ::protobuf::MessageStatic for CCloud_EnumerateUserFiles_Response {
    fn new() -> CCloud_EnumerateUserFiles_Response {
        CCloud_EnumerateUserFiles_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCloud_EnumerateUserFiles_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CCloud_UserFile>>(
                    "files",
                    CCloud_EnumerateUserFiles_Response::get_files_for_reflect,
                    CCloud_EnumerateUserFiles_Response::mut_files_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "total_files",
                    CCloud_EnumerateUserFiles_Response::get_total_files_for_reflect,
                    CCloud_EnumerateUserFiles_Response::mut_total_files_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCloud_EnumerateUserFiles_Response>(
                    "CCloud_EnumerateUserFiles_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCloud_EnumerateUserFiles_Response {
    fn clear(&mut self) {
        self.clear_files();
        self.clear_total_files();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCloud_EnumerateUserFiles_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCloud_EnumerateUserFiles_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCloud_Delete_Request {
    // message fields
    filename: ::protobuf::SingularField<::std::string::String>,
    appid: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCloud_Delete_Request {}

impl CCloud_Delete_Request {
    pub fn new() -> CCloud_Delete_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCloud_Delete_Request {
        static mut instance: ::protobuf::lazy::Lazy<CCloud_Delete_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCloud_Delete_Request,
        };
        unsafe {
            instance.get(CCloud_Delete_Request::new)
        }
    }

    // optional string filename = 1;

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

    // optional uint32 appid = 2;

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
}

impl ::protobuf::Message for CCloud_Delete_Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.filename)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.appid = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.filename.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.filename.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.appid {
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

impl ::protobuf::MessageStatic for CCloud_Delete_Request {
    fn new() -> CCloud_Delete_Request {
        CCloud_Delete_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCloud_Delete_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filename",
                    CCloud_Delete_Request::get_filename_for_reflect,
                    CCloud_Delete_Request::mut_filename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CCloud_Delete_Request::get_appid_for_reflect,
                    CCloud_Delete_Request::mut_appid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCloud_Delete_Request>(
                    "CCloud_Delete_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCloud_Delete_Request {
    fn clear(&mut self) {
        self.clear_filename();
        self.clear_appid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCloud_Delete_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCloud_Delete_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCloud_Delete_Response {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCloud_Delete_Response {}

impl CCloud_Delete_Response {
    pub fn new() -> CCloud_Delete_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCloud_Delete_Response {
        static mut instance: ::protobuf::lazy::Lazy<CCloud_Delete_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCloud_Delete_Response,
        };
        unsafe {
            instance.get(CCloud_Delete_Response::new)
        }
    }
}

impl ::protobuf::Message for CCloud_Delete_Response {
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

impl ::protobuf::MessageStatic for CCloud_Delete_Response {
    fn new() -> CCloud_Delete_Response {
        CCloud_Delete_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCloud_Delete_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CCloud_Delete_Response>(
                    "CCloud_Delete_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCloud_Delete_Response {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCloud_Delete_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCloud_Delete_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'steammessages_cloud.steamworkssdk.proto\x1a.steammessages_unified_bas\
    e.steamworkssdk.proto\"k\n\"CCloud_GetUploadServerInfo_Request\x12E\n\
    \x05appid\x18\x01\x20\x01(\rR\x05appidB/\x82\xb5\x18+App\x20ID\x20to\x20\
    which\x20a\x20file\x20will\x20be\x20uploaded\x20to.\"D\n#CCloud_GetUploa\
    dServerInfo_Response\x12\x1d\n\nserver_url\x18\x01\x20\x01(\tR\tserverUr\
    l\"\x9a\x01\n\x1dCCloud_GetFileDetails_Request\x12B\n\x05ugcid\x18\x01\
    \x20\x01(\x04R\x05ugcidB,\x82\xb5\x18(ID\x20of\x20the\x20Cloud\x20file\
    \x20to\x20get\x20details\x20for.\x125\n\x05appid\x18\x02\x20\x01(\rR\x05\
    appidB\x1f\x82\xb5\x18\x1bApp\x20ID\x20the\x20file\x20belongs\x20to.\"\
    \xcf\x01\n\x0fCCloud_UserFile\x12\x14\n\x05appid\x18\x01\x20\x01(\rR\x05\
    appid\x12\x14\n\x05ugcid\x18\x02\x20\x01(\x04R\x05ugcid\x12\x1a\n\x08fil\
    ename\x18\x03\x20\x01(\tR\x08filename\x12\x1c\n\ttimestamp\x18\x04\x20\
    \x01(\x04R\ttimestamp\x12\x1b\n\tfile_size\x18\x05\x20\x01(\rR\x08fileSi\
    ze\x12\x10\n\x03url\x18\x06\x20\x01(\tR\x03url\x12'\n\x0fsteamid_creator\
    \x18\x07\x20\x01(\x06R\x0esteamidCreator\"L\n\x1eCCloud_GetFileDetails_R\
    esponse\x12*\n\x07details\x18\x01\x20\x01(\x0b2\x10.CCloud_UserFileR\x07\
    details\"\x95\x04\n!CCloud_EnumerateUserFiles_Request\x12;\n\x05appid\
    \x18\x01\x20\x01(\rR\x05appidB%\x82\xb5\x18!App\x20ID\x20to\x20enumerate\
    \x20the\x20files\x20of.\x12\xac\x01\n\x10extended_details\x18\x02\x20\
    \x01(\x08R\x0fextendedDetailsB\x80\x01\x82\xb5\x18|(Optional)\x20Get\x20\
    extended\x20details\x20back\x20on\x20the\x20files\x20found.\x20Defaults\
    \x20to\x20only\x20returned\x20the\x20app\x20Id\x20and\x20UGC\x20Id\x20of\
    \x20the\x20files\x20found.\x12\x83\x01\n\x05count\x18\x03\x20\x01(\rR\
    \x05countBm\x82\xb5\x18i(Optional)\x20Maximum\x20number\x20of\x20results\
    \x20to\x20return\x20on\x20this\x20call.\x20Defaults\x20to\x20a\x20maximu\
    m\x20of\x20500\x20files\x20returned.\x12~\n\x0bstart_index\x18\x04\x20\
    \x01(\rR\nstartIndexB]\x82\xb5\x18Y(Optional)\x20Starting\x20index\x20to\
    \x20begin\x20enumeration\x20at.\x20Defaults\x20to\x20the\x20beginning\
    \x20of\x20the\x20list.\"m\n\"CCloud_EnumerateUserFiles_Response\x12&\n\
    \x05files\x18\x01\x20\x03(\x0b2\x10.CCloud_UserFileR\x05files\x12\x1f\n\
    \x0btotal_files\x18\x02\x20\x01(\rR\ntotalFiles\"j\n\x15CCloud_Delete_Re\
    quest\x12\x1a\n\x08filename\x18\x01\x20\x01(\tR\x08filename\x125\n\x05ap\
    pid\x18\x02\x20\x01(\rR\x05appidB\x1f\x82\xb5\x18\x1bApp\x20ID\x20the\
    \x20file\x20belongs\x20to.\"\x18\n\x16CCloud_Delete_Response2\xed\x04\n\
    \x05Cloud\x12\x9c\x01\n\x13GetUploadServerInfo\x12#.CCloud_GetUploadServ\
    erInfo_Request\x1a$.CCloud_GetUploadServerInfo_Response\":\x82\xb5\x186R\
    eturns\x20the\x20URL\x20of\x20the\x20proper\x20cloud\x20server\x20for\
    \x20a\x20user.\x12w\n\x0eGetFileDetails\x12\x1e.CCloud_GetFileDetails_Re\
    quest\x1a\x1f.CCloud_GetFileDetails_Response\"$\x82\xb5\x18\x20Returns\
    \x20details\x20on\x20a\x20Cloud\x20file.\x12\xba\x01\n\x12EnumerateUserF\
    iles\x12\".CCloud_EnumerateUserFiles_Request\x1a#.CCloud_EnumerateUserFi\
    les_Response\"[\x82\xb5\x18WEnumerates\x20Cloud\x20files\x20for\x20a\x20\
    user\x20of\x20a\x20given\x20app\x20ID.\x20Returns\x20up\x20to\x20500\x20\
    files\x20at\x20a\x20time.\x12d\n\x06Delete\x12\x16.CCloud_Delete_Request\
    \x1a\x17.CCloud_Delete_Response\")\x82\xb5\x18%Deletes\x20a\x20file\x20f\
    rom\x20the\x20user's\x20cloud.\x1a)\x82\xb5\x18%A\x20service\x20for\x20S\
    team\x20Cloud\x20operations.\
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
