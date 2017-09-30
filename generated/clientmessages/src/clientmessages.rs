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
pub struct CClientMsg_CustomGameEvent {
    // message fields
    event_name: ::protobuf::SingularField<::std::string::String>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CClientMsg_CustomGameEvent {}

impl CClientMsg_CustomGameEvent {
    pub fn new() -> CClientMsg_CustomGameEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CClientMsg_CustomGameEvent {
        static mut instance: ::protobuf::lazy::Lazy<CClientMsg_CustomGameEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CClientMsg_CustomGameEvent,
        };
        unsafe {
            instance.get(CClientMsg_CustomGameEvent::new)
        }
    }

    // optional string event_name = 1;

    pub fn clear_event_name(&mut self) {
        self.event_name.clear();
    }

    pub fn has_event_name(&self) -> bool {
        self.event_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_name(&mut self, v: ::std::string::String) {
        self.event_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_name(&mut self) -> &mut ::std::string::String {
        if self.event_name.is_none() {
            self.event_name.set_default();
        }
        self.event_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_name(&mut self) -> ::std::string::String {
        self.event_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_event_name(&self) -> &str {
        match self.event_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_event_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.event_name
    }

    fn mut_event_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.event_name
    }

    // optional bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }
}

impl ::protobuf::Message for CClientMsg_CustomGameEvent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.event_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
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
        if let Some(ref v) = self.event_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.event_name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.data.as_ref() {
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

impl ::protobuf::MessageStatic for CClientMsg_CustomGameEvent {
    fn new() -> CClientMsg_CustomGameEvent {
        CClientMsg_CustomGameEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CClientMsg_CustomGameEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "event_name",
                    CClientMsg_CustomGameEvent::get_event_name_for_reflect,
                    CClientMsg_CustomGameEvent::mut_event_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CClientMsg_CustomGameEvent::get_data_for_reflect,
                    CClientMsg_CustomGameEvent::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CClientMsg_CustomGameEvent>(
                    "CClientMsg_CustomGameEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CClientMsg_CustomGameEvent {
    fn clear(&mut self) {
        self.clear_event_name();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CClientMsg_CustomGameEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CClientMsg_CustomGameEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CClientMsg_CustomGameEventBounce {
    // message fields
    event_name: ::protobuf::SingularField<::std::string::String>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    player_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CClientMsg_CustomGameEventBounce {}

impl CClientMsg_CustomGameEventBounce {
    pub fn new() -> CClientMsg_CustomGameEventBounce {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CClientMsg_CustomGameEventBounce {
        static mut instance: ::protobuf::lazy::Lazy<CClientMsg_CustomGameEventBounce> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CClientMsg_CustomGameEventBounce,
        };
        unsafe {
            instance.get(CClientMsg_CustomGameEventBounce::new)
        }
    }

    // optional string event_name = 1;

    pub fn clear_event_name(&mut self) {
        self.event_name.clear();
    }

    pub fn has_event_name(&self) -> bool {
        self.event_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_name(&mut self, v: ::std::string::String) {
        self.event_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_name(&mut self) -> &mut ::std::string::String {
        if self.event_name.is_none() {
            self.event_name.set_default();
        }
        self.event_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_name(&mut self) -> ::std::string::String {
        self.event_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_event_name(&self) -> &str {
        match self.event_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_event_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.event_name
    }

    fn mut_event_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.event_name
    }

    // optional bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }

    // optional int32 player_index = 3;

    pub fn clear_player_index(&mut self) {
        self.player_index = ::std::option::Option::None;
    }

    pub fn has_player_index(&self) -> bool {
        self.player_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_index(&mut self, v: i32) {
        self.player_index = ::std::option::Option::Some(v);
    }

    pub fn get_player_index(&self) -> i32 {
        self.player_index.unwrap_or(0)
    }

    fn get_player_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.player_index
    }

    fn mut_player_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.player_index
    }
}

impl ::protobuf::Message for CClientMsg_CustomGameEventBounce {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.event_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.player_index = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.event_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.player_index {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.event_name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(v) = self.player_index {
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

impl ::protobuf::MessageStatic for CClientMsg_CustomGameEventBounce {
    fn new() -> CClientMsg_CustomGameEventBounce {
        CClientMsg_CustomGameEventBounce::new()
    }

    fn descriptor_static(_: ::std::option::Option<CClientMsg_CustomGameEventBounce>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "event_name",
                    CClientMsg_CustomGameEventBounce::get_event_name_for_reflect,
                    CClientMsg_CustomGameEventBounce::mut_event_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CClientMsg_CustomGameEventBounce::get_data_for_reflect,
                    CClientMsg_CustomGameEventBounce::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "player_index",
                    CClientMsg_CustomGameEventBounce::get_player_index_for_reflect,
                    CClientMsg_CustomGameEventBounce::mut_player_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CClientMsg_CustomGameEventBounce>(
                    "CClientMsg_CustomGameEventBounce",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CClientMsg_CustomGameEventBounce {
    fn clear(&mut self) {
        self.clear_event_name();
        self.clear_data();
        self.clear_player_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CClientMsg_CustomGameEventBounce {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CClientMsg_CustomGameEventBounce {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CClientMsg_ClientUIEvent {
    // message fields
    event: ::std::option::Option<EClientUIEvent>,
    ent_ehandle: ::std::option::Option<u32>,
    client_ehandle: ::std::option::Option<u32>,
    data1: ::protobuf::SingularField<::std::string::String>,
    data2: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CClientMsg_ClientUIEvent {}

impl CClientMsg_ClientUIEvent {
    pub fn new() -> CClientMsg_ClientUIEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CClientMsg_ClientUIEvent {
        static mut instance: ::protobuf::lazy::Lazy<CClientMsg_ClientUIEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CClientMsg_ClientUIEvent,
        };
        unsafe {
            instance.get(CClientMsg_ClientUIEvent::new)
        }
    }

    // optional .EClientUIEvent event = 1;

    pub fn clear_event(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_event(&self) -> bool {
        self.event.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event(&mut self, v: EClientUIEvent) {
        self.event = ::std::option::Option::Some(v);
    }

    pub fn get_event(&self) -> EClientUIEvent {
        self.event.unwrap_or(EClientUIEvent::EClientUIEvent_Invalid)
    }

    fn get_event_for_reflect(&self) -> &::std::option::Option<EClientUIEvent> {
        &self.event
    }

    fn mut_event_for_reflect(&mut self) -> &mut ::std::option::Option<EClientUIEvent> {
        &mut self.event
    }

    // optional uint32 ent_ehandle = 2;

    pub fn clear_ent_ehandle(&mut self) {
        self.ent_ehandle = ::std::option::Option::None;
    }

    pub fn has_ent_ehandle(&self) -> bool {
        self.ent_ehandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ent_ehandle(&mut self, v: u32) {
        self.ent_ehandle = ::std::option::Option::Some(v);
    }

    pub fn get_ent_ehandle(&self) -> u32 {
        self.ent_ehandle.unwrap_or(0)
    }

    fn get_ent_ehandle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ent_ehandle
    }

    fn mut_ent_ehandle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ent_ehandle
    }

    // optional uint32 client_ehandle = 3;

    pub fn clear_client_ehandle(&mut self) {
        self.client_ehandle = ::std::option::Option::None;
    }

    pub fn has_client_ehandle(&self) -> bool {
        self.client_ehandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_ehandle(&mut self, v: u32) {
        self.client_ehandle = ::std::option::Option::Some(v);
    }

    pub fn get_client_ehandle(&self) -> u32 {
        self.client_ehandle.unwrap_or(0)
    }

    fn get_client_ehandle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_ehandle
    }

    fn mut_client_ehandle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_ehandle
    }

    // optional string data1 = 4;

    pub fn clear_data1(&mut self) {
        self.data1.clear();
    }

    pub fn has_data1(&self) -> bool {
        self.data1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data1(&mut self, v: ::std::string::String) {
        self.data1 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data1(&mut self) -> &mut ::std::string::String {
        if self.data1.is_none() {
            self.data1.set_default();
        }
        self.data1.as_mut().unwrap()
    }

    // Take field
    pub fn take_data1(&mut self) -> ::std::string::String {
        self.data1.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_data1(&self) -> &str {
        match self.data1.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_data1_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.data1
    }

    fn mut_data1_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.data1
    }

    // optional string data2 = 5;

    pub fn clear_data2(&mut self) {
        self.data2.clear();
    }

    pub fn has_data2(&self) -> bool {
        self.data2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data2(&mut self, v: ::std::string::String) {
        self.data2 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data2(&mut self) -> &mut ::std::string::String {
        if self.data2.is_none() {
            self.data2.set_default();
        }
        self.data2.as_mut().unwrap()
    }

    // Take field
    pub fn take_data2(&mut self) -> ::std::string::String {
        self.data2.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_data2(&self) -> &str {
        match self.data2.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_data2_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.data2
    }

    fn mut_data2_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.data2
    }
}

impl ::protobuf::Message for CClientMsg_ClientUIEvent {
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
                    self.event = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ent_ehandle = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.client_ehandle = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.data1)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.data2)?;
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
        if let Some(v) = self.event {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.ent_ehandle {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.client_ehandle {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.data1.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.data2.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.event {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.ent_ehandle {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.client_ehandle {
            os.write_uint32(3, v)?;
        }
        if let Some(ref v) = self.data1.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.data2.as_ref() {
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

impl ::protobuf::MessageStatic for CClientMsg_ClientUIEvent {
    fn new() -> CClientMsg_ClientUIEvent {
        CClientMsg_ClientUIEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CClientMsg_ClientUIEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EClientUIEvent>>(
                    "event",
                    CClientMsg_ClientUIEvent::get_event_for_reflect,
                    CClientMsg_ClientUIEvent::mut_event_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ent_ehandle",
                    CClientMsg_ClientUIEvent::get_ent_ehandle_for_reflect,
                    CClientMsg_ClientUIEvent::mut_ent_ehandle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_ehandle",
                    CClientMsg_ClientUIEvent::get_client_ehandle_for_reflect,
                    CClientMsg_ClientUIEvent::mut_client_ehandle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "data1",
                    CClientMsg_ClientUIEvent::get_data1_for_reflect,
                    CClientMsg_ClientUIEvent::mut_data1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "data2",
                    CClientMsg_ClientUIEvent::get_data2_for_reflect,
                    CClientMsg_ClientUIEvent::mut_data2_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CClientMsg_ClientUIEvent>(
                    "CClientMsg_ClientUIEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CClientMsg_ClientUIEvent {
    fn clear(&mut self) {
        self.clear_event();
        self.clear_ent_ehandle();
        self.clear_client_ehandle();
        self.clear_data1();
        self.clear_data2();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CClientMsg_ClientUIEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CClientMsg_ClientUIEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CClientMsg_DevPaletteVisibilityChangedEvent {
    // message fields
    visible: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CClientMsg_DevPaletteVisibilityChangedEvent {}

impl CClientMsg_DevPaletteVisibilityChangedEvent {
    pub fn new() -> CClientMsg_DevPaletteVisibilityChangedEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CClientMsg_DevPaletteVisibilityChangedEvent {
        static mut instance: ::protobuf::lazy::Lazy<CClientMsg_DevPaletteVisibilityChangedEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CClientMsg_DevPaletteVisibilityChangedEvent,
        };
        unsafe {
            instance.get(CClientMsg_DevPaletteVisibilityChangedEvent::new)
        }
    }

    // optional bool visible = 1;

    pub fn clear_visible(&mut self) {
        self.visible = ::std::option::Option::None;
    }

    pub fn has_visible(&self) -> bool {
        self.visible.is_some()
    }

    // Param is passed by value, moved
    pub fn set_visible(&mut self, v: bool) {
        self.visible = ::std::option::Option::Some(v);
    }

    pub fn get_visible(&self) -> bool {
        self.visible.unwrap_or(false)
    }

    fn get_visible_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.visible
    }

    fn mut_visible_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.visible
    }
}

impl ::protobuf::Message for CClientMsg_DevPaletteVisibilityChangedEvent {
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
                    self.visible = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.visible {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.visible {
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

impl ::protobuf::MessageStatic for CClientMsg_DevPaletteVisibilityChangedEvent {
    fn new() -> CClientMsg_DevPaletteVisibilityChangedEvent {
        CClientMsg_DevPaletteVisibilityChangedEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CClientMsg_DevPaletteVisibilityChangedEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "visible",
                    CClientMsg_DevPaletteVisibilityChangedEvent::get_visible_for_reflect,
                    CClientMsg_DevPaletteVisibilityChangedEvent::mut_visible_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CClientMsg_DevPaletteVisibilityChangedEvent>(
                    "CClientMsg_DevPaletteVisibilityChangedEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CClientMsg_DevPaletteVisibilityChangedEvent {
    fn clear(&mut self) {
        self.clear_visible();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CClientMsg_DevPaletteVisibilityChangedEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CClientMsg_DevPaletteVisibilityChangedEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CClientMsg_WorldUIControllerHasPanelChangedEvent {
    // message fields
    has_panel: ::std::option::Option<bool>,
    client_ehandle: ::std::option::Option<u32>,
    literal_hand_type: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CClientMsg_WorldUIControllerHasPanelChangedEvent {}

impl CClientMsg_WorldUIControllerHasPanelChangedEvent {
    pub fn new() -> CClientMsg_WorldUIControllerHasPanelChangedEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CClientMsg_WorldUIControllerHasPanelChangedEvent {
        static mut instance: ::protobuf::lazy::Lazy<CClientMsg_WorldUIControllerHasPanelChangedEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CClientMsg_WorldUIControllerHasPanelChangedEvent,
        };
        unsafe {
            instance.get(CClientMsg_WorldUIControllerHasPanelChangedEvent::new)
        }
    }

    // optional bool has_panel = 1;

    pub fn clear_has_panel(&mut self) {
        self.has_panel = ::std::option::Option::None;
    }

    pub fn has_has_panel(&self) -> bool {
        self.has_panel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_has_panel(&mut self, v: bool) {
        self.has_panel = ::std::option::Option::Some(v);
    }

    pub fn get_has_panel(&self) -> bool {
        self.has_panel.unwrap_or(false)
    }

    fn get_has_panel_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.has_panel
    }

    fn mut_has_panel_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.has_panel
    }

    // optional uint32 client_ehandle = 2;

    pub fn clear_client_ehandle(&mut self) {
        self.client_ehandle = ::std::option::Option::None;
    }

    pub fn has_client_ehandle(&self) -> bool {
        self.client_ehandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_ehandle(&mut self, v: u32) {
        self.client_ehandle = ::std::option::Option::Some(v);
    }

    pub fn get_client_ehandle(&self) -> u32 {
        self.client_ehandle.unwrap_or(0)
    }

    fn get_client_ehandle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_ehandle
    }

    fn mut_client_ehandle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_ehandle
    }

    // optional uint32 literal_hand_type = 3;

    pub fn clear_literal_hand_type(&mut self) {
        self.literal_hand_type = ::std::option::Option::None;
    }

    pub fn has_literal_hand_type(&self) -> bool {
        self.literal_hand_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_literal_hand_type(&mut self, v: u32) {
        self.literal_hand_type = ::std::option::Option::Some(v);
    }

    pub fn get_literal_hand_type(&self) -> u32 {
        self.literal_hand_type.unwrap_or(0)
    }

    fn get_literal_hand_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.literal_hand_type
    }

    fn mut_literal_hand_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.literal_hand_type
    }
}

impl ::protobuf::Message for CClientMsg_WorldUIControllerHasPanelChangedEvent {
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
                    self.has_panel = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.client_ehandle = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.literal_hand_type = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.has_panel {
            my_size += 2;
        }
        if let Some(v) = self.client_ehandle {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.literal_hand_type {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.has_panel {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.client_ehandle {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.literal_hand_type {
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

impl ::protobuf::MessageStatic for CClientMsg_WorldUIControllerHasPanelChangedEvent {
    fn new() -> CClientMsg_WorldUIControllerHasPanelChangedEvent {
        CClientMsg_WorldUIControllerHasPanelChangedEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CClientMsg_WorldUIControllerHasPanelChangedEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_panel",
                    CClientMsg_WorldUIControllerHasPanelChangedEvent::get_has_panel_for_reflect,
                    CClientMsg_WorldUIControllerHasPanelChangedEvent::mut_has_panel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_ehandle",
                    CClientMsg_WorldUIControllerHasPanelChangedEvent::get_client_ehandle_for_reflect,
                    CClientMsg_WorldUIControllerHasPanelChangedEvent::mut_client_ehandle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "literal_hand_type",
                    CClientMsg_WorldUIControllerHasPanelChangedEvent::get_literal_hand_type_for_reflect,
                    CClientMsg_WorldUIControllerHasPanelChangedEvent::mut_literal_hand_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CClientMsg_WorldUIControllerHasPanelChangedEvent>(
                    "CClientMsg_WorldUIControllerHasPanelChangedEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CClientMsg_WorldUIControllerHasPanelChangedEvent {
    fn clear(&mut self) {
        self.clear_has_panel();
        self.clear_client_ehandle();
        self.clear_literal_hand_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CClientMsg_WorldUIControllerHasPanelChangedEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CClientMsg_WorldUIControllerHasPanelChangedEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CClientMsg_RotateAnchor {
    // message fields
    angle: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CClientMsg_RotateAnchor {}

impl CClientMsg_RotateAnchor {
    pub fn new() -> CClientMsg_RotateAnchor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CClientMsg_RotateAnchor {
        static mut instance: ::protobuf::lazy::Lazy<CClientMsg_RotateAnchor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CClientMsg_RotateAnchor,
        };
        unsafe {
            instance.get(CClientMsg_RotateAnchor::new)
        }
    }

    // optional float angle = 1;

    pub fn clear_angle(&mut self) {
        self.angle = ::std::option::Option::None;
    }

    pub fn has_angle(&self) -> bool {
        self.angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle(&mut self, v: f32) {
        self.angle = ::std::option::Option::Some(v);
    }

    pub fn get_angle(&self) -> f32 {
        self.angle.unwrap_or(0.)
    }

    fn get_angle_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.angle
    }

    fn mut_angle_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.angle
    }
}

impl ::protobuf::Message for CClientMsg_RotateAnchor {
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
                    self.angle = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.angle {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.angle {
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

impl ::protobuf::MessageStatic for CClientMsg_RotateAnchor {
    fn new() -> CClientMsg_RotateAnchor {
        CClientMsg_RotateAnchor::new()
    }

    fn descriptor_static(_: ::std::option::Option<CClientMsg_RotateAnchor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "angle",
                    CClientMsg_RotateAnchor::get_angle_for_reflect,
                    CClientMsg_RotateAnchor::mut_angle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CClientMsg_RotateAnchor>(
                    "CClientMsg_RotateAnchor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CClientMsg_RotateAnchor {
    fn clear(&mut self) {
        self.clear_angle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CClientMsg_RotateAnchor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CClientMsg_RotateAnchor {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EBaseClientMessages {
    CM_CustomGameEvent = 280,
    CM_CustomGameEventBounce = 281,
    CM_ClientUIEvent = 282,
    CM_DevPaletteVisibilityChanged = 283,
    CM_WorldUIControllerHasPanelChanged = 284,
    CM_RotateAnchor = 285,
    CM_MAX_BASE = 300,
}

impl ::protobuf::ProtobufEnum for EBaseClientMessages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EBaseClientMessages> {
        match value {
            280 => ::std::option::Option::Some(EBaseClientMessages::CM_CustomGameEvent),
            281 => ::std::option::Option::Some(EBaseClientMessages::CM_CustomGameEventBounce),
            282 => ::std::option::Option::Some(EBaseClientMessages::CM_ClientUIEvent),
            283 => ::std::option::Option::Some(EBaseClientMessages::CM_DevPaletteVisibilityChanged),
            284 => ::std::option::Option::Some(EBaseClientMessages::CM_WorldUIControllerHasPanelChanged),
            285 => ::std::option::Option::Some(EBaseClientMessages::CM_RotateAnchor),
            300 => ::std::option::Option::Some(EBaseClientMessages::CM_MAX_BASE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EBaseClientMessages] = &[
            EBaseClientMessages::CM_CustomGameEvent,
            EBaseClientMessages::CM_CustomGameEventBounce,
            EBaseClientMessages::CM_ClientUIEvent,
            EBaseClientMessages::CM_DevPaletteVisibilityChanged,
            EBaseClientMessages::CM_WorldUIControllerHasPanelChanged,
            EBaseClientMessages::CM_RotateAnchor,
            EBaseClientMessages::CM_MAX_BASE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EBaseClientMessages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EBaseClientMessages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EBaseClientMessages {
}

impl ::protobuf::reflect::ProtobufValue for EBaseClientMessages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EClientUIEvent {
    EClientUIEvent_Invalid = 0,
    EClientUIEvent_DialogFinished = 1,
    EClientUIEvent_FireOutput = 2,
}

impl ::protobuf::ProtobufEnum for EClientUIEvent {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EClientUIEvent> {
        match value {
            0 => ::std::option::Option::Some(EClientUIEvent::EClientUIEvent_Invalid),
            1 => ::std::option::Option::Some(EClientUIEvent::EClientUIEvent_DialogFinished),
            2 => ::std::option::Option::Some(EClientUIEvent::EClientUIEvent_FireOutput),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EClientUIEvent] = &[
            EClientUIEvent::EClientUIEvent_Invalid,
            EClientUIEvent::EClientUIEvent_DialogFinished,
            EClientUIEvent::EClientUIEvent_FireOutput,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EClientUIEvent>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EClientUIEvent", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EClientUIEvent {
}

impl ::protobuf::reflect::ProtobufValue for EClientUIEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14clientmessages.proto\"O\n\x1aCClientMsg_CustomGameEvent\x12\x1d\n\
    \nevent_name\x18\x01\x20\x01(\tR\teventName\x12\x12\n\x04data\x18\x02\
    \x20\x01(\x0cR\x04data\"x\n\x20CClientMsg_CustomGameEventBounce\x12\x1d\
    \n\nevent_name\x18\x01\x20\x01(\tR\teventName\x12\x12\n\x04data\x18\x02\
    \x20\x01(\x0cR\x04data\x12!\n\x0cplayer_index\x18\x03\x20\x01(\x05R\x0bp\
    layerIndex\"\xcd\x01\n\x18CClientMsg_ClientUIEvent\x12=\n\x05event\x18\
    \x01\x20\x01(\x0e2\x0f.EClientUIEvent:\x16EClientUIEvent_InvalidR\x05eve\
    nt\x12\x1f\n\x0bent_ehandle\x18\x02\x20\x01(\rR\nentEhandle\x12%\n\x0ecl\
    ient_ehandle\x18\x03\x20\x01(\rR\rclientEhandle\x12\x14\n\x05data1\x18\
    \x04\x20\x01(\tR\x05data1\x12\x14\n\x05data2\x18\x05\x20\x01(\tR\x05data\
    2\"G\n+CClientMsg_DevPaletteVisibilityChangedEvent\x12\x18\n\x07visible\
    \x18\x01\x20\x01(\x08R\x07visible\"\xa2\x01\n0CClientMsg_WorldUIControll\
    erHasPanelChangedEvent\x12\x1b\n\thas_panel\x18\x01\x20\x01(\x08R\x08has\
    Panel\x12%\n\x0eclient_ehandle\x18\x02\x20\x01(\rR\rclientEhandle\x12*\n\
    \x11literal_hand_type\x18\x03\x20\x01(\rR\x0fliteralHandType\"/\n\x17CCl\
    ientMsg_RotateAnchor\x12\x14\n\x05angle\x18\x01\x20\x01(\x02R\x05angle*\
    \xdb\x01\n\x13EBaseClientMessages\x12\x17\n\x12CM_CustomGameEvent\x10\
    \x98\x02\x12\x1d\n\x18CM_CustomGameEventBounce\x10\x99\x02\x12\x15\n\x10\
    CM_ClientUIEvent\x10\x9a\x02\x12#\n\x1eCM_DevPaletteVisibilityChanged\
    \x10\x9b\x02\x12(\n#CM_WorldUIControllerHasPanelChanged\x10\x9c\x02\x12\
    \x14\n\x0fCM_RotateAnchor\x10\x9d\x02\x12\x10\n\x0bCM_MAX_BASE\x10\xac\
    \x02*n\n\x0eEClientUIEvent\x12\x1a\n\x16EClientUIEvent_Invalid\x10\0\x12\
    !\n\x1dEClientUIEvent_DialogFinished\x10\x01\x12\x1d\n\x19EClientUIEvent\
    _FireOutput\x10\x02B\x05H\x01\x80\x01\0\
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
