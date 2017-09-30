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
pub enum EGCSystemMsg {
    k_EGCMsgInvalid = 0,
    k_EGCMsgMulti = 1,
    k_EGCMsgGenericReply = 10,
    k_EGCMsgSystemBase = 50,
    k_EGCMsgAchievementAwarded = 51,
    k_EGCMsgConCommand = 52,
    k_EGCMsgStartPlaying = 53,
    k_EGCMsgStopPlaying = 54,
    k_EGCMsgStartGameserver = 55,
    k_EGCMsgStopGameserver = 56,
    k_EGCMsgWGRequest = 57,
    k_EGCMsgWGResponse = 58,
    k_EGCMsgGetUserGameStatsSchema = 59,
    k_EGCMsgGetUserGameStatsSchemaResponse = 60,
    k_EGCMsgGetUserStatsDEPRECATED = 61,
    k_EGCMsgGetUserStatsResponse = 62,
    k_EGCMsgAppInfoUpdated = 63,
    k_EGCMsgValidateSession = 64,
    k_EGCMsgValidateSessionResponse = 65,
    k_EGCMsgLookupAccountFromInput = 66,
    k_EGCMsgSendHTTPRequest = 67,
    k_EGCMsgSendHTTPRequestResponse = 68,
    k_EGCMsgPreTestSetup = 69,
    k_EGCMsgRecordSupportAction = 70,
    k_EGCMsgGetAccountDetails_DEPRECATED = 71,
    k_EGCMsgReceiveInterAppMessage = 73,
    k_EGCMsgFindAccounts = 74,
    k_EGCMsgPostAlert = 75,
    k_EGCMsgGetLicenses = 76,
    k_EGCMsgGetUserStats = 77,
    k_EGCMsgGetCommands = 78,
    k_EGCMsgGetCommandsResponse = 79,
    k_EGCMsgAddFreeLicense = 80,
    k_EGCMsgAddFreeLicenseResponse = 81,
    k_EGCMsgGetIPLocation = 82,
    k_EGCMsgGetIPLocationResponse = 83,
    k_EGCMsgSystemStatsSchema = 84,
    k_EGCMsgGetSystemStats = 85,
    k_EGCMsgGetSystemStatsResponse = 86,
    k_EGCMsgSendEmail = 87,
    k_EGCMsgSendEmailResponse = 88,
    k_EGCMsgGetEmailTemplate = 89,
    k_EGCMsgGetEmailTemplateResponse = 90,
    k_EGCMsgGrantGuestPass = 91,
    k_EGCMsgGrantGuestPassResponse = 92,
    k_EGCMsgGetAccountDetails = 93,
    k_EGCMsgGetAccountDetailsResponse = 94,
    k_EGCMsgGetPersonaNames = 95,
    k_EGCMsgGetPersonaNamesResponse = 96,
    k_EGCMsgMultiplexMsg = 97,
    k_EGCMsgWebAPIRegisterInterfaces = 101,
    k_EGCMsgWebAPIJobRequest = 102,
    k_EGCMsgWebAPIJobRequestHttpResponse = 104,
    k_EGCMsgWebAPIJobRequestForwardResponse = 105,
    k_EGCMsgMemCachedGet = 200,
    k_EGCMsgMemCachedGetResponse = 201,
    k_EGCMsgMemCachedSet = 202,
    k_EGCMsgMemCachedDelete = 203,
    k_EGCMsgMemCachedStats = 204,
    k_EGCMsgMemCachedStatsResponse = 205,
    k_EGCMsgSQLStats = 210,
    k_EGCMsgSQLStatsResponse = 211,
    k_EGCMsgMasterSetDirectory = 220,
    k_EGCMsgMasterSetDirectoryResponse = 221,
    k_EGCMsgMasterSetWebAPIRouting = 222,
    k_EGCMsgMasterSetWebAPIRoutingResponse = 223,
    k_EGCMsgMasterSetClientMsgRouting = 224,
    k_EGCMsgMasterSetClientMsgRoutingResponse = 225,
    k_EGCMsgSetOptions = 226,
    k_EGCMsgSetOptionsResponse = 227,
    k_EGCMsgSystemBase2 = 500,
    k_EGCMsgGetPurchaseTrustStatus = 501,
    k_EGCMsgGetPurchaseTrustStatusResponse = 502,
    k_EGCMsgUpdateSession = 503,
    k_EGCMsgGCAccountVacStatusChange = 504,
    k_EGCMsgCheckFriendship = 505,
    k_EGCMsgCheckFriendshipResponse = 506,
    k_EGCMsgGetPartnerAccountLink = 507,
    k_EGCMsgGetPartnerAccountLinkResponse = 508,
    k_EGCMsgVSReportedSuspiciousActivity = 509,
    k_EGCMsgDPPartnerMicroTxns = 512,
    k_EGCMsgDPPartnerMicroTxnsResponse = 513,
    k_EGCMsgGetIPASN = 514,
    k_EGCMsgGetIPASNResponse = 515,
    k_EGCMsgGetAppFriendsList = 516,
    k_EGCMsgGetAppFriendsListResponse = 517,
    k_EGCMsgVacVerificationChange = 518,
    k_EGCMsgAccountPhoneNumberChange = 519,
}

