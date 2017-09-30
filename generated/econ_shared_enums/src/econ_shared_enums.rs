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
pub struct CMsgGenericResult {
    // message fields
    eresult: ::std::option::Option<u32>,
    debug_message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGenericResult {}

impl CMsgGenericResult {
    pub fn new() -> CMsgGenericResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGenericResult {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGenericResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGenericResult,
        };
        unsafe {
            instance.get(CMsgGenericResult::new)
        }
    }

    // optional uint32 eresult = 1;

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

    // optional string debug_message = 2;

    pub fn clear_debug_message(&mut self) {
        self.debug_message.clear();
    }

    pub fn has_debug_message(&self) -> bool {
        self.debug_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_debug_message(&mut self, v: ::std::string::String) {
        self.debug_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_debug_message(&mut self) -> &mut ::std::string::String {
        if self.debug_message.is_none() {
            self.debug_message.set_default();
        }
        self.debug_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_debug_message(&mut self) -> ::std::string::String {
        self.debug_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_debug_message(&self) -> &str {
        match self.debug_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_debug_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.debug_message
    }

    fn mut_debug_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.debug_message
    }
}

impl ::protobuf::Message for CMsgGenericResult {
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
                    self.eresult = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.debug_message)?;
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
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.debug_message.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eresult {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.debug_message.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgGenericResult {
    fn new() -> CMsgGenericResult {
        CMsgGenericResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGenericResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "eresult",
                    CMsgGenericResult::get_eresult_for_reflect,
                    CMsgGenericResult::mut_eresult_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "debug_message",
                    CMsgGenericResult::get_debug_message_for_reflect,
                    CMsgGenericResult::mut_debug_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGenericResult>(
                    "CMsgGenericResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGenericResult {
    fn clear(&mut self) {
        self.clear_eresult();
        self.clear_debug_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGenericResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGenericResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EGCEconBaseMsg {
    k_EMsgGCGenericResult = 2579,
}

impl ::protobuf::ProtobufEnum for EGCEconBaseMsg {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EGCEconBaseMsg> {
        match value {
            2579 => ::std::option::Option::Some(EGCEconBaseMsg::k_EMsgGCGenericResult),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EGCEconBaseMsg] = &[
            EGCEconBaseMsg::k_EMsgGCGenericResult,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EGCEconBaseMsg>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EGCEconBaseMsg", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EGCEconBaseMsg {
}

impl ::protobuf::reflect::ProtobufValue for EGCEconBaseMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EGCMsgResponse {
    k_EGCMsgResponseOK = 0,
    k_EGCMsgResponseDenied = 1,
    k_EGCMsgResponseServerError = 2,
    k_EGCMsgResponseTimeout = 3,
    k_EGCMsgResponseInvalid = 4,
    k_EGCMsgResponseNoMatch = 5,
    k_EGCMsgResponseUnknownError = 6,
    k_EGCMsgResponseNotLoggedOn = 7,
    k_EGCMsgFailedToCreate = 8,
}

impl ::protobuf::ProtobufEnum for EGCMsgResponse {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EGCMsgResponse> {
        match value {
            0 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseOK),
            1 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseDenied),
            2 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseServerError),
            3 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseTimeout),
            4 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseInvalid),
            5 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseNoMatch),
            6 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseUnknownError),
            7 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgResponseNotLoggedOn),
            8 => ::std::option::Option::Some(EGCMsgResponse::k_EGCMsgFailedToCreate),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EGCMsgResponse] = &[
            EGCMsgResponse::k_EGCMsgResponseOK,
            EGCMsgResponse::k_EGCMsgResponseDenied,
            EGCMsgResponse::k_EGCMsgResponseServerError,
            EGCMsgResponse::k_EGCMsgResponseTimeout,
            EGCMsgResponse::k_EGCMsgResponseInvalid,
            EGCMsgResponse::k_EGCMsgResponseNoMatch,
            EGCMsgResponse::k_EGCMsgResponseUnknownError,
            EGCMsgResponse::k_EGCMsgResponseNotLoggedOn,
            EGCMsgResponse::k_EGCMsgFailedToCreate,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EGCMsgResponse>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EGCMsgResponse", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EGCMsgResponse {
}

impl ::protobuf::reflect::ProtobufValue for EGCMsgResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EGCPartnerRequestResponse {
    k_EPartnerRequestOK = 1,
    k_EPartnerRequestBadAccount = 2,
    k_EPartnerRequestNotLinked = 3,
    k_EPartnerRequestUnsupportedPartnerType = 4,
}

impl ::protobuf::ProtobufEnum for EGCPartnerRequestResponse {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EGCPartnerRequestResponse> {
        match value {
            1 => ::std::option::Option::Some(EGCPartnerRequestResponse::k_EPartnerRequestOK),
            2 => ::std::option::Option::Some(EGCPartnerRequestResponse::k_EPartnerRequestBadAccount),
            3 => ::std::option::Option::Some(EGCPartnerRequestResponse::k_EPartnerRequestNotLinked),
            4 => ::std::option::Option::Some(EGCPartnerRequestResponse::k_EPartnerRequestUnsupportedPartnerType),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EGCPartnerRequestResponse] = &[
            EGCPartnerRequestResponse::k_EPartnerRequestOK,
            EGCPartnerRequestResponse::k_EPartnerRequestBadAccount,
            EGCPartnerRequestResponse::k_EPartnerRequestNotLinked,
            EGCPartnerRequestResponse::k_EPartnerRequestUnsupportedPartnerType,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EGCPartnerRequestResponse>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EGCPartnerRequestResponse", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EGCPartnerRequestResponse {
}

impl ::protobuf::reflect::ProtobufValue for EGCPartnerRequestResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EGCMsgUseItemResponse {
    k_EGCMsgUseItemResponse_ItemUsed = 0,
    k_EGCMsgUseItemResponse_GiftNoOtherPlayers = 1,
    k_EGCMsgUseItemResponse_ServerError = 2,
    k_EGCMsgUseItemResponse_MiniGameAlreadyStarted = 3,
    k_EGCMsgUseItemResponse_ItemUsed_ItemsGranted = 4,
    k_EGCMsgUseItemResponse_DropRateBonusAlreadyGranted = 5,
    k_EGCMsgUseItemResponse_NotInLowPriorityPool = 6,
    k_EGCMsgUseItemResponse_NotHighEnoughLevel = 7,
    k_EGCMsgUseItemResponse_EventNotActive = 8,
    k_EGCMsgUseItemResponse_ItemUsed_EventPointsGranted = 9,
    k_EGCMsgUseItemResponse_MissingRequirement = 10,
    k_EGCMsgUseItemResponse_EmoticonUnlock_NoNew = 11,
    k_EGCMsgUseItemResponse_EmoticonUnlock_Complete = 12,
    k_EGCMsgUseItemResponse_ItemUsed_Compendium = 13,
}

impl ::protobuf::ProtobufEnum for EGCMsgUseItemResponse {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EGCMsgUseItemResponse> {
        match value {
            0 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed),
            1 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_GiftNoOtherPlayers),
            2 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ServerError),
            3 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_MiniGameAlreadyStarted),
            4 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed_ItemsGranted),
            5 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_DropRateBonusAlreadyGranted),
            6 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_NotInLowPriorityPool),
            7 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_NotHighEnoughLevel),
            8 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_EventNotActive),
            9 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed_EventPointsGranted),
            10 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_MissingRequirement),
            11 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_EmoticonUnlock_NoNew),
            12 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_EmoticonUnlock_Complete),
            13 => ::std::option::Option::Some(EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed_Compendium),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EGCMsgUseItemResponse] = &[
            EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed,
            EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_GiftNoOtherPlayers,
            EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ServerError,
            EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_MiniGameAlreadyStarted,
            EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed_ItemsGranted,
            EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_DropRateBonusAlreadyGranted,
            EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_NotInLowPriorityPool,
            EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_NotHighEnoughLevel,
            EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_EventNotActive,
            EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed_EventPointsGranted,
            EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_MissingRequirement,
            EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_EmoticonUnlock_NoNew,
            EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_EmoticonUnlock_Complete,
            EGCMsgUseItemResponse::k_EGCMsgUseItemResponse_ItemUsed_Compendium,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EGCMsgUseItemResponse>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EGCMsgUseItemResponse", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EGCMsgUseItemResponse {
}

