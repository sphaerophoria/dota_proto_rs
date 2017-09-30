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
pub struct C2S_CONNECT_Message {
    // message fields
    host_version: ::std::option::Option<u32>,
    auth_protocol: ::std::option::Option<u32>,
    challenge_number: ::std::option::Option<u32>,
    reservation_cookie: ::std::option::Option<u64>,
    low_violence: ::std::option::Option<bool>,
    encrypted_password: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    splitplayers: ::protobuf::RepeatedField<super::netmessages::CCLCMsg_SplitPlayerConnect>,
    auth_steam: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    challenge_context: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for C2S_CONNECT_Message {}

impl C2S_CONNECT_Message {
    pub fn new() -> C2S_CONNECT_Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static C2S_CONNECT_Message {
        static mut instance: ::protobuf::lazy::Lazy<C2S_CONNECT_Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const C2S_CONNECT_Message,
        };
        unsafe {
            instance.get(C2S_CONNECT_Message::new)
        }
    }

    // optional uint32 host_version = 1;

    pub fn clear_host_version(&mut self) {
        self.host_version = ::std::option::Option::None;
    }

    pub fn has_host_version(&self) -> bool {
        self.host_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_version(&mut self, v: u32) {
        self.host_version = ::std::option::Option::Some(v);
    }

    pub fn get_host_version(&self) -> u32 {
        self.host_version.unwrap_or(0)
    }

    fn get_host_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.host_version
    }