impl ::protobuf::ProtobufEnum for EGCSystemMsg {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EGCSystemMsg> {
        match value {
            0 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgInvalid),
            1 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgMulti),
            10 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGenericReply),
            50 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgSystemBase),
            51 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgAchievementAwarded),
            52 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgConCommand),
            53 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgStartPlaying),
            54 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgStopPlaying),
            55 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgStartGameserver),
            56 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgStopGameserver),
            57 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgWGRequest),
            58 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgWGResponse),
            59 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetUserGameStatsSchema),
            60 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetUserGameStatsSchemaResponse),
            61 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetUserStatsDEPRECATED),
            62 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetUserStatsResponse),
            63 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgAppInfoUpdated),
            64 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgValidateSession),
            65 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgValidateSessionResponse),
            66 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgLookupAccountFromInput),
            67 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgSendHTTPRequest),
            68 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgSendHTTPRequestResponse),
            69 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgPreTestSetup),
            70 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgRecordSupportAction),
            71 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetAccountDetails_DEPRECATED),
            73 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgReceiveInterAppMessage),
            74 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgFindAccounts),
            75 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgPostAlert),
            76 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetLicenses),
            77 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetUserStats),
            78 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetCommands),
            79 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetCommandsResponse),
            80 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgAddFreeLicense),
            81 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgAddFreeLicenseResponse),
            82 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetIPLocation),
            83 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetIPLocationResponse),
            84 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgSystemStatsSchema),
            85 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetSystemStats),
            86 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetSystemStatsResponse),
            87 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgSendEmail),
            88 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgSendEmailResponse),
            89 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetEmailTemplate),
            90 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetEmailTemplateResponse),
            91 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGrantGuestPass),
            92 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGrantGuestPassResponse),
            93 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetAccountDetails),
            94 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetAccountDetailsResponse),
            95 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetPersonaNames),
            96 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetPersonaNamesResponse),
            97 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgMultiplexMsg),
            101 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgWebAPIRegisterInterfaces),
            102 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgWebAPIJobRequest),
            104 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgWebAPIJobRequestHttpResponse),
            105 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgWebAPIJobRequestForwardResponse),
            200 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgMemCachedGet),
            201 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgMemCachedGetResponse),
            202 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgMemCachedSet),
            203 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgMemCachedDelete),
            204 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgMemCachedStats),
            205 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgMemCachedStatsResponse),
            210 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgSQLStats),
            211 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgSQLStatsResponse),
            220 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgMasterSetDirectory),
            221 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgMasterSetDirectoryResponse),
            222 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgMasterSetWebAPIRouting),
            223 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgMasterSetWebAPIRoutingResponse),
            224 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgMasterSetClientMsgRouting),
            225 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgMasterSetClientMsgRoutingResponse),
            226 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgSetOptions),
            227 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgSetOptionsResponse),
            500 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgSystemBase2),
            501 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetPurchaseTrustStatus),
            502 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetPurchaseTrustStatusResponse),
            503 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgUpdateSession),
            504 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGCAccountVacStatusChange),
            505 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgCheckFriendship),
            506 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgCheckFriendshipResponse),
            507 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetPartnerAccountLink),
            508 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetPartnerAccountLinkResponse),
            509 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgVSReportedSuspiciousActivity),
            512 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgDPPartnerMicroTxns),
            513 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgDPPartnerMicroTxnsResponse),
            514 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetIPASN),
            515 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetIPASNResponse),
            516 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetAppFriendsList),
            517 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgGetAppFriendsListResponse),
            518 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgVacVerificationChange),
            519 => ::std::option::Option::Some(EGCSystemMsg::k_EGCMsgAccountPhoneNumberChange),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EGCSystemMsg] = &[
            EGCSystemMsg::k_EGCMsgInvalid,
            EGCSystemMsg::k_EGCMsgMulti,
            EGCSystemMsg::k_EGCMsgGenericReply,
            EGCSystemMsg::k_EGCMsgSystemBase,
            EGCSystemMsg::k_EGCMsgAchievementAwarded,
            EGCSystemMsg::k_EGCMsgConCommand,
            EGCSystemMsg::k_EGCMsgStartPlaying,
            EGCSystemMsg::k_EGCMsgStopPlaying,
            EGCSystemMsg::k_EGCMsgStartGameserver,
            EGCSystemMsg::k_EGCMsgStopGameserver,
            EGCSystemMsg::k_EGCMsgWGRequest,
            EGCSystemMsg::k_EGCMsgWGResponse,
            EGCSystemMsg::k_EGCMsgGetUserGameStatsSchema,
            EGCSystemMsg::k_EGCMsgGetUserGameStatsSchemaResponse,
            EGCSystemMsg::k_EGCMsgGetUserStatsDEPRECATED,
            EGCSystemMsg::k_EGCMsgGetUserStatsResponse,
            EGCSystemMsg::k_EGCMsgAppInfoUpdated,
            EGCSystemMsg::k_EGCMsgValidateSession,
            EGCSystemMsg::k_EGCMsgValidateSessionResponse,
            EGCSystemMsg::k_EGCMsgLookupAccountFromInput,
            EGCSystemMsg::k_EGCMsgSendHTTPRequest,
            EGCSystemMsg::k_EGCMsgSendHTTPRequestResponse,
            EGCSystemMsg::k_EGCMsgPreTestSetup,
            EGCSystemMsg::k_EGCMsgRecordSupportAction,
            EGCSystemMsg::k_EGCMsgGetAccountDetails_DEPRECATED,
            EGCSystemMsg::k_EGCMsgReceiveInterAppMessage,
            EGCSystemMsg::k_EGCMsgFindAccounts,
            EGCSystemMsg::k_EGCMsgPostAlert,
            EGCSystemMsg::k_EGCMsgGetLicenses,
            EGCSystemMsg::k_EGCMsgGetUserStats,
            EGCSystemMsg::k_EGCMsgGetCommands,
            EGCSystemMsg::k_EGCMsgGetCommandsResponse,
            EGCSystemMsg::k_EGCMsgAddFreeLicense,
            EGCSystemMsg::k_EGCMsgAddFreeLicenseResponse,
            EGCSystemMsg::k_EGCMsgGetIPLocation,
            EGCSystemMsg::k_EGCMsgGetIPLocationResponse,
            EGCSystemMsg::k_EGCMsgSystemStatsSchema,
            EGCSystemMsg::k_EGCMsgGetSystemStats,
            EGCSystemMsg::k_EGCMsgGetSystemStatsResponse,
            EGCSystemMsg::k_EGCMsgSendEmail,
            EGCSystemMsg::k_EGCMsgSendEmailResponse,
            EGCSystemMsg::k_EGCMsgGetEmailTemplate,
            EGCSystemMsg::k_EGCMsgGetEmailTemplateResponse,
            EGCSystemMsg::k_EGCMsgGrantGuestPass,
            EGCSystemMsg::k_EGCMsgGrantGuestPassResponse,
            EGCSystemMsg::k_EGCMsgGetAccountDetails,
            EGCSystemMsg::k_EGCMsgGetAccountDetailsResponse,
            EGCSystemMsg::k_EGCMsgGetPersonaNames,
            EGCSystemMsg::k_EGCMsgGetPersonaNamesResponse,
            EGCSystemMsg::k_EGCMsgMultiplexMsg,
            EGCSystemMsg::k_EGCMsgWebAPIRegisterInterfaces,
            EGCSystemMsg::k_EGCMsgWebAPIJobRequest,
            EGCSystemMsg::k_EGCMsgWebAPIJobRequestHttpResponse,
            EGCSystemMsg::k_EGCMsgWebAPIJobRequestForwardResponse,
            EGCSystemMsg::k_EGCMsgMemCachedGet,
            EGCSystemMsg::k_EGCMsgMemCachedGetResponse,
            EGCSystemMsg::k_EGCMsgMemCachedSet,
            EGCSystemMsg::k_EGCMsgMemCachedDelete,
            EGCSystemMsg::k_EGCMsgMemCachedStats,
            EGCSystemMsg::k_EGCMsgMemCachedStatsResponse,
            EGCSystemMsg::k_EGCMsgSQLStats,
            EGCSystemMsg::k_EGCMsgSQLStatsResponse,
            EGCSystemMsg::k_EGCMsgMasterSetDirectory,
            EGCSystemMsg::k_EGCMsgMasterSetDirectoryResponse,
            EGCSystemMsg::k_EGCMsgMasterSetWebAPIRouting,
            EGCSystemMsg::k_EGCMsgMasterSetWebAPIRoutingResponse,
            EGCSystemMsg::k_EGCMsgMasterSetClientMsgRouting,
            EGCSystemMsg::k_EGCMsgMasterSetClientMsgRoutingResponse,
            EGCSystemMsg::k_EGCMsgSetOptions,
            EGCSystemMsg::k_EGCMsgSetOptionsResponse,
            EGCSystemMsg::k_EGCMsgSystemBase2,
            EGCSystemMsg::k_EGCMsgGetPurchaseTrustStatus,
            EGCSystemMsg::k_EGCMsgGetPurchaseTrustStatusResponse,
            EGCSystemMsg::k_EGCMsgUpdateSession,
            EGCSystemMsg::k_EGCMsgGCAccountVacStatusChange,
            EGCSystemMsg::k_EGCMsgCheckFriendship,
            EGCSystemMsg::k_EGCMsgCheckFriendshipResponse,
            EGCSystemMsg::k_EGCMsgGetPartnerAccountLink,
            EGCSystemMsg::k_EGCMsgGetPartnerAccountLinkResponse,
            EGCSystemMsg::k_EGCMsgVSReportedSuspiciousActivity,
            EGCSystemMsg::k_EGCMsgDPPartnerMicroTxns,
            EGCSystemMsg::k_EGCMsgDPPartnerMicroTxnsResponse,
            EGCSystemMsg::k_EGCMsgGetIPASN,
            EGCSystemMsg::k_EGCMsgGetIPASNResponse,
            EGCSystemMsg::k_EGCMsgGetAppFriendsList,
            EGCSystemMsg::k_EGCMsgGetAppFriendsListResponse,
            EGCSystemMsg::k_EGCMsgVacVerificationChange,
            EGCSystemMsg::k_EGCMsgAccountPhoneNumberChange,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EGCSystemMsg>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EGCSystemMsg", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EGCSystemMsg {
}

