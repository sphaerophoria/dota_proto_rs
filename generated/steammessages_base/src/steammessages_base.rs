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
pub struct CMsgProtoBufHeader {
    // message fields
    steamid: ::std::option::Option<u64>,
    client_sessionid: ::std::option::Option<i32>,
    routing_appid: ::std::option::Option<u32>,
    jobid_source: ::std::option::Option<u64>,
    jobid_target: ::std::option::Option<u64>,
    target_job_name: ::protobuf::SingularField<::std::string::String>,
    seq_num: ::std::option::Option<i32>,
    eresult: ::std::option::Option<i32>,
    error_message: ::protobuf::SingularField<::std::string::String>,
    ip: ::std::option::Option<u32>,
    auth_account_flags: ::std::option::Option<u32>,
    token_source: ::std::option::Option<u32>,
    admin_spoofing_user: ::std::option::Option<bool>,
    transport_error: ::std::option::Option<i32>,
    messageid: ::std::option::Option<u64>,
    publisher_group_id: ::std::option::Option<u32>,
    sysid: ::std::option::Option<u32>,
    trace_tag: ::std::option::Option<u64>,
    webapi_key_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgProtoBufHeader {}

impl CMsgProtoBufHeader {
    pub fn new() -> CMsgProtoBufHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgProtoBufHeader {
        static mut instance: ::protobuf::lazy::Lazy<CMsgProtoBufHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgProtoBufHeader,
        };
        unsafe {
            instance.get(CMsgProtoBufHeader::new)
        }
    }

    // optional fixed64 steamid = 1;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    fn get_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid
    }

    fn mut_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid
    }

    // optional int32 client_sessionid = 2;

    pub fn clear_client_sessionid(&mut self) {
        self.client_sessionid = ::std::option::Option::None;
    }

    pub fn has_client_sessionid(&self) -> bool {
        self.client_sessionid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_sessionid(&mut self, v: i32) {
        self.client_sessionid = ::std::option::Option::Some(v);
    }

    pub fn get_client_sessionid(&self) -> i32 {
        self.client_sessionid.unwrap_or(0)
    }

    fn get_client_sessionid_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.client_sessionid
    }

    fn mut_client_sessionid_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.client_sessionid
    }

    // optional uint32 routing_appid = 3;

    pub fn clear_routing_appid(&mut self) {
        self.routing_appid = ::std::option::Option::None;
    }

    pub fn has_routing_appid(&self) -> bool {
        self.routing_appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_routing_appid(&mut self, v: u32) {
        self.routing_appid = ::std::option::Option::Some(v);
    }

    pub fn get_routing_appid(&self) -> u32 {
        self.routing_appid.unwrap_or(0)
    }

    fn get_routing_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.routing_appid
    }

    fn mut_routing_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.routing_appid
    }

    // optional fixed64 jobid_source = 10;

    pub fn clear_jobid_source(&mut self) {
        self.jobid_source = ::std::option::Option::None;
    }

    pub fn has_jobid_source(&self) -> bool {
        self.jobid_source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jobid_source(&mut self, v: u64) {
        self.jobid_source = ::std::option::Option::Some(v);
    }

    pub fn get_jobid_source(&self) -> u64 {
        self.jobid_source.unwrap_or(18446744073709551615u64)
    }

    fn get_jobid_source_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.jobid_source
    }

    fn mut_jobid_source_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.jobid_source
    }

    // optional fixed64 jobid_target = 11;

    pub fn clear_jobid_target(&mut self) {
        self.jobid_target = ::std::option::Option::None;
    }

    pub fn has_jobid_target(&self) -> bool {
        self.jobid_target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jobid_target(&mut self, v: u64) {
        self.jobid_target = ::std::option::Option::Some(v);
    }

    pub fn get_jobid_target(&self) -> u64 {
        self.jobid_target.unwrap_or(18446744073709551615u64)
    }

    fn get_jobid_target_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.jobid_target
    }

    fn mut_jobid_target_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.jobid_target
    }

    // optional string target_job_name = 12;

    pub fn clear_target_job_name(&mut self) {
        self.target_job_name.clear();
    }

    pub fn has_target_job_name(&self) -> bool {
        self.target_job_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_job_name(&mut self, v: ::std::string::String) {
        self.target_job_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target_job_name(&mut self) -> &mut ::std::string::String {
        if self.target_job_name.is_none() {
            self.target_job_name.set_default();
        }
        self.target_job_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_target_job_name(&mut self) -> ::std::string::String {
        self.target_job_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_target_job_name(&self) -> &str {
        match self.target_job_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_target_job_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.target_job_name
    }

    fn mut_target_job_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.target_job_name
    }

    // optional int32 seq_num = 24;

    pub fn clear_seq_num(&mut self) {
        self.seq_num = ::std::option::Option::None;
    }

    pub fn has_seq_num(&self) -> bool {
        self.seq_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num(&mut self, v: i32) {
        self.seq_num = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num(&self) -> i32 {
        self.seq_num.unwrap_or(0)
    }

    fn get_seq_num_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.seq_num
    }

    fn mut_seq_num_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.seq_num
    }

    // optional int32 eresult = 13;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: i32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult(&self) -> i32 {
        self.eresult.unwrap_or(2i32)
    }

    fn get_eresult_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.eresult
    }

    fn mut_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.eresult
    }

    // optional string error_message = 14;

    pub fn clear_error_message(&mut self) {
        self.error_message.clear();
    }

    pub fn has_error_message(&self) -> bool {
        self.error_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_message(&mut self, v: ::std::string::String) {
        self.error_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_message(&mut self) -> &mut ::std::string::String {
        if self.error_message.is_none() {
            self.error_message.set_default();
        }
        self.error_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_message(&mut self) -> ::std::string::String {
        self.error_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_message(&self) -> &str {
        match self.error_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error_message
    }

    fn mut_error_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error_message
    }

    // optional uint32 ip = 15;

    pub fn clear_ip(&mut self) {
        self.ip = ::std::option::Option::None;
    }

    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: u32) {
        self.ip = ::std::option::Option::Some(v);
    }

    pub fn get_ip(&self) -> u32 {
        self.ip.unwrap_or(0)
    }

    fn get_ip_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ip
    }

    fn mut_ip_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ip
    }

    // optional uint32 auth_account_flags = 16;

    pub fn clear_auth_account_flags(&mut self) {
        self.auth_account_flags = ::std::option::Option::None;
    }

    pub fn has_auth_account_flags(&self) -> bool {
        self.auth_account_flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auth_account_flags(&mut self, v: u32) {
        self.auth_account_flags = ::std::option::Option::Some(v);
    }

    pub fn get_auth_account_flags(&self) -> u32 {
        self.auth_account_flags.unwrap_or(0)
    }

    fn get_auth_account_flags_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.auth_account_flags
    }

    fn mut_auth_account_flags_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.auth_account_flags
    }

    // optional uint32 token_source = 22;

    pub fn clear_token_source(&mut self) {
        self.token_source = ::std::option::Option::None;
    }

    pub fn has_token_source(&self) -> bool {
        self.token_source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token_source(&mut self, v: u32) {
        self.token_source = ::std::option::Option::Some(v);
    }

    pub fn get_token_source(&self) -> u32 {
        self.token_source.unwrap_or(0)
    }

    fn get_token_source_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.token_source
    }

    fn mut_token_source_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.token_source
    }

    // optional bool admin_spoofing_user = 23;

    pub fn clear_admin_spoofing_user(&mut self) {
        self.admin_spoofing_user = ::std::option::Option::None;
    }

    pub fn has_admin_spoofing_user(&self) -> bool {
        self.admin_spoofing_user.is_some()
    }

    // Param is passed by value, moved
    pub fn set_admin_spoofing_user(&mut self, v: bool) {
        self.admin_spoofing_user = ::std::option::Option::Some(v);
    }

    pub fn get_admin_spoofing_user(&self) -> bool {
        self.admin_spoofing_user.unwrap_or(false)
    }

    fn get_admin_spoofing_user_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.admin_spoofing_user
    }

    fn mut_admin_spoofing_user_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.admin_spoofing_user
    }

    // optional int32 transport_error = 17;

    pub fn clear_transport_error(&mut self) {
        self.transport_error = ::std::option::Option::None;
    }

    pub fn has_transport_error(&self) -> bool {
        self.transport_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transport_error(&mut self, v: i32) {
        self.transport_error = ::std::option::Option::Some(v);
    }

    pub fn get_transport_error(&self) -> i32 {
        self.transport_error.unwrap_or(1i32)
    }

    fn get_transport_error_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.transport_error
    }

    fn mut_transport_error_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.transport_error
    }

    // optional uint64 messageid = 18;

    pub fn clear_messageid(&mut self) {
        self.messageid = ::std::option::Option::None;
    }

    pub fn has_messageid(&self) -> bool {
        self.messageid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_messageid(&mut self, v: u64) {
        self.messageid = ::std::option::Option::Some(v);
    }

    pub fn get_messageid(&self) -> u64 {
        self.messageid.unwrap_or(18446744073709551615u64)
    }

    fn get_messageid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.messageid
    }

    fn mut_messageid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.messageid
    }

    // optional uint32 publisher_group_id = 19;

    pub fn clear_publisher_group_id(&mut self) {
        self.publisher_group_id = ::std::option::Option::None;
    }

    pub fn has_publisher_group_id(&self) -> bool {
        self.publisher_group_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publisher_group_id(&mut self, v: u32) {
        self.publisher_group_id = ::std::option::Option::Some(v);
    }

    pub fn get_publisher_group_id(&self) -> u32 {
        self.publisher_group_id.unwrap_or(0)
    }

    fn get_publisher_group_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.publisher_group_id
    }

    fn mut_publisher_group_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.publisher_group_id
    }

    // optional uint32 sysid = 20;

    pub fn clear_sysid(&mut self) {
        self.sysid = ::std::option::Option::None;
    }

    pub fn has_sysid(&self) -> bool {
        self.sysid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sysid(&mut self, v: u32) {
        self.sysid = ::std::option::Option::Some(v);
    }

    pub fn get_sysid(&self) -> u32 {
        self.sysid.unwrap_or(0)
    }

    fn get_sysid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sysid
    }

    fn mut_sysid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sysid
    }

    // optional uint64 trace_tag = 21;

    pub fn clear_trace_tag(&mut self) {
        self.trace_tag = ::std::option::Option::None;
    }

    pub fn has_trace_tag(&self) -> bool {
        self.trace_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trace_tag(&mut self, v: u64) {
        self.trace_tag = ::std::option::Option::Some(v);
    }

    pub fn get_trace_tag(&self) -> u64 {
        self.trace_tag.unwrap_or(0)
    }

    fn get_trace_tag_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.trace_tag
    }

    fn mut_trace_tag_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.trace_tag
    }

    // optional uint32 webapi_key_id = 25;

    pub fn clear_webapi_key_id(&mut self) {
        self.webapi_key_id = ::std::option::Option::None;
    }

    pub fn has_webapi_key_id(&self) -> bool {
        self.webapi_key_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_webapi_key_id(&mut self, v: u32) {
        self.webapi_key_id = ::std::option::Option::Some(v);
    }

    pub fn get_webapi_key_id(&self) -> u32 {
        self.webapi_key_id.unwrap_or(0)
    }

    fn get_webapi_key_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.webapi_key_id
    }

    fn mut_webapi_key_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.webapi_key_id
    }
}

