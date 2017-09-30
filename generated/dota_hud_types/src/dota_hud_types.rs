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
pub enum EHeroSelectionText {
    k_EHeroSelectionText_Invalid = -1,
    k_EHeroSelectionText_None = 0,
    k_EHeroSelectionText_ChooseHero = 1,
    k_EHeroSelectionText_AllDraft_Planning_YouFirst = 2,
    k_EHeroSelectionText_AllDraft_Planning_TheyFirst = 3,
    k_EHeroSelectionText_AllDraft_BanSelected_YouFirst = 4,
    k_EHeroSelectionText_AllDraft_BanSelected_TheyFirst = 5,
    k_EHeroSelectionText_AllDraft_YouPicking = 6,
    k_EHeroSelectionText_AllDraft_TheyPicking = 7,
    k_EHeroSelectionText_AllDraft_TeammateRandomed = 8,
    k_EHeroSelectionText_AllDraft_YouPicking_LosingGold = 9,
    k_EHeroSelectionText_AllDraft_TheyPicking_LosingGold = 10,
    k_EHeroSelectionText_CaptainsMode_ChooseCaptain = 11,
    k_EHeroSelectionText_CaptainsMode_WaitingForChooseCaptain = 12,
    k_EHeroSelectionText_CaptainsMode_YouSelect = 13,
    k_EHeroSelectionText_CaptainsMode_TheySelect = 14,
    k_EHeroSelectionText_CaptainsMode_YouBan = 15,
    k_EHeroSelectionText_CaptainsMode_TheyBan = 16,
}

impl ::protobuf::ProtobufEnum for EHeroSelectionText {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EHeroSelectionText> {
        match value {
            -1 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_Invalid),
            0 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_None),
            1 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_ChooseHero),
            2 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_Planning_YouFirst),
            3 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_Planning_TheyFirst),
            4 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_BanSelected_YouFirst),
            5 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_BanSelected_TheyFirst),
            6 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_YouPicking),
            7 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_TheyPicking),
            8 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_TeammateRandomed),
            9 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_YouPicking_LosingGold),
            10 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_TheyPicking_LosingGold),
            11 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_CaptainsMode_ChooseCaptain),
            12 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_CaptainsMode_WaitingForChooseCaptain),
            13 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_CaptainsMode_YouSelect),
            14 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_CaptainsMode_TheySelect),
            15 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_CaptainsMode_YouBan),
            16 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_CaptainsMode_TheyBan),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EHeroSelectionText] = &[
            EHeroSelectionText::k_EHeroSelectionText_Invalid,
            EHeroSelectionText::k_EHeroSelectionText_None,
            EHeroSelectionText::k_EHeroSelectionText_ChooseHero,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_Planning_YouFirst,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_Planning_TheyFirst,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_BanSelected_YouFirst,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_BanSelected_TheyFirst,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_YouPicking,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_TheyPicking,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_TeammateRandomed,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_YouPicking_LosingGold,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_TheyPicking_LosingGold,
            EHeroSelectionText::k_EHeroSelectionText_CaptainsMode_ChooseCaptain,
            EHeroSelectionText::k_EHeroSelectionText_CaptainsMode_WaitingForChooseCaptain,
            EHeroSelectionText::k_EHeroSelectionText_CaptainsMode_YouSelect,
            EHeroSelectionText::k_EHeroSelectionText_CaptainsMode_TheySelect,
            EHeroSelectionText::k_EHeroSelectionText_CaptainsMode_YouBan,
            EHeroSelectionText::k_EHeroSelectionText_CaptainsMode_TheyBan,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EHeroSelectionText>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EHeroSelectionText", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EHeroSelectionText {
}

impl ::protobuf::reflect::ProtobufValue for EHeroSelectionText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

pub mod exts {
    use protobuf::Message as Message_imported_for_functions;

