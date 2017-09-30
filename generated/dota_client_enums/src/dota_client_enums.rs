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
pub enum ETournamentTemplate {
    k_ETournamentTemplate_None = 0,
    k_ETournamentTemplate_AutomatedWin3 = 1,
}

impl ::protobuf::ProtobufEnum for ETournamentTemplate {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ETournamentTemplate> {
        match value {
            0 => ::std::option::Option::Some(ETournamentTemplate::k_ETournamentTemplate_None),
            1 => ::std::option::Option::Some(ETournamentTemplate::k_ETournamentTemplate_AutomatedWin3),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ETournamentTemplate] = &[
            ETournamentTemplate::k_ETournamentTemplate_None,
            ETournamentTemplate::k_ETournamentTemplate_AutomatedWin3,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ETournamentTemplate>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ETournamentTemplate", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ETournamentTemplate {
}

impl ::protobuf::reflect::ProtobufValue for ETournamentTemplate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ETournamentGameState {
    k_ETournamentGameState_Unknown = 0,
    k_ETournamentGameState_Canceled = 1,
    k_ETournamentGameState_Scheduled = 2,
    k_ETournamentGameState_Active = 3,
    k_ETournamentGameState_RadVictory = 20,
    k_ETournamentGameState_DireVictory = 21,
    k_ETournamentGameState_RadVictoryByForfeit = 22,
    k_ETournamentGameState_DireVictoryByForfeit = 23,
    k_ETournamentGameState_ServerFailure = 40,
    k_ETournamentGameState_NotNeeded = 41,
}

impl ::protobuf::ProtobufEnum for ETournamentGameState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ETournamentGameState> {
        match value {
            0 => ::std::option::Option::Some(ETournamentGameState::k_ETournamentGameState_Unknown),
            1 => ::std::option::Option::Some(ETournamentGameState::k_ETournamentGameState_Canceled),
            2 => ::std::option::Option::Some(ETournamentGameState::k_ETournamentGameState_Scheduled),
            3 => ::std::option::Option::Some(ETournamentGameState::k_ETournamentGameState_Active),
            20 => ::std::option::Option::Some(ETournamentGameState::k_ETournamentGameState_RadVictory),
            21 => ::std::option::Option::Some(ETournamentGameState::k_ETournamentGameState_DireVictory),
            22 => ::std::option::Option::Some(ETournamentGameState::k_ETournamentGameState_RadVictoryByForfeit),
            23 => ::std::option::Option::Some(ETournamentGameState::k_ETournamentGameState_DireVictoryByForfeit),
            40 => ::std::option::Option::Some(ETournamentGameState::k_ETournamentGameState_ServerFailure),
            41 => ::std::option::Option::Some(ETournamentGameState::k_ETournamentGameState_NotNeeded),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ETournamentGameState] = &[
            ETournamentGameState::k_ETournamentGameState_Unknown,
            ETournamentGameState::k_ETournamentGameState_Canceled,
            ETournamentGameState::k_ETournamentGameState_Scheduled,
            ETournamentGameState::k_ETournamentGameState_Active,
            ETournamentGameState::k_ETournamentGameState_RadVictory,
            ETournamentGameState::k_ETournamentGameState_DireVictory,
            ETournamentGameState::k_ETournamentGameState_RadVictoryByForfeit,
            ETournamentGameState::k_ETournamentGameState_DireVictoryByForfeit,
            ETournamentGameState::k_ETournamentGameState_ServerFailure,
            ETournamentGameState::k_ETournamentGameState_NotNeeded,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ETournamentGameState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ETournamentGameState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ETournamentGameState {
}

impl ::protobuf::reflect::ProtobufValue for ETournamentGameState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ETournamentTeamState {
    k_ETournamentTeamState_Unknown = 0,
    k_ETournamentTeamState_Node1 = 1,
    k_ETournamentTeamState_NodeMax = 1024,
    k_ETournamentTeamState_Eliminated = 14003,
    k_ETournamentTeamState_Forfeited = 14004,
    k_ETournamentTeamState_Finished1st = 15001,
    k_ETournamentTeamState_Finished2nd = 15002,
    k_ETournamentTeamState_Finished3rd = 15003,
    k_ETournamentTeamState_Finished4th = 15004,
    k_ETournamentTeamState_Finished5th = 15005,
    k_ETournamentTeamState_Finished6th = 15006,
    k_ETournamentTeamState_Finished7th = 15007,
    k_ETournamentTeamState_Finished8th = 15008,
    k_ETournamentTeamState_Finished9th = 15009,
    k_ETournamentTeamState_Finished10th = 15010,
    k_ETournamentTeamState_Finished11th = 15011,
    k_ETournamentTeamState_Finished12th = 15012,
    k_ETournamentTeamState_Finished13th = 15013,
    k_ETournamentTeamState_Finished14th = 15014,
    k_ETournamentTeamState_Finished15th = 15015,
    k_ETournamentTeamState_Finished16th = 15016,
}

impl ::protobuf::ProtobufEnum for ETournamentTeamState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ETournamentTeamState> {
        match value {
            0 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Unknown),
            1 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Node1),
            1024 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_NodeMax),
            14003 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Eliminated),
            14004 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Forfeited),
            15001 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished1st),
            15002 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished2nd),
            15003 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished3rd),
            15004 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished4th),
            15005 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished5th),
            15006 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished6th),
            15007 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished7th),
            15008 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished8th),
            15009 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished9th),
            15010 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished10th),
            15011 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished11th),
            15012 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished12th),
            15013 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished13th),
            15014 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished14th),
            15015 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished15th),
            15016 => ::std::option::Option::Some(ETournamentTeamState::k_ETournamentTeamState_Finished16th),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ETournamentTeamState] = &[
            ETournamentTeamState::k_ETournamentTeamState_Unknown,
            ETournamentTeamState::k_ETournamentTeamState_Node1,
            ETournamentTeamState::k_ETournamentTeamState_NodeMax,
            ETournamentTeamState::k_ETournamentTeamState_Eliminated,
            ETournamentTeamState::k_ETournamentTeamState_Forfeited,
            ETournamentTeamState::k_ETournamentTeamState_Finished1st,
            ETournamentTeamState::k_ETournamentTeamState_Finished2nd,
            ETournamentTeamState::k_ETournamentTeamState_Finished3rd,
            ETournamentTeamState::k_ETournamentTeamState_Finished4th,
            ETournamentTeamState::k_ETournamentTeamState_Finished5th,
            ETournamentTeamState::k_ETournamentTeamState_Finished6th,
            ETournamentTeamState::k_ETournamentTeamState_Finished7th,
            ETournamentTeamState::k_ETournamentTeamState_Finished8th,
            ETournamentTeamState::k_ETournamentTeamState_Finished9th,
            ETournamentTeamState::k_ETournamentTeamState_Finished10th,
            ETournamentTeamState::k_ETournamentTeamState_Finished11th,
            ETournamentTeamState::k_ETournamentTeamState_Finished12th,
            ETournamentTeamState::k_ETournamentTeamState_Finished13th,
            ETournamentTeamState::k_ETournamentTeamState_Finished14th,
            ETournamentTeamState::k_ETournamentTeamState_Finished15th,
            ETournamentTeamState::k_ETournamentTeamState_Finished16th,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ETournamentTeamState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ETournamentTeamState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ETournamentTeamState {
}

impl ::protobuf::reflect::ProtobufValue for ETournamentTeamState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ETournamentState {
    k_ETournamentState_Unknown = 0,
    k_ETournamentState_CanceledByAdmin = 1,
    k_ETournamentState_Completed = 2,
    k_ETournamentState_Merged = 3,
    k_ETournamentState_ServerFailure = 4,
    k_ETournamentState_TeamAbandoned = 5,
    k_ETournamentState_TeamTimeoutForfeit = 6,
    k_ETournamentState_TeamTimeoutRefund = 7,
    k_ETournamentState_ServerFailureGrantedVictory = 8,
    k_ETournamentState_TeamTimeoutGrantedVictory = 9,
    k_ETournamentState_InProgress = 100,
    k_ETournamentState_WaitingToMerge = 101,
}

impl ::protobuf::ProtobufEnum for ETournamentState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ETournamentState> {
        match value {
            0 => ::std::option::Option::Some(ETournamentState::k_ETournamentState_Unknown),
            1 => ::std::option::Option::Some(ETournamentState::k_ETournamentState_CanceledByAdmin),
            2 => ::std::option::Option::Some(ETournamentState::k_ETournamentState_Completed),
            3 => ::std::option::Option::Some(ETournamentState::k_ETournamentState_Merged),
            4 => ::std::option::Option::Some(ETournamentState::k_ETournamentState_ServerFailure),
            5 => ::std::option::Option::Some(ETournamentState::k_ETournamentState_TeamAbandoned),
            6 => ::std::option::Option::Some(ETournamentState::k_ETournamentState_TeamTimeoutForfeit),
            7 => ::std::option::Option::Some(ETournamentState::k_ETournamentState_TeamTimeoutRefund),
            8 => ::std::option::Option::Some(ETournamentState::k_ETournamentState_ServerFailureGrantedVictory),
            9 => ::std::option::Option::Some(ETournamentState::k_ETournamentState_TeamTimeoutGrantedVictory),
            100 => ::std::option::Option::Some(ETournamentState::k_ETournamentState_InProgress),
            101 => ::std::option::Option::Some(ETournamentState::k_ETournamentState_WaitingToMerge),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ETournamentState] = &[
            ETournamentState::k_ETournamentState_Unknown,
            ETournamentState::k_ETournamentState_CanceledByAdmin,
            ETournamentState::k_ETournamentState_Completed,
            ETournamentState::k_ETournamentState_Merged,
            ETournamentState::k_ETournamentState_ServerFailure,
            ETournamentState::k_ETournamentState_TeamAbandoned,
            ETournamentState::k_ETournamentState_TeamTimeoutForfeit,
            ETournamentState::k_ETournamentState_TeamTimeoutRefund,
            ETournamentState::k_ETournamentState_ServerFailureGrantedVictory,
            ETournamentState::k_ETournamentState_TeamTimeoutGrantedVictory,
            ETournamentState::k_ETournamentState_InProgress,
            ETournamentState::k_ETournamentState_WaitingToMerge,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ETournamentState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ETournamentState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ETournamentState {
}

impl ::protobuf::reflect::ProtobufValue for ETournamentState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ETournamentNodeState {
    k_ETournamentNodeState_Unknown = 0,
    k_ETournamentNodeState_Canceled = 1,
    k_ETournamentNodeState_TeamsNotYetAssigned = 2,
    k_ETournamentNodeState_InBetweenGames = 3,
    k_ETournamentNodeState_GameInProgress = 4,
    k_ETournamentNodeState_A_Won = 5,
    k_ETournamentNodeState_B_Won = 6,
    k_ETournamentNodeState_A_WonByForfeit = 7,
    k_ETournamentNodeState_B_WonByForfeit = 8,
    k_ETournamentNodeState_A_Bye = 9,
    k_ETournamentNodeState_A_Abandoned = 10,
    k_ETournamentNodeState_ServerFailure = 11,
    k_ETournamentNodeState_A_TimeoutForfeit = 12,
    k_ETournamentNodeState_A_TimeoutRefund = 13,
}

impl ::protobuf::ProtobufEnum for ETournamentNodeState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ETournamentNodeState> {
        match value {
            0 => ::std::option::Option::Some(ETournamentNodeState::k_ETournamentNodeState_Unknown),
            1 => ::std::option::Option::Some(ETournamentNodeState::k_ETournamentNodeState_Canceled),
            2 => ::std::option::Option::Some(ETournamentNodeState::k_ETournamentNodeState_TeamsNotYetAssigned),
            3 => ::std::option::Option::Some(ETournamentNodeState::k_ETournamentNodeState_InBetweenGames),
            4 => ::std::option::Option::Some(ETournamentNodeState::k_ETournamentNodeState_GameInProgress),
            5 => ::std::option::Option::Some(ETournamentNodeState::k_ETournamentNodeState_A_Won),
            6 => ::std::option::Option::Some(ETournamentNodeState::k_ETournamentNodeState_B_Won),
            7 => ::std::option::Option::Some(ETournamentNodeState::k_ETournamentNodeState_A_WonByForfeit),
            8 => ::std::option::Option::Some(ETournamentNodeState::k_ETournamentNodeState_B_WonByForfeit),
            9 => ::std::option::Option::Some(ETournamentNodeState::k_ETournamentNodeState_A_Bye),
            10 => ::std::option::Option::Some(ETournamentNodeState::k_ETournamentNodeState_A_Abandoned),
            11 => ::std::option::Option::Some(ETournamentNodeState::k_ETournamentNodeState_ServerFailure),
            12 => ::std::option::Option::Some(ETournamentNodeState::k_ETournamentNodeState_A_TimeoutForfeit),
            13 => ::std::option::Option::Some(ETournamentNodeState::k_ETournamentNodeState_A_TimeoutRefund),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ETournamentNodeState] = &[
            ETournamentNodeState::k_ETournamentNodeState_Unknown,
            ETournamentNodeState::k_ETournamentNodeState_Canceled,
            ETournamentNodeState::k_ETournamentNodeState_TeamsNotYetAssigned,
            ETournamentNodeState::k_ETournamentNodeState_InBetweenGames,
            ETournamentNodeState::k_ETournamentNodeState_GameInProgress,
            ETournamentNodeState::k_ETournamentNodeState_A_Won,
            ETournamentNodeState::k_ETournamentNodeState_B_Won,
            ETournamentNodeState::k_ETournamentNodeState_A_WonByForfeit,
            ETournamentNodeState::k_ETournamentNodeState_B_WonByForfeit,
            ETournamentNodeState::k_ETournamentNodeState_A_Bye,
            ETournamentNodeState::k_ETournamentNodeState_A_Abandoned,
            ETournamentNodeState::k_ETournamentNodeState_ServerFailure,
            ETournamentNodeState::k_ETournamentNodeState_A_TimeoutForfeit,
            ETournamentNodeState::k_ETournamentNodeState_A_TimeoutRefund,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ETournamentNodeState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ETournamentNodeState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ETournamentNodeState {
}

impl ::protobuf::reflect::ProtobufValue for ETournamentNodeState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EDOTAGroupMergeResult {
    k_EDOTAGroupMergeResult_OK = 0,
    k_EDOTAGroupMergeResult_FAILED_GENERIC = 1,
    k_EDOTAGroupMergeResult_NOT_LEADER = 2,
    k_EDOTAGroupMergeResult_TOO_MANY_PLAYERS = 3,
    k_EDOTAGroupMergeResult_TOO_MANY_COACHES = 4,
    k_EDOTAGroupMergeResult_ENGINE_MISMATCH = 5,
    k_EDOTAGroupMergeResult_NO_SUCH_GROUP = 6,
    k_EDOTAGroupMergeResult_OTHER_GROUP_NOT_OPEN = 7,
    k_EDOTAGroupMergeResult_ALREADY_INVITED = 8,
    k_EDOTAGroupMergeResult_NOT_INVITED = 9,
}

impl ::protobuf::ProtobufEnum for EDOTAGroupMergeResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EDOTAGroupMergeResult> {
        match value {
            0 => ::std::option::Option::Some(EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_OK),
            1 => ::std::option::Option::Some(EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_FAILED_GENERIC),
            2 => ::std::option::Option::Some(EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_NOT_LEADER),
            3 => ::std::option::Option::Some(EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_TOO_MANY_PLAYERS),
            4 => ::std::option::Option::Some(EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_TOO_MANY_COACHES),
            5 => ::std::option::Option::Some(EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_ENGINE_MISMATCH),
            6 => ::std::option::Option::Some(EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_NO_SUCH_GROUP),
            7 => ::std::option::Option::Some(EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_OTHER_GROUP_NOT_OPEN),
            8 => ::std::option::Option::Some(EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_ALREADY_INVITED),
            9 => ::std::option::Option::Some(EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_NOT_INVITED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EDOTAGroupMergeResult] = &[
            EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_OK,
            EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_FAILED_GENERIC,
            EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_NOT_LEADER,
            EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_TOO_MANY_PLAYERS,
            EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_TOO_MANY_COACHES,
            EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_ENGINE_MISMATCH,
            EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_NO_SUCH_GROUP,
            EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_OTHER_GROUP_NOT_OPEN,
            EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_ALREADY_INVITED,
            EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_NOT_INVITED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EDOTAGroupMergeResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EDOTAGroupMergeResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EDOTAGroupMergeResult {
}

impl ::protobuf::reflect::ProtobufValue for EDOTAGroupMergeResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17dota_client_enums.proto*^\n\x13ETournamentTemplate\x12\x1e\n\x1ak_\
    ETournamentTemplate_None\x10\0\x12'\n#k_ETournamentTemplate_AutomatedWin\
    3\x10\x01*\xa8\x03\n\x14ETournamentGameState\x12\"\n\x1ek_ETournamentGam\
    eState_Unknown\x10\0\x12#\n\x1fk_ETournamentGameState_Canceled\x10\x01\
    \x12$\n\x20k_ETournamentGameState_Scheduled\x10\x02\x12!\n\x1dk_ETournam\
    entGameState_Active\x10\x03\x12%\n!k_ETournamentGameState_RadVictory\x10\
    \x14\x12&\n\"k_ETournamentGameState_DireVictory\x10\x15\x12.\n*k_ETourna\
    mentGameState_RadVictoryByForfeit\x10\x16\x12/\n+k_ETournamentGameState_\
    DireVictoryByForfeit\x10\x17\x12(\n$k_ETournamentGameState_ServerFailure\
    \x10(\x12$\n\x20k_ETournamentGameState_NotNeeded\x10)*\xe7\x06\n\x14ETou\
    rnamentTeamState\x12\"\n\x1ek_ETournamentTeamState_Unknown\x10\0\x12\x20\
    \n\x1ck_ETournamentTeamState_Node1\x10\x01\x12#\n\x1ek_ETournamentTeamSt\
    ate_NodeMax\x10\x80\x08\x12&\n!k_ETournamentTeamState_Eliminated\x10\xb3\
    m\x12%\n\x20k_ETournamentTeamState_Forfeited\x10\xb4m\x12'\n\"k_ETournam\
    entTeamState_Finished1st\x10\x99u\x12'\n\"k_ETournamentTeamState_Finishe\
    d2nd\x10\x9au\x12'\n\"k_ETournamentTeamState_Finished3rd\x10\x9bu\x12'\n\
    \"k_ETournamentTeamState_Finished4th\x10\x9cu\x12'\n\"k_ETournamentTeamS\
    tate_Finished5th\x10\x9du\x12'\n\"k_ETournamentTeamState_Finished6th\x10\
    \x9eu\x12'\n\"k_ETournamentTeamState_Finished7th\x10\x9fu\x12'\n\"k_ETou\
    rnamentTeamState_Finished8th\x10\xa0u\x12'\n\"k_ETournamentTeamState_Fin\
    ished9th\x10\xa1u\x12(\n#k_ETournamentTeamState_Finished10th\x10\xa2u\
    \x12(\n#k_ETournamentTeamState_Finished11th\x10\xa3u\x12(\n#k_ETournamen\
    tTeamState_Finished12th\x10\xa4u\x12(\n#k_ETournamentTeamState_Finished1\
    3th\x10\xa5u\x12(\n#k_ETournamentTeamState_Finished14th\x10\xa6u\x12(\n#\
    k_ETournamentTeamState_Finished15th\x10\xa7u\x12(\n#k_ETournamentTeamSta\
    te_Finished16th\x10\xa8u*\xec\x03\n\x10ETournamentState\x12\x1e\n\x1ak_E\
    TournamentState_Unknown\x10\0\x12&\n\"k_ETournamentState_CanceledByAdmin\
    \x10\x01\x12\x20\n\x1ck_ETournamentState_Completed\x10\x02\x12\x1d\n\x19\
    k_ETournamentState_Merged\x10\x03\x12$\n\x20k_ETournamentState_ServerFai\
    lure\x10\x04\x12$\n\x20k_ETournamentState_TeamAbandoned\x10\x05\x12)\n%k\
    _ETournamentState_TeamTimeoutForfeit\x10\x06\x12(\n$k_ETournamentState_T\
    eamTimeoutRefund\x10\x07\x122\n.k_ETournamentState_ServerFailureGrantedV\
    ictory\x10\x08\x120\n,k_ETournamentState_TeamTimeoutGrantedVictory\x10\t\
    \x12!\n\x1dk_ETournamentState_InProgress\x10d\x12%\n!k_ETournamentState_\
    WaitingToMerge\x10e*\xcc\x04\n\x14ETournamentNodeState\x12\"\n\x1ek_ETou\
    rnamentNodeState_Unknown\x10\0\x12#\n\x1fk_ETournamentNodeState_Canceled\
    \x10\x01\x12.\n*k_ETournamentNodeState_TeamsNotYetAssigned\x10\x02\x12)\
    \n%k_ETournamentNodeState_InBetweenGames\x10\x03\x12)\n%k_ETournamentNod\
    eState_GameInProgress\x10\x04\x12\x20\n\x1ck_ETournamentNodeState_A_Won\
    \x10\x05\x12\x20\n\x1ck_ETournamentNodeState_B_Won\x10\x06\x12)\n%k_ETou\
    rnamentNodeState_A_WonByForfeit\x10\x07\x12)\n%k_ETournamentNodeState_B_\
    WonByForfeit\x10\x08\x12\x20\n\x1ck_ETournamentNodeState_A_Bye\x10\t\x12\
    &\n\"k_ETournamentNodeState_A_Abandoned\x10\n\x12(\n$k_ETournamentNodeSt\
    ate_ServerFailure\x10\x0b\x12+\n'k_ETournamentNodeState_A_TimeoutForfeit\
    \x10\x0c\x12*\n&k_ETournamentNodeState_A_TimeoutRefund\x10\r*\xc7\x03\n\
    \x15EDOTAGroupMergeResult\x12\x1e\n\x1ak_EDOTAGroupMergeResult_OK\x10\0\
    \x12*\n&k_EDOTAGroupMergeResult_FAILED_GENERIC\x10\x01\x12&\n\"k_EDOTAGr\
    oupMergeResult_NOT_LEADER\x10\x02\x12,\n(k_EDOTAGroupMergeResult_TOO_MAN\
    Y_PLAYERS\x10\x03\x12,\n(k_EDOTAGroupMergeResult_TOO_MANY_COACHES\x10\
    \x04\x12+\n'k_EDOTAGroupMergeResult_ENGINE_MISMATCH\x10\x05\x12)\n%k_EDO\
    TAGroupMergeResult_NO_SUCH_GROUP\x10\x06\x120\n,k_EDOTAGroupMergeResult_\
    OTHER_GROUP_NOT_OPEN\x10\x07\x12+\n'k_EDOTAGroupMergeResult_ALREADY_INVI\
    TED\x10\x08\x12'\n#k_EDOTAGroupMergeResult_NOT_INVITED\x10\tB\x05H\x01\
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