impl ::protobuf::Message for CMsgProtoBufHeader {
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
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.client_sessionid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.routing_appid = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.jobid_source = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.jobid_target = ::std::option::Option::Some(tmp);
                },
                12 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.target_job_name)?;
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.seq_num = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.eresult = ::std::option::Option::Some(tmp);
                },
                14 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_message)?;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ip = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.auth_account_flags = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.token_source = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.admin_spoofing_user = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.transport_error = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.messageid = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.publisher_group_id = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.sysid = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.trace_tag = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.webapi_key_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.steamid {
            my_size += 9;
        }
        if let Some(v) = self.client_sessionid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.routing_appid {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.jobid_source {
            my_size += 9;
        }
        if let Some(v) = self.jobid_target {
            my_size += 9;
        }
        if let Some(ref v) = self.target_job_name.as_ref() {
            my_size += ::protobuf::rt::string_size(12, &v);
        }
        if let Some(v) = self.seq_num {
            my_size += ::protobuf::rt::value_size(24, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.error_message.as_ref() {
            my_size += ::protobuf::rt::string_size(14, &v);
        }
        if let Some(v) = self.ip {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.auth_account_flags {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.token_source {
            my_size += ::protobuf::rt::value_size(22, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.admin_spoofing_user {
            my_size += 3;
        }
        if let Some(v) = self.transport_error {
            my_size += ::protobuf::rt::value_size(17, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.messageid {
            my_size += ::protobuf::rt::value_size(18, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.publisher_group_id {
            my_size += ::protobuf::rt::value_size(19, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.sysid {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.trace_tag {
            my_size += ::protobuf::rt::value_size(21, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.webapi_key_id {
            my_size += ::protobuf::rt::value_size(25, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.client_sessionid {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.routing_appid {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.jobid_source {
            os.write_fixed64(10, v)?;
        }
        if let Some(v) = self.jobid_target {
            os.write_fixed64(11, v)?;
        }
        if let Some(ref v) = self.target_job_name.as_ref() {
            os.write_string(12, &v)?;
        }
        if let Some(v) = self.seq_num {
            os.write_int32(24, v)?;
        }
        if let Some(v) = self.eresult {
            os.write_int32(13, v)?;
        }
        if let Some(ref v) = self.error_message.as_ref() {
            os.write_string(14, &v)?;
        }
        if let Some(v) = self.ip {
            os.write_uint32(15, v)?;
        }
        if let Some(v) = self.auth_account_flags {
            os.write_uint32(16, v)?;
        }
        if let Some(v) = self.token_source {
            os.write_uint32(22, v)?;
        }
        if let Some(v) = self.admin_spoofing_user {
            os.write_bool(23, v)?;
        }
        if let Some(v) = self.transport_error {
            os.write_int32(17, v)?;
        }
        if let Some(v) = self.messageid {
            os.write_uint64(18, v)?;
        }
        if let Some(v) = self.publisher_group_id {
            os.write_uint32(19, v)?;
        }
        if let Some(v) = self.sysid {
            os.write_uint32(20, v)?;
        }
        if let Some(v) = self.trace_tag {
            os.write_uint64(21, v)?;
        }
        if let Some(v) = self.webapi_key_id {
            os.write_uint32(25, v)?;
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

impl ::protobuf::MessageStatic for CMsgProtoBufHeader {
    fn new() -> CMsgProtoBufHeader {
        CMsgProtoBufHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgProtoBufHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CMsgProtoBufHeader::get_steamid_for_reflect,
                    CMsgProtoBufHeader::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "client_sessionid",
                    CMsgProtoBufHeader::get_client_sessionid_for_reflect,
                    CMsgProtoBufHeader::mut_client_sessionid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "routing_appid",
                    CMsgProtoBufHeader::get_routing_appid_for_reflect,
                    CMsgProtoBufHeader::mut_routing_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "jobid_source",
                    CMsgProtoBufHeader::get_jobid_source_for_reflect,
                    CMsgProtoBufHeader::mut_jobid_source_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "jobid_target",
                    CMsgProtoBufHeader::get_jobid_target_for_reflect,
                    CMsgProtoBufHeader::mut_jobid_target_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "target_job_name",
                    CMsgProtoBufHeader::get_target_job_name_for_reflect,
                    CMsgProtoBufHeader::mut_target_job_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "seq_num",
                    CMsgProtoBufHeader::get_seq_num_for_reflect,
                    CMsgProtoBufHeader::mut_seq_num_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eresult",
                    CMsgProtoBufHeader::get_eresult_for_reflect,
                    CMsgProtoBufHeader::mut_eresult_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_message",
                    CMsgProtoBufHeader::get_error_message_for_reflect,
                    CMsgProtoBufHeader::mut_error_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ip",
                    CMsgProtoBufHeader::get_ip_for_reflect,
                    CMsgProtoBufHeader::mut_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "auth_account_flags",
                    CMsgProtoBufHeader::get_auth_account_flags_for_reflect,
                    CMsgProtoBufHeader::mut_auth_account_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "token_source",
                    CMsgProtoBufHeader::get_token_source_for_reflect,
                    CMsgProtoBufHeader::mut_token_source_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "admin_spoofing_user",
                    CMsgProtoBufHeader::get_admin_spoofing_user_for_reflect,
                    CMsgProtoBufHeader::mut_admin_spoofing_user_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "transport_error",
                    CMsgProtoBufHeader::get_transport_error_for_reflect,
                    CMsgProtoBufHeader::mut_transport_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "messageid",
                    CMsgProtoBufHeader::get_messageid_for_reflect,
                    CMsgProtoBufHeader::mut_messageid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "publisher_group_id",
                    CMsgProtoBufHeader::get_publisher_group_id_for_reflect,
                    CMsgProtoBufHeader::mut_publisher_group_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sysid",
                    CMsgProtoBufHeader::get_sysid_for_reflect,
                    CMsgProtoBufHeader::mut_sysid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "trace_tag",
                    CMsgProtoBufHeader::get_trace_tag_for_reflect,
                    CMsgProtoBufHeader::mut_trace_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "webapi_key_id",
                    CMsgProtoBufHeader::get_webapi_key_id_for_reflect,
                    CMsgProtoBufHeader::mut_webapi_key_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgProtoBufHeader>(
                    "CMsgProtoBufHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgProtoBufHeader {
    fn clear(&mut self) {
        self.clear_steamid();
        self.clear_client_sessionid();
        self.clear_routing_appid();
        self.clear_jobid_source();
        self.clear_jobid_target();
        self.clear_target_job_name();
        self.clear_seq_num();
        self.clear_eresult();
        self.clear_error_message();
        self.clear_ip();
        self.clear_auth_account_flags();
        self.clear_token_source();
        self.clear_admin_spoofing_user();
        self.clear_transport_error();
        self.clear_messageid();
        self.clear_publisher_group_id();
        self.clear_sysid();
        self.clear_trace_tag();
        self.clear_webapi_key_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgProtoBufHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgProtoBufHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgMulti {
    // message fields
    size_unzipped: ::std::option::Option<u32>,
    message_body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgMulti {}

impl CMsgMulti {
    pub fn new() -> CMsgMulti {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgMulti {
        static mut instance: ::protobuf::lazy::Lazy<CMsgMulti> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgMulti,
        };
        unsafe {
            instance.get(CMsgMulti::new)
        }
    }

    // optional uint32 size_unzipped = 1;

    pub fn clear_size_unzipped(&mut self) {
        self.size_unzipped = ::std::option::Option::None;
    }

    pub fn has_size_unzipped(&self) -> bool {
        self.size_unzipped.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size_unzipped(&mut self, v: u32) {
        self.size_unzipped = ::std::option::Option::Some(v);
    }

    pub fn get_size_unzipped(&self) -> u32 {
        self.size_unzipped.unwrap_or(0)
    }

    fn get_size_unzipped_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.size_unzipped
    }

    fn mut_size_unzipped_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.size_unzipped
    }

    // optional bytes message_body = 2;

    pub fn clear_message_body(&mut self) {
        self.message_body.clear();
    }

    pub fn has_message_body(&self) -> bool {
        self.message_body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.message_body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message_body(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.message_body.is_none() {
            self.message_body.set_default();
        }
        self.message_body.as_mut().unwrap()
    }

    // Take field
    pub fn take_message_body(&mut self) -> ::std::vec::Vec<u8> {
        self.message_body.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_message_body(&self) -> &[u8] {
        match self.message_body.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_message_body_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.message_body
    }

    fn mut_message_body_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.message_body
    }
}

impl ::protobuf::Message for CMsgMulti {
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
                    self.size_unzipped = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.message_body)?;
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
        if let Some(v) = self.size_unzipped {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.message_body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.size_unzipped {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.message_body.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgMulti {
    fn new() -> CMsgMulti {
        CMsgMulti::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgMulti>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "size_unzipped",
                    CMsgMulti::get_size_unzipped_for_reflect,
                    CMsgMulti::mut_size_unzipped_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "message_body",
                    CMsgMulti::get_message_body_for_reflect,
                    CMsgMulti::mut_message_body_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgMulti>(
                    "CMsgMulti",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgMulti {
    fn clear(&mut self) {
        self.clear_size_unzipped();
        self.clear_message_body();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgMulti {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgMulti {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgProtobufWrapped {
    // message fields
    message_body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgProtobufWrapped {}

impl CMsgProtobufWrapped {
    pub fn new() -> CMsgProtobufWrapped {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgProtobufWrapped {
        static mut instance: ::protobuf::lazy::Lazy<CMsgProtobufWrapped> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgProtobufWrapped,
        };
        unsafe {
            instance.get(CMsgProtobufWrapped::new)
        }
    }

    // optional bytes message_body = 1;

    pub fn clear_message_body(&mut self) {
        self.message_body.clear();
    }

    pub fn has_message_body(&self) -> bool {
        self.message_body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.message_body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message_body(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.message_body.is_none() {
            self.message_body.set_default();
        }
        self.message_body.as_mut().unwrap()
    }

    // Take field
    pub fn take_message_body(&mut self) -> ::std::vec::Vec<u8> {
        self.message_body.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_message_body(&self) -> &[u8] {
        match self.message_body.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_message_body_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.message_body
    }

    fn mut_message_body_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.message_body
    }
}

impl ::protobuf::Message for CMsgProtobufWrapped {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.message_body)?;
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
        if let Some(ref v) = self.message_body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.message_body.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgProtobufWrapped {
    fn new() -> CMsgProtobufWrapped {
        CMsgProtobufWrapped::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgProtobufWrapped>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "message_body",
                    CMsgProtobufWrapped::get_message_body_for_reflect,
                    CMsgProtobufWrapped::mut_message_body_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgProtobufWrapped>(
                    "CMsgProtobufWrapped",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgProtobufWrapped {
    fn clear(&mut self) {
        self.clear_message_body();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgProtobufWrapped {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgProtobufWrapped {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAuthTicket {
    // message fields
    estate: ::std::option::Option<u32>,
    eresult: ::std::option::Option<u32>,
    steamid: ::std::option::Option<u64>,
    gameid: ::std::option::Option<u64>,
    h_steam_pipe: ::std::option::Option<u32>,
    ticket_crc: ::std::option::Option<u32>,
    ticket: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAuthTicket {}

impl CMsgAuthTicket {
    pub fn new() -> CMsgAuthTicket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAuthTicket {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAuthTicket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAuthTicket,
        };
        unsafe {
            instance.get(CMsgAuthTicket::new)
        }
    }

    // optional uint32 estate = 1;

    pub fn clear_estate(&mut self) {
        self.estate = ::std::option::Option::None;
    }

    pub fn has_estate(&self) -> bool {
        self.estate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_estate(&mut self, v: u32) {
        self.estate = ::std::option::Option::Some(v);
    }

    pub fn get_estate(&self) -> u32 {
        self.estate.unwrap_or(0)
    }

    fn get_estate_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.estate
    }

    fn mut_estate_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.estate
    }

    // optional uint32 eresult = 2;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: u32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult(&self) -> u32 {
        self.eresult.unwrap_or(2u32)
    }

    fn get_eresult_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.eresult
    }

    fn mut_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.eresult
    }

    // optional fixed64 steamid = 3;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    fn get_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid
    }

    fn mut_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid
    }

    // optional fixed64 gameid = 4;

    pub fn clear_gameid(&mut self) {
        self.gameid = ::std::option::Option::None;
    }

    pub fn has_gameid(&self) -> bool {
        self.gameid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gameid(&mut self, v: u64) {
        self.gameid = ::std::option::Option::Some(v);
    }

    pub fn get_gameid(&self) -> u64 {
        self.gameid.unwrap_or(0)
    }

    fn get_gameid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.gameid
    }

    fn mut_gameid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.gameid
    }

    // optional uint32 h_steam_pipe = 5;

    pub fn clear_h_steam_pipe(&mut self) {
        self.h_steam_pipe = ::std::option::Option::None;
    }

    pub fn has_h_steam_pipe(&self) -> bool {
        self.h_steam_pipe.is_some()
    }

    // Param is passed by value, moved
    pub fn set_h_steam_pipe(&mut self, v: u32) {
        self.h_steam_pipe = ::std::option::Option::Some(v);
    }

    pub fn get_h_steam_pipe(&self) -> u32 {
        self.h_steam_pipe.unwrap_or(0)
    }

    fn get_h_steam_pipe_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.h_steam_pipe
    }

    fn mut_h_steam_pipe_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.h_steam_pipe
    }

    // optional uint32 ticket_crc = 6;

    pub fn clear_ticket_crc(&mut self) {
        self.ticket_crc = ::std::option::Option::None;
    }

    pub fn has_ticket_crc(&self) -> bool {
        self.ticket_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ticket_crc(&mut self, v: u32) {
        self.ticket_crc = ::std::option::Option::Some(v);
    }

    pub fn get_ticket_crc(&self) -> u32 {
        self.ticket_crc.unwrap_or(0)
    }

    fn get_ticket_crc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ticket_crc
    }

    fn mut_ticket_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ticket_crc
    }

    // optional bytes ticket = 7;

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
}

impl ::protobuf::Message for CMsgAuthTicket {
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
                    self.estate = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.eresult = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.gameid = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.h_steam_pipe = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ticket_crc = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ticket)?;
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
        if let Some(v) = self.estate {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.steamid {
            my_size += 9;
        }
        if let Some(v) = self.gameid {
            my_size += 9;
        }
        if let Some(v) = self.h_steam_pipe {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ticket_crc {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.ticket.as_ref() {
            my_size += ::protobuf::rt::bytes_size(7, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.estate {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.eresult {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.steamid {
            os.write_fixed64(3, v)?;
        }
        if let Some(v) = self.gameid {
            os.write_fixed64(4, v)?;
        }
        if let Some(v) = self.h_steam_pipe {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.ticket_crc {
            os.write_uint32(6, v)?;
        }
        if let Some(ref v) = self.ticket.as_ref() {
            os.write_bytes(7, &v)?;
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

impl ::protobuf::MessageStatic for CMsgAuthTicket {
    fn new() -> CMsgAuthTicket {
        CMsgAuthTicket::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAuthTicket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "estate",
                    CMsgAuthTicket::get_estate_for_reflect,
                    CMsgAuthTicket::mut_estate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "eresult",
                    CMsgAuthTicket::get_eresult_for_reflect,
                    CMsgAuthTicket::mut_eresult_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CMsgAuthTicket::get_steamid_for_reflect,
                    CMsgAuthTicket::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "gameid",
                    CMsgAuthTicket::get_gameid_for_reflect,
                    CMsgAuthTicket::mut_gameid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "h_steam_pipe",
                    CMsgAuthTicket::get_h_steam_pipe_for_reflect,
                    CMsgAuthTicket::mut_h_steam_pipe_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ticket_crc",
                    CMsgAuthTicket::get_ticket_crc_for_reflect,
                    CMsgAuthTicket::mut_ticket_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ticket",
                    CMsgAuthTicket::get_ticket_for_reflect,
                    CMsgAuthTicket::mut_ticket_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAuthTicket>(
                    "CMsgAuthTicket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAuthTicket {
    fn clear(&mut self) {
        self.clear_estate();
        self.clear_eresult();
        self.clear_steamid();
        self.clear_gameid();
        self.clear_h_steam_pipe();
        self.clear_ticket_crc();
        self.clear_ticket();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAuthTicket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAuthTicket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCDDBAppDetailCommon {
    // message fields
    appid: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    icon: ::protobuf::SingularField<::std::string::String>,
    logo: ::protobuf::SingularField<::std::string::String>,
    logo_small: ::protobuf::SingularField<::std::string::String>,
    tool: ::std::option::Option<bool>,
    demo: ::std::option::Option<bool>,
    media: ::std::option::Option<bool>,
    community_visible_stats: ::std::option::Option<bool>,
    friendly_name: ::protobuf::SingularField<::std::string::String>,
    propagation: ::protobuf::SingularField<::std::string::String>,
    has_adult_content: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCDDBAppDetailCommon {}

impl CCDDBAppDetailCommon {
    pub fn new() -> CCDDBAppDetailCommon {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCDDBAppDetailCommon {
        static mut instance: ::protobuf::lazy::Lazy<CCDDBAppDetailCommon> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCDDBAppDetailCommon,
        };
        unsafe {
            instance.get(CCDDBAppDetailCommon::new)
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

    // optional string icon = 3;

    pub fn clear_icon(&mut self) {
        self.icon.clear();
    }

    pub fn has_icon(&self) -> bool {
        self.icon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_icon(&mut self, v: ::std::string::String) {
        self.icon = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_icon(&mut self) -> &mut ::std::string::String {
        if self.icon.is_none() {
            self.icon.set_default();
        }
        self.icon.as_mut().unwrap()
    }

    // Take field
    pub fn take_icon(&mut self) -> ::std::string::String {
        self.icon.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_icon(&self) -> &str {
        match self.icon.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_icon_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.icon
    }

    fn mut_icon_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.icon
    }

    // optional string logo = 4;

    pub fn clear_logo(&mut self) {
        self.logo.clear();
    }

    pub fn has_logo(&self) -> bool {
        self.logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logo(&mut self, v: ::std::string::String) {
        self.logo = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_logo(&mut self) -> &mut ::std::string::String {
        if self.logo.is_none() {
            self.logo.set_default();
        }
        self.logo.as_mut().unwrap()
    }

    // Take field
    pub fn take_logo(&mut self) -> ::std::string::String {
        self.logo.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_logo(&self) -> &str {
        match self.logo.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_logo_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.logo
    }

    fn mut_logo_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.logo
    }

    // optional string logo_small = 5;

    pub fn clear_logo_small(&mut self) {
        self.logo_small.clear();
    }

    pub fn has_logo_small(&self) -> bool {
        self.logo_small.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logo_small(&mut self, v: ::std::string::String) {
        self.logo_small = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_logo_small(&mut self) -> &mut ::std::string::String {
        if self.logo_small.is_none() {
            self.logo_small.set_default();
        }
        self.logo_small.as_mut().unwrap()
    }

    // Take field
    pub fn take_logo_small(&mut self) -> ::std::string::String {
        self.logo_small.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_logo_small(&self) -> &str {
        match self.logo_small.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_logo_small_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.logo_small
    }

    fn mut_logo_small_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.logo_small
    }

    // optional bool tool = 6;

    pub fn clear_tool(&mut self) {
        self.tool = ::std::option::Option::None;
    }

    pub fn has_tool(&self) -> bool {
        self.tool.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tool(&mut self, v: bool) {
        self.tool = ::std::option::Option::Some(v);
    }

    pub fn get_tool(&self) -> bool {
        self.tool.unwrap_or(false)
    }

    fn get_tool_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.tool
    }

    fn mut_tool_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.tool
    }

    // optional bool demo = 7;

    pub fn clear_demo(&mut self) {
        self.demo = ::std::option::Option::None;
    }

    pub fn has_demo(&self) -> bool {
        self.demo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_demo(&mut self, v: bool) {
        self.demo = ::std::option::Option::Some(v);
    }

    pub fn get_demo(&self) -> bool {
        self.demo.unwrap_or(false)
    }

    fn get_demo_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.demo
    }

    fn mut_demo_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.demo
    }

    // optional bool media = 8;

    pub fn clear_media(&mut self) {
        self.media = ::std::option::Option::None;
    }

    pub fn has_media(&self) -> bool {
        self.media.is_some()
    }

    // Param is passed by value, moved
    pub fn set_media(&mut self, v: bool) {
        self.media = ::std::option::Option::Some(v);
    }

    pub fn get_media(&self) -> bool {
        self.media.unwrap_or(false)
    }

    fn get_media_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.media
    }

    fn mut_media_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.media
    }

    // optional bool community_visible_stats = 9;

    pub fn clear_community_visible_stats(&mut self) {
        self.community_visible_stats = ::std::option::Option::None;
    }

    pub fn has_community_visible_stats(&self) -> bool {
        self.community_visible_stats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_community_visible_stats(&mut self, v: bool) {
        self.community_visible_stats = ::std::option::Option::Some(v);
    }

    pub fn get_community_visible_stats(&self) -> bool {
        self.community_visible_stats.unwrap_or(false)
    }

    fn get_community_visible_stats_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.community_visible_stats
    }

    fn mut_community_visible_stats_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.community_visible_stats
    }

    // optional string friendly_name = 10;

    pub fn clear_friendly_name(&mut self) {
        self.friendly_name.clear();
    }

    pub fn has_friendly_name(&self) -> bool {
        self.friendly_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friendly_name(&mut self, v: ::std::string::String) {
        self.friendly_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_friendly_name(&mut self) -> &mut ::std::string::String {
        if self.friendly_name.is_none() {
            self.friendly_name.set_default();
        }
        self.friendly_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_friendly_name(&mut self) -> ::std::string::String {
        self.friendly_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_friendly_name(&self) -> &str {
        match self.friendly_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_friendly_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.friendly_name
    }

    fn mut_friendly_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.friendly_name
    }

    // optional string propagation = 11;

    pub fn clear_propagation(&mut self) {
        self.propagation.clear();
    }

    pub fn has_propagation(&self) -> bool {
        self.propagation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_propagation(&mut self, v: ::std::string::String) {
        self.propagation = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_propagation(&mut self) -> &mut ::std::string::String {
        if self.propagation.is_none() {
            self.propagation.set_default();
        }
        self.propagation.as_mut().unwrap()
    }

    // Take field
    pub fn take_propagation(&mut self) -> ::std::string::String {
        self.propagation.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_propagation(&self) -> &str {
        match self.propagation.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_propagation_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.propagation
    }

    fn mut_propagation_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.propagation
    }

    // optional bool has_adult_content = 12;

    pub fn clear_has_adult_content(&mut self) {
        self.has_adult_content = ::std::option::Option::None;
    }

    pub fn has_has_adult_content(&self) -> bool {
        self.has_adult_content.is_some()
    }

    // Param is passed by value, moved
    pub fn set_has_adult_content(&mut self, v: bool) {
        self.has_adult_content = ::std::option::Option::Some(v);
    }

    pub fn get_has_adult_content(&self) -> bool {
        self.has_adult_content.unwrap_or(false)
    }

    fn get_has_adult_content_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.has_adult_content
    }

    fn mut_has_adult_content_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.has_adult_content
    }
}

impl ::protobuf::Message for CCDDBAppDetailCommon {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.icon)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.logo)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.logo_small)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.tool = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.demo = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.media = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.community_visible_stats = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.friendly_name)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.propagation)?;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.has_adult_content = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.icon.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.logo.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.logo_small.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(v) = self.tool {
            my_size += 2;
        }
        if let Some(v) = self.demo {
            my_size += 2;
        }
        if let Some(v) = self.media {
            my_size += 2;
        }
        if let Some(v) = self.community_visible_stats {
            my_size += 2;
        }
        if let Some(ref v) = self.friendly_name.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        if let Some(ref v) = self.propagation.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        }
        if let Some(v) = self.has_adult_content {
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
        if let Some(ref v) = self.icon.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.logo.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.logo_small.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(v) = self.tool {
            os.write_bool(6, v)?;
        }
        if let Some(v) = self.demo {
            os.write_bool(7, v)?;
        }
        if let Some(v) = self.media {
            os.write_bool(8, v)?;
        }
        if let Some(v) = self.community_visible_stats {
            os.write_bool(9, v)?;
        }
        if let Some(ref v) = self.friendly_name.as_ref() {
            os.write_string(10, &v)?;
        }
        if let Some(ref v) = self.propagation.as_ref() {
            os.write_string(11, &v)?;
        }
        if let Some(v) = self.has_adult_content {
            os.write_bool(12, v)?;
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

impl ::protobuf::MessageStatic for CCDDBAppDetailCommon {
    fn new() -> CCDDBAppDetailCommon {
        CCDDBAppDetailCommon::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCDDBAppDetailCommon>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CCDDBAppDetailCommon::get_appid_for_reflect,
                    CCDDBAppDetailCommon::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CCDDBAppDetailCommon::get_name_for_reflect,
                    CCDDBAppDetailCommon::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "icon",
                    CCDDBAppDetailCommon::get_icon_for_reflect,
                    CCDDBAppDetailCommon::mut_icon_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "logo",
                    CCDDBAppDetailCommon::get_logo_for_reflect,
                    CCDDBAppDetailCommon::mut_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "logo_small",
                    CCDDBAppDetailCommon::get_logo_small_for_reflect,
                    CCDDBAppDetailCommon::mut_logo_small_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "tool",
                    CCDDBAppDetailCommon::get_tool_for_reflect,
                    CCDDBAppDetailCommon::mut_tool_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "demo",
                    CCDDBAppDetailCommon::get_demo_for_reflect,
                    CCDDBAppDetailCommon::mut_demo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "media",
                    CCDDBAppDetailCommon::get_media_for_reflect,
                    CCDDBAppDetailCommon::mut_media_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "community_visible_stats",
                    CCDDBAppDetailCommon::get_community_visible_stats_for_reflect,
                    CCDDBAppDetailCommon::mut_community_visible_stats_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "friendly_name",
                    CCDDBAppDetailCommon::get_friendly_name_for_reflect,
                    CCDDBAppDetailCommon::mut_friendly_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "propagation",
                    CCDDBAppDetailCommon::get_propagation_for_reflect,
                    CCDDBAppDetailCommon::mut_propagation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_adult_content",
                    CCDDBAppDetailCommon::get_has_adult_content_for_reflect,
                    CCDDBAppDetailCommon::mut_has_adult_content_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCDDBAppDetailCommon>(
                    "CCDDBAppDetailCommon",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCDDBAppDetailCommon {
    fn clear(&mut self) {
        self.clear_appid();
        self.clear_name();
        self.clear_icon();
        self.clear_logo();
        self.clear_logo_small();
        self.clear_tool();
        self.clear_demo();
        self.clear_media();
        self.clear_community_visible_stats();
        self.clear_friendly_name();
        self.clear_propagation();
        self.clear_has_adult_content();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCDDBAppDetailCommon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCDDBAppDetailCommon {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAppRights {
    // message fields
    edit_info: ::std::option::Option<bool>,
    publish: ::std::option::Option<bool>,
    view_error_data: ::std::option::Option<bool>,
    download: ::std::option::Option<bool>,
    upload_cdkeys: ::std::option::Option<bool>,
    generate_cdkeys: ::std::option::Option<bool>,
    view_financials: ::std::option::Option<bool>,
    manage_ceg: ::std::option::Option<bool>,
    manage_signing: ::std::option::Option<bool>,
    manage_cdkeys: ::std::option::Option<bool>,
    edit_marketing: ::std::option::Option<bool>,
    economy_support: ::std::option::Option<bool>,
    economy_support_supervisor: ::std::option::Option<bool>,
    manage_pricing: ::std::option::Option<bool>,
    broadcast_live: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAppRights {}

impl CMsgAppRights {
    pub fn new() -> CMsgAppRights {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAppRights {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAppRights> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAppRights,
        };
        unsafe {
            instance.get(CMsgAppRights::new)
        }
    }

    // optional bool edit_info = 1;

    pub fn clear_edit_info(&mut self) {
        self.edit_info = ::std::option::Option::None;
    }

    pub fn has_edit_info(&self) -> bool {
        self.edit_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_edit_info(&mut self, v: bool) {
        self.edit_info = ::std::option::Option::Some(v);
    }

    pub fn get_edit_info(&self) -> bool {
        self.edit_info.unwrap_or(false)
    }

    fn get_edit_info_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.edit_info
    }

    fn mut_edit_info_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.edit_info
    }

    // optional bool publish = 2;

    pub fn clear_publish(&mut self) {
        self.publish = ::std::option::Option::None;
    }

    pub fn has_publish(&self) -> bool {
        self.publish.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publish(&mut self, v: bool) {
        self.publish = ::std::option::Option::Some(v);
    }

    pub fn get_publish(&self) -> bool {
        self.publish.unwrap_or(false)
    }

    fn get_publish_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.publish
    }

    fn mut_publish_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.publish
    }

    // optional bool view_error_data = 3;

    pub fn clear_view_error_data(&mut self) {
        self.view_error_data = ::std::option::Option::None;
    }

    pub fn has_view_error_data(&self) -> bool {
        self.view_error_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view_error_data(&mut self, v: bool) {
        self.view_error_data = ::std::option::Option::Some(v);
    }

    pub fn get_view_error_data(&self) -> bool {
        self.view_error_data.unwrap_or(false)
    }

    fn get_view_error_data_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.view_error_data
    }

    fn mut_view_error_data_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.view_error_data
    }

    // optional bool download = 4;

    pub fn clear_download(&mut self) {
        self.download = ::std::option::Option::None;
    }

    pub fn has_download(&self) -> bool {
        self.download.is_some()
    }

    // Param is passed by value, moved
    pub fn set_download(&mut self, v: bool) {
        self.download = ::std::option::Option::Some(v);
    }

    pub fn get_download(&self) -> bool {
        self.download.unwrap_or(false)
    }

    fn get_download_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.download
    }

    fn mut_download_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.download
    }

    // optional bool upload_cdkeys = 5;

    pub fn clear_upload_cdkeys(&mut self) {
        self.upload_cdkeys = ::std::option::Option::None;
    }

    pub fn has_upload_cdkeys(&self) -> bool {
        self.upload_cdkeys.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upload_cdkeys(&mut self, v: bool) {
        self.upload_cdkeys = ::std::option::Option::Some(v);
    }

    pub fn get_upload_cdkeys(&self) -> bool {
        self.upload_cdkeys.unwrap_or(false)
    }

    fn get_upload_cdkeys_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.upload_cdkeys
    }

    fn mut_upload_cdkeys_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.upload_cdkeys
    }

    // optional bool generate_cdkeys = 6;

    pub fn clear_generate_cdkeys(&mut self) {
        self.generate_cdkeys = ::std::option::Option::None;
    }

    pub fn has_generate_cdkeys(&self) -> bool {
        self.generate_cdkeys.is_some()
    }

    // Param is passed by value, moved
    pub fn set_generate_cdkeys(&mut self, v: bool) {
        self.generate_cdkeys = ::std::option::Option::Some(v);
    }

    pub fn get_generate_cdkeys(&self) -> bool {
        self.generate_cdkeys.unwrap_or(false)
    }

    fn get_generate_cdkeys_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.generate_cdkeys
    }

    fn mut_generate_cdkeys_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.generate_cdkeys
    }

    // optional bool view_financials = 7;

    pub fn clear_view_financials(&mut self) {
        self.view_financials = ::std::option::Option::None;
    }

    pub fn has_view_financials(&self) -> bool {
        self.view_financials.is_some()
    }

    // Param is passed by value, moved
    pub fn set_view_financials(&mut self, v: bool) {
        self.view_financials = ::std::option::Option::Some(v);
    }

    pub fn get_view_financials(&self) -> bool {
        self.view_financials.unwrap_or(false)
    }

    fn get_view_financials_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.view_financials
    }

    fn mut_view_financials_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.view_financials
    }

    // optional bool manage_ceg = 8;

    pub fn clear_manage_ceg(&mut self) {
        self.manage_ceg = ::std::option::Option::None;
    }

    pub fn has_manage_ceg(&self) -> bool {
        self.manage_ceg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manage_ceg(&mut self, v: bool) {
        self.manage_ceg = ::std::option::Option::Some(v);
    }

    pub fn get_manage_ceg(&self) -> bool {
        self.manage_ceg.unwrap_or(false)
    }

    fn get_manage_ceg_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.manage_ceg
    }

    fn mut_manage_ceg_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.manage_ceg
    }

    // optional bool manage_signing = 9;

    pub fn clear_manage_signing(&mut self) {
        self.manage_signing = ::std::option::Option::None;
    }

    pub fn has_manage_signing(&self) -> bool {
        self.manage_signing.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manage_signing(&mut self, v: bool) {
        self.manage_signing = ::std::option::Option::Some(v);
    }

    pub fn get_manage_signing(&self) -> bool {
        self.manage_signing.unwrap_or(false)
    }

    fn get_manage_signing_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.manage_signing
    }

    fn mut_manage_signing_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.manage_signing
    }

    // optional bool manage_cdkeys = 10;

    pub fn clear_manage_cdkeys(&mut self) {
        self.manage_cdkeys = ::std::option::Option::None;
    }

    pub fn has_manage_cdkeys(&self) -> bool {
        self.manage_cdkeys.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manage_cdkeys(&mut self, v: bool) {
        self.manage_cdkeys = ::std::option::Option::Some(v);
    }

    pub fn get_manage_cdkeys(&self) -> bool {
        self.manage_cdkeys.unwrap_or(false)
    }

    fn get_manage_cdkeys_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.manage_cdkeys
    }

    fn mut_manage_cdkeys_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.manage_cdkeys
    }

    // optional bool edit_marketing = 11;

    pub fn clear_edit_marketing(&mut self) {
        self.edit_marketing = ::std::option::Option::None;
    }

    pub fn has_edit_marketing(&self) -> bool {
        self.edit_marketing.is_some()
    }

    // Param is passed by value, moved
    pub fn set_edit_marketing(&mut self, v: bool) {
        self.edit_marketing = ::std::option::Option::Some(v);
    }

    pub fn get_edit_marketing(&self) -> bool {
        self.edit_marketing.unwrap_or(false)
    }

    fn get_edit_marketing_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.edit_marketing
    }

    fn mut_edit_marketing_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.edit_marketing
    }

    // optional bool economy_support = 12;

    pub fn clear_economy_support(&mut self) {
        self.economy_support = ::std::option::Option::None;
    }

    pub fn has_economy_support(&self) -> bool {
        self.economy_support.is_some()
    }

    // Param is passed by value, moved
    pub fn set_economy_support(&mut self, v: bool) {
        self.economy_support = ::std::option::Option::Some(v);
    }

    pub fn get_economy_support(&self) -> bool {
        self.economy_support.unwrap_or(false)
    }

    fn get_economy_support_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.economy_support
    }

    fn mut_economy_support_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.economy_support
    }

    // optional bool economy_support_supervisor = 13;

    pub fn clear_economy_support_supervisor(&mut self) {
        self.economy_support_supervisor = ::std::option::Option::None;
    }

    pub fn has_economy_support_supervisor(&self) -> bool {
        self.economy_support_supervisor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_economy_support_supervisor(&mut self, v: bool) {
        self.economy_support_supervisor = ::std::option::Option::Some(v);
    }

    pub fn get_economy_support_supervisor(&self) -> bool {
        self.economy_support_supervisor.unwrap_or(false)
    }

    fn get_economy_support_supervisor_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.economy_support_supervisor
    }

    fn mut_economy_support_supervisor_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.economy_support_supervisor
    }

    // optional bool manage_pricing = 14;

    pub fn clear_manage_pricing(&mut self) {
        self.manage_pricing = ::std::option::Option::None;
    }

    pub fn has_manage_pricing(&self) -> bool {
        self.manage_pricing.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manage_pricing(&mut self, v: bool) {
        self.manage_pricing = ::std::option::Option::Some(v);
    }

    pub fn get_manage_pricing(&self) -> bool {
        self.manage_pricing.unwrap_or(false)
    }

    fn get_manage_pricing_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.manage_pricing
    }

    fn mut_manage_pricing_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.manage_pricing
    }

    // optional bool broadcast_live = 15;

    pub fn clear_broadcast_live(&mut self) {
        self.broadcast_live = ::std::option::Option::None;
    }

    pub fn has_broadcast_live(&self) -> bool {
        self.broadcast_live.is_some()
    }

    // Param is passed by value, moved
    pub fn set_broadcast_live(&mut self, v: bool) {
        self.broadcast_live = ::std::option::Option::Some(v);
    }

    pub fn get_broadcast_live(&self) -> bool {
        self.broadcast_live.unwrap_or(false)
    }

    fn get_broadcast_live_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.broadcast_live
    }

    fn mut_broadcast_live_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.broadcast_live
    }
}

impl ::protobuf::Message for CMsgAppRights {
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
                    self.edit_info = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.publish = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.view_error_data = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.download = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.upload_cdkeys = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.generate_cdkeys = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.view_financials = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.manage_ceg = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.manage_signing = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.manage_cdkeys = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.edit_marketing = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.economy_support = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.economy_support_supervisor = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.manage_pricing = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.broadcast_live = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.edit_info {
            my_size += 2;
        }
        if let Some(v) = self.publish {
            my_size += 2;
        }
        if let Some(v) = self.view_error_data {
            my_size += 2;
        }
        if let Some(v) = self.download {
            my_size += 2;
        }
        if let Some(v) = self.upload_cdkeys {
            my_size += 2;
        }
        if let Some(v) = self.generate_cdkeys {
            my_size += 2;
        }
        if let Some(v) = self.view_financials {
            my_size += 2;
        }
        if let Some(v) = self.manage_ceg {
            my_size += 2;
        }
        if let Some(v) = self.manage_signing {
            my_size += 2;
        }
        if let Some(v) = self.manage_cdkeys {
            my_size += 2;
        }
        if let Some(v) = self.edit_marketing {
            my_size += 2;
        }
        if let Some(v) = self.economy_support {
            my_size += 2;
        }
        if let Some(v) = self.economy_support_supervisor {
            my_size += 2;
        }
        if let Some(v) = self.manage_pricing {
            my_size += 2;
        }
        if let Some(v) = self.broadcast_live {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.edit_info {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.publish {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.view_error_data {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.download {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.upload_cdkeys {
            os.write_bool(5, v)?;
        }
        if let Some(v) = self.generate_cdkeys {
            os.write_bool(6, v)?;
        }
        if let Some(v) = self.view_financials {
            os.write_bool(7, v)?;
        }
        if let Some(v) = self.manage_ceg {
            os.write_bool(8, v)?;
        }
        if let Some(v) = self.manage_signing {
            os.write_bool(9, v)?;
        }
        if let Some(v) = self.manage_cdkeys {
            os.write_bool(10, v)?;
        }
        if let Some(v) = self.edit_marketing {
            os.write_bool(11, v)?;
        }
        if let Some(v) = self.economy_support {
            os.write_bool(12, v)?;
        }
        if let Some(v) = self.economy_support_supervisor {
            os.write_bool(13, v)?;
        }
        if let Some(v) = self.manage_pricing {
            os.write_bool(14, v)?;
        }
        if let Some(v) = self.broadcast_live {
            os.write_bool(15, v)?;
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

impl ::protobuf::MessageStatic for CMsgAppRights {
    fn new() -> CMsgAppRights {
        CMsgAppRights::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAppRights>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "edit_info",
                    CMsgAppRights::get_edit_info_for_reflect,
                    CMsgAppRights::mut_edit_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "publish",
                    CMsgAppRights::get_publish_for_reflect,
                    CMsgAppRights::mut_publish_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "view_error_data",
                    CMsgAppRights::get_view_error_data_for_reflect,
                    CMsgAppRights::mut_view_error_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "download",
                    CMsgAppRights::get_download_for_reflect,
                    CMsgAppRights::mut_download_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "upload_cdkeys",
                    CMsgAppRights::get_upload_cdkeys_for_reflect,
                    CMsgAppRights::mut_upload_cdkeys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "generate_cdkeys",
                    CMsgAppRights::get_generate_cdkeys_for_reflect,
                    CMsgAppRights::mut_generate_cdkeys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "view_financials",
                    CMsgAppRights::get_view_financials_for_reflect,
                    CMsgAppRights::mut_view_financials_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "manage_ceg",
                    CMsgAppRights::get_manage_ceg_for_reflect,
                    CMsgAppRights::mut_manage_ceg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "manage_signing",
                    CMsgAppRights::get_manage_signing_for_reflect,
                    CMsgAppRights::mut_manage_signing_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "manage_cdkeys",
                    CMsgAppRights::get_manage_cdkeys_for_reflect,
                    CMsgAppRights::mut_manage_cdkeys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "edit_marketing",
                    CMsgAppRights::get_edit_marketing_for_reflect,
                    CMsgAppRights::mut_edit_marketing_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "economy_support",
                    CMsgAppRights::get_economy_support_for_reflect,
                    CMsgAppRights::mut_economy_support_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "economy_support_supervisor",
                    CMsgAppRights::get_economy_support_supervisor_for_reflect,
                    CMsgAppRights::mut_economy_support_supervisor_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "manage_pricing",
                    CMsgAppRights::get_manage_pricing_for_reflect,
                    CMsgAppRights::mut_manage_pricing_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "broadcast_live",
                    CMsgAppRights::get_broadcast_live_for_reflect,
                    CMsgAppRights::mut_broadcast_live_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAppRights>(
                    "CMsgAppRights",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAppRights {
    fn clear(&mut self) {
        self.clear_edit_info();
        self.clear_publish();
        self.clear_view_error_data();
        self.clear_download();
        self.clear_upload_cdkeys();
        self.clear_generate_cdkeys();
        self.clear_view_financials();
        self.clear_manage_ceg();
        self.clear_manage_signing();
        self.clear_manage_cdkeys();
        self.clear_edit_marketing();
        self.clear_economy_support();
        self.clear_economy_support_supervisor();
        self.clear_manage_pricing();
        self.clear_broadcast_live();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAppRights {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAppRights {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

pub mod exts {
    use protobuf::Message as Message_imported_for_functions;

    pub const msgpool_soft_limit: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeInt32> = ::protobuf::ext::ExtFieldOptional { field_number: 50000, phantom: ::std::marker::PhantomData };

    pub const msgpool_hard_limit: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeInt32> = ::protobuf::ext::ExtFieldOptional { field_number: 50001, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18steammessages_base.proto\x1a\x20google/protobuf/descriptor.proto\"\
    \xe9\x05\n\x12CMsgProtoBufHeader\x12\x18\n\x07steamid\x18\x01\x20\x01(\
    \x06R\x07steamid\x12)\n\x10client_sessionid\x18\x02\x20\x01(\x05R\x0fcli\
    entSessionid\x12#\n\rrouting_appid\x18\x03\x20\x01(\rR\x0croutingAppid\
    \x127\n\x0cjobid_source\x18\n\x20\x01(\x06:\x1418446744073709551615R\x0b\
    jobidSource\x127\n\x0cjobid_target\x18\x0b\x20\x01(\x06:\x14184467440737\
    09551615R\x0bjobidTarget\x12&\n\x0ftarget_job_name\x18\x0c\x20\x01(\tR\r\
    targetJobName\x12\x17\n\x07seq_num\x18\x18\x20\x01(\x05R\x06seqNum\x12\
    \x1b\n\x07eresult\x18\r\x20\x01(\x05:\x012R\x07eresult\x12#\n\rerror_mes\
    sage\x18\x0e\x20\x01(\tR\x0cerrorMessage\x12\x0e\n\x02ip\x18\x0f\x20\x01\
    (\rR\x02ip\x12,\n\x12auth_account_flags\x18\x10\x20\x01(\rR\x10authAccou\
    ntFlags\x12!\n\x0ctoken_source\x18\x16\x20\x01(\rR\x0btokenSource\x12.\n\
    \x13admin_spoofing_user\x18\x17\x20\x01(\x08R\x11adminSpoofingUser\x12*\
    \n\x0ftransport_error\x18\x11\x20\x01(\x05:\x011R\x0etransportError\x122\
    \n\tmessageid\x18\x12\x20\x01(\x04:\x1418446744073709551615R\tmessageid\
    \x12,\n\x12publisher_group_id\x18\x13\x20\x01(\rR\x10publisherGroupId\
    \x12\x14\n\x05sysid\x18\x14\x20\x01(\rR\x05sysid\x12\x1b\n\ttrace_tag\
    \x18\x15\x20\x01(\x04R\x08traceTag\x12\"\n\rwebapi_key_id\x18\x19\x20\
    \x01(\rR\x0bwebapiKeyId\"S\n\tCMsgMulti\x12#\n\rsize_unzipped\x18\x01\
    \x20\x01(\rR\x0csizeUnzipped\x12!\n\x0cmessage_body\x18\x02\x20\x01(\x0c\
    R\x0bmessageBody\"8\n\x13CMsgProtobufWrapped\x12!\n\x0cmessage_body\x18\
    \x01\x20\x01(\x0cR\x0bmessageBody\"\xd0\x01\n\x0eCMsgAuthTicket\x12\x16\
    \n\x06estate\x18\x01\x20\x01(\rR\x06estate\x12\x1b\n\x07eresult\x18\x02\
    \x20\x01(\r:\x012R\x07eresult\x12\x18\n\x07steamid\x18\x03\x20\x01(\x06R\
    \x07steamid\x12\x16\n\x06gameid\x18\x04\x20\x01(\x06R\x06gameid\x12\x20\
    \n\x0ch_steam_pipe\x18\x05\x20\x01(\rR\nhSteamPipe\x12\x1d\n\nticket_crc\
    \x18\x06\x20\x01(\rR\tticketCrc\x12\x16\n\x06ticket\x18\x07\x20\x01(\x0c\
    R\x06ticket\"\xf0\x02\n\x14CCDDBAppDetailCommon\x12\x14\n\x05appid\x18\
    \x01\x20\x01(\rR\x05appid\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\
    \x12\x12\n\x04icon\x18\x03\x20\x01(\tR\x04icon\x12\x12\n\x04logo\x18\x04\
    \x20\x01(\tR\x04logo\x12\x1d\n\nlogo_small\x18\x05\x20\x01(\tR\tlogoSmal\
    l\x12\x12\n\x04tool\x18\x06\x20\x01(\x08R\x04tool\x12\x12\n\x04demo\x18\
    \x07\x20\x01(\x08R\x04demo\x12\x14\n\x05media\x18\x08\x20\x01(\x08R\x05m\
    edia\x126\n\x17community_visible_stats\x18\t\x20\x01(\x08R\x15communityV\
    isibleStats\x12#\n\rfriendly_name\x18\n\x20\x01(\tR\x0cfriendlyName\x12\
    \x20\n\x0bpropagation\x18\x0b\x20\x01(\tR\x0bpropagation\x12*\n\x11has_a\
    dult_content\x18\x0c\x20\x01(\x08R\x0fhasAdultContent\"\xc8\x04\n\rCMsgA\
    ppRights\x12\x1b\n\tedit_info\x18\x01\x20\x01(\x08R\x08editInfo\x12\x18\
    \n\x07publish\x18\x02\x20\x01(\x08R\x07publish\x12&\n\x0fview_error_data\
    \x18\x03\x20\x01(\x08R\rviewErrorData\x12\x1a\n\x08download\x18\x04\x20\
    \x01(\x08R\x08download\x12#\n\rupload_cdkeys\x18\x05\x20\x01(\x08R\x0cup\
    loadCdkeys\x12'\n\x0fgenerate_cdkeys\x18\x06\x20\x01(\x08R\x0egenerateCd\
    keys\x12'\n\x0fview_financials\x18\x07\x20\x01(\x08R\x0eviewFinancials\
    \x12\x1d\n\nmanage_ceg\x18\x08\x20\x01(\x08R\tmanageCeg\x12%\n\x0emanage\
    _signing\x18\t\x20\x01(\x08R\rmanageSigning\x12#\n\rmanage_cdkeys\x18\n\
    \x20\x01(\x08R\x0cmanageCdkeys\x12%\n\x0eedit_marketing\x18\x0b\x20\x01(\
    \x08R\reditMarketing\x12'\n\x0feconomy_support\x18\x0c\x20\x01(\x08R\x0e\
    economySupport\x12<\n\x1aeconomy_support_supervisor\x18\r\x20\x01(\x08R\
    \x18economySupportSupervisor\x12%\n\x0emanage_pricing\x18\x0e\x20\x01(\
    \x08R\rmanagePricing\x12%\n\x0ebroadcast_live\x18\x0f\x20\x01(\x08R\rbro\
    adcastLive:S\n\x12msgpool_soft_limit\x18\xd0\x86\x03\x20\x01(\x05\x12\
    \x1f.google.protobuf.MessageOptions:\x0232R\x10msgpoolSoftLimit:T\n\x12m\
    sgpool_hard_limit\x18\xd1\x86\x03\x20\x01(\x05\x12\x1f.google.protobuf.M\
    essageOptions:\x03384R\x10msgpoolHardLimitB\x05H\x01\x80\x01\0\
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
