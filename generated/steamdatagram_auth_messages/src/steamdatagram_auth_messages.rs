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
pub struct CMsgSteamDatagramRelayAuthTicket {
    // message fields
    time_expiry: ::std::option::Option<u32>,
    authorized_steam_id: ::std::option::Option<u64>,
    authorized_public_ip: ::std::option::Option<u32>,
    gameserver_steam_id: ::std::option::Option<u64>,
    gameserver_net_id: ::std::option::Option<u64>,
    legacy_signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    app_id: ::std::option::Option<u32>,
    extra_fields: ::protobuf::RepeatedField<CMsgSteamDatagramRelayAuthTicket_ExtraField>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramRelayAuthTicket {}

impl CMsgSteamDatagramRelayAuthTicket {
    pub fn new() -> CMsgSteamDatagramRelayAuthTicket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramRelayAuthTicket {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramRelayAuthTicket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramRelayAuthTicket,
        };
        unsafe {
            instance.get(CMsgSteamDatagramRelayAuthTicket::new)
        }
    }

    // optional fixed32 time_expiry = 1;

    pub fn clear_time_expiry(&mut self) {
        self.time_expiry = ::std::option::Option::None;
    }

    pub fn has_time_expiry(&self) -> bool {
        self.time_expiry.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_expiry(&mut self, v: u32) {
        self.time_expiry = ::std::option::Option::Some(v);
    }

    pub fn get_time_expiry(&self) -> u32 {
        self.time_expiry.unwrap_or(0)
    }

    fn get_time_expiry_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_expiry
    }

