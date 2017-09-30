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
pub struct CUIFontFilePB {
    // message fields
    font_file_name: ::protobuf::SingularField<::std::string::String>,
    opentype_font_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUIFontFilePB {}

impl CUIFontFilePB {
    pub fn new() -> CUIFontFilePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUIFontFilePB {
        static mut instance: ::protobuf::lazy::Lazy<CUIFontFilePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUIFontFilePB,
        };
        unsafe {
            instance.get(CUIFontFilePB::new)
        }
    }

    // optional string font_file_name = 1;

    pub fn clear_font_file_name(&mut self) {
        self.font_file_name.clear();
    }

    pub fn has_font_file_name(&self) -> bool {
        self.font_file_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_font_file_name(&mut self, v: ::std::string::String) {
        self.font_file_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_font_file_name(&mut self) -> &mut ::std::string::String {
        if self.font_file_name.is_none() {
            self.font_file_name.set_default();
        }
        self.font_file_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_font_file_name(&mut self) -> ::std::string::String {
        self.font_file_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_font_file_name(&self) -> &str {
        match self.font_file_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_font_file_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.font_file_name
    }

    fn mut_font_file_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.font_file_name
    }

    // optional bytes opentype_font_data = 2;

    pub fn clear_opentype_font_data(&mut self) {
        self.opentype_font_data.clear();
    }

    pub fn has_opentype_font_data(&self) -> bool {
        self.opentype_font_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_opentype_font_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.opentype_font_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_opentype_font_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.opentype_font_data.is_none() {
            self.opentype_font_data.set_default();
        }
        self.opentype_font_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_opentype_font_data(&mut self) -> ::std::vec::Vec<u8> {
        self.opentype_font_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_opentype_font_data(&self) -> &[u8] {
        match self.opentype_font_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_opentype_font_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.opentype_font_data
    }

    fn mut_opentype_font_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.opentype_font_data
    }
}

impl ::protobuf::Message for CUIFontFilePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.font_file_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.opentype_font_data)?;
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
        if let Some(ref v) = self.font_file_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.opentype_font_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.font_file_name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.opentype_font_data.as_ref() {
            os.write_bytes(2, &v)?;
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

impl ::protobuf::MessageStatic for CUIFontFilePB {
    fn new() -> CUIFontFilePB {
        CUIFontFilePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUIFontFilePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "font_file_name",
                    CUIFontFilePB::get_font_file_name_for_reflect,
                    CUIFontFilePB::mut_font_file_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "opentype_font_data",
                    CUIFontFilePB::get_opentype_font_data_for_reflect,
                    CUIFontFilePB::mut_opentype_font_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUIFontFilePB>(
                    "CUIFontFilePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUIFontFilePB {
    fn clear(&mut self) {
        self.clear_font_file_name();
        self.clear_opentype_font_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUIFontFilePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUIFontFilePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUIFontFilePackagePB {
    // message fields
    package_version: ::std::option::Option<u32>,
    encrypted_font_files: ::protobuf::RepeatedField<CUIFontFilePackagePB_CUIEncryptedFontFilePB>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUIFontFilePackagePB {}

impl CUIFontFilePackagePB {
    pub fn new() -> CUIFontFilePackagePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUIFontFilePackagePB {
        static mut instance: ::protobuf::lazy::Lazy<CUIFontFilePackagePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUIFontFilePackagePB,
        };
        unsafe {
            instance.get(CUIFontFilePackagePB::new)
        }
    }

    // required uint32 package_version = 1;

    pub fn clear_package_version(&mut self) {
        self.package_version = ::std::option::Option::None;
    }

    pub fn has_package_version(&self) -> bool {
        self.package_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_package_version(&mut self, v: u32) {
        self.package_version = ::std::option::Option::Some(v);
    }

    pub fn get_package_version(&self) -> u32 {
        self.package_version.unwrap_or(0)
    }

    fn get_package_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.package_version
    }

    fn mut_package_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.package_version
    }

    // repeated .CUIFontFilePackagePB.CUIEncryptedFontFilePB encrypted_font_files = 2;

    pub fn clear_encrypted_font_files(&mut self) {
        self.encrypted_font_files.clear();
    }

    // Param is passed by value, moved
    pub fn set_encrypted_font_files(&mut self, v: ::protobuf::RepeatedField<CUIFontFilePackagePB_CUIEncryptedFontFilePB>) {
        self.encrypted_font_files = v;
    }

    // Mutable pointer to the field.
    pub fn mut_encrypted_font_files(&mut self) -> &mut ::protobuf::RepeatedField<CUIFontFilePackagePB_CUIEncryptedFontFilePB> {
        &mut self.encrypted_font_files
    }

    // Take field
    pub fn take_encrypted_font_files(&mut self) -> ::protobuf::RepeatedField<CUIFontFilePackagePB_CUIEncryptedFontFilePB> {
        ::std::mem::replace(&mut self.encrypted_font_files, ::protobuf::RepeatedField::new())
    }

    pub fn get_encrypted_font_files(&self) -> &[CUIFontFilePackagePB_CUIEncryptedFontFilePB] {
        &self.encrypted_font_files
    }

    fn get_encrypted_font_files_for_reflect(&self) -> &::protobuf::RepeatedField<CUIFontFilePackagePB_CUIEncryptedFontFilePB> {
        &self.encrypted_font_files
    }

    fn mut_encrypted_font_files_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CUIFontFilePackagePB_CUIEncryptedFontFilePB> {
        &mut self.encrypted_font_files
    }
}

impl ::protobuf::Message for CUIFontFilePackagePB {
    fn is_initialized(&self) -> bool {
        if self.package_version.is_none() {
            return false;
        }
        for v in &self.encrypted_font_files {
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
                    self.package_version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.encrypted_font_files)?;
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
        if let Some(v) = self.package_version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.encrypted_font_files {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.package_version {
            os.write_uint32(1, v)?;
        }
        for v in &self.encrypted_font_files {
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

impl ::protobuf::MessageStatic for CUIFontFilePackagePB {
    fn new() -> CUIFontFilePackagePB {
        CUIFontFilePackagePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUIFontFilePackagePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "package_version",
                    CUIFontFilePackagePB::get_package_version_for_reflect,
                    CUIFontFilePackagePB::mut_package_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUIFontFilePackagePB_CUIEncryptedFontFilePB>>(
                    "encrypted_font_files",
                    CUIFontFilePackagePB::get_encrypted_font_files_for_reflect,
                    CUIFontFilePackagePB::mut_encrypted_font_files_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUIFontFilePackagePB>(
                    "CUIFontFilePackagePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUIFontFilePackagePB {
    fn clear(&mut self) {
        self.clear_package_version();
        self.clear_encrypted_font_files();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUIFontFilePackagePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUIFontFilePackagePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUIFontFilePackagePB_CUIEncryptedFontFilePB {
    // message fields
    encrypted_contents: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUIFontFilePackagePB_CUIEncryptedFontFilePB {}

impl CUIFontFilePackagePB_CUIEncryptedFontFilePB {
    pub fn new() -> CUIFontFilePackagePB_CUIEncryptedFontFilePB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUIFontFilePackagePB_CUIEncryptedFontFilePB {
        static mut instance: ::protobuf::lazy::Lazy<CUIFontFilePackagePB_CUIEncryptedFontFilePB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUIFontFilePackagePB_CUIEncryptedFontFilePB,
        };
        unsafe {
            instance.get(CUIFontFilePackagePB_CUIEncryptedFontFilePB::new)
        }
    }

    // optional bytes encrypted_contents = 1;

    pub fn clear_encrypted_contents(&mut self) {
        self.encrypted_contents.clear();
    }

    pub fn has_encrypted_contents(&self) -> bool {
        self.encrypted_contents.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encrypted_contents(&mut self, v: ::std::vec::Vec<u8>) {
        self.encrypted_contents = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encrypted_contents(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.encrypted_contents.is_none() {
            self.encrypted_contents.set_default();
        }
        self.encrypted_contents.as_mut().unwrap()
    }

    // Take field
    pub fn take_encrypted_contents(&mut self) -> ::std::vec::Vec<u8> {
        self.encrypted_contents.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encrypted_contents(&self) -> &[u8] {
        match self.encrypted_contents.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_encrypted_contents_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.encrypted_contents
    }

    fn mut_encrypted_contents_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.encrypted_contents
    }
}

impl ::protobuf::Message for CUIFontFilePackagePB_CUIEncryptedFontFilePB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.encrypted_contents)?;
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
        if let Some(ref v) = self.encrypted_contents.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.encrypted_contents.as_ref() {
            os.write_bytes(1, &v)?;
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

impl ::protobuf::MessageStatic for CUIFontFilePackagePB_CUIEncryptedFontFilePB {
    fn new() -> CUIFontFilePackagePB_CUIEncryptedFontFilePB {
        CUIFontFilePackagePB_CUIEncryptedFontFilePB::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUIFontFilePackagePB_CUIEncryptedFontFilePB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "encrypted_contents",
                    CUIFontFilePackagePB_CUIEncryptedFontFilePB::get_encrypted_contents_for_reflect,
                    CUIFontFilePackagePB_CUIEncryptedFontFilePB::mut_encrypted_contents_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUIFontFilePackagePB_CUIEncryptedFontFilePB>(
                    "CUIFontFilePackagePB_CUIEncryptedFontFilePB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUIFontFilePackagePB_CUIEncryptedFontFilePB {
    fn clear(&mut self) {
        self.clear_encrypted_contents();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUIFontFilePackagePB_CUIEncryptedFontFilePB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUIFontFilePackagePB_CUIEncryptedFontFilePB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17uifontfile_format.proto\"c\n\rCUIFontFilePB\x12$\n\x0efont_file_na\
    me\x18\x01\x20\x01(\tR\x0cfontFileName\x12,\n\x12opentype_font_data\x18\
    \x02\x20\x01(\x0cR\x10opentypeFontData\"\xe8\x01\n\x14CUIFontFilePackage\
    PB\x12'\n\x0fpackage_version\x18\x01\x20\x02(\rR\x0epackageVersion\x12^\
    \n\x14encrypted_font_files\x18\x02\x20\x03(\x0b2,.CUIFontFilePackagePB.C\
    UIEncryptedFontFilePBR\x12encryptedFontFiles\x1aG\n\x16CUIEncryptedFontF\
    ilePB\x12-\n\x12encrypted_contents\x18\x01\x20\x01(\x0cR\x11encryptedCon\
    tentsB\x05H\x01\x80\x01\0\
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
