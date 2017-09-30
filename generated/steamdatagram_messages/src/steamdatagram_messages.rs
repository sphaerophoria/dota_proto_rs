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
pub struct CMsgSteamDatagramRouterPingReply {
    // message fields
    client_timestamp: ::std::option::Option<u32>,
    latency_datacenter_ids: ::std::vec::Vec<u32>,
    latency_ping_ms: ::std::vec::Vec<u32>,
    your_public_ip: ::std::option::Option<u32>,
    server_time: ::std::option::Option<u32>,
    challenge: ::std::option::Option<u64>,
    seconds_until_shutdown: ::std::option::Option<u32>,
    client_cookie: ::std::option::Option<u32>,
    scoring_penalty_relay_cluster: ::std::option::Option<u32>,
    route_exceptions: ::protobuf::RepeatedField<CMsgSteamDatagramRouterPingReply_RouteException>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramRouterPingReply {}

impl CMsgSteamDatagramRouterPingReply {
    pub fn new() -> CMsgSteamDatagramRouterPingReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramRouterPingReply {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramRouterPingReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramRouterPingReply,
        };
        unsafe {
            instance.get(CMsgSteamDatagramRouterPingReply::new)
        }
    }

    // optional fixed32 client_timestamp = 1;

    pub fn clear_client_timestamp(&mut self) {
        self.client_timestamp = ::std::option::Option::None;
    }

    pub fn has_client_timestamp(&self) -> bool {
        self.client_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_timestamp(&mut self, v: u32) {
        self.client_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_client_timestamp(&self) -> u32 {
        self.client_timestamp.unwrap_or(0)
    }

    fn get_client_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_timestamp
    }

    fn mut_client_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_timestamp
    }

    // repeated fixed32 latency_datacenter_ids = 2;

    pub fn clear_latency_datacenter_ids(&mut self) {
        self.latency_datacenter_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_latency_datacenter_ids(&mut self, v: ::std::vec::Vec<u32>) {
        self.latency_datacenter_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_latency_datacenter_ids(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.latency_datacenter_ids
    }

    // Take field
    pub fn take_latency_datacenter_ids(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.latency_datacenter_ids, ::std::vec::Vec::new())
    }

    pub fn get_latency_datacenter_ids(&self) -> &[u32] {
        &self.latency_datacenter_ids
    }

    fn get_latency_datacenter_ids_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.latency_datacenter_ids
    }

    fn mut_latency_datacenter_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.latency_datacenter_ids
    }

    // repeated uint32 latency_ping_ms = 3;

    pub fn clear_latency_ping_ms(&mut self) {
        self.latency_ping_ms.clear();
    }

    // Param is passed by value, moved
    pub fn set_latency_ping_ms(&mut self, v: ::std::vec::Vec<u32>) {
        self.latency_ping_ms = v;
    }

    // Mutable pointer to the field.
    pub fn mut_latency_ping_ms(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.latency_ping_ms
    }

    // Take field
    pub fn take_latency_ping_ms(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.latency_ping_ms, ::std::vec::Vec::new())
    }

    pub fn get_latency_ping_ms(&self) -> &[u32] {
        &self.latency_ping_ms
    }

    fn get_latency_ping_ms_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.latency_ping_ms
    }

    fn mut_latency_ping_ms_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.latency_ping_ms
    }

    // optional fixed32 your_public_ip = 4;

    pub fn clear_your_public_ip(&mut self) {
        self.your_public_ip = ::std::option::Option::None;
    }

    pub fn has_your_public_ip(&self) -> bool {
        self.your_public_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_your_public_ip(&mut self, v: u32) {
        self.your_public_ip = ::std::option::Option::Some(v);
    }

    pub fn get_your_public_ip(&self) -> u32 {
        self.your_public_ip.unwrap_or(0)
    }

    fn get_your_public_ip_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.your_public_ip
    }

    fn mut_your_public_ip_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.your_public_ip
    }

    // optional fixed32 server_time = 5;

    pub fn clear_server_time(&mut self) {
        self.server_time = ::std::option::Option::None;
    }

    pub fn has_server_time(&self) -> bool {
        self.server_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_time(&mut self, v: u32) {
        self.server_time = ::std::option::Option::Some(v);
    }

    pub fn get_server_time(&self) -> u32 {
        self.server_time.unwrap_or(0)
    }

    fn get_server_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_time
    }

    fn mut_server_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_time
    }

    // optional fixed64 challenge = 6;

    pub fn clear_challenge(&mut self) {
        self.challenge = ::std::option::Option::None;
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: u64) {
        self.challenge = ::std::option::Option::Some(v);
    }

    pub fn get_challenge(&self) -> u64 {
        self.challenge.unwrap_or(0)
    }

    fn get_challenge_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.challenge
    }

    fn mut_challenge_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.challenge
    }

    // optional uint32 seconds_until_shutdown = 7;

    pub fn clear_seconds_until_shutdown(&mut self) {
        self.seconds_until_shutdown = ::std::option::Option::None;
    }

    pub fn has_seconds_until_shutdown(&self) -> bool {
        self.seconds_until_shutdown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seconds_until_shutdown(&mut self, v: u32) {
        self.seconds_until_shutdown = ::std::option::Option::Some(v);
    }

    pub fn get_seconds_until_shutdown(&self) -> u32 {
        self.seconds_until_shutdown.unwrap_or(0)
    }

    fn get_seconds_until_shutdown_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seconds_until_shutdown
    }

    fn mut_seconds_until_shutdown_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seconds_until_shutdown
    }

    // optional fixed32 client_cookie = 8;

    pub fn clear_client_cookie(&mut self) {
        self.client_cookie = ::std::option::Option::None;
    }

    pub fn has_client_cookie(&self) -> bool {
        self.client_cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_cookie(&mut self, v: u32) {
        self.client_cookie = ::std::option::Option::Some(v);
    }

    pub fn get_client_cookie(&self) -> u32 {
        self.client_cookie.unwrap_or(0)
    }

    fn get_client_cookie_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_cookie
    }

    fn mut_client_cookie_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_cookie
    }

    // optional uint32 scoring_penalty_relay_cluster = 9;

    pub fn clear_scoring_penalty_relay_cluster(&mut self) {
        self.scoring_penalty_relay_cluster = ::std::option::Option::None;
    }

    pub fn has_scoring_penalty_relay_cluster(&self) -> bool {
        self.scoring_penalty_relay_cluster.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scoring_penalty_relay_cluster(&mut self, v: u32) {
        self.scoring_penalty_relay_cluster = ::std::option::Option::Some(v);
    }

    pub fn get_scoring_penalty_relay_cluster(&self) -> u32 {
        self.scoring_penalty_relay_cluster.unwrap_or(0)
    }

    fn get_scoring_penalty_relay_cluster_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.scoring_penalty_relay_cluster
    }

    fn mut_scoring_penalty_relay_cluster_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.scoring_penalty_relay_cluster
    }

    // repeated .CMsgSteamDatagramRouterPingReply.RouteException route_exceptions = 10;

    pub fn clear_route_exceptions(&mut self) {
        self.route_exceptions.clear();
    }

    // Param is passed by value, moved
    pub fn set_route_exceptions(&mut self, v: ::protobuf::RepeatedField<CMsgSteamDatagramRouterPingReply_RouteException>) {
        self.route_exceptions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_route_exceptions(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramRouterPingReply_RouteException> {
        &mut self.route_exceptions
    }

    // Take field
    pub fn take_route_exceptions(&mut self) -> ::protobuf::RepeatedField<CMsgSteamDatagramRouterPingReply_RouteException> {
        ::std::mem::replace(&mut self.route_exceptions, ::protobuf::RepeatedField::new())
    }

    pub fn get_route_exceptions(&self) -> &[CMsgSteamDatagramRouterPingReply_RouteException] {
        &self.route_exceptions
    }

    fn get_route_exceptions_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSteamDatagramRouterPingReply_RouteException> {
        &self.route_exceptions
    }

    fn mut_route_exceptions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramRouterPingReply_RouteException> {
        &mut self.route_exceptions
    }
}

impl ::protobuf::Message for CMsgSteamDatagramRouterPingReply {
    fn is_initialized(&self) -> bool {
        for v in &self.route_exceptions {
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
                    self.client_timestamp = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.latency_datacenter_ids)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.latency_ping_ms)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.your_public_ip = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.server_time = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.challenge = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seconds_until_shutdown = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.client_cookie = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.scoring_penalty_relay_cluster = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.route_exceptions)?;
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
        if let Some(v) = self.client_timestamp {
            my_size += 5;
        }
        if !self.latency_datacenter_ids.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.latency_datacenter_ids.len() as u32) + (self.latency_datacenter_ids.len() * 4) as u32;
        }
        if !self.latency_ping_ms.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(3, &self.latency_ping_ms);
        }
        if let Some(v) = self.your_public_ip {
            my_size += 5;
        }
        if let Some(v) = self.server_time {
            my_size += 5;
        }
        if let Some(v) = self.challenge {
            my_size += 9;
        }
        if let Some(v) = self.seconds_until_shutdown {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.client_cookie {
            my_size += 5;
        }
        if let Some(v) = self.scoring_penalty_relay_cluster {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.route_exceptions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_timestamp {
            os.write_fixed32(1, v)?;
        }
        if !self.latency_datacenter_ids.is_empty() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.latency_datacenter_ids.len() * 4) as u32)?;
            for v in &self.latency_datacenter_ids {
                os.write_fixed32_no_tag(*v)?;
            };
        }
        if !self.latency_ping_ms.is_empty() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.latency_ping_ms))?;
            for v in &self.latency_ping_ms {
                os.write_uint32_no_tag(*v)?;
            };
        }
        if let Some(v) = self.your_public_ip {
            os.write_fixed32(4, v)?;
        }
        if let Some(v) = self.server_time {
            os.write_fixed32(5, v)?;
        }
        if let Some(v) = self.challenge {
            os.write_fixed64(6, v)?;
        }
        if let Some(v) = self.seconds_until_shutdown {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.client_cookie {
            os.write_fixed32(8, v)?;
        }
        if let Some(v) = self.scoring_penalty_relay_cluster {
            os.write_uint32(9, v)?;
        }
        for v in &self.route_exceptions {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramRouterPingReply {
    fn new() -> CMsgSteamDatagramRouterPingReply {
        CMsgSteamDatagramRouterPingReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramRouterPingReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_timestamp",
                    CMsgSteamDatagramRouterPingReply::get_client_timestamp_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_client_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "latency_datacenter_ids",
                    CMsgSteamDatagramRouterPingReply::get_latency_datacenter_ids_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_latency_datacenter_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "latency_ping_ms",
                    CMsgSteamDatagramRouterPingReply::get_latency_ping_ms_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_latency_ping_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "your_public_ip",
                    CMsgSteamDatagramRouterPingReply::get_your_public_ip_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_your_public_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "server_time",
                    CMsgSteamDatagramRouterPingReply::get_server_time_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_server_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "challenge",
                    CMsgSteamDatagramRouterPingReply::get_challenge_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seconds_until_shutdown",
                    CMsgSteamDatagramRouterPingReply::get_seconds_until_shutdown_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_seconds_until_shutdown_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_cookie",
                    CMsgSteamDatagramRouterPingReply::get_client_cookie_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_client_cookie_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "scoring_penalty_relay_cluster",
                    CMsgSteamDatagramRouterPingReply::get_scoring_penalty_relay_cluster_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_scoring_penalty_relay_cluster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramRouterPingReply_RouteException>>(
                    "route_exceptions",
                    CMsgSteamDatagramRouterPingReply::get_route_exceptions_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_route_exceptions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramRouterPingReply>(
                    "CMsgSteamDatagramRouterPingReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramRouterPingReply {
    fn clear(&mut self) {
        self.clear_client_timestamp();
        self.clear_latency_datacenter_ids();
        self.clear_latency_ping_ms();
        self.clear_your_public_ip();
        self.clear_server_time();
        self.clear_challenge();
        self.clear_seconds_until_shutdown();
        self.clear_client_cookie();
        self.clear_scoring_penalty_relay_cluster();
        self.clear_route_exceptions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramRouterPingReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramRouterPingReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramRouterPingReply_RouteException {
    // message fields
    data_center_id: ::std::option::Option<u32>,
    flags: ::std::option::Option<u32>,
    penalty: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramRouterPingReply_RouteException {}

impl CMsgSteamDatagramRouterPingReply_RouteException {
    pub fn new() -> CMsgSteamDatagramRouterPingReply_RouteException {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramRouterPingReply_RouteException {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramRouterPingReply_RouteException> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramRouterPingReply_RouteException,
        };
        unsafe {
            instance.get(CMsgSteamDatagramRouterPingReply_RouteException::new)
        }
    }

    // optional fixed32 data_center_id = 1;

    pub fn clear_data_center_id(&mut self) {
        self.data_center_id = ::std::option::Option::None;
    }

    pub fn has_data_center_id(&self) -> bool {
        self.data_center_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_center_id(&mut self, v: u32) {
        self.data_center_id = ::std::option::Option::Some(v);
    }

    pub fn get_data_center_id(&self) -> u32 {
        self.data_center_id.unwrap_or(0)
    }

    fn get_data_center_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.data_center_id
    }

    fn mut_data_center_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.data_center_id
    }

    // optional uint32 flags = 2;

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

    // optional uint32 penalty = 3;

    pub fn clear_penalty(&mut self) {
        self.penalty = ::std::option::Option::None;
    }

    pub fn has_penalty(&self) -> bool {
        self.penalty.is_some()
    }

    // Param is passed by value, moved
    pub fn set_penalty(&mut self, v: u32) {
        self.penalty = ::std::option::Option::Some(v);
    }

    pub fn get_penalty(&self) -> u32 {
        self.penalty.unwrap_or(0)
    }

    fn get_penalty_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.penalty
    }

    fn mut_penalty_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.penalty
    }
}

impl ::protobuf::Message for CMsgSteamDatagramRouterPingReply_RouteException {
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
                    self.data_center_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.penalty = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.data_center_id {
            my_size += 5;
        }
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.penalty {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.data_center_id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.flags {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.penalty {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramRouterPingReply_RouteException {
    fn new() -> CMsgSteamDatagramRouterPingReply_RouteException {
        CMsgSteamDatagramRouterPingReply_RouteException::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramRouterPingReply_RouteException>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "data_center_id",
                    CMsgSteamDatagramRouterPingReply_RouteException::get_data_center_id_for_reflect,
                    CMsgSteamDatagramRouterPingReply_RouteException::mut_data_center_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CMsgSteamDatagramRouterPingReply_RouteException::get_flags_for_reflect,
                    CMsgSteamDatagramRouterPingReply_RouteException::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "penalty",
                    CMsgSteamDatagramRouterPingReply_RouteException::get_penalty_for_reflect,
                    CMsgSteamDatagramRouterPingReply_RouteException::mut_penalty_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramRouterPingReply_RouteException>(
                    "CMsgSteamDatagramRouterPingReply_RouteException",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramRouterPingReply_RouteException {
    fn clear(&mut self) {
        self.clear_data_center_id();
        self.clear_flags();
        self.clear_penalty();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramRouterPingReply_RouteException {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramRouterPingReply_RouteException {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramGameserverPing {
    // message fields
    legacy_client_session: ::std::option::Option<u32>,
    client_steam_id: ::std::option::Option<u64>,
    client_timestamp: ::std::option::Option<u32>,
    router_timestamp: ::std::option::Option<u32>,
    router_gameserver_latency: ::std::option::Option<u32>,
    seq_number_router: ::std::option::Option<u32>,
    seq_number_e2e: ::std::option::Option<u32>,
    relay_session_id: ::std::option::Option<u32>,
    connection_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramGameserverPing {}

impl CMsgSteamDatagramGameserverPing {
    pub fn new() -> CMsgSteamDatagramGameserverPing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramGameserverPing {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramGameserverPing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramGameserverPing,
        };
        unsafe {
            instance.get(CMsgSteamDatagramGameserverPing::new)
        }
    }

    // optional uint32 legacy_client_session = 1;

    pub fn clear_legacy_client_session(&mut self) {
        self.legacy_client_session = ::std::option::Option::None;
    }

    pub fn has_legacy_client_session(&self) -> bool {
        self.legacy_client_session.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_client_session(&mut self, v: u32) {
        self.legacy_client_session = ::std::option::Option::Some(v);
    }

    pub fn get_legacy_client_session(&self) -> u32 {
        self.legacy_client_session.unwrap_or(0)
    }

    fn get_legacy_client_session_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.legacy_client_session
    }

    fn mut_legacy_client_session_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.legacy_client_session
    }

    // optional fixed64 client_steam_id = 2;

    pub fn clear_client_steam_id(&mut self) {
        self.client_steam_id = ::std::option::Option::None;
    }

    pub fn has_client_steam_id(&self) -> bool {
        self.client_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_steam_id(&mut self, v: u64) {
        self.client_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_steam_id(&self) -> u64 {
        self.client_steam_id.unwrap_or(0)
    }

    fn get_client_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.client_steam_id
    }

    fn mut_client_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.client_steam_id
    }

    // optional fixed32 client_timestamp = 3;

    pub fn clear_client_timestamp(&mut self) {
        self.client_timestamp = ::std::option::Option::None;
    }

    pub fn has_client_timestamp(&self) -> bool {
        self.client_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_timestamp(&mut self, v: u32) {
        self.client_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_client_timestamp(&self) -> u32 {
        self.client_timestamp.unwrap_or(0)
    }

    fn get_client_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_timestamp
    }

    fn mut_client_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_timestamp
    }

    // optional fixed32 router_timestamp = 4;

    pub fn clear_router_timestamp(&mut self) {
        self.router_timestamp = ::std::option::Option::None;
    }

    pub fn has_router_timestamp(&self) -> bool {
        self.router_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_router_timestamp(&mut self, v: u32) {
        self.router_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_router_timestamp(&self) -> u32 {
        self.router_timestamp.unwrap_or(0)
    }

    fn get_router_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.router_timestamp
    }

    fn mut_router_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.router_timestamp
    }

    // optional uint32 router_gameserver_latency = 5;

    pub fn clear_router_gameserver_latency(&mut self) {
        self.router_gameserver_latency = ::std::option::Option::None;
    }

    pub fn has_router_gameserver_latency(&self) -> bool {
        self.router_gameserver_latency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_router_gameserver_latency(&mut self, v: u32) {
        self.router_gameserver_latency = ::std::option::Option::Some(v);
    }

    pub fn get_router_gameserver_latency(&self) -> u32 {
        self.router_gameserver_latency.unwrap_or(0)
    }

    fn get_router_gameserver_latency_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.router_gameserver_latency
    }

    fn mut_router_gameserver_latency_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.router_gameserver_latency
    }

    // optional uint32 seq_number_router = 6;

    pub fn clear_seq_number_router(&mut self) {
        self.seq_number_router = ::std::option::Option::None;
    }

    pub fn has_seq_number_router(&self) -> bool {
        self.seq_number_router.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_number_router(&mut self, v: u32) {
        self.seq_number_router = ::std::option::Option::Some(v);
    }

    pub fn get_seq_number_router(&self) -> u32 {
        self.seq_number_router.unwrap_or(0)
    }

    fn get_seq_number_router_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_number_router
    }

    fn mut_seq_number_router_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_number_router
    }

    // optional uint32 seq_number_e2e = 7;

    pub fn clear_seq_number_e2e(&mut self) {
        self.seq_number_e2e = ::std::option::Option::None;
    }

    pub fn has_seq_number_e2e(&self) -> bool {
        self.seq_number_e2e.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_number_e2e(&mut self, v: u32) {
        self.seq_number_e2e = ::std::option::Option::Some(v);
    }

    pub fn get_seq_number_e2e(&self) -> u32 {
        self.seq_number_e2e.unwrap_or(0)
    }

    fn get_seq_number_e2e_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_number_e2e
    }

    fn mut_seq_number_e2e_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_number_e2e
    }

    // optional uint32 relay_session_id = 8;

    pub fn clear_relay_session_id(&mut self) {
        self.relay_session_id = ::std::option::Option::None;
    }

    pub fn has_relay_session_id(&self) -> bool {
        self.relay_session_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_relay_session_id(&mut self, v: u32) {
        self.relay_session_id = ::std::option::Option::Some(v);
    }

    pub fn get_relay_session_id(&self) -> u32 {
        self.relay_session_id.unwrap_or(0)
    }

    fn get_relay_session_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.relay_session_id
    }

    fn mut_relay_session_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.relay_session_id
    }

    // optional fixed32 connection_id = 9;

    pub fn clear_connection_id(&mut self) {
        self.connection_id = ::std::option::Option::None;
    }

    pub fn has_connection_id(&self) -> bool {
        self.connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connection_id(&mut self, v: u32) {
        self.connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_connection_id(&self) -> u32 {
        self.connection_id.unwrap_or(0)
    }

    fn get_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.connection_id
    }

    fn mut_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.connection_id
    }
}

impl ::protobuf::Message for CMsgSteamDatagramGameserverPing {
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
                    self.legacy_client_session = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.client_steam_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.client_timestamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.router_timestamp = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.router_gameserver_latency = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seq_number_router = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seq_number_e2e = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.relay_session_id = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.connection_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.legacy_client_session {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.client_steam_id {
            my_size += 9;
        }
        if let Some(v) = self.client_timestamp {
            my_size += 5;
        }
        if let Some(v) = self.router_timestamp {
            my_size += 5;
        }
        if let Some(v) = self.router_gameserver_latency {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.seq_number_router {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.seq_number_e2e {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.relay_session_id {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.connection_id {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.legacy_client_session {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.client_steam_id {
            os.write_fixed64(2, v)?;
        }
        if let Some(v) = self.client_timestamp {
            os.write_fixed32(3, v)?;
        }
        if let Some(v) = self.router_timestamp {
            os.write_fixed32(4, v)?;
        }
        if let Some(v) = self.router_gameserver_latency {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.seq_number_router {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.seq_number_e2e {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.relay_session_id {
            os.write_uint32(8, v)?;
        }
        if let Some(v) = self.connection_id {
            os.write_fixed32(9, v)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramGameserverPing {
    fn new() -> CMsgSteamDatagramGameserverPing {
        CMsgSteamDatagramGameserverPing::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramGameserverPing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "legacy_client_session",
                    CMsgSteamDatagramGameserverPing::get_legacy_client_session_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_legacy_client_session_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "client_steam_id",
                    CMsgSteamDatagramGameserverPing::get_client_steam_id_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_client_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_timestamp",
                    CMsgSteamDatagramGameserverPing::get_client_timestamp_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_client_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "router_timestamp",
                    CMsgSteamDatagramGameserverPing::get_router_timestamp_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_router_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "router_gameserver_latency",
                    CMsgSteamDatagramGameserverPing::get_router_gameserver_latency_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_router_gameserver_latency_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_number_router",
                    CMsgSteamDatagramGameserverPing::get_seq_number_router_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_seq_number_router_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_number_e2e",
                    CMsgSteamDatagramGameserverPing::get_seq_number_e2e_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_seq_number_e2e_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "relay_session_id",
                    CMsgSteamDatagramGameserverPing::get_relay_session_id_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_relay_session_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "connection_id",
                    CMsgSteamDatagramGameserverPing::get_connection_id_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_connection_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramGameserverPing>(
                    "CMsgSteamDatagramGameserverPing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramGameserverPing {
    fn clear(&mut self) {
        self.clear_legacy_client_session();
        self.clear_client_steam_id();
        self.clear_client_timestamp();
        self.clear_router_timestamp();
        self.clear_router_gameserver_latency();
        self.clear_seq_number_router();
        self.clear_seq_number_e2e();
        self.clear_relay_session_id();
        self.clear_connection_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramGameserverPing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramGameserverPing {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramSessionCryptInfo {
    // message fields
    key_type: ::std::option::Option<CMsgSteamDatagramSessionCryptInfo_EKeyType>,
    key_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    nonce: ::std::option::Option<u64>,
    is_snp: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramSessionCryptInfo {}

impl CMsgSteamDatagramSessionCryptInfo {
    pub fn new() -> CMsgSteamDatagramSessionCryptInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramSessionCryptInfo {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramSessionCryptInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramSessionCryptInfo,
        };
        unsafe {
            instance.get(CMsgSteamDatagramSessionCryptInfo::new)
        }
    }

    // optional .CMsgSteamDatagramSessionCryptInfo.EKeyType key_type = 1;

    pub fn clear_key_type(&mut self) {
        self.key_type = ::std::option::Option::None;
    }

    pub fn has_key_type(&self) -> bool {
        self.key_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_type(&mut self, v: CMsgSteamDatagramSessionCryptInfo_EKeyType) {
        self.key_type = ::std::option::Option::Some(v);
    }

    pub fn get_key_type(&self) -> CMsgSteamDatagramSessionCryptInfo_EKeyType {
        self.key_type.unwrap_or(CMsgSteamDatagramSessionCryptInfo_EKeyType::INVALID)
    }

    fn get_key_type_for_reflect(&self) -> &::std::option::Option<CMsgSteamDatagramSessionCryptInfo_EKeyType> {
        &self.key_type
    }

    fn mut_key_type_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgSteamDatagramSessionCryptInfo_EKeyType> {
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

    // optional fixed64 nonce = 3;

    pub fn clear_nonce(&mut self) {
        self.nonce = ::std::option::Option::None;
    }

    pub fn has_nonce(&self) -> bool {
        self.nonce.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: u64) {
        self.nonce = ::std::option::Option::Some(v);
    }

    pub fn get_nonce(&self) -> u64 {
        self.nonce.unwrap_or(0)
    }

    fn get_nonce_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.nonce
    }

    fn mut_nonce_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.nonce
    }

    // optional bool is_snp = 4;

    pub fn clear_is_snp(&mut self) {
        self.is_snp = ::std::option::Option::None;
    }

    pub fn has_is_snp(&self) -> bool {
        self.is_snp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_snp(&mut self, v: bool) {
        self.is_snp = ::std::option::Option::Some(v);
    }

    pub fn get_is_snp(&self) -> bool {
        self.is_snp.unwrap_or(false)
    }

    fn get_is_snp_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_snp
    }

    fn mut_is_snp_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_snp
    }
}

impl ::protobuf::Message for CMsgSteamDatagramSessionCryptInfo {
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
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.nonce = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_snp = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.nonce {
            my_size += 9;
        }
        if let Some(v) = self.is_snp {
            my_size += 2;
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
        if let Some(v) = self.nonce {
            os.write_fixed64(3, v)?;
        }
        if let Some(v) = self.is_snp {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramSessionCryptInfo {
    fn new() -> CMsgSteamDatagramSessionCryptInfo {
        CMsgSteamDatagramSessionCryptInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramSessionCryptInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgSteamDatagramSessionCryptInfo_EKeyType>>(
                    "key_type",
                    CMsgSteamDatagramSessionCryptInfo::get_key_type_for_reflect,
                    CMsgSteamDatagramSessionCryptInfo::mut_key_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key_data",
                    CMsgSteamDatagramSessionCryptInfo::get_key_data_for_reflect,
                    CMsgSteamDatagramSessionCryptInfo::mut_key_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "nonce",
                    CMsgSteamDatagramSessionCryptInfo::get_nonce_for_reflect,
                    CMsgSteamDatagramSessionCryptInfo::mut_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_snp",
                    CMsgSteamDatagramSessionCryptInfo::get_is_snp_for_reflect,
                    CMsgSteamDatagramSessionCryptInfo::mut_is_snp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramSessionCryptInfo>(
                    "CMsgSteamDatagramSessionCryptInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramSessionCryptInfo {
    fn clear(&mut self) {
        self.clear_key_type();
        self.clear_key_data();
        self.clear_nonce();
        self.clear_is_snp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramSessionCryptInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramSessionCryptInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgSteamDatagramSessionCryptInfo_EKeyType {
    INVALID = 0,
    CURVE25519 = 1,
}

impl ::protobuf::ProtobufEnum for CMsgSteamDatagramSessionCryptInfo_EKeyType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgSteamDatagramSessionCryptInfo_EKeyType> {
        match value {
            0 => ::std::option::Option::Some(CMsgSteamDatagramSessionCryptInfo_EKeyType::INVALID),
            1 => ::std::option::Option::Some(CMsgSteamDatagramSessionCryptInfo_EKeyType::CURVE25519),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgSteamDatagramSessionCryptInfo_EKeyType] = &[
            CMsgSteamDatagramSessionCryptInfo_EKeyType::INVALID,
            CMsgSteamDatagramSessionCryptInfo_EKeyType::CURVE25519,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CMsgSteamDatagramSessionCryptInfo_EKeyType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgSteamDatagramSessionCryptInfo_EKeyType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgSteamDatagramSessionCryptInfo_EKeyType {
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramSessionCryptInfo_EKeyType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramSessionCryptInfoSigned {
    // message fields
    info: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramSessionCryptInfoSigned {}

impl CMsgSteamDatagramSessionCryptInfoSigned {
    pub fn new() -> CMsgSteamDatagramSessionCryptInfoSigned {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramSessionCryptInfoSigned {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramSessionCryptInfoSigned> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramSessionCryptInfoSigned,
        };
        unsafe {
            instance.get(CMsgSteamDatagramSessionCryptInfoSigned::new)
        }
    }

    // optional bytes info = 1;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: ::std::vec::Vec<u8>) {
        self.info = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.info.is_none() {
            self.info.set_default();
        }
        self.info.as_mut().unwrap()
    }

    // Take field
    pub fn take_info(&mut self) -> ::std::vec::Vec<u8> {
        self.info.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_info(&self) -> &[u8] {
        match self.info.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_info_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.info
    }

    fn mut_info_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.info
    }

    // optional bytes signature = 2;

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

impl ::protobuf::Message for CMsgSteamDatagramSessionCryptInfoSigned {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.info)?;
                },
                2 => {
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
        if let Some(ref v) = self.info.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.info.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.signature.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramSessionCryptInfoSigned {
    fn new() -> CMsgSteamDatagramSessionCryptInfoSigned {
        CMsgSteamDatagramSessionCryptInfoSigned::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramSessionCryptInfoSigned>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "info",
                    CMsgSteamDatagramSessionCryptInfoSigned::get_info_for_reflect,
                    CMsgSteamDatagramSessionCryptInfoSigned::mut_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    CMsgSteamDatagramSessionCryptInfoSigned::get_signature_for_reflect,
                    CMsgSteamDatagramSessionCryptInfoSigned::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramSessionCryptInfoSigned>(
                    "CMsgSteamDatagramSessionCryptInfoSigned",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramSessionCryptInfoSigned {
    fn clear(&mut self) {
        self.clear_info();
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramSessionCryptInfoSigned {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramSessionCryptInfoSigned {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramGameserverSessionRequest {
    // message fields
    ticket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    challenge_time: ::std::option::Option<u32>,
    challenge: ::std::option::Option<u64>,
    client_connection_id: ::std::option::Option<u32>,
    server_connection_id: ::std::option::Option<u32>,
    network_config_version: ::std::option::Option<u32>,
    protocol_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramGameserverSessionRequest {}

impl CMsgSteamDatagramGameserverSessionRequest {
    pub fn new() -> CMsgSteamDatagramGameserverSessionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramGameserverSessionRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramGameserverSessionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramGameserverSessionRequest,
        };
        unsafe {
            instance.get(CMsgSteamDatagramGameserverSessionRequest::new)
        }
    }

    // optional bytes ticket = 1;

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

    // optional fixed32 challenge_time = 3;

    pub fn clear_challenge_time(&mut self) {
        self.challenge_time = ::std::option::Option::None;
    }

    pub fn has_challenge_time(&self) -> bool {
        self.challenge_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge_time(&mut self, v: u32) {
        self.challenge_time = ::std::option::Option::Some(v);
    }

    pub fn get_challenge_time(&self) -> u32 {
        self.challenge_time.unwrap_or(0)
    }

    fn get_challenge_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.challenge_time
    }

    fn mut_challenge_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.challenge_time
    }

    // optional fixed64 challenge = 4;

    pub fn clear_challenge(&mut self) {
        self.challenge = ::std::option::Option::None;
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: u64) {
        self.challenge = ::std::option::Option::Some(v);
    }

    pub fn get_challenge(&self) -> u64 {
        self.challenge.unwrap_or(0)
    }

    fn get_challenge_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.challenge
    }

    fn mut_challenge_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.challenge
    }

    // optional fixed32 client_connection_id = 5;

    pub fn clear_client_connection_id(&mut self) {
        self.client_connection_id = ::std::option::Option::None;
    }

    pub fn has_client_connection_id(&self) -> bool {
        self.client_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_connection_id(&mut self, v: u32) {
        self.client_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_connection_id(&self) -> u32 {
        self.client_connection_id.unwrap_or(0)
    }

    fn get_client_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_connection_id
    }

    fn mut_client_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_connection_id
    }

    // optional fixed32 server_connection_id = 8;

    pub fn clear_server_connection_id(&mut self) {
        self.server_connection_id = ::std::option::Option::None;
    }

    pub fn has_server_connection_id(&self) -> bool {
        self.server_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_connection_id(&mut self, v: u32) {
        self.server_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_server_connection_id(&self) -> u32 {
        self.server_connection_id.unwrap_or(0)
    }

    fn get_server_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_connection_id
    }

    fn mut_server_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_connection_id
    }

    // optional uint32 network_config_version = 6;

    pub fn clear_network_config_version(&mut self) {
        self.network_config_version = ::std::option::Option::None;
    }

    pub fn has_network_config_version(&self) -> bool {
        self.network_config_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_network_config_version(&mut self, v: u32) {
        self.network_config_version = ::std::option::Option::Some(v);
    }

    pub fn get_network_config_version(&self) -> u32 {
        self.network_config_version.unwrap_or(0)
    }

    fn get_network_config_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.network_config_version
    }

    fn mut_network_config_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.network_config_version
    }

    // optional uint32 protocol_version = 7;

    pub fn clear_protocol_version(&mut self) {
        self.protocol_version = ::std::option::Option::None;
    }

    pub fn has_protocol_version(&self) -> bool {
        self.protocol_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol_version(&mut self, v: u32) {
        self.protocol_version = ::std::option::Option::Some(v);
    }

    pub fn get_protocol_version(&self) -> u32 {
        self.protocol_version.unwrap_or(0)
    }

    fn get_protocol_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.protocol_version
    }

    fn mut_protocol_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.protocol_version
    }
}

impl ::protobuf::Message for CMsgSteamDatagramGameserverSessionRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ticket)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.challenge_time = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.challenge = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.client_connection_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.server_connection_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.network_config_version = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.protocol_version = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.ticket.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.challenge_time {
            my_size += 5;
        }
        if let Some(v) = self.challenge {
            my_size += 9;
        }
        if let Some(v) = self.client_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.server_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.network_config_version {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.protocol_version {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ticket.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(v) = self.challenge_time {
            os.write_fixed32(3, v)?;
        }
        if let Some(v) = self.challenge {
            os.write_fixed64(4, v)?;
        }
        if let Some(v) = self.client_connection_id {
            os.write_fixed32(5, v)?;
        }
        if let Some(v) = self.server_connection_id {
            os.write_fixed32(8, v)?;
        }
        if let Some(v) = self.network_config_version {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.protocol_version {
            os.write_uint32(7, v)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramGameserverSessionRequest {
    fn new() -> CMsgSteamDatagramGameserverSessionRequest {
        CMsgSteamDatagramGameserverSessionRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramGameserverSessionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ticket",
                    CMsgSteamDatagramGameserverSessionRequest::get_ticket_for_reflect,
                    CMsgSteamDatagramGameserverSessionRequest::mut_ticket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "challenge_time",
                    CMsgSteamDatagramGameserverSessionRequest::get_challenge_time_for_reflect,
                    CMsgSteamDatagramGameserverSessionRequest::mut_challenge_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "challenge",
                    CMsgSteamDatagramGameserverSessionRequest::get_challenge_for_reflect,
                    CMsgSteamDatagramGameserverSessionRequest::mut_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_connection_id",
                    CMsgSteamDatagramGameserverSessionRequest::get_client_connection_id_for_reflect,
                    CMsgSteamDatagramGameserverSessionRequest::mut_client_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "server_connection_id",
                    CMsgSteamDatagramGameserverSessionRequest::get_server_connection_id_for_reflect,
                    CMsgSteamDatagramGameserverSessionRequest::mut_server_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "network_config_version",
                    CMsgSteamDatagramGameserverSessionRequest::get_network_config_version_for_reflect,
                    CMsgSteamDatagramGameserverSessionRequest::mut_network_config_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "protocol_version",
                    CMsgSteamDatagramGameserverSessionRequest::get_protocol_version_for_reflect,
                    CMsgSteamDatagramGameserverSessionRequest::mut_protocol_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramGameserverSessionRequest>(
                    "CMsgSteamDatagramGameserverSessionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramGameserverSessionRequest {
    fn clear(&mut self) {
        self.clear_ticket();
        self.clear_challenge_time();
        self.clear_challenge();
        self.clear_client_connection_id();
        self.clear_server_connection_id();
        self.clear_network_config_version();
        self.clear_protocol_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramGameserverSessionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramGameserverSessionRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramGameserverSessionEstablished {
    // message fields
    connection_id: ::std::option::Option<u32>,
    gameserver_steam_id: ::std::option::Option<u64>,
    seconds_until_shutdown: ::std::option::Option<u32>,
    relay_session_id: ::std::option::Option<u32>,
    seq_num_r2c: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramGameserverSessionEstablished {}

impl CMsgSteamDatagramGameserverSessionEstablished {
    pub fn new() -> CMsgSteamDatagramGameserverSessionEstablished {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramGameserverSessionEstablished {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramGameserverSessionEstablished> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramGameserverSessionEstablished,
        };
        unsafe {
            instance.get(CMsgSteamDatagramGameserverSessionEstablished::new)
        }
    }

    // optional fixed32 connection_id = 1;

    pub fn clear_connection_id(&mut self) {
        self.connection_id = ::std::option::Option::None;
    }

    pub fn has_connection_id(&self) -> bool {
        self.connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connection_id(&mut self, v: u32) {
        self.connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_connection_id(&self) -> u32 {
        self.connection_id.unwrap_or(0)
    }

    fn get_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.connection_id
    }

    fn mut_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.connection_id
    }

    // optional fixed64 gameserver_steam_id = 3;

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

    // optional uint32 seconds_until_shutdown = 4;

    pub fn clear_seconds_until_shutdown(&mut self) {
        self.seconds_until_shutdown = ::std::option::Option::None;
    }

    pub fn has_seconds_until_shutdown(&self) -> bool {
        self.seconds_until_shutdown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seconds_until_shutdown(&mut self, v: u32) {
        self.seconds_until_shutdown = ::std::option::Option::Some(v);
    }

    pub fn get_seconds_until_shutdown(&self) -> u32 {
        self.seconds_until_shutdown.unwrap_or(0)
    }

    fn get_seconds_until_shutdown_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seconds_until_shutdown
    }

    fn mut_seconds_until_shutdown_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seconds_until_shutdown
    }

    // optional uint32 relay_session_id = 5;

    pub fn clear_relay_session_id(&mut self) {
        self.relay_session_id = ::std::option::Option::None;
    }

    pub fn has_relay_session_id(&self) -> bool {
        self.relay_session_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_relay_session_id(&mut self, v: u32) {
        self.relay_session_id = ::std::option::Option::Some(v);
    }

    pub fn get_relay_session_id(&self) -> u32 {
        self.relay_session_id.unwrap_or(0)
    }

    fn get_relay_session_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.relay_session_id
    }

    fn mut_relay_session_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.relay_session_id
    }

    // optional uint32 seq_num_r2c = 6;

    pub fn clear_seq_num_r2c(&mut self) {
        self.seq_num_r2c = ::std::option::Option::None;
    }

    pub fn has_seq_num_r2c(&self) -> bool {
        self.seq_num_r2c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_r2c(&mut self, v: u32) {
        self.seq_num_r2c = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_r2c(&self) -> u32 {
        self.seq_num_r2c.unwrap_or(0)
    }

    fn get_seq_num_r2c_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_r2c
    }

    fn mut_seq_num_r2c_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_r2c
    }
}

impl ::protobuf::Message for CMsgSteamDatagramGameserverSessionEstablished {
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
                    self.connection_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.gameserver_steam_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seconds_until_shutdown = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.relay_session_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seq_num_r2c = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.connection_id {
            my_size += 5;
        }
        if let Some(v) = self.gameserver_steam_id {
            my_size += 9;
        }
        if let Some(v) = self.seconds_until_shutdown {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.relay_session_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.seq_num_r2c {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.connection_id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.gameserver_steam_id {
            os.write_fixed64(3, v)?;
        }
        if let Some(v) = self.seconds_until_shutdown {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.relay_session_id {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.seq_num_r2c {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramGameserverSessionEstablished {
    fn new() -> CMsgSteamDatagramGameserverSessionEstablished {
        CMsgSteamDatagramGameserverSessionEstablished::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramGameserverSessionEstablished>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "connection_id",
                    CMsgSteamDatagramGameserverSessionEstablished::get_connection_id_for_reflect,
                    CMsgSteamDatagramGameserverSessionEstablished::mut_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "gameserver_steam_id",
                    CMsgSteamDatagramGameserverSessionEstablished::get_gameserver_steam_id_for_reflect,
                    CMsgSteamDatagramGameserverSessionEstablished::mut_gameserver_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seconds_until_shutdown",
                    CMsgSteamDatagramGameserverSessionEstablished::get_seconds_until_shutdown_for_reflect,
                    CMsgSteamDatagramGameserverSessionEstablished::mut_seconds_until_shutdown_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "relay_session_id",
                    CMsgSteamDatagramGameserverSessionEstablished::get_relay_session_id_for_reflect,
                    CMsgSteamDatagramGameserverSessionEstablished::mut_relay_session_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_r2c",
                    CMsgSteamDatagramGameserverSessionEstablished::get_seq_num_r2c_for_reflect,
                    CMsgSteamDatagramGameserverSessionEstablished::mut_seq_num_r2c_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramGameserverSessionEstablished>(
                    "CMsgSteamDatagramGameserverSessionEstablished",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramGameserverSessionEstablished {
    fn clear(&mut self) {
        self.clear_connection_id();
        self.clear_gameserver_steam_id();
        self.clear_seconds_until_shutdown();
        self.clear_relay_session_id();
        self.clear_seq_num_r2c();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramGameserverSessionEstablished {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramGameserverSessionEstablished {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramNoSessionRelayToClient {
    // message fields
    relay_session_id: ::std::option::Option<u32>,
    connection_id: ::std::option::Option<u32>,
    your_public_ip: ::std::option::Option<u32>,
    server_time: ::std::option::Option<u32>,
    challenge: ::std::option::Option<u64>,
    seconds_until_shutdown: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramNoSessionRelayToClient {}

impl CMsgSteamDatagramNoSessionRelayToClient {
    pub fn new() -> CMsgSteamDatagramNoSessionRelayToClient {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramNoSessionRelayToClient {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramNoSessionRelayToClient> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramNoSessionRelayToClient,
        };
        unsafe {
            instance.get(CMsgSteamDatagramNoSessionRelayToClient::new)
        }
    }

    // optional uint32 relay_session_id = 1;

    pub fn clear_relay_session_id(&mut self) {
        self.relay_session_id = ::std::option::Option::None;
    }

    pub fn has_relay_session_id(&self) -> bool {
        self.relay_session_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_relay_session_id(&mut self, v: u32) {
        self.relay_session_id = ::std::option::Option::Some(v);
    }

    pub fn get_relay_session_id(&self) -> u32 {
        self.relay_session_id.unwrap_or(0)
    }

    fn get_relay_session_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.relay_session_id
    }

    fn mut_relay_session_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.relay_session_id
    }

    // optional fixed32 connection_id = 7;

    pub fn clear_connection_id(&mut self) {
        self.connection_id = ::std::option::Option::None;
    }

    pub fn has_connection_id(&self) -> bool {
        self.connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connection_id(&mut self, v: u32) {
        self.connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_connection_id(&self) -> u32 {
        self.connection_id.unwrap_or(0)
    }

    fn get_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.connection_id
    }

    fn mut_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.connection_id
    }

    // optional fixed32 your_public_ip = 2;

    pub fn clear_your_public_ip(&mut self) {
        self.your_public_ip = ::std::option::Option::None;
    }

    pub fn has_your_public_ip(&self) -> bool {
        self.your_public_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_your_public_ip(&mut self, v: u32) {
        self.your_public_ip = ::std::option::Option::Some(v);
    }

    pub fn get_your_public_ip(&self) -> u32 {
        self.your_public_ip.unwrap_or(0)
    }

    fn get_your_public_ip_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.your_public_ip
    }

    fn mut_your_public_ip_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.your_public_ip
    }

    // optional fixed32 server_time = 3;

    pub fn clear_server_time(&mut self) {
        self.server_time = ::std::option::Option::None;
    }

    pub fn has_server_time(&self) -> bool {
        self.server_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_time(&mut self, v: u32) {
        self.server_time = ::std::option::Option::Some(v);
    }

    pub fn get_server_time(&self) -> u32 {
        self.server_time.unwrap_or(0)
    }

    fn get_server_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_time
    }

    fn mut_server_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_time
    }

    // optional fixed64 challenge = 4;

    pub fn clear_challenge(&mut self) {
        self.challenge = ::std::option::Option::None;
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: u64) {
        self.challenge = ::std::option::Option::Some(v);
    }

    pub fn get_challenge(&self) -> u64 {
        self.challenge.unwrap_or(0)
    }

    fn get_challenge_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.challenge
    }

    fn mut_challenge_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.challenge
    }

    // optional uint32 seconds_until_shutdown = 5;

    pub fn clear_seconds_until_shutdown(&mut self) {
        self.seconds_until_shutdown = ::std::option::Option::None;
    }

    pub fn has_seconds_until_shutdown(&self) -> bool {
        self.seconds_until_shutdown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seconds_until_shutdown(&mut self, v: u32) {
        self.seconds_until_shutdown = ::std::option::Option::Some(v);
    }

    pub fn get_seconds_until_shutdown(&self) -> u32 {
        self.seconds_until_shutdown.unwrap_or(0)
    }

    fn get_seconds_until_shutdown_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seconds_until_shutdown
    }

    fn mut_seconds_until_shutdown_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seconds_until_shutdown
    }
}

impl ::protobuf::Message for CMsgSteamDatagramNoSessionRelayToClient {
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
                    self.relay_session_id = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.connection_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.your_public_ip = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.server_time = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.challenge = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seconds_until_shutdown = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.relay_session_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.connection_id {
            my_size += 5;
        }
        if let Some(v) = self.your_public_ip {
            my_size += 5;
        }
        if let Some(v) = self.server_time {
            my_size += 5;
        }
        if let Some(v) = self.challenge {
            my_size += 9;
        }
        if let Some(v) = self.seconds_until_shutdown {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.relay_session_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.connection_id {
            os.write_fixed32(7, v)?;
        }
        if let Some(v) = self.your_public_ip {
            os.write_fixed32(2, v)?;
        }
        if let Some(v) = self.server_time {
            os.write_fixed32(3, v)?;
        }
        if let Some(v) = self.challenge {
            os.write_fixed64(4, v)?;
        }
        if let Some(v) = self.seconds_until_shutdown {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramNoSessionRelayToClient {
    fn new() -> CMsgSteamDatagramNoSessionRelayToClient {
        CMsgSteamDatagramNoSessionRelayToClient::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramNoSessionRelayToClient>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "relay_session_id",
                    CMsgSteamDatagramNoSessionRelayToClient::get_relay_session_id_for_reflect,
                    CMsgSteamDatagramNoSessionRelayToClient::mut_relay_session_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "connection_id",
                    CMsgSteamDatagramNoSessionRelayToClient::get_connection_id_for_reflect,
                    CMsgSteamDatagramNoSessionRelayToClient::mut_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "your_public_ip",
                    CMsgSteamDatagramNoSessionRelayToClient::get_your_public_ip_for_reflect,
                    CMsgSteamDatagramNoSessionRelayToClient::mut_your_public_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "server_time",
                    CMsgSteamDatagramNoSessionRelayToClient::get_server_time_for_reflect,
                    CMsgSteamDatagramNoSessionRelayToClient::mut_server_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "challenge",
                    CMsgSteamDatagramNoSessionRelayToClient::get_challenge_for_reflect,
                    CMsgSteamDatagramNoSessionRelayToClient::mut_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seconds_until_shutdown",
                    CMsgSteamDatagramNoSessionRelayToClient::get_seconds_until_shutdown_for_reflect,
                    CMsgSteamDatagramNoSessionRelayToClient::mut_seconds_until_shutdown_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramNoSessionRelayToClient>(
                    "CMsgSteamDatagramNoSessionRelayToClient",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramNoSessionRelayToClient {
    fn clear(&mut self) {
        self.clear_relay_session_id();
        self.clear_connection_id();
        self.clear_your_public_ip();
        self.clear_server_time();
        self.clear_challenge();
        self.clear_seconds_until_shutdown();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramNoSessionRelayToClient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramNoSessionRelayToClient {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramNoSessionRelayToServer {
    // message fields
    relay_session_id: ::std::option::Option<u32>,
    client_connection_id: ::std::option::Option<u32>,
    server_connection_id: ::std::option::Option<u32>,
    kludge_pad: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramNoSessionRelayToServer {}

impl CMsgSteamDatagramNoSessionRelayToServer {
    pub fn new() -> CMsgSteamDatagramNoSessionRelayToServer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramNoSessionRelayToServer {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramNoSessionRelayToServer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramNoSessionRelayToServer,
        };
        unsafe {
            instance.get(CMsgSteamDatagramNoSessionRelayToServer::new)
        }
    }

    // optional uint32 relay_session_id = 1;

    pub fn clear_relay_session_id(&mut self) {
        self.relay_session_id = ::std::option::Option::None;
    }

    pub fn has_relay_session_id(&self) -> bool {
        self.relay_session_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_relay_session_id(&mut self, v: u32) {
        self.relay_session_id = ::std::option::Option::Some(v);
    }

    pub fn get_relay_session_id(&self) -> u32 {
        self.relay_session_id.unwrap_or(0)
    }

    fn get_relay_session_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.relay_session_id
    }

    fn mut_relay_session_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.relay_session_id
    }

    // optional fixed32 client_connection_id = 7;

    pub fn clear_client_connection_id(&mut self) {
        self.client_connection_id = ::std::option::Option::None;
    }

    pub fn has_client_connection_id(&self) -> bool {
        self.client_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_connection_id(&mut self, v: u32) {
        self.client_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_connection_id(&self) -> u32 {
        self.client_connection_id.unwrap_or(0)
    }

    fn get_client_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_connection_id
    }

    fn mut_client_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_connection_id
    }

    // optional fixed32 server_connection_id = 8;

    pub fn clear_server_connection_id(&mut self) {
        self.server_connection_id = ::std::option::Option::None;
    }

    pub fn has_server_connection_id(&self) -> bool {
        self.server_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_connection_id(&mut self, v: u32) {
        self.server_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_server_connection_id(&self) -> u32 {
        self.server_connection_id.unwrap_or(0)
    }

    fn get_server_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_connection_id
    }

    fn mut_server_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_connection_id
    }

    // optional fixed64 kludge_pad = 99;

    pub fn clear_kludge_pad(&mut self) {
        self.kludge_pad = ::std::option::Option::None;
    }

    pub fn has_kludge_pad(&self) -> bool {
        self.kludge_pad.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kludge_pad(&mut self, v: u64) {
        self.kludge_pad = ::std::option::Option::Some(v);
    }

    pub fn get_kludge_pad(&self) -> u64 {
        self.kludge_pad.unwrap_or(0)
    }

    fn get_kludge_pad_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.kludge_pad
    }

    fn mut_kludge_pad_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.kludge_pad
    }
}

impl ::protobuf::Message for CMsgSteamDatagramNoSessionRelayToServer {
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
                    self.relay_session_id = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.client_connection_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.server_connection_id = ::std::option::Option::Some(tmp);
                },
                99 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.kludge_pad = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.relay_session_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.client_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.server_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.kludge_pad {
            my_size += 10;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.relay_session_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.client_connection_id {
            os.write_fixed32(7, v)?;
        }
        if let Some(v) = self.server_connection_id {
            os.write_fixed32(8, v)?;
        }
        if let Some(v) = self.kludge_pad {
            os.write_fixed64(99, v)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramNoSessionRelayToServer {
    fn new() -> CMsgSteamDatagramNoSessionRelayToServer {
        CMsgSteamDatagramNoSessionRelayToServer::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramNoSessionRelayToServer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "relay_session_id",
                    CMsgSteamDatagramNoSessionRelayToServer::get_relay_session_id_for_reflect,
                    CMsgSteamDatagramNoSessionRelayToServer::mut_relay_session_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_connection_id",
                    CMsgSteamDatagramNoSessionRelayToServer::get_client_connection_id_for_reflect,
                    CMsgSteamDatagramNoSessionRelayToServer::mut_client_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "server_connection_id",
                    CMsgSteamDatagramNoSessionRelayToServer::get_server_connection_id_for_reflect,
                    CMsgSteamDatagramNoSessionRelayToServer::mut_server_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "kludge_pad",
                    CMsgSteamDatagramNoSessionRelayToServer::get_kludge_pad_for_reflect,
                    CMsgSteamDatagramNoSessionRelayToServer::mut_kludge_pad_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramNoSessionRelayToServer>(
                    "CMsgSteamDatagramNoSessionRelayToServer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramNoSessionRelayToServer {
    fn clear(&mut self) {
        self.clear_relay_session_id();
        self.clear_client_connection_id();
        self.clear_server_connection_id();
        self.clear_kludge_pad();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramNoSessionRelayToServer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramNoSessionRelayToServer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramDiagnostic {
    // message fields
    severity: ::std::option::Option<u32>,
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramDiagnostic {}

impl CMsgSteamDatagramDiagnostic {
    pub fn new() -> CMsgSteamDatagramDiagnostic {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramDiagnostic {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramDiagnostic> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramDiagnostic,
        };
        unsafe {
            instance.get(CMsgSteamDatagramDiagnostic::new)
        }
    }

    // optional uint32 severity = 1;

    pub fn clear_severity(&mut self) {
        self.severity = ::std::option::Option::None;
    }

    pub fn has_severity(&self) -> bool {
        self.severity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_severity(&mut self, v: u32) {
        self.severity = ::std::option::Option::Some(v);
    }

    pub fn get_severity(&self) -> u32 {
        self.severity.unwrap_or(0)
    }

    fn get_severity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.severity
    }

    fn mut_severity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.severity
    }

    // optional string text = 2;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        if self.text.is_none() {
            self.text.set_default();
        }
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        self.text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        match self.text.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_text_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.text
    }

    fn mut_text_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.text
    }
}

impl ::protobuf::Message for CMsgSteamDatagramDiagnostic {
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
                    self.severity = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text)?;
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
        if let Some(v) = self.severity {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.severity {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.text.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramDiagnostic {
    fn new() -> CMsgSteamDatagramDiagnostic {
        CMsgSteamDatagramDiagnostic::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramDiagnostic>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "severity",
                    CMsgSteamDatagramDiagnostic::get_severity_for_reflect,
                    CMsgSteamDatagramDiagnostic::mut_severity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    CMsgSteamDatagramDiagnostic::get_text_for_reflect,
                    CMsgSteamDatagramDiagnostic::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramDiagnostic>(
                    "CMsgSteamDatagramDiagnostic",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramDiagnostic {
    fn clear(&mut self) {
        self.clear_severity();
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramDiagnostic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramDiagnostic {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramDataCenterState {
    // message fields
    data_centers: ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_DataCenter>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramDataCenterState {}

impl CMsgSteamDatagramDataCenterState {
    pub fn new() -> CMsgSteamDatagramDataCenterState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramDataCenterState {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramDataCenterState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramDataCenterState,
        };
        unsafe {
            instance.get(CMsgSteamDatagramDataCenterState::new)
        }
    }

    // repeated .CMsgSteamDatagramDataCenterState.DataCenter data_centers = 1;

    pub fn clear_data_centers(&mut self) {
        self.data_centers.clear();
    }

    // Param is passed by value, moved
    pub fn set_data_centers(&mut self, v: ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_DataCenter>) {
        self.data_centers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_data_centers(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_DataCenter> {
        &mut self.data_centers
    }

    // Take field
    pub fn take_data_centers(&mut self) -> ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_DataCenter> {
        ::std::mem::replace(&mut self.data_centers, ::protobuf::RepeatedField::new())
    }

    pub fn get_data_centers(&self) -> &[CMsgSteamDatagramDataCenterState_DataCenter] {
        &self.data_centers
    }

    fn get_data_centers_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_DataCenter> {
        &self.data_centers
    }

    fn mut_data_centers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_DataCenter> {
        &mut self.data_centers
    }
}

impl ::protobuf::Message for CMsgSteamDatagramDataCenterState {
    fn is_initialized(&self) -> bool {
        for v in &self.data_centers {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.data_centers)?;
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
        for value in &self.data_centers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.data_centers {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramDataCenterState {
    fn new() -> CMsgSteamDatagramDataCenterState {
        CMsgSteamDatagramDataCenterState::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramDataCenterState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramDataCenterState_DataCenter>>(
                    "data_centers",
                    CMsgSteamDatagramDataCenterState::get_data_centers_for_reflect,
                    CMsgSteamDatagramDataCenterState::mut_data_centers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramDataCenterState>(
                    "CMsgSteamDatagramDataCenterState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramDataCenterState {
    fn clear(&mut self) {
        self.clear_data_centers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramDataCenterState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramDataCenterState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramDataCenterState_Server {
    // message fields
    address: ::protobuf::SingularField<::std::string::String>,
    ping_ms: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramDataCenterState_Server {}

impl CMsgSteamDatagramDataCenterState_Server {
    pub fn new() -> CMsgSteamDatagramDataCenterState_Server {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramDataCenterState_Server {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramDataCenterState_Server> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramDataCenterState_Server,
        };
        unsafe {
            instance.get(CMsgSteamDatagramDataCenterState_Server::new)
        }
    }

    // optional string address = 1;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        if self.address.is_none() {
            self.address.set_default();
        }
        self.address.as_mut().unwrap()
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        self.address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_address(&self) -> &str {
        match self.address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_address_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.address
    }

    // optional uint32 ping_ms = 2;

    pub fn clear_ping_ms(&mut self) {
        self.ping_ms = ::std::option::Option::None;
    }

    pub fn has_ping_ms(&self) -> bool {
        self.ping_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ms(&mut self, v: u32) {
        self.ping_ms = ::std::option::Option::Some(v);
    }

    pub fn get_ping_ms(&self) -> u32 {
        self.ping_ms.unwrap_or(0)
    }

    fn get_ping_ms_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_ms
    }

    fn mut_ping_ms_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_ms
    }
}

impl ::protobuf::Message for CMsgSteamDatagramDataCenterState_Server {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.address)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_ms = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.address.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.ping_ms {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.address.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.ping_ms {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramDataCenterState_Server {
    fn new() -> CMsgSteamDatagramDataCenterState_Server {
        CMsgSteamDatagramDataCenterState_Server::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramDataCenterState_Server>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address",
                    CMsgSteamDatagramDataCenterState_Server::get_address_for_reflect,
                    CMsgSteamDatagramDataCenterState_Server::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_ms",
                    CMsgSteamDatagramDataCenterState_Server::get_ping_ms_for_reflect,
                    CMsgSteamDatagramDataCenterState_Server::mut_ping_ms_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramDataCenterState_Server>(
                    "CMsgSteamDatagramDataCenterState_Server",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramDataCenterState_Server {
    fn clear(&mut self) {
        self.clear_address();
        self.clear_ping_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramDataCenterState_Server {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramDataCenterState_Server {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramDataCenterState_DataCenter {
    // message fields
    code: ::protobuf::SingularField<::std::string::String>,
    server_sample: ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_Server>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramDataCenterState_DataCenter {}

impl CMsgSteamDatagramDataCenterState_DataCenter {
    pub fn new() -> CMsgSteamDatagramDataCenterState_DataCenter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramDataCenterState_DataCenter {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramDataCenterState_DataCenter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramDataCenterState_DataCenter,
        };
        unsafe {
            instance.get(CMsgSteamDatagramDataCenterState_DataCenter::new)
        }
    }

    // optional string code = 1;

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

    // repeated .CMsgSteamDatagramDataCenterState.Server server_sample = 2;

    pub fn clear_server_sample(&mut self) {
        self.server_sample.clear();
    }

    // Param is passed by value, moved
    pub fn set_server_sample(&mut self, v: ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_Server>) {
        self.server_sample = v;
    }

    // Mutable pointer to the field.
    pub fn mut_server_sample(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_Server> {
        &mut self.server_sample
    }

    // Take field
    pub fn take_server_sample(&mut self) -> ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_Server> {
        ::std::mem::replace(&mut self.server_sample, ::protobuf::RepeatedField::new())
    }

    pub fn get_server_sample(&self) -> &[CMsgSteamDatagramDataCenterState_Server] {
        &self.server_sample
    }

    fn get_server_sample_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_Server> {
        &self.server_sample
    }

    fn mut_server_sample_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_Server> {
        &mut self.server_sample
    }
}

impl ::protobuf::Message for CMsgSteamDatagramDataCenterState_DataCenter {
    fn is_initialized(&self) -> bool {
        for v in &self.server_sample {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.code)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.server_sample)?;
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
        if let Some(ref v) = self.code.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.server_sample {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.code.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.server_sample {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramDataCenterState_DataCenter {
    fn new() -> CMsgSteamDatagramDataCenterState_DataCenter {
        CMsgSteamDatagramDataCenterState_DataCenter::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramDataCenterState_DataCenter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "code",
                    CMsgSteamDatagramDataCenterState_DataCenter::get_code_for_reflect,
                    CMsgSteamDatagramDataCenterState_DataCenter::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramDataCenterState_Server>>(
                    "server_sample",
                    CMsgSteamDatagramDataCenterState_DataCenter::get_server_sample_for_reflect,
                    CMsgSteamDatagramDataCenterState_DataCenter::mut_server_sample_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramDataCenterState_DataCenter>(
                    "CMsgSteamDatagramDataCenterState_DataCenter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramDataCenterState_DataCenter {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_server_sample();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramDataCenterState_DataCenter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramDataCenterState_DataCenter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramLinkInstantaneousStats {
    // message fields
    out_packets_per_sec_x10: ::std::option::Option<u32>,
    out_bytes_per_sec: ::std::option::Option<u32>,
    in_packets_per_sec_x10: ::std::option::Option<u32>,
    in_bytes_per_sec: ::std::option::Option<u32>,
    ping_ms: ::std::option::Option<u32>,
    packets_dropped_pct: ::std::option::Option<u32>,
    packets_weird_sequence_pct: ::std::option::Option<u32>,
    peak_jitter_usec: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramLinkInstantaneousStats {}

impl CMsgSteamDatagramLinkInstantaneousStats {
    pub fn new() -> CMsgSteamDatagramLinkInstantaneousStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramLinkInstantaneousStats {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramLinkInstantaneousStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramLinkInstantaneousStats,
        };
        unsafe {
            instance.get(CMsgSteamDatagramLinkInstantaneousStats::new)
        }
    }

    // optional uint32 out_packets_per_sec_x10 = 1;

    pub fn clear_out_packets_per_sec_x10(&mut self) {
        self.out_packets_per_sec_x10 = ::std::option::Option::None;
    }

    pub fn has_out_packets_per_sec_x10(&self) -> bool {
        self.out_packets_per_sec_x10.is_some()
    }

    // Param is passed by value, moved
    pub fn set_out_packets_per_sec_x10(&mut self, v: u32) {
        self.out_packets_per_sec_x10 = ::std::option::Option::Some(v);
    }

    pub fn get_out_packets_per_sec_x10(&self) -> u32 {
        self.out_packets_per_sec_x10.unwrap_or(0)
    }

    fn get_out_packets_per_sec_x10_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.out_packets_per_sec_x10
    }

    fn mut_out_packets_per_sec_x10_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.out_packets_per_sec_x10
    }

    // optional uint32 out_bytes_per_sec = 2;

    pub fn clear_out_bytes_per_sec(&mut self) {
        self.out_bytes_per_sec = ::std::option::Option::None;
    }

    pub fn has_out_bytes_per_sec(&self) -> bool {
        self.out_bytes_per_sec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_out_bytes_per_sec(&mut self, v: u32) {
        self.out_bytes_per_sec = ::std::option::Option::Some(v);
    }

    pub fn get_out_bytes_per_sec(&self) -> u32 {
        self.out_bytes_per_sec.unwrap_or(0)
    }

    fn get_out_bytes_per_sec_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.out_bytes_per_sec
    }

    fn mut_out_bytes_per_sec_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.out_bytes_per_sec
    }

    // optional uint32 in_packets_per_sec_x10 = 3;

    pub fn clear_in_packets_per_sec_x10(&mut self) {
        self.in_packets_per_sec_x10 = ::std::option::Option::None;
    }

    pub fn has_in_packets_per_sec_x10(&self) -> bool {
        self.in_packets_per_sec_x10.is_some()
    }

    // Param is passed by value, moved
    pub fn set_in_packets_per_sec_x10(&mut self, v: u32) {
        self.in_packets_per_sec_x10 = ::std::option::Option::Some(v);
    }

    pub fn get_in_packets_per_sec_x10(&self) -> u32 {
        self.in_packets_per_sec_x10.unwrap_or(0)
    }

    fn get_in_packets_per_sec_x10_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.in_packets_per_sec_x10
    }

    fn mut_in_packets_per_sec_x10_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.in_packets_per_sec_x10
    }

    // optional uint32 in_bytes_per_sec = 4;

    pub fn clear_in_bytes_per_sec(&mut self) {
        self.in_bytes_per_sec = ::std::option::Option::None;
    }

    pub fn has_in_bytes_per_sec(&self) -> bool {
        self.in_bytes_per_sec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_in_bytes_per_sec(&mut self, v: u32) {
        self.in_bytes_per_sec = ::std::option::Option::Some(v);
    }

    pub fn get_in_bytes_per_sec(&self) -> u32 {
        self.in_bytes_per_sec.unwrap_or(0)
    }

    fn get_in_bytes_per_sec_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.in_bytes_per_sec
    }

    fn mut_in_bytes_per_sec_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.in_bytes_per_sec
    }

    // optional uint32 ping_ms = 5;

    pub fn clear_ping_ms(&mut self) {
        self.ping_ms = ::std::option::Option::None;
    }

    pub fn has_ping_ms(&self) -> bool {
        self.ping_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ms(&mut self, v: u32) {
        self.ping_ms = ::std::option::Option::Some(v);
    }

    pub fn get_ping_ms(&self) -> u32 {
        self.ping_ms.unwrap_or(0)
    }

    fn get_ping_ms_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_ms
    }

    fn mut_ping_ms_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_ms
    }

    // optional uint32 packets_dropped_pct = 6;

    pub fn clear_packets_dropped_pct(&mut self) {
        self.packets_dropped_pct = ::std::option::Option::None;
    }

    pub fn has_packets_dropped_pct(&self) -> bool {
        self.packets_dropped_pct.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_dropped_pct(&mut self, v: u32) {
        self.packets_dropped_pct = ::std::option::Option::Some(v);
    }

    pub fn get_packets_dropped_pct(&self) -> u32 {
        self.packets_dropped_pct.unwrap_or(0)
    }

    fn get_packets_dropped_pct_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.packets_dropped_pct
    }

    fn mut_packets_dropped_pct_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.packets_dropped_pct
    }

    // optional uint32 packets_weird_sequence_pct = 7;

    pub fn clear_packets_weird_sequence_pct(&mut self) {
        self.packets_weird_sequence_pct = ::std::option::Option::None;
    }

    pub fn has_packets_weird_sequence_pct(&self) -> bool {
        self.packets_weird_sequence_pct.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_weird_sequence_pct(&mut self, v: u32) {
        self.packets_weird_sequence_pct = ::std::option::Option::Some(v);
    }

    pub fn get_packets_weird_sequence_pct(&self) -> u32 {
        self.packets_weird_sequence_pct.unwrap_or(0)
    }

    fn get_packets_weird_sequence_pct_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.packets_weird_sequence_pct
    }

    fn mut_packets_weird_sequence_pct_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.packets_weird_sequence_pct
    }

    // optional uint32 peak_jitter_usec = 8;

    pub fn clear_peak_jitter_usec(&mut self) {
        self.peak_jitter_usec = ::std::option::Option::None;
    }

    pub fn has_peak_jitter_usec(&self) -> bool {
        self.peak_jitter_usec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peak_jitter_usec(&mut self, v: u32) {
        self.peak_jitter_usec = ::std::option::Option::Some(v);
    }

    pub fn get_peak_jitter_usec(&self) -> u32 {
        self.peak_jitter_usec.unwrap_or(0)
    }

    fn get_peak_jitter_usec_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.peak_jitter_usec
    }

    fn mut_peak_jitter_usec_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.peak_jitter_usec
    }
}

impl ::protobuf::Message for CMsgSteamDatagramLinkInstantaneousStats {
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
                    self.out_packets_per_sec_x10 = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.out_bytes_per_sec = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.in_packets_per_sec_x10 = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.in_bytes_per_sec = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_ms = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.packets_dropped_pct = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.packets_weird_sequence_pct = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.peak_jitter_usec = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.out_packets_per_sec_x10 {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.out_bytes_per_sec {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.in_packets_per_sec_x10 {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.in_bytes_per_sec {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ping_ms {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.packets_dropped_pct {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.packets_weird_sequence_pct {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.peak_jitter_usec {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.out_packets_per_sec_x10 {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.out_bytes_per_sec {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.in_packets_per_sec_x10 {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.in_bytes_per_sec {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.ping_ms {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.packets_dropped_pct {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.packets_weird_sequence_pct {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.peak_jitter_usec {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramLinkInstantaneousStats {
    fn new() -> CMsgSteamDatagramLinkInstantaneousStats {
        CMsgSteamDatagramLinkInstantaneousStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramLinkInstantaneousStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "out_packets_per_sec_x10",
                    CMsgSteamDatagramLinkInstantaneousStats::get_out_packets_per_sec_x10_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_out_packets_per_sec_x10_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "out_bytes_per_sec",
                    CMsgSteamDatagramLinkInstantaneousStats::get_out_bytes_per_sec_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_out_bytes_per_sec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "in_packets_per_sec_x10",
                    CMsgSteamDatagramLinkInstantaneousStats::get_in_packets_per_sec_x10_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_in_packets_per_sec_x10_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "in_bytes_per_sec",
                    CMsgSteamDatagramLinkInstantaneousStats::get_in_bytes_per_sec_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_in_bytes_per_sec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_ms",
                    CMsgSteamDatagramLinkInstantaneousStats::get_ping_ms_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_ping_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "packets_dropped_pct",
                    CMsgSteamDatagramLinkInstantaneousStats::get_packets_dropped_pct_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_packets_dropped_pct_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "packets_weird_sequence_pct",
                    CMsgSteamDatagramLinkInstantaneousStats::get_packets_weird_sequence_pct_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_packets_weird_sequence_pct_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "peak_jitter_usec",
                    CMsgSteamDatagramLinkInstantaneousStats::get_peak_jitter_usec_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_peak_jitter_usec_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramLinkInstantaneousStats>(
                    "CMsgSteamDatagramLinkInstantaneousStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramLinkInstantaneousStats {
    fn clear(&mut self) {
        self.clear_out_packets_per_sec_x10();
        self.clear_out_bytes_per_sec();
        self.clear_in_packets_per_sec_x10();
        self.clear_in_bytes_per_sec();
        self.clear_ping_ms();
        self.clear_packets_dropped_pct();
        self.clear_packets_weird_sequence_pct();
        self.clear_peak_jitter_usec();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramLinkInstantaneousStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramLinkInstantaneousStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramLinkLifetimeStats {
    // message fields
    packets_sent: ::std::option::Option<u64>,
    kb_sent: ::std::option::Option<u64>,
    packets_recv: ::std::option::Option<u64>,
    kb_recv: ::std::option::Option<u64>,
    packets_recv_sequenced: ::std::option::Option<u64>,
    packets_recv_dropped: ::std::option::Option<u64>,
    packets_recv_out_of_order: ::std::option::Option<u64>,
    packets_recv_duplicate: ::std::option::Option<u64>,
    packets_recv_lurch: ::std::option::Option<u64>,
    quality_histogram_100: ::std::option::Option<u32>,
    quality_histogram_99: ::std::option::Option<u32>,
    quality_histogram_97: ::std::option::Option<u32>,
    quality_histogram_95: ::std::option::Option<u32>,
    quality_histogram_90: ::std::option::Option<u32>,
    quality_histogram_75: ::std::option::Option<u32>,
    quality_histogram_50: ::std::option::Option<u32>,
    quality_histogram_1: ::std::option::Option<u32>,
    quality_histogram_dead: ::std::option::Option<u32>,
    quality_ntile_2nd: ::std::option::Option<u32>,
    quality_ntile_5th: ::std::option::Option<u32>,
    quality_ntile_25th: ::std::option::Option<u32>,
    quality_ntile_50th: ::std::option::Option<u32>,
    ping_histogram_25: ::std::option::Option<u32>,
    ping_histogram_50: ::std::option::Option<u32>,
    ping_histogram_75: ::std::option::Option<u32>,
    ping_histogram_100: ::std::option::Option<u32>,
    ping_histogram_125: ::std::option::Option<u32>,
    ping_histogram_150: ::std::option::Option<u32>,
    ping_histogram_200: ::std::option::Option<u32>,
    ping_histogram_300: ::std::option::Option<u32>,
    ping_histogram_max: ::std::option::Option<u32>,
    ping_ntile_5th: ::std::option::Option<u32>,
    ping_ntile_50th: ::std::option::Option<u32>,
    ping_ntile_75th: ::std::option::Option<u32>,
    ping_ntile_95th: ::std::option::Option<u32>,
    ping_ntile_98th: ::std::option::Option<u32>,
    jitter_histogram_negligible: ::std::option::Option<u32>,
    jitter_histogram_1: ::std::option::Option<u32>,
    jitter_histogram_2: ::std::option::Option<u32>,
    jitter_histogram_5: ::std::option::Option<u32>,
    jitter_histogram_10: ::std::option::Option<u32>,
    jitter_histogram_20: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramLinkLifetimeStats {}

impl CMsgSteamDatagramLinkLifetimeStats {
    pub fn new() -> CMsgSteamDatagramLinkLifetimeStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramLinkLifetimeStats {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramLinkLifetimeStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramLinkLifetimeStats,
        };
        unsafe {
            instance.get(CMsgSteamDatagramLinkLifetimeStats::new)
        }
    }

    // optional uint64 packets_sent = 3;

    pub fn clear_packets_sent(&mut self) {
        self.packets_sent = ::std::option::Option::None;
    }

    pub fn has_packets_sent(&self) -> bool {
        self.packets_sent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_sent(&mut self, v: u64) {
        self.packets_sent = ::std::option::Option::Some(v);
    }

    pub fn get_packets_sent(&self) -> u64 {
        self.packets_sent.unwrap_or(0)
    }

    fn get_packets_sent_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.packets_sent
    }

    fn mut_packets_sent_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.packets_sent
    }

    // optional uint64 kb_sent = 4;

    pub fn clear_kb_sent(&mut self) {
        self.kb_sent = ::std::option::Option::None;
    }

    pub fn has_kb_sent(&self) -> bool {
        self.kb_sent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kb_sent(&mut self, v: u64) {
        self.kb_sent = ::std::option::Option::Some(v);
    }

    pub fn get_kb_sent(&self) -> u64 {
        self.kb_sent.unwrap_or(0)
    }

    fn get_kb_sent_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.kb_sent
    }

    fn mut_kb_sent_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.kb_sent
    }

    // optional uint64 packets_recv = 5;

    pub fn clear_packets_recv(&mut self) {
        self.packets_recv = ::std::option::Option::None;
    }

    pub fn has_packets_recv(&self) -> bool {
        self.packets_recv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_recv(&mut self, v: u64) {
        self.packets_recv = ::std::option::Option::Some(v);
    }

    pub fn get_packets_recv(&self) -> u64 {
        self.packets_recv.unwrap_or(0)
    }

    fn get_packets_recv_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.packets_recv
    }

    fn mut_packets_recv_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.packets_recv
    }

    // optional uint64 kb_recv = 6;

    pub fn clear_kb_recv(&mut self) {
        self.kb_recv = ::std::option::Option::None;
    }

    pub fn has_kb_recv(&self) -> bool {
        self.kb_recv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kb_recv(&mut self, v: u64) {
        self.kb_recv = ::std::option::Option::Some(v);
    }

    pub fn get_kb_recv(&self) -> u64 {
        self.kb_recv.unwrap_or(0)
    }

    fn get_kb_recv_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.kb_recv
    }

    fn mut_kb_recv_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.kb_recv
    }

    // optional uint64 packets_recv_sequenced = 7;

    pub fn clear_packets_recv_sequenced(&mut self) {
        self.packets_recv_sequenced = ::std::option::Option::None;
    }

    pub fn has_packets_recv_sequenced(&self) -> bool {
        self.packets_recv_sequenced.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_recv_sequenced(&mut self, v: u64) {
        self.packets_recv_sequenced = ::std::option::Option::Some(v);
    }

    pub fn get_packets_recv_sequenced(&self) -> u64 {
        self.packets_recv_sequenced.unwrap_or(0)
    }

    fn get_packets_recv_sequenced_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.packets_recv_sequenced
    }

    fn mut_packets_recv_sequenced_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.packets_recv_sequenced
    }

    // optional uint64 packets_recv_dropped = 8;

    pub fn clear_packets_recv_dropped(&mut self) {
        self.packets_recv_dropped = ::std::option::Option::None;
    }

    pub fn has_packets_recv_dropped(&self) -> bool {
        self.packets_recv_dropped.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_recv_dropped(&mut self, v: u64) {
        self.packets_recv_dropped = ::std::option::Option::Some(v);
    }

    pub fn get_packets_recv_dropped(&self) -> u64 {
        self.packets_recv_dropped.unwrap_or(0)
    }

    fn get_packets_recv_dropped_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.packets_recv_dropped
    }

    fn mut_packets_recv_dropped_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.packets_recv_dropped
    }

    // optional uint64 packets_recv_out_of_order = 9;

    pub fn clear_packets_recv_out_of_order(&mut self) {
        self.packets_recv_out_of_order = ::std::option::Option::None;
    }

    pub fn has_packets_recv_out_of_order(&self) -> bool {
        self.packets_recv_out_of_order.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_recv_out_of_order(&mut self, v: u64) {
        self.packets_recv_out_of_order = ::std::option::Option::Some(v);
    }

    pub fn get_packets_recv_out_of_order(&self) -> u64 {
        self.packets_recv_out_of_order.unwrap_or(0)
    }

    fn get_packets_recv_out_of_order_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.packets_recv_out_of_order
    }

    fn mut_packets_recv_out_of_order_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.packets_recv_out_of_order
    }

    // optional uint64 packets_recv_duplicate = 10;

    pub fn clear_packets_recv_duplicate(&mut self) {
        self.packets_recv_duplicate = ::std::option::Option::None;
    }

    pub fn has_packets_recv_duplicate(&self) -> bool {
        self.packets_recv_duplicate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_recv_duplicate(&mut self, v: u64) {
        self.packets_recv_duplicate = ::std::option::Option::Some(v);
    }

    pub fn get_packets_recv_duplicate(&self) -> u64 {
        self.packets_recv_duplicate.unwrap_or(0)
    }

    fn get_packets_recv_duplicate_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.packets_recv_duplicate
    }

    fn mut_packets_recv_duplicate_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.packets_recv_duplicate
    }

    // optional uint64 packets_recv_lurch = 11;

    pub fn clear_packets_recv_lurch(&mut self) {
        self.packets_recv_lurch = ::std::option::Option::None;
    }

    pub fn has_packets_recv_lurch(&self) -> bool {
        self.packets_recv_lurch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_recv_lurch(&mut self, v: u64) {
        self.packets_recv_lurch = ::std::option::Option::Some(v);
    }

    pub fn get_packets_recv_lurch(&self) -> u64 {
        self.packets_recv_lurch.unwrap_or(0)
    }

    fn get_packets_recv_lurch_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.packets_recv_lurch
    }

    fn mut_packets_recv_lurch_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.packets_recv_lurch
    }

    // optional uint32 quality_histogram_100 = 21;

    pub fn clear_quality_histogram_100(&mut self) {
        self.quality_histogram_100 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_100(&self) -> bool {
        self.quality_histogram_100.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_100(&mut self, v: u32) {
        self.quality_histogram_100 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_100(&self) -> u32 {
        self.quality_histogram_100.unwrap_or(0)
    }

    fn get_quality_histogram_100_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_100
    }

    fn mut_quality_histogram_100_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_100
    }

    // optional uint32 quality_histogram_99 = 22;

    pub fn clear_quality_histogram_99(&mut self) {
        self.quality_histogram_99 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_99(&self) -> bool {
        self.quality_histogram_99.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_99(&mut self, v: u32) {
        self.quality_histogram_99 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_99(&self) -> u32 {
        self.quality_histogram_99.unwrap_or(0)
    }

    fn get_quality_histogram_99_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_99
    }

    fn mut_quality_histogram_99_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_99
    }

    // optional uint32 quality_histogram_97 = 23;

    pub fn clear_quality_histogram_97(&mut self) {
        self.quality_histogram_97 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_97(&self) -> bool {
        self.quality_histogram_97.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_97(&mut self, v: u32) {
        self.quality_histogram_97 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_97(&self) -> u32 {
        self.quality_histogram_97.unwrap_or(0)
    }

    fn get_quality_histogram_97_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_97
    }

    fn mut_quality_histogram_97_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_97
    }

    // optional uint32 quality_histogram_95 = 24;

    pub fn clear_quality_histogram_95(&mut self) {
        self.quality_histogram_95 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_95(&self) -> bool {
        self.quality_histogram_95.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_95(&mut self, v: u32) {
        self.quality_histogram_95 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_95(&self) -> u32 {
        self.quality_histogram_95.unwrap_or(0)
    }

    fn get_quality_histogram_95_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_95
    }

    fn mut_quality_histogram_95_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_95
    }

    // optional uint32 quality_histogram_90 = 25;

    pub fn clear_quality_histogram_90(&mut self) {
        self.quality_histogram_90 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_90(&self) -> bool {
        self.quality_histogram_90.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_90(&mut self, v: u32) {
        self.quality_histogram_90 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_90(&self) -> u32 {
        self.quality_histogram_90.unwrap_or(0)
    }

    fn get_quality_histogram_90_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_90
    }

    fn mut_quality_histogram_90_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_90
    }

    // optional uint32 quality_histogram_75 = 26;

    pub fn clear_quality_histogram_75(&mut self) {
        self.quality_histogram_75 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_75(&self) -> bool {
        self.quality_histogram_75.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_75(&mut self, v: u32) {
        self.quality_histogram_75 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_75(&self) -> u32 {
        self.quality_histogram_75.unwrap_or(0)
    }

    fn get_quality_histogram_75_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_75
    }

    fn mut_quality_histogram_75_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_75
    }

    // optional uint32 quality_histogram_50 = 27;

    pub fn clear_quality_histogram_50(&mut self) {
        self.quality_histogram_50 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_50(&self) -> bool {
        self.quality_histogram_50.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_50(&mut self, v: u32) {
        self.quality_histogram_50 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_50(&self) -> u32 {
        self.quality_histogram_50.unwrap_or(0)
    }

    fn get_quality_histogram_50_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_50
    }

    fn mut_quality_histogram_50_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_50
    }

    // optional uint32 quality_histogram_1 = 28;

    pub fn clear_quality_histogram_1(&mut self) {
        self.quality_histogram_1 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_1(&self) -> bool {
        self.quality_histogram_1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_1(&mut self, v: u32) {
        self.quality_histogram_1 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_1(&self) -> u32 {
        self.quality_histogram_1.unwrap_or(0)
    }

    fn get_quality_histogram_1_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_1
    }

    fn mut_quality_histogram_1_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_1
    }

    // optional uint32 quality_histogram_dead = 29;

    pub fn clear_quality_histogram_dead(&mut self) {
        self.quality_histogram_dead = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_dead(&self) -> bool {
        self.quality_histogram_dead.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_dead(&mut self, v: u32) {
        self.quality_histogram_dead = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_dead(&self) -> u32 {
        self.quality_histogram_dead.unwrap_or(0)
    }

    fn get_quality_histogram_dead_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_dead
    }

    fn mut_quality_histogram_dead_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_dead
    }

    // optional uint32 quality_ntile_2nd = 30;

    pub fn clear_quality_ntile_2nd(&mut self) {
        self.quality_ntile_2nd = ::std::option::Option::None;
    }

    pub fn has_quality_ntile_2nd(&self) -> bool {
        self.quality_ntile_2nd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_ntile_2nd(&mut self, v: u32) {
        self.quality_ntile_2nd = ::std::option::Option::Some(v);
    }

    pub fn get_quality_ntile_2nd(&self) -> u32 {
        self.quality_ntile_2nd.unwrap_or(0)
    }

    fn get_quality_ntile_2nd_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_ntile_2nd
    }

    fn mut_quality_ntile_2nd_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_ntile_2nd
    }

    // optional uint32 quality_ntile_5th = 31;

    pub fn clear_quality_ntile_5th(&mut self) {
        self.quality_ntile_5th = ::std::option::Option::None;
    }

    pub fn has_quality_ntile_5th(&self) -> bool {
        self.quality_ntile_5th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_ntile_5th(&mut self, v: u32) {
        self.quality_ntile_5th = ::std::option::Option::Some(v);
    }

    pub fn get_quality_ntile_5th(&self) -> u32 {
        self.quality_ntile_5th.unwrap_or(0)
    }

    fn get_quality_ntile_5th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_ntile_5th
    }

    fn mut_quality_ntile_5th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_ntile_5th
    }

    // optional uint32 quality_ntile_25th = 32;

    pub fn clear_quality_ntile_25th(&mut self) {
        self.quality_ntile_25th = ::std::option::Option::None;
    }

    pub fn has_quality_ntile_25th(&self) -> bool {
        self.quality_ntile_25th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_ntile_25th(&mut self, v: u32) {
        self.quality_ntile_25th = ::std::option::Option::Some(v);
    }

    pub fn get_quality_ntile_25th(&self) -> u32 {
        self.quality_ntile_25th.unwrap_or(0)
    }

    fn get_quality_ntile_25th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_ntile_25th
    }

    fn mut_quality_ntile_25th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_ntile_25th
    }

    // optional uint32 quality_ntile_50th = 33;

    pub fn clear_quality_ntile_50th(&mut self) {
        self.quality_ntile_50th = ::std::option::Option::None;
    }

    pub fn has_quality_ntile_50th(&self) -> bool {
        self.quality_ntile_50th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_ntile_50th(&mut self, v: u32) {
        self.quality_ntile_50th = ::std::option::Option::Some(v);
    }

    pub fn get_quality_ntile_50th(&self) -> u32 {
        self.quality_ntile_50th.unwrap_or(0)
    }

    fn get_quality_ntile_50th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_ntile_50th
    }

    fn mut_quality_ntile_50th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_ntile_50th
    }

    // optional uint32 ping_histogram_25 = 41;

    pub fn clear_ping_histogram_25(&mut self) {
        self.ping_histogram_25 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_25(&self) -> bool {
        self.ping_histogram_25.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_25(&mut self, v: u32) {
        self.ping_histogram_25 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_25(&self) -> u32 {
        self.ping_histogram_25.unwrap_or(0)
    }

    fn get_ping_histogram_25_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_25
    }

    fn mut_ping_histogram_25_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_25
    }

    // optional uint32 ping_histogram_50 = 42;

    pub fn clear_ping_histogram_50(&mut self) {
        self.ping_histogram_50 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_50(&self) -> bool {
        self.ping_histogram_50.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_50(&mut self, v: u32) {
        self.ping_histogram_50 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_50(&self) -> u32 {
        self.ping_histogram_50.unwrap_or(0)
    }

    fn get_ping_histogram_50_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_50
    }

    fn mut_ping_histogram_50_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_50
    }

    // optional uint32 ping_histogram_75 = 43;

    pub fn clear_ping_histogram_75(&mut self) {
        self.ping_histogram_75 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_75(&self) -> bool {
        self.ping_histogram_75.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_75(&mut self, v: u32) {
        self.ping_histogram_75 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_75(&self) -> u32 {
        self.ping_histogram_75.unwrap_or(0)
    }

    fn get_ping_histogram_75_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_75
    }

    fn mut_ping_histogram_75_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_75
    }

    // optional uint32 ping_histogram_100 = 44;

    pub fn clear_ping_histogram_100(&mut self) {
        self.ping_histogram_100 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_100(&self) -> bool {
        self.ping_histogram_100.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_100(&mut self, v: u32) {
        self.ping_histogram_100 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_100(&self) -> u32 {
        self.ping_histogram_100.unwrap_or(0)
    }

    fn get_ping_histogram_100_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_100
    }

    fn mut_ping_histogram_100_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_100
    }

    // optional uint32 ping_histogram_125 = 45;

    pub fn clear_ping_histogram_125(&mut self) {
        self.ping_histogram_125 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_125(&self) -> bool {
        self.ping_histogram_125.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_125(&mut self, v: u32) {
        self.ping_histogram_125 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_125(&self) -> u32 {
        self.ping_histogram_125.unwrap_or(0)
    }

    fn get_ping_histogram_125_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_125
    }

    fn mut_ping_histogram_125_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_125
    }

    // optional uint32 ping_histogram_150 = 46;

    pub fn clear_ping_histogram_150(&mut self) {
        self.ping_histogram_150 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_150(&self) -> bool {
        self.ping_histogram_150.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_150(&mut self, v: u32) {
        self.ping_histogram_150 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_150(&self) -> u32 {
        self.ping_histogram_150.unwrap_or(0)
    }

    fn get_ping_histogram_150_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_150
    }

    fn mut_ping_histogram_150_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_150
    }

    // optional uint32 ping_histogram_200 = 47;

    pub fn clear_ping_histogram_200(&mut self) {
        self.ping_histogram_200 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_200(&self) -> bool {
        self.ping_histogram_200.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_200(&mut self, v: u32) {
        self.ping_histogram_200 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_200(&self) -> u32 {
        self.ping_histogram_200.unwrap_or(0)
    }

    fn get_ping_histogram_200_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_200
    }

    fn mut_ping_histogram_200_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_200
    }

    // optional uint32 ping_histogram_300 = 48;

    pub fn clear_ping_histogram_300(&mut self) {
        self.ping_histogram_300 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_300(&self) -> bool {
        self.ping_histogram_300.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_300(&mut self, v: u32) {
        self.ping_histogram_300 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_300(&self) -> u32 {
        self.ping_histogram_300.unwrap_or(0)
    }

    fn get_ping_histogram_300_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_300
    }

    fn mut_ping_histogram_300_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_300
    }

    // optional uint32 ping_histogram_max = 49;

    pub fn clear_ping_histogram_max(&mut self) {
        self.ping_histogram_max = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_max(&self) -> bool {
        self.ping_histogram_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_max(&mut self, v: u32) {
        self.ping_histogram_max = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_max(&self) -> u32 {
        self.ping_histogram_max.unwrap_or(0)
    }

    fn get_ping_histogram_max_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_max
    }

    fn mut_ping_histogram_max_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_max
    }

    // optional uint32 ping_ntile_5th = 50;

    pub fn clear_ping_ntile_5th(&mut self) {
        self.ping_ntile_5th = ::std::option::Option::None;
    }

    pub fn has_ping_ntile_5th(&self) -> bool {
        self.ping_ntile_5th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ntile_5th(&mut self, v: u32) {
        self.ping_ntile_5th = ::std::option::Option::Some(v);
    }

    pub fn get_ping_ntile_5th(&self) -> u32 {
        self.ping_ntile_5th.unwrap_or(0)
    }

    fn get_ping_ntile_5th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_ntile_5th
    }

    fn mut_ping_ntile_5th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_ntile_5th
    }

    // optional uint32 ping_ntile_50th = 51;

    pub fn clear_ping_ntile_50th(&mut self) {
        self.ping_ntile_50th = ::std::option::Option::None;
    }

    pub fn has_ping_ntile_50th(&self) -> bool {
        self.ping_ntile_50th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ntile_50th(&mut self, v: u32) {
        self.ping_ntile_50th = ::std::option::Option::Some(v);
    }

    pub fn get_ping_ntile_50th(&self) -> u32 {
        self.ping_ntile_50th.unwrap_or(0)
    }

    fn get_ping_ntile_50th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_ntile_50th
    }

    fn mut_ping_ntile_50th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_ntile_50th
    }

    // optional uint32 ping_ntile_75th = 52;

    pub fn clear_ping_ntile_75th(&mut self) {
        self.ping_ntile_75th = ::std::option::Option::None;
    }

    pub fn has_ping_ntile_75th(&self) -> bool {
        self.ping_ntile_75th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ntile_75th(&mut self, v: u32) {
        self.ping_ntile_75th = ::std::option::Option::Some(v);
    }

    pub fn get_ping_ntile_75th(&self) -> u32 {
        self.ping_ntile_75th.unwrap_or(0)
    }

    fn get_ping_ntile_75th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_ntile_75th
    }

    fn mut_ping_ntile_75th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_ntile_75th
    }

    // optional uint32 ping_ntile_95th = 53;

    pub fn clear_ping_ntile_95th(&mut self) {
        self.ping_ntile_95th = ::std::option::Option::None;
    }

    pub fn has_ping_ntile_95th(&self) -> bool {
        self.ping_ntile_95th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ntile_95th(&mut self, v: u32) {
        self.ping_ntile_95th = ::std::option::Option::Some(v);
    }

    pub fn get_ping_ntile_95th(&self) -> u32 {
        self.ping_ntile_95th.unwrap_or(0)
    }

    fn get_ping_ntile_95th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_ntile_95th
    }

    fn mut_ping_ntile_95th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_ntile_95th
    }

    // optional uint32 ping_ntile_98th = 54;

    pub fn clear_ping_ntile_98th(&mut self) {
        self.ping_ntile_98th = ::std::option::Option::None;
    }

    pub fn has_ping_ntile_98th(&self) -> bool {
        self.ping_ntile_98th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ntile_98th(&mut self, v: u32) {
        self.ping_ntile_98th = ::std::option::Option::Some(v);
    }

    pub fn get_ping_ntile_98th(&self) -> u32 {
        self.ping_ntile_98th.unwrap_or(0)
    }

    fn get_ping_ntile_98th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_ntile_98th
    }

    fn mut_ping_ntile_98th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_ntile_98th
    }

    // optional uint32 jitter_histogram_negligible = 61;

    pub fn clear_jitter_histogram_negligible(&mut self) {
        self.jitter_histogram_negligible = ::std::option::Option::None;
    }

    pub fn has_jitter_histogram_negligible(&self) -> bool {
        self.jitter_histogram_negligible.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jitter_histogram_negligible(&mut self, v: u32) {
        self.jitter_histogram_negligible = ::std::option::Option::Some(v);
    }

    pub fn get_jitter_histogram_negligible(&self) -> u32 {
        self.jitter_histogram_negligible.unwrap_or(0)
    }

    fn get_jitter_histogram_negligible_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.jitter_histogram_negligible
    }

    fn mut_jitter_histogram_negligible_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.jitter_histogram_negligible
    }

    // optional uint32 jitter_histogram_1 = 62;

    pub fn clear_jitter_histogram_1(&mut self) {
        self.jitter_histogram_1 = ::std::option::Option::None;
    }

    pub fn has_jitter_histogram_1(&self) -> bool {
        self.jitter_histogram_1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jitter_histogram_1(&mut self, v: u32) {
        self.jitter_histogram_1 = ::std::option::Option::Some(v);
    }

    pub fn get_jitter_histogram_1(&self) -> u32 {
        self.jitter_histogram_1.unwrap_or(0)
    }

    fn get_jitter_histogram_1_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.jitter_histogram_1
    }

    fn mut_jitter_histogram_1_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.jitter_histogram_1
    }

    // optional uint32 jitter_histogram_2 = 63;

    pub fn clear_jitter_histogram_2(&mut self) {
        self.jitter_histogram_2 = ::std::option::Option::None;
    }

    pub fn has_jitter_histogram_2(&self) -> bool {
        self.jitter_histogram_2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jitter_histogram_2(&mut self, v: u32) {
        self.jitter_histogram_2 = ::std::option::Option::Some(v);
    }

    pub fn get_jitter_histogram_2(&self) -> u32 {
        self.jitter_histogram_2.unwrap_or(0)
    }

    fn get_jitter_histogram_2_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.jitter_histogram_2
    }

    fn mut_jitter_histogram_2_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.jitter_histogram_2
    }

    // optional uint32 jitter_histogram_5 = 64;

    pub fn clear_jitter_histogram_5(&mut self) {
        self.jitter_histogram_5 = ::std::option::Option::None;
    }

    pub fn has_jitter_histogram_5(&self) -> bool {
        self.jitter_histogram_5.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jitter_histogram_5(&mut self, v: u32) {
        self.jitter_histogram_5 = ::std::option::Option::Some(v);
    }

    pub fn get_jitter_histogram_5(&self) -> u32 {
        self.jitter_histogram_5.unwrap_or(0)
    }

    fn get_jitter_histogram_5_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.jitter_histogram_5
    }

    fn mut_jitter_histogram_5_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.jitter_histogram_5
    }

    // optional uint32 jitter_histogram_10 = 65;

    pub fn clear_jitter_histogram_10(&mut self) {
        self.jitter_histogram_10 = ::std::option::Option::None;
    }

    pub fn has_jitter_histogram_10(&self) -> bool {
        self.jitter_histogram_10.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jitter_histogram_10(&mut self, v: u32) {
        self.jitter_histogram_10 = ::std::option::Option::Some(v);
    }

    pub fn get_jitter_histogram_10(&self) -> u32 {
        self.jitter_histogram_10.unwrap_or(0)
    }

    fn get_jitter_histogram_10_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.jitter_histogram_10
    }

    fn mut_jitter_histogram_10_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.jitter_histogram_10
    }

    // optional uint32 jitter_histogram_20 = 66;

    pub fn clear_jitter_histogram_20(&mut self) {
        self.jitter_histogram_20 = ::std::option::Option::None;
    }

    pub fn has_jitter_histogram_20(&self) -> bool {
        self.jitter_histogram_20.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jitter_histogram_20(&mut self, v: u32) {
        self.jitter_histogram_20 = ::std::option::Option::Some(v);
    }

    pub fn get_jitter_histogram_20(&self) -> u32 {
        self.jitter_histogram_20.unwrap_or(0)
    }

    fn get_jitter_histogram_20_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.jitter_histogram_20
    }

    fn mut_jitter_histogram_20_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.jitter_histogram_20
    }
}

impl ::protobuf::Message for CMsgSteamDatagramLinkLifetimeStats {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.packets_sent = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.kb_sent = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.packets_recv = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.kb_recv = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.packets_recv_sequenced = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.packets_recv_dropped = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.packets_recv_out_of_order = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.packets_recv_duplicate = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.packets_recv_lurch = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_100 = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_99 = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_97 = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_95 = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_90 = ::std::option::Option::Some(tmp);
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_75 = ::std::option::Option::Some(tmp);
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_50 = ::std::option::Option::Some(tmp);
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_1 = ::std::option::Option::Some(tmp);
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_dead = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quality_ntile_2nd = ::std::option::Option::Some(tmp);
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quality_ntile_5th = ::std::option::Option::Some(tmp);
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quality_ntile_25th = ::std::option::Option::Some(tmp);
                },
                33 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quality_ntile_50th = ::std::option::Option::Some(tmp);
                },
                41 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_25 = ::std::option::Option::Some(tmp);
                },
                42 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_50 = ::std::option::Option::Some(tmp);
                },
                43 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_75 = ::std::option::Option::Some(tmp);
                },
                44 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_100 = ::std::option::Option::Some(tmp);
                },
                45 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_125 = ::std::option::Option::Some(tmp);
                },
                46 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_150 = ::std::option::Option::Some(tmp);
                },
                47 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_200 = ::std::option::Option::Some(tmp);
                },
                48 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_300 = ::std::option::Option::Some(tmp);
                },
                49 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_max = ::std::option::Option::Some(tmp);
                },
                50 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_ntile_5th = ::std::option::Option::Some(tmp);
                },
                51 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_ntile_50th = ::std::option::Option::Some(tmp);
                },
                52 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_ntile_75th = ::std::option::Option::Some(tmp);
                },
                53 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_ntile_95th = ::std::option::Option::Some(tmp);
                },
                54 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_ntile_98th = ::std::option::Option::Some(tmp);
                },
                61 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.jitter_histogram_negligible = ::std::option::Option::Some(tmp);
                },
                62 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.jitter_histogram_1 = ::std::option::Option::Some(tmp);
                },
                63 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.jitter_histogram_2 = ::std::option::Option::Some(tmp);
                },
                64 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.jitter_histogram_5 = ::std::option::Option::Some(tmp);
                },
                65 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.jitter_histogram_10 = ::std::option::Option::Some(tmp);
                },
                66 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.jitter_histogram_20 = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.packets_sent {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.kb_sent {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.packets_recv {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.kb_recv {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.packets_recv_sequenced {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.packets_recv_dropped {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.packets_recv_out_of_order {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.packets_recv_duplicate {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.packets_recv_lurch {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quality_histogram_100 {
            my_size += ::protobuf::rt::value_size(21, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quality_histogram_99 {
            my_size += ::protobuf::rt::value_size(22, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quality_histogram_97 {
            my_size += ::protobuf::rt::value_size(23, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quality_histogram_95 {
            my_size += ::protobuf::rt::value_size(24, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quality_histogram_90 {
            my_size += ::protobuf::rt::value_size(25, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quality_histogram_75 {
            my_size += ::protobuf::rt::value_size(26, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quality_histogram_50 {
            my_size += ::protobuf::rt::value_size(27, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quality_histogram_1 {
            my_size += ::protobuf::rt::value_size(28, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quality_histogram_dead {
            my_size += ::protobuf::rt::value_size(29, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quality_ntile_2nd {
            my_size += ::protobuf::rt::value_size(30, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quality_ntile_5th {
            my_size += ::protobuf::rt::value_size(31, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quality_ntile_25th {
            my_size += ::protobuf::rt::value_size(32, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quality_ntile_50th {
            my_size += ::protobuf::rt::value_size(33, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ping_histogram_25 {
            my_size += ::protobuf::rt::value_size(41, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ping_histogram_50 {
            my_size += ::protobuf::rt::value_size(42, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ping_histogram_75 {
            my_size += ::protobuf::rt::value_size(43, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ping_histogram_100 {
            my_size += ::protobuf::rt::value_size(44, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ping_histogram_125 {
            my_size += ::protobuf::rt::value_size(45, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ping_histogram_150 {
            my_size += ::protobuf::rt::value_size(46, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ping_histogram_200 {
            my_size += ::protobuf::rt::value_size(47, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ping_histogram_300 {
            my_size += ::protobuf::rt::value_size(48, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ping_histogram_max {
            my_size += ::protobuf::rt::value_size(49, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ping_ntile_5th {
            my_size += ::protobuf::rt::value_size(50, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ping_ntile_50th {
            my_size += ::protobuf::rt::value_size(51, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ping_ntile_75th {
            my_size += ::protobuf::rt::value_size(52, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ping_ntile_95th {
            my_size += ::protobuf::rt::value_size(53, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ping_ntile_98th {
            my_size += ::protobuf::rt::value_size(54, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.jitter_histogram_negligible {
            my_size += ::protobuf::rt::value_size(61, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.jitter_histogram_1 {
            my_size += ::protobuf::rt::value_size(62, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.jitter_histogram_2 {
            my_size += ::protobuf::rt::value_size(63, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.jitter_histogram_5 {
            my_size += ::protobuf::rt::value_size(64, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.jitter_histogram_10 {
            my_size += ::protobuf::rt::value_size(65, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.jitter_histogram_20 {
            my_size += ::protobuf::rt::value_size(66, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.packets_sent {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.kb_sent {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.packets_recv {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.kb_recv {
            os.write_uint64(6, v)?;
        }
        if let Some(v) = self.packets_recv_sequenced {
            os.write_uint64(7, v)?;
        }
        if let Some(v) = self.packets_recv_dropped {
            os.write_uint64(8, v)?;
        }
        if let Some(v) = self.packets_recv_out_of_order {
            os.write_uint64(9, v)?;
        }
        if let Some(v) = self.packets_recv_duplicate {
            os.write_uint64(10, v)?;
        }
        if let Some(v) = self.packets_recv_lurch {
            os.write_uint64(11, v)?;
        }
        if let Some(v) = self.quality_histogram_100 {
            os.write_uint32(21, v)?;
        }
        if let Some(v) = self.quality_histogram_99 {
            os.write_uint32(22, v)?;
        }
        if let Some(v) = self.quality_histogram_97 {
            os.write_uint32(23, v)?;
        }
        if let Some(v) = self.quality_histogram_95 {
            os.write_uint32(24, v)?;
        }
        if let Some(v) = self.quality_histogram_90 {
            os.write_uint32(25, v)?;
        }
        if let Some(v) = self.quality_histogram_75 {
            os.write_uint32(26, v)?;
        }
        if let Some(v) = self.quality_histogram_50 {
            os.write_uint32(27, v)?;
        }
        if let Some(v) = self.quality_histogram_1 {
            os.write_uint32(28, v)?;
        }
        if let Some(v) = self.quality_histogram_dead {
            os.write_uint32(29, v)?;
        }
        if let Some(v) = self.quality_ntile_2nd {
            os.write_uint32(30, v)?;
        }
        if let Some(v) = self.quality_ntile_5th {
            os.write_uint32(31, v)?;
        }
        if let Some(v) = self.quality_ntile_25th {
            os.write_uint32(32, v)?;
        }
        if let Some(v) = self.quality_ntile_50th {
            os.write_uint32(33, v)?;
        }
        if let Some(v) = self.ping_histogram_25 {
            os.write_uint32(41, v)?;
        }
        if let Some(v) = self.ping_histogram_50 {
            os.write_uint32(42, v)?;
        }
        if let Some(v) = self.ping_histogram_75 {
            os.write_uint32(43, v)?;
        }
        if let Some(v) = self.ping_histogram_100 {
            os.write_uint32(44, v)?;
        }
        if let Some(v) = self.ping_histogram_125 {
            os.write_uint32(45, v)?;
        }
        if let Some(v) = self.ping_histogram_150 {
            os.write_uint32(46, v)?;
        }
        if let Some(v) = self.ping_histogram_200 {
            os.write_uint32(47, v)?;
        }
        if let Some(v) = self.ping_histogram_300 {
            os.write_uint32(48, v)?;
        }
        if let Some(v) = self.ping_histogram_max {
            os.write_uint32(49, v)?;
        }
        if let Some(v) = self.ping_ntile_5th {
            os.write_uint32(50, v)?;
        }
        if let Some(v) = self.ping_ntile_50th {
            os.write_uint32(51, v)?;
        }
        if let Some(v) = self.ping_ntile_75th {
            os.write_uint32(52, v)?;
        }
        if let Some(v) = self.ping_ntile_95th {
            os.write_uint32(53, v)?;
        }
        if let Some(v) = self.ping_ntile_98th {
            os.write_uint32(54, v)?;
        }
        if let Some(v) = self.jitter_histogram_negligible {
            os.write_uint32(61, v)?;
        }
        if let Some(v) = self.jitter_histogram_1 {
            os.write_uint32(62, v)?;
        }
        if let Some(v) = self.jitter_histogram_2 {
            os.write_uint32(63, v)?;
        }
        if let Some(v) = self.jitter_histogram_5 {
            os.write_uint32(64, v)?;
        }
        if let Some(v) = self.jitter_histogram_10 {
            os.write_uint32(65, v)?;
        }
        if let Some(v) = self.jitter_histogram_20 {
            os.write_uint32(66, v)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramLinkLifetimeStats {
    fn new() -> CMsgSteamDatagramLinkLifetimeStats {
        CMsgSteamDatagramLinkLifetimeStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramLinkLifetimeStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "packets_sent",
                    CMsgSteamDatagramLinkLifetimeStats::get_packets_sent_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_packets_sent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "kb_sent",
                    CMsgSteamDatagramLinkLifetimeStats::get_kb_sent_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_kb_sent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "packets_recv",
                    CMsgSteamDatagramLinkLifetimeStats::get_packets_recv_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_packets_recv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "kb_recv",
                    CMsgSteamDatagramLinkLifetimeStats::get_kb_recv_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_kb_recv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "packets_recv_sequenced",
                    CMsgSteamDatagramLinkLifetimeStats::get_packets_recv_sequenced_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_packets_recv_sequenced_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "packets_recv_dropped",
                    CMsgSteamDatagramLinkLifetimeStats::get_packets_recv_dropped_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_packets_recv_dropped_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "packets_recv_out_of_order",
                    CMsgSteamDatagramLinkLifetimeStats::get_packets_recv_out_of_order_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_packets_recv_out_of_order_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "packets_recv_duplicate",
                    CMsgSteamDatagramLinkLifetimeStats::get_packets_recv_duplicate_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_packets_recv_duplicate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "packets_recv_lurch",
                    CMsgSteamDatagramLinkLifetimeStats::get_packets_recv_lurch_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_packets_recv_lurch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_100",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_100_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_100_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_99",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_99_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_99_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_97",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_97_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_97_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_95",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_95_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_95_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_90",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_90_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_90_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_75",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_75_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_75_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_50",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_50_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_50_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_1",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_1_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_dead",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_dead_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_dead_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_ntile_2nd",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_ntile_2nd_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_ntile_2nd_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_ntile_5th",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_ntile_5th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_ntile_5th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_ntile_25th",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_ntile_25th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_ntile_25th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_ntile_50th",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_ntile_50th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_ntile_50th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_25",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_25_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_25_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_50",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_50_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_50_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_75",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_75_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_75_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_100",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_100_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_100_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_125",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_125_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_125_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_150",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_150_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_150_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_200",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_200_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_200_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_300",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_300_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_300_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_max",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_max_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_max_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_ntile_5th",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_ntile_5th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_ntile_5th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_ntile_50th",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_ntile_50th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_ntile_50th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_ntile_75th",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_ntile_75th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_ntile_75th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_ntile_95th",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_ntile_95th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_ntile_95th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_ntile_98th",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_ntile_98th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_ntile_98th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "jitter_histogram_negligible",
                    CMsgSteamDatagramLinkLifetimeStats::get_jitter_histogram_negligible_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_jitter_histogram_negligible_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "jitter_histogram_1",
                    CMsgSteamDatagramLinkLifetimeStats::get_jitter_histogram_1_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_jitter_histogram_1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "jitter_histogram_2",
                    CMsgSteamDatagramLinkLifetimeStats::get_jitter_histogram_2_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_jitter_histogram_2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "jitter_histogram_5",
                    CMsgSteamDatagramLinkLifetimeStats::get_jitter_histogram_5_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_jitter_histogram_5_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "jitter_histogram_10",
                    CMsgSteamDatagramLinkLifetimeStats::get_jitter_histogram_10_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_jitter_histogram_10_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "jitter_histogram_20",
                    CMsgSteamDatagramLinkLifetimeStats::get_jitter_histogram_20_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_jitter_histogram_20_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramLinkLifetimeStats>(
                    "CMsgSteamDatagramLinkLifetimeStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramLinkLifetimeStats {
    fn clear(&mut self) {
        self.clear_packets_sent();
        self.clear_kb_sent();
        self.clear_packets_recv();
        self.clear_kb_recv();
        self.clear_packets_recv_sequenced();
        self.clear_packets_recv_dropped();
        self.clear_packets_recv_out_of_order();
        self.clear_packets_recv_duplicate();
        self.clear_packets_recv_lurch();
        self.clear_quality_histogram_100();
        self.clear_quality_histogram_99();
        self.clear_quality_histogram_97();
        self.clear_quality_histogram_95();
        self.clear_quality_histogram_90();
        self.clear_quality_histogram_75();
        self.clear_quality_histogram_50();
        self.clear_quality_histogram_1();
        self.clear_quality_histogram_dead();
        self.clear_quality_ntile_2nd();
        self.clear_quality_ntile_5th();
        self.clear_quality_ntile_25th();
        self.clear_quality_ntile_50th();
        self.clear_ping_histogram_25();
        self.clear_ping_histogram_50();
        self.clear_ping_histogram_75();
        self.clear_ping_histogram_100();
        self.clear_ping_histogram_125();
        self.clear_ping_histogram_150();
        self.clear_ping_histogram_200();
        self.clear_ping_histogram_300();
        self.clear_ping_histogram_max();
        self.clear_ping_ntile_5th();
        self.clear_ping_ntile_50th();
        self.clear_ping_ntile_75th();
        self.clear_ping_ntile_95th();
        self.clear_ping_ntile_98th();
        self.clear_jitter_histogram_negligible();
        self.clear_jitter_histogram_1();
        self.clear_jitter_histogram_2();
        self.clear_jitter_histogram_5();
        self.clear_jitter_histogram_10();
        self.clear_jitter_histogram_20();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramLinkLifetimeStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramLinkLifetimeStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramConnectionQuality {
    // message fields
    instantaneous: ::protobuf::SingularPtrField<CMsgSteamDatagramLinkInstantaneousStats>,
    lifetime: ::protobuf::SingularPtrField<CMsgSteamDatagramLinkLifetimeStats>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramConnectionQuality {}

impl CMsgSteamDatagramConnectionQuality {
    pub fn new() -> CMsgSteamDatagramConnectionQuality {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramConnectionQuality {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramConnectionQuality> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramConnectionQuality,
        };
        unsafe {
            instance.get(CMsgSteamDatagramConnectionQuality::new)
        }
    }

    // optional .CMsgSteamDatagramLinkInstantaneousStats instantaneous = 1;

    pub fn clear_instantaneous(&mut self) {
        self.instantaneous.clear();
    }

    pub fn has_instantaneous(&self) -> bool {
        self.instantaneous.is_some()
    }

    // Param is passed by value, moved
    pub fn set_instantaneous(&mut self, v: CMsgSteamDatagramLinkInstantaneousStats) {
        self.instantaneous = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_instantaneous(&mut self) -> &mut CMsgSteamDatagramLinkInstantaneousStats {
        if self.instantaneous.is_none() {
            self.instantaneous.set_default();
        }
        self.instantaneous.as_mut().unwrap()
    }

    // Take field
    pub fn take_instantaneous(&mut self) -> CMsgSteamDatagramLinkInstantaneousStats {
        self.instantaneous.take().unwrap_or_else(|| CMsgSteamDatagramLinkInstantaneousStats::new())
    }

    pub fn get_instantaneous(&self) -> &CMsgSteamDatagramLinkInstantaneousStats {
        self.instantaneous.as_ref().unwrap_or_else(|| CMsgSteamDatagramLinkInstantaneousStats::default_instance())
    }

    fn get_instantaneous_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramLinkInstantaneousStats> {
        &self.instantaneous
    }

    fn mut_instantaneous_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramLinkInstantaneousStats> {
        &mut self.instantaneous
    }

    // optional .CMsgSteamDatagramLinkLifetimeStats lifetime = 2;

    pub fn clear_lifetime(&mut self) {
        self.lifetime.clear();
    }

    pub fn has_lifetime(&self) -> bool {
        self.lifetime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lifetime(&mut self, v: CMsgSteamDatagramLinkLifetimeStats) {
        self.lifetime = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lifetime(&mut self) -> &mut CMsgSteamDatagramLinkLifetimeStats {
        if self.lifetime.is_none() {
            self.lifetime.set_default();
        }
        self.lifetime.as_mut().unwrap()
    }

    // Take field
    pub fn take_lifetime(&mut self) -> CMsgSteamDatagramLinkLifetimeStats {
        self.lifetime.take().unwrap_or_else(|| CMsgSteamDatagramLinkLifetimeStats::new())
    }

    pub fn get_lifetime(&self) -> &CMsgSteamDatagramLinkLifetimeStats {
        self.lifetime.as_ref().unwrap_or_else(|| CMsgSteamDatagramLinkLifetimeStats::default_instance())
    }

    fn get_lifetime_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramLinkLifetimeStats> {
        &self.lifetime
    }

    fn mut_lifetime_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramLinkLifetimeStats> {
        &mut self.lifetime
    }
}

impl ::protobuf::Message for CMsgSteamDatagramConnectionQuality {
    fn is_initialized(&self) -> bool {
        for v in &self.instantaneous {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.lifetime {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.instantaneous)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lifetime)?;
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
        if let Some(ref v) = self.instantaneous.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.lifetime.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.instantaneous.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.lifetime.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramConnectionQuality {
    fn new() -> CMsgSteamDatagramConnectionQuality {
        CMsgSteamDatagramConnectionQuality::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionQuality>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramLinkInstantaneousStats>>(
                    "instantaneous",
                    CMsgSteamDatagramConnectionQuality::get_instantaneous_for_reflect,
                    CMsgSteamDatagramConnectionQuality::mut_instantaneous_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramLinkLifetimeStats>>(
                    "lifetime",
                    CMsgSteamDatagramConnectionQuality::get_lifetime_for_reflect,
                    CMsgSteamDatagramConnectionQuality::mut_lifetime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramConnectionQuality>(
                    "CMsgSteamDatagramConnectionQuality",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramConnectionQuality {
    fn clear(&mut self) {
        self.clear_instantaneous();
        self.clear_lifetime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramConnectionQuality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionQuality {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramConnectionStatsClientToRouter {
    // message fields
    c2r: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    c2s: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    legacy_client_timestamp: ::std::option::Option<u32>,
    ack_relay: ::std::vec::Vec<u32>,
    ack_e2e: ::std::vec::Vec<u32>,
    flags: ::std::option::Option<u32>,
    client_connection_id: ::std::option::Option<u32>,
    seq_num_c2r: ::std::option::Option<u32>,
    seq_num_c2s: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramConnectionStatsClientToRouter {}

impl CMsgSteamDatagramConnectionStatsClientToRouter {
    pub fn new() -> CMsgSteamDatagramConnectionStatsClientToRouter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramConnectionStatsClientToRouter {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramConnectionStatsClientToRouter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramConnectionStatsClientToRouter,
        };
        unsafe {
            instance.get(CMsgSteamDatagramConnectionStatsClientToRouter::new)
        }
    }

    // optional .CMsgSteamDatagramConnectionQuality c2r = 1;

    pub fn clear_c2r(&mut self) {
        self.c2r.clear();
    }

    pub fn has_c2r(&self) -> bool {
        self.c2r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c2r(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.c2r = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_c2r(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.c2r.is_none() {
            self.c2r.set_default();
        }
        self.c2r.as_mut().unwrap()
    }

    // Take field
    pub fn take_c2r(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.c2r.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_c2r(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.c2r.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_c2r_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.c2r
    }

    fn mut_c2r_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.c2r
    }

    // optional .CMsgSteamDatagramConnectionQuality c2s = 2;

    pub fn clear_c2s(&mut self) {
        self.c2s.clear();
    }

    pub fn has_c2s(&self) -> bool {
        self.c2s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c2s(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.c2s = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_c2s(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.c2s.is_none() {
            self.c2s.set_default();
        }
        self.c2s.as_mut().unwrap()
    }

    // Take field
    pub fn take_c2s(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.c2s.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_c2s(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.c2s.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_c2s_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.c2s
    }

    fn mut_c2s_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.c2s
    }

    // optional fixed32 legacy_client_timestamp = 3;

    pub fn clear_legacy_client_timestamp(&mut self) {
        self.legacy_client_timestamp = ::std::option::Option::None;
    }

    pub fn has_legacy_client_timestamp(&self) -> bool {
        self.legacy_client_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_client_timestamp(&mut self, v: u32) {
        self.legacy_client_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_legacy_client_timestamp(&self) -> u32 {
        self.legacy_client_timestamp.unwrap_or(0)
    }

    fn get_legacy_client_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.legacy_client_timestamp
    }

    fn mut_legacy_client_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.legacy_client_timestamp
    }

    // repeated fixed32 ack_relay = 4;

    pub fn clear_ack_relay(&mut self) {
        self.ack_relay.clear();
    }

    // Param is passed by value, moved
    pub fn set_ack_relay(&mut self, v: ::std::vec::Vec<u32>) {
        self.ack_relay = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ack_relay(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_relay
    }

    // Take field
    pub fn take_ack_relay(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.ack_relay, ::std::vec::Vec::new())
    }

    pub fn get_ack_relay(&self) -> &[u32] {
        &self.ack_relay
    }

    fn get_ack_relay_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.ack_relay
    }

    fn mut_ack_relay_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_relay
    }

    // repeated fixed32 ack_e2e = 5;

    pub fn clear_ack_e2e(&mut self) {
        self.ack_e2e.clear();
    }

    // Param is passed by value, moved
    pub fn set_ack_e2e(&mut self, v: ::std::vec::Vec<u32>) {
        self.ack_e2e = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ack_e2e(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_e2e
    }

    // Take field
    pub fn take_ack_e2e(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.ack_e2e, ::std::vec::Vec::new())
    }

    pub fn get_ack_e2e(&self) -> &[u32] {
        &self.ack_e2e
    }

    fn get_ack_e2e_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.ack_e2e
    }

    fn mut_ack_e2e_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_e2e
    }

    // optional uint32 flags = 6;

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

    // optional fixed32 client_connection_id = 8;

    pub fn clear_client_connection_id(&mut self) {
        self.client_connection_id = ::std::option::Option::None;
    }

    pub fn has_client_connection_id(&self) -> bool {
        self.client_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_connection_id(&mut self, v: u32) {
        self.client_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_connection_id(&self) -> u32 {
        self.client_connection_id.unwrap_or(0)
    }

    fn get_client_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_connection_id
    }

    fn mut_client_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_connection_id
    }

    // optional uint32 seq_num_c2r = 9;

    pub fn clear_seq_num_c2r(&mut self) {
        self.seq_num_c2r = ::std::option::Option::None;
    }

    pub fn has_seq_num_c2r(&self) -> bool {
        self.seq_num_c2r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_c2r(&mut self, v: u32) {
        self.seq_num_c2r = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_c2r(&self) -> u32 {
        self.seq_num_c2r.unwrap_or(0)
    }

    fn get_seq_num_c2r_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_c2r
    }

    fn mut_seq_num_c2r_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_c2r
    }

    // optional uint32 seq_num_c2s = 10;

    pub fn clear_seq_num_c2s(&mut self) {
        self.seq_num_c2s = ::std::option::Option::None;
    }

    pub fn has_seq_num_c2s(&self) -> bool {
        self.seq_num_c2s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_c2s(&mut self, v: u32) {
        self.seq_num_c2s = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_c2s(&self) -> u32 {
        self.seq_num_c2s.unwrap_or(0)
    }

    fn get_seq_num_c2s_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_c2s
    }

    fn mut_seq_num_c2s_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_c2s
    }
}

impl ::protobuf::Message for CMsgSteamDatagramConnectionStatsClientToRouter {
    fn is_initialized(&self) -> bool {
        for v in &self.c2r {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.c2s {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.c2r)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.c2s)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.legacy_client_timestamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.ack_relay)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.ack_e2e)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.client_connection_id = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seq_num_c2r = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seq_num_c2s = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.c2r.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.c2s.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.legacy_client_timestamp {
            my_size += 5;
        }
        my_size += 5 * self.ack_relay.len() as u32;
        my_size += 5 * self.ack_e2e.len() as u32;
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.client_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.seq_num_c2r {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.seq_num_c2s {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.c2r.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.c2s.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.legacy_client_timestamp {
            os.write_fixed32(3, v)?;
        }
        for v in &self.ack_relay {
            os.write_fixed32(4, *v)?;
        };
        for v in &self.ack_e2e {
            os.write_fixed32(5, *v)?;
        };
        if let Some(v) = self.flags {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.client_connection_id {
            os.write_fixed32(8, v)?;
        }
        if let Some(v) = self.seq_num_c2r {
            os.write_uint32(9, v)?;
        }
        if let Some(v) = self.seq_num_c2s {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramConnectionStatsClientToRouter {
    fn new() -> CMsgSteamDatagramConnectionStatsClientToRouter {
        CMsgSteamDatagramConnectionStatsClientToRouter::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionStatsClientToRouter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "c2r",
                    CMsgSteamDatagramConnectionStatsClientToRouter::get_c2r_for_reflect,
                    CMsgSteamDatagramConnectionStatsClientToRouter::mut_c2r_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "c2s",
                    CMsgSteamDatagramConnectionStatsClientToRouter::get_c2s_for_reflect,
                    CMsgSteamDatagramConnectionStatsClientToRouter::mut_c2s_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "legacy_client_timestamp",
                    CMsgSteamDatagramConnectionStatsClientToRouter::get_legacy_client_timestamp_for_reflect,
                    CMsgSteamDatagramConnectionStatsClientToRouter::mut_legacy_client_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "ack_relay",
                    CMsgSteamDatagramConnectionStatsClientToRouter::get_ack_relay_for_reflect,
                    CMsgSteamDatagramConnectionStatsClientToRouter::mut_ack_relay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "ack_e2e",
                    CMsgSteamDatagramConnectionStatsClientToRouter::get_ack_e2e_for_reflect,
                    CMsgSteamDatagramConnectionStatsClientToRouter::mut_ack_e2e_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CMsgSteamDatagramConnectionStatsClientToRouter::get_flags_for_reflect,
                    CMsgSteamDatagramConnectionStatsClientToRouter::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_connection_id",
                    CMsgSteamDatagramConnectionStatsClientToRouter::get_client_connection_id_for_reflect,
                    CMsgSteamDatagramConnectionStatsClientToRouter::mut_client_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_c2r",
                    CMsgSteamDatagramConnectionStatsClientToRouter::get_seq_num_c2r_for_reflect,
                    CMsgSteamDatagramConnectionStatsClientToRouter::mut_seq_num_c2r_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_c2s",
                    CMsgSteamDatagramConnectionStatsClientToRouter::get_seq_num_c2s_for_reflect,
                    CMsgSteamDatagramConnectionStatsClientToRouter::mut_seq_num_c2s_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramConnectionStatsClientToRouter>(
                    "CMsgSteamDatagramConnectionStatsClientToRouter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramConnectionStatsClientToRouter {
    fn clear(&mut self) {
        self.clear_c2r();
        self.clear_c2s();
        self.clear_legacy_client_timestamp();
        self.clear_ack_relay();
        self.clear_ack_e2e();
        self.clear_flags();
        self.clear_client_connection_id();
        self.clear_seq_num_c2r();
        self.clear_seq_num_c2s();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramConnectionStatsClientToRouter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionStatsClientToRouter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgSteamDatagramConnectionStatsClientToRouter_Flags {
    ACK_REQUEST_RELAY = 1,
    ACK_REQUEST_E2E = 2,
    ACK_REQUEST_IMMEDIATE = 4,
}

impl ::protobuf::ProtobufEnum for CMsgSteamDatagramConnectionStatsClientToRouter_Flags {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgSteamDatagramConnectionStatsClientToRouter_Flags> {
        match value {
            1 => ::std::option::Option::Some(CMsgSteamDatagramConnectionStatsClientToRouter_Flags::ACK_REQUEST_RELAY),
            2 => ::std::option::Option::Some(CMsgSteamDatagramConnectionStatsClientToRouter_Flags::ACK_REQUEST_E2E),
            4 => ::std::option::Option::Some(CMsgSteamDatagramConnectionStatsClientToRouter_Flags::ACK_REQUEST_IMMEDIATE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgSteamDatagramConnectionStatsClientToRouter_Flags] = &[
            CMsgSteamDatagramConnectionStatsClientToRouter_Flags::ACK_REQUEST_RELAY,
            CMsgSteamDatagramConnectionStatsClientToRouter_Flags::ACK_REQUEST_E2E,
            CMsgSteamDatagramConnectionStatsClientToRouter_Flags::ACK_REQUEST_IMMEDIATE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionStatsClientToRouter_Flags>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgSteamDatagramConnectionStatsClientToRouter_Flags", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgSteamDatagramConnectionStatsClientToRouter_Flags {
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionStatsClientToRouter_Flags {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramConnectionStatsRouterToClient {
    // message fields
    r2c: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    s2c: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    legacy_client_timestamp_from_router: ::std::option::Option<u32>,
    legacy_client_timestamp_from_server: ::std::option::Option<u32>,
    router_gameserver_latency: ::std::option::Option<u32>,
    seconds_until_shutdown: ::std::option::Option<u32>,
    migrate_request_ip: ::std::option::Option<u32>,
    migrate_request_port: ::std::option::Option<u32>,
    scoring_penalty_relay_cluster: ::std::option::Option<u32>,
    ack_relay: ::std::vec::Vec<u32>,
    ack_e2e: ::std::vec::Vec<u32>,
    flags: ::std::option::Option<u32>,
    client_connection_id: ::std::option::Option<u32>,
    seq_num_r2c: ::std::option::Option<u32>,
    seq_num_s2c: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramConnectionStatsRouterToClient {}

impl CMsgSteamDatagramConnectionStatsRouterToClient {
    pub fn new() -> CMsgSteamDatagramConnectionStatsRouterToClient {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramConnectionStatsRouterToClient {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramConnectionStatsRouterToClient> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramConnectionStatsRouterToClient,
        };
        unsafe {
            instance.get(CMsgSteamDatagramConnectionStatsRouterToClient::new)
        }
    }

    // optional .CMsgSteamDatagramConnectionQuality r2c = 1;

    pub fn clear_r2c(&mut self) {
        self.r2c.clear();
    }

    pub fn has_r2c(&self) -> bool {
        self.r2c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r2c(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.r2c = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_r2c(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.r2c.is_none() {
            self.r2c.set_default();
        }
        self.r2c.as_mut().unwrap()
    }

    // Take field
    pub fn take_r2c(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.r2c.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_r2c(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.r2c.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_r2c_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.r2c
    }

    fn mut_r2c_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.r2c
    }

    // optional .CMsgSteamDatagramConnectionQuality s2c = 2;

    pub fn clear_s2c(&mut self) {
        self.s2c.clear();
    }

    pub fn has_s2c(&self) -> bool {
        self.s2c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s2c(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.s2c = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s2c(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.s2c.is_none() {
            self.s2c.set_default();
        }
        self.s2c.as_mut().unwrap()
    }

    // Take field
    pub fn take_s2c(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.s2c.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_s2c(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.s2c.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_s2c_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.s2c
    }

    fn mut_s2c_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.s2c
    }

    // optional fixed32 legacy_client_timestamp_from_router = 3;

    pub fn clear_legacy_client_timestamp_from_router(&mut self) {
        self.legacy_client_timestamp_from_router = ::std::option::Option::None;
    }

    pub fn has_legacy_client_timestamp_from_router(&self) -> bool {
        self.legacy_client_timestamp_from_router.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_client_timestamp_from_router(&mut self, v: u32) {
        self.legacy_client_timestamp_from_router = ::std::option::Option::Some(v);
    }

    pub fn get_legacy_client_timestamp_from_router(&self) -> u32 {
        self.legacy_client_timestamp_from_router.unwrap_or(0)
    }

    fn get_legacy_client_timestamp_from_router_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.legacy_client_timestamp_from_router
    }

    fn mut_legacy_client_timestamp_from_router_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.legacy_client_timestamp_from_router
    }

    // optional fixed32 legacy_client_timestamp_from_server = 4;

    pub fn clear_legacy_client_timestamp_from_server(&mut self) {
        self.legacy_client_timestamp_from_server = ::std::option::Option::None;
    }

    pub fn has_legacy_client_timestamp_from_server(&self) -> bool {
        self.legacy_client_timestamp_from_server.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_client_timestamp_from_server(&mut self, v: u32) {
        self.legacy_client_timestamp_from_server = ::std::option::Option::Some(v);
    }

    pub fn get_legacy_client_timestamp_from_server(&self) -> u32 {
        self.legacy_client_timestamp_from_server.unwrap_or(0)
    }

    fn get_legacy_client_timestamp_from_server_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.legacy_client_timestamp_from_server
    }

    fn mut_legacy_client_timestamp_from_server_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.legacy_client_timestamp_from_server
    }

    // optional uint32 router_gameserver_latency = 5;

    pub fn clear_router_gameserver_latency(&mut self) {
        self.router_gameserver_latency = ::std::option::Option::None;
    }

    pub fn has_router_gameserver_latency(&self) -> bool {
        self.router_gameserver_latency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_router_gameserver_latency(&mut self, v: u32) {
        self.router_gameserver_latency = ::std::option::Option::Some(v);
    }

    pub fn get_router_gameserver_latency(&self) -> u32 {
        self.router_gameserver_latency.unwrap_or(0)
    }

    fn get_router_gameserver_latency_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.router_gameserver_latency
    }

    fn mut_router_gameserver_latency_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.router_gameserver_latency
    }

    // optional uint32 seconds_until_shutdown = 6;

    pub fn clear_seconds_until_shutdown(&mut self) {
        self.seconds_until_shutdown = ::std::option::Option::None;
    }

    pub fn has_seconds_until_shutdown(&self) -> bool {
        self.seconds_until_shutdown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seconds_until_shutdown(&mut self, v: u32) {
        self.seconds_until_shutdown = ::std::option::Option::Some(v);
    }

    pub fn get_seconds_until_shutdown(&self) -> u32 {
        self.seconds_until_shutdown.unwrap_or(0)
    }

    fn get_seconds_until_shutdown_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seconds_until_shutdown
    }

    fn mut_seconds_until_shutdown_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seconds_until_shutdown
    }

    // optional fixed32 migrate_request_ip = 10;

    pub fn clear_migrate_request_ip(&mut self) {
        self.migrate_request_ip = ::std::option::Option::None;
    }

    pub fn has_migrate_request_ip(&self) -> bool {
        self.migrate_request_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_migrate_request_ip(&mut self, v: u32) {
        self.migrate_request_ip = ::std::option::Option::Some(v);
    }

    pub fn get_migrate_request_ip(&self) -> u32 {
        self.migrate_request_ip.unwrap_or(0)
    }

    fn get_migrate_request_ip_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.migrate_request_ip
    }

    fn mut_migrate_request_ip_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.migrate_request_ip
    }

    // optional uint32 migrate_request_port = 11;

    pub fn clear_migrate_request_port(&mut self) {
        self.migrate_request_port = ::std::option::Option::None;
    }

    pub fn has_migrate_request_port(&self) -> bool {
        self.migrate_request_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_migrate_request_port(&mut self, v: u32) {
        self.migrate_request_port = ::std::option::Option::Some(v);
    }

    pub fn get_migrate_request_port(&self) -> u32 {
        self.migrate_request_port.unwrap_or(0)
    }

    fn get_migrate_request_port_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.migrate_request_port
    }

    fn mut_migrate_request_port_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.migrate_request_port
    }

    // optional uint32 scoring_penalty_relay_cluster = 12;

    pub fn clear_scoring_penalty_relay_cluster(&mut self) {
        self.scoring_penalty_relay_cluster = ::std::option::Option::None;
    }

    pub fn has_scoring_penalty_relay_cluster(&self) -> bool {
        self.scoring_penalty_relay_cluster.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scoring_penalty_relay_cluster(&mut self, v: u32) {
        self.scoring_penalty_relay_cluster = ::std::option::Option::Some(v);
    }

    pub fn get_scoring_penalty_relay_cluster(&self) -> u32 {
        self.scoring_penalty_relay_cluster.unwrap_or(0)
    }

    fn get_scoring_penalty_relay_cluster_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.scoring_penalty_relay_cluster
    }

    fn mut_scoring_penalty_relay_cluster_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.scoring_penalty_relay_cluster
    }

    // repeated fixed32 ack_relay = 13;

    pub fn clear_ack_relay(&mut self) {
        self.ack_relay.clear();
    }

    // Param is passed by value, moved
    pub fn set_ack_relay(&mut self, v: ::std::vec::Vec<u32>) {
        self.ack_relay = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ack_relay(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_relay
    }

    // Take field
    pub fn take_ack_relay(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.ack_relay, ::std::vec::Vec::new())
    }

    pub fn get_ack_relay(&self) -> &[u32] {
        &self.ack_relay
    }

    fn get_ack_relay_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.ack_relay
    }

    fn mut_ack_relay_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_relay
    }

    // repeated fixed32 ack_e2e = 14;

    pub fn clear_ack_e2e(&mut self) {
        self.ack_e2e.clear();
    }

    // Param is passed by value, moved
    pub fn set_ack_e2e(&mut self, v: ::std::vec::Vec<u32>) {
        self.ack_e2e = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ack_e2e(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_e2e
    }

    // Take field
    pub fn take_ack_e2e(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.ack_e2e, ::std::vec::Vec::new())
    }

    pub fn get_ack_e2e(&self) -> &[u32] {
        &self.ack_e2e
    }

    fn get_ack_e2e_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.ack_e2e
    }

    fn mut_ack_e2e_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_e2e
    }

    // optional uint32 flags = 15;

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

    // optional fixed32 client_connection_id = 7;

    pub fn clear_client_connection_id(&mut self) {
        self.client_connection_id = ::std::option::Option::None;
    }

    pub fn has_client_connection_id(&self) -> bool {
        self.client_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_connection_id(&mut self, v: u32) {
        self.client_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_connection_id(&self) -> u32 {
        self.client_connection_id.unwrap_or(0)
    }

    fn get_client_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_connection_id
    }

    fn mut_client_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_connection_id
    }

    // optional uint32 seq_num_r2c = 8;

    pub fn clear_seq_num_r2c(&mut self) {
        self.seq_num_r2c = ::std::option::Option::None;
    }

    pub fn has_seq_num_r2c(&self) -> bool {
        self.seq_num_r2c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_r2c(&mut self, v: u32) {
        self.seq_num_r2c = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_r2c(&self) -> u32 {
        self.seq_num_r2c.unwrap_or(0)
    }

    fn get_seq_num_r2c_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_r2c
    }

    fn mut_seq_num_r2c_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_r2c
    }

    // optional uint32 seq_num_s2c = 9;

    pub fn clear_seq_num_s2c(&mut self) {
        self.seq_num_s2c = ::std::option::Option::None;
    }

    pub fn has_seq_num_s2c(&self) -> bool {
        self.seq_num_s2c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_s2c(&mut self, v: u32) {
        self.seq_num_s2c = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_s2c(&self) -> u32 {
        self.seq_num_s2c.unwrap_or(0)
    }

    fn get_seq_num_s2c_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_s2c
    }

    fn mut_seq_num_s2c_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_s2c
    }
}

impl ::protobuf::Message for CMsgSteamDatagramConnectionStatsRouterToClient {
    fn is_initialized(&self) -> bool {
        for v in &self.r2c {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.s2c {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.r2c)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.s2c)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.legacy_client_timestamp_from_router = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.legacy_client_timestamp_from_server = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.router_gameserver_latency = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seconds_until_shutdown = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.migrate_request_ip = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.migrate_request_port = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.scoring_penalty_relay_cluster = ::std::option::Option::Some(tmp);
                },
                13 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.ack_relay)?;
                },
                14 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.ack_e2e)?;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.client_connection_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seq_num_r2c = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seq_num_s2c = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.r2c.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.s2c.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.legacy_client_timestamp_from_router {
            my_size += 5;
        }
        if let Some(v) = self.legacy_client_timestamp_from_server {
            my_size += 5;
        }
        if let Some(v) = self.router_gameserver_latency {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.seconds_until_shutdown {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.migrate_request_ip {
            my_size += 5;
        }
        if let Some(v) = self.migrate_request_port {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.scoring_penalty_relay_cluster {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += 5 * self.ack_relay.len() as u32;
        my_size += 5 * self.ack_e2e.len() as u32;
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.client_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.seq_num_r2c {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.seq_num_s2c {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.r2c.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.s2c.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.legacy_client_timestamp_from_router {
            os.write_fixed32(3, v)?;
        }
        if let Some(v) = self.legacy_client_timestamp_from_server {
            os.write_fixed32(4, v)?;
        }
        if let Some(v) = self.router_gameserver_latency {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.seconds_until_shutdown {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.migrate_request_ip {
            os.write_fixed32(10, v)?;
        }
        if let Some(v) = self.migrate_request_port {
            os.write_uint32(11, v)?;
        }
        if let Some(v) = self.scoring_penalty_relay_cluster {
            os.write_uint32(12, v)?;
        }
        for v in &self.ack_relay {
            os.write_fixed32(13, *v)?;
        };
        for v in &self.ack_e2e {
            os.write_fixed32(14, *v)?;
        };
        if let Some(v) = self.flags {
            os.write_uint32(15, v)?;
        }
        if let Some(v) = self.client_connection_id {
            os.write_fixed32(7, v)?;
        }
        if let Some(v) = self.seq_num_r2c {
            os.write_uint32(8, v)?;
        }
        if let Some(v) = self.seq_num_s2c {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramConnectionStatsRouterToClient {
    fn new() -> CMsgSteamDatagramConnectionStatsRouterToClient {
        CMsgSteamDatagramConnectionStatsRouterToClient::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionStatsRouterToClient>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "r2c",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_r2c_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_r2c_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "s2c",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_s2c_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_s2c_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "legacy_client_timestamp_from_router",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_legacy_client_timestamp_from_router_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_legacy_client_timestamp_from_router_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "legacy_client_timestamp_from_server",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_legacy_client_timestamp_from_server_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_legacy_client_timestamp_from_server_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "router_gameserver_latency",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_router_gameserver_latency_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_router_gameserver_latency_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seconds_until_shutdown",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_seconds_until_shutdown_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_seconds_until_shutdown_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "migrate_request_ip",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_migrate_request_ip_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_migrate_request_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "migrate_request_port",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_migrate_request_port_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_migrate_request_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "scoring_penalty_relay_cluster",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_scoring_penalty_relay_cluster_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_scoring_penalty_relay_cluster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "ack_relay",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_ack_relay_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_ack_relay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "ack_e2e",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_ack_e2e_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_ack_e2e_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_flags_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_connection_id",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_client_connection_id_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_client_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_r2c",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_seq_num_r2c_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_seq_num_r2c_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_s2c",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_seq_num_s2c_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_seq_num_s2c_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramConnectionStatsRouterToClient>(
                    "CMsgSteamDatagramConnectionStatsRouterToClient",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramConnectionStatsRouterToClient {
    fn clear(&mut self) {
        self.clear_r2c();
        self.clear_s2c();
        self.clear_legacy_client_timestamp_from_router();
        self.clear_legacy_client_timestamp_from_server();
        self.clear_router_gameserver_latency();
        self.clear_seconds_until_shutdown();
        self.clear_migrate_request_ip();
        self.clear_migrate_request_port();
        self.clear_scoring_penalty_relay_cluster();
        self.clear_ack_relay();
        self.clear_ack_e2e();
        self.clear_flags();
        self.clear_client_connection_id();
        self.clear_seq_num_r2c();
        self.clear_seq_num_s2c();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramConnectionStatsRouterToClient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionStatsRouterToClient {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgSteamDatagramConnectionStatsRouterToClient_Flags {
    ACK_REQUEST_RELAY = 1,
    ACK_REQUEST_E2E = 2,
    ACK_REQUEST_IMMEDIATE = 4,
}

impl ::protobuf::ProtobufEnum for CMsgSteamDatagramConnectionStatsRouterToClient_Flags {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgSteamDatagramConnectionStatsRouterToClient_Flags> {
        match value {
            1 => ::std::option::Option::Some(CMsgSteamDatagramConnectionStatsRouterToClient_Flags::ACK_REQUEST_RELAY),
            2 => ::std::option::Option::Some(CMsgSteamDatagramConnectionStatsRouterToClient_Flags::ACK_REQUEST_E2E),
            4 => ::std::option::Option::Some(CMsgSteamDatagramConnectionStatsRouterToClient_Flags::ACK_REQUEST_IMMEDIATE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgSteamDatagramConnectionStatsRouterToClient_Flags] = &[
            CMsgSteamDatagramConnectionStatsRouterToClient_Flags::ACK_REQUEST_RELAY,
            CMsgSteamDatagramConnectionStatsRouterToClient_Flags::ACK_REQUEST_E2E,
            CMsgSteamDatagramConnectionStatsRouterToClient_Flags::ACK_REQUEST_IMMEDIATE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionStatsRouterToClient_Flags>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgSteamDatagramConnectionStatsRouterToClient_Flags", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgSteamDatagramConnectionStatsRouterToClient_Flags {
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionStatsRouterToClient_Flags {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramConnectionStatsRouterToServer {
    // message fields
    r2s: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    c2s: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    legacy_client_timestamp: ::std::option::Option<u32>,
    legacy_router_timestamp: ::std::option::Option<u32>,
    ack_relay: ::std::vec::Vec<u32>,
    ack_e2e: ::std::vec::Vec<u32>,
    flags: ::std::option::Option<u32>,
    seq_num_r2s: ::std::option::Option<u32>,
    seq_num_c2s: ::std::option::Option<u32>,
    client_steam_id: ::std::option::Option<u64>,
    relay_session_id: ::std::option::Option<u32>,
    client_connection_id: ::std::option::Option<u32>,
    server_connection_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramConnectionStatsRouterToServer {}

impl CMsgSteamDatagramConnectionStatsRouterToServer {
    pub fn new() -> CMsgSteamDatagramConnectionStatsRouterToServer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramConnectionStatsRouterToServer {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramConnectionStatsRouterToServer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramConnectionStatsRouterToServer,
        };
        unsafe {
            instance.get(CMsgSteamDatagramConnectionStatsRouterToServer::new)
        }
    }

    // optional .CMsgSteamDatagramConnectionQuality r2s = 1;

    pub fn clear_r2s(&mut self) {
        self.r2s.clear();
    }

    pub fn has_r2s(&self) -> bool {
        self.r2s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r2s(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.r2s = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_r2s(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.r2s.is_none() {
            self.r2s.set_default();
        }
        self.r2s.as_mut().unwrap()
    }

    // Take field
    pub fn take_r2s(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.r2s.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_r2s(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.r2s.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_r2s_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.r2s
    }

    fn mut_r2s_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.r2s
    }

    // optional .CMsgSteamDatagramConnectionQuality c2s = 2;

    pub fn clear_c2s(&mut self) {
        self.c2s.clear();
    }

    pub fn has_c2s(&self) -> bool {
        self.c2s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c2s(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.c2s = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_c2s(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.c2s.is_none() {
            self.c2s.set_default();
        }
        self.c2s.as_mut().unwrap()
    }

    // Take field
    pub fn take_c2s(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.c2s.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_c2s(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.c2s.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_c2s_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.c2s
    }

    fn mut_c2s_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.c2s
    }

    // optional fixed32 legacy_client_timestamp = 3;

    pub fn clear_legacy_client_timestamp(&mut self) {
        self.legacy_client_timestamp = ::std::option::Option::None;
    }

    pub fn has_legacy_client_timestamp(&self) -> bool {
        self.legacy_client_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_client_timestamp(&mut self, v: u32) {
        self.legacy_client_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_legacy_client_timestamp(&self) -> u32 {
        self.legacy_client_timestamp.unwrap_or(0)
    }

    fn get_legacy_client_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.legacy_client_timestamp
    }

    fn mut_legacy_client_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.legacy_client_timestamp
    }

    // optional fixed32 legacy_router_timestamp = 4;

    pub fn clear_legacy_router_timestamp(&mut self) {
        self.legacy_router_timestamp = ::std::option::Option::None;
    }

    pub fn has_legacy_router_timestamp(&self) -> bool {
        self.legacy_router_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_router_timestamp(&mut self, v: u32) {
        self.legacy_router_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_legacy_router_timestamp(&self) -> u32 {
        self.legacy_router_timestamp.unwrap_or(0)
    }

    fn get_legacy_router_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.legacy_router_timestamp
    }

    fn mut_legacy_router_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.legacy_router_timestamp
    }

    // repeated fixed32 ack_relay = 10;

    pub fn clear_ack_relay(&mut self) {
        self.ack_relay.clear();
    }

    // Param is passed by value, moved
    pub fn set_ack_relay(&mut self, v: ::std::vec::Vec<u32>) {
        self.ack_relay = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ack_relay(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_relay
    }

    // Take field
    pub fn take_ack_relay(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.ack_relay, ::std::vec::Vec::new())
    }

    pub fn get_ack_relay(&self) -> &[u32] {
        &self.ack_relay
    }

    fn get_ack_relay_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.ack_relay
    }

    fn mut_ack_relay_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_relay
    }

    // repeated fixed32 ack_e2e = 11;

    pub fn clear_ack_e2e(&mut self) {
        self.ack_e2e.clear();
    }

    // Param is passed by value, moved
    pub fn set_ack_e2e(&mut self, v: ::std::vec::Vec<u32>) {
        self.ack_e2e = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ack_e2e(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_e2e
    }

    // Take field
    pub fn take_ack_e2e(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.ack_e2e, ::std::vec::Vec::new())
    }

    pub fn get_ack_e2e(&self) -> &[u32] {
        &self.ack_e2e
    }

    fn get_ack_e2e_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.ack_e2e
    }

    fn mut_ack_e2e_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_e2e
    }

    // optional uint32 flags = 12;

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

    // optional uint32 seq_num_r2s = 5;

    pub fn clear_seq_num_r2s(&mut self) {
        self.seq_num_r2s = ::std::option::Option::None;
    }

    pub fn has_seq_num_r2s(&self) -> bool {
        self.seq_num_r2s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_r2s(&mut self, v: u32) {
        self.seq_num_r2s = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_r2s(&self) -> u32 {
        self.seq_num_r2s.unwrap_or(0)
    }

    fn get_seq_num_r2s_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_r2s
    }

    fn mut_seq_num_r2s_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_r2s
    }

    // optional uint32 seq_num_c2s = 6;

    pub fn clear_seq_num_c2s(&mut self) {
        self.seq_num_c2s = ::std::option::Option::None;
    }

    pub fn has_seq_num_c2s(&self) -> bool {
        self.seq_num_c2s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_c2s(&mut self, v: u32) {
        self.seq_num_c2s = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_c2s(&self) -> u32 {
        self.seq_num_c2s.unwrap_or(0)
    }

    fn get_seq_num_c2s_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_c2s
    }

    fn mut_seq_num_c2s_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_c2s
    }

    // optional fixed64 client_steam_id = 7;

    pub fn clear_client_steam_id(&mut self) {
        self.client_steam_id = ::std::option::Option::None;
    }

    pub fn has_client_steam_id(&self) -> bool {
        self.client_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_steam_id(&mut self, v: u64) {
        self.client_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_steam_id(&self) -> u64 {
        self.client_steam_id.unwrap_or(0)
    }

    fn get_client_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.client_steam_id
    }

    fn mut_client_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.client_steam_id
    }

    // optional uint32 relay_session_id = 8;

    pub fn clear_relay_session_id(&mut self) {
        self.relay_session_id = ::std::option::Option::None;
    }

    pub fn has_relay_session_id(&self) -> bool {
        self.relay_session_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_relay_session_id(&mut self, v: u32) {
        self.relay_session_id = ::std::option::Option::Some(v);
    }

    pub fn get_relay_session_id(&self) -> u32 {
        self.relay_session_id.unwrap_or(0)
    }

    fn get_relay_session_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.relay_session_id
    }

    fn mut_relay_session_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.relay_session_id
    }

    // optional fixed32 client_connection_id = 9;

    pub fn clear_client_connection_id(&mut self) {
        self.client_connection_id = ::std::option::Option::None;
    }

    pub fn has_client_connection_id(&self) -> bool {
        self.client_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_connection_id(&mut self, v: u32) {
        self.client_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_connection_id(&self) -> u32 {
        self.client_connection_id.unwrap_or(0)
    }

    fn get_client_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_connection_id
    }

    fn mut_client_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_connection_id
    }

    // optional fixed32 server_connection_id = 13;

    pub fn clear_server_connection_id(&mut self) {
        self.server_connection_id = ::std::option::Option::None;
    }

    pub fn has_server_connection_id(&self) -> bool {
        self.server_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_connection_id(&mut self, v: u32) {
        self.server_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_server_connection_id(&self) -> u32 {
        self.server_connection_id.unwrap_or(0)
    }

    fn get_server_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_connection_id
    }

    fn mut_server_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_connection_id
    }
}

impl ::protobuf::Message for CMsgSteamDatagramConnectionStatsRouterToServer {
    fn is_initialized(&self) -> bool {
        for v in &self.r2s {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.c2s {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.r2s)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.c2s)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.legacy_client_timestamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.legacy_router_timestamp = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.ack_relay)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.ack_e2e)?;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seq_num_r2s = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seq_num_c2s = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.client_steam_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.relay_session_id = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.client_connection_id = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.server_connection_id = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.r2s.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.c2s.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.legacy_client_timestamp {
            my_size += 5;
        }
        if let Some(v) = self.legacy_router_timestamp {
            my_size += 5;
        }
        my_size += 5 * self.ack_relay.len() as u32;
        my_size += 5 * self.ack_e2e.len() as u32;
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.seq_num_r2s {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.seq_num_c2s {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.client_steam_id {
            my_size += 9;
        }
        if let Some(v) = self.relay_session_id {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.client_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.server_connection_id {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.r2s.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.c2s.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.legacy_client_timestamp {
            os.write_fixed32(3, v)?;
        }
        if let Some(v) = self.legacy_router_timestamp {
            os.write_fixed32(4, v)?;
        }
        for v in &self.ack_relay {
            os.write_fixed32(10, *v)?;
        };
        for v in &self.ack_e2e {
            os.write_fixed32(11, *v)?;
        };
        if let Some(v) = self.flags {
            os.write_uint32(12, v)?;
        }
        if let Some(v) = self.seq_num_r2s {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.seq_num_c2s {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.client_steam_id {
            os.write_fixed64(7, v)?;
        }
        if let Some(v) = self.relay_session_id {
            os.write_uint32(8, v)?;
        }
        if let Some(v) = self.client_connection_id {
            os.write_fixed32(9, v)?;
        }
        if let Some(v) = self.server_connection_id {
            os.write_fixed32(13, v)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramConnectionStatsRouterToServer {
    fn new() -> CMsgSteamDatagramConnectionStatsRouterToServer {
        CMsgSteamDatagramConnectionStatsRouterToServer::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionStatsRouterToServer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "r2s",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_r2s_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_r2s_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "c2s",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_c2s_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_c2s_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "legacy_client_timestamp",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_legacy_client_timestamp_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_legacy_client_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "legacy_router_timestamp",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_legacy_router_timestamp_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_legacy_router_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "ack_relay",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_ack_relay_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_ack_relay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "ack_e2e",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_ack_e2e_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_ack_e2e_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_flags_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_r2s",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_seq_num_r2s_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_seq_num_r2s_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_c2s",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_seq_num_c2s_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_seq_num_c2s_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "client_steam_id",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_client_steam_id_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_client_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "relay_session_id",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_relay_session_id_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_relay_session_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_connection_id",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_client_connection_id_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_client_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "server_connection_id",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_server_connection_id_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_server_connection_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramConnectionStatsRouterToServer>(
                    "CMsgSteamDatagramConnectionStatsRouterToServer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramConnectionStatsRouterToServer {
    fn clear(&mut self) {
        self.clear_r2s();
        self.clear_c2s();
        self.clear_legacy_client_timestamp();
        self.clear_legacy_router_timestamp();
        self.clear_ack_relay();
        self.clear_ack_e2e();
        self.clear_flags();
        self.clear_seq_num_r2s();
        self.clear_seq_num_c2s();
        self.clear_client_steam_id();
        self.clear_relay_session_id();
        self.clear_client_connection_id();
        self.clear_server_connection_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramConnectionStatsRouterToServer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionStatsRouterToServer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgSteamDatagramConnectionStatsRouterToServer_Flags {
    ACK_REQUEST_RELAY = 1,
    ACK_REQUEST_E2E = 2,
    ACK_REQUEST_IMMEDIATE = 4,
}

impl ::protobuf::ProtobufEnum for CMsgSteamDatagramConnectionStatsRouterToServer_Flags {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgSteamDatagramConnectionStatsRouterToServer_Flags> {
        match value {
            1 => ::std::option::Option::Some(CMsgSteamDatagramConnectionStatsRouterToServer_Flags::ACK_REQUEST_RELAY),
            2 => ::std::option::Option::Some(CMsgSteamDatagramConnectionStatsRouterToServer_Flags::ACK_REQUEST_E2E),
            4 => ::std::option::Option::Some(CMsgSteamDatagramConnectionStatsRouterToServer_Flags::ACK_REQUEST_IMMEDIATE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgSteamDatagramConnectionStatsRouterToServer_Flags] = &[
            CMsgSteamDatagramConnectionStatsRouterToServer_Flags::ACK_REQUEST_RELAY,
            CMsgSteamDatagramConnectionStatsRouterToServer_Flags::ACK_REQUEST_E2E,
            CMsgSteamDatagramConnectionStatsRouterToServer_Flags::ACK_REQUEST_IMMEDIATE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionStatsRouterToServer_Flags>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgSteamDatagramConnectionStatsRouterToServer_Flags", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgSteamDatagramConnectionStatsRouterToServer_Flags {
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionStatsRouterToServer_Flags {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramConnectionStatsServerToRouter {
    // message fields
    s2r: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    s2c: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    ack_relay: ::std::vec::Vec<u32>,
    ack_e2e: ::std::vec::Vec<u32>,
    flags: ::std::option::Option<u32>,
    seq_num_s2r: ::std::option::Option<u32>,
    seq_num_s2c: ::std::option::Option<u32>,
    client_steam_id: ::std::option::Option<u64>,
    relay_session_id: ::std::option::Option<u32>,
    client_connection_id: ::std::option::Option<u32>,
    server_connection_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramConnectionStatsServerToRouter {}

impl CMsgSteamDatagramConnectionStatsServerToRouter {
    pub fn new() -> CMsgSteamDatagramConnectionStatsServerToRouter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramConnectionStatsServerToRouter {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramConnectionStatsServerToRouter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramConnectionStatsServerToRouter,
        };
        unsafe {
            instance.get(CMsgSteamDatagramConnectionStatsServerToRouter::new)
        }
    }

    // optional .CMsgSteamDatagramConnectionQuality s2r = 1;

    pub fn clear_s2r(&mut self) {
        self.s2r.clear();
    }

    pub fn has_s2r(&self) -> bool {
        self.s2r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s2r(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.s2r = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s2r(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.s2r.is_none() {
            self.s2r.set_default();
        }
        self.s2r.as_mut().unwrap()
    }

    // Take field
    pub fn take_s2r(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.s2r.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_s2r(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.s2r.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_s2r_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.s2r
    }

    fn mut_s2r_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.s2r
    }

    // optional .CMsgSteamDatagramConnectionQuality s2c = 2;

    pub fn clear_s2c(&mut self) {
        self.s2c.clear();
    }

    pub fn has_s2c(&self) -> bool {
        self.s2c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s2c(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.s2c = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s2c(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.s2c.is_none() {
            self.s2c.set_default();
        }
        self.s2c.as_mut().unwrap()
    }

    // Take field
    pub fn take_s2c(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.s2c.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_s2c(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.s2c.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_s2c_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.s2c
    }

    fn mut_s2c_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.s2c
    }

    // repeated fixed32 ack_relay = 8;

    pub fn clear_ack_relay(&mut self) {
        self.ack_relay.clear();
    }

    // Param is passed by value, moved
    pub fn set_ack_relay(&mut self, v: ::std::vec::Vec<u32>) {
        self.ack_relay = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ack_relay(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_relay
    }

    // Take field
    pub fn take_ack_relay(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.ack_relay, ::std::vec::Vec::new())
    }

    pub fn get_ack_relay(&self) -> &[u32] {
        &self.ack_relay
    }

    fn get_ack_relay_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.ack_relay
    }

    fn mut_ack_relay_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_relay
    }

    // repeated fixed32 ack_e2e = 9;

    pub fn clear_ack_e2e(&mut self) {
        self.ack_e2e.clear();
    }

    // Param is passed by value, moved
    pub fn set_ack_e2e(&mut self, v: ::std::vec::Vec<u32>) {
        self.ack_e2e = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ack_e2e(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_e2e
    }

    // Take field
    pub fn take_ack_e2e(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.ack_e2e, ::std::vec::Vec::new())
    }

    pub fn get_ack_e2e(&self) -> &[u32] {
        &self.ack_e2e
    }

    fn get_ack_e2e_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.ack_e2e
    }

    fn mut_ack_e2e_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_e2e
    }

    // optional uint32 flags = 10;

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

    // optional uint32 seq_num_s2r = 3;

    pub fn clear_seq_num_s2r(&mut self) {
        self.seq_num_s2r = ::std::option::Option::None;
    }

    pub fn has_seq_num_s2r(&self) -> bool {
        self.seq_num_s2r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_s2r(&mut self, v: u32) {
        self.seq_num_s2r = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_s2r(&self) -> u32 {
        self.seq_num_s2r.unwrap_or(0)
    }

    fn get_seq_num_s2r_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_s2r
    }

    fn mut_seq_num_s2r_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_s2r
    }

    // optional uint32 seq_num_s2c = 4;

    pub fn clear_seq_num_s2c(&mut self) {
        self.seq_num_s2c = ::std::option::Option::None;
    }

    pub fn has_seq_num_s2c(&self) -> bool {
        self.seq_num_s2c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_s2c(&mut self, v: u32) {
        self.seq_num_s2c = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_s2c(&self) -> u32 {
        self.seq_num_s2c.unwrap_or(0)
    }

    fn get_seq_num_s2c_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_s2c
    }

    fn mut_seq_num_s2c_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_s2c
    }

    // optional fixed64 client_steam_id = 5;

    pub fn clear_client_steam_id(&mut self) {
        self.client_steam_id = ::std::option::Option::None;
    }

    pub fn has_client_steam_id(&self) -> bool {
        self.client_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_steam_id(&mut self, v: u64) {
        self.client_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_steam_id(&self) -> u64 {
        self.client_steam_id.unwrap_or(0)
    }

    fn get_client_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.client_steam_id
    }

    fn mut_client_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.client_steam_id
    }

    // optional uint32 relay_session_id = 6;

    pub fn clear_relay_session_id(&mut self) {
        self.relay_session_id = ::std::option::Option::None;
    }

    pub fn has_relay_session_id(&self) -> bool {
        self.relay_session_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_relay_session_id(&mut self, v: u32) {
        self.relay_session_id = ::std::option::Option::Some(v);
    }

    pub fn get_relay_session_id(&self) -> u32 {
        self.relay_session_id.unwrap_or(0)
    }

    fn get_relay_session_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.relay_session_id
    }

    fn mut_relay_session_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.relay_session_id
    }

    // optional fixed32 client_connection_id = 7;

    pub fn clear_client_connection_id(&mut self) {
        self.client_connection_id = ::std::option::Option::None;
    }

    pub fn has_client_connection_id(&self) -> bool {
        self.client_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_connection_id(&mut self, v: u32) {
        self.client_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_connection_id(&self) -> u32 {
        self.client_connection_id.unwrap_or(0)
    }

    fn get_client_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_connection_id
    }

    fn mut_client_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_connection_id
    }

    // optional fixed32 server_connection_id = 11;

    pub fn clear_server_connection_id(&mut self) {
        self.server_connection_id = ::std::option::Option::None;
    }

    pub fn has_server_connection_id(&self) -> bool {
        self.server_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_connection_id(&mut self, v: u32) {
        self.server_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_server_connection_id(&self) -> u32 {
        self.server_connection_id.unwrap_or(0)
    }

    fn get_server_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_connection_id
    }

    fn mut_server_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_connection_id
    }
}

impl ::protobuf::Message for CMsgSteamDatagramConnectionStatsServerToRouter {
    fn is_initialized(&self) -> bool {
        for v in &self.s2r {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.s2c {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.s2r)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.s2c)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.ack_relay)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.ack_e2e)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seq_num_s2r = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seq_num_s2c = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.client_steam_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.relay_session_id = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.client_connection_id = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.server_connection_id = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.s2r.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.s2c.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += 5 * self.ack_relay.len() as u32;
        my_size += 5 * self.ack_e2e.len() as u32;
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.seq_num_s2r {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.seq_num_s2c {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.client_steam_id {
            my_size += 9;
        }
        if let Some(v) = self.relay_session_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.client_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.server_connection_id {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.s2r.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.s2c.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.ack_relay {
            os.write_fixed32(8, *v)?;
        };
        for v in &self.ack_e2e {
            os.write_fixed32(9, *v)?;
        };
        if let Some(v) = self.flags {
            os.write_uint32(10, v)?;
        }
        if let Some(v) = self.seq_num_s2r {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.seq_num_s2c {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.client_steam_id {
            os.write_fixed64(5, v)?;
        }
        if let Some(v) = self.relay_session_id {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.client_connection_id {
            os.write_fixed32(7, v)?;
        }
        if let Some(v) = self.server_connection_id {
            os.write_fixed32(11, v)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramConnectionStatsServerToRouter {
    fn new() -> CMsgSteamDatagramConnectionStatsServerToRouter {
        CMsgSteamDatagramConnectionStatsServerToRouter::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionStatsServerToRouter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "s2r",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_s2r_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_s2r_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "s2c",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_s2c_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_s2c_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "ack_relay",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_ack_relay_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_ack_relay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "ack_e2e",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_ack_e2e_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_ack_e2e_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_flags_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_s2r",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_seq_num_s2r_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_seq_num_s2r_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_s2c",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_seq_num_s2c_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_seq_num_s2c_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "client_steam_id",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_client_steam_id_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_client_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "relay_session_id",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_relay_session_id_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_relay_session_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_connection_id",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_client_connection_id_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_client_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "server_connection_id",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_server_connection_id_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_server_connection_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramConnectionStatsServerToRouter>(
                    "CMsgSteamDatagramConnectionStatsServerToRouter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramConnectionStatsServerToRouter {
    fn clear(&mut self) {
        self.clear_s2r();
        self.clear_s2c();
        self.clear_ack_relay();
        self.clear_ack_e2e();
        self.clear_flags();
        self.clear_seq_num_s2r();
        self.clear_seq_num_s2c();
        self.clear_client_steam_id();
        self.clear_relay_session_id();
        self.clear_client_connection_id();
        self.clear_server_connection_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramConnectionStatsServerToRouter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionStatsServerToRouter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgSteamDatagramConnectionStatsServerToRouter_Flags {
    ACK_REQUEST_RELAY = 1,
    ACK_REQUEST_E2E = 2,
    ACK_REQUEST_IMMEDIATE = 4,
}

impl ::protobuf::ProtobufEnum for CMsgSteamDatagramConnectionStatsServerToRouter_Flags {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgSteamDatagramConnectionStatsServerToRouter_Flags> {
        match value {
            1 => ::std::option::Option::Some(CMsgSteamDatagramConnectionStatsServerToRouter_Flags::ACK_REQUEST_RELAY),
            2 => ::std::option::Option::Some(CMsgSteamDatagramConnectionStatsServerToRouter_Flags::ACK_REQUEST_E2E),
            4 => ::std::option::Option::Some(CMsgSteamDatagramConnectionStatsServerToRouter_Flags::ACK_REQUEST_IMMEDIATE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgSteamDatagramConnectionStatsServerToRouter_Flags] = &[
            CMsgSteamDatagramConnectionStatsServerToRouter_Flags::ACK_REQUEST_RELAY,
            CMsgSteamDatagramConnectionStatsServerToRouter_Flags::ACK_REQUEST_E2E,
            CMsgSteamDatagramConnectionStatsServerToRouter_Flags::ACK_REQUEST_IMMEDIATE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionStatsServerToRouter_Flags>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgSteamDatagramConnectionStatsServerToRouter_Flags", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgSteamDatagramConnectionStatsServerToRouter_Flags {
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionStatsServerToRouter_Flags {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramClientPingSampleRequest {
    // message fields
    connection_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramClientPingSampleRequest {}

impl CMsgSteamDatagramClientPingSampleRequest {
    pub fn new() -> CMsgSteamDatagramClientPingSampleRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramClientPingSampleRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramClientPingSampleRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramClientPingSampleRequest,
        };
        unsafe {
            instance.get(CMsgSteamDatagramClientPingSampleRequest::new)
        }
    }

    // optional fixed32 connection_id = 1;

    pub fn clear_connection_id(&mut self) {
        self.connection_id = ::std::option::Option::None;
    }

    pub fn has_connection_id(&self) -> bool {
        self.connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connection_id(&mut self, v: u32) {
        self.connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_connection_id(&self) -> u32 {
        self.connection_id.unwrap_or(0)
    }

    fn get_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.connection_id
    }

    fn mut_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.connection_id
    }
}

impl ::protobuf::Message for CMsgSteamDatagramClientPingSampleRequest {
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
                    self.connection_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.connection_id {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.connection_id {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramClientPingSampleRequest {
    fn new() -> CMsgSteamDatagramClientPingSampleRequest {
        CMsgSteamDatagramClientPingSampleRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramClientPingSampleRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "connection_id",
                    CMsgSteamDatagramClientPingSampleRequest::get_connection_id_for_reflect,
                    CMsgSteamDatagramClientPingSampleRequest::mut_connection_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramClientPingSampleRequest>(
                    "CMsgSteamDatagramClientPingSampleRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramClientPingSampleRequest {
    fn clear(&mut self) {
        self.clear_connection_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramClientPingSampleRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramClientPingSampleRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramClientPingSampleReply {
    // message fields
    connection_id: ::std::option::Option<u32>,
    routing_clusters: ::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_RoutingCluster>,
    data_centers: ::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_DataCenter>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramClientPingSampleReply {}

impl CMsgSteamDatagramClientPingSampleReply {
    pub fn new() -> CMsgSteamDatagramClientPingSampleReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramClientPingSampleReply {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramClientPingSampleReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramClientPingSampleReply,
        };
        unsafe {
            instance.get(CMsgSteamDatagramClientPingSampleReply::new)
        }
    }

    // optional fixed32 connection_id = 1;

    pub fn clear_connection_id(&mut self) {
        self.connection_id = ::std::option::Option::None;
    }

    pub fn has_connection_id(&self) -> bool {
        self.connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connection_id(&mut self, v: u32) {
        self.connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_connection_id(&self) -> u32 {
        self.connection_id.unwrap_or(0)
    }

    fn get_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.connection_id
    }

    fn mut_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.connection_id
    }

    // repeated .CMsgSteamDatagramClientPingSampleReply.RoutingCluster routing_clusters = 2;

    pub fn clear_routing_clusters(&mut self) {
        self.routing_clusters.clear();
    }

    // Param is passed by value, moved
    pub fn set_routing_clusters(&mut self, v: ::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_RoutingCluster>) {
        self.routing_clusters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_routing_clusters(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_RoutingCluster> {
        &mut self.routing_clusters
    }

    // Take field
    pub fn take_routing_clusters(&mut self) -> ::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_RoutingCluster> {
        ::std::mem::replace(&mut self.routing_clusters, ::protobuf::RepeatedField::new())
    }

    pub fn get_routing_clusters(&self) -> &[CMsgSteamDatagramClientPingSampleReply_RoutingCluster] {
        &self.routing_clusters
    }

    fn get_routing_clusters_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_RoutingCluster> {
        &self.routing_clusters
    }

    fn mut_routing_clusters_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_RoutingCluster> {
        &mut self.routing_clusters
    }

    // repeated .CMsgSteamDatagramClientPingSampleReply.DataCenter data_centers = 3;

    pub fn clear_data_centers(&mut self) {
        self.data_centers.clear();
    }

    // Param is passed by value, moved
    pub fn set_data_centers(&mut self, v: ::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_DataCenter>) {
        self.data_centers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_data_centers(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_DataCenter> {
        &mut self.data_centers
    }

    // Take field
    pub fn take_data_centers(&mut self) -> ::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_DataCenter> {
        ::std::mem::replace(&mut self.data_centers, ::protobuf::RepeatedField::new())
    }

    pub fn get_data_centers(&self) -> &[CMsgSteamDatagramClientPingSampleReply_DataCenter] {
        &self.data_centers
    }

    fn get_data_centers_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_DataCenter> {
        &self.data_centers
    }

    fn mut_data_centers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_DataCenter> {
        &mut self.data_centers
    }
}

impl ::protobuf::Message for CMsgSteamDatagramClientPingSampleReply {
    fn is_initialized(&self) -> bool {
        for v in &self.routing_clusters {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.data_centers {
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
                    self.connection_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.routing_clusters)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.data_centers)?;
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
        if let Some(v) = self.connection_id {
            my_size += 5;
        }
        for value in &self.routing_clusters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.data_centers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.connection_id {
            os.write_fixed32(1, v)?;
        }
        for v in &self.routing_clusters {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.data_centers {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramClientPingSampleReply {
    fn new() -> CMsgSteamDatagramClientPingSampleReply {
        CMsgSteamDatagramClientPingSampleReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramClientPingSampleReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "connection_id",
                    CMsgSteamDatagramClientPingSampleReply::get_connection_id_for_reflect,
                    CMsgSteamDatagramClientPingSampleReply::mut_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramClientPingSampleReply_RoutingCluster>>(
                    "routing_clusters",
                    CMsgSteamDatagramClientPingSampleReply::get_routing_clusters_for_reflect,
                    CMsgSteamDatagramClientPingSampleReply::mut_routing_clusters_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramClientPingSampleReply_DataCenter>>(
                    "data_centers",
                    CMsgSteamDatagramClientPingSampleReply::get_data_centers_for_reflect,
                    CMsgSteamDatagramClientPingSampleReply::mut_data_centers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramClientPingSampleReply>(
                    "CMsgSteamDatagramClientPingSampleReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramClientPingSampleReply {
    fn clear(&mut self) {
        self.clear_connection_id();
        self.clear_routing_clusters();
        self.clear_data_centers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramClientPingSampleReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramClientPingSampleReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
    // message fields
    id: ::std::option::Option<u32>,
    front_ping_ms: ::std::option::Option<u32>,
    e2e_ping_ms: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramClientPingSampleReply_RoutingCluster {}

impl CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
    pub fn new() -> CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramClientPingSampleReply_RoutingCluster> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramClientPingSampleReply_RoutingCluster,
        };
        unsafe {
            instance.get(CMsgSteamDatagramClientPingSampleReply_RoutingCluster::new)
        }
    }

    // optional fixed32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.id
    }

    // optional uint32 front_ping_ms = 2;

    pub fn clear_front_ping_ms(&mut self) {
        self.front_ping_ms = ::std::option::Option::None;
    }

    pub fn has_front_ping_ms(&self) -> bool {
        self.front_ping_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_front_ping_ms(&mut self, v: u32) {
        self.front_ping_ms = ::std::option::Option::Some(v);
    }

    pub fn get_front_ping_ms(&self) -> u32 {
        self.front_ping_ms.unwrap_or(0)
    }

    fn get_front_ping_ms_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.front_ping_ms
    }

    fn mut_front_ping_ms_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.front_ping_ms
    }

    // optional uint32 e2e_ping_ms = 3;

    pub fn clear_e2e_ping_ms(&mut self) {
        self.e2e_ping_ms = ::std::option::Option::None;
    }

    pub fn has_e2e_ping_ms(&self) -> bool {
        self.e2e_ping_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_e2e_ping_ms(&mut self, v: u32) {
        self.e2e_ping_ms = ::std::option::Option::Some(v);
    }

    pub fn get_e2e_ping_ms(&self) -> u32 {
        self.e2e_ping_ms.unwrap_or(0)
    }

    fn get_e2e_ping_ms_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.e2e_ping_ms
    }

    fn mut_e2e_ping_ms_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.e2e_ping_ms
    }
}

impl ::protobuf::Message for CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
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
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.front_ping_ms = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.e2e_ping_ms = ::std::option::Option::Some(tmp);
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
            my_size += 5;
        }
        if let Some(v) = self.front_ping_ms {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.e2e_ping_ms {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.front_ping_ms {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.e2e_ping_ms {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
    fn new() -> CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
        CMsgSteamDatagramClientPingSampleReply_RoutingCluster::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramClientPingSampleReply_RoutingCluster>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "id",
                    CMsgSteamDatagramClientPingSampleReply_RoutingCluster::get_id_for_reflect,
                    CMsgSteamDatagramClientPingSampleReply_RoutingCluster::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "front_ping_ms",
                    CMsgSteamDatagramClientPingSampleReply_RoutingCluster::get_front_ping_ms_for_reflect,
                    CMsgSteamDatagramClientPingSampleReply_RoutingCluster::mut_front_ping_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "e2e_ping_ms",
                    CMsgSteamDatagramClientPingSampleReply_RoutingCluster::get_e2e_ping_ms_for_reflect,
                    CMsgSteamDatagramClientPingSampleReply_RoutingCluster::mut_e2e_ping_ms_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramClientPingSampleReply_RoutingCluster>(
                    "CMsgSteamDatagramClientPingSampleReply_RoutingCluster",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_front_ping_ms();
        self.clear_e2e_ping_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramClientPingSampleReply_DataCenter {
    // message fields
    data_center_id: ::std::option::Option<u32>,
    via_relay_id: ::std::option::Option<u32>,
    e2e_ping_ms: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramClientPingSampleReply_DataCenter {}

impl CMsgSteamDatagramClientPingSampleReply_DataCenter {
    pub fn new() -> CMsgSteamDatagramClientPingSampleReply_DataCenter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramClientPingSampleReply_DataCenter {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramClientPingSampleReply_DataCenter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramClientPingSampleReply_DataCenter,
        };
        unsafe {
            instance.get(CMsgSteamDatagramClientPingSampleReply_DataCenter::new)
        }
    }

    // optional fixed32 data_center_id = 1;

    pub fn clear_data_center_id(&mut self) {
        self.data_center_id = ::std::option::Option::None;
    }

    pub fn has_data_center_id(&self) -> bool {
        self.data_center_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_center_id(&mut self, v: u32) {
        self.data_center_id = ::std::option::Option::Some(v);
    }

    pub fn get_data_center_id(&self) -> u32 {
        self.data_center_id.unwrap_or(0)
    }

    fn get_data_center_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.data_center_id
    }

    fn mut_data_center_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.data_center_id
    }

    // optional fixed32 via_relay_id = 2;

    pub fn clear_via_relay_id(&mut self) {
        self.via_relay_id = ::std::option::Option::None;
    }

    pub fn has_via_relay_id(&self) -> bool {
        self.via_relay_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_via_relay_id(&mut self, v: u32) {
        self.via_relay_id = ::std::option::Option::Some(v);
    }

    pub fn get_via_relay_id(&self) -> u32 {
        self.via_relay_id.unwrap_or(0)
    }

    fn get_via_relay_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.via_relay_id
    }

    fn mut_via_relay_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.via_relay_id
    }

    // optional uint32 e2e_ping_ms = 3;

    pub fn clear_e2e_ping_ms(&mut self) {
        self.e2e_ping_ms = ::std::option::Option::None;
    }

    pub fn has_e2e_ping_ms(&self) -> bool {
        self.e2e_ping_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_e2e_ping_ms(&mut self, v: u32) {
        self.e2e_ping_ms = ::std::option::Option::Some(v);
    }

    pub fn get_e2e_ping_ms(&self) -> u32 {
        self.e2e_ping_ms.unwrap_or(0)
    }

    fn get_e2e_ping_ms_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.e2e_ping_ms
    }

    fn mut_e2e_ping_ms_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.e2e_ping_ms
    }
}

impl ::protobuf::Message for CMsgSteamDatagramClientPingSampleReply_DataCenter {
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
                    self.data_center_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.via_relay_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.e2e_ping_ms = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.data_center_id {
            my_size += 5;
        }
        if let Some(v) = self.via_relay_id {
            my_size += 5;
        }
        if let Some(v) = self.e2e_ping_ms {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.data_center_id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.via_relay_id {
            os.write_fixed32(2, v)?;
        }
        if let Some(v) = self.e2e_ping_ms {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramClientPingSampleReply_DataCenter {
    fn new() -> CMsgSteamDatagramClientPingSampleReply_DataCenter {
        CMsgSteamDatagramClientPingSampleReply_DataCenter::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramClientPingSampleReply_DataCenter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "data_center_id",
                    CMsgSteamDatagramClientPingSampleReply_DataCenter::get_data_center_id_for_reflect,
                    CMsgSteamDatagramClientPingSampleReply_DataCenter::mut_data_center_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "via_relay_id",
                    CMsgSteamDatagramClientPingSampleReply_DataCenter::get_via_relay_id_for_reflect,
                    CMsgSteamDatagramClientPingSampleReply_DataCenter::mut_via_relay_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "e2e_ping_ms",
                    CMsgSteamDatagramClientPingSampleReply_DataCenter::get_e2e_ping_ms_for_reflect,
                    CMsgSteamDatagramClientPingSampleReply_DataCenter::mut_e2e_ping_ms_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramClientPingSampleReply_DataCenter>(
                    "CMsgSteamDatagramClientPingSampleReply_DataCenter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramClientPingSampleReply_DataCenter {
    fn clear(&mut self) {
        self.clear_data_center_id();
        self.clear_via_relay_id();
        self.clear_e2e_ping_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramClientPingSampleReply_DataCenter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramClientPingSampleReply_DataCenter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramClientSwitchedPrimary {
    // message fields
    connection_id: ::std::option::Option<u32>,
    from_ip: ::std::option::Option<u32>,
    from_port: ::std::option::Option<u32>,
    from_router_cluster: ::std::option::Option<u32>,
    from_active_time: ::std::option::Option<u32>,
    from_active_packets_recv: ::std::option::Option<u32>,
    from_dropped_reason: ::protobuf::SingularField<::std::string::String>,
    gap_ms: ::std::option::Option<u32>,
    from_quality_now: ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>,
    to_quality_now: ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>,
    from_quality_then: ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>,
    to_quality_then: ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramClientSwitchedPrimary {}

impl CMsgSteamDatagramClientSwitchedPrimary {
    pub fn new() -> CMsgSteamDatagramClientSwitchedPrimary {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramClientSwitchedPrimary {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramClientSwitchedPrimary> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramClientSwitchedPrimary,
        };
        unsafe {
            instance.get(CMsgSteamDatagramClientSwitchedPrimary::new)
        }
    }

    // optional fixed32 connection_id = 1;

    pub fn clear_connection_id(&mut self) {
        self.connection_id = ::std::option::Option::None;
    }

    pub fn has_connection_id(&self) -> bool {
        self.connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connection_id(&mut self, v: u32) {
        self.connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_connection_id(&self) -> u32 {
        self.connection_id.unwrap_or(0)
    }

    fn get_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.connection_id
    }

    fn mut_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.connection_id
    }

    // optional fixed32 from_ip = 2;

    pub fn clear_from_ip(&mut self) {
        self.from_ip = ::std::option::Option::None;
    }

    pub fn has_from_ip(&self) -> bool {
        self.from_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_ip(&mut self, v: u32) {
        self.from_ip = ::std::option::Option::Some(v);
    }

    pub fn get_from_ip(&self) -> u32 {
        self.from_ip.unwrap_or(0)
    }

    fn get_from_ip_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.from_ip
    }

    fn mut_from_ip_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.from_ip
    }

    // optional uint32 from_port = 3;

    pub fn clear_from_port(&mut self) {
        self.from_port = ::std::option::Option::None;
    }

    pub fn has_from_port(&self) -> bool {
        self.from_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_port(&mut self, v: u32) {
        self.from_port = ::std::option::Option::Some(v);
    }

    pub fn get_from_port(&self) -> u32 {
        self.from_port.unwrap_or(0)
    }

    fn get_from_port_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.from_port
    }

    fn mut_from_port_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.from_port
    }

    // optional fixed32 from_router_cluster = 4;

    pub fn clear_from_router_cluster(&mut self) {
        self.from_router_cluster = ::std::option::Option::None;
    }

    pub fn has_from_router_cluster(&self) -> bool {
        self.from_router_cluster.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_router_cluster(&mut self, v: u32) {
        self.from_router_cluster = ::std::option::Option::Some(v);
    }

    pub fn get_from_router_cluster(&self) -> u32 {
        self.from_router_cluster.unwrap_or(0)
    }

    fn get_from_router_cluster_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.from_router_cluster
    }

    fn mut_from_router_cluster_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.from_router_cluster
    }

    // optional uint32 from_active_time = 5;

    pub fn clear_from_active_time(&mut self) {
        self.from_active_time = ::std::option::Option::None;
    }

    pub fn has_from_active_time(&self) -> bool {
        self.from_active_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_active_time(&mut self, v: u32) {
        self.from_active_time = ::std::option::Option::Some(v);
    }

    pub fn get_from_active_time(&self) -> u32 {
        self.from_active_time.unwrap_or(0)
    }

    fn get_from_active_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.from_active_time
    }

    fn mut_from_active_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.from_active_time
    }

    // optional uint32 from_active_packets_recv = 6;

    pub fn clear_from_active_packets_recv(&mut self) {
        self.from_active_packets_recv = ::std::option::Option::None;
    }

    pub fn has_from_active_packets_recv(&self) -> bool {
        self.from_active_packets_recv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_active_packets_recv(&mut self, v: u32) {
        self.from_active_packets_recv = ::std::option::Option::Some(v);
    }

    pub fn get_from_active_packets_recv(&self) -> u32 {
        self.from_active_packets_recv.unwrap_or(0)
    }

    fn get_from_active_packets_recv_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.from_active_packets_recv
    }

    fn mut_from_active_packets_recv_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.from_active_packets_recv
    }

    // optional string from_dropped_reason = 7;

    pub fn clear_from_dropped_reason(&mut self) {
        self.from_dropped_reason.clear();
    }

    pub fn has_from_dropped_reason(&self) -> bool {
        self.from_dropped_reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_dropped_reason(&mut self, v: ::std::string::String) {
        self.from_dropped_reason = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from_dropped_reason(&mut self) -> &mut ::std::string::String {
        if self.from_dropped_reason.is_none() {
            self.from_dropped_reason.set_default();
        }
        self.from_dropped_reason.as_mut().unwrap()
    }

    // Take field
    pub fn take_from_dropped_reason(&mut self) -> ::std::string::String {
        self.from_dropped_reason.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_from_dropped_reason(&self) -> &str {
        match self.from_dropped_reason.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_from_dropped_reason_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.from_dropped_reason
    }

    fn mut_from_dropped_reason_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.from_dropped_reason
    }

    // optional uint32 gap_ms = 8;

    pub fn clear_gap_ms(&mut self) {
        self.gap_ms = ::std::option::Option::None;
    }

    pub fn has_gap_ms(&self) -> bool {
        self.gap_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gap_ms(&mut self, v: u32) {
        self.gap_ms = ::std::option::Option::Some(v);
    }

    pub fn get_gap_ms(&self) -> u32 {
        self.gap_ms.unwrap_or(0)
    }

    fn get_gap_ms_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.gap_ms
    }

    fn mut_gap_ms_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.gap_ms
    }

    // optional .CMsgSteamDatagramClientSwitchedPrimary.RouterQuality from_quality_now = 9;

    pub fn clear_from_quality_now(&mut self) {
        self.from_quality_now.clear();
    }

    pub fn has_from_quality_now(&self) -> bool {
        self.from_quality_now.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_quality_now(&mut self, v: CMsgSteamDatagramClientSwitchedPrimary_RouterQuality) {
        self.from_quality_now = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from_quality_now(&mut self) -> &mut CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        if self.from_quality_now.is_none() {
            self.from_quality_now.set_default();
        }
        self.from_quality_now.as_mut().unwrap()
    }

    // Take field
    pub fn take_from_quality_now(&mut self) -> CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.from_quality_now.take().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::new())
    }

    pub fn get_from_quality_now(&self) -> &CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.from_quality_now.as_ref().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::default_instance())
    }

    fn get_from_quality_now_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &self.from_quality_now
    }

    fn mut_from_quality_now_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &mut self.from_quality_now
    }

    // optional .CMsgSteamDatagramClientSwitchedPrimary.RouterQuality to_quality_now = 10;

    pub fn clear_to_quality_now(&mut self) {
        self.to_quality_now.clear();
    }

    pub fn has_to_quality_now(&self) -> bool {
        self.to_quality_now.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to_quality_now(&mut self, v: CMsgSteamDatagramClientSwitchedPrimary_RouterQuality) {
        self.to_quality_now = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to_quality_now(&mut self) -> &mut CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        if self.to_quality_now.is_none() {
            self.to_quality_now.set_default();
        }
        self.to_quality_now.as_mut().unwrap()
    }

    // Take field
    pub fn take_to_quality_now(&mut self) -> CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.to_quality_now.take().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::new())
    }

    pub fn get_to_quality_now(&self) -> &CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.to_quality_now.as_ref().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::default_instance())
    }

    fn get_to_quality_now_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &self.to_quality_now
    }

    fn mut_to_quality_now_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &mut self.to_quality_now
    }

    // optional .CMsgSteamDatagramClientSwitchedPrimary.RouterQuality from_quality_then = 11;

    pub fn clear_from_quality_then(&mut self) {
        self.from_quality_then.clear();
    }

    pub fn has_from_quality_then(&self) -> bool {
        self.from_quality_then.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_quality_then(&mut self, v: CMsgSteamDatagramClientSwitchedPrimary_RouterQuality) {
        self.from_quality_then = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from_quality_then(&mut self) -> &mut CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        if self.from_quality_then.is_none() {
            self.from_quality_then.set_default();
        }
        self.from_quality_then.as_mut().unwrap()
    }

    // Take field
    pub fn take_from_quality_then(&mut self) -> CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.from_quality_then.take().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::new())
    }

    pub fn get_from_quality_then(&self) -> &CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.from_quality_then.as_ref().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::default_instance())
    }

    fn get_from_quality_then_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &self.from_quality_then
    }

    fn mut_from_quality_then_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &mut self.from_quality_then
    }

    // optional .CMsgSteamDatagramClientSwitchedPrimary.RouterQuality to_quality_then = 12;

    pub fn clear_to_quality_then(&mut self) {
        self.to_quality_then.clear();
    }

    pub fn has_to_quality_then(&self) -> bool {
        self.to_quality_then.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to_quality_then(&mut self, v: CMsgSteamDatagramClientSwitchedPrimary_RouterQuality) {
        self.to_quality_then = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to_quality_then(&mut self) -> &mut CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        if self.to_quality_then.is_none() {
            self.to_quality_then.set_default();
        }
        self.to_quality_then.as_mut().unwrap()
    }

    // Take field
    pub fn take_to_quality_then(&mut self) -> CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.to_quality_then.take().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::new())
    }

    pub fn get_to_quality_then(&self) -> &CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.to_quality_then.as_ref().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::default_instance())
    }

    fn get_to_quality_then_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &self.to_quality_then
    }

    fn mut_to_quality_then_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &mut self.to_quality_then
    }
}

impl ::protobuf::Message for CMsgSteamDatagramClientSwitchedPrimary {
    fn is_initialized(&self) -> bool {
        for v in &self.from_quality_now {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.to_quality_now {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.from_quality_then {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.to_quality_then {
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
                    self.connection_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.from_ip = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.from_port = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.from_router_cluster = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.from_active_time = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.from_active_packets_recv = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.from_dropped_reason)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.gap_ms = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.from_quality_now)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.to_quality_now)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.from_quality_then)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.to_quality_then)?;
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
        if let Some(v) = self.connection_id {
            my_size += 5;
        }
        if let Some(v) = self.from_ip {
            my_size += 5;
        }
        if let Some(v) = self.from_port {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.from_router_cluster {
            my_size += 5;
        }
        if let Some(v) = self.from_active_time {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.from_active_packets_recv {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.from_dropped_reason.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        if let Some(v) = self.gap_ms {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.from_quality_now.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.to_quality_now.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.from_quality_then.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.to_quality_then.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.connection_id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.from_ip {
            os.write_fixed32(2, v)?;
        }
        if let Some(v) = self.from_port {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.from_router_cluster {
            os.write_fixed32(4, v)?;
        }
        if let Some(v) = self.from_active_time {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.from_active_packets_recv {
            os.write_uint32(6, v)?;
        }
        if let Some(ref v) = self.from_dropped_reason.as_ref() {
            os.write_string(7, &v)?;
        }
        if let Some(v) = self.gap_ms {
            os.write_uint32(8, v)?;
        }
        if let Some(ref v) = self.from_quality_now.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.to_quality_now.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.from_quality_then.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.to_quality_then.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramClientSwitchedPrimary {
    fn new() -> CMsgSteamDatagramClientSwitchedPrimary {
        CMsgSteamDatagramClientSwitchedPrimary::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramClientSwitchedPrimary>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "connection_id",
                    CMsgSteamDatagramClientSwitchedPrimary::get_connection_id_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "from_ip",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_ip_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "from_port",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_port_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "from_router_cluster",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_router_cluster_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_router_cluster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "from_active_time",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_active_time_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_active_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "from_active_packets_recv",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_active_packets_recv_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_active_packets_recv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "from_dropped_reason",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_dropped_reason_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_dropped_reason_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "gap_ms",
                    CMsgSteamDatagramClientSwitchedPrimary::get_gap_ms_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_gap_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>>(
                    "from_quality_now",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_quality_now_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_quality_now_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>>(
                    "to_quality_now",
                    CMsgSteamDatagramClientSwitchedPrimary::get_to_quality_now_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_to_quality_now_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>>(
                    "from_quality_then",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_quality_then_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_quality_then_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>>(
                    "to_quality_then",
                    CMsgSteamDatagramClientSwitchedPrimary::get_to_quality_then_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_to_quality_then_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramClientSwitchedPrimary>(
                    "CMsgSteamDatagramClientSwitchedPrimary",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramClientSwitchedPrimary {
    fn clear(&mut self) {
        self.clear_connection_id();
        self.clear_from_ip();
        self.clear_from_port();
        self.clear_from_router_cluster();
        self.clear_from_active_time();
        self.clear_from_active_packets_recv();
        self.clear_from_dropped_reason();
        self.clear_gap_ms();
        self.clear_from_quality_now();
        self.clear_to_quality_now();
        self.clear_from_quality_then();
        self.clear_to_quality_then();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramClientSwitchedPrimary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramClientSwitchedPrimary {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
    // message fields
    score: ::std::option::Option<u32>,
    front_ping: ::std::option::Option<u32>,
    back_ping: ::std::option::Option<u32>,
    seconds_until_down: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {}

impl CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
    pub fn new() -> CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramClientSwitchedPrimary_RouterQuality,
        };
        unsafe {
            instance.get(CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::new)
        }
    }

    // optional uint32 score = 1;

    pub fn clear_score(&mut self) {
        self.score = ::std::option::Option::None;
    }

    pub fn has_score(&self) -> bool {
        self.score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_score(&mut self, v: u32) {
        self.score = ::std::option::Option::Some(v);
    }

    pub fn get_score(&self) -> u32 {
        self.score.unwrap_or(0)
    }

    fn get_score_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.score
    }

    fn mut_score_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.score
    }

    // optional uint32 front_ping = 2;

    pub fn clear_front_ping(&mut self) {
        self.front_ping = ::std::option::Option::None;
    }

    pub fn has_front_ping(&self) -> bool {
        self.front_ping.is_some()
    }

    // Param is passed by value, moved
    pub fn set_front_ping(&mut self, v: u32) {
        self.front_ping = ::std::option::Option::Some(v);
    }

    pub fn get_front_ping(&self) -> u32 {
        self.front_ping.unwrap_or(0)
    }

    fn get_front_ping_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.front_ping
    }

    fn mut_front_ping_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.front_ping
    }

    // optional uint32 back_ping = 3;

    pub fn clear_back_ping(&mut self) {
        self.back_ping = ::std::option::Option::None;
    }

    pub fn has_back_ping(&self) -> bool {
        self.back_ping.is_some()
    }

    // Param is passed by value, moved
    pub fn set_back_ping(&mut self, v: u32) {
        self.back_ping = ::std::option::Option::Some(v);
    }

    pub fn get_back_ping(&self) -> u32 {
        self.back_ping.unwrap_or(0)
    }

    fn get_back_ping_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.back_ping
    }

    fn mut_back_ping_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.back_ping
    }

    // optional uint32 seconds_until_down = 4;

    pub fn clear_seconds_until_down(&mut self) {
        self.seconds_until_down = ::std::option::Option::None;
    }

    pub fn has_seconds_until_down(&self) -> bool {
        self.seconds_until_down.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seconds_until_down(&mut self, v: u32) {
        self.seconds_until_down = ::std::option::Option::Some(v);
    }

    pub fn get_seconds_until_down(&self) -> u32 {
        self.seconds_until_down.unwrap_or(0)
    }

    fn get_seconds_until_down_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seconds_until_down
    }

    fn mut_seconds_until_down_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seconds_until_down
    }
}

impl ::protobuf::Message for CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
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
                    self.score = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.front_ping = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.back_ping = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seconds_until_down = ::std::option::Option::Some(tmp);
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
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.front_ping {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.back_ping {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.seconds_until_down {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.score {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.front_ping {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.back_ping {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.seconds_until_down {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
    fn new() -> CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "score",
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::get_score_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::mut_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "front_ping",
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::get_front_ping_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::mut_front_ping_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "back_ping",
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::get_back_ping_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::mut_back_ping_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seconds_until_down",
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::get_seconds_until_down_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::mut_seconds_until_down_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>(
                    "CMsgSteamDatagramClientSwitchedPrimary_RouterQuality",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
    fn clear(&mut self) {
        self.clear_score();
        self.clear_front_ping();
        self.clear_back_ping();
        self.clear_seconds_until_down();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramRouterHealth {
    // message fields
    cpu_load: ::std::option::Option<f32>,
    active_sessions: ::std::option::Option<u32>,
    data_pkts_sec: ::std::option::Option<u32>,
    other_pkts_sec: ::std::option::Option<u32>,
    seconds_until_shutdown: ::std::option::Option<u32>,
    cpu_cost_per_user: ::std::option::Option<f32>,
    cpu_cost_per_packet: ::std::option::Option<f32>,
    data_centers: ::protobuf::RepeatedField<CMsgSteamDatagramRouterHealth_DataCenter>,
    magic: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramRouterHealth {}

impl CMsgSteamDatagramRouterHealth {
    pub fn new() -> CMsgSteamDatagramRouterHealth {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramRouterHealth {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramRouterHealth> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramRouterHealth,
        };
        unsafe {
            instance.get(CMsgSteamDatagramRouterHealth::new)
        }
    }

    // optional float cpu_load = 1;

    pub fn clear_cpu_load(&mut self) {
        self.cpu_load = ::std::option::Option::None;
    }

    pub fn has_cpu_load(&self) -> bool {
        self.cpu_load.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpu_load(&mut self, v: f32) {
        self.cpu_load = ::std::option::Option::Some(v);
    }

    pub fn get_cpu_load(&self) -> f32 {
        self.cpu_load.unwrap_or(0.)
    }

    fn get_cpu_load_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.cpu_load
    }

    fn mut_cpu_load_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.cpu_load
    }

    // optional uint32 active_sessions = 2;

    pub fn clear_active_sessions(&mut self) {
        self.active_sessions = ::std::option::Option::None;
    }

    pub fn has_active_sessions(&self) -> bool {
        self.active_sessions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_active_sessions(&mut self, v: u32) {
        self.active_sessions = ::std::option::Option::Some(v);
    }

    pub fn get_active_sessions(&self) -> u32 {
        self.active_sessions.unwrap_or(0)
    }

    fn get_active_sessions_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.active_sessions
    }

    fn mut_active_sessions_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.active_sessions
    }

    // optional uint32 data_pkts_sec = 3;

    pub fn clear_data_pkts_sec(&mut self) {
        self.data_pkts_sec = ::std::option::Option::None;
    }

    pub fn has_data_pkts_sec(&self) -> bool {
        self.data_pkts_sec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_pkts_sec(&mut self, v: u32) {
        self.data_pkts_sec = ::std::option::Option::Some(v);
    }

    pub fn get_data_pkts_sec(&self) -> u32 {
        self.data_pkts_sec.unwrap_or(0)
    }

    fn get_data_pkts_sec_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.data_pkts_sec
    }

    fn mut_data_pkts_sec_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.data_pkts_sec
    }

    // optional uint32 other_pkts_sec = 4;

    pub fn clear_other_pkts_sec(&mut self) {
        self.other_pkts_sec = ::std::option::Option::None;
    }

    pub fn has_other_pkts_sec(&self) -> bool {
        self.other_pkts_sec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_other_pkts_sec(&mut self, v: u32) {
        self.other_pkts_sec = ::std::option::Option::Some(v);
    }

    pub fn get_other_pkts_sec(&self) -> u32 {
        self.other_pkts_sec.unwrap_or(0)
    }

    fn get_other_pkts_sec_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.other_pkts_sec
    }

    fn mut_other_pkts_sec_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.other_pkts_sec
    }

    // optional uint32 seconds_until_shutdown = 5;

    pub fn clear_seconds_until_shutdown(&mut self) {
        self.seconds_until_shutdown = ::std::option::Option::None;
    }

    pub fn has_seconds_until_shutdown(&self) -> bool {
        self.seconds_until_shutdown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seconds_until_shutdown(&mut self, v: u32) {
        self.seconds_until_shutdown = ::std::option::Option::Some(v);
    }

    pub fn get_seconds_until_shutdown(&self) -> u32 {
        self.seconds_until_shutdown.unwrap_or(0)
    }

    fn get_seconds_until_shutdown_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seconds_until_shutdown
    }

    fn mut_seconds_until_shutdown_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seconds_until_shutdown
    }

    // optional float cpu_cost_per_user = 8;

    pub fn clear_cpu_cost_per_user(&mut self) {
        self.cpu_cost_per_user = ::std::option::Option::None;
    }

    pub fn has_cpu_cost_per_user(&self) -> bool {
        self.cpu_cost_per_user.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpu_cost_per_user(&mut self, v: f32) {
        self.cpu_cost_per_user = ::std::option::Option::Some(v);
    }

    pub fn get_cpu_cost_per_user(&self) -> f32 {
        self.cpu_cost_per_user.unwrap_or(0.)
    }

    fn get_cpu_cost_per_user_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.cpu_cost_per_user
    }

    fn mut_cpu_cost_per_user_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.cpu_cost_per_user
    }

    // optional float cpu_cost_per_packet = 9;

    pub fn clear_cpu_cost_per_packet(&mut self) {
        self.cpu_cost_per_packet = ::std::option::Option::None;
    }

    pub fn has_cpu_cost_per_packet(&self) -> bool {
        self.cpu_cost_per_packet.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpu_cost_per_packet(&mut self, v: f32) {
        self.cpu_cost_per_packet = ::std::option::Option::Some(v);
    }

    pub fn get_cpu_cost_per_packet(&self) -> f32 {
        self.cpu_cost_per_packet.unwrap_or(0.)
    }

    fn get_cpu_cost_per_packet_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.cpu_cost_per_packet
    }

    fn mut_cpu_cost_per_packet_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.cpu_cost_per_packet
    }

    // repeated .CMsgSteamDatagramRouterHealth.DataCenter data_centers = 6;

    pub fn clear_data_centers(&mut self) {
        self.data_centers.clear();
    }

    // Param is passed by value, moved
    pub fn set_data_centers(&mut self, v: ::protobuf::RepeatedField<CMsgSteamDatagramRouterHealth_DataCenter>) {
        self.data_centers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_data_centers(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramRouterHealth_DataCenter> {
        &mut self.data_centers
    }

    // Take field
    pub fn take_data_centers(&mut self) -> ::protobuf::RepeatedField<CMsgSteamDatagramRouterHealth_DataCenter> {
        ::std::mem::replace(&mut self.data_centers, ::protobuf::RepeatedField::new())
    }

    pub fn get_data_centers(&self) -> &[CMsgSteamDatagramRouterHealth_DataCenter] {
        &self.data_centers
    }

    fn get_data_centers_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSteamDatagramRouterHealth_DataCenter> {
        &self.data_centers
    }

    fn mut_data_centers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramRouterHealth_DataCenter> {
        &mut self.data_centers
    }

    // optional fixed64 magic = 7;

    pub fn clear_magic(&mut self) {
        self.magic = ::std::option::Option::None;
    }

    pub fn has_magic(&self) -> bool {
        self.magic.is_some()
    }

    // Param is passed by value, moved
    pub fn set_magic(&mut self, v: u64) {
        self.magic = ::std::option::Option::Some(v);
    }

    pub fn get_magic(&self) -> u64 {
        self.magic.unwrap_or(0)
    }

    fn get_magic_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.magic
    }

    fn mut_magic_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.magic
    }
}

impl ::protobuf::Message for CMsgSteamDatagramRouterHealth {
    fn is_initialized(&self) -> bool {
        for v in &self.data_centers {
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
                    let tmp = is.read_float()?;
                    self.cpu_load = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.active_sessions = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.data_pkts_sec = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.other_pkts_sec = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seconds_until_shutdown = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.cpu_cost_per_user = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.cpu_cost_per_packet = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.data_centers)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.magic = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.cpu_load {
            my_size += 5;
        }
        if let Some(v) = self.active_sessions {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.data_pkts_sec {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.other_pkts_sec {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.seconds_until_shutdown {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.cpu_cost_per_user {
            my_size += 5;
        }
        if let Some(v) = self.cpu_cost_per_packet {
            my_size += 5;
        }
        for value in &self.data_centers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.magic {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cpu_load {
            os.write_float(1, v)?;
        }
        if let Some(v) = self.active_sessions {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.data_pkts_sec {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.other_pkts_sec {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.seconds_until_shutdown {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.cpu_cost_per_user {
            os.write_float(8, v)?;
        }
        if let Some(v) = self.cpu_cost_per_packet {
            os.write_float(9, v)?;
        }
        for v in &self.data_centers {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.magic {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramRouterHealth {
    fn new() -> CMsgSteamDatagramRouterHealth {
        CMsgSteamDatagramRouterHealth::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramRouterHealth>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "cpu_load",
                    CMsgSteamDatagramRouterHealth::get_cpu_load_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_cpu_load_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "active_sessions",
                    CMsgSteamDatagramRouterHealth::get_active_sessions_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_active_sessions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "data_pkts_sec",
                    CMsgSteamDatagramRouterHealth::get_data_pkts_sec_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_data_pkts_sec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "other_pkts_sec",
                    CMsgSteamDatagramRouterHealth::get_other_pkts_sec_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_other_pkts_sec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seconds_until_shutdown",
                    CMsgSteamDatagramRouterHealth::get_seconds_until_shutdown_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_seconds_until_shutdown_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "cpu_cost_per_user",
                    CMsgSteamDatagramRouterHealth::get_cpu_cost_per_user_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_cpu_cost_per_user_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "cpu_cost_per_packet",
                    CMsgSteamDatagramRouterHealth::get_cpu_cost_per_packet_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_cpu_cost_per_packet_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramRouterHealth_DataCenter>>(
                    "data_centers",
                    CMsgSteamDatagramRouterHealth::get_data_centers_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_data_centers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "magic",
                    CMsgSteamDatagramRouterHealth::get_magic_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_magic_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramRouterHealth>(
                    "CMsgSteamDatagramRouterHealth",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramRouterHealth {
    fn clear(&mut self) {
        self.clear_cpu_load();
        self.clear_active_sessions();
        self.clear_data_pkts_sec();
        self.clear_other_pkts_sec();
        self.clear_seconds_until_shutdown();
        self.clear_cpu_cost_per_user();
        self.clear_cpu_cost_per_packet();
        self.clear_data_centers();
        self.clear_magic();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramRouterHealth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramRouterHealth {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramRouterHealth_DataCenter {
    // message fields
    datacenter_id: ::std::option::Option<u32>,
    state: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramRouterHealth_DataCenter {}

impl CMsgSteamDatagramRouterHealth_DataCenter {
    pub fn new() -> CMsgSteamDatagramRouterHealth_DataCenter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramRouterHealth_DataCenter {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramRouterHealth_DataCenter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramRouterHealth_DataCenter,
        };
        unsafe {
            instance.get(CMsgSteamDatagramRouterHealth_DataCenter::new)
        }
    }

    // optional fixed32 datacenter_id = 1;

    pub fn clear_datacenter_id(&mut self) {
        self.datacenter_id = ::std::option::Option::None;
    }

    pub fn has_datacenter_id(&self) -> bool {
        self.datacenter_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_datacenter_id(&mut self, v: u32) {
        self.datacenter_id = ::std::option::Option::Some(v);
    }

    pub fn get_datacenter_id(&self) -> u32 {
        self.datacenter_id.unwrap_or(0)
    }

    fn get_datacenter_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.datacenter_id
    }

    fn mut_datacenter_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.datacenter_id
    }

    // optional uint32 state = 2;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: u32) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> u32 {
        self.state.unwrap_or(0)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.state
    }
}

impl ::protobuf::Message for CMsgSteamDatagramRouterHealth_DataCenter {
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
                    self.datacenter_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.state = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.datacenter_id {
            my_size += 5;
        }
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.datacenter_id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.state {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramRouterHealth_DataCenter {
    fn new() -> CMsgSteamDatagramRouterHealth_DataCenter {
        CMsgSteamDatagramRouterHealth_DataCenter::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramRouterHealth_DataCenter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "datacenter_id",
                    CMsgSteamDatagramRouterHealth_DataCenter::get_datacenter_id_for_reflect,
                    CMsgSteamDatagramRouterHealth_DataCenter::mut_datacenter_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "state",
                    CMsgSteamDatagramRouterHealth_DataCenter::get_state_for_reflect,
                    CMsgSteamDatagramRouterHealth_DataCenter::mut_state_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramRouterHealth_DataCenter>(
                    "CMsgSteamDatagramRouterHealth_DataCenter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramRouterHealth_DataCenter {
    fn clear(&mut self) {
        self.clear_datacenter_id();
        self.clear_state();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramRouterHealth_DataCenter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramRouterHealth_DataCenter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramConnectRequest {
    // message fields
    connection_id: ::std::option::Option<u32>,
    relay_session_id: ::std::option::Option<u32>,
    client_steam_id: ::std::option::Option<u64>,
    my_timestamp: ::std::option::Option<u64>,
    ping_est_ms: ::std::option::Option<u32>,
    crypt: ::protobuf::SingularPtrField<CMsgSteamDatagramSessionCryptInfoSigned>,
    cert: ::protobuf::SingularPtrField<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned>,
    protocol_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramConnectRequest {}

impl CMsgSteamDatagramConnectRequest {
    pub fn new() -> CMsgSteamDatagramConnectRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramConnectRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramConnectRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramConnectRequest,
        };
        unsafe {
            instance.get(CMsgSteamDatagramConnectRequest::new)
        }
    }

    // optional fixed32 connection_id = 1;

    pub fn clear_connection_id(&mut self) {
        self.connection_id = ::std::option::Option::None;
    }

    pub fn has_connection_id(&self) -> bool {
        self.connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connection_id(&mut self, v: u32) {
        self.connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_connection_id(&self) -> u32 {
        self.connection_id.unwrap_or(0)
    }

    fn get_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.connection_id
    }

    fn mut_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.connection_id
    }

    // optional uint32 relay_session_id = 2;

    pub fn clear_relay_session_id(&mut self) {
        self.relay_session_id = ::std::option::Option::None;
    }

    pub fn has_relay_session_id(&self) -> bool {
        self.relay_session_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_relay_session_id(&mut self, v: u32) {
        self.relay_session_id = ::std::option::Option::Some(v);
    }

    pub fn get_relay_session_id(&self) -> u32 {
        self.relay_session_id.unwrap_or(0)
    }

    fn get_relay_session_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.relay_session_id
    }

    fn mut_relay_session_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.relay_session_id
    }

    // optional fixed64 client_steam_id = 3;

    pub fn clear_client_steam_id(&mut self) {
        self.client_steam_id = ::std::option::Option::None;
    }

    pub fn has_client_steam_id(&self) -> bool {
        self.client_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_steam_id(&mut self, v: u64) {
        self.client_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_steam_id(&self) -> u64 {
        self.client_steam_id.unwrap_or(0)
    }

    fn get_client_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.client_steam_id
    }

    fn mut_client_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.client_steam_id
    }

    // optional fixed64 my_timestamp = 4;

    pub fn clear_my_timestamp(&mut self) {
        self.my_timestamp = ::std::option::Option::None;
    }

    pub fn has_my_timestamp(&self) -> bool {
        self.my_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_my_timestamp(&mut self, v: u64) {
        self.my_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_my_timestamp(&self) -> u64 {
        self.my_timestamp.unwrap_or(0)
    }

    fn get_my_timestamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.my_timestamp
    }

    fn mut_my_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.my_timestamp
    }

    // optional uint32 ping_est_ms = 5;

    pub fn clear_ping_est_ms(&mut self) {
        self.ping_est_ms = ::std::option::Option::None;
    }

    pub fn has_ping_est_ms(&self) -> bool {
        self.ping_est_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_est_ms(&mut self, v: u32) {
        self.ping_est_ms = ::std::option::Option::Some(v);
    }

    pub fn get_ping_est_ms(&self) -> u32 {
        self.ping_est_ms.unwrap_or(0)
    }

    fn get_ping_est_ms_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_est_ms
    }

    fn mut_ping_est_ms_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_est_ms
    }

    // optional .CMsgSteamDatagramSessionCryptInfoSigned crypt = 6;

    pub fn clear_crypt(&mut self) {
        self.crypt.clear();
    }

    pub fn has_crypt(&self) -> bool {
        self.crypt.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crypt(&mut self, v: CMsgSteamDatagramSessionCryptInfoSigned) {
        self.crypt = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_crypt(&mut self) -> &mut CMsgSteamDatagramSessionCryptInfoSigned {
        if self.crypt.is_none() {
            self.crypt.set_default();
        }
        self.crypt.as_mut().unwrap()
    }

    // Take field
    pub fn take_crypt(&mut self) -> CMsgSteamDatagramSessionCryptInfoSigned {
        self.crypt.take().unwrap_or_else(|| CMsgSteamDatagramSessionCryptInfoSigned::new())
    }

    pub fn get_crypt(&self) -> &CMsgSteamDatagramSessionCryptInfoSigned {
        self.crypt.as_ref().unwrap_or_else(|| CMsgSteamDatagramSessionCryptInfoSigned::default_instance())
    }

    fn get_crypt_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramSessionCryptInfoSigned> {
        &self.crypt
    }

    fn mut_crypt_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramSessionCryptInfoSigned> {
        &mut self.crypt
    }

    // optional .CMsgSteamDatagramCertificateSigned cert = 7;

    pub fn clear_cert(&mut self) {
        self.cert.clear();
    }

    pub fn has_cert(&self) -> bool {
        self.cert.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cert(&mut self, v: super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned) {
        self.cert = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cert(&mut self) -> &mut super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned {
        if self.cert.is_none() {
            self.cert.set_default();
        }
        self.cert.as_mut().unwrap()
    }

    // Take field
    pub fn take_cert(&mut self) -> super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned {
        self.cert.take().unwrap_or_else(|| super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned::new())
    }

    pub fn get_cert(&self) -> &super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned {
        self.cert.as_ref().unwrap_or_else(|| super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned::default_instance())
    }

    fn get_cert_for_reflect(&self) -> &::protobuf::SingularPtrField<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned> {
        &self.cert
    }

    fn mut_cert_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned> {
        &mut self.cert
    }

    // optional uint32 protocol_version = 8;

    pub fn clear_protocol_version(&mut self) {
        self.protocol_version = ::std::option::Option::None;
    }

    pub fn has_protocol_version(&self) -> bool {
        self.protocol_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol_version(&mut self, v: u32) {
        self.protocol_version = ::std::option::Option::Some(v);
    }

    pub fn get_protocol_version(&self) -> u32 {
        self.protocol_version.unwrap_or(0)
    }

    fn get_protocol_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.protocol_version
    }

    fn mut_protocol_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.protocol_version
    }
}

impl ::protobuf::Message for CMsgSteamDatagramConnectRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.crypt {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cert {
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
                    self.connection_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.relay_session_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.client_steam_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.my_timestamp = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_est_ms = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.crypt)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cert)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.protocol_version = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.connection_id {
            my_size += 5;
        }
        if let Some(v) = self.relay_session_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.client_steam_id {
            my_size += 9;
        }
        if let Some(v) = self.my_timestamp {
            my_size += 9;
        }
        if let Some(v) = self.ping_est_ms {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.crypt.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.cert.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.protocol_version {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.connection_id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.relay_session_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.client_steam_id {
            os.write_fixed64(3, v)?;
        }
        if let Some(v) = self.my_timestamp {
            os.write_fixed64(4, v)?;
        }
        if let Some(v) = self.ping_est_ms {
            os.write_uint32(5, v)?;
        }
        if let Some(ref v) = self.crypt.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.cert.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.protocol_version {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramConnectRequest {
    fn new() -> CMsgSteamDatagramConnectRequest {
        CMsgSteamDatagramConnectRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "connection_id",
                    CMsgSteamDatagramConnectRequest::get_connection_id_for_reflect,
                    CMsgSteamDatagramConnectRequest::mut_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "relay_session_id",
                    CMsgSteamDatagramConnectRequest::get_relay_session_id_for_reflect,
                    CMsgSteamDatagramConnectRequest::mut_relay_session_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "client_steam_id",
                    CMsgSteamDatagramConnectRequest::get_client_steam_id_for_reflect,
                    CMsgSteamDatagramConnectRequest::mut_client_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "my_timestamp",
                    CMsgSteamDatagramConnectRequest::get_my_timestamp_for_reflect,
                    CMsgSteamDatagramConnectRequest::mut_my_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_est_ms",
                    CMsgSteamDatagramConnectRequest::get_ping_est_ms_for_reflect,
                    CMsgSteamDatagramConnectRequest::mut_ping_est_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramSessionCryptInfoSigned>>(
                    "crypt",
                    CMsgSteamDatagramConnectRequest::get_crypt_for_reflect,
                    CMsgSteamDatagramConnectRequest::mut_crypt_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned>>(
                    "cert",
                    CMsgSteamDatagramConnectRequest::get_cert_for_reflect,
                    CMsgSteamDatagramConnectRequest::mut_cert_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "protocol_version",
                    CMsgSteamDatagramConnectRequest::get_protocol_version_for_reflect,
                    CMsgSteamDatagramConnectRequest::mut_protocol_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramConnectRequest>(
                    "CMsgSteamDatagramConnectRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramConnectRequest {
    fn clear(&mut self) {
        self.clear_connection_id();
        self.clear_relay_session_id();
        self.clear_client_steam_id();
        self.clear_my_timestamp();
        self.clear_ping_est_ms();
        self.clear_crypt();
        self.clear_cert();
        self.clear_protocol_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramConnectRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramConnectOK {
    // message fields
    client_connection_id: ::std::option::Option<u32>,
    server_connection_id: ::std::option::Option<u32>,
    relay_session_id: ::std::option::Option<u32>,
    your_timestamp: ::std::option::Option<u64>,
    delay_time_usec: ::std::option::Option<u32>,
    crypt: ::protobuf::SingularPtrField<CMsgSteamDatagramSessionCryptInfoSigned>,
    cert: ::protobuf::SingularPtrField<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned>,
    protocol_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramConnectOK {}

impl CMsgSteamDatagramConnectOK {
    pub fn new() -> CMsgSteamDatagramConnectOK {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramConnectOK {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramConnectOK> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramConnectOK,
        };
        unsafe {
            instance.get(CMsgSteamDatagramConnectOK::new)
        }
    }

    // optional fixed32 client_connection_id = 1;

    pub fn clear_client_connection_id(&mut self) {
        self.client_connection_id = ::std::option::Option::None;
    }

    pub fn has_client_connection_id(&self) -> bool {
        self.client_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_connection_id(&mut self, v: u32) {
        self.client_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_connection_id(&self) -> u32 {
        self.client_connection_id.unwrap_or(0)
    }

    fn get_client_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_connection_id
    }

    fn mut_client_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_connection_id
    }

    // optional fixed32 server_connection_id = 7;

    pub fn clear_server_connection_id(&mut self) {
        self.server_connection_id = ::std::option::Option::None;
    }

    pub fn has_server_connection_id(&self) -> bool {
        self.server_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_connection_id(&mut self, v: u32) {
        self.server_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_server_connection_id(&self) -> u32 {
        self.server_connection_id.unwrap_or(0)
    }

    fn get_server_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_connection_id
    }

    fn mut_server_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_connection_id
    }

    // optional uint32 relay_session_id = 2;

    pub fn clear_relay_session_id(&mut self) {
        self.relay_session_id = ::std::option::Option::None;
    }

    pub fn has_relay_session_id(&self) -> bool {
        self.relay_session_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_relay_session_id(&mut self, v: u32) {
        self.relay_session_id = ::std::option::Option::Some(v);
    }

    pub fn get_relay_session_id(&self) -> u32 {
        self.relay_session_id.unwrap_or(0)
    }

    fn get_relay_session_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.relay_session_id
    }

    fn mut_relay_session_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.relay_session_id
    }

    // optional fixed64 your_timestamp = 3;

    pub fn clear_your_timestamp(&mut self) {
        self.your_timestamp = ::std::option::Option::None;
    }

    pub fn has_your_timestamp(&self) -> bool {
        self.your_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_your_timestamp(&mut self, v: u64) {
        self.your_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_your_timestamp(&self) -> u64 {
        self.your_timestamp.unwrap_or(0)
    }

    fn get_your_timestamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.your_timestamp
    }

    fn mut_your_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.your_timestamp
    }

    // optional uint32 delay_time_usec = 4;

    pub fn clear_delay_time_usec(&mut self) {
        self.delay_time_usec = ::std::option::Option::None;
    }

    pub fn has_delay_time_usec(&self) -> bool {
        self.delay_time_usec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delay_time_usec(&mut self, v: u32) {
        self.delay_time_usec = ::std::option::Option::Some(v);
    }

    pub fn get_delay_time_usec(&self) -> u32 {
        self.delay_time_usec.unwrap_or(0)
    }

    fn get_delay_time_usec_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.delay_time_usec
    }

    fn mut_delay_time_usec_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.delay_time_usec
    }

    // optional .CMsgSteamDatagramSessionCryptInfoSigned crypt = 5;

    pub fn clear_crypt(&mut self) {
        self.crypt.clear();
    }

    pub fn has_crypt(&self) -> bool {
        self.crypt.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crypt(&mut self, v: CMsgSteamDatagramSessionCryptInfoSigned) {
        self.crypt = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_crypt(&mut self) -> &mut CMsgSteamDatagramSessionCryptInfoSigned {
        if self.crypt.is_none() {
            self.crypt.set_default();
        }
        self.crypt.as_mut().unwrap()
    }

    // Take field
    pub fn take_crypt(&mut self) -> CMsgSteamDatagramSessionCryptInfoSigned {
        self.crypt.take().unwrap_or_else(|| CMsgSteamDatagramSessionCryptInfoSigned::new())
    }

    pub fn get_crypt(&self) -> &CMsgSteamDatagramSessionCryptInfoSigned {
        self.crypt.as_ref().unwrap_or_else(|| CMsgSteamDatagramSessionCryptInfoSigned::default_instance())
    }

    fn get_crypt_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramSessionCryptInfoSigned> {
        &self.crypt
    }

    fn mut_crypt_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramSessionCryptInfoSigned> {
        &mut self.crypt
    }

    // optional .CMsgSteamDatagramCertificateSigned cert = 6;

    pub fn clear_cert(&mut self) {
        self.cert.clear();
    }

    pub fn has_cert(&self) -> bool {
        self.cert.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cert(&mut self, v: super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned) {
        self.cert = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cert(&mut self) -> &mut super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned {
        if self.cert.is_none() {
            self.cert.set_default();
        }
        self.cert.as_mut().unwrap()
    }

    // Take field
    pub fn take_cert(&mut self) -> super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned {
        self.cert.take().unwrap_or_else(|| super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned::new())
    }

    pub fn get_cert(&self) -> &super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned {
        self.cert.as_ref().unwrap_or_else(|| super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned::default_instance())
    }

    fn get_cert_for_reflect(&self) -> &::protobuf::SingularPtrField<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned> {
        &self.cert
    }

    fn mut_cert_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned> {
        &mut self.cert
    }

    // optional uint32 protocol_version = 8;

    pub fn clear_protocol_version(&mut self) {
        self.protocol_version = ::std::option::Option::None;
    }

    pub fn has_protocol_version(&self) -> bool {
        self.protocol_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol_version(&mut self, v: u32) {
        self.protocol_version = ::std::option::Option::Some(v);
    }

    pub fn get_protocol_version(&self) -> u32 {
        self.protocol_version.unwrap_or(0)
    }

    fn get_protocol_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.protocol_version
    }

    fn mut_protocol_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.protocol_version
    }
}

impl ::protobuf::Message for CMsgSteamDatagramConnectOK {
    fn is_initialized(&self) -> bool {
        for v in &self.crypt {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cert {
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
                    self.client_connection_id = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.server_connection_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.relay_session_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.your_timestamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.delay_time_usec = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.crypt)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cert)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.protocol_version = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.client_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.server_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.relay_session_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.your_timestamp {
            my_size += 9;
        }
        if let Some(v) = self.delay_time_usec {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.crypt.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.cert.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.protocol_version {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_connection_id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.server_connection_id {
            os.write_fixed32(7, v)?;
        }
        if let Some(v) = self.relay_session_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.your_timestamp {
            os.write_fixed64(3, v)?;
        }
        if let Some(v) = self.delay_time_usec {
            os.write_uint32(4, v)?;
        }
        if let Some(ref v) = self.crypt.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.cert.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.protocol_version {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramConnectOK {
    fn new() -> CMsgSteamDatagramConnectOK {
        CMsgSteamDatagramConnectOK::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectOK>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_connection_id",
                    CMsgSteamDatagramConnectOK::get_client_connection_id_for_reflect,
                    CMsgSteamDatagramConnectOK::mut_client_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "server_connection_id",
                    CMsgSteamDatagramConnectOK::get_server_connection_id_for_reflect,
                    CMsgSteamDatagramConnectOK::mut_server_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "relay_session_id",
                    CMsgSteamDatagramConnectOK::get_relay_session_id_for_reflect,
                    CMsgSteamDatagramConnectOK::mut_relay_session_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "your_timestamp",
                    CMsgSteamDatagramConnectOK::get_your_timestamp_for_reflect,
                    CMsgSteamDatagramConnectOK::mut_your_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "delay_time_usec",
                    CMsgSteamDatagramConnectOK::get_delay_time_usec_for_reflect,
                    CMsgSteamDatagramConnectOK::mut_delay_time_usec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramSessionCryptInfoSigned>>(
                    "crypt",
                    CMsgSteamDatagramConnectOK::get_crypt_for_reflect,
                    CMsgSteamDatagramConnectOK::mut_crypt_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned>>(
                    "cert",
                    CMsgSteamDatagramConnectOK::get_cert_for_reflect,
                    CMsgSteamDatagramConnectOK::mut_cert_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "protocol_version",
                    CMsgSteamDatagramConnectOK::get_protocol_version_for_reflect,
                    CMsgSteamDatagramConnectOK::mut_protocol_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramConnectOK>(
                    "CMsgSteamDatagramConnectOK",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramConnectOK {
    fn clear(&mut self) {
        self.clear_client_connection_id();
        self.clear_server_connection_id();
        self.clear_relay_session_id();
        self.clear_your_timestamp();
        self.clear_delay_time_usec();
        self.clear_crypt();
        self.clear_cert();
        self.clear_protocol_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramConnectOK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectOK {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramConnectionClosed {
    // message fields
    legacy_client_connection_id: ::std::option::Option<u32>,
    to_connection_id: ::std::option::Option<u32>,
    from_connection_id: ::std::option::Option<u32>,
    relay_session_id: ::std::option::Option<u32>,
    peer_steam_id: ::std::option::Option<u64>,
    relay_mode: ::std::option::Option<CMsgSteamDatagramConnectionClosed_ERelayMode>,
    debug: ::protobuf::SingularField<::std::string::String>,
    reason_code: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramConnectionClosed {}

impl CMsgSteamDatagramConnectionClosed {
    pub fn new() -> CMsgSteamDatagramConnectionClosed {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramConnectionClosed {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramConnectionClosed> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramConnectionClosed,
        };
        unsafe {
            instance.get(CMsgSteamDatagramConnectionClosed::new)
        }
    }

    // optional fixed32 legacy_client_connection_id = 1;

    pub fn clear_legacy_client_connection_id(&mut self) {
        self.legacy_client_connection_id = ::std::option::Option::None;
    }

    pub fn has_legacy_client_connection_id(&self) -> bool {
        self.legacy_client_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_client_connection_id(&mut self, v: u32) {
        self.legacy_client_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_legacy_client_connection_id(&self) -> u32 {
        self.legacy_client_connection_id.unwrap_or(0)
    }

    fn get_legacy_client_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.legacy_client_connection_id
    }

    fn mut_legacy_client_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.legacy_client_connection_id
    }

    // optional fixed32 to_connection_id = 7;

    pub fn clear_to_connection_id(&mut self) {
        self.to_connection_id = ::std::option::Option::None;
    }

    pub fn has_to_connection_id(&self) -> bool {
        self.to_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to_connection_id(&mut self, v: u32) {
        self.to_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_to_connection_id(&self) -> u32 {
        self.to_connection_id.unwrap_or(0)
    }

    fn get_to_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.to_connection_id
    }

    fn mut_to_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.to_connection_id
    }

    // optional fixed32 from_connection_id = 8;

    pub fn clear_from_connection_id(&mut self) {
        self.from_connection_id = ::std::option::Option::None;
    }

    pub fn has_from_connection_id(&self) -> bool {
        self.from_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_connection_id(&mut self, v: u32) {
        self.from_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_from_connection_id(&self) -> u32 {
        self.from_connection_id.unwrap_or(0)
    }

    fn get_from_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.from_connection_id
    }

    fn mut_from_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.from_connection_id
    }

    // optional uint32 relay_session_id = 2;

    pub fn clear_relay_session_id(&mut self) {
        self.relay_session_id = ::std::option::Option::None;
    }

    pub fn has_relay_session_id(&self) -> bool {
        self.relay_session_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_relay_session_id(&mut self, v: u32) {
        self.relay_session_id = ::std::option::Option::Some(v);
    }

    pub fn get_relay_session_id(&self) -> u32 {
        self.relay_session_id.unwrap_or(0)
    }

    fn get_relay_session_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.relay_session_id
    }

    fn mut_relay_session_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.relay_session_id
    }

    // optional fixed64 peer_steam_id = 3;

    pub fn clear_peer_steam_id(&mut self) {
        self.peer_steam_id = ::std::option::Option::None;
    }

    pub fn has_peer_steam_id(&self) -> bool {
        self.peer_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer_steam_id(&mut self, v: u64) {
        self.peer_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_peer_steam_id(&self) -> u64 {
        self.peer_steam_id.unwrap_or(0)
    }

    fn get_peer_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.peer_steam_id
    }

    fn mut_peer_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.peer_steam_id
    }

    // optional .CMsgSteamDatagramConnectionClosed.ERelayMode relay_mode = 4;

    pub fn clear_relay_mode(&mut self) {
        self.relay_mode = ::std::option::Option::None;
    }

    pub fn has_relay_mode(&self) -> bool {
        self.relay_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_relay_mode(&mut self, v: CMsgSteamDatagramConnectionClosed_ERelayMode) {
        self.relay_mode = ::std::option::Option::Some(v);
    }

    pub fn get_relay_mode(&self) -> CMsgSteamDatagramConnectionClosed_ERelayMode {
        self.relay_mode.unwrap_or(CMsgSteamDatagramConnectionClosed_ERelayMode::None)
    }

    fn get_relay_mode_for_reflect(&self) -> &::std::option::Option<CMsgSteamDatagramConnectionClosed_ERelayMode> {
        &self.relay_mode
    }

    fn mut_relay_mode_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgSteamDatagramConnectionClosed_ERelayMode> {
        &mut self.relay_mode
    }

    // optional string debug = 5;

    pub fn clear_debug(&mut self) {
        self.debug.clear();
    }

    pub fn has_debug(&self) -> bool {
        self.debug.is_some()
    }

    // Param is passed by value, moved
    pub fn set_debug(&mut self, v: ::std::string::String) {
        self.debug = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_debug(&mut self) -> &mut ::std::string::String {
        if self.debug.is_none() {
            self.debug.set_default();
        }
        self.debug.as_mut().unwrap()
    }

    // Take field
    pub fn take_debug(&mut self) -> ::std::string::String {
        self.debug.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_debug(&self) -> &str {
        match self.debug.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_debug_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.debug
    }

    fn mut_debug_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.debug
    }

    // optional uint32 reason_code = 6;

    pub fn clear_reason_code(&mut self) {
        self.reason_code = ::std::option::Option::None;
    }

    pub fn has_reason_code(&self) -> bool {
        self.reason_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason_code(&mut self, v: u32) {
        self.reason_code = ::std::option::Option::Some(v);
    }

    pub fn get_reason_code(&self) -> u32 {
        self.reason_code.unwrap_or(0)
    }

    fn get_reason_code_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.reason_code
    }

    fn mut_reason_code_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.reason_code
    }
}

impl ::protobuf::Message for CMsgSteamDatagramConnectionClosed {
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
                    self.legacy_client_connection_id = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.to_connection_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.from_connection_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.relay_session_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.peer_steam_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.relay_mode = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.debug)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.reason_code = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.legacy_client_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.to_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.from_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.relay_session_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.peer_steam_id {
            my_size += 9;
        }
        if let Some(v) = self.relay_mode {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        if let Some(ref v) = self.debug.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(v) = self.reason_code {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.legacy_client_connection_id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.to_connection_id {
            os.write_fixed32(7, v)?;
        }
        if let Some(v) = self.from_connection_id {
            os.write_fixed32(8, v)?;
        }
        if let Some(v) = self.relay_session_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.peer_steam_id {
            os.write_fixed64(3, v)?;
        }
        if let Some(v) = self.relay_mode {
            os.write_enum(4, v.value())?;
        }
        if let Some(ref v) = self.debug.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(v) = self.reason_code {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramConnectionClosed {
    fn new() -> CMsgSteamDatagramConnectionClosed {
        CMsgSteamDatagramConnectionClosed::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionClosed>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "legacy_client_connection_id",
                    CMsgSteamDatagramConnectionClosed::get_legacy_client_connection_id_for_reflect,
                    CMsgSteamDatagramConnectionClosed::mut_legacy_client_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "to_connection_id",
                    CMsgSteamDatagramConnectionClosed::get_to_connection_id_for_reflect,
                    CMsgSteamDatagramConnectionClosed::mut_to_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "from_connection_id",
                    CMsgSteamDatagramConnectionClosed::get_from_connection_id_for_reflect,
                    CMsgSteamDatagramConnectionClosed::mut_from_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "relay_session_id",
                    CMsgSteamDatagramConnectionClosed::get_relay_session_id_for_reflect,
                    CMsgSteamDatagramConnectionClosed::mut_relay_session_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "peer_steam_id",
                    CMsgSteamDatagramConnectionClosed::get_peer_steam_id_for_reflect,
                    CMsgSteamDatagramConnectionClosed::mut_peer_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgSteamDatagramConnectionClosed_ERelayMode>>(
                    "relay_mode",
                    CMsgSteamDatagramConnectionClosed::get_relay_mode_for_reflect,
                    CMsgSteamDatagramConnectionClosed::mut_relay_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "debug",
                    CMsgSteamDatagramConnectionClosed::get_debug_for_reflect,
                    CMsgSteamDatagramConnectionClosed::mut_debug_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "reason_code",
                    CMsgSteamDatagramConnectionClosed::get_reason_code_for_reflect,
                    CMsgSteamDatagramConnectionClosed::mut_reason_code_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramConnectionClosed>(
                    "CMsgSteamDatagramConnectionClosed",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramConnectionClosed {
    fn clear(&mut self) {
        self.clear_legacy_client_connection_id();
        self.clear_to_connection_id();
        self.clear_from_connection_id();
        self.clear_relay_session_id();
        self.clear_peer_steam_id();
        self.clear_relay_mode();
        self.clear_debug();
        self.clear_reason_code();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramConnectionClosed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionClosed {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgSteamDatagramConnectionClosed_ERelayMode {
    None = 0,
    EndToEnd = 1,
    ClosedByPeer = 2,
}

impl ::protobuf::ProtobufEnum for CMsgSteamDatagramConnectionClosed_ERelayMode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgSteamDatagramConnectionClosed_ERelayMode> {
        match value {
            0 => ::std::option::Option::Some(CMsgSteamDatagramConnectionClosed_ERelayMode::None),
            1 => ::std::option::Option::Some(CMsgSteamDatagramConnectionClosed_ERelayMode::EndToEnd),
            2 => ::std::option::Option::Some(CMsgSteamDatagramConnectionClosed_ERelayMode::ClosedByPeer),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgSteamDatagramConnectionClosed_ERelayMode] = &[
            CMsgSteamDatagramConnectionClosed_ERelayMode::None,
            CMsgSteamDatagramConnectionClosed_ERelayMode::EndToEnd,
            CMsgSteamDatagramConnectionClosed_ERelayMode::ClosedByPeer,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionClosed_ERelayMode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgSteamDatagramConnectionClosed_ERelayMode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgSteamDatagramConnectionClosed_ERelayMode {
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionClosed_ERelayMode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramNoConnection {
    // message fields
    legacy_client_connection_id: ::std::option::Option<u32>,
    to_connection_id: ::std::option::Option<u32>,
    from_connection_id: ::std::option::Option<u32>,
    relay_session_id: ::std::option::Option<u32>,
    peer_steam_id: ::std::option::Option<u64>,
    end_to_end: ::std::option::Option<bool>,
    dummy_pad: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramNoConnection {}

impl CMsgSteamDatagramNoConnection {
    pub fn new() -> CMsgSteamDatagramNoConnection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramNoConnection {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramNoConnection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramNoConnection,
        };
        unsafe {
            instance.get(CMsgSteamDatagramNoConnection::new)
        }
    }

    // optional fixed32 legacy_client_connection_id = 1;

    pub fn clear_legacy_client_connection_id(&mut self) {
        self.legacy_client_connection_id = ::std::option::Option::None;
    }

    pub fn has_legacy_client_connection_id(&self) -> bool {
        self.legacy_client_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_client_connection_id(&mut self, v: u32) {
        self.legacy_client_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_legacy_client_connection_id(&self) -> u32 {
        self.legacy_client_connection_id.unwrap_or(0)
    }

    fn get_legacy_client_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.legacy_client_connection_id
    }

    fn mut_legacy_client_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.legacy_client_connection_id
    }

    // optional fixed32 to_connection_id = 5;

    pub fn clear_to_connection_id(&mut self) {
        self.to_connection_id = ::std::option::Option::None;
    }

    pub fn has_to_connection_id(&self) -> bool {
        self.to_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to_connection_id(&mut self, v: u32) {
        self.to_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_to_connection_id(&self) -> u32 {
        self.to_connection_id.unwrap_or(0)
    }

    fn get_to_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.to_connection_id
    }

    fn mut_to_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.to_connection_id
    }

    // optional fixed32 from_connection_id = 6;

    pub fn clear_from_connection_id(&mut self) {
        self.from_connection_id = ::std::option::Option::None;
    }

    pub fn has_from_connection_id(&self) -> bool {
        self.from_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_connection_id(&mut self, v: u32) {
        self.from_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_from_connection_id(&self) -> u32 {
        self.from_connection_id.unwrap_or(0)
    }

    fn get_from_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.from_connection_id
    }

    fn mut_from_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.from_connection_id
    }

    // optional uint32 relay_session_id = 2;

    pub fn clear_relay_session_id(&mut self) {
        self.relay_session_id = ::std::option::Option::None;
    }

    pub fn has_relay_session_id(&self) -> bool {
        self.relay_session_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_relay_session_id(&mut self, v: u32) {
        self.relay_session_id = ::std::option::Option::Some(v);
    }

    pub fn get_relay_session_id(&self) -> u32 {
        self.relay_session_id.unwrap_or(0)
    }

    fn get_relay_session_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.relay_session_id
    }

    fn mut_relay_session_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.relay_session_id
    }

    // optional fixed64 peer_steam_id = 3;

    pub fn clear_peer_steam_id(&mut self) {
        self.peer_steam_id = ::std::option::Option::None;
    }

    pub fn has_peer_steam_id(&self) -> bool {
        self.peer_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer_steam_id(&mut self, v: u64) {
        self.peer_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_peer_steam_id(&self) -> u64 {
        self.peer_steam_id.unwrap_or(0)
    }

    fn get_peer_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.peer_steam_id
    }

    fn mut_peer_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.peer_steam_id
    }

    // optional bool end_to_end = 4;

    pub fn clear_end_to_end(&mut self) {
        self.end_to_end = ::std::option::Option::None;
    }

    pub fn has_end_to_end(&self) -> bool {
        self.end_to_end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_to_end(&mut self, v: bool) {
        self.end_to_end = ::std::option::Option::Some(v);
    }

    pub fn get_end_to_end(&self) -> bool {
        self.end_to_end.unwrap_or(false)
    }

    fn get_end_to_end_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.end_to_end
    }

    fn mut_end_to_end_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.end_to_end
    }

    // optional fixed32 dummy_pad = 1023;

    pub fn clear_dummy_pad(&mut self) {
        self.dummy_pad = ::std::option::Option::None;
    }

    pub fn has_dummy_pad(&self) -> bool {
        self.dummy_pad.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dummy_pad(&mut self, v: u32) {
        self.dummy_pad = ::std::option::Option::Some(v);
    }

    pub fn get_dummy_pad(&self) -> u32 {
        self.dummy_pad.unwrap_or(0)
    }

    fn get_dummy_pad_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dummy_pad
    }

    fn mut_dummy_pad_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dummy_pad
    }
}

impl ::protobuf::Message for CMsgSteamDatagramNoConnection {
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
                    self.legacy_client_connection_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.to_connection_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.from_connection_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.relay_session_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.peer_steam_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.end_to_end = ::std::option::Option::Some(tmp);
                },
                1023 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.dummy_pad = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.legacy_client_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.to_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.from_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.relay_session_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.peer_steam_id {
            my_size += 9;
        }
        if let Some(v) = self.end_to_end {
            my_size += 2;
        }
        if let Some(v) = self.dummy_pad {
            my_size += 6;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.legacy_client_connection_id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.to_connection_id {
            os.write_fixed32(5, v)?;
        }
        if let Some(v) = self.from_connection_id {
            os.write_fixed32(6, v)?;
        }
        if let Some(v) = self.relay_session_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.peer_steam_id {
            os.write_fixed64(3, v)?;
        }
        if let Some(v) = self.end_to_end {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.dummy_pad {
            os.write_fixed32(1023, v)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramNoConnection {
    fn new() -> CMsgSteamDatagramNoConnection {
        CMsgSteamDatagramNoConnection::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramNoConnection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "legacy_client_connection_id",
                    CMsgSteamDatagramNoConnection::get_legacy_client_connection_id_for_reflect,
                    CMsgSteamDatagramNoConnection::mut_legacy_client_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "to_connection_id",
                    CMsgSteamDatagramNoConnection::get_to_connection_id_for_reflect,
                    CMsgSteamDatagramNoConnection::mut_to_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "from_connection_id",
                    CMsgSteamDatagramNoConnection::get_from_connection_id_for_reflect,
                    CMsgSteamDatagramNoConnection::mut_from_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "relay_session_id",
                    CMsgSteamDatagramNoConnection::get_relay_session_id_for_reflect,
                    CMsgSteamDatagramNoConnection::mut_relay_session_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "peer_steam_id",
                    CMsgSteamDatagramNoConnection::get_peer_steam_id_for_reflect,
                    CMsgSteamDatagramNoConnection::mut_peer_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "end_to_end",
                    CMsgSteamDatagramNoConnection::get_end_to_end_for_reflect,
                    CMsgSteamDatagramNoConnection::mut_end_to_end_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "dummy_pad",
                    CMsgSteamDatagramNoConnection::get_dummy_pad_for_reflect,
                    CMsgSteamDatagramNoConnection::mut_dummy_pad_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramNoConnection>(
                    "CMsgSteamDatagramNoConnection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramNoConnection {
    fn clear(&mut self) {
        self.clear_legacy_client_connection_id();
        self.clear_to_connection_id();
        self.clear_from_connection_id();
        self.clear_relay_session_id();
        self.clear_peer_steam_id();
        self.clear_end_to_end();
        self.clear_dummy_pad();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramNoConnection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramNoConnection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamSockets_UDP_ChallengeRequest {
    // message fields
    connection_id: ::std::option::Option<u32>,
    my_timestamp: ::std::option::Option<u64>,
    protocol_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamSockets_UDP_ChallengeRequest {}

impl CMsgSteamSockets_UDP_ChallengeRequest {
    pub fn new() -> CMsgSteamSockets_UDP_ChallengeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamSockets_UDP_ChallengeRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamSockets_UDP_ChallengeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamSockets_UDP_ChallengeRequest,
        };
        unsafe {
            instance.get(CMsgSteamSockets_UDP_ChallengeRequest::new)
        }
    }

    // optional fixed32 connection_id = 1;

    pub fn clear_connection_id(&mut self) {
        self.connection_id = ::std::option::Option::None;
    }

    pub fn has_connection_id(&self) -> bool {
        self.connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connection_id(&mut self, v: u32) {
        self.connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_connection_id(&self) -> u32 {
        self.connection_id.unwrap_or(0)
    }

    fn get_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.connection_id
    }

    fn mut_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.connection_id
    }

    // optional fixed64 my_timestamp = 3;

    pub fn clear_my_timestamp(&mut self) {
        self.my_timestamp = ::std::option::Option::None;
    }

    pub fn has_my_timestamp(&self) -> bool {
        self.my_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_my_timestamp(&mut self, v: u64) {
        self.my_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_my_timestamp(&self) -> u64 {
        self.my_timestamp.unwrap_or(0)
    }

    fn get_my_timestamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.my_timestamp
    }

    fn mut_my_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.my_timestamp
    }

    // optional uint32 protocol_version = 4;

    pub fn clear_protocol_version(&mut self) {
        self.protocol_version = ::std::option::Option::None;
    }

    pub fn has_protocol_version(&self) -> bool {
        self.protocol_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol_version(&mut self, v: u32) {
        self.protocol_version = ::std::option::Option::Some(v);
    }

    pub fn get_protocol_version(&self) -> u32 {
        self.protocol_version.unwrap_or(0)
    }

    fn get_protocol_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.protocol_version
    }

    fn mut_protocol_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.protocol_version
    }
}

impl ::protobuf::Message for CMsgSteamSockets_UDP_ChallengeRequest {
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
                    self.connection_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.my_timestamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.protocol_version = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.connection_id {
            my_size += 5;
        }
        if let Some(v) = self.my_timestamp {
            my_size += 9;
        }
        if let Some(v) = self.protocol_version {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.connection_id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.my_timestamp {
            os.write_fixed64(3, v)?;
        }
        if let Some(v) = self.protocol_version {
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

impl ::protobuf::MessageStatic for CMsgSteamSockets_UDP_ChallengeRequest {
    fn new() -> CMsgSteamSockets_UDP_ChallengeRequest {
        CMsgSteamSockets_UDP_ChallengeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamSockets_UDP_ChallengeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "connection_id",
                    CMsgSteamSockets_UDP_ChallengeRequest::get_connection_id_for_reflect,
                    CMsgSteamSockets_UDP_ChallengeRequest::mut_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "my_timestamp",
                    CMsgSteamSockets_UDP_ChallengeRequest::get_my_timestamp_for_reflect,
                    CMsgSteamSockets_UDP_ChallengeRequest::mut_my_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "protocol_version",
                    CMsgSteamSockets_UDP_ChallengeRequest::get_protocol_version_for_reflect,
                    CMsgSteamSockets_UDP_ChallengeRequest::mut_protocol_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamSockets_UDP_ChallengeRequest>(
                    "CMsgSteamSockets_UDP_ChallengeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamSockets_UDP_ChallengeRequest {
    fn clear(&mut self) {
        self.clear_connection_id();
        self.clear_my_timestamp();
        self.clear_protocol_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamSockets_UDP_ChallengeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamSockets_UDP_ChallengeRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamSockets_UDP_ChallengeReply {
    // message fields
    connection_id: ::std::option::Option<u32>,
    challenge: ::std::option::Option<u64>,
    your_timestamp: ::std::option::Option<u64>,
    protocol_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamSockets_UDP_ChallengeReply {}

impl CMsgSteamSockets_UDP_ChallengeReply {
    pub fn new() -> CMsgSteamSockets_UDP_ChallengeReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamSockets_UDP_ChallengeReply {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamSockets_UDP_ChallengeReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamSockets_UDP_ChallengeReply,
        };
        unsafe {
            instance.get(CMsgSteamSockets_UDP_ChallengeReply::new)
        }
    }

    // optional fixed32 connection_id = 1;

    pub fn clear_connection_id(&mut self) {
        self.connection_id = ::std::option::Option::None;
    }

    pub fn has_connection_id(&self) -> bool {
        self.connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connection_id(&mut self, v: u32) {
        self.connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_connection_id(&self) -> u32 {
        self.connection_id.unwrap_or(0)
    }

    fn get_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.connection_id
    }

    fn mut_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.connection_id
    }

    // optional fixed64 challenge = 2;

    pub fn clear_challenge(&mut self) {
        self.challenge = ::std::option::Option::None;
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: u64) {
        self.challenge = ::std::option::Option::Some(v);
    }

    pub fn get_challenge(&self) -> u64 {
        self.challenge.unwrap_or(0)
    }

    fn get_challenge_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.challenge
    }

    fn mut_challenge_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.challenge
    }

    // optional fixed64 your_timestamp = 3;

    pub fn clear_your_timestamp(&mut self) {
        self.your_timestamp = ::std::option::Option::None;
    }

    pub fn has_your_timestamp(&self) -> bool {
        self.your_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_your_timestamp(&mut self, v: u64) {
        self.your_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_your_timestamp(&self) -> u64 {
        self.your_timestamp.unwrap_or(0)
    }

    fn get_your_timestamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.your_timestamp
    }

    fn mut_your_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.your_timestamp
    }

    // optional uint32 protocol_version = 4;

    pub fn clear_protocol_version(&mut self) {
        self.protocol_version = ::std::option::Option::None;
    }

    pub fn has_protocol_version(&self) -> bool {
        self.protocol_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol_version(&mut self, v: u32) {
        self.protocol_version = ::std::option::Option::Some(v);
    }

    pub fn get_protocol_version(&self) -> u32 {
        self.protocol_version.unwrap_or(0)
    }

    fn get_protocol_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.protocol_version
    }

    fn mut_protocol_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.protocol_version
    }
}

impl ::protobuf::Message for CMsgSteamSockets_UDP_ChallengeReply {
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
                    self.connection_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.challenge = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.your_timestamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.protocol_version = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.connection_id {
            my_size += 5;
        }
        if let Some(v) = self.challenge {
            my_size += 9;
        }
        if let Some(v) = self.your_timestamp {
            my_size += 9;
        }
        if let Some(v) = self.protocol_version {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.connection_id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.challenge {
            os.write_fixed64(2, v)?;
        }
        if let Some(v) = self.your_timestamp {
            os.write_fixed64(3, v)?;
        }
        if let Some(v) = self.protocol_version {
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

impl ::protobuf::MessageStatic for CMsgSteamSockets_UDP_ChallengeReply {
    fn new() -> CMsgSteamSockets_UDP_ChallengeReply {
        CMsgSteamSockets_UDP_ChallengeReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamSockets_UDP_ChallengeReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "connection_id",
                    CMsgSteamSockets_UDP_ChallengeReply::get_connection_id_for_reflect,
                    CMsgSteamSockets_UDP_ChallengeReply::mut_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "challenge",
                    CMsgSteamSockets_UDP_ChallengeReply::get_challenge_for_reflect,
                    CMsgSteamSockets_UDP_ChallengeReply::mut_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "your_timestamp",
                    CMsgSteamSockets_UDP_ChallengeReply::get_your_timestamp_for_reflect,
                    CMsgSteamSockets_UDP_ChallengeReply::mut_your_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "protocol_version",
                    CMsgSteamSockets_UDP_ChallengeReply::get_protocol_version_for_reflect,
                    CMsgSteamSockets_UDP_ChallengeReply::mut_protocol_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamSockets_UDP_ChallengeReply>(
                    "CMsgSteamSockets_UDP_ChallengeReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamSockets_UDP_ChallengeReply {
    fn clear(&mut self) {
        self.clear_connection_id();
        self.clear_challenge();
        self.clear_your_timestamp();
        self.clear_protocol_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamSockets_UDP_ChallengeReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamSockets_UDP_ChallengeReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamSockets_UDP_ConnectRequest {
    // message fields
    client_connection_id: ::std::option::Option<u32>,
    challenge: ::std::option::Option<u64>,
    client_steam_id: ::std::option::Option<u64>,
    my_timestamp: ::std::option::Option<u64>,
    ping_est_ms: ::std::option::Option<u32>,
    crypt: ::protobuf::SingularPtrField<CMsgSteamDatagramSessionCryptInfoSigned>,
    cert: ::protobuf::SingularPtrField<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned>,
    protocol_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamSockets_UDP_ConnectRequest {}

impl CMsgSteamSockets_UDP_ConnectRequest {
    pub fn new() -> CMsgSteamSockets_UDP_ConnectRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamSockets_UDP_ConnectRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamSockets_UDP_ConnectRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamSockets_UDP_ConnectRequest,
        };
        unsafe {
            instance.get(CMsgSteamSockets_UDP_ConnectRequest::new)
        }
    }

    // optional fixed32 client_connection_id = 1;

    pub fn clear_client_connection_id(&mut self) {
        self.client_connection_id = ::std::option::Option::None;
    }

    pub fn has_client_connection_id(&self) -> bool {
        self.client_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_connection_id(&mut self, v: u32) {
        self.client_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_connection_id(&self) -> u32 {
        self.client_connection_id.unwrap_or(0)
    }

    fn get_client_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_connection_id
    }

    fn mut_client_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_connection_id
    }

    // optional fixed64 challenge = 2;

    pub fn clear_challenge(&mut self) {
        self.challenge = ::std::option::Option::None;
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: u64) {
        self.challenge = ::std::option::Option::Some(v);
    }

    pub fn get_challenge(&self) -> u64 {
        self.challenge.unwrap_or(0)
    }

    fn get_challenge_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.challenge
    }

    fn mut_challenge_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.challenge
    }

    // optional fixed64 client_steam_id = 3;

    pub fn clear_client_steam_id(&mut self) {
        self.client_steam_id = ::std::option::Option::None;
    }

    pub fn has_client_steam_id(&self) -> bool {
        self.client_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_steam_id(&mut self, v: u64) {
        self.client_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_steam_id(&self) -> u64 {
        self.client_steam_id.unwrap_or(0)
    }

    fn get_client_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.client_steam_id
    }

    fn mut_client_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.client_steam_id
    }

    // optional fixed64 my_timestamp = 5;

    pub fn clear_my_timestamp(&mut self) {
        self.my_timestamp = ::std::option::Option::None;
    }

    pub fn has_my_timestamp(&self) -> bool {
        self.my_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_my_timestamp(&mut self, v: u64) {
        self.my_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_my_timestamp(&self) -> u64 {
        self.my_timestamp.unwrap_or(0)
    }

    fn get_my_timestamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.my_timestamp
    }

    fn mut_my_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.my_timestamp
    }

    // optional uint32 ping_est_ms = 6;

    pub fn clear_ping_est_ms(&mut self) {
        self.ping_est_ms = ::std::option::Option::None;
    }

    pub fn has_ping_est_ms(&self) -> bool {
        self.ping_est_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_est_ms(&mut self, v: u32) {
        self.ping_est_ms = ::std::option::Option::Some(v);
    }

    pub fn get_ping_est_ms(&self) -> u32 {
        self.ping_est_ms.unwrap_or(0)
    }

    fn get_ping_est_ms_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_est_ms
    }

    fn mut_ping_est_ms_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_est_ms
    }

    // optional .CMsgSteamDatagramSessionCryptInfoSigned crypt = 7;

    pub fn clear_crypt(&mut self) {
        self.crypt.clear();
    }

    pub fn has_crypt(&self) -> bool {
        self.crypt.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crypt(&mut self, v: CMsgSteamDatagramSessionCryptInfoSigned) {
        self.crypt = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_crypt(&mut self) -> &mut CMsgSteamDatagramSessionCryptInfoSigned {
        if self.crypt.is_none() {
            self.crypt.set_default();
        }
        self.crypt.as_mut().unwrap()
    }

    // Take field
    pub fn take_crypt(&mut self) -> CMsgSteamDatagramSessionCryptInfoSigned {
        self.crypt.take().unwrap_or_else(|| CMsgSteamDatagramSessionCryptInfoSigned::new())
    }

    pub fn get_crypt(&self) -> &CMsgSteamDatagramSessionCryptInfoSigned {
        self.crypt.as_ref().unwrap_or_else(|| CMsgSteamDatagramSessionCryptInfoSigned::default_instance())
    }

    fn get_crypt_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramSessionCryptInfoSigned> {
        &self.crypt
    }

    fn mut_crypt_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramSessionCryptInfoSigned> {
        &mut self.crypt
    }

    // optional .CMsgSteamDatagramCertificateSigned cert = 4;

    pub fn clear_cert(&mut self) {
        self.cert.clear();
    }

    pub fn has_cert(&self) -> bool {
        self.cert.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cert(&mut self, v: super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned) {
        self.cert = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cert(&mut self) -> &mut super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned {
        if self.cert.is_none() {
            self.cert.set_default();
        }
        self.cert.as_mut().unwrap()
    }

    // Take field
    pub fn take_cert(&mut self) -> super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned {
        self.cert.take().unwrap_or_else(|| super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned::new())
    }

    pub fn get_cert(&self) -> &super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned {
        self.cert.as_ref().unwrap_or_else(|| super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned::default_instance())
    }

    fn get_cert_for_reflect(&self) -> &::protobuf::SingularPtrField<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned> {
        &self.cert
    }

    fn mut_cert_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned> {
        &mut self.cert
    }

    // optional uint32 protocol_version = 8;

    pub fn clear_protocol_version(&mut self) {
        self.protocol_version = ::std::option::Option::None;
    }

    pub fn has_protocol_version(&self) -> bool {
        self.protocol_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol_version(&mut self, v: u32) {
        self.protocol_version = ::std::option::Option::Some(v);
    }

    pub fn get_protocol_version(&self) -> u32 {
        self.protocol_version.unwrap_or(0)
    }

    fn get_protocol_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.protocol_version
    }

    fn mut_protocol_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.protocol_version
    }
}

impl ::protobuf::Message for CMsgSteamSockets_UDP_ConnectRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.crypt {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cert {
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
                    self.client_connection_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.challenge = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.client_steam_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.my_timestamp = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ping_est_ms = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.crypt)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cert)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.protocol_version = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.client_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.challenge {
            my_size += 9;
        }
        if let Some(v) = self.client_steam_id {
            my_size += 9;
        }
        if let Some(v) = self.my_timestamp {
            my_size += 9;
        }
        if let Some(v) = self.ping_est_ms {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.crypt.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.cert.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.protocol_version {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_connection_id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.challenge {
            os.write_fixed64(2, v)?;
        }
        if let Some(v) = self.client_steam_id {
            os.write_fixed64(3, v)?;
        }
        if let Some(v) = self.my_timestamp {
            os.write_fixed64(5, v)?;
        }
        if let Some(v) = self.ping_est_ms {
            os.write_uint32(6, v)?;
        }
        if let Some(ref v) = self.crypt.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.cert.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.protocol_version {
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

impl ::protobuf::MessageStatic for CMsgSteamSockets_UDP_ConnectRequest {
    fn new() -> CMsgSteamSockets_UDP_ConnectRequest {
        CMsgSteamSockets_UDP_ConnectRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamSockets_UDP_ConnectRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_connection_id",
                    CMsgSteamSockets_UDP_ConnectRequest::get_client_connection_id_for_reflect,
                    CMsgSteamSockets_UDP_ConnectRequest::mut_client_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "challenge",
                    CMsgSteamSockets_UDP_ConnectRequest::get_challenge_for_reflect,
                    CMsgSteamSockets_UDP_ConnectRequest::mut_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "client_steam_id",
                    CMsgSteamSockets_UDP_ConnectRequest::get_client_steam_id_for_reflect,
                    CMsgSteamSockets_UDP_ConnectRequest::mut_client_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "my_timestamp",
                    CMsgSteamSockets_UDP_ConnectRequest::get_my_timestamp_for_reflect,
                    CMsgSteamSockets_UDP_ConnectRequest::mut_my_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_est_ms",
                    CMsgSteamSockets_UDP_ConnectRequest::get_ping_est_ms_for_reflect,
                    CMsgSteamSockets_UDP_ConnectRequest::mut_ping_est_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramSessionCryptInfoSigned>>(
                    "crypt",
                    CMsgSteamSockets_UDP_ConnectRequest::get_crypt_for_reflect,
                    CMsgSteamSockets_UDP_ConnectRequest::mut_crypt_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned>>(
                    "cert",
                    CMsgSteamSockets_UDP_ConnectRequest::get_cert_for_reflect,
                    CMsgSteamSockets_UDP_ConnectRequest::mut_cert_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "protocol_version",
                    CMsgSteamSockets_UDP_ConnectRequest::get_protocol_version_for_reflect,
                    CMsgSteamSockets_UDP_ConnectRequest::mut_protocol_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamSockets_UDP_ConnectRequest>(
                    "CMsgSteamSockets_UDP_ConnectRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamSockets_UDP_ConnectRequest {
    fn clear(&mut self) {
        self.clear_client_connection_id();
        self.clear_challenge();
        self.clear_client_steam_id();
        self.clear_my_timestamp();
        self.clear_ping_est_ms();
        self.clear_crypt();
        self.clear_cert();
        self.clear_protocol_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamSockets_UDP_ConnectRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamSockets_UDP_ConnectRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamSockets_UDP_ConnectOK {
    // message fields
    client_connection_id: ::std::option::Option<u32>,
    server_connection_id: ::std::option::Option<u32>,
    server_steam_id: ::std::option::Option<u64>,
    your_timestamp: ::std::option::Option<u64>,
    delay_time_usec: ::std::option::Option<u32>,
    crypt: ::protobuf::SingularPtrField<CMsgSteamDatagramSessionCryptInfoSigned>,
    cert: ::protobuf::SingularPtrField<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned>,
    protocol_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamSockets_UDP_ConnectOK {}

impl CMsgSteamSockets_UDP_ConnectOK {
    pub fn new() -> CMsgSteamSockets_UDP_ConnectOK {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamSockets_UDP_ConnectOK {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamSockets_UDP_ConnectOK> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamSockets_UDP_ConnectOK,
        };
        unsafe {
            instance.get(CMsgSteamSockets_UDP_ConnectOK::new)
        }
    }

    // optional fixed32 client_connection_id = 1;

    pub fn clear_client_connection_id(&mut self) {
        self.client_connection_id = ::std::option::Option::None;
    }

    pub fn has_client_connection_id(&self) -> bool {
        self.client_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_connection_id(&mut self, v: u32) {
        self.client_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_connection_id(&self) -> u32 {
        self.client_connection_id.unwrap_or(0)
    }

    fn get_client_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_connection_id
    }

    fn mut_client_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_connection_id
    }

    // optional fixed32 server_connection_id = 5;

    pub fn clear_server_connection_id(&mut self) {
        self.server_connection_id = ::std::option::Option::None;
    }

    pub fn has_server_connection_id(&self) -> bool {
        self.server_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_connection_id(&mut self, v: u32) {
        self.server_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_server_connection_id(&self) -> u32 {
        self.server_connection_id.unwrap_or(0)
    }

    fn get_server_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_connection_id
    }

    fn mut_server_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_connection_id
    }

    // optional fixed64 server_steam_id = 2;

    pub fn clear_server_steam_id(&mut self) {
        self.server_steam_id = ::std::option::Option::None;
    }

    pub fn has_server_steam_id(&self) -> bool {
        self.server_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_steam_id(&mut self, v: u64) {
        self.server_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_server_steam_id(&self) -> u64 {
        self.server_steam_id.unwrap_or(0)
    }

    fn get_server_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.server_steam_id
    }

    fn mut_server_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.server_steam_id
    }

    // optional fixed64 your_timestamp = 3;

    pub fn clear_your_timestamp(&mut self) {
        self.your_timestamp = ::std::option::Option::None;
    }

    pub fn has_your_timestamp(&self) -> bool {
        self.your_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_your_timestamp(&mut self, v: u64) {
        self.your_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_your_timestamp(&self) -> u64 {
        self.your_timestamp.unwrap_or(0)
    }

    fn get_your_timestamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.your_timestamp
    }

    fn mut_your_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.your_timestamp
    }

    // optional uint32 delay_time_usec = 4;

    pub fn clear_delay_time_usec(&mut self) {
        self.delay_time_usec = ::std::option::Option::None;
    }

    pub fn has_delay_time_usec(&self) -> bool {
        self.delay_time_usec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delay_time_usec(&mut self, v: u32) {
        self.delay_time_usec = ::std::option::Option::Some(v);
    }

    pub fn get_delay_time_usec(&self) -> u32 {
        self.delay_time_usec.unwrap_or(0)
    }

    fn get_delay_time_usec_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.delay_time_usec
    }

    fn mut_delay_time_usec_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.delay_time_usec
    }

    // optional .CMsgSteamDatagramSessionCryptInfoSigned crypt = 7;

    pub fn clear_crypt(&mut self) {
        self.crypt.clear();
    }

    pub fn has_crypt(&self) -> bool {
        self.crypt.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crypt(&mut self, v: CMsgSteamDatagramSessionCryptInfoSigned) {
        self.crypt = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_crypt(&mut self) -> &mut CMsgSteamDatagramSessionCryptInfoSigned {
        if self.crypt.is_none() {
            self.crypt.set_default();
        }
        self.crypt.as_mut().unwrap()
    }

    // Take field
    pub fn take_crypt(&mut self) -> CMsgSteamDatagramSessionCryptInfoSigned {
        self.crypt.take().unwrap_or_else(|| CMsgSteamDatagramSessionCryptInfoSigned::new())
    }

    pub fn get_crypt(&self) -> &CMsgSteamDatagramSessionCryptInfoSigned {
        self.crypt.as_ref().unwrap_or_else(|| CMsgSteamDatagramSessionCryptInfoSigned::default_instance())
    }

    fn get_crypt_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramSessionCryptInfoSigned> {
        &self.crypt
    }

    fn mut_crypt_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramSessionCryptInfoSigned> {
        &mut self.crypt
    }

    // optional .CMsgSteamDatagramCertificateSigned cert = 8;

    pub fn clear_cert(&mut self) {
        self.cert.clear();
    }

    pub fn has_cert(&self) -> bool {
        self.cert.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cert(&mut self, v: super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned) {
        self.cert = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cert(&mut self) -> &mut super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned {
        if self.cert.is_none() {
            self.cert.set_default();
        }
        self.cert.as_mut().unwrap()
    }

    // Take field
    pub fn take_cert(&mut self) -> super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned {
        self.cert.take().unwrap_or_else(|| super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned::new())
    }

    pub fn get_cert(&self) -> &super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned {
        self.cert.as_ref().unwrap_or_else(|| super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned::default_instance())
    }

    fn get_cert_for_reflect(&self) -> &::protobuf::SingularPtrField<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned> {
        &self.cert
    }

    fn mut_cert_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned> {
        &mut self.cert
    }

    // optional uint32 protocol_version = 9;

    pub fn clear_protocol_version(&mut self) {
        self.protocol_version = ::std::option::Option::None;
    }

    pub fn has_protocol_version(&self) -> bool {
        self.protocol_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol_version(&mut self, v: u32) {
        self.protocol_version = ::std::option::Option::Some(v);
    }

    pub fn get_protocol_version(&self) -> u32 {
        self.protocol_version.unwrap_or(0)
    }

    fn get_protocol_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.protocol_version
    }

    fn mut_protocol_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.protocol_version
    }
}

impl ::protobuf::Message for CMsgSteamSockets_UDP_ConnectOK {
    fn is_initialized(&self) -> bool {
        for v in &self.crypt {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cert {
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
                    self.client_connection_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.server_connection_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.server_steam_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.your_timestamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.delay_time_usec = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.crypt)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cert)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.protocol_version = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.client_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.server_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.server_steam_id {
            my_size += 9;
        }
        if let Some(v) = self.your_timestamp {
            my_size += 9;
        }
        if let Some(v) = self.delay_time_usec {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.crypt.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.cert.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.protocol_version {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_connection_id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.server_connection_id {
            os.write_fixed32(5, v)?;
        }
        if let Some(v) = self.server_steam_id {
            os.write_fixed64(2, v)?;
        }
        if let Some(v) = self.your_timestamp {
            os.write_fixed64(3, v)?;
        }
        if let Some(v) = self.delay_time_usec {
            os.write_uint32(4, v)?;
        }
        if let Some(ref v) = self.crypt.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.cert.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.protocol_version {
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

impl ::protobuf::MessageStatic for CMsgSteamSockets_UDP_ConnectOK {
    fn new() -> CMsgSteamSockets_UDP_ConnectOK {
        CMsgSteamSockets_UDP_ConnectOK::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamSockets_UDP_ConnectOK>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_connection_id",
                    CMsgSteamSockets_UDP_ConnectOK::get_client_connection_id_for_reflect,
                    CMsgSteamSockets_UDP_ConnectOK::mut_client_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "server_connection_id",
                    CMsgSteamSockets_UDP_ConnectOK::get_server_connection_id_for_reflect,
                    CMsgSteamSockets_UDP_ConnectOK::mut_server_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "server_steam_id",
                    CMsgSteamSockets_UDP_ConnectOK::get_server_steam_id_for_reflect,
                    CMsgSteamSockets_UDP_ConnectOK::mut_server_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "your_timestamp",
                    CMsgSteamSockets_UDP_ConnectOK::get_your_timestamp_for_reflect,
                    CMsgSteamSockets_UDP_ConnectOK::mut_your_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "delay_time_usec",
                    CMsgSteamSockets_UDP_ConnectOK::get_delay_time_usec_for_reflect,
                    CMsgSteamSockets_UDP_ConnectOK::mut_delay_time_usec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramSessionCryptInfoSigned>>(
                    "crypt",
                    CMsgSteamSockets_UDP_ConnectOK::get_crypt_for_reflect,
                    CMsgSteamSockets_UDP_ConnectOK::mut_crypt_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::steamdatagram_auth_messages::CMsgSteamDatagramCertificateSigned>>(
                    "cert",
                    CMsgSteamSockets_UDP_ConnectOK::get_cert_for_reflect,
                    CMsgSteamSockets_UDP_ConnectOK::mut_cert_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "protocol_version",
                    CMsgSteamSockets_UDP_ConnectOK::get_protocol_version_for_reflect,
                    CMsgSteamSockets_UDP_ConnectOK::mut_protocol_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamSockets_UDP_ConnectOK>(
                    "CMsgSteamSockets_UDP_ConnectOK",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamSockets_UDP_ConnectOK {
    fn clear(&mut self) {
        self.clear_client_connection_id();
        self.clear_server_connection_id();
        self.clear_server_steam_id();
        self.clear_your_timestamp();
        self.clear_delay_time_usec();
        self.clear_crypt();
        self.clear_cert();
        self.clear_protocol_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamSockets_UDP_ConnectOK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamSockets_UDP_ConnectOK {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamSockets_UDP_ConnectionClosed {
    // message fields
    legacy_client_connection_id: ::std::option::Option<u32>,
    to_connection_id: ::std::option::Option<u32>,
    from_connection_id: ::std::option::Option<u32>,
    debug: ::protobuf::SingularField<::std::string::String>,
    reason_code: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamSockets_UDP_ConnectionClosed {}

impl CMsgSteamSockets_UDP_ConnectionClosed {
    pub fn new() -> CMsgSteamSockets_UDP_ConnectionClosed {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamSockets_UDP_ConnectionClosed {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamSockets_UDP_ConnectionClosed> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamSockets_UDP_ConnectionClosed,
        };
        unsafe {
            instance.get(CMsgSteamSockets_UDP_ConnectionClosed::new)
        }
    }

    // optional fixed32 legacy_client_connection_id = 1;

    pub fn clear_legacy_client_connection_id(&mut self) {
        self.legacy_client_connection_id = ::std::option::Option::None;
    }

    pub fn has_legacy_client_connection_id(&self) -> bool {
        self.legacy_client_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_client_connection_id(&mut self, v: u32) {
        self.legacy_client_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_legacy_client_connection_id(&self) -> u32 {
        self.legacy_client_connection_id.unwrap_or(0)
    }

    fn get_legacy_client_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.legacy_client_connection_id
    }

    fn mut_legacy_client_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.legacy_client_connection_id
    }

    // optional fixed32 to_connection_id = 4;

    pub fn clear_to_connection_id(&mut self) {
        self.to_connection_id = ::std::option::Option::None;
    }

    pub fn has_to_connection_id(&self) -> bool {
        self.to_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to_connection_id(&mut self, v: u32) {
        self.to_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_to_connection_id(&self) -> u32 {
        self.to_connection_id.unwrap_or(0)
    }

    fn get_to_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.to_connection_id
    }

    fn mut_to_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.to_connection_id
    }

    // optional fixed32 from_connection_id = 5;

    pub fn clear_from_connection_id(&mut self) {
        self.from_connection_id = ::std::option::Option::None;
    }

    pub fn has_from_connection_id(&self) -> bool {
        self.from_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_connection_id(&mut self, v: u32) {
        self.from_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_from_connection_id(&self) -> u32 {
        self.from_connection_id.unwrap_or(0)
    }

    fn get_from_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.from_connection_id
    }

    fn mut_from_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.from_connection_id
    }

    // optional string debug = 2;

    pub fn clear_debug(&mut self) {
        self.debug.clear();
    }

    pub fn has_debug(&self) -> bool {
        self.debug.is_some()
    }

    // Param is passed by value, moved
    pub fn set_debug(&mut self, v: ::std::string::String) {
        self.debug = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_debug(&mut self) -> &mut ::std::string::String {
        if self.debug.is_none() {
            self.debug.set_default();
        }
        self.debug.as_mut().unwrap()
    }

    // Take field
    pub fn take_debug(&mut self) -> ::std::string::String {
        self.debug.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_debug(&self) -> &str {
        match self.debug.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_debug_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.debug
    }

    fn mut_debug_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.debug
    }

    // optional uint32 reason_code = 3;

    pub fn clear_reason_code(&mut self) {
        self.reason_code = ::std::option::Option::None;
    }

    pub fn has_reason_code(&self) -> bool {
        self.reason_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason_code(&mut self, v: u32) {
        self.reason_code = ::std::option::Option::Some(v);
    }

    pub fn get_reason_code(&self) -> u32 {
        self.reason_code.unwrap_or(0)
    }

    fn get_reason_code_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.reason_code
    }

    fn mut_reason_code_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.reason_code
    }
}

impl ::protobuf::Message for CMsgSteamSockets_UDP_ConnectionClosed {
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
                    self.legacy_client_connection_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.to_connection_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.from_connection_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.debug)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.reason_code = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.legacy_client_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.to_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.from_connection_id {
            my_size += 5;
        }
        if let Some(ref v) = self.debug.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.reason_code {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.legacy_client_connection_id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.to_connection_id {
            os.write_fixed32(4, v)?;
        }
        if let Some(v) = self.from_connection_id {
            os.write_fixed32(5, v)?;
        }
        if let Some(ref v) = self.debug.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.reason_code {
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

impl ::protobuf::MessageStatic for CMsgSteamSockets_UDP_ConnectionClosed {
    fn new() -> CMsgSteamSockets_UDP_ConnectionClosed {
        CMsgSteamSockets_UDP_ConnectionClosed::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamSockets_UDP_ConnectionClosed>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "legacy_client_connection_id",
                    CMsgSteamSockets_UDP_ConnectionClosed::get_legacy_client_connection_id_for_reflect,
                    CMsgSteamSockets_UDP_ConnectionClosed::mut_legacy_client_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "to_connection_id",
                    CMsgSteamSockets_UDP_ConnectionClosed::get_to_connection_id_for_reflect,
                    CMsgSteamSockets_UDP_ConnectionClosed::mut_to_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "from_connection_id",
                    CMsgSteamSockets_UDP_ConnectionClosed::get_from_connection_id_for_reflect,
                    CMsgSteamSockets_UDP_ConnectionClosed::mut_from_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "debug",
                    CMsgSteamSockets_UDP_ConnectionClosed::get_debug_for_reflect,
                    CMsgSteamSockets_UDP_ConnectionClosed::mut_debug_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "reason_code",
                    CMsgSteamSockets_UDP_ConnectionClosed::get_reason_code_for_reflect,
                    CMsgSteamSockets_UDP_ConnectionClosed::mut_reason_code_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamSockets_UDP_ConnectionClosed>(
                    "CMsgSteamSockets_UDP_ConnectionClosed",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamSockets_UDP_ConnectionClosed {
    fn clear(&mut self) {
        self.clear_legacy_client_connection_id();
        self.clear_to_connection_id();
        self.clear_from_connection_id();
        self.clear_debug();
        self.clear_reason_code();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamSockets_UDP_ConnectionClosed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamSockets_UDP_ConnectionClosed {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamSockets_UDP_NoConnection {
    // message fields
    legacy_client_connection_id: ::std::option::Option<u32>,
    from_connection_id: ::std::option::Option<u32>,
    to_connection_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamSockets_UDP_NoConnection {}

impl CMsgSteamSockets_UDP_NoConnection {
    pub fn new() -> CMsgSteamSockets_UDP_NoConnection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamSockets_UDP_NoConnection {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamSockets_UDP_NoConnection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamSockets_UDP_NoConnection,
        };
        unsafe {
            instance.get(CMsgSteamSockets_UDP_NoConnection::new)
        }
    }

    // optional fixed32 legacy_client_connection_id = 1;

    pub fn clear_legacy_client_connection_id(&mut self) {
        self.legacy_client_connection_id = ::std::option::Option::None;
    }

    pub fn has_legacy_client_connection_id(&self) -> bool {
        self.legacy_client_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_client_connection_id(&mut self, v: u32) {
        self.legacy_client_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_legacy_client_connection_id(&self) -> u32 {
        self.legacy_client_connection_id.unwrap_or(0)
    }

    fn get_legacy_client_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.legacy_client_connection_id
    }

    fn mut_legacy_client_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.legacy_client_connection_id
    }

    // optional fixed32 from_connection_id = 2;

    pub fn clear_from_connection_id(&mut self) {
        self.from_connection_id = ::std::option::Option::None;
    }

    pub fn has_from_connection_id(&self) -> bool {
        self.from_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_connection_id(&mut self, v: u32) {
        self.from_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_from_connection_id(&self) -> u32 {
        self.from_connection_id.unwrap_or(0)
    }

    fn get_from_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.from_connection_id
    }

    fn mut_from_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.from_connection_id
    }

    // optional fixed32 to_connection_id = 3;

    pub fn clear_to_connection_id(&mut self) {
        self.to_connection_id = ::std::option::Option::None;
    }

    pub fn has_to_connection_id(&self) -> bool {
        self.to_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to_connection_id(&mut self, v: u32) {
        self.to_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_to_connection_id(&self) -> u32 {
        self.to_connection_id.unwrap_or(0)
    }

    fn get_to_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.to_connection_id
    }

    fn mut_to_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.to_connection_id
    }
}

impl ::protobuf::Message for CMsgSteamSockets_UDP_NoConnection {
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
                    self.legacy_client_connection_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.from_connection_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.to_connection_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.legacy_client_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.from_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.to_connection_id {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.legacy_client_connection_id {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.from_connection_id {
            os.write_fixed32(2, v)?;
        }
        if let Some(v) = self.to_connection_id {
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

impl ::protobuf::MessageStatic for CMsgSteamSockets_UDP_NoConnection {
    fn new() -> CMsgSteamSockets_UDP_NoConnection {
        CMsgSteamSockets_UDP_NoConnection::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamSockets_UDP_NoConnection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "legacy_client_connection_id",
                    CMsgSteamSockets_UDP_NoConnection::get_legacy_client_connection_id_for_reflect,
                    CMsgSteamSockets_UDP_NoConnection::mut_legacy_client_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "from_connection_id",
                    CMsgSteamSockets_UDP_NoConnection::get_from_connection_id_for_reflect,
                    CMsgSteamSockets_UDP_NoConnection::mut_from_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "to_connection_id",
                    CMsgSteamSockets_UDP_NoConnection::get_to_connection_id_for_reflect,
                    CMsgSteamSockets_UDP_NoConnection::mut_to_connection_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamSockets_UDP_NoConnection>(
                    "CMsgSteamSockets_UDP_NoConnection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamSockets_UDP_NoConnection {
    fn clear(&mut self) {
        self.clear_legacy_client_connection_id();
        self.clear_from_connection_id();
        self.clear_to_connection_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamSockets_UDP_NoConnection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamSockets_UDP_NoConnection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamSockets_UDP_Stats {
    // message fields
    stats: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    ack_e2e: ::std::vec::Vec<u32>,
    flags: ::std::option::Option<u32>,
    legacy_client_connection_id: ::std::option::Option<u32>,
    to_connection_id: ::std::option::Option<u32>,
    from_connection_id: ::std::option::Option<u32>,
    seq_num: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamSockets_UDP_Stats {}

impl CMsgSteamSockets_UDP_Stats {
    pub fn new() -> CMsgSteamSockets_UDP_Stats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamSockets_UDP_Stats {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamSockets_UDP_Stats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamSockets_UDP_Stats,
        };
        unsafe {
            instance.get(CMsgSteamSockets_UDP_Stats::new)
        }
    }

    // optional .CMsgSteamDatagramConnectionQuality stats = 1;

    pub fn clear_stats(&mut self) {
        self.stats.clear();
    }

    pub fn has_stats(&self) -> bool {
        self.stats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stats(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.stats = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stats(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.stats.is_none() {
            self.stats.set_default();
        }
        self.stats.as_mut().unwrap()
    }

    // Take field
    pub fn take_stats(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.stats.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_stats(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.stats.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_stats_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.stats
    }

    fn mut_stats_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.stats
    }

    // repeated fixed32 ack_e2e = 2;

    pub fn clear_ack_e2e(&mut self) {
        self.ack_e2e.clear();
    }

    // Param is passed by value, moved
    pub fn set_ack_e2e(&mut self, v: ::std::vec::Vec<u32>) {
        self.ack_e2e = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ack_e2e(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_e2e
    }

    // Take field
    pub fn take_ack_e2e(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.ack_e2e, ::std::vec::Vec::new())
    }

    pub fn get_ack_e2e(&self) -> &[u32] {
        &self.ack_e2e
    }

    fn get_ack_e2e_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.ack_e2e
    }

    fn mut_ack_e2e_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ack_e2e
    }

    // optional uint32 flags = 3;

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

    // optional fixed32 legacy_client_connection_id = 8;

    pub fn clear_legacy_client_connection_id(&mut self) {
        self.legacy_client_connection_id = ::std::option::Option::None;
    }

    pub fn has_legacy_client_connection_id(&self) -> bool {
        self.legacy_client_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_client_connection_id(&mut self, v: u32) {
        self.legacy_client_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_legacy_client_connection_id(&self) -> u32 {
        self.legacy_client_connection_id.unwrap_or(0)
    }

    fn get_legacy_client_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.legacy_client_connection_id
    }

    fn mut_legacy_client_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.legacy_client_connection_id
    }

    // optional fixed32 to_connection_id = 9;

    pub fn clear_to_connection_id(&mut self) {
        self.to_connection_id = ::std::option::Option::None;
    }

    pub fn has_to_connection_id(&self) -> bool {
        self.to_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to_connection_id(&mut self, v: u32) {
        self.to_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_to_connection_id(&self) -> u32 {
        self.to_connection_id.unwrap_or(0)
    }

    fn get_to_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.to_connection_id
    }

    fn mut_to_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.to_connection_id
    }

    // optional fixed32 from_connection_id = 10;

    pub fn clear_from_connection_id(&mut self) {
        self.from_connection_id = ::std::option::Option::None;
    }

    pub fn has_from_connection_id(&self) -> bool {
        self.from_connection_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_connection_id(&mut self, v: u32) {
        self.from_connection_id = ::std::option::Option::Some(v);
    }

    pub fn get_from_connection_id(&self) -> u32 {
        self.from_connection_id.unwrap_or(0)
    }

    fn get_from_connection_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.from_connection_id
    }

    fn mut_from_connection_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.from_connection_id
    }

    // optional uint32 seq_num = 4;

    pub fn clear_seq_num(&mut self) {
        self.seq_num = ::std::option::Option::None;
    }

    pub fn has_seq_num(&self) -> bool {
        self.seq_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num(&mut self, v: u32) {
        self.seq_num = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num(&self) -> u32 {
        self.seq_num.unwrap_or(0)
    }

    fn get_seq_num_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num
    }

    fn mut_seq_num_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num
    }
}

impl ::protobuf::Message for CMsgSteamSockets_UDP_Stats {
    fn is_initialized(&self) -> bool {
        for v in &self.stats {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stats)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.ack_e2e)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.legacy_client_connection_id = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.to_connection_id = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.from_connection_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seq_num = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.stats.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += 5 * self.ack_e2e.len() as u32;
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.legacy_client_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.to_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.from_connection_id {
            my_size += 5;
        }
        if let Some(v) = self.seq_num {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.stats.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.ack_e2e {
            os.write_fixed32(2, *v)?;
        };
        if let Some(v) = self.flags {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.legacy_client_connection_id {
            os.write_fixed32(8, v)?;
        }
        if let Some(v) = self.to_connection_id {
            os.write_fixed32(9, v)?;
        }
        if let Some(v) = self.from_connection_id {
            os.write_fixed32(10, v)?;
        }
        if let Some(v) = self.seq_num {
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

impl ::protobuf::MessageStatic for CMsgSteamSockets_UDP_Stats {
    fn new() -> CMsgSteamSockets_UDP_Stats {
        CMsgSteamSockets_UDP_Stats::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamSockets_UDP_Stats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "stats",
                    CMsgSteamSockets_UDP_Stats::get_stats_for_reflect,
                    CMsgSteamSockets_UDP_Stats::mut_stats_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "ack_e2e",
                    CMsgSteamSockets_UDP_Stats::get_ack_e2e_for_reflect,
                    CMsgSteamSockets_UDP_Stats::mut_ack_e2e_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CMsgSteamSockets_UDP_Stats::get_flags_for_reflect,
                    CMsgSteamSockets_UDP_Stats::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "legacy_client_connection_id",
                    CMsgSteamSockets_UDP_Stats::get_legacy_client_connection_id_for_reflect,
                    CMsgSteamSockets_UDP_Stats::mut_legacy_client_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "to_connection_id",
                    CMsgSteamSockets_UDP_Stats::get_to_connection_id_for_reflect,
                    CMsgSteamSockets_UDP_Stats::mut_to_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "from_connection_id",
                    CMsgSteamSockets_UDP_Stats::get_from_connection_id_for_reflect,
                    CMsgSteamSockets_UDP_Stats::mut_from_connection_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num",
                    CMsgSteamSockets_UDP_Stats::get_seq_num_for_reflect,
                    CMsgSteamSockets_UDP_Stats::mut_seq_num_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamSockets_UDP_Stats>(
                    "CMsgSteamSockets_UDP_Stats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamSockets_UDP_Stats {
    fn clear(&mut self) {
        self.clear_stats();
        self.clear_ack_e2e();
        self.clear_flags();
        self.clear_legacy_client_connection_id();
        self.clear_to_connection_id();
        self.clear_from_connection_id();
        self.clear_seq_num();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamSockets_UDP_Stats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamSockets_UDP_Stats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgSteamSockets_UDP_Stats_Flags {
    ACK_REQUEST_E2E = 2,
    ACK_REQUEST_IMMEDIATE = 4,
}

impl ::protobuf::ProtobufEnum for CMsgSteamSockets_UDP_Stats_Flags {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgSteamSockets_UDP_Stats_Flags> {
        match value {
            2 => ::std::option::Option::Some(CMsgSteamSockets_UDP_Stats_Flags::ACK_REQUEST_E2E),
            4 => ::std::option::Option::Some(CMsgSteamSockets_UDP_Stats_Flags::ACK_REQUEST_IMMEDIATE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgSteamSockets_UDP_Stats_Flags] = &[
            CMsgSteamSockets_UDP_Stats_Flags::ACK_REQUEST_E2E,
            CMsgSteamSockets_UDP_Stats_Flags::ACK_REQUEST_IMMEDIATE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CMsgSteamSockets_UDP_Stats_Flags>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgSteamSockets_UDP_Stats_Flags", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgSteamSockets_UDP_Stats_Flags {
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamSockets_UDP_Stats_Flags {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ESteamDatagramMsgID {
    k_ESteamDatagramMsg_Invalid = 0,
    k_ESteamDatagramMsg_RouterPingRequest = 1,
    k_ESteamDatagramMsg_RouterPingReply = 2,
    k_ESteamDatagramMsg_GameserverPingRequest = 3,
    k_ESteamDatagramMsg_GameserverPingReply = 4,
    k_ESteamDatagramMsg_GameserverSessionRequest = 5,
    k_ESteamDatagramMsg_GameserverSessionEstablished = 6,
    k_ESteamDatagramMsg_NoSession = 7,
    k_ESteamDatagramMsg_Diagnostic = 8,
    k_ESteamDatagramMsg_DataClientToRouter = 9,
    k_ESteamDatagramMsg_DataRouterToServer = 10,
    k_ESteamDatagramMsg_DataServerToRouter = 11,
    k_ESteamDatagramMsg_DataRouterToClient = 12,
    k_ESteamDatagramMsg_Stats = 13,
    k_ESteamDatagramMsg_ClientPingSampleRequest = 14,
    k_ESteamDatagramMsg_ClientPingSampleReply = 15,
    k_ESteamDatagramMsg_ClientToRouterSwitchedPrimary = 16,
    k_ESteamDatagramMsg_RelayHealth = 17,
    k_ESteamDatagramMsg_ConnectRequest = 18,
    k_ESteamDatagramMsg_ConnectOK = 19,
    k_ESteamDatagramMsg_ConnectionClosed = 20,
    k_ESteamDatagramMsg_NoConnection = 21,
}

impl ::protobuf::ProtobufEnum for ESteamDatagramMsgID {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ESteamDatagramMsgID> {
        match value {
            0 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_Invalid),
            1 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_RouterPingRequest),
            2 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_RouterPingReply),
            3 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverPingRequest),
            4 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverPingReply),
            5 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverSessionRequest),
            6 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverSessionEstablished),
            7 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_NoSession),
            8 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_Diagnostic),
            9 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_DataClientToRouter),
            10 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_DataRouterToServer),
            11 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_DataServerToRouter),
            12 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_DataRouterToClient),
            13 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_Stats),
            14 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_ClientPingSampleRequest),
            15 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_ClientPingSampleReply),
            16 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_ClientToRouterSwitchedPrimary),
            17 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_RelayHealth),
            18 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_ConnectRequest),
            19 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_ConnectOK),
            20 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_ConnectionClosed),
            21 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_NoConnection),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ESteamDatagramMsgID] = &[
            ESteamDatagramMsgID::k_ESteamDatagramMsg_Invalid,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_RouterPingRequest,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_RouterPingReply,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverPingRequest,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverPingReply,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverSessionRequest,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverSessionEstablished,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_NoSession,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_Diagnostic,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_DataClientToRouter,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_DataRouterToServer,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_DataServerToRouter,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_DataRouterToClient,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_Stats,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_ClientPingSampleRequest,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_ClientPingSampleReply,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_ClientToRouterSwitchedPrimary,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_RelayHealth,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_ConnectRequest,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_ConnectOK,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_ConnectionClosed,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_NoConnection,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ESteamDatagramMsgID>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ESteamDatagramMsgID", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ESteamDatagramMsgID {
}

impl ::protobuf::reflect::ProtobufValue for ESteamDatagramMsgID {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ESteamNetworkingUDPMsgID {
    k_ESteamNetworkingUDPMsg_ChallengeRequest = 32,
    k_ESteamNetworkingUDPMsg_ChallengeReply = 33,
    k_ESteamNetworkingUDPMsg_ConnectRequest = 34,
    k_ESteamNetworkingUDPMsg_ConnectOK = 35,
    k_ESteamNetworkingUDPMsg_ConnectionClosed = 36,
    k_ESteamNetworkingUDPMsg_NoConnection = 37,
    k_ESteamNetworkingUDPMsg_Stats = 38,
}

impl ::protobuf::ProtobufEnum for ESteamNetworkingUDPMsgID {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ESteamNetworkingUDPMsgID> {
        match value {
            32 => ::std::option::Option::Some(ESteamNetworkingUDPMsgID::k_ESteamNetworkingUDPMsg_ChallengeRequest),
            33 => ::std::option::Option::Some(ESteamNetworkingUDPMsgID::k_ESteamNetworkingUDPMsg_ChallengeReply),
            34 => ::std::option::Option::Some(ESteamNetworkingUDPMsgID::k_ESteamNetworkingUDPMsg_ConnectRequest),
            35 => ::std::option::Option::Some(ESteamNetworkingUDPMsgID::k_ESteamNetworkingUDPMsg_ConnectOK),
            36 => ::std::option::Option::Some(ESteamNetworkingUDPMsgID::k_ESteamNetworkingUDPMsg_ConnectionClosed),
            37 => ::std::option::Option::Some(ESteamNetworkingUDPMsgID::k_ESteamNetworkingUDPMsg_NoConnection),
            38 => ::std::option::Option::Some(ESteamNetworkingUDPMsgID::k_ESteamNetworkingUDPMsg_Stats),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ESteamNetworkingUDPMsgID] = &[
            ESteamNetworkingUDPMsgID::k_ESteamNetworkingUDPMsg_ChallengeRequest,
            ESteamNetworkingUDPMsgID::k_ESteamNetworkingUDPMsg_ChallengeReply,
            ESteamNetworkingUDPMsgID::k_ESteamNetworkingUDPMsg_ConnectRequest,
            ESteamNetworkingUDPMsgID::k_ESteamNetworkingUDPMsg_ConnectOK,
            ESteamNetworkingUDPMsgID::k_ESteamNetworkingUDPMsg_ConnectionClosed,
            ESteamNetworkingUDPMsgID::k_ESteamNetworkingUDPMsg_NoConnection,
            ESteamNetworkingUDPMsgID::k_ESteamNetworkingUDPMsg_Stats,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ESteamNetworkingUDPMsgID>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ESteamNetworkingUDPMsgID", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ESteamNetworkingUDPMsgID {
}

impl ::protobuf::reflect::ProtobufValue for ESteamNetworkingUDPMsgID {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1csteamdatagram_messages.proto\x1a!steamdatagram_auth_messages.proto\
    \"\xfb\x04\n\x20CMsgSteamDatagramRouterPingReply\x12)\n\x10client_timest\
    amp\x18\x01\x20\x01(\x07R\x0fclientTimestamp\x128\n\x16latency_datacente\
    r_ids\x18\x02\x20\x03(\x07R\x14latencyDatacenterIdsB\x02\x10\x01\x12*\n\
    \x0flatency_ping_ms\x18\x03\x20\x03(\rR\rlatencyPingMsB\x02\x10\x01\x12$\
    \n\x0eyour_public_ip\x18\x04\x20\x01(\x07R\x0cyourPublicIp\x12\x1f\n\x0b\
    server_time\x18\x05\x20\x01(\x07R\nserverTime\x12\x1c\n\tchallenge\x18\
    \x06\x20\x01(\x06R\tchallenge\x124\n\x16seconds_until_shutdown\x18\x07\
    \x20\x01(\rR\x14secondsUntilShutdown\x12#\n\rclient_cookie\x18\x08\x20\
    \x01(\x07R\x0cclientCookie\x12A\n\x1dscoring_penalty_relay_cluster\x18\t\
    \x20\x01(\rR\x1ascoringPenaltyRelayCluster\x12[\n\x10route_exceptions\
    \x18\n\x20\x03(\x0b20.CMsgSteamDatagramRouterPingReply.RouteExceptionR\
    \x0frouteExceptions\x1af\n\x0eRouteException\x12$\n\x0edata_center_id\
    \x18\x01\x20\x01(\x07R\x0cdataCenterId\x12\x14\n\x05flags\x18\x02\x20\
    \x01(\rR\x05flags\x12\x18\n\x07penalty\x18\x03\x20\x01(\rR\x07penalty\"\
    \xb0\x03\n\x1fCMsgSteamDatagramGameserverPing\x122\n\x15legacy_client_se\
    ssion\x18\x01\x20\x01(\rR\x13legacyClientSession\x12&\n\x0fclient_steam_\
    id\x18\x02\x20\x01(\x06R\rclientSteamId\x12)\n\x10client_timestamp\x18\
    \x03\x20\x01(\x07R\x0fclientTimestamp\x12)\n\x10router_timestamp\x18\x04\
    \x20\x01(\x07R\x0frouterTimestamp\x12:\n\x19router_gameserver_latency\
    \x18\x05\x20\x01(\rR\x17routerGameserverLatency\x12*\n\x11seq_number_rou\
    ter\x18\x06\x20\x01(\rR\x0fseqNumberRouter\x12$\n\x0eseq_number_e2e\x18\
    \x07\x20\x01(\rR\x0cseqNumberE2e\x12(\n\x10relay_session_id\x18\x08\x20\
    \x01(\rR\x0erelaySessionId\x12#\n\rconnection_id\x18\t\x20\x01(\x07R\x0c\
    connectionId\"\xe5\x01\n!CMsgSteamDatagramSessionCryptInfo\x12O\n\x08key\
    _type\x18\x01\x20\x01(\x0e2+.CMsgSteamDatagramSessionCryptInfo.EKeyType:\
    \x07INVALIDR\x07keyType\x12\x19\n\x08key_data\x18\x02\x20\x01(\x0cR\x07k\
    eyData\x12\x14\n\x05nonce\x18\x03\x20\x01(\x06R\x05nonce\x12\x15\n\x06is\
    _snp\x18\x04\x20\x01(\x08R\x05isSnp\"'\n\x08EKeyType\x12\x0b\n\x07INVALI\
    D\x10\0\x12\x0e\n\nCURVE25519\x10\x01\"[\n'CMsgSteamDatagramSessionCrypt\
    InfoSigned\x12\x12\n\x04info\x18\x01\x20\x01(\x0cR\x04info\x12\x1c\n\tsi\
    gnature\x18\x02\x20\x01(\x0cR\tsignature\"\xcd\x02\n)CMsgSteamDatagramGa\
    meserverSessionRequest\x12\x16\n\x06ticket\x18\x01\x20\x01(\x0cR\x06tick\
    et\x12%\n\x0echallenge_time\x18\x03\x20\x01(\x07R\rchallengeTime\x12\x1c\
    \n\tchallenge\x18\x04\x20\x01(\x06R\tchallenge\x120\n\x14client_connecti\
    on_id\x18\x05\x20\x01(\x07R\x12clientConnectionId\x120\n\x14server_conne\
    ction_id\x18\x08\x20\x01(\x07R\x12serverConnectionId\x124\n\x16network_c\
    onfig_version\x18\x06\x20\x01(\rR\x14networkConfigVersion\x12)\n\x10prot\
    ocol_version\x18\x07\x20\x01(\rR\x0fprotocolVersion\"\x84\x02\n-CMsgStea\
    mDatagramGameserverSessionEstablished\x12#\n\rconnection_id\x18\x01\x20\
    \x01(\x07R\x0cconnectionId\x12.\n\x13gameserver_steam_id\x18\x03\x20\x01\
    (\x06R\x11gameserverSteamId\x124\n\x16seconds_until_shutdown\x18\x04\x20\
    \x01(\rR\x14secondsUntilShutdown\x12(\n\x10relay_session_id\x18\x05\x20\
    \x01(\rR\x0erelaySessionId\x12\x1e\n\x0bseq_num_r2c\x18\x06\x20\x01(\rR\
    \tseqNumR2c\"\x93\x02\n'CMsgSteamDatagramNoSessionRelayToClient\x12(\n\
    \x10relay_session_id\x18\x01\x20\x01(\rR\x0erelaySessionId\x12#\n\rconne\
    ction_id\x18\x07\x20\x01(\x07R\x0cconnectionId\x12$\n\x0eyour_public_ip\
    \x18\x02\x20\x01(\x07R\x0cyourPublicIp\x12\x1f\n\x0bserver_time\x18\x03\
    \x20\x01(\x07R\nserverTime\x12\x1c\n\tchallenge\x18\x04\x20\x01(\x06R\tc\
    hallenge\x124\n\x16seconds_until_shutdown\x18\x05\x20\x01(\rR\x14seconds\
    UntilShutdown\"\xd6\x01\n'CMsgSteamDatagramNoSessionRelayToServer\x12(\n\
    \x10relay_session_id\x18\x01\x20\x01(\rR\x0erelaySessionId\x120\n\x14cli\
    ent_connection_id\x18\x07\x20\x01(\x07R\x12clientConnectionId\x120\n\x14\
    server_connection_id\x18\x08\x20\x01(\x07R\x12serverConnectionId\x12\x1d\
    \n\nkludge_pad\x18c\x20\x01(\x06R\tkludgePad\"M\n\x1bCMsgSteamDatagramDi\
    agnostic\x12\x1a\n\x08severity\x18\x01\x20\x01(\rR\x08severity\x12\x12\n\
    \x04text\x18\x02\x20\x01(\tR\x04text\"\xa1\x02\n\x20CMsgSteamDatagramDat\
    aCenterState\x12O\n\x0cdata_centers\x18\x01\x20\x03(\x0b2,.CMsgSteamData\
    gramDataCenterState.DataCenterR\x0bdataCenters\x1a;\n\x06Server\x12\x18\
    \n\x07address\x18\x01\x20\x01(\tR\x07address\x12\x17\n\x07ping_ms\x18\
    \x02\x20\x01(\rR\x06pingMs\x1ao\n\nDataCenter\x12\x12\n\x04code\x18\x01\
    \x20\x01(\tR\x04code\x12M\n\rserver_sample\x18\x02\x20\x03(\x0b2(.CMsgSt\
    eamDatagramDataCenterState.ServerR\x0cserverSample\"\x97\x03\n'CMsgSteam\
    DatagramLinkInstantaneousStats\x124\n\x17out_packets_per_sec_x10\x18\x01\
    \x20\x01(\rR\x13outPacketsPerSecX10\x12)\n\x11out_bytes_per_sec\x18\x02\
    \x20\x01(\rR\x0eoutBytesPerSec\x122\n\x16in_packets_per_sec_x10\x18\x03\
    \x20\x01(\rR\x12inPacketsPerSecX10\x12'\n\x10in_bytes_per_sec\x18\x04\
    \x20\x01(\rR\rinBytesPerSec\x12\x17\n\x07ping_ms\x18\x05\x20\x01(\rR\x06\
    pingMs\x12.\n\x13packets_dropped_pct\x18\x06\x20\x01(\rR\x11packetsDropp\
    edPct\x12;\n\x1apackets_weird_sequence_pct\x18\x07\x20\x01(\rR\x17packet\
    sWeirdSequencePct\x12(\n\x10peak_jitter_usec\x18\x08\x20\x01(\rR\x0epeak\
    JitterUsec\"\xa4\x0f\n\"CMsgSteamDatagramLinkLifetimeStats\x12!\n\x0cpac\
    kets_sent\x18\x03\x20\x01(\x04R\x0bpacketsSent\x12\x17\n\x07kb_sent\x18\
    \x04\x20\x01(\x04R\x06kbSent\x12!\n\x0cpackets_recv\x18\x05\x20\x01(\x04\
    R\x0bpacketsRecv\x12\x17\n\x07kb_recv\x18\x06\x20\x01(\x04R\x06kbRecv\
    \x124\n\x16packets_recv_sequenced\x18\x07\x20\x01(\x04R\x14packetsRecvSe\
    quenced\x120\n\x14packets_recv_dropped\x18\x08\x20\x01(\x04R\x12packetsR\
    ecvDropped\x128\n\x19packets_recv_out_of_order\x18\t\x20\x01(\x04R\x15pa\
    cketsRecvOutOfOrder\x124\n\x16packets_recv_duplicate\x18\n\x20\x01(\x04R\
    \x14packetsRecvDuplicate\x12,\n\x12packets_recv_lurch\x18\x0b\x20\x01(\
    \x04R\x10packetsRecvLurch\x122\n\x15quality_histogram_100\x18\x15\x20\
    \x01(\rR\x13qualityHistogram100\x120\n\x14quality_histogram_99\x18\x16\
    \x20\x01(\rR\x12qualityHistogram99\x120\n\x14quality_histogram_97\x18\
    \x17\x20\x01(\rR\x12qualityHistogram97\x120\n\x14quality_histogram_95\
    \x18\x18\x20\x01(\rR\x12qualityHistogram95\x120\n\x14quality_histogram_9\
    0\x18\x19\x20\x01(\rR\x12qualityHistogram90\x120\n\x14quality_histogram_\
    75\x18\x1a\x20\x01(\rR\x12qualityHistogram75\x120\n\x14quality_histogram\
    _50\x18\x1b\x20\x01(\rR\x12qualityHistogram50\x12.\n\x13quality_histogra\
    m_1\x18\x1c\x20\x01(\rR\x11qualityHistogram1\x124\n\x16quality_histogram\
    _dead\x18\x1d\x20\x01(\rR\x14qualityHistogramDead\x12*\n\x11quality_ntil\
    e_2nd\x18\x1e\x20\x01(\rR\x0fqualityNtile2nd\x12*\n\x11quality_ntile_5th\
    \x18\x1f\x20\x01(\rR\x0fqualityNtile5th\x12,\n\x12quality_ntile_25th\x18\
    \x20\x20\x01(\rR\x10qualityNtile25th\x12,\n\x12quality_ntile_50th\x18!\
    \x20\x01(\rR\x10qualityNtile50th\x12*\n\x11ping_histogram_25\x18)\x20\
    \x01(\rR\x0fpingHistogram25\x12*\n\x11ping_histogram_50\x18*\x20\x01(\rR\
    \x0fpingHistogram50\x12*\n\x11ping_histogram_75\x18+\x20\x01(\rR\x0fping\
    Histogram75\x12,\n\x12ping_histogram_100\x18,\x20\x01(\rR\x10pingHistogr\
    am100\x12,\n\x12ping_histogram_125\x18-\x20\x01(\rR\x10pingHistogram125\
    \x12,\n\x12ping_histogram_150\x18.\x20\x01(\rR\x10pingHistogram150\x12,\
    \n\x12ping_histogram_200\x18/\x20\x01(\rR\x10pingHistogram200\x12,\n\x12\
    ping_histogram_300\x180\x20\x01(\rR\x10pingHistogram300\x12,\n\x12ping_h\
    istogram_max\x181\x20\x01(\rR\x10pingHistogramMax\x12$\n\x0eping_ntile_5\
    th\x182\x20\x01(\rR\x0cpingNtile5th\x12&\n\x0fping_ntile_50th\x183\x20\
    \x01(\rR\rpingNtile50th\x12&\n\x0fping_ntile_75th\x184\x20\x01(\rR\rping\
    Ntile75th\x12&\n\x0fping_ntile_95th\x185\x20\x01(\rR\rpingNtile95th\x12&\
    \n\x0fping_ntile_98th\x186\x20\x01(\rR\rpingNtile98th\x12>\n\x1bjitter_h\
    istogram_negligible\x18=\x20\x01(\rR\x19jitterHistogramNegligible\x12,\n\
    \x12jitter_histogram_1\x18>\x20\x01(\rR\x10jitterHistogram1\x12,\n\x12ji\
    tter_histogram_2\x18?\x20\x01(\rR\x10jitterHistogram2\x12,\n\x12jitter_h\
    istogram_5\x18@\x20\x01(\rR\x10jitterHistogram5\x12.\n\x13jitter_histogr\
    am_10\x18A\x20\x01(\rR\x11jitterHistogram10\x12.\n\x13jitter_histogram_2\
    0\x18B\x20\x01(\rR\x11jitterHistogram20\"\xb5\x01\n\"CMsgSteamDatagramCo\
    nnectionQuality\x12N\n\rinstantaneous\x18\x01\x20\x01(\x0b2(.CMsgSteamDa\
    tagramLinkInstantaneousStatsR\rinstantaneous\x12?\n\x08lifetime\x18\x02\
    \x20\x01(\x0b2#.CMsgSteamDatagramLinkLifetimeStatsR\x08lifetime\"\xe4\
    \x03\n.CMsgSteamDatagramConnectionStatsClientToRouter\x125\n\x03c2r\x18\
    \x01\x20\x01(\x0b2#.CMsgSteamDatagramConnectionQualityR\x03c2r\x125\n\
    \x03c2s\x18\x02\x20\x01(\x0b2#.CMsgSteamDatagramConnectionQualityR\x03c2\
    s\x126\n\x17legacy_client_timestamp\x18\x03\x20\x01(\x07R\x15legacyClien\
    tTimestamp\x12\x1b\n\tack_relay\x18\x04\x20\x03(\x07R\x08ackRelay\x12\
    \x17\n\x07ack_e2e\x18\x05\x20\x03(\x07R\x06ackE2e\x12\x14\n\x05flags\x18\
    \x06\x20\x01(\rR\x05flags\x120\n\x14client_connection_id\x18\x08\x20\x01\
    (\x07R\x12clientConnectionId\x12\x1e\n\x0bseq_num_c2r\x18\t\x20\x01(\rR\
    \tseqNumC2r\x12\x1e\n\x0bseq_num_c2s\x18\n\x20\x01(\rR\tseqNumC2s\"N\n\
    \x05Flags\x12\x15\n\x11ACK_REQUEST_RELAY\x10\x01\x12\x13\n\x0fACK_REQUES\
    T_E2E\x10\x02\x12\x19\n\x15ACK_REQUEST_IMMEDIATE\x10\x04\"\xdd\x06\n.CMs\
    gSteamDatagramConnectionStatsRouterToClient\x125\n\x03r2c\x18\x01\x20\
    \x01(\x0b2#.CMsgSteamDatagramConnectionQualityR\x03r2c\x125\n\x03s2c\x18\
    \x02\x20\x01(\x0b2#.CMsgSteamDatagramConnectionQualityR\x03s2c\x12L\n#le\
    gacy_client_timestamp_from_router\x18\x03\x20\x01(\x07R\x1flegacyClientT\
    imestampFromRouter\x12L\n#legacy_client_timestamp_from_server\x18\x04\
    \x20\x01(\x07R\x1flegacyClientTimestampFromServer\x12:\n\x19router_games\
    erver_latency\x18\x05\x20\x01(\rR\x17routerGameserverLatency\x124\n\x16s\
    econds_until_shutdown\x18\x06\x20\x01(\rR\x14secondsUntilShutdown\x12,\n\
    \x12migrate_request_ip\x18\n\x20\x01(\x07R\x10migrateRequestIp\x120\n\
    \x14migrate_request_port\x18\x0b\x20\x01(\rR\x12migrateRequestPort\x12A\
    \n\x1dscoring_penalty_relay_cluster\x18\x0c\x20\x01(\rR\x1ascoringPenalt\
    yRelayCluster\x12\x1b\n\tack_relay\x18\r\x20\x03(\x07R\x08ackRelay\x12\
    \x17\n\x07ack_e2e\x18\x0e\x20\x03(\x07R\x06ackE2e\x12\x14\n\x05flags\x18\
    \x0f\x20\x01(\rR\x05flags\x120\n\x14client_connection_id\x18\x07\x20\x01\
    (\x07R\x12clientConnectionId\x12\x1e\n\x0bseq_num_r2c\x18\x08\x20\x01(\r\
    R\tseqNumR2c\x12\x1e\n\x0bseq_num_s2c\x18\t\x20\x01(\rR\tseqNumS2c\"N\n\
    \x05Flags\x12\x15\n\x11ACK_REQUEST_RELAY\x10\x01\x12\x13\n\x0fACK_REQUES\
    T_E2E\x10\x02\x12\x19\n\x15ACK_REQUEST_IMMEDIATE\x10\x04\"\xa0\x05\n.CMs\
    gSteamDatagramConnectionStatsRouterToServer\x125\n\x03r2s\x18\x01\x20\
    \x01(\x0b2#.CMsgSteamDatagramConnectionQualityR\x03r2s\x125\n\x03c2s\x18\
    \x02\x20\x01(\x0b2#.CMsgSteamDatagramConnectionQualityR\x03c2s\x126\n\
    \x17legacy_client_timestamp\x18\x03\x20\x01(\x07R\x15legacyClientTimesta\
    mp\x126\n\x17legacy_router_timestamp\x18\x04\x20\x01(\x07R\x15legacyRout\
    erTimestamp\x12\x1b\n\tack_relay\x18\n\x20\x03(\x07R\x08ackRelay\x12\x17\
    \n\x07ack_e2e\x18\x0b\x20\x03(\x07R\x06ackE2e\x12\x14\n\x05flags\x18\x0c\
    \x20\x01(\rR\x05flags\x12\x1e\n\x0bseq_num_r2s\x18\x05\x20\x01(\rR\tseqN\
    umR2s\x12\x1e\n\x0bseq_num_c2s\x18\x06\x20\x01(\rR\tseqNumC2s\x12&\n\x0f\
    client_steam_id\x18\x07\x20\x01(\x06R\rclientSteamId\x12(\n\x10relay_ses\
    sion_id\x18\x08\x20\x01(\rR\x0erelaySessionId\x120\n\x14client_connectio\
    n_id\x18\t\x20\x01(\x07R\x12clientConnectionId\x120\n\x14server_connecti\
    on_id\x18\r\x20\x01(\x07R\x12serverConnectionId\"N\n\x05Flags\x12\x15\n\
    \x11ACK_REQUEST_RELAY\x10\x01\x12\x13\n\x0fACK_REQUEST_E2E\x10\x02\x12\
    \x19\n\x15ACK_REQUEST_IMMEDIATE\x10\x04\"\xb0\x04\n.CMsgSteamDatagramCon\
    nectionStatsServerToRouter\x125\n\x03s2r\x18\x01\x20\x01(\x0b2#.CMsgStea\
    mDatagramConnectionQualityR\x03s2r\x125\n\x03s2c\x18\x02\x20\x01(\x0b2#.\
    CMsgSteamDatagramConnectionQualityR\x03s2c\x12\x1b\n\tack_relay\x18\x08\
    \x20\x03(\x07R\x08ackRelay\x12\x17\n\x07ack_e2e\x18\t\x20\x03(\x07R\x06a\
    ckE2e\x12\x14\n\x05flags\x18\n\x20\x01(\rR\x05flags\x12\x1e\n\x0bseq_num\
    _s2r\x18\x03\x20\x01(\rR\tseqNumS2r\x12\x1e\n\x0bseq_num_s2c\x18\x04\x20\
    \x01(\rR\tseqNumS2c\x12&\n\x0fclient_steam_id\x18\x05\x20\x01(\x06R\rcli\
    entSteamId\x12(\n\x10relay_session_id\x18\x06\x20\x01(\rR\x0erelaySessio\
    nId\x120\n\x14client_connection_id\x18\x07\x20\x01(\x07R\x12clientConnec\
    tionId\x120\n\x14server_connection_id\x18\x0b\x20\x01(\x07R\x12serverCon\
    nectionId\"N\n\x05Flags\x12\x15\n\x11ACK_REQUEST_RELAY\x10\x01\x12\x13\n\
    \x0fACK_REQUEST_E2E\x10\x02\x12\x19\n\x15ACK_REQUEST_IMMEDIATE\x10\x04\"\
    O\n(CMsgSteamDatagramClientPingSampleRequest\x12#\n\rconnection_id\x18\
    \x01\x20\x01(\x07R\x0cconnectionId\"\xe3\x03\n&CMsgSteamDatagramClientPi\
    ngSampleReply\x12#\n\rconnection_id\x18\x01\x20\x01(\x07R\x0cconnectionI\
    d\x12a\n\x10routing_clusters\x18\x02\x20\x03(\x0b26.CMsgSteamDatagramCli\
    entPingSampleReply.RoutingClusterR\x0froutingClusters\x12U\n\x0cdata_cen\
    ters\x18\x03\x20\x03(\x0b22.CMsgSteamDatagramClientPingSampleReply.DataC\
    enterR\x0bdataCenters\x1ad\n\x0eRoutingCluster\x12\x0e\n\x02id\x18\x01\
    \x20\x01(\x07R\x02id\x12\"\n\rfront_ping_ms\x18\x02\x20\x01(\rR\x0bfront\
    PingMs\x12\x1e\n\x0be2e_ping_ms\x18\x03\x20\x01(\rR\te2ePingMs\x1at\n\nD\
    ataCenter\x12$\n\x0edata_center_id\x18\x01\x20\x01(\x07R\x0cdataCenterId\
    \x12\x20\n\x0cvia_relay_id\x18\x02\x20\x01(\x07R\nviaRelayId\x12\x1e\n\
    \x0be2e_ping_ms\x18\x03\x20\x01(\rR\te2ePingMs\"\xef\x06\n&CMsgSteamData\
    gramClientSwitchedPrimary\x12#\n\rconnection_id\x18\x01\x20\x01(\x07R\
    \x0cconnectionId\x12\x17\n\x07from_ip\x18\x02\x20\x01(\x07R\x06fromIp\
    \x12\x1b\n\tfrom_port\x18\x03\x20\x01(\rR\x08fromPort\x12.\n\x13from_rou\
    ter_cluster\x18\x04\x20\x01(\x07R\x11fromRouterCluster\x12(\n\x10from_ac\
    tive_time\x18\x05\x20\x01(\rR\x0efromActiveTime\x127\n\x18from_active_pa\
    ckets_recv\x18\x06\x20\x01(\rR\x15fromActivePacketsRecv\x12.\n\x13from_d\
    ropped_reason\x18\x07\x20\x01(\tR\x11fromDroppedReason\x12\x15\n\x06gap_\
    ms\x18\x08\x20\x01(\rR\x05gapMs\x12_\n\x10from_quality_now\x18\t\x20\x01\
    (\x0b25.CMsgSteamDatagramClientSwitchedPrimary.RouterQualityR\x0efromQua\
    lityNow\x12[\n\x0eto_quality_now\x18\n\x20\x01(\x0b25.CMsgSteamDatagramC\
    lientSwitchedPrimary.RouterQualityR\x0ctoQualityNow\x12a\n\x11from_quali\
    ty_then\x18\x0b\x20\x01(\x0b25.CMsgSteamDatagramClientSwitchedPrimary.Ro\
    uterQualityR\x0ffromQualityThen\x12]\n\x0fto_quality_then\x18\x0c\x20\
    \x01(\x0b25.CMsgSteamDatagramClientSwitchedPrimary.RouterQualityR\rtoQua\
    lityThen\x1a\x8f\x01\n\rRouterQuality\x12\x14\n\x05score\x18\x01\x20\x01\
    (\rR\x05score\x12\x1d\n\nfront_ping\x18\x02\x20\x01(\rR\tfrontPing\x12\
    \x1b\n\tback_ping\x18\x03\x20\x01(\rR\x08backPing\x12,\n\x12seconds_unti\
    l_down\x18\x04\x20\x01(\rR\x10secondsUntilDown\"\xea\x03\n\x1dCMsgSteamD\
    atagramRouterHealth\x12\x19\n\x08cpu_load\x18\x01\x20\x01(\x02R\x07cpuLo\
    ad\x12'\n\x0factive_sessions\x18\x02\x20\x01(\rR\x0eactiveSessions\x12\"\
    \n\rdata_pkts_sec\x18\x03\x20\x01(\rR\x0bdataPktsSec\x12$\n\x0eother_pkt\
    s_sec\x18\x04\x20\x01(\rR\x0cotherPktsSec\x124\n\x16seconds_until_shutdo\
    wn\x18\x05\x20\x01(\rR\x14secondsUntilShutdown\x12)\n\x11cpu_cost_per_us\
    er\x18\x08\x20\x01(\x02R\x0ecpuCostPerUser\x12-\n\x13cpu_cost_per_packet\
    \x18\t\x20\x01(\x02R\x10cpuCostPerPacket\x12L\n\x0cdata_centers\x18\x06\
    \x20\x03(\x0b2).CMsgSteamDatagramRouterHealth.DataCenterR\x0bdataCenters\
    \x12\x14\n\x05magic\x18\x07\x20\x01(\x06R\x05magic\x1aG\n\nDataCenter\
    \x12#\n\rdatacenter_id\x18\x01\x20\x01(\x07R\x0cdatacenterId\x12\x14\n\
    \x05state\x18\x02\x20\x01(\rR\x05state\"\xff\x02\n\x1fCMsgSteamDatagramC\
    onnectRequest\x12#\n\rconnection_id\x18\x01\x20\x01(\x07R\x0cconnectionI\
    d\x12(\n\x10relay_session_id\x18\x02\x20\x01(\rR\x0erelaySessionId\x12&\
    \n\x0fclient_steam_id\x18\x03\x20\x01(\x06R\rclientSteamId\x12!\n\x0cmy_\
    timestamp\x18\x04\x20\x01(\x06R\x0bmyTimestamp\x12\x1e\n\x0bping_est_ms\
    \x18\x05\x20\x01(\rR\tpingEstMs\x12>\n\x05crypt\x18\x06\x20\x01(\x0b2(.C\
    MsgSteamDatagramSessionCryptInfoSignedR\x05crypt\x127\n\x04cert\x18\x07\
    \x20\x01(\x0b2#.CMsgSteamDatagramCertificateSignedR\x04cert\x12)\n\x10pr\
    otocol_version\x18\x08\x20\x01(\rR\x0fprotocolVersion\"\x9d\x03\n\x1aCMs\
    gSteamDatagramConnectOK\x120\n\x14client_connection_id\x18\x01\x20\x01(\
    \x07R\x12clientConnectionId\x120\n\x14server_connection_id\x18\x07\x20\
    \x01(\x07R\x12serverConnectionId\x12(\n\x10relay_session_id\x18\x02\x20\
    \x01(\rR\x0erelaySessionId\x12%\n\x0eyour_timestamp\x18\x03\x20\x01(\x06\
    R\ryourTimestamp\x12&\n\x0fdelay_time_usec\x18\x04\x20\x01(\rR\rdelayTim\
    eUsec\x12>\n\x05crypt\x18\x05\x20\x01(\x0b2(.CMsgSteamDatagramSessionCry\
    ptInfoSignedR\x05crypt\x127\n\x04cert\x18\x06\x20\x01(\x0b2#.CMsgSteamDa\
    tagramCertificateSignedR\x04cert\x12)\n\x10protocol_version\x18\x08\x20\
    \x01(\rR\x0fprotocolVersion\"\xcb\x03\n!CMsgSteamDatagramConnectionClose\
    d\x12=\n\x1blegacy_client_connection_id\x18\x01\x20\x01(\x07R\x18legacyC\
    lientConnectionId\x12(\n\x10to_connection_id\x18\x07\x20\x01(\x07R\x0eto\
    ConnectionId\x12,\n\x12from_connection_id\x18\x08\x20\x01(\x07R\x10fromC\
    onnectionId\x12(\n\x10relay_session_id\x18\x02\x20\x01(\rR\x0erelaySessi\
    onId\x12\"\n\rpeer_steam_id\x18\x03\x20\x01(\x06R\x0bpeerSteamId\x12R\n\
    \nrelay_mode\x18\x04\x20\x01(\x0e2-.CMsgSteamDatagramConnectionClosed.ER\
    elayMode:\x04NoneR\trelayMode\x12\x14\n\x05debug\x18\x05\x20\x01(\tR\x05\
    debug\x12\x1f\n\x0breason_code\x18\x06\x20\x01(\rR\nreasonCode\"6\n\nERe\
    layMode\x12\x08\n\x04None\x10\0\x12\x0c\n\x08EndToEnd\x10\x01\x12\x10\n\
    \x0cClosedByPeer\x10\x02\"\xc0\x02\n\x1dCMsgSteamDatagramNoConnection\
    \x12=\n\x1blegacy_client_connection_id\x18\x01\x20\x01(\x07R\x18legacyCl\
    ientConnectionId\x12(\n\x10to_connection_id\x18\x05\x20\x01(\x07R\x0etoC\
    onnectionId\x12,\n\x12from_connection_id\x18\x06\x20\x01(\x07R\x10fromCo\
    nnectionId\x12(\n\x10relay_session_id\x18\x02\x20\x01(\rR\x0erelaySessio\
    nId\x12\"\n\rpeer_steam_id\x18\x03\x20\x01(\x06R\x0bpeerSteamId\x12\x1c\
    \n\nend_to_end\x18\x04\x20\x01(\x08R\x08endToEnd\x12\x1c\n\tdummy_pad\
    \x18\xff\x07\x20\x01(\x07R\x08dummyPad\"\x9a\x01\n%CMsgSteamSockets_UDP_\
    ChallengeRequest\x12#\n\rconnection_id\x18\x01\x20\x01(\x07R\x0cconnecti\
    onId\x12!\n\x0cmy_timestamp\x18\x03\x20\x01(\x06R\x0bmyTimestamp\x12)\n\
    \x10protocol_version\x18\x04\x20\x01(\rR\x0fprotocolVersion\"\xba\x01\n#\
    CMsgSteamSockets_UDP_ChallengeReply\x12#\n\rconnection_id\x18\x01\x20\
    \x01(\x07R\x0cconnectionId\x12\x1c\n\tchallenge\x18\x02\x20\x01(\x06R\tc\
    hallenge\x12%\n\x0eyour_timestamp\x18\x03\x20\x01(\x06R\ryourTimestamp\
    \x12)\n\x10protocol_version\x18\x04\x20\x01(\rR\x0fprotocolVersion\"\x84\
    \x03\n#CMsgSteamSockets_UDP_ConnectRequest\x120\n\x14client_connection_i\
    d\x18\x01\x20\x01(\x07R\x12clientConnectionId\x12\x1c\n\tchallenge\x18\
    \x02\x20\x01(\x06R\tchallenge\x12&\n\x0fclient_steam_id\x18\x03\x20\x01(\
    \x06R\rclientSteamId\x12!\n\x0cmy_timestamp\x18\x05\x20\x01(\x06R\x0bmyT\
    imestamp\x12\x1e\n\x0bping_est_ms\x18\x06\x20\x01(\rR\tpingEstMs\x12>\n\
    \x05crypt\x18\x07\x20\x01(\x0b2(.CMsgSteamDatagramSessionCryptInfoSigned\
    R\x05crypt\x127\n\x04cert\x18\x04\x20\x01(\x0b2#.CMsgSteamDatagramCertif\
    icateSignedR\x04cert\x12)\n\x10protocol_version\x18\x08\x20\x01(\rR\x0fp\
    rotocolVersion\"\x9f\x03\n\x1eCMsgSteamSockets_UDP_ConnectOK\x120\n\x14c\
    lient_connection_id\x18\x01\x20\x01(\x07R\x12clientConnectionId\x120\n\
    \x14server_connection_id\x18\x05\x20\x01(\x07R\x12serverConnectionId\x12\
    &\n\x0fserver_steam_id\x18\x02\x20\x01(\x06R\rserverSteamId\x12%\n\x0eyo\
    ur_timestamp\x18\x03\x20\x01(\x06R\ryourTimestamp\x12&\n\x0fdelay_time_u\
    sec\x18\x04\x20\x01(\rR\rdelayTimeUsec\x12>\n\x05crypt\x18\x07\x20\x01(\
    \x0b2(.CMsgSteamDatagramSessionCryptInfoSignedR\x05crypt\x127\n\x04cert\
    \x18\x08\x20\x01(\x0b2#.CMsgSteamDatagramCertificateSignedR\x04cert\x12)\
    \n\x10protocol_version\x18\t\x20\x01(\rR\x0fprotocolVersion\"\xf5\x01\n%\
    CMsgSteamSockets_UDP_ConnectionClosed\x12=\n\x1blegacy_client_connection\
    _id\x18\x01\x20\x01(\x07R\x18legacyClientConnectionId\x12(\n\x10to_conne\
    ction_id\x18\x04\x20\x01(\x07R\x0etoConnectionId\x12,\n\x12from_connecti\
    on_id\x18\x05\x20\x01(\x07R\x10fromConnectionId\x12\x14\n\x05debug\x18\
    \x02\x20\x01(\tR\x05debug\x12\x1f\n\x0breason_code\x18\x03\x20\x01(\rR\n\
    reasonCode\"\xba\x01\n!CMsgSteamSockets_UDP_NoConnection\x12=\n\x1blegac\
    y_client_connection_id\x18\x01\x20\x01(\x07R\x18legacyClientConnectionId\
    \x12,\n\x12from_connection_id\x18\x02\x20\x01(\x07R\x10fromConnectionId\
    \x12(\n\x10to_connection_id\x18\x03\x20\x01(\x07R\x0etoConnectionId\"\
    \xef\x02\n\x1aCMsgSteamSockets_UDP_Stats\x129\n\x05stats\x18\x01\x20\x01\
    (\x0b2#.CMsgSteamDatagramConnectionQualityR\x05stats\x12\x17\n\x07ack_e2\
    e\x18\x02\x20\x03(\x07R\x06ackE2e\x12\x14\n\x05flags\x18\x03\x20\x01(\rR\
    \x05flags\x12=\n\x1blegacy_client_connection_id\x18\x08\x20\x01(\x07R\
    \x18legacyClientConnectionId\x12(\n\x10to_connection_id\x18\t\x20\x01(\
    \x07R\x0etoConnectionId\x12,\n\x12from_connection_id\x18\n\x20\x01(\x07R\
    \x10fromConnectionId\x12\x17\n\x07seq_num\x18\x04\x20\x01(\rR\x06seqNum\
    \"7\n\x05Flags\x12\x13\n\x0fACK_REQUEST_E2E\x10\x02\x12\x19\n\x15ACK_REQ\
    UEST_IMMEDIATE\x10\x04*\xbb\x07\n\x13ESteamDatagramMsgID\x12\x1f\n\x1bk_\
    ESteamDatagramMsg_Invalid\x10\0\x12)\n%k_ESteamDatagramMsg_RouterPingReq\
    uest\x10\x01\x12'\n#k_ESteamDatagramMsg_RouterPingReply\x10\x02\x12-\n)k\
    _ESteamDatagramMsg_GameserverPingRequest\x10\x03\x12+\n'k_ESteamDatagram\
    Msg_GameserverPingReply\x10\x04\x120\n,k_ESteamDatagramMsg_GameserverSes\
    sionRequest\x10\x05\x124\n0k_ESteamDatagramMsg_GameserverSessionEstablis\
    hed\x10\x06\x12!\n\x1dk_ESteamDatagramMsg_NoSession\x10\x07\x12\"\n\x1ek\
    _ESteamDatagramMsg_Diagnostic\x10\x08\x12*\n&k_ESteamDatagramMsg_DataCli\
    entToRouter\x10\t\x12*\n&k_ESteamDatagramMsg_DataRouterToServer\x10\n\
    \x12*\n&k_ESteamDatagramMsg_DataServerToRouter\x10\x0b\x12*\n&k_ESteamDa\
    tagramMsg_DataRouterToClient\x10\x0c\x12\x1d\n\x19k_ESteamDatagramMsg_St\
    ats\x10\r\x12/\n+k_ESteamDatagramMsg_ClientPingSampleRequest\x10\x0e\x12\
    -\n)k_ESteamDatagramMsg_ClientPingSampleReply\x10\x0f\x125\n1k_ESteamDat\
    agramMsg_ClientToRouterSwitchedPrimary\x10\x10\x12#\n\x1fk_ESteamDatagra\
    mMsg_RelayHealth\x10\x11\x12&\n\"k_ESteamDatagramMsg_ConnectRequest\x10\
    \x12\x12!\n\x1dk_ESteamDatagramMsg_ConnectOK\x10\x13\x12(\n$k_ESteamData\
    gramMsg_ConnectionClosed\x10\x14\x12$\n\x20k_ESteamDatagramMsg_NoConnect\
    ion\x10\x15*\xc9\x02\n\x18ESteamNetworkingUDPMsgID\x12-\n)k_ESteamNetwor\
    kingUDPMsg_ChallengeRequest\x10\x20\x12+\n'k_ESteamNetworkingUDPMsg_Chal\
    lengeReply\x10!\x12+\n'k_ESteamNetworkingUDPMsg_ConnectRequest\x10\"\x12\
    &\n\"k_ESteamNetworkingUDPMsg_ConnectOK\x10#\x12-\n)k_ESteamNetworkingUD\
    PMsg_ConnectionClosed\x10$\x12)\n%k_ESteamNetworkingUDPMsg_NoConnection\
    \x10%\x12\"\n\x1ek_ESteamNetworkingUDPMsg_Stats\x10&B\x03\x80\x01\0\
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