    fn mut_host_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.host_version
    }

    // optional uint32 auth_protocol = 2;

    pub fn clear_auth_protocol(&mut self) {
        self.auth_protocol = ::std::option::Option::None;
    }

    pub fn has_auth_protocol(&self) -> bool {
        self.auth_protocol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_protocol(&mut self, v: u32) {
        self.auth_protocol = ::std::option::Option::Some(v);
    }

    pub fn get_auth_protocol(&self) -> u32 {
        self.auth_protocol.unwrap_or(0)
    }

    fn get_auth_protocol_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.auth_protocol
    }

    fn mut_auth_protocol_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.auth_protocol
    }

    // optional uint32 challenge_number = 3;

    pub fn clear_challenge_number(&mut self) {
        self.challenge_number = ::std::option::Option::None;
    }

    pub fn has_challenge_number(&self) -> bool {
        self.challenge_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge_number(&mut self, v: u32) {
        self.challenge_number = ::std::option::Option::Some(v);
    }

    pub fn get_challenge_number(&self) -> u32 {
        self.challenge_number.unwrap_or(0)
    }

    fn get_challenge_number_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.challenge_number
    }

    fn mut_challenge_number_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.challenge_number
    }

    // optional fixed64 reservation_cookie = 4;

    pub fn clear_reservation_cookie(&mut self) {
        self.reservation_cookie = ::std::option::Option::None;
    }

    pub fn has_reservation_cookie(&self) -> bool {
        self.reservation_cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reservation_cookie(&mut self, v: u64) {
        self.reservation_cookie = ::std::option::Option::Some(v);
    }

    pub fn get_reservation_cookie(&self) -> u64 {
        self.reservation_cookie.unwrap_or(0)
    }

    fn get_reservation_cookie_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.reservation_cookie
    }

    fn mut_reservation_cookie_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.reservation_cookie
    }

    // optional bool low_violence = 5;

    pub fn clear_low_violence(&mut self) {
        self.low_violence = ::std::option::Option::None;
    }

    pub fn has_low_violence(&self) -> bool {
        self.low_violence.is_some()
    }

    // Param is passed by value, moved
    pub fn set_low_violence(&mut self, v: bool) {
        self.low_violence = ::std::option::Option::Some(v);
    }

    pub fn get_low_violence(&self) -> bool {
        self.low_violence.unwrap_or(false)
    }

    fn get_low_violence_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.low_violence
    }

    fn mut_low_violence_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.low_violence
    }

    // optional bytes encrypted_password = 6;

    pub fn clear_encrypted_password(&mut self) {
        self.encrypted_password.clear();
    }

    pub fn has_encrypted_password(&self) -> bool {
        self.encrypted_password.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encrypted_password(&mut self, v: ::std::vec::Vec<u8>) {
        self.encrypted_password = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encrypted_password(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.encrypted_password.is_none() {
            self.encrypted_password.set_default();
        }
        self.encrypted_password.as_mut().unwrap()
    }

    // Take field
    pub fn take_encrypted_password(&mut self) -> ::std::vec::Vec<u8> {
        self.encrypted_password.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encrypted_password(&self) -> &[u8] {
        match self.encrypted_password.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_encrypted_password_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.encrypted_password
    }

    fn mut_encrypted_password_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.encrypted_password
    }

    // repeated .CCLCMsg_SplitPlayerConnect splitplayers = 7;

    pub fn clear_splitplayers(&mut self) {
        self.splitplayers.clear();
    }

    // Param is passed by value, moved
    pub fn set_splitplayers(&mut self, v: ::protobuf::RepeatedField<super::netmessages::CCLCMsg_SplitPlayerConnect>) {
        self.splitplayers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_splitplayers(&mut self) -> &mut ::protobuf::RepeatedField<super::netmessages::CCLCMsg_SplitPlayerConnect> {
        &mut self.splitplayers
    }

    // Take field
    pub fn take_splitplayers(&mut self) -> ::protobuf::RepeatedField<super::netmessages::CCLCMsg_SplitPlayerConnect> {
        ::std::mem::replace(&mut self.splitplayers, ::protobuf::RepeatedField::new())
    }

    pub fn get_splitplayers(&self) -> &[super::netmessages::CCLCMsg_SplitPlayerConnect] {
        &self.splitplayers
    }

    fn get_splitplayers_for_reflect(&self) -> &::protobuf::RepeatedField<super::netmessages::CCLCMsg_SplitPlayerConnect> {
        &self.splitplayers
    }

    fn mut_splitplayers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::netmessages::CCLCMsg_SplitPlayerConnect> {
        &mut self.splitplayers
    }

    // optional bytes auth_steam = 8;

    pub fn clear_auth_steam(&mut self) {
        self.auth_steam.clear();
    }

    pub fn has_auth_steam(&self) -> bool {
        self.auth_steam.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_steam(&mut self, v: ::std::vec::Vec<u8>) {
        self.auth_steam = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auth_steam(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.auth_steam.is_none() {
            self.auth_steam.set_default();
        }
        self.auth_steam.as_mut().unwrap()
    }

    // Take field
    pub fn take_auth_steam(&mut self) -> ::std::vec::Vec<u8> {
        self.auth_steam.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_auth_steam(&self) -> &[u8] {
        match self.auth_steam.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_auth_steam_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.auth_steam
    }

    fn mut_auth_steam_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.auth_steam
    }

    // optional string challenge_context = 9;

    pub fn clear_challenge_context(&mut self) {
        self.challenge_context.clear();
    }

    pub fn has_challenge_context(&self) -> bool {
        self.challenge_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge_context(&mut self, v: ::std::string::String) {
        self.challenge_context = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_challenge_context(&mut self) -> &mut ::std::string::String {
        if self.challenge_context.is_none() {
            self.challenge_context.set_default();
        }
        self.challenge_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_challenge_context(&mut self) -> ::std::string::String {
        self.challenge_context.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_challenge_context(&self) -> &str {
        match self.challenge_context.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_challenge_context_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.challenge_context
    }

    fn mut_challenge_context_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.challenge_context
    }
}

impl ::protobuf::Message for C2S_CONNECT_Message {
    fn is_initialized(&self) -> bool {
        for v in &self.splitplayers {
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
                    self.host_version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.auth_protocol = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.challenge_number = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.reservation_cookie = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.low_violence = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.encrypted_password)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.splitplayers)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.auth_steam)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.challenge_context)?;
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
        if let Some(v) = self.host_version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.auth_protocol {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.challenge_number {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.reservation_cookie {
            my_size += 9;
        }
        if let Some(v) = self.low_violence {
            my_size += 2;
        }
        if let Some(ref v) = self.encrypted_password.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
        }
        for value in &self.splitplayers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.auth_steam.as_ref() {
            my_size += ::protobuf::rt::bytes_size(8, &v);
        }
        if let Some(ref v) = self.challenge_context.as_ref() {
            my_size += ::protobuf::rt::string_size(9, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.host_version {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.auth_protocol {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.challenge_number {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.reservation_cookie {
            os.write_fixed64(4, v)?;
        }
        if let Some(v) = self.low_violence {
            os.write_bool(5, v)?;
        }
        if let Some(ref v) = self.encrypted_password.as_ref() {
            os.write_bytes(6, &v)?;
        }
        for v in &self.splitplayers {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.auth_steam.as_ref() {
            os.write_bytes(8, &v)?;
        }
        if let Some(ref v) = self.challenge_context.as_ref() {
            os.write_string(9, &v)?;
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

impl ::protobuf::MessageStatic for C2S_CONNECT_Message {
    fn new() -> C2S_CONNECT_Message {
        C2S_CONNECT_Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<C2S_CONNECT_Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "host_version",
                    C2S_CONNECT_Message::get_host_version_for_reflect,
                    C2S_CONNECT_Message::mut_host_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "auth_protocol",
                    C2S_CONNECT_Message::get_auth_protocol_for_reflect,
                    C2S_CONNECT_Message::mut_auth_protocol_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "challenge_number",
                    C2S_CONNECT_Message::get_challenge_number_for_reflect,
                    C2S_CONNECT_Message::mut_challenge_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "reservation_cookie",
                    C2S_CONNECT_Message::get_reservation_cookie_for_reflect,
                    C2S_CONNECT_Message::mut_reservation_cookie_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "low_violence",
                    C2S_CONNECT_Message::get_low_violence_for_reflect,
                    C2S_CONNECT_Message::mut_low_violence_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "encrypted_password",
                    C2S_CONNECT_Message::get_encrypted_password_for_reflect,
                    C2S_CONNECT_Message::mut_encrypted_password_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::netmessages::CCLCMsg_SplitPlayerConnect>>(
                    "splitplayers",
                    C2S_CONNECT_Message::get_splitplayers_for_reflect,
                    C2S_CONNECT_Message::mut_splitplayers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "auth_steam",
                    C2S_CONNECT_Message::get_auth_steam_for_reflect,
                    C2S_CONNECT_Message::mut_auth_steam_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "challenge_context",
                    C2S_CONNECT_Message::get_challenge_context_for_reflect,
                    C2S_CONNECT_Message::mut_challenge_context_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<C2S_CONNECT_Message>(
                    "C2S_CONNECT_Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for C2S_CONNECT_Message {
    fn clear(&mut self) {
        self.clear_host_version();
        self.clear_auth_protocol();
        self.clear_challenge_number();
        self.clear_reservation_cookie();
        self.clear_low_violence();
        self.clear_encrypted_password();
        self.clear_splitplayers();
        self.clear_auth_steam();
        self.clear_challenge_context();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for C2S_CONNECT_Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for C2S_CONNECT_Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct C2S_CONNECTION_Message {
    // message fields
    addon_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for C2S_CONNECTION_Message {}

impl C2S_CONNECTION_Message {
    pub fn new() -> C2S_CONNECTION_Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static C2S_CONNECTION_Message {
        static mut instance: ::protobuf::lazy::Lazy<C2S_CONNECTION_Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const C2S_CONNECTION_Message,
        };
        unsafe {
            instance.get(C2S_CONNECTION_Message::new)
        }
    }

    // optional string addon_name = 1;

    pub fn clear_addon_name(&mut self) {
        self.addon_name.clear();
    }

    pub fn has_addon_name(&self) -> bool {
        self.addon_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_addon_name(&mut self, v: ::std::string::String) {
        self.addon_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_addon_name(&mut self) -> &mut ::std::string::String {
        if self.addon_name.is_none() {
            self.addon_name.set_default();
        }
        self.addon_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_addon_name(&mut self) -> ::std::string::String {
        self.addon_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_addon_name(&self) -> &str {
        match self.addon_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_addon_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.addon_name
    }

    fn mut_addon_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.addon_name
    }
}

impl ::protobuf::Message for C2S_CONNECTION_Message {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.addon_name)?;
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
        if let Some(ref v) = self.addon_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.addon_name.as_ref() {
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

impl ::protobuf::MessageStatic for C2S_CONNECTION_Message {
    fn new() -> C2S_CONNECTION_Message {
        C2S_CONNECTION_Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<C2S_CONNECTION_Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "addon_name",
                    C2S_CONNECTION_Message::get_addon_name_for_reflect,
                    C2S_CONNECTION_Message::mut_addon_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<C2S_CONNECTION_Message>(
                    "C2S_CONNECTION_Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for C2S_CONNECTION_Message {
    fn clear(&mut self) {
        self.clear_addon_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for C2S_CONNECTION_Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for C2S_CONNECTION_Message {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20connectionless_netmessages.proto\x1a\x11netmessages.proto\"\x96\
    \x03\n\x13C2S_CONNECT_Message\x12!\n\x0chost_version\x18\x01\x20\x01(\rR\
    \x0bhostVersion\x12#\n\rauth_protocol\x18\x02\x20\x01(\rR\x0cauthProtoco\
    l\x12)\n\x10challenge_number\x18\x03\x20\x01(\rR\x0fchallengeNumber\x12-\
    \n\x12reservation_cookie\x18\x04\x20\x01(\x06R\x11reservationCookie\x12!\
    \n\x0clow_violence\x18\x05\x20\x01(\x08R\x0blowViolence\x12-\n\x12encryp\
    ted_password\x18\x06\x20\x01(\x0cR\x11encryptedPassword\x12?\n\x0csplitp\
    layers\x18\x07\x20\x03(\x0b2\x1b.CCLCMsg_SplitPlayerConnectR\x0csplitpla\
    yers\x12\x1d\n\nauth_steam\x18\x08\x20\x01(\x0cR\tauthSteam\x12+\n\x11ch\
    allenge_context\x18\t\x20\x01(\tR\x10challengeContext\"7\n\x16C2S_CONNEC\
    TION_Message\x12\x1d\n\naddon_name\x18\x01\x20\x01(\tR\taddonNameB\x03\
    \x80\x01\0\
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