impl ::protobuf::reflect::ProtobufValue for EGCMsgUseItemResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17econ_shared_enums.proto\"U\n\x11CMsgGenericResult\x12\x1b\n\x07ere\
    sult\x18\x01\x20\x01(\r:\x012R\x07eresult\x12#\n\rdebug_message\x18\x02\
    \x20\x01(\tR\x0cdebugMessage*,\n\x0eEGCEconBaseMsg\x12\x1a\n\x15k_EMsgGC\
    GenericResult\x10\x93\x14*\x9b\x02\n\x0eEGCMsgResponse\x12\x16\n\x12k_EG\
    CMsgResponseOK\x10\0\x12\x1a\n\x16k_EGCMsgResponseDenied\x10\x01\x12\x1f\
    \n\x1bk_EGCMsgResponseServerError\x10\x02\x12\x1b\n\x17k_EGCMsgResponseT\
    imeout\x10\x03\x12\x1b\n\x17k_EGCMsgResponseInvalid\x10\x04\x12\x1b\n\
    \x17k_EGCMsgResponseNoMatch\x10\x05\x12\x20\n\x1ck_EGCMsgResponseUnknown\
    Error\x10\x06\x12\x1f\n\x1bk_EGCMsgResponseNotLoggedOn\x10\x07\x12\x1a\n\
    \x16k_EGCMsgFailedToCreate\x10\x08*\xa2\x01\n\x19EGCPartnerRequestRespon\
    se\x12\x17\n\x13k_EPartnerRequestOK\x10\x01\x12\x1f\n\x1bk_EPartnerReque\
    stBadAccount\x10\x02\x12\x1e\n\x1ak_EPartnerRequestNotLinked\x10\x03\x12\
    +\n'k_EPartnerRequestUnsupportedPartnerType\x10\x04*\xc5\x05\n\x15EGCMsg\
    UseItemResponse\x12$\n\x20k_EGCMsgUseItemResponse_ItemUsed\x10\0\x12.\n*\
    k_EGCMsgUseItemResponse_GiftNoOtherPlayers\x10\x01\x12'\n#k_EGCMsgUseIte\
    mResponse_ServerError\x10\x02\x122\n.k_EGCMsgUseItemResponse_MiniGameAlr\
    eadyStarted\x10\x03\x121\n-k_EGCMsgUseItemResponse_ItemUsed_ItemsGranted\
    \x10\x04\x127\n3k_EGCMsgUseItemResponse_DropRateBonusAlreadyGranted\x10\
    \x05\x120\n,k_EGCMsgUseItemResponse_NotInLowPriorityPool\x10\x06\x12.\n*\
    k_EGCMsgUseItemResponse_NotHighEnoughLevel\x10\x07\x12*\n&k_EGCMsgUseIte\
    mResponse_EventNotActive\x10\x08\x127\n3k_EGCMsgUseItemResponse_ItemUsed\
    _EventPointsGranted\x10\t\x12.\n*k_EGCMsgUseItemResponse_MissingRequirem\
    ent\x10\n\x120\n,k_EGCMsgUseItemResponse_EmoticonUnlock_NoNew\x10\x0b\
    \x123\n/k_EGCMsgUseItemResponse_EmoticonUnlock_Complete\x10\x0c\x12/\n+k\
    _EGCMsgUseItemResponse_ItemUsed_Compendium\x10\rB\x05H\x01\x80\x01\0\
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