    fn mut_time_expiry_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_expiry
    }

    // optional fixed64 authorized_steam_id = 2;

    pub fn clear_authorized_steam_id(&mut self) {
        self.authorized_steam_id = ::std::option::Option::None;
    }

    pub fn has_authorized_steam_id(&self) -> bool {
        self.authorized_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_authorized_steam_id(&mut self, v: u64) {
        self.authorized_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_authorized_steam_id(&self) -> u64 {
        self.authorized_steam_id.unwrap_or(0)
    }

    fn get_authorized_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.authorized_steam_id
    }

    fn mut_authorized_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.authorized_steam_id
    }

    // optional fixed32 authorized_public_ip = 3;

    pub fn clear_authorized_public_ip(&mut self) {
        self.authorized_public_ip = ::std::option::Option::None;
    }

    pub fn has_authorized_public_ip(&self) -> bool {
        self.authorized_public_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_authorized_public_ip(&mut self, v: u32) {
        self.authorized_public_ip = ::std::option::Option::Some(v);
    }

    pub fn get_authorized_public_ip(&self) -> u32 {
        self.authorized_public_ip.unwrap_or(0)
    }

    fn get_authorized_public_ip_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.authorized_public_ip
    }

    fn mut_authorized_public_ip_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.authorized_public_ip
    }

    // optional fixed64 gameserver_steam_id = 4;

    pub fn clear_gameserver_steam_id(&mut self) {
        self.gameserver_steam_id = ::std::option::Option::None;
    }

    pub fn has_gameserver_steam_id(&self) -> bool {
        self.gameserver_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gameserver_steam_id(&mut self, v: u64) {
        self.gameserver_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_gameserver_steam_id(&self) -> u64 {
        self.gameserver_steam_id.unwrap_or(0)
    }

    fn get_gameserver_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.gameserver_steam_id
    }

    fn mut_gameserver_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.gameserver_steam_id
    }

    // optional fixed64 gameserver_net_id = 5;

    pub fn clear_gameserver_net_id(&mut self) {
        self.gameserver_net_id = ::std::option::Option::None;
    }

    pub fn has_gameserver_net_id(&self) -> bool {
        self.gameserver_net_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gameserver_net_id(&mut self, v: u64) {
        self.gameserver_net_id = ::std::option::Option::Some(v);
    }

    pub fn get_gameserver_net_id(&self) -> u64 {
        self.gameserver_net_id.unwrap_or(0)
    }

    fn get_gameserver_net_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.gameserver_net_id
    }

    fn mut_gameserver_net_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.gameserver_net_id
    }

    // optional bytes legacy_signature = 6;

    pub fn clear_legacy_signature(&mut self) {
        self.legacy_signature.clear();
    }

    pub fn has_legacy_signature(&self) -> bool {
        self.legacy_signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.legacy_signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_legacy_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.legacy_signature.is_none() {
            self.legacy_signature.set_default();
        }
        self.legacy_signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_legacy_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.legacy_signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_legacy_signature(&self) -> &[u8] {
        match self.legacy_signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_legacy_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.legacy_signature
    }

    fn mut_legacy_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.legacy_signature
    }

    // optional uint32 app_id = 7;

    pub fn clear_app_id(&mut self) {
        self.app_id = ::std::option::Option::None;
    }

    pub fn has_app_id(&self) -> bool {
        self.app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_id(&mut self, v: u32) {
        self.app_id = ::std::option::Option::Some(v);
    }

    pub fn get_app_id(&self) -> u32 {
        self.app_id.unwrap_or(0)
    }

    fn get_app_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.app_id
    }

    fn mut_app_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.app_id
    }

    // repeated .CMsgSteamDatagramRelayAuthTicket.ExtraField extra_fields = 8;

    pub fn clear_extra_fields(&mut self) {
        self.extra_fields.clear();
    }

    // Param is passed by value, moved
    pub fn set_extra_fields(&mut self, v: ::protobuf::RepeatedField<CMsgSteamDatagramRelayAuthTicket_ExtraField>) {
        self.extra_fields = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extra_fields(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramRelayAuthTicket_ExtraField> {
        &mut self.extra_fields
    }

    // Take field
    pub fn take_extra_fields(&mut self) -> ::protobuf::RepeatedField<CMsgSteamDatagramRelayAuthTicket_ExtraField> {
        ::std::mem::replace(&mut self.extra_fields, ::protobuf::RepeatedField::new())
    }

    pub fn get_extra_fields(&self) -> &[CMsgSteamDatagramRelayAuthTicket_ExtraField] {
        &self.extra_fields
    }

    fn get_extra_fields_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSteamDatagramRelayAuthTicket_ExtraField> {
        &self.extra_fields
    }

    fn mut_extra_fields_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramRelayAuthTicket_ExtraField> {
        &mut self.extra_fields
    }
}

impl ::protobuf::Message for CMsgSteamDatagramRelayAuthTicket {
    fn is_initialized(&self) -> bool {
        for v in &self.extra_fields {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.time_expiry = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.authorized_steam_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.authorized_public_ip = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.gameserver_steam_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.gameserver_net_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.legacy_signature)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.app_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.extra_fields)?;
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
        if let Some(v) = self.time_expiry {
            my_size += 5;
        }
        if let Some(v) = self.authorized_steam_id {
            my_size += 9;
        }
        if let Some(v) = self.authorized_public_ip {
            my_size += 5;
        }
        if let Some(v) = self.gameserver_steam_id {
            my_size += 9;
        }
        if let Some(v) = self.gameserver_net_id {
            my_size += 9;
        }
        if let Some(ref v) = self.legacy_signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
        }
        if let Some(v) = self.app_id {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.extra_fields {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.time_expiry {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.authorized_steam_id {
            os.write_fixed64(2, v)?;
        }
        if let Some(v) = self.authorized_public_ip {
            os.write_fixed32(3, v)?;
        }
        if let Some(v) = self.gameserver_steam_id {
            os.write_fixed64(4, v)?;
        }
        if let Some(v) = self.gameserver_net_id {
            os.write_fixed64(5, v)?;
        }
        if let Some(ref v) = self.legacy_signature.as_ref() {
            os.write_bytes(6, &v)?;
        }
        if let Some(v) = self.app_id {
            os.write_uint32(7, v)?;
        }
        for v in &self.extra_fields {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramRelayAuthTicket {
    fn new() -> CMsgSteamDatagramRelayAuthTicket {
        CMsgSteamDatagramRelayAuthTicket::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramRelayAuthTicket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "time_expiry",
                    CMsgSteamDatagramRelayAuthTicket::get_time_expiry_for_reflect,
                    CMsgSteamDatagramRelayAuthTicket::mut_time_expiry_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "authorized_steam_id",
                    CMsgSteamDatagramRelayAuthTicket::get_authorized_steam_id_for_reflect,
                    CMsgSteamDatagramRelayAuthTicket::mut_authorized_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "authorized_public_ip",
                    CMsgSteamDatagramRelayAuthTicket::get_authorized_public_ip_for_reflect,
                    CMsgSteamDatagramRelayAuthTicket::mut_authorized_public_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "gameserver_steam_id",
                    CMsgSteamDatagramRelayAuthTicket::get_gameserver_steam_id_for_reflect,
                    CMsgSteamDatagramRelayAuthTicket::mut_gameserver_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "gameserver_net_id",
                    CMsgSteamDatagramRelayAuthTicket::get_gameserver_net_id_for_reflect,
                    CMsgSteamDatagramRelayAuthTicket::mut_gameserver_net_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "legacy_signature",
                    CMsgSteamDatagramRelayAuthTicket::get_legacy_signature_for_reflect,
                    CMsgSteamDatagramRelayAuthTicket::mut_legacy_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "app_id",
                    CMsgSteamDatagramRelayAuthTicket::get_app_id_for_reflect,
                    CMsgSteamDatagramRelayAuthTicket::mut_app_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramRelayAuthTicket_ExtraField>>(
                    "extra_fields",
                    CMsgSteamDatagramRelayAuthTicket::get_extra_fields_for_reflect,
                    CMsgSteamDatagramRelayAuthTicket::mut_extra_fields_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramRelayAuthTicket>(
                    "CMsgSteamDatagramRelayAuthTicket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramRelayAuthTicket {
    fn clear(&mut self) {
        self.clear_time_expiry();
        self.clear_authorized_steam_id();
        self.clear_authorized_public_ip();
        self.clear_gameserver_steam_id();
        self.clear_gameserver_net_id();
        self.clear_legacy_signature();
        self.clear_app_id();
        self.clear_extra_fields();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramRelayAuthTicket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramRelayAuthTicket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramRelayAuthTicket_ExtraField {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    string_value: ::protobuf::SingularField<::std::string::String>,
    int64_value: ::std::option::Option<i64>,
    fixed64_value: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramRelayAuthTicket_ExtraField {}

impl CMsgSteamDatagramRelayAuthTicket_ExtraField {
    pub fn new() -> CMsgSteamDatagramRelayAuthTicket_ExtraField {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramRelayAuthTicket_ExtraField {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramRelayAuthTicket_ExtraField> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramRelayAuthTicket_ExtraField,
        };
        unsafe {
            instance.get(CMsgSteamDatagramRelayAuthTicket_ExtraField::new)
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

    // optional string string_value = 2;

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

    // optional sint64 int64_value = 3;

    pub fn clear_int64_value(&mut self) {
        self.int64_value = ::std::option::Option::None;
    }

    pub fn has_int64_value(&self) -> bool {
        self.int64_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int64_value(&mut self, v: i64) {
        self.int64_value = ::std::option::Option::Some(v);
    }

    pub fn get_int64_value(&self) -> i64 {
        self.int64_value.unwrap_or(0)
    }

    fn get_int64_value_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.int64_value
    }

    fn mut_int64_value_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.int64_value
    }

    // optional fixed64 fixed64_value = 5;

    pub fn clear_fixed64_value(&mut self) {
        self.fixed64_value = ::std::option::Option::None;
    }

    pub fn has_fixed64_value(&self) -> bool {
        self.fixed64_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fixed64_value(&mut self, v: u64) {
        self.fixed64_value = ::std::option::Option::Some(v);
    }

    pub fn get_fixed64_value(&self) -> u64 {
        self.fixed64_value.unwrap_or(0)
    }

    fn get_fixed64_value_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.fixed64_value
    }

    fn mut_fixed64_value_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.fixed64_value
    }
}

impl ::protobuf::Message for CMsgSteamDatagramRelayAuthTicket_ExtraField {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.string_value)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.int64_value = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.fixed64_value = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.string_value.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.int64_value {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        }
        if let Some(v) = self.fixed64_value {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.string_value.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.int64_value {
            os.write_sint64(3, v)?;
        }
        if let Some(v) = self.fixed64_value {
            os.write_fixed64(5, v)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramRelayAuthTicket_ExtraField {
    fn new() -> CMsgSteamDatagramRelayAuthTicket_ExtraField {
        CMsgSteamDatagramRelayAuthTicket_ExtraField::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramRelayAuthTicket_ExtraField>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgSteamDatagramRelayAuthTicket_ExtraField::get_name_for_reflect,
                    CMsgSteamDatagramRelayAuthTicket_ExtraField::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "string_value",
                    CMsgSteamDatagramRelayAuthTicket_ExtraField::get_string_value_for_reflect,
                    CMsgSteamDatagramRelayAuthTicket_ExtraField::mut_string_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "int64_value",
                    CMsgSteamDatagramRelayAuthTicket_ExtraField::get_int64_value_for_reflect,
                    CMsgSteamDatagramRelayAuthTicket_ExtraField::mut_int64_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "fixed64_value",
                    CMsgSteamDatagramRelayAuthTicket_ExtraField::get_fixed64_value_for_reflect,
                    CMsgSteamDatagramRelayAuthTicket_ExtraField::mut_fixed64_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramRelayAuthTicket_ExtraField>(
                    "CMsgSteamDatagramRelayAuthTicket_ExtraField",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramRelayAuthTicket_ExtraField {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_string_value();
        self.clear_int64_value();
        self.clear_fixed64_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramRelayAuthTicket_ExtraField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramRelayAuthTicket_ExtraField {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramSignedRelayAuthTicket {
    // message fields
    reserved_do_not_use: ::std::option::Option<u64>,
    key_id: ::std::option::Option<u64>,
    ticket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramSignedRelayAuthTicket {}

impl CMsgSteamDatagramSignedRelayAuthTicket {
    pub fn new() -> CMsgSteamDatagramSignedRelayAuthTicket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramSignedRelayAuthTicket {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramSignedRelayAuthTicket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramSignedRelayAuthTicket,
        };
        unsafe {
            instance.get(CMsgSteamDatagramSignedRelayAuthTicket::new)
        }
    }

    // optional fixed64 reserved_do_not_use = 1;

    pub fn clear_reserved_do_not_use(&mut self) {
        self.reserved_do_not_use = ::std::option::Option::None;
    }

    pub fn has_reserved_do_not_use(&self) -> bool {
        self.reserved_do_not_use.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reserved_do_not_use(&mut self, v: u64) {
        self.reserved_do_not_use = ::std::option::Option::Some(v);
    }

    pub fn get_reserved_do_not_use(&self) -> u64 {
        self.reserved_do_not_use.unwrap_or(0)
    }

    fn get_reserved_do_not_use_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.reserved_do_not_use
    }

    fn mut_reserved_do_not_use_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.reserved_do_not_use
    }

    // optional fixed64 key_id = 2;

    pub fn clear_key_id(&mut self) {
        self.key_id = ::std::option::Option::None;
    }

    pub fn has_key_id(&self) -> bool {
        self.key_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_id(&mut self, v: u64) {
        self.key_id = ::std::option::Option::Some(v);
    }

    pub fn get_key_id(&self) -> u64 {
        self.key_id.unwrap_or(0)
    }

    fn get_key_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.key_id
    }

    fn mut_key_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.key_id
    }

    // optional bytes ticket = 3;

    pub fn clear_ticket(&mut self) {
        self.ticket.clear();
    }

    pub fn has_ticket(&self) -> bool {
        self.ticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ticket(&mut self, v: ::std::vec::Vec<u8>) {
        self.ticket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ticket(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.ticket.is_none() {
            self.ticket.set_default();
        }
        self.ticket.as_mut().unwrap()
    }

    // Take field
    pub fn take_ticket(&mut self) -> ::std::vec::Vec<u8> {
        self.ticket.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ticket(&self) -> &[u8] {
        match self.ticket.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_ticket_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.ticket
    }

    fn mut_ticket_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.ticket
    }

    // optional bytes signature = 4;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.signature.is_none() {
            self.signature.set_default();
        }
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        match self.signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.signature
    }
}

impl ::protobuf::Message for CMsgSteamDatagramSignedRelayAuthTicket {
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
                    self.reserved_do_not_use = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.key_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ticket)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signature)?;
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
        if let Some(v) = self.reserved_do_not_use {
            my_size += 9;
        }
        if let Some(v) = self.key_id {
            my_size += 9;
        }
        if let Some(ref v) = self.ticket.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reserved_do_not_use {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.key_id {
            os.write_fixed64(2, v)?;
        }
        if let Some(ref v) = self.ticket.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(ref v) = self.signature.as_ref() {
            os.write_bytes(4, &v)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramSignedRelayAuthTicket {
    fn new() -> CMsgSteamDatagramSignedRelayAuthTicket {
        CMsgSteamDatagramSignedRelayAuthTicket::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramSignedRelayAuthTicket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "reserved_do_not_use",
                    CMsgSteamDatagramSignedRelayAuthTicket::get_reserved_do_not_use_for_reflect,
                    CMsgSteamDatagramSignedRelayAuthTicket::mut_reserved_do_not_use_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "key_id",
                    CMsgSteamDatagramSignedRelayAuthTicket::get_key_id_for_reflect,
                    CMsgSteamDatagramSignedRelayAuthTicket::mut_key_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ticket",
                    CMsgSteamDatagramSignedRelayAuthTicket::get_ticket_for_reflect,
                    CMsgSteamDatagramSignedRelayAuthTicket::mut_ticket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    CMsgSteamDatagramSignedRelayAuthTicket::get_signature_for_reflect,
                    CMsgSteamDatagramSignedRelayAuthTicket::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramSignedRelayAuthTicket>(
                    "CMsgSteamDatagramSignedRelayAuthTicket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramSignedRelayAuthTicket {
    fn clear(&mut self) {
        self.clear_reserved_do_not_use();
        self.clear_key_id();
        self.clear_ticket();
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramSignedRelayAuthTicket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramSignedRelayAuthTicket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramCertificate {
    // message fields
    key_type: ::std::option::Option<CMsgSteamDatagramCertificate_EKeyType>,
    key_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    steam_id: ::std::option::Option<u64>,
    gameserver_datacenter_id: ::std::option::Option<u32>,
    time_created: ::std::option::Option<u32>,
    time_expiry: ::std::option::Option<u32>,
    app_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramCertificate {}

impl CMsgSteamDatagramCertificate {
    pub fn new() -> CMsgSteamDatagramCertificate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramCertificate {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramCertificate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramCertificate,
        };
        unsafe {
            instance.get(CMsgSteamDatagramCertificate::new)
        }
    }

    // optional .CMsgSteamDatagramCertificate.EKeyType key_type = 1;

    pub fn clear_key_type(&mut self) {
        self.key_type = ::std::option::Option::None;
    }

    pub fn has_key_type(&self) -> bool {
        self.key_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_type(&mut self, v: CMsgSteamDatagramCertificate_EKeyType) {
        self.key_type = ::std::option::Option::Some(v);
    }

    pub fn get_key_type(&self) -> CMsgSteamDatagramCertificate_EKeyType {
        self.key_type.unwrap_or(CMsgSteamDatagramCertificate_EKeyType::INVALID)
    }

    fn get_key_type_for_reflect(&self) -> &::std::option::Option<CMsgSteamDatagramCertificate_EKeyType> {
        &self.key_type
    }

    fn mut_key_type_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgSteamDatagramCertificate_EKeyType> {
        &mut self.key_type
    }

    // optional bytes key_data = 2;

    pub fn clear_key_data(&mut self) {
        self.key_data.clear();
    }

    pub fn has_key_data(&self) -> bool {
        self.key_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.key_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key_data.is_none() {
            self.key_data.set_default();
        }
        self.key_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_key_data(&mut self) -> ::std::vec::Vec<u8> {
        self.key_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key_data(&self) -> &[u8] {
        match self.key_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key_data
    }

    fn mut_key_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key_data
    }

    // optional fixed64 steam_id = 4;

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

    // optional fixed32 gameserver_datacenter_id = 5;

    pub fn clear_gameserver_datacenter_id(&mut self) {
        self.gameserver_datacenter_id = ::std::option::Option::None;
    }

    pub fn has_gameserver_datacenter_id(&self) -> bool {
        self.gameserver_datacenter_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gameserver_datacenter_id(&mut self, v: u32) {
        self.gameserver_datacenter_id = ::std::option::Option::Some(v);
    }

    pub fn get_gameserver_datacenter_id(&self) -> u32 {
        self.gameserver_datacenter_id.unwrap_or(0)
    }

    fn get_gameserver_datacenter_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.gameserver_datacenter_id
    }

    fn mut_gameserver_datacenter_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.gameserver_datacenter_id
    }

    // optional fixed32 time_created = 8;

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

    // optional fixed32 time_expiry = 9;

    pub fn clear_time_expiry(&mut self) {
        self.time_expiry = ::std::option::Option::None;
    }

    pub fn has_time_expiry(&self) -> bool {
        self.time_expiry.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_expiry(&mut self, v: u32) {
        self.time_expiry = ::std::option::Option::Some(v);
    }

    pub fn get_time_expiry(&self) -> u32 {
        self.time_expiry.unwrap_or(0)
    }

    fn get_time_expiry_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_expiry
    }

    fn mut_time_expiry_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_expiry
    }

    // optional uint32 app_id = 10;

    pub fn clear_app_id(&mut self) {
        self.app_id = ::std::option::Option::None;
    }

    pub fn has_app_id(&self) -> bool {
        self.app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_id(&mut self, v: u32) {
        self.app_id = ::std::option::Option::Some(v);
    }

    pub fn get_app_id(&self) -> u32 {
        self.app_id.unwrap_or(0)
    }

    fn get_app_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.app_id
    }

    fn mut_app_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.app_id
    }
}

impl ::protobuf::Message for CMsgSteamDatagramCertificate {
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
                    self.key_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key_data)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.gameserver_datacenter_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.time_created = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.time_expiry = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.app_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.key_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.key_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.steam_id {
            my_size += 9;
        }
        if let Some(v) = self.gameserver_datacenter_id {
            my_size += 5;
        }
        if let Some(v) = self.time_created {
            my_size += 5;
        }
        if let Some(v) = self.time_expiry {
            my_size += 5;
        }
        if let Some(v) = self.app_id {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.key_data.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(v) = self.steam_id {
            os.write_fixed64(4, v)?;
        }
        if let Some(v) = self.gameserver_datacenter_id {
            os.write_fixed32(5, v)?;
        }
        if let Some(v) = self.time_created {
            os.write_fixed32(8, v)?;
        }
        if let Some(v) = self.time_expiry {
            os.write_fixed32(9, v)?;
        }
        if let Some(v) = self.app_id {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramCertificate {
    fn new() -> CMsgSteamDatagramCertificate {
        CMsgSteamDatagramCertificate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramCertificate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgSteamDatagramCertificate_EKeyType>>(
                    "key_type",
                    CMsgSteamDatagramCertificate::get_key_type_for_reflect,
                    CMsgSteamDatagramCertificate::mut_key_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key_data",
                    CMsgSteamDatagramCertificate::get_key_data_for_reflect,
                    CMsgSteamDatagramCertificate::mut_key_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgSteamDatagramCertificate::get_steam_id_for_reflect,
                    CMsgSteamDatagramCertificate::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "gameserver_datacenter_id",
                    CMsgSteamDatagramCertificate::get_gameserver_datacenter_id_for_reflect,
                    CMsgSteamDatagramCertificate::mut_gameserver_datacenter_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "time_created",
                    CMsgSteamDatagramCertificate::get_time_created_for_reflect,
                    CMsgSteamDatagramCertificate::mut_time_created_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "time_expiry",
                    CMsgSteamDatagramCertificate::get_time_expiry_for_reflect,
                    CMsgSteamDatagramCertificate::mut_time_expiry_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "app_id",
                    CMsgSteamDatagramCertificate::get_app_id_for_reflect,
                    CMsgSteamDatagramCertificate::mut_app_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramCertificate>(
                    "CMsgSteamDatagramCertificate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramCertificate {
    fn clear(&mut self) {
        self.clear_key_type();
        self.clear_key_data();
        self.clear_steam_id();
        self.clear_gameserver_datacenter_id();
        self.clear_time_created();
        self.clear_time_expiry();
        self.clear_app_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramCertificate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramCertificate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgSteamDatagramCertificate_EKeyType {
    INVALID = 0,
    ED25519 = 1,
}

impl ::protobuf::ProtobufEnum for CMsgSteamDatagramCertificate_EKeyType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgSteamDatagramCertificate_EKeyType> {
        match value {
            0 => ::std::option::Option::Some(CMsgSteamDatagramCertificate_EKeyType::INVALID),
            1 => ::std::option::Option::Some(CMsgSteamDatagramCertificate_EKeyType::ED25519),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgSteamDatagramCertificate_EKeyType] = &[
            CMsgSteamDatagramCertificate_EKeyType::INVALID,
            CMsgSteamDatagramCertificate_EKeyType::ED25519,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CMsgSteamDatagramCertificate_EKeyType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgSteamDatagramCertificate_EKeyType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgSteamDatagramCertificate_EKeyType {
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramCertificate_EKeyType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramCertificateSigned {
    // message fields
    cert: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    ca_key_id: ::std::option::Option<u64>,
    ca_signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramCertificateSigned {}

impl CMsgSteamDatagramCertificateSigned {
    pub fn new() -> CMsgSteamDatagramCertificateSigned {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramCertificateSigned {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramCertificateSigned> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramCertificateSigned,
        };
        unsafe {
            instance.get(CMsgSteamDatagramCertificateSigned::new)
        }
    }

    // optional bytes cert = 4;

    pub fn clear_cert(&mut self) {
        self.cert.clear();
    }

    pub fn has_cert(&self) -> bool {
        self.cert.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cert(&mut self, v: ::std::vec::Vec<u8>) {
        self.cert = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cert(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.cert.is_none() {
            self.cert.set_default();
        }
        self.cert.as_mut().unwrap()
    }

    // Take field
    pub fn take_cert(&mut self) -> ::std::vec::Vec<u8> {
        self.cert.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_cert(&self) -> &[u8] {
        match self.cert.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_cert_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.cert
    }

    fn mut_cert_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.cert
    }

    // optional fixed64 ca_key_id = 5;

    pub fn clear_ca_key_id(&mut self) {
        self.ca_key_id = ::std::option::Option::None;
    }

    pub fn has_ca_key_id(&self) -> bool {
        self.ca_key_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ca_key_id(&mut self, v: u64) {
        self.ca_key_id = ::std::option::Option::Some(v);
    }

    pub fn get_ca_key_id(&self) -> u64 {
        self.ca_key_id.unwrap_or(0)
    }

    fn get_ca_key_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ca_key_id
    }

    fn mut_ca_key_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ca_key_id
    }

    // optional bytes ca_signature = 6;

    pub fn clear_ca_signature(&mut self) {
        self.ca_signature.clear();
    }

    pub fn has_ca_signature(&self) -> bool {
        self.ca_signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ca_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.ca_signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ca_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.ca_signature.is_none() {
            self.ca_signature.set_default();
        }
        self.ca_signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_ca_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.ca_signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ca_signature(&self) -> &[u8] {
        match self.ca_signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_ca_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.ca_signature
    }

    fn mut_ca_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.ca_signature
    }
}

impl ::protobuf::Message for CMsgSteamDatagramCertificateSigned {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.cert)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.ca_key_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ca_signature)?;
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
        if let Some(ref v) = self.cert.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(v) = self.ca_key_id {
            my_size += 9;
        }
        if let Some(ref v) = self.ca_signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.cert.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(v) = self.ca_key_id {
            os.write_fixed64(5, v)?;
        }
        if let Some(ref v) = self.ca_signature.as_ref() {
            os.write_bytes(6, &v)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramCertificateSigned {
    fn new() -> CMsgSteamDatagramCertificateSigned {
        CMsgSteamDatagramCertificateSigned::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramCertificateSigned>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "cert",
                    CMsgSteamDatagramCertificateSigned::get_cert_for_reflect,
                    CMsgSteamDatagramCertificateSigned::mut_cert_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "ca_key_id",
                    CMsgSteamDatagramCertificateSigned::get_ca_key_id_for_reflect,
                    CMsgSteamDatagramCertificateSigned::mut_ca_key_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ca_signature",
                    CMsgSteamDatagramCertificateSigned::get_ca_signature_for_reflect,
                    CMsgSteamDatagramCertificateSigned::mut_ca_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramCertificateSigned>(
                    "CMsgSteamDatagramCertificateSigned",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramCertificateSigned {
    fn clear(&mut self) {
        self.clear_cert();
        self.clear_ca_key_id();
        self.clear_ca_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramCertificateSigned {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramCertificateSigned {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!steamdatagram_auth_messages.proto\"\xa0\x04\n\x20CMsgSteamDatagramRel\
    ayAuthTicket\x12\x1f\n\x0btime_expiry\x18\x01\x20\x01(\x07R\ntimeExpiry\
    \x12.\n\x13authorized_steam_id\x18\x02\x20\x01(\x06R\x11authorizedSteamI\
    d\x120\n\x14authorized_public_ip\x18\x03\x20\x01(\x07R\x12authorizedPubl\
    icIp\x12.\n\x13gameserver_steam_id\x18\x04\x20\x01(\x06R\x11gameserverSt\
    eamId\x12*\n\x11gameserver_net_id\x18\x05\x20\x01(\x06R\x0fgameserverNet\
    Id\x12)\n\x10legacy_signature\x18\x06\x20\x01(\x0cR\x0flegacySignature\
    \x12\x15\n\x06app_id\x18\x07\x20\x01(\rR\x05appId\x12O\n\x0cextra_fields\
    \x18\x08\x20\x03(\x0b2,.CMsgSteamDatagramRelayAuthTicket.ExtraFieldR\x0b\
    extraFields\x1a\x89\x01\n\nExtraField\x12\x12\n\x04name\x18\x01\x20\x01(\
    \tR\x04name\x12!\n\x0cstring_value\x18\x02\x20\x01(\tR\x0bstringValue\
    \x12\x1f\n\x0bint64_value\x18\x03\x20\x01(\x12R\nint64Value\x12#\n\rfixe\
    d64_value\x18\x05\x20\x01(\x06R\x0cfixed64Value\"\xa4\x01\n&CMsgSteamDat\
    agramSignedRelayAuthTicket\x12-\n\x13reserved_do_not_use\x18\x01\x20\x01\
    (\x06R\x10reservedDoNotUse\x12\x15\n\x06key_id\x18\x02\x20\x01(\x06R\x05\
    keyId\x12\x16\n\x06ticket\x18\x03\x20\x01(\x0cR\x06ticket\x12\x1c\n\tsig\
    nature\x18\x04\x20\x01(\x0cR\tsignature\"\xdb\x02\n\x1cCMsgSteamDatagram\
    Certificate\x12J\n\x08key_type\x18\x01\x20\x01(\x0e2&.CMsgSteamDatagramC\
    ertificate.EKeyType:\x07INVALIDR\x07keyType\x12\x19\n\x08key_data\x18\
    \x02\x20\x01(\x0cR\x07keyData\x12\x19\n\x08steam_id\x18\x04\x20\x01(\x06\
    R\x07steamId\x128\n\x18gameserver_datacenter_id\x18\x05\x20\x01(\x07R\
    \x16gameserverDatacenterId\x12!\n\x0ctime_created\x18\x08\x20\x01(\x07R\
    \x0btimeCreated\x12\x1f\n\x0btime_expiry\x18\t\x20\x01(\x07R\ntimeExpiry\
    \x12\x15\n\x06app_id\x18\n\x20\x01(\rR\x05appId\"$\n\x08EKeyType\x12\x0b\
    \n\x07INVALID\x10\0\x12\x0b\n\x07ED25519\x10\x01\"w\n\"CMsgSteamDatagram\
    CertificateSigned\x12\x12\n\x04cert\x18\x04\x20\x01(\x0cR\x04cert\x12\
    \x1a\n\tca_key_id\x18\x05\x20\x01(\x06R\x07caKeyId\x12!\n\x0cca_signatur\
    e\x18\x06\x20\x01(\x0cR\x0bcaSignatureB\x03\x80\x01\0\
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