impl ::protobuf::reflect::ProtobufValue for EGCSystemMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ESOMsg {
    k_ESOMsg_Create = 21,
    k_ESOMsg_Update = 22,
    k_ESOMsg_Destroy = 23,
    k_ESOMsg_CacheSubscribed = 24,
    k_ESOMsg_CacheUnsubscribed = 25,
    k_ESOMsg_UpdateMultiple = 26,
    k_ESOMsg_CacheSubscriptionRefresh = 28,
    k_ESOMsg_CacheSubscribedUpToDate = 29,
}

impl ::protobuf::ProtobufEnum for ESOMsg {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ESOMsg> {
        match value {
            21 => ::std::option::Option::Some(ESOMsg::k_ESOMsg_Create),
            22 => ::std::option::Option::Some(ESOMsg::k_ESOMsg_Update),
            23 => ::std::option::Option::Some(ESOMsg::k_ESOMsg_Destroy),
            24 => ::std::option::Option::Some(ESOMsg::k_ESOMsg_CacheSubscribed),
            25 => ::std::option::Option::Some(ESOMsg::k_ESOMsg_CacheUnsubscribed),
            26 => ::std::option::Option::Some(ESOMsg::k_ESOMsg_UpdateMultiple),
            28 => ::std::option::Option::Some(ESOMsg::k_ESOMsg_CacheSubscriptionRefresh),
            29 => ::std::option::Option::Some(ESOMsg::k_ESOMsg_CacheSubscribedUpToDate),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ESOMsg] = &[
            ESOMsg::k_ESOMsg_Create,
            ESOMsg::k_ESOMsg_Update,
            ESOMsg::k_ESOMsg_Destroy,
            ESOMsg::k_ESOMsg_CacheSubscribed,
            ESOMsg::k_ESOMsg_CacheUnsubscribed,
            ESOMsg::k_ESOMsg_UpdateMultiple,
            ESOMsg::k_ESOMsg_CacheSubscriptionRefresh,
            ESOMsg::k_ESOMsg_CacheSubscribedUpToDate,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ESOMsg>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ESOMsg", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ESOMsg {
}

impl ::protobuf::reflect::ProtobufValue for ESOMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EGCBaseClientMsg {
    k_EMsgGCPingRequest = 3001,
    k_EMsgGCPingResponse = 3002,
    k_EMsgGCToClientPollConvarRequest = 3003,
    k_EMsgGCToClientPollConvarResponse = 3004,
    k_EMsgGCClientWelcome = 4004,
    k_EMsgGCServerWelcome = 4005,
    k_EMsgGCClientHello = 4006,
    k_EMsgGCServerHello = 4007,
    k_EMsgGCClientConnectionStatus = 4009,
    k_EMsgGCServerConnectionStatus = 4010,
}

impl ::protobuf::ProtobufEnum for EGCBaseClientMsg {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EGCBaseClientMsg> {
        match value {
            3001 => ::std::option::Option::Some(EGCBaseClientMsg::k_EMsgGCPingRequest),
            3002 => ::std::option::Option::Some(EGCBaseClientMsg::k_EMsgGCPingResponse),
            3003 => ::std::option::Option::Some(EGCBaseClientMsg::k_EMsgGCToClientPollConvarRequest),
            3004 => ::std::option::Option::Some(EGCBaseClientMsg::k_EMsgGCToClientPollConvarResponse),
            4004 => ::std::option::Option::Some(EGCBaseClientMsg::k_EMsgGCClientWelcome),
            4005 => ::std::option::Option::Some(EGCBaseClientMsg::k_EMsgGCServerWelcome),
            4006 => ::std::option::Option::Some(EGCBaseClientMsg::k_EMsgGCClientHello),
            4007 => ::std::option::Option::Some(EGCBaseClientMsg::k_EMsgGCServerHello),
            4009 => ::std::option::Option::Some(EGCBaseClientMsg::k_EMsgGCClientConnectionStatus),
            4010 => ::std::option::Option::Some(EGCBaseClientMsg::k_EMsgGCServerConnectionStatus),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EGCBaseClientMsg] = &[
            EGCBaseClientMsg::k_EMsgGCPingRequest,
            EGCBaseClientMsg::k_EMsgGCPingResponse,
            EGCBaseClientMsg::k_EMsgGCToClientPollConvarRequest,
            EGCBaseClientMsg::k_EMsgGCToClientPollConvarResponse,
            EGCBaseClientMsg::k_EMsgGCClientWelcome,
            EGCBaseClientMsg::k_EMsgGCServerWelcome,
            EGCBaseClientMsg::k_EMsgGCClientHello,
            EGCBaseClientMsg::k_EMsgGCServerHello,
            EGCBaseClientMsg::k_EMsgGCClientConnectionStatus,
            EGCBaseClientMsg::k_EMsgGCServerConnectionStatus,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EGCBaseClientMsg>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EGCBaseClientMsg", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EGCBaseClientMsg {
}

impl ::protobuf::reflect::ProtobufValue for EGCBaseClientMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EGCToGCMsg {
    k_EGCToGCMsgMasterAck = 150,
    k_EGCToGCMsgMasterAckResponse = 151,
    k_EGCToGCMsgRouted = 152,
    k_EGCToGCMsgRoutedReply = 153,
    k_EMsgGCUpdateSubGCSessionInfo = 154,
    k_EMsgGCRequestSubGCSessionInfo = 155,
    k_EMsgGCRequestSubGCSessionInfoResponse = 156,
    k_EGCToGCMsgMasterStartupComplete = 157,
    k_EMsgGCToGCSOCacheSubscribe = 158,
    k_EMsgGCToGCSOCacheUnsubscribe = 159,
    k_EMsgGCToGCLoadSessionSOCache = 160,
    k_EMsgGCToGCLoadSessionSOCacheResponse = 161,
    k_EMsgGCToGCUpdateSessionStats = 162,
    k_EMsgGCToGCUniverseStartup = 163,
    k_EMsgGCToGCUniverseStartupResponse = 164,
    k_EMsgGCToGCForwardAccountDetails = 165,
}

impl ::protobuf::ProtobufEnum for EGCToGCMsg {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EGCToGCMsg> {
        match value {
            150 => ::std::option::Option::Some(EGCToGCMsg::k_EGCToGCMsgMasterAck),
            151 => ::std::option::Option::Some(EGCToGCMsg::k_EGCToGCMsgMasterAckResponse),
            152 => ::std::option::Option::Some(EGCToGCMsg::k_EGCToGCMsgRouted),
            153 => ::std::option::Option::Some(EGCToGCMsg::k_EGCToGCMsgRoutedReply),
            154 => ::std::option::Option::Some(EGCToGCMsg::k_EMsgGCUpdateSubGCSessionInfo),
            155 => ::std::option::Option::Some(EGCToGCMsg::k_EMsgGCRequestSubGCSessionInfo),
            156 => ::std::option::Option::Some(EGCToGCMsg::k_EMsgGCRequestSubGCSessionInfoResponse),
            157 => ::std::option::Option::Some(EGCToGCMsg::k_EGCToGCMsgMasterStartupComplete),
            158 => ::std::option::Option::Some(EGCToGCMsg::k_EMsgGCToGCSOCacheSubscribe),
            159 => ::std::option::Option::Some(EGCToGCMsg::k_EMsgGCToGCSOCacheUnsubscribe),
            160 => ::std::option::Option::Some(EGCToGCMsg::k_EMsgGCToGCLoadSessionSOCache),
            161 => ::std::option::Option::Some(EGCToGCMsg::k_EMsgGCToGCLoadSessionSOCacheResponse),
            162 => ::std::option::Option::Some(EGCToGCMsg::k_EMsgGCToGCUpdateSessionStats),
            163 => ::std::option::Option::Some(EGCToGCMsg::k_EMsgGCToGCUniverseStartup),
            164 => ::std::option::Option::Some(EGCToGCMsg::k_EMsgGCToGCUniverseStartupResponse),
            165 => ::std::option::Option::Some(EGCToGCMsg::k_EMsgGCToGCForwardAccountDetails),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EGCToGCMsg] = &[
            EGCToGCMsg::k_EGCToGCMsgMasterAck,
            EGCToGCMsg::k_EGCToGCMsgMasterAckResponse,
            EGCToGCMsg::k_EGCToGCMsgRouted,
            EGCToGCMsg::k_EGCToGCMsgRoutedReply,
            EGCToGCMsg::k_EMsgGCUpdateSubGCSessionInfo,
            EGCToGCMsg::k_EMsgGCRequestSubGCSessionInfo,
            EGCToGCMsg::k_EMsgGCRequestSubGCSessionInfoResponse,
            EGCToGCMsg::k_EGCToGCMsgMasterStartupComplete,
            EGCToGCMsg::k_EMsgGCToGCSOCacheSubscribe,
            EGCToGCMsg::k_EMsgGCToGCSOCacheUnsubscribe,
            EGCToGCMsg::k_EMsgGCToGCLoadSessionSOCache,
            EGCToGCMsg::k_EMsgGCToGCLoadSessionSOCacheResponse,
            EGCToGCMsg::k_EMsgGCToGCUpdateSessionStats,
            EGCToGCMsg::k_EMsgGCToGCUniverseStartup,
            EGCToGCMsg::k_EMsgGCToGCUniverseStartupResponse,
            EGCToGCMsg::k_EMsgGCToGCForwardAccountDetails,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EGCToGCMsg>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EGCToGCMsg", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EGCToGCMsg {
}

impl ::protobuf::reflect::ProtobufValue for EGCToGCMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12gcsystemmsgs.proto*\xae\x16\n\x0cEGCSystemMsg\x12\x13\n\x0fk_EGCMs\
    gInvalid\x10\0\x12\x11\n\rk_EGCMsgMulti\x10\x01\x12\x18\n\x14k_EGCMsgGen\
    ericReply\x10\n\x12\x16\n\x12k_EGCMsgSystemBase\x102\x12\x1e\n\x1ak_EGCM\
    sgAchievementAwarded\x103\x12\x16\n\x12k_EGCMsgConCommand\x104\x12\x18\n\
    \x14k_EGCMsgStartPlaying\x105\x12\x17\n\x13k_EGCMsgStopPlaying\x106\x12\
    \x1b\n\x17k_EGCMsgStartGameserver\x107\x12\x1a\n\x16k_EGCMsgStopGameserv\
    er\x108\x12\x15\n\x11k_EGCMsgWGRequest\x109\x12\x16\n\x12k_EGCMsgWGRespo\
    nse\x10:\x12\"\n\x1ek_EGCMsgGetUserGameStatsSchema\x10;\x12*\n&k_EGCMsgG\
    etUserGameStatsSchemaResponse\x10<\x12\"\n\x1ek_EGCMsgGetUserStatsDEPREC\
    ATED\x10=\x12\x20\n\x1ck_EGCMsgGetUserStatsResponse\x10>\x12\x1a\n\x16k_\
    EGCMsgAppInfoUpdated\x10?\x12\x1b\n\x17k_EGCMsgValidateSession\x10@\x12#\
    \n\x1fk_EGCMsgValidateSessionResponse\x10A\x12\"\n\x1ek_EGCMsgLookupAcco\
    untFromInput\x10B\x12\x1b\n\x17k_EGCMsgSendHTTPRequest\x10C\x12#\n\x1fk_\
    EGCMsgSendHTTPRequestResponse\x10D\x12\x18\n\x14k_EGCMsgPreTestSetup\x10\
    E\x12\x1f\n\x1bk_EGCMsgRecordSupportAction\x10F\x12(\n$k_EGCMsgGetAccoun\
    tDetails_DEPRECATED\x10G\x12\"\n\x1ek_EGCMsgReceiveInterAppMessage\x10I\
    \x12\x18\n\x14k_EGCMsgFindAccounts\x10J\x12\x15\n\x11k_EGCMsgPostAlert\
    \x10K\x12\x17\n\x13k_EGCMsgGetLicenses\x10L\x12\x18\n\x14k_EGCMsgGetUser\
    Stats\x10M\x12\x17\n\x13k_EGCMsgGetCommands\x10N\x12\x1f\n\x1bk_EGCMsgGe\
    tCommandsResponse\x10O\x12\x1a\n\x16k_EGCMsgAddFreeLicense\x10P\x12\"\n\
    \x1ek_EGCMsgAddFreeLicenseResponse\x10Q\x12\x19\n\x15k_EGCMsgGetIPLocati\
    on\x10R\x12!\n\x1dk_EGCMsgGetIPLocationResponse\x10S\x12\x1d\n\x19k_EGCM\
    sgSystemStatsSchema\x10T\x12\x1a\n\x16k_EGCMsgGetSystemStats\x10U\x12\"\
    \n\x1ek_EGCMsgGetSystemStatsResponse\x10V\x12\x15\n\x11k_EGCMsgSendEmail\
    \x10W\x12\x1d\n\x19k_EGCMsgSendEmailResponse\x10X\x12\x1c\n\x18k_EGCMsgG\
    etEmailTemplate\x10Y\x12$\n\x20k_EGCMsgGetEmailTemplateResponse\x10Z\x12\
    \x1a\n\x16k_EGCMsgGrantGuestPass\x10[\x12\"\n\x1ek_EGCMsgGrantGuestPassR\
    esponse\x10\\\x12\x1d\n\x19k_EGCMsgGetAccountDetails\x10]\x12%\n!k_EGCMs\
    gGetAccountDetailsResponse\x10^\x12\x1b\n\x17k_EGCMsgGetPersonaNames\x10\
    _\x12#\n\x1fk_EGCMsgGetPersonaNamesResponse\x10`\x12\x18\n\x14k_EGCMsgMu\
    ltiplexMsg\x10a\x12$\n\x20k_EGCMsgWebAPIRegisterInterfaces\x10e\x12\x1c\
    \n\x18k_EGCMsgWebAPIJobRequest\x10f\x12(\n$k_EGCMsgWebAPIJobRequestHttpR\
    esponse\x10h\x12+\n'k_EGCMsgWebAPIJobRequestForwardResponse\x10i\x12\x19\
    \n\x14k_EGCMsgMemCachedGet\x10\xc8\x01\x12!\n\x1ck_EGCMsgMemCachedGetRes\
    ponse\x10\xc9\x01\x12\x19\n\x14k_EGCMsgMemCachedSet\x10\xca\x01\x12\x1c\
    \n\x17k_EGCMsgMemCachedDelete\x10\xcb\x01\x12\x1b\n\x16k_EGCMsgMemCached\
    Stats\x10\xcc\x01\x12#\n\x1ek_EGCMsgMemCachedStatsResponse\x10\xcd\x01\
    \x12\x15\n\x10k_EGCMsgSQLStats\x10\xd2\x01\x12\x1d\n\x18k_EGCMsgSQLStats\
    Response\x10\xd3\x01\x12\x1f\n\x1ak_EGCMsgMasterSetDirectory\x10\xdc\x01\
    \x12'\n\"k_EGCMsgMasterSetDirectoryResponse\x10\xdd\x01\x12#\n\x1ek_EGCM\
    sgMasterSetWebAPIRouting\x10\xde\x01\x12+\n&k_EGCMsgMasterSetWebAPIRouti\
    ngResponse\x10\xdf\x01\x12&\n!k_EGCMsgMasterSetClientMsgRouting\x10\xe0\
    \x01\x12.\n)k_EGCMsgMasterSetClientMsgRoutingResponse\x10\xe1\x01\x12\
    \x17\n\x12k_EGCMsgSetOptions\x10\xe2\x01\x12\x1f\n\x1ak_EGCMsgSetOptions\
    Response\x10\xe3\x01\x12\x18\n\x13k_EGCMsgSystemBase2\x10\xf4\x03\x12#\n\
    \x1ek_EGCMsgGetPurchaseTrustStatus\x10\xf5\x03\x12+\n&k_EGCMsgGetPurchas\
    eTrustStatusResponse\x10\xf6\x03\x12\x1a\n\x15k_EGCMsgUpdateSession\x10\
    \xf7\x03\x12%\n\x20k_EGCMsgGCAccountVacStatusChange\x10\xf8\x03\x12\x1c\
    \n\x17k_EGCMsgCheckFriendship\x10\xf9\x03\x12$\n\x1fk_EGCMsgCheckFriends\
    hipResponse\x10\xfa\x03\x12\"\n\x1dk_EGCMsgGetPartnerAccountLink\x10\xfb\
    \x03\x12*\n%k_EGCMsgGetPartnerAccountLinkResponse\x10\xfc\x03\x12)\n$k_E\
    GCMsgVSReportedSuspiciousActivity\x10\xfd\x03\x12\x1f\n\x1ak_EGCMsgDPPar\
    tnerMicroTxns\x10\x80\x04\x12'\n\"k_EGCMsgDPPartnerMicroTxnsResponse\x10\
    \x81\x04\x12\x15\n\x10k_EGCMsgGetIPASN\x10\x82\x04\x12\x1d\n\x18k_EGCMsg\
    GetIPASNResponse\x10\x83\x04\x12\x1e\n\x19k_EGCMsgGetAppFriendsList\x10\
    \x84\x04\x12&\n!k_EGCMsgGetAppFriendsListResponse\x10\x85\x04\x12\"\n\
    \x1dk_EGCMsgVacVerificationChange\x10\x86\x04\x12%\n\x20k_EGCMsgAccountP\
    honeNumberChange\x10\x87\x04*\xf0\x01\n\x06ESOMsg\x12\x13\n\x0fk_ESOMsg_\
    Create\x10\x15\x12\x13\n\x0fk_ESOMsg_Update\x10\x16\x12\x14\n\x10k_ESOMs\
    g_Destroy\x10\x17\x12\x1c\n\x18k_ESOMsg_CacheSubscribed\x10\x18\x12\x1e\
    \n\x1ak_ESOMsg_CacheUnsubscribed\x10\x19\x12\x1b\n\x17k_ESOMsg_UpdateMul\
    tiple\x10\x1a\x12%\n!k_ESOMsg_CacheSubscriptionRefresh\x10\x1c\x12$\n\
    \x20k_ESOMsg_CacheSubscribedUpToDate\x10\x1d*\xce\x02\n\x10EGCBaseClient\
    Msg\x12\x18\n\x13k_EMsgGCPingRequest\x10\xb9\x17\x12\x19\n\x14k_EMsgGCPi\
    ngResponse\x10\xba\x17\x12&\n!k_EMsgGCToClientPollConvarRequest\x10\xbb\
    \x17\x12'\n\"k_EMsgGCToClientPollConvarResponse\x10\xbc\x17\x12\x1a\n\
    \x15k_EMsgGCClientWelcome\x10\xa4\x1f\x12\x1a\n\x15k_EMsgGCServerWelcome\
    \x10\xa5\x1f\x12\x18\n\x13k_EMsgGCClientHello\x10\xa6\x1f\x12\x18\n\x13k\
    _EMsgGCServerHello\x10\xa7\x1f\x12#\n\x1ek_EMsgGCClientConnectionStatus\
    \x10\xa9\x1f\x12#\n\x1ek_EMsgGCServerConnectionStatus\x10\xaa\x1f*\xd7\
    \x04\n\nEGCToGCMsg\x12\x1a\n\x15k_EGCToGCMsgMasterAck\x10\x96\x01\x12\"\
    \n\x1dk_EGCToGCMsgMasterAckResponse\x10\x97\x01\x12\x17\n\x12k_EGCToGCMs\
    gRouted\x10\x98\x01\x12\x1c\n\x17k_EGCToGCMsgRoutedReply\x10\x99\x01\x12\
    #\n\x1ek_EMsgGCUpdateSubGCSessionInfo\x10\x9a\x01\x12$\n\x1fk_EMsgGCRequ\
    estSubGCSessionInfo\x10\x9b\x01\x12,\n'k_EMsgGCRequestSubGCSessionInfoRe\
    sponse\x10\x9c\x01\x12&\n!k_EGCToGCMsgMasterStartupComplete\x10\x9d\x01\
    \x12!\n\x1ck_EMsgGCToGCSOCacheSubscribe\x10\x9e\x01\x12#\n\x1ek_EMsgGCTo\
    GCSOCacheUnsubscribe\x10\x9f\x01\x12#\n\x1ek_EMsgGCToGCLoadSessionSOCach\
    e\x10\xa0\x01\x12+\n&k_EMsgGCToGCLoadSessionSOCacheResponse\x10\xa1\x01\
    \x12#\n\x1ek_EMsgGCToGCUpdateSessionStats\x10\xa2\x01\x12\x20\n\x1bk_EMs\
    gGCToGCUniverseStartup\x10\xa3\x01\x12(\n#k_EMsgGCToGCUniverseStartupRes\
    ponse\x10\xa4\x01\x12&\n!k_EMsgGCToGCForwardAccountDetails\x10\xa5\x01B\
    \x05H\x01\x80\x01\0\
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
