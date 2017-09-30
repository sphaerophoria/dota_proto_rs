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
pub struct CP2P_TextMessage {
    // message fields
    text: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CP2P_TextMessage {}

impl CP2P_TextMessage {
    pub fn new() -> CP2P_TextMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CP2P_TextMessage {
        static mut instance: ::protobuf::lazy::Lazy<CP2P_TextMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CP2P_TextMessage,
        };
        unsafe {
            instance.get(CP2P_TextMessage::new)
        }
    }

    // optional bytes text = 1;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::vec::Vec<u8>) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.text.is_none() {
            self.text.set_default();
        }
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::vec::Vec<u8> {
        self.text.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_text(&self) -> &[u8] {
        match self.text.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_text_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.text
    }

    fn mut_text_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.text
    }
}

impl ::protobuf::Message for CP2P_TextMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.text)?;
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
        if let Some(ref v) = self.text.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.text.as_ref() {
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

impl ::protobuf::MessageStatic for CP2P_TextMessage {
    fn new() -> CP2P_TextMessage {
        CP2P_TextMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CP2P_TextMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "text",
                    CP2P_TextMessage::get_text_for_reflect,
                    CP2P_TextMessage::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CP2P_TextMessage>(
                    "CP2P_TextMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CP2P_TextMessage {
    fn clear(&mut self) {
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CP2P_TextMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CP2P_TextMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSteam_Voice_Encoding {
    // message fields
    voice_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSteam_Voice_Encoding {}

impl CSteam_Voice_Encoding {
    pub fn new() -> CSteam_Voice_Encoding {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSteam_Voice_Encoding {
        static mut instance: ::protobuf::lazy::Lazy<CSteam_Voice_Encoding> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSteam_Voice_Encoding,
        };
        unsafe {
            instance.get(CSteam_Voice_Encoding::new)
        }
    }

    // optional bytes voice_data = 1;

    pub fn clear_voice_data(&mut self) {
        self.voice_data.clear();
    }

    pub fn has_voice_data(&self) -> bool {
        self.voice_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_voice_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.voice_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_voice_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.voice_data.is_none() {
            self.voice_data.set_default();
        }
        self.voice_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_voice_data(&mut self) -> ::std::vec::Vec<u8> {
        self.voice_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_voice_data(&self) -> &[u8] {
        match self.voice_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_voice_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.voice_data
    }

    fn mut_voice_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.voice_data
    }
}

impl ::protobuf::Message for CSteam_Voice_Encoding {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.voice_data)?;
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
        if let Some(ref v) = self.voice_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.voice_data.as_ref() {
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

impl ::protobuf::MessageStatic for CSteam_Voice_Encoding {
    fn new() -> CSteam_Voice_Encoding {
        CSteam_Voice_Encoding::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSteam_Voice_Encoding>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "voice_data",
                    CSteam_Voice_Encoding::get_voice_data_for_reflect,
                    CSteam_Voice_Encoding::mut_voice_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSteam_Voice_Encoding>(
                    "CSteam_Voice_Encoding",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSteam_Voice_Encoding {
    fn clear(&mut self) {
        self.clear_voice_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSteam_Voice_Encoding {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSteam_Voice_Encoding {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CP2P_Voice {
    // message fields
    audio: ::protobuf::SingularPtrField<super::netmessages::CMsgVoiceAudio>,
    broadcast_group: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CP2P_Voice {}

impl CP2P_Voice {
    pub fn new() -> CP2P_Voice {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CP2P_Voice {
        static mut instance: ::protobuf::lazy::Lazy<CP2P_Voice> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CP2P_Voice,
        };
        unsafe {
            instance.get(CP2P_Voice::new)
        }
    }

    // optional .CMsgVoiceAudio audio = 1;

    pub fn clear_audio(&mut self) {
        self.audio.clear();
    }

    pub fn has_audio(&self) -> bool {
        self.audio.is_some()
    }

    // Param is passed by value, moved
    pub fn set_audio(&mut self, v: super::netmessages::CMsgVoiceAudio) {
        self.audio = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_audio(&mut self) -> &mut super::netmessages::CMsgVoiceAudio {
        if self.audio.is_none() {
            self.audio.set_default();
        }
        self.audio.as_mut().unwrap()
    }

    // Take field
    pub fn take_audio(&mut self) -> super::netmessages::CMsgVoiceAudio {
        self.audio.take().unwrap_or_else(|| super::netmessages::CMsgVoiceAudio::new())
    }

    pub fn get_audio(&self) -> &super::netmessages::CMsgVoiceAudio {
        self.audio.as_ref().unwrap_or_else(|| super::netmessages::CMsgVoiceAudio::default_instance())
    }

    fn get_audio_for_reflect(&self) -> &::protobuf::SingularPtrField<super::netmessages::CMsgVoiceAudio> {
        &self.audio
    }

    fn mut_audio_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::netmessages::CMsgVoiceAudio> {
        &mut self.audio
    }

    // optional uint32 broadcast_group = 2;

    pub fn clear_broadcast_group(&mut self) {
        self.broadcast_group = ::std::option::Option::None;
    }

    pub fn has_broadcast_group(&self) -> bool {
        self.broadcast_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_broadcast_group(&mut self, v: u32) {
        self.broadcast_group = ::std::option::Option::Some(v);
    }

    pub fn get_broadcast_group(&self) -> u32 {
        self.broadcast_group.unwrap_or(0)
    }

    fn get_broadcast_group_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.broadcast_group
    }

    fn mut_broadcast_group_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.broadcast_group
    }
}

impl ::protobuf::Message for CP2P_Voice {
    fn is_initialized(&self) -> bool {
        for v in &self.audio {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.audio)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.broadcast_group = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.audio.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.broadcast_group {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.audio.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.broadcast_group {
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

impl ::protobuf::MessageStatic for CP2P_Voice {
    fn new() -> CP2P_Voice {
        CP2P_Voice::new()
    }

    fn descriptor_static(_: ::std::option::Option<CP2P_Voice>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::netmessages::CMsgVoiceAudio>>(
                    "audio",
                    CP2P_Voice::get_audio_for_reflect,
                    CP2P_Voice::mut_audio_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "broadcast_group",
                    CP2P_Voice::get_broadcast_group_for_reflect,
                    CP2P_Voice::mut_broadcast_group_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CP2P_Voice>(
                    "CP2P_Voice",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CP2P_Voice {
    fn clear(&mut self) {
        self.clear_audio();
        self.clear_broadcast_group();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CP2P_Voice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CP2P_Voice {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CP2P_Voice_Handler_Flags {
    Played_Audio = 1,
}

impl ::protobuf::ProtobufEnum for CP2P_Voice_Handler_Flags {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CP2P_Voice_Handler_Flags> {
        match value {
            1 => ::std::option::Option::Some(CP2P_Voice_Handler_Flags::Played_Audio),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CP2P_Voice_Handler_Flags] = &[
            CP2P_Voice_Handler_Flags::Played_Audio,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CP2P_Voice_Handler_Flags>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CP2P_Voice_Handler_Flags", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CP2P_Voice_Handler_Flags {
}

impl ::protobuf::reflect::ProtobufValue for CP2P_Voice_Handler_Flags {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CP2P_Ping {
    // message fields
    send_time: ::std::option::Option<u64>,
    is_reply: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CP2P_Ping {}

impl CP2P_Ping {
    pub fn new() -> CP2P_Ping {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CP2P_Ping {
        static mut instance: ::protobuf::lazy::Lazy<CP2P_Ping> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CP2P_Ping,
        };
        unsafe {
            instance.get(CP2P_Ping::new)
        }
    }

    // required uint64 send_time = 1;

    pub fn clear_send_time(&mut self) {
        self.send_time = ::std::option::Option::None;
    }

    pub fn has_send_time(&self) -> bool {
        self.send_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_send_time(&mut self, v: u64) {
        self.send_time = ::std::option::Option::Some(v);
    }

    pub fn get_send_time(&self) -> u64 {
        self.send_time.unwrap_or(0)
    }

    fn get_send_time_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.send_time
    }

    fn mut_send_time_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.send_time
    }

    // required bool is_reply = 2;

    pub fn clear_is_reply(&mut self) {
        self.is_reply = ::std::option::Option::None;
    }

    pub fn has_is_reply(&self) -> bool {
        self.is_reply.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_reply(&mut self, v: bool) {
        self.is_reply = ::std::option::Option::Some(v);
    }

    pub fn get_is_reply(&self) -> bool {
        self.is_reply.unwrap_or(false)
    }

    fn get_is_reply_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_reply
    }

    fn mut_is_reply_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_reply
    }
}

impl ::protobuf::Message for CP2P_Ping {
    fn is_initialized(&self) -> bool {
        if self.send_time.is_none() {
            return false;
        }
        if self.is_reply.is_none() {
            return false;
        }
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
                    self.send_time = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_reply = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.send_time {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_reply {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.send_time {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.is_reply {
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

impl ::protobuf::MessageStatic for CP2P_Ping {
    fn new() -> CP2P_Ping {
        CP2P_Ping::new()
    }

    fn descriptor_static(_: ::std::option::Option<CP2P_Ping>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "send_time",
                    CP2P_Ping::get_send_time_for_reflect,
                    CP2P_Ping::mut_send_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_reply",
                    CP2P_Ping::get_is_reply_for_reflect,
                    CP2P_Ping::mut_is_reply_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CP2P_Ping>(
                    "CP2P_Ping",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CP2P_Ping {
    fn clear(&mut self) {
        self.clear_send_time();
        self.clear_is_reply();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CP2P_Ping {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CP2P_Ping {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CP2P_VRAvatarPosition {
    // message fields
    body_parts: ::protobuf::RepeatedField<CP2P_VRAvatarPosition_COrientation>,
    hat_id: ::std::option::Option<i32>,
    scene_id: ::std::option::Option<i32>,
    world_scale: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CP2P_VRAvatarPosition {}

impl CP2P_VRAvatarPosition {
    pub fn new() -> CP2P_VRAvatarPosition {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CP2P_VRAvatarPosition {
        static mut instance: ::protobuf::lazy::Lazy<CP2P_VRAvatarPosition> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CP2P_VRAvatarPosition,
        };
        unsafe {
            instance.get(CP2P_VRAvatarPosition::new)
        }
    }

    // repeated .CP2P_VRAvatarPosition.COrientation body_parts = 1;

    pub fn clear_body_parts(&mut self) {
        self.body_parts.clear();
    }

    // Param is passed by value, moved
    pub fn set_body_parts(&mut self, v: ::protobuf::RepeatedField<CP2P_VRAvatarPosition_COrientation>) {
        self.body_parts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_body_parts(&mut self) -> &mut ::protobuf::RepeatedField<CP2P_VRAvatarPosition_COrientation> {
        &mut self.body_parts
    }

    // Take field
    pub fn take_body_parts(&mut self) -> ::protobuf::RepeatedField<CP2P_VRAvatarPosition_COrientation> {
        ::std::mem::replace(&mut self.body_parts, ::protobuf::RepeatedField::new())
    }

    pub fn get_body_parts(&self) -> &[CP2P_VRAvatarPosition_COrientation] {
        &self.body_parts
    }

    fn get_body_parts_for_reflect(&self) -> &::protobuf::RepeatedField<CP2P_VRAvatarPosition_COrientation> {
        &self.body_parts
    }

    fn mut_body_parts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CP2P_VRAvatarPosition_COrientation> {
        &mut self.body_parts
    }

    // optional int32 hat_id = 2;

    pub fn clear_hat_id(&mut self) {
        self.hat_id = ::std::option::Option::None;
    }

    pub fn has_hat_id(&self) -> bool {
        self.hat_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hat_id(&mut self, v: i32) {
        self.hat_id = ::std::option::Option::Some(v);
    }

    pub fn get_hat_id(&self) -> i32 {
        self.hat_id.unwrap_or(0)
    }

    fn get_hat_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.hat_id
    }

    fn mut_hat_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.hat_id
    }

    // optional int32 scene_id = 3;

    pub fn clear_scene_id(&mut self) {
        self.scene_id = ::std::option::Option::None;
    }

    pub fn has_scene_id(&self) -> bool {
        self.scene_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scene_id(&mut self, v: i32) {
        self.scene_id = ::std::option::Option::Some(v);
    }

    pub fn get_scene_id(&self) -> i32 {
        self.scene_id.unwrap_or(0)
    }

    fn get_scene_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.scene_id
    }

    fn mut_scene_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.scene_id
    }

    // optional int32 world_scale = 4;

    pub fn clear_world_scale(&mut self) {
        self.world_scale = ::std::option::Option::None;
    }

    pub fn has_world_scale(&self) -> bool {
        self.world_scale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_world_scale(&mut self, v: i32) {
        self.world_scale = ::std::option::Option::Some(v);
    }

    pub fn get_world_scale(&self) -> i32 {
        self.world_scale.unwrap_or(0)
    }

    fn get_world_scale_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.world_scale
    }

    fn mut_world_scale_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.world_scale
    }
}

impl ::protobuf::Message for CP2P_VRAvatarPosition {
    fn is_initialized(&self) -> bool {
        for v in &self.body_parts {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.body_parts)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.hat_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.scene_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.world_scale = ::std::option::Option::Some(tmp);
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
        for value in &self.body_parts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.hat_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.scene_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.world_scale {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.body_parts {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.hat_id {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.scene_id {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.world_scale {
            os.write_int32(4, v)?;
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

impl ::protobuf::MessageStatic for CP2P_VRAvatarPosition {
    fn new() -> CP2P_VRAvatarPosition {
        CP2P_VRAvatarPosition::new()
    }

    fn descriptor_static(_: ::std::option::Option<CP2P_VRAvatarPosition>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CP2P_VRAvatarPosition_COrientation>>(
                    "body_parts",
                    CP2P_VRAvatarPosition::get_body_parts_for_reflect,
                    CP2P_VRAvatarPosition::mut_body_parts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "hat_id",
                    CP2P_VRAvatarPosition::get_hat_id_for_reflect,
                    CP2P_VRAvatarPosition::mut_hat_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "scene_id",
                    CP2P_VRAvatarPosition::get_scene_id_for_reflect,
                    CP2P_VRAvatarPosition::mut_scene_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "world_scale",
                    CP2P_VRAvatarPosition::get_world_scale_for_reflect,
                    CP2P_VRAvatarPosition::mut_world_scale_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CP2P_VRAvatarPosition>(
                    "CP2P_VRAvatarPosition",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CP2P_VRAvatarPosition {
    fn clear(&mut self) {
        self.clear_body_parts();
        self.clear_hat_id();
        self.clear_scene_id();
        self.clear_world_scale();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CP2P_VRAvatarPosition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CP2P_VRAvatarPosition {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CP2P_VRAvatarPosition_COrientation {
    // message fields
    pos: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    ang: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CP2P_VRAvatarPosition_COrientation {}

impl CP2P_VRAvatarPosition_COrientation {
    pub fn new() -> CP2P_VRAvatarPosition_COrientation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CP2P_VRAvatarPosition_COrientation {
        static mut instance: ::protobuf::lazy::Lazy<CP2P_VRAvatarPosition_COrientation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CP2P_VRAvatarPosition_COrientation,
        };
        unsafe {
            instance.get(CP2P_VRAvatarPosition_COrientation::new)
        }
    }

    // optional .CMsgVector pos = 1;

    pub fn clear_pos(&mut self) {
        self.pos.clear();
    }

    pub fn has_pos(&self) -> bool {
        self.pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pos(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.pos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pos(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.pos.is_none() {
            self.pos.set_default();
        }
        self.pos.as_mut().unwrap()
    }

    // Take field
    pub fn take_pos(&mut self) -> super::networkbasetypes::CMsgVector {
        self.pos.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_pos(&self) -> &super::networkbasetypes::CMsgVector {
        self.pos.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_pos_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.pos
    }

    fn mut_pos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.pos
    }

    // optional .CMsgQAngle ang = 2;

    pub fn clear_ang(&mut self) {
        self.ang.clear();
    }

    pub fn has_ang(&self) -> bool {
        self.ang.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ang(&mut self, v: super::networkbasetypes::CMsgQAngle) {
        self.ang = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ang(&mut self) -> &mut super::networkbasetypes::CMsgQAngle {
        if self.ang.is_none() {
            self.ang.set_default();
        }
        self.ang.as_mut().unwrap()
    }

    // Take field
    pub fn take_ang(&mut self) -> super::networkbasetypes::CMsgQAngle {
        self.ang.take().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::new())
    }

    pub fn get_ang(&self) -> &super::networkbasetypes::CMsgQAngle {
        self.ang.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::default_instance())
    }

    fn get_ang_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &self.ang
    }

    fn mut_ang_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &mut self.ang
    }
}

impl ::protobuf::Message for CP2P_VRAvatarPosition_COrientation {
    fn is_initialized(&self) -> bool {
        for v in &self.pos {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.ang {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pos)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ang)?;
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
        if let Some(ref v) = self.pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.ang.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.pos.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.ang.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CP2P_VRAvatarPosition_COrientation {
    fn new() -> CP2P_VRAvatarPosition_COrientation {
        CP2P_VRAvatarPosition_COrientation::new()
    }

    fn descriptor_static(_: ::std::option::Option<CP2P_VRAvatarPosition_COrientation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "pos",
                    CP2P_VRAvatarPosition_COrientation::get_pos_for_reflect,
                    CP2P_VRAvatarPosition_COrientation::mut_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgQAngle>>(
                    "ang",
                    CP2P_VRAvatarPosition_COrientation::get_ang_for_reflect,
                    CP2P_VRAvatarPosition_COrientation::mut_ang_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CP2P_VRAvatarPosition_COrientation>(
                    "CP2P_VRAvatarPosition_COrientation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CP2P_VRAvatarPosition_COrientation {
    fn clear(&mut self) {
        self.clear_pos();
        self.clear_ang();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CP2P_VRAvatarPosition_COrientation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CP2P_VRAvatarPosition_COrientation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CP2P_WatchSynchronization {
    // message fields
    demo_tick: ::std::option::Option<i32>,
    paused: ::std::option::Option<bool>,
    tv_listen_voice_indices: ::std::option::Option<i32>,
    dota_spectator_mode: ::std::option::Option<i32>,
    dota_spectator_watching_broadcaster: ::std::option::Option<i32>,
    dota_spectator_hero_index: ::std::option::Option<i32>,
    dota_spectator_autospeed: ::std::option::Option<i32>,
    dota_replay_speed: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CP2P_WatchSynchronization {}

impl CP2P_WatchSynchronization {
    pub fn new() -> CP2P_WatchSynchronization {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CP2P_WatchSynchronization {
        static mut instance: ::protobuf::lazy::Lazy<CP2P_WatchSynchronization> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CP2P_WatchSynchronization,
        };
        unsafe {
            instance.get(CP2P_WatchSynchronization::new)
        }
    }

    // optional int32 demo_tick = 1;

    pub fn clear_demo_tick(&mut self) {
        self.demo_tick = ::std::option::Option::None;
    }

    pub fn has_demo_tick(&self) -> bool {
        self.demo_tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_demo_tick(&mut self, v: i32) {
        self.demo_tick = ::std::option::Option::Some(v);
    }

    pub fn get_demo_tick(&self) -> i32 {
        self.demo_tick.unwrap_or(0)
    }

    fn get_demo_tick_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.demo_tick
    }

    fn mut_demo_tick_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.demo_tick
    }

    // optional bool paused = 2;

    pub fn clear_paused(&mut self) {
        self.paused = ::std::option::Option::None;
    }

    pub fn has_paused(&self) -> bool {
        self.paused.is_some()
    }

    // Param is passed by value, moved
    pub fn set_paused(&mut self, v: bool) {
        self.paused = ::std::option::Option::Some(v);
    }

    pub fn get_paused(&self) -> bool {
        self.paused.unwrap_or(false)
    }

    fn get_paused_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.paused
    }

    fn mut_paused_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.paused
    }

    // optional int32 tv_listen_voice_indices = 3;

    pub fn clear_tv_listen_voice_indices(&mut self) {
        self.tv_listen_voice_indices = ::std::option::Option::None;
    }

    pub fn has_tv_listen_voice_indices(&self) -> bool {
        self.tv_listen_voice_indices.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tv_listen_voice_indices(&mut self, v: i32) {
        self.tv_listen_voice_indices = ::std::option::Option::Some(v);
    }

    pub fn get_tv_listen_voice_indices(&self) -> i32 {
        self.tv_listen_voice_indices.unwrap_or(0)
    }

    fn get_tv_listen_voice_indices_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.tv_listen_voice_indices
    }

    fn mut_tv_listen_voice_indices_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.tv_listen_voice_indices
    }

    // optional int32 dota_spectator_mode = 4;

    pub fn clear_dota_spectator_mode(&mut self) {
        self.dota_spectator_mode = ::std::option::Option::None;
    }

    pub fn has_dota_spectator_mode(&self) -> bool {
        self.dota_spectator_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota_spectator_mode(&mut self, v: i32) {
        self.dota_spectator_mode = ::std::option::Option::Some(v);
    }

    pub fn get_dota_spectator_mode(&self) -> i32 {
        self.dota_spectator_mode.unwrap_or(0)
    }

    fn get_dota_spectator_mode_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dota_spectator_mode
    }

    fn mut_dota_spectator_mode_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dota_spectator_mode
    }

    // optional int32 dota_spectator_watching_broadcaster = 5;

    pub fn clear_dota_spectator_watching_broadcaster(&mut self) {
        self.dota_spectator_watching_broadcaster = ::std::option::Option::None;
    }

    pub fn has_dota_spectator_watching_broadcaster(&self) -> bool {
        self.dota_spectator_watching_broadcaster.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota_spectator_watching_broadcaster(&mut self, v: i32) {
        self.dota_spectator_watching_broadcaster = ::std::option::Option::Some(v);
    }

    pub fn get_dota_spectator_watching_broadcaster(&self) -> i32 {
        self.dota_spectator_watching_broadcaster.unwrap_or(0)
    }

    fn get_dota_spectator_watching_broadcaster_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dota_spectator_watching_broadcaster
    }

    fn mut_dota_spectator_watching_broadcaster_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dota_spectator_watching_broadcaster
    }

    // optional int32 dota_spectator_hero_index = 6;

    pub fn clear_dota_spectator_hero_index(&mut self) {
        self.dota_spectator_hero_index = ::std::option::Option::None;
    }

    pub fn has_dota_spectator_hero_index(&self) -> bool {
        self.dota_spectator_hero_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota_spectator_hero_index(&mut self, v: i32) {
        self.dota_spectator_hero_index = ::std::option::Option::Some(v);
    }

    pub fn get_dota_spectator_hero_index(&self) -> i32 {
        self.dota_spectator_hero_index.unwrap_or(0)
    }

    fn get_dota_spectator_hero_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dota_spectator_hero_index
    }

    fn mut_dota_spectator_hero_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dota_spectator_hero_index
    }

    // optional int32 dota_spectator_autospeed = 7;

    pub fn clear_dota_spectator_autospeed(&mut self) {
        self.dota_spectator_autospeed = ::std::option::Option::None;
    }

    pub fn has_dota_spectator_autospeed(&self) -> bool {
        self.dota_spectator_autospeed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota_spectator_autospeed(&mut self, v: i32) {
        self.dota_spectator_autospeed = ::std::option::Option::Some(v);
    }

    pub fn get_dota_spectator_autospeed(&self) -> i32 {
        self.dota_spectator_autospeed.unwrap_or(0)
    }

    fn get_dota_spectator_autospeed_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dota_spectator_autospeed
    }

    fn mut_dota_spectator_autospeed_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dota_spectator_autospeed
    }

    // optional int32 dota_replay_speed = 8;

    pub fn clear_dota_replay_speed(&mut self) {
        self.dota_replay_speed = ::std::option::Option::None;
    }

    pub fn has_dota_replay_speed(&self) -> bool {
        self.dota_replay_speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota_replay_speed(&mut self, v: i32) {
        self.dota_replay_speed = ::std::option::Option::Some(v);
    }

    pub fn get_dota_replay_speed(&self) -> i32 {
        self.dota_replay_speed.unwrap_or(0)
    }

    fn get_dota_replay_speed_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dota_replay_speed
    }

    fn mut_dota_replay_speed_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dota_replay_speed
    }
}

impl ::protobuf::Message for CP2P_WatchSynchronization {
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
                    self.demo_tick = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.paused = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.tv_listen_voice_indices = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.dota_spectator_mode = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.dota_spectator_watching_broadcaster = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.dota_spectator_hero_index = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.dota_spectator_autospeed = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.dota_replay_speed = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.demo_tick {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.paused {
            my_size += 2;
        }
        if let Some(v) = self.tv_listen_voice_indices {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.dota_spectator_mode {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.dota_spectator_watching_broadcaster {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.dota_spectator_hero_index {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.dota_spectator_autospeed {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.dota_replay_speed {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.demo_tick {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.paused {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.tv_listen_voice_indices {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.dota_spectator_mode {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.dota_spectator_watching_broadcaster {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.dota_spectator_hero_index {
            os.write_int32(6, v)?;
        }
        if let Some(v) = self.dota_spectator_autospeed {
            os.write_int32(7, v)?;
        }
        if let Some(v) = self.dota_replay_speed {
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

impl ::protobuf::MessageStatic for CP2P_WatchSynchronization {
    fn new() -> CP2P_WatchSynchronization {
        CP2P_WatchSynchronization::new()
    }

    fn descriptor_static(_: ::std::option::Option<CP2P_WatchSynchronization>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "demo_tick",
                    CP2P_WatchSynchronization::get_demo_tick_for_reflect,
                    CP2P_WatchSynchronization::mut_demo_tick_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "paused",
                    CP2P_WatchSynchronization::get_paused_for_reflect,
                    CP2P_WatchSynchronization::mut_paused_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "tv_listen_voice_indices",
                    CP2P_WatchSynchronization::get_tv_listen_voice_indices_for_reflect,
                    CP2P_WatchSynchronization::mut_tv_listen_voice_indices_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dota_spectator_mode",
                    CP2P_WatchSynchronization::get_dota_spectator_mode_for_reflect,
                    CP2P_WatchSynchronization::mut_dota_spectator_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dota_spectator_watching_broadcaster",
                    CP2P_WatchSynchronization::get_dota_spectator_watching_broadcaster_for_reflect,
                    CP2P_WatchSynchronization::mut_dota_spectator_watching_broadcaster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dota_spectator_hero_index",
                    CP2P_WatchSynchronization::get_dota_spectator_hero_index_for_reflect,
                    CP2P_WatchSynchronization::mut_dota_spectator_hero_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dota_spectator_autospeed",
                    CP2P_WatchSynchronization::get_dota_spectator_autospeed_for_reflect,
                    CP2P_WatchSynchronization::mut_dota_spectator_autospeed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dota_replay_speed",
                    CP2P_WatchSynchronization::get_dota_replay_speed_for_reflect,
                    CP2P_WatchSynchronization::mut_dota_replay_speed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CP2P_WatchSynchronization>(
                    "CP2P_WatchSynchronization",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CP2P_WatchSynchronization {
    fn clear(&mut self) {
        self.clear_demo_tick();
        self.clear_paused();
        self.clear_tv_listen_voice_indices();
        self.clear_dota_spectator_mode();
        self.clear_dota_spectator_watching_broadcaster();
        self.clear_dota_spectator_hero_index();
        self.clear_dota_spectator_autospeed();
        self.clear_dota_replay_speed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CP2P_WatchSynchronization {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CP2P_WatchSynchronization {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum P2P_Messages {
    p2p_TextMessage = 256,
    p2p_Voice = 257,
    p2p_Ping = 258,
    p2p_VRAvatarPosition = 259,
    p2p_WatchSynchronization = 260,
}

impl ::protobuf::ProtobufEnum for P2P_Messages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<P2P_Messages> {
        match value {
            256 => ::std::option::Option::Some(P2P_Messages::p2p_TextMessage),
            257 => ::std::option::Option::Some(P2P_Messages::p2p_Voice),
            258 => ::std::option::Option::Some(P2P_Messages::p2p_Ping),
            259 => ::std::option::Option::Some(P2P_Messages::p2p_VRAvatarPosition),
            260 => ::std::option::Option::Some(P2P_Messages::p2p_WatchSynchronization),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [P2P_Messages] = &[
            P2P_Messages::p2p_TextMessage,
            P2P_Messages::p2p_Voice,
            P2P_Messages::p2p_Ping,
            P2P_Messages::p2p_VRAvatarPosition,
            P2P_Messages::p2p_WatchSynchronization,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<P2P_Messages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("P2P_Messages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for P2P_Messages {
}

impl ::protobuf::reflect::ProtobufValue for P2P_Messages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dc_peer2peer_netmessages.proto\x1a\x11netmessages.proto\x1a\x16netw\
    orkbasetypes.proto\"&\n\x10CP2P_TextMessage\x12\x12\n\x04text\x18\x01\
    \x20\x01(\x0cR\x04text\"6\n\x15CSteam_Voice_Encoding\x12\x1d\n\nvoice_da\
    ta\x18\x01\x20\x01(\x0cR\tvoiceData\"\x7f\n\nCP2P_Voice\x12%\n\x05audio\
    \x18\x01\x20\x01(\x0b2\x0f.CMsgVoiceAudioR\x05audio\x12'\n\x0fbroadcast_\
    group\x18\x02\x20\x01(\rR\x0ebroadcastGroup\"!\n\rHandler_Flags\x12\x10\
    \n\x0cPlayed_Audio\x10\x01\"C\n\tCP2P_Ping\x12\x1b\n\tsend_time\x18\x01\
    \x20\x02(\x04R\x08sendTime\x12\x19\n\x08is_reply\x18\x02\x20\x02(\x08R\
    \x07isReply\"\xfc\x01\n\x15CP2P_VRAvatarPosition\x12B\n\nbody_parts\x18\
    \x01\x20\x03(\x0b2#.CP2P_VRAvatarPosition.COrientationR\tbodyParts\x12\
    \x15\n\x06hat_id\x18\x02\x20\x01(\x05R\x05hatId\x12\x19\n\x08scene_id\
    \x18\x03\x20\x01(\x05R\x07sceneId\x12\x1f\n\x0bworld_scale\x18\x04\x20\
    \x01(\x05R\nworldScale\x1aL\n\x0cCOrientation\x12\x1d\n\x03pos\x18\x01\
    \x20\x01(\x0b2\x0b.CMsgVectorR\x03pos\x12\x1d\n\x03ang\x18\x02\x20\x01(\
    \x0b2\x0b.CMsgQAngleR\x03ang\"\xa7\x03\n\x19CP2P_WatchSynchronization\
    \x12\x1b\n\tdemo_tick\x18\x01\x20\x01(\x05R\x08demoTick\x12\x16\n\x06pau\
    sed\x18\x02\x20\x01(\x08R\x06paused\x125\n\x17tv_listen_voice_indices\
    \x18\x03\x20\x01(\x05R\x14tvListenVoiceIndices\x12.\n\x13dota_spectator_\
    mode\x18\x04\x20\x01(\x05R\x11dotaSpectatorMode\x12M\n#dota_spectator_wa\
    tching_broadcaster\x18\x05\x20\x01(\x05R\x20dotaSpectatorWatchingBroadca\
    ster\x129\n\x19dota_spectator_hero_index\x18\x06\x20\x01(\x05R\x16dotaSp\
    ectatorHeroIndex\x128\n\x18dota_spectator_autospeed\x18\x07\x20\x01(\x05\
    R\x16dotaSpectatorAutospeed\x12*\n\x11dota_replay_speed\x18\x08\x20\x01(\
    \x05R\x0fdotaReplaySpeed*}\n\x0cP2P_Messages\x12\x14\n\x0fp2p_TextMessag\
    e\x10\x80\x02\x12\x0e\n\tp2p_Voice\x10\x81\x02\x12\r\n\x08p2p_Ping\x10\
    \x82\x02\x12\x19\n\x14p2p_VRAvatarPosition\x10\x83\x02\x12\x1d\n\x18p2p_\
    WatchSynchronization\x10\x84\x02B\x03\x80\x01\0\
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
