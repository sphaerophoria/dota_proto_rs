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

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EProtoExecutionSite {
    k_EProtoExecutionSiteUnknown = 0,
    k_EProtoExecutionSiteSteamClient = 3,
}

impl ::protobuf::ProtobufEnum for EProtoExecutionSite {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EProtoExecutionSite> {
        match value {
            0 => ::std::option::Option::Some(EProtoExecutionSite::k_EProtoExecutionSiteUnknown),
            3 => ::std::option::Option::Some(EProtoExecutionSite::k_EProtoExecutionSiteSteamClient),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EProtoExecutionSite] = &[
            EProtoExecutionSite::k_EProtoExecutionSiteUnknown,
            EProtoExecutionSite::k_EProtoExecutionSiteSteamClient,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EProtoExecutionSite>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EProtoExecutionSite", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EProtoExecutionSite {
}

impl ::protobuf::reflect::ProtobufValue for EProtoExecutionSite {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

pub mod exts {
    use protobuf::Message as Message_imported_for_functions;

    pub const description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 50000, phantom: ::std::marker::PhantomData };

    pub const service_description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::ServiceOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 50000, phantom: ::std::marker::PhantomData };

    pub const service_execution_site: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::ServiceOptions, ::protobuf::types::ProtobufTypeEnum<super::EProtoExecutionSite>> = ::protobuf::ext::ExtFieldOptional { field_number: 50008, phantom: ::std::marker::PhantomData };

    pub const method_description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MethodOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 50000, phantom: ::std::marker::PhantomData };

    pub const enum_description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 50000, phantom: ::std::marker::PhantomData };

    pub const enum_value_description: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumValueOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 50000, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n.steammessages_unified_base.steamworkssdk.proto\x1a\x20google/protobuf\
    /descriptor.proto*]\n\x13EProtoExecutionSite\x12\x20\n\x1ck_EProtoExecut\
    ionSiteUnknown\x10\0\x12$\n\x20k_EProtoExecutionSiteSteamClient\x10\x03:\
    A\n\x0bdescription\x18\xd0\x86\x03\x20\x01(\t\x12\x1d.google.protobuf.Fi\
    eldOptionsR\x0bdescription:R\n\x13service_description\x18\xd0\x86\x03\
    \x20\x01(\t\x12\x1f.google.protobuf.ServiceOptionsR\x12serviceDescriptio\
    n:\x8b\x01\n\x16service_execution_site\x18\xd8\x86\x03\x20\x01(\x0e2\x14\
    .EProtoExecutionSite\x12\x1f.google.protobuf.ServiceOptions:\x1ck_EProto\
    ExecutionSiteUnknownR\x14serviceExecutionSite:O\n\x12method_description\
    \x18\xd0\x86\x03\x20\x01(\t\x12\x1e.google.protobuf.MethodOptionsR\x11me\
    thodDescription:I\n\x10enum_description\x18\xd0\x86\x03\x20\x01(\t\x12\
    \x1c.google.protobuf.EnumOptionsR\x0fenumDescription:Y\n\x16enum_value_d\
    escription\x18\xd0\x86\x03\x20\x01(\t\x12!.google.protobuf.EnumValueOpti\
    onsR\x14enumValueDescriptionB\x05H\x01\x80\x01\0\
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