    pub const hud_localize_token: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumValueOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 50501, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14dota_hud_types.proto\x1a\x20google/protobuf/descriptor.proto*\xd9\
    \r\n\x12EHeroSelectionText\x12)\n\x1ck_EHeroSelectionText_Invalid\x10\
    \xff\xff\xff\xff\xff\xff\xff\xff\xff\x01\x12\x1d\n\x19k_EHeroSelectionTe\
    xt_None\x10\0\x12H\n\x1fk_EHeroSelectionText_ChooseHero\x10\x01\x1a#\xaa\
    \xd4\x18\x1f#DOTA_Hero_Selection_ChooseHero\x12h\n/k_EHeroSelectionText_\
    AllDraft_Planning_YouFirst\x10\x02\x1a3\xaa\xd4\x18/#DOTA_Hero_Selection\
    _AllDraft_Planning_YouFirst\x12j\n0k_EHeroSelectionText_AllDraft_Plannin\
    g_TheyFirst\x10\x03\x1a4\xaa\xd4\x180#DOTA_Hero_Selection_AllDraft_Plann\
    ing_TheyFirst\x12n\n2k_EHeroSelectionText_AllDraft_BanSelected_YouFirst\
    \x10\x04\x1a6\xaa\xd4\x182#DOTA_Hero_Selection_AllDraft_BanSelected_YouF\
    irst\x12p\n3k_EHeroSelectionText_AllDraft_BanSelected_TheyFirst\x10\x05\
    \x1a7\xaa\xd4\x183#DOTA_Hero_Selection_AllDraft_BanSelected_TheyFirst\
    \x12Z\n(k_EHeroSelectionText_AllDraft_YouPicking\x10\x06\x1a,\xaa\xd4\
    \x18(#DOTA_Hero_Selection_AllDraft_YouPicking\x12\\\n)k_EHeroSelectionTe\
    xt_AllDraft_TheyPicking\x10\x07\x1a-\xaa\xd4\x18)#DOTA_Hero_Selection_Al\
    lDraft_TheyPicking\x12o\n.k_EHeroSelectionText_AllDraft_TeammateRandomed\
    \x10\x08\x1a;\xaa\xd4\x187#DOTA_Hero_Selection_AllDraft_TeammateRandomed\
    _Panorama\x12p\n3k_EHeroSelectionText_AllDraft_YouPicking_LosingGold\x10\
    \t\x1a7\xaa\xd4\x183#DOTA_Hero_Selection_AllDraft_YouPicking_LosingGold\
    \x12r\n4k_EHeroSelectionText_AllDraft_TheyPicking_LosingGold\x10\n\x1a8\
    \xaa\xd4\x184#DOTA_Hero_Selection_AllDraft_TheyPicking_LosingGold\x12h\n\
    /k_EHeroSelectionText_CaptainsMode_ChooseCaptain\x10\x0b\x1a3\xaa\xd4\
    \x18/#DOTA_Hero_Selection_CaptainsMode_ChooseCaptain\x12|\n9k_EHeroSelec\
    tionText_CaptainsMode_WaitingForChooseCaptain\x10\x0c\x1a=\xaa\xd4\x189#\
    DOTA_Hero_Selection_CaptainsMode_WaitingForChooseCaptain\x12`\n+k_EHeroS\
    electionText_CaptainsMode_YouSelect\x10\r\x1a/\xaa\xd4\x18+#DOTA_Hero_Se\
    lection_CaptainsMode_YouSelect\x12b\n,k_EHeroSelectionText_CaptainsMode_\
    TheySelect\x10\x0e\x1a0\xaa\xd4\x18,#DOTA_Hero_Selection_CaptainsMode_Th\
    eySelect\x12Z\n(k_EHeroSelectionText_CaptainsMode_YouBan\x10\x0f\x1a,\
    \xaa\xd4\x18(#DOTA_Hero_Selection_CaptainsMode_YouBan\x12\\\n)k_EHeroSel\
    ectionText_CaptainsMode_TheyBan\x10\x10\x1a-\xaa\xd4\x18)#DOTA_Hero_Sele\
    ction_CaptainsMode_TheyBan:Q\n\x12hud_localize_token\x18\xc5\x8a\x03\x20\
    \x01(\t\x12!.google.protobuf.EnumValueOptionsR\x10hudLocalizeTokenB\x03\
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
