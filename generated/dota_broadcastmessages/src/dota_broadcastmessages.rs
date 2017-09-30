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
pub struct CDOTABroadcastMsg {
    // message fields
    field_type: ::std::option::Option<EDotaBroadcastMessages>,
    msg: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTABroadcastMsg {}

impl CDOTABroadcastMsg {
    pub fn new() -> CDOTABroadcastMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTABroadcastMsg {
        static mut instance: ::protobuf::lazy::Lazy<CDOTABroadcastMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTABroadcastMsg,
        };
        unsafe {
            instance.get(CDOTABroadcastMsg::new)
        }
    }

    // required .EDotaBroadcastMessages type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: EDotaBroadcastMessages) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> EDotaBroadcastMessages {
        self.field_type.unwrap_or(EDotaBroadcastMessages::DOTA_BM_LANLobbyRequest)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<EDotaBroadcastMessages> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<EDotaBroadcastMessages> {
        &mut self.field_type
    }

    // optional bytes msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::vec::Vec<u8>) {
        self.msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.msg.is_none() {
            self.msg.set_default();
        }
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::vec::Vec<u8> {
        self.msg.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_msg(&self) -> &[u8] {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_msg_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.msg
    }
}

impl ::protobuf::Message for CDOTABroadcastMsg {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
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
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.msg)?;
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.msg.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.msg.as_ref() {
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

impl ::protobuf::MessageStatic for CDOTABroadcastMsg {
    fn new() -> CDOTABroadcastMsg {
        CDOTABroadcastMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTABroadcastMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EDotaBroadcastMessages>>(
                    "type",
                    CDOTABroadcastMsg::get_field_type_for_reflect,
                    CDOTABroadcastMsg::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "msg",
                    CDOTABroadcastMsg::get_msg_for_reflect,
                    CDOTABroadcastMsg::mut_msg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTABroadcastMsg>(
                    "CDOTABroadcastMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTABroadcastMsg {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTABroadcastMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTABroadcastMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTABroadcastMsg_LANLobbyRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTABroadcastMsg_LANLobbyRequest {}

impl CDOTABroadcastMsg_LANLobbyRequest {
    pub fn new() -> CDOTABroadcastMsg_LANLobbyRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTABroadcastMsg_LANLobbyRequest {
        static mut instance: ::protobuf::lazy::Lazy<CDOTABroadcastMsg_LANLobbyRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTABroadcastMsg_LANLobbyRequest,
        };
        unsafe {
            instance.get(CDOTABroadcastMsg_LANLobbyRequest::new)
        }
    }
}

impl ::protobuf::Message for CDOTABroadcastMsg_LANLobbyRequest {
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

impl ::protobuf::MessageStatic for CDOTABroadcastMsg_LANLobbyRequest {
    fn new() -> CDOTABroadcastMsg_LANLobbyRequest {
        CDOTABroadcastMsg_LANLobbyRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTABroadcastMsg_LANLobbyRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CDOTABroadcastMsg_LANLobbyRequest>(
                    "CDOTABroadcastMsg_LANLobbyRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTABroadcastMsg_LANLobbyRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTABroadcastMsg_LANLobbyRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTABroadcastMsg_LANLobbyRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTABroadcastMsg_LANLobbyReply {
    // message fields
    id: ::std::option::Option<u64>,
    tournament_id: ::std::option::Option<u32>,
    tournament_game_id: ::std::option::Option<u32>,
    members: ::protobuf::RepeatedField<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember>,
    requires_pass_key: ::std::option::Option<bool>,
    leader_account_id: ::std::option::Option<u32>,
    game_mode: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    players: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTABroadcastMsg_LANLobbyReply {}

impl CDOTABroadcastMsg_LANLobbyReply {
    pub fn new() -> CDOTABroadcastMsg_LANLobbyReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTABroadcastMsg_LANLobbyReply {
        static mut instance: ::protobuf::lazy::Lazy<CDOTABroadcastMsg_LANLobbyReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTABroadcastMsg_LANLobbyReply,
        };
        unsafe {
            instance.get(CDOTABroadcastMsg_LANLobbyReply::new)
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

    // optional uint32 tournament_id = 2;

    pub fn clear_tournament_id(&mut self) {
        self.tournament_id = ::std::option::Option::None;
    }

    pub fn has_tournament_id(&self) -> bool {
        self.tournament_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tournament_id(&mut self, v: u32) {
        self.tournament_id = ::std::option::Option::Some(v);
    }

    pub fn get_tournament_id(&self) -> u32 {
        self.tournament_id.unwrap_or(0)
    }

    fn get_tournament_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tournament_id
    }

    fn mut_tournament_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tournament_id
    }

    // optional uint32 tournament_game_id = 3;

    pub fn clear_tournament_game_id(&mut self) {
        self.tournament_game_id = ::std::option::Option::None;
    }

    pub fn has_tournament_game_id(&self) -> bool {
        self.tournament_game_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tournament_game_id(&mut self, v: u32) {
        self.tournament_game_id = ::std::option::Option::Some(v);
    }

    pub fn get_tournament_game_id(&self) -> u32 {
        self.tournament_game_id.unwrap_or(0)
    }

    fn get_tournament_game_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tournament_game_id
    }

    fn mut_tournament_game_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tournament_game_id
    }

    // repeated .CDOTABroadcastMsg_LANLobbyReply.CLobbyMember members = 4;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[CDOTABroadcastMsg_LANLobbyReply_CLobbyMember] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember> {
        &mut self.members
    }

    // optional bool requires_pass_key = 5;

    pub fn clear_requires_pass_key(&mut self) {
        self.requires_pass_key = ::std::option::Option::None;
    }

    pub fn has_requires_pass_key(&self) -> bool {
        self.requires_pass_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_requires_pass_key(&mut self, v: bool) {
        self.requires_pass_key = ::std::option::Option::Some(v);
    }

    pub fn get_requires_pass_key(&self) -> bool {
        self.requires_pass_key.unwrap_or(false)
    }

    fn get_requires_pass_key_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.requires_pass_key
    }

    fn mut_requires_pass_key_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.requires_pass_key
    }

    // optional uint32 leader_account_id = 6;

    pub fn clear_leader_account_id(&mut self) {
        self.leader_account_id = ::std::option::Option::None;
    }

    pub fn has_leader_account_id(&self) -> bool {
        self.leader_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader_account_id(&mut self, v: u32) {
        self.leader_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_leader_account_id(&self) -> u32 {
        self.leader_account_id.unwrap_or(0)
    }

    fn get_leader_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.leader_account_id
    }

    fn mut_leader_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.leader_account_id
    }

    // optional uint32 game_mode = 7;

    pub fn clear_game_mode(&mut self) {
        self.game_mode = ::std::option::Option::None;
    }

    pub fn has_game_mode(&self) -> bool {
        self.game_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_mode(&mut self, v: u32) {
        self.game_mode = ::std::option::Option::Some(v);
    }

    pub fn get_game_mode(&self) -> u32 {
        self.game_mode.unwrap_or(0)
    }

    fn get_game_mode_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.game_mode
    }

    fn mut_game_mode_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.game_mode
    }

    // optional string name = 8;

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

    // optional uint32 players = 9;

    pub fn clear_players(&mut self) {
        self.players = ::std::option::Option::None;
    }

    pub fn has_players(&self) -> bool {
        self.players.is_some()
    }

    // Param is passed by value, moved
    pub fn set_players(&mut self, v: u32) {
        self.players = ::std::option::Option::Some(v);
    }

    pub fn get_players(&self) -> u32 {
        self.players.unwrap_or(0)
    }

    fn get_players_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.players
    }

    fn mut_players_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.players
    }
}

impl ::protobuf::Message for CDOTABroadcastMsg_LANLobbyReply {
    fn is_initialized(&self) -> bool {
        for v in &self.members {
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
                    self.tournament_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.tournament_game_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.requires_pass_key = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.leader_account_id = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.game_mode = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.players = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.tournament_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.tournament_game_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.requires_pass_key {
            my_size += 2;
        }
        if let Some(v) = self.leader_account_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.game_mode {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(v) = self.players {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.tournament_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.tournament_game_id {
            os.write_uint32(3, v)?;
        }
        for v in &self.members {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.requires_pass_key {
            os.write_bool(5, v)?;
        }
        if let Some(v) = self.leader_account_id {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.game_mode {
            os.write_uint32(7, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(8, &v)?;
        }
        if let Some(v) = self.players {
            os.write_uint32(9, v)?;
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

impl ::protobuf::MessageStatic for CDOTABroadcastMsg_LANLobbyReply {
    fn new() -> CDOTABroadcastMsg_LANLobbyReply {
        CDOTABroadcastMsg_LANLobbyReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTABroadcastMsg_LANLobbyReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    CDOTABroadcastMsg_LANLobbyReply::get_id_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tournament_id",
                    CDOTABroadcastMsg_LANLobbyReply::get_tournament_id_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_tournament_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tournament_game_id",
                    CDOTABroadcastMsg_LANLobbyReply::get_tournament_game_id_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_tournament_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember>>(
                    "members",
                    CDOTABroadcastMsg_LANLobbyReply::get_members_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "requires_pass_key",
                    CDOTABroadcastMsg_LANLobbyReply::get_requires_pass_key_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_requires_pass_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "leader_account_id",
                    CDOTABroadcastMsg_LANLobbyReply::get_leader_account_id_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_leader_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_mode",
                    CDOTABroadcastMsg_LANLobbyReply::get_game_mode_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_game_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CDOTABroadcastMsg_LANLobbyReply::get_name_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "players",
                    CDOTABroadcastMsg_LANLobbyReply::get_players_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_players_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTABroadcastMsg_LANLobbyReply>(
                    "CDOTABroadcastMsg_LANLobbyReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTABroadcastMsg_LANLobbyReply {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_tournament_id();
        self.clear_tournament_game_id();
        self.clear_members();
        self.clear_requires_pass_key();
        self.clear_leader_account_id();
        self.clear_game_mode();
        self.clear_name();
        self.clear_players();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTABroadcastMsg_LANLobbyReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTABroadcastMsg_LANLobbyReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
    // message fields
    account_id: ::std::option::Option<u32>,
    player_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {}

impl CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
    pub fn new() -> CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
        static mut instance: ::protobuf::lazy::Lazy<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTABroadcastMsg_LANLobbyReply_CLobbyMember,
        };
        unsafe {
            instance.get(CDOTABroadcastMsg_LANLobbyReply_CLobbyMember::new)
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

    // optional string player_name = 2;

    pub fn clear_player_name(&mut self) {
        self.player_name.clear();
    }

    pub fn has_player_name(&self) -> bool {
        self.player_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_name(&mut self, v: ::std::string::String) {
        self.player_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_name(&mut self) -> &mut ::std::string::String {
        if self.player_name.is_none() {
            self.player_name.set_default();
        }
        self.player_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_name(&mut self) -> ::std::string::String {
        self.player_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_player_name(&self) -> &str {
        match self.player_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_player_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.player_name
    }

    fn mut_player_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.player_name
    }
}

impl ::protobuf::Message for CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.player_name)?;
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
        if let Some(ref v) = self.player_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.player_name.as_ref() {
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

impl ::protobuf::MessageStatic for CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
    fn new() -> CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
        CDOTABroadcastMsg_LANLobbyReply_CLobbyMember::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CDOTABroadcastMsg_LANLobbyReply_CLobbyMember::get_account_id_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply_CLobbyMember::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "player_name",
                    CDOTABroadcastMsg_LANLobbyReply_CLobbyMember::get_player_name_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply_CLobbyMember::mut_player_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember>(
                    "CDOTABroadcastMsg_LANLobbyReply_CLobbyMember",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_player_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EDotaBroadcastMessages {
    DOTA_BM_LANLobbyRequest = 1,
    DOTA_BM_LANLobbyReply = 2,
}

impl ::protobuf::ProtobufEnum for EDotaBroadcastMessages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EDotaBroadcastMessages> {
        match value {
            1 => ::std::option::Option::Some(EDotaBroadcastMessages::DOTA_BM_LANLobbyRequest),
            2 => ::std::option::Option::Some(EDotaBroadcastMessages::DOTA_BM_LANLobbyReply),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EDotaBroadcastMessages] = &[
            EDotaBroadcastMessages::DOTA_BM_LANLobbyRequest,
            EDotaBroadcastMessages::DOTA_BM_LANLobbyReply,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EDotaBroadcastMessages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EDotaBroadcastMessages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EDotaBroadcastMessages {
}

impl ::protobuf::reflect::ProtobufValue for EDotaBroadcastMessages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cdota_broadcastmessages.proto\"k\n\x11CDOTABroadcastMsg\x12D\n\x04t\
    ype\x18\x01\x20\x02(\x0e2\x17.EDotaBroadcastMessages:\x17DOTA_BM_LANLobb\
    yRequestR\x04type\x12\x10\n\x03msg\x18\x02\x20\x01(\x0cR\x03msg\"#\n!CDO\
    TABroadcastMsg_LANLobbyRequest\"\xc0\x03\n\x1fCDOTABroadcastMsg_LANLobby\
    Reply\x12\x0e\n\x02id\x18\x01\x20\x01(\x04R\x02id\x12#\n\rtournament_id\
    \x18\x02\x20\x01(\rR\x0ctournamentId\x12,\n\x12tournament_game_id\x18\
    \x03\x20\x01(\rR\x10tournamentGameId\x12G\n\x07members\x18\x04\x20\x03(\
    \x0b2-.CDOTABroadcastMsg_LANLobbyReply.CLobbyMemberR\x07members\x12*\n\
    \x11requires_pass_key\x18\x05\x20\x01(\x08R\x0frequiresPassKey\x12*\n\
    \x11leader_account_id\x18\x06\x20\x01(\rR\x0fleaderAccountId\x12\x1b\n\t\
    game_mode\x18\x07\x20\x01(\rR\x08gameMode\x12\x12\n\x04name\x18\x08\x20\
    \x01(\tR\x04name\x12\x18\n\x07players\x18\t\x20\x01(\rR\x07players\x1aN\
    \n\x0cCLobbyMember\x12\x1d\n\naccount_id\x18\x01\x20\x01(\rR\taccountId\
    \x12\x1f\n\x0bplayer_name\x18\x02\x20\x01(\tR\nplayerName*P\n\x16EDotaBr\
    oadcastMessages\x12\x1b\n\x17DOTA_BM_LANLobbyRequest\x10\x01\x12\x19\n\
    \x15DOTA_BM_LANLobbyReply\x10\x02B\x05H\x01\x80\x01\0\
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
