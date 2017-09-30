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
pub enum ENetworkDisconnectionReason {
    NETWORK_DISCONNECT_INVALID = 0,
    NETWORK_DISCONNECT_SHUTDOWN = 1,
    NETWORK_DISCONNECT_DISCONNECT_BY_USER = 2,
    NETWORK_DISCONNECT_DISCONNECT_BY_SERVER = 3,
    NETWORK_DISCONNECT_LOST = 4,
    NETWORK_DISCONNECT_OVERFLOW = 5,
    NETWORK_DISCONNECT_STEAM_BANNED = 6,
    NETWORK_DISCONNECT_STEAM_INUSE = 7,
    NETWORK_DISCONNECT_STEAM_TICKET = 8,
    NETWORK_DISCONNECT_STEAM_LOGON = 9,
    NETWORK_DISCONNECT_STEAM_AUTHCANCELLED = 10,
    NETWORK_DISCONNECT_STEAM_AUTHALREADYUSED = 11,
    NETWORK_DISCONNECT_STEAM_AUTHINVALID = 12,
    NETWORK_DISCONNECT_STEAM_VACBANSTATE = 13,
    NETWORK_DISCONNECT_STEAM_LOGGED_IN_ELSEWHERE = 14,
    NETWORK_DISCONNECT_STEAM_VAC_CHECK_TIMEDOUT = 15,
    NETWORK_DISCONNECT_STEAM_DROPPED = 16,
    NETWORK_DISCONNECT_STEAM_OWNERSHIP = 17,
    NETWORK_DISCONNECT_SERVERINFO_OVERFLOW = 18,
    NETWORK_DISCONNECT_TICKMSG_OVERFLOW = 19,
    NETWORK_DISCONNECT_STRINGTABLEMSG_OVERFLOW = 20,
    NETWORK_DISCONNECT_DELTAENTMSG_OVERFLOW = 21,
    NETWORK_DISCONNECT_TEMPENTMSG_OVERFLOW = 22,
    NETWORK_DISCONNECT_SOUNDSMSG_OVERFLOW = 23,
    NETWORK_DISCONNECT_SNAPSHOTOVERFLOW = 24,
    NETWORK_DISCONNECT_SNAPSHOTERROR = 25,
    NETWORK_DISCONNECT_RELIABLEOVERFLOW = 26,
    NETWORK_DISCONNECT_BADDELTATICK = 27,
    NETWORK_DISCONNECT_NOMORESPLITS = 28,
    NETWORK_DISCONNECT_TIMEDOUT = 29,
    NETWORK_DISCONNECT_DISCONNECTED = 30,
    NETWORK_DISCONNECT_LEAVINGSPLIT = 31,
    NETWORK_DISCONNECT_DIFFERENTCLASSTABLES = 32,
    NETWORK_DISCONNECT_BADRELAYPASSWORD = 33,
    NETWORK_DISCONNECT_BADSPECTATORPASSWORD = 34,
    NETWORK_DISCONNECT_HLTVRESTRICTED = 35,
    NETWORK_DISCONNECT_NOSPECTATORS = 36,
    NETWORK_DISCONNECT_HLTVUNAVAILABLE = 37,
    NETWORK_DISCONNECT_HLTVSTOP = 38,
    NETWORK_DISCONNECT_KICKED = 39,
    NETWORK_DISCONNECT_BANADDED = 40,
    NETWORK_DISCONNECT_KICKBANADDED = 41,
    NETWORK_DISCONNECT_HLTVDIRECT = 42,
    NETWORK_DISCONNECT_PURESERVER_CLIENTEXTRA = 43,
    NETWORK_DISCONNECT_PURESERVER_MISMATCH = 44,
    NETWORK_DISCONNECT_USERCMD = 45,
    NETWORK_DISCONNECT_REJECTED_BY_GAME = 46,
    NETWORK_DISCONNECT_MESSAGE_PARSE_ERROR = 47,
    NETWORK_DISCONNECT_INVALID_MESSAGE_ERROR = 48,
    NETWORK_DISCONNECT_BAD_SERVER_PASSWORD = 49,
    NETWORK_DISCONNECT_DIRECT_CONNECT_RESERVATION = 50,
    NETWORK_DISCONNECT_CONNECTION_FAILURE = 51,
    NETWORK_DISCONNECT_NO_PEER_GROUP_HANDLERS = 52,
    NETWORK_DISCONNECT_RECONNECTION = 53,
    NETWORK_DISCONNECT_LOOPSHUTDOWN = 54,
    NETWORK_DISCONNECT_LOOPDEACTIVATE = 55,
    NETWORK_DISCONNECT_HOST_ENDGAME = 56,
    NETWORK_DISCONNECT_LOOP_LEVELLOAD_ACTIVATE = 57,
    NETWORK_DISCONNECT_CREATE_SERVER_FAILED = 58,
    NETWORK_DISCONNECT_EXITING = 59,
    NETWORK_DISCONNECT_REQUEST_HOSTSTATE_IDLE = 60,
    NETWORK_DISCONNECT_REQUEST_HOSTSTATE_HLTVRELAY = 61,
    NETWORK_DISCONNECT_CLIENT_CONSISTENCY_FAIL = 62,
    NETWORK_DISCONNECT_CLIENT_UNABLE_TO_CRC_MAP = 63,
    NETWORK_DISCONNECT_CLIENT_NO_MAP = 64,
    NETWORK_DISCONNECT_CLIENT_DIFFERENT_MAP = 65,
    NETWORK_DISCONNECT_SERVER_REQUIRES_STEAM = 66,
    NETWORK_DISCONNECT_STEAM_DENY_MISC = 67,
    NETWORK_DISCONNECT_STEAM_DENY_BAD_ANTI_CHEAT = 68,
    NETWORK_DISCONNECT_SERVER_SHUTDOWN = 69,
    NETWORK_DISCONNECT_SPLITPACKET_SEND_OVERFLOW = 70,
    NETWORK_DISCONNECT_REPLAY_INCOMPATIBLE = 71,
    NETWORK_DISCONNECT_CONNECT_REQUEST_TIMEDOUT = 72,
    NETWORK_DISCONNECT_SERVER_INCOMPATIBLE = 73,
    NETWORK_DISCONNECT_LOCALPROBLEM_MANYRELAYS = 74,
    NETWORK_DISCONNECT_LOCALPROBLEM_HOSTEDSERVERPRIMARYRELAY = 75,
    NETWORK_DISCONNECT_LOCALPROBLEM_NETWORKCONFIG = 76,
    NETWORK_DISCONNECT_LOCALPROBLEM_OTHER = 77,
    NETWORK_DISCONNECT_REMOTE_TIMEOUT = 79,
    NETWORK_DISCONNECT_REMOTE_TIMEOUT_CONNECTING = 80,
    NETWORK_DISCONNECT_REMOTE_OTHER = 81,
    NETWORK_DISCONNECT_REMOTE_BADCRYPT = 82,
    NETWORK_DISCONNECT_REMOTE_CERTNOTTRUSTED = 83,
    NETWORK_DISCONNECT_UNUSUAL = 84,
    NETWORK_DISCONNECT_INTERNAL_ERROR = 85,
    NETWORK_DISCONNECT_REJECT_BADCHALLENGE = 128,
    NETWORK_DISCONNECT_REJECT_NOLOBBY = 129,
    NETWORK_DISCONNECT_REJECT_BACKGROUND_MAP = 130,
    NETWORK_DISCONNECT_REJECT_SINGLE_PLAYER = 131,
    NETWORK_DISCONNECT_REJECT_HIDDEN_GAME = 132,
    NETWORK_DISCONNECT_REJECT_LANRESTRICT = 133,
    NETWORK_DISCONNECT_REJECT_BADPASSWORD = 134,
    NETWORK_DISCONNECT_REJECT_SERVERFULL = 135,
    NETWORK_DISCONNECT_REJECT_INVALIDRESERVATION = 136,
    NETWORK_DISCONNECT_REJECT_FAILEDCHANNEL = 137,
    NETWORK_DISCONNECT_REJECT_CONNECT_FROM_LOBBY = 138,
    NETWORK_DISCONNECT_REJECT_RESERVED_FOR_LOBBY = 139,
    NETWORK_DISCONNECT_REJECT_INVALIDKEYLENGTH = 140,
    NETWORK_DISCONNECT_REJECT_OLDPROTOCOL = 141,
    NETWORK_DISCONNECT_REJECT_NEWPROTOCOL = 142,
    NETWORK_DISCONNECT_REJECT_INVALIDCONNECTION = 143,
    NETWORK_DISCONNECT_REJECT_INVALIDCERTLEN = 144,
    NETWORK_DISCONNECT_REJECT_INVALIDSTEAMCERTLEN = 145,
    NETWORK_DISCONNECT_REJECT_STEAM = 146,
    NETWORK_DISCONNECT_REJECT_SERVERAUTHDISABLED = 147,
    NETWORK_DISCONNECT_REJECT_SERVERCDKEYAUTHINVALID = 148,
    NETWORK_DISCONNECT_REJECT_BANNED = 149,
}

impl ::protobuf::ProtobufEnum for ENetworkDisconnectionReason {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ENetworkDisconnectionReason> {
        match value {
            0 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_INVALID),
            1 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_SHUTDOWN),
            2 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_DISCONNECT_BY_USER),
            3 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_DISCONNECT_BY_SERVER),
            4 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOST),
            5 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_OVERFLOW),
            6 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_BANNED),
            7 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_INUSE),
            8 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_TICKET),
            9 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_LOGON),
            10 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_AUTHCANCELLED),
            11 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_AUTHALREADYUSED),
            12 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_AUTHINVALID),
            13 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_VACBANSTATE),
            14 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_LOGGED_IN_ELSEWHERE),
            15 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_VAC_CHECK_TIMEDOUT),
            16 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_DROPPED),
            17 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_OWNERSHIP),
            18 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_SERVERINFO_OVERFLOW),
            19 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_TICKMSG_OVERFLOW),
            20 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_STRINGTABLEMSG_OVERFLOW),
            21 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_DELTAENTMSG_OVERFLOW),
            22 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_TEMPENTMSG_OVERFLOW),
            23 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_SOUNDSMSG_OVERFLOW),
            24 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_SNAPSHOTOVERFLOW),
            25 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_SNAPSHOTERROR),
            26 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_RELIABLEOVERFLOW),
            27 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_BADDELTATICK),
            28 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_NOMORESPLITS),
            29 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_TIMEDOUT),
            30 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_DISCONNECTED),
            31 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_LEAVINGSPLIT),
            32 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_DIFFERENTCLASSTABLES),
            33 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_BADRELAYPASSWORD),
            34 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_BADSPECTATORPASSWORD),
            35 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_HLTVRESTRICTED),
            36 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_NOSPECTATORS),
            37 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_HLTVUNAVAILABLE),
            38 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_HLTVSTOP),
            39 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_KICKED),
            40 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_BANADDED),
            41 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_KICKBANADDED),
            42 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_HLTVDIRECT),
            43 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_PURESERVER_CLIENTEXTRA),
            44 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_PURESERVER_MISMATCH),
            45 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_USERCMD),
            46 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECTED_BY_GAME),
            47 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_MESSAGE_PARSE_ERROR),
            48 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_INVALID_MESSAGE_ERROR),
            49 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_BAD_SERVER_PASSWORD),
            50 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_DIRECT_CONNECT_RESERVATION),
            51 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_CONNECTION_FAILURE),
            52 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_NO_PEER_GROUP_HANDLERS),
            53 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_RECONNECTION),
            54 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOOPSHUTDOWN),
            55 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOOPDEACTIVATE),
            56 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_HOST_ENDGAME),
            57 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOOP_LEVELLOAD_ACTIVATE),
            58 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_CREATE_SERVER_FAILED),
            59 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_EXITING),
            60 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REQUEST_HOSTSTATE_IDLE),
            61 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REQUEST_HOSTSTATE_HLTVRELAY),
            62 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_CLIENT_CONSISTENCY_FAIL),
            63 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_CLIENT_UNABLE_TO_CRC_MAP),
            64 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_CLIENT_NO_MAP),
            65 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_CLIENT_DIFFERENT_MAP),
            66 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_SERVER_REQUIRES_STEAM),
            67 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_DENY_MISC),
            68 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_DENY_BAD_ANTI_CHEAT),
            69 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_SERVER_SHUTDOWN),
            70 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_SPLITPACKET_SEND_OVERFLOW),
            71 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REPLAY_INCOMPATIBLE),
            72 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_CONNECT_REQUEST_TIMEDOUT),
            73 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_SERVER_INCOMPATIBLE),
            74 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOCALPROBLEM_MANYRELAYS),
            75 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOCALPROBLEM_HOSTEDSERVERPRIMARYRELAY),
            76 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOCALPROBLEM_NETWORKCONFIG),
            77 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOCALPROBLEM_OTHER),
            79 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REMOTE_TIMEOUT),
            80 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REMOTE_TIMEOUT_CONNECTING),
            81 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REMOTE_OTHER),
            82 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REMOTE_BADCRYPT),
            83 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REMOTE_CERTNOTTRUSTED),
            84 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_UNUSUAL),
            85 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_INTERNAL_ERROR),
            128 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_BADCHALLENGE),
            129 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_NOLOBBY),
            130 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_BACKGROUND_MAP),
            131 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_SINGLE_PLAYER),
            132 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_HIDDEN_GAME),
            133 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_LANRESTRICT),
            134 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_BADPASSWORD),
            135 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_SERVERFULL),
            136 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_INVALIDRESERVATION),
            137 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_FAILEDCHANNEL),
            138 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_CONNECT_FROM_LOBBY),
            139 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_RESERVED_FOR_LOBBY),
            140 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_INVALIDKEYLENGTH),
            141 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_OLDPROTOCOL),
            142 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_NEWPROTOCOL),
            143 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_INVALIDCONNECTION),
            144 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_INVALIDCERTLEN),
            145 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_INVALIDSTEAMCERTLEN),
            146 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_STEAM),
            147 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_SERVERAUTHDISABLED),
            148 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_SERVERCDKEYAUTHINVALID),
            149 => ::std::option::Option::Some(ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_BANNED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ENetworkDisconnectionReason] = &[
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_INVALID,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_SHUTDOWN,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_DISCONNECT_BY_USER,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_DISCONNECT_BY_SERVER,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOST,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_OVERFLOW,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_BANNED,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_INUSE,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_TICKET,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_LOGON,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_AUTHCANCELLED,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_AUTHALREADYUSED,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_AUTHINVALID,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_VACBANSTATE,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_LOGGED_IN_ELSEWHERE,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_VAC_CHECK_TIMEDOUT,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_DROPPED,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_OWNERSHIP,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_SERVERINFO_OVERFLOW,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_TICKMSG_OVERFLOW,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_STRINGTABLEMSG_OVERFLOW,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_DELTAENTMSG_OVERFLOW,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_TEMPENTMSG_OVERFLOW,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_SOUNDSMSG_OVERFLOW,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_SNAPSHOTOVERFLOW,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_SNAPSHOTERROR,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_RELIABLEOVERFLOW,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_BADDELTATICK,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_NOMORESPLITS,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_TIMEDOUT,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_DISCONNECTED,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_LEAVINGSPLIT,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_DIFFERENTCLASSTABLES,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_BADRELAYPASSWORD,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_BADSPECTATORPASSWORD,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_HLTVRESTRICTED,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_NOSPECTATORS,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_HLTVUNAVAILABLE,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_HLTVSTOP,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_KICKED,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_BANADDED,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_KICKBANADDED,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_HLTVDIRECT,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_PURESERVER_CLIENTEXTRA,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_PURESERVER_MISMATCH,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_USERCMD,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECTED_BY_GAME,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_MESSAGE_PARSE_ERROR,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_INVALID_MESSAGE_ERROR,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_BAD_SERVER_PASSWORD,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_DIRECT_CONNECT_RESERVATION,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_CONNECTION_FAILURE,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_NO_PEER_GROUP_HANDLERS,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_RECONNECTION,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOOPSHUTDOWN,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOOPDEACTIVATE,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_HOST_ENDGAME,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOOP_LEVELLOAD_ACTIVATE,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_CREATE_SERVER_FAILED,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_EXITING,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REQUEST_HOSTSTATE_IDLE,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REQUEST_HOSTSTATE_HLTVRELAY,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_CLIENT_CONSISTENCY_FAIL,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_CLIENT_UNABLE_TO_CRC_MAP,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_CLIENT_NO_MAP,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_CLIENT_DIFFERENT_MAP,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_SERVER_REQUIRES_STEAM,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_DENY_MISC,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_STEAM_DENY_BAD_ANTI_CHEAT,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_SERVER_SHUTDOWN,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_SPLITPACKET_SEND_OVERFLOW,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REPLAY_INCOMPATIBLE,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_CONNECT_REQUEST_TIMEDOUT,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_SERVER_INCOMPATIBLE,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOCALPROBLEM_MANYRELAYS,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOCALPROBLEM_HOSTEDSERVERPRIMARYRELAY,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOCALPROBLEM_NETWORKCONFIG,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_LOCALPROBLEM_OTHER,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REMOTE_TIMEOUT,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REMOTE_TIMEOUT_CONNECTING,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REMOTE_OTHER,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REMOTE_BADCRYPT,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REMOTE_CERTNOTTRUSTED,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_UNUSUAL,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_INTERNAL_ERROR,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_BADCHALLENGE,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_NOLOBBY,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_BACKGROUND_MAP,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_SINGLE_PLAYER,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_HIDDEN_GAME,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_LANRESTRICT,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_BADPASSWORD,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_SERVERFULL,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_INVALIDRESERVATION,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_FAILEDCHANNEL,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_CONNECT_FROM_LOBBY,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_RESERVED_FOR_LOBBY,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_INVALIDKEYLENGTH,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_OLDPROTOCOL,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_NEWPROTOCOL,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_INVALIDCONNECTION,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_INVALIDCERTLEN,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_INVALIDSTEAMCERTLEN,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_STEAM,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_SERVERAUTHDISABLED,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_SERVERCDKEYAUTHINVALID,
            ENetworkDisconnectionReason::NETWORK_DISCONNECT_REJECT_BANNED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ENetworkDisconnectionReason>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ENetworkDisconnectionReason", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ENetworkDisconnectionReason {
}

impl ::protobuf::reflect::ProtobufValue for ENetworkDisconnectionReason {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

pub mod exts {
    use protobuf::Message as Message_imported_for_functions;

    pub const network_connection_token: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumValueOptions, ::protobuf::types::ProtobufTypeString> = ::protobuf::ext::ExtFieldOptional { field_number: 50500, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18network_connection.proto\x1a\x20google/protobuf/descriptor.proto*\
    \xa3C\n\x1bENetworkDisconnectionReason\x12\x1e\n\x1aNETWORK_DISCONNECT_I\
    NVALID\x10\0\x12\x1f\n\x1bNETWORK_DISCONNECT_SHUTDOWN\x10\x01\x12F\n%NET\
    WORK_DISCONNECT_DISCONNECT_BY_USER\x10\x02\x1a\x1b\xa2\xd4\x18\x17#GameU\
    I_Disconnect_User\x12J\n'NETWORK_DISCONNECT_DISCONNECT_BY_SERVER\x10\x03\
    \x1a\x1d\xa2\xd4\x18\x19#GameUI_Disconnect_Server\x12B\n\x17NETWORK_DISC\
    ONNECT_LOST\x10\x04\x1a%\xa2\xd4\x18!#GameUI_Disconnect_ConnectionLost\
    \x12J\n\x1bNETWORK_DISCONNECT_OVERFLOW\x10\x05\x1a)\xa2\xd4\x18%#GameUI_\
    Disconnect_ConnectionOverflow\x12I\n\x1fNETWORK_DISCONNECT_STEAM_BANNED\
    \x10\x06\x1a$\xa2\xd4\x18\x20#GameUI_Disconnect_SteamIDBanned\x12G\n\x1e\
    NETWORK_DISCONNECT_STEAM_INUSE\x10\x07\x1a#\xa2\xd4\x18\x1f#GameUI_Disco\
    nnect_SteamIDInUse\x12G\n\x1fNETWORK_DISCONNECT_STEAM_TICKET\x10\x08\x1a\
    \"\xa2\xd4\x18\x1e#GameUI_Disconnect_SteamTicket\x12E\n\x1eNETWORK_DISCO\
    NNECT_STEAM_LOGON\x10\t\x1a!\xa2\xd4\x18\x1d#GameUI_Disconnect_SteamLogo\
    n\x12M\n&NETWORK_DISCONNECT_STEAM_AUTHCANCELLED\x10\n\x1a!\xa2\xd4\x18\
    \x1d#GameUI_Disconnect_SteamLogon\x12O\n(NETWORK_DISCONNECT_STEAM_AUTHAL\
    READYUSED\x10\x0b\x1a!\xa2\xd4\x18\x1d#GameUI_Disconnect_SteamLogon\x12K\
    \n$NETWORK_DISCONNECT_STEAM_AUTHINVALID\x10\x0c\x1a!\xa2\xd4\x18\x1d#Gam\
    eUI_Disconnect_SteamLogon\x12I\n$NETWORK_DISCONNECT_STEAM_VACBANSTATE\
    \x10\r\x1a\x1f\xa2\xd4\x18\x1b#GameUI_Disconnect_SteamVAC\x12S\n,NETWORK\
    _DISCONNECT_STEAM_LOGGED_IN_ELSEWHERE\x10\x0e\x1a!\xa2\xd4\x18\x1d#GameU\
    I_Disconnect_SteamInUse\x12T\n+NETWORK_DISCONNECT_STEAM_VAC_CHECK_TIMEDO\
    UT\x10\x0f\x1a#\xa2\xd4\x18\x1f#GameUI_Disconnect_SteamTimeOut\x12I\n\
    \x20NETWORK_DISCONNECT_STEAM_DROPPED\x10\x10\x1a#\xa2\xd4\x18\x1f#GameUI\
    _Disconnect_SteamDropped\x12M\n\"NETWORK_DISCONNECT_STEAM_OWNERSHIP\x10\
    \x11\x1a%\xa2\xd4\x18!#GameUI_Disconnect_SteamOwnership\x12U\n&NETWORK_D\
    ISCONNECT_SERVERINFO_OVERFLOW\x10\x12\x1a)\xa2\xd4\x18%#GameUI_Disconnec\
    t_ServerInfoOverflow\x12K\n#NETWORK_DISCONNECT_TICKMSG_OVERFLOW\x10\x13\
    \x1a\"\xa2\xd4\x18\x1e#GameUI_Disconnect_TickMessage\x12Y\n*NETWORK_DISC\
    ONNECT_STRINGTABLEMSG_OVERFLOW\x10\x14\x1a)\xa2\xd4\x18%#GameUI_Disconne\
    ct_StringTableMessage\x12S\n'NETWORK_DISCONNECT_DELTAENTMSG_OVERFLOW\x10\
    \x15\x1a&\xa2\xd4\x18\"#GameUI_Disconnect_DeltaEntMessage\x12Q\n&NETWORK\
    _DISCONNECT_TEMPENTMSG_OVERFLOW\x10\x16\x1a%\xa2\xd4\x18!#GameUI_Disconn\
    ect_TempEntMessage\x12O\n%NETWORK_DISCONNECT_SOUNDSMSG_OVERFLOW\x10\x17\
    \x1a$\xa2\xd4\x18\x20#GameUI_Disconnect_SoundsMessage\x12P\n#NETWORK_DIS\
    CONNECT_SNAPSHOTOVERFLOW\x10\x18\x1a'\xa2\xd4\x18##GameUI_Disconnect_Sna\
    pshotOverflow\x12J\n\x20NETWORK_DISCONNECT_SNAPSHOTERROR\x10\x19\x1a$\
    \xa2\xd4\x18\x20#GameUI_Disconnect_SnapshotError\x12P\n#NETWORK_DISCONNE\
    CT_RELIABLEOVERFLOW\x10\x1a\x1a'\xa2\xd4\x18##GameUI_Disconnect_Reliable\
    Overflow\x12N\n\x1fNETWORK_DISCONNECT_BADDELTATICK\x10\x1b\x1a)\xa2\xd4\
    \x18%#GameUI_Disconnect_BadClientDeltaTick\x12H\n\x1fNETWORK_DISCONNECT_\
    NOMORESPLITS\x10\x1c\x1a#\xa2\xd4\x18\x1f#GameUI_Disconnect_NoMoreSplits\
    \x12@\n\x1bNETWORK_DISCONNECT_TIMEDOUT\x10\x1d\x1a\x1f\xa2\xd4\x18\x1b#G\
    ameUI_Disconnect_TimedOut\x12H\n\x1fNETWORK_DISCONNECT_DISCONNECTED\x10\
    \x1e\x1a#\xa2\xd4\x18\x1f#GameUI_Disconnect_Disconnected\x12H\n\x1fNETWO\
    RK_DISCONNECT_LEAVINGSPLIT\x10\x1f\x1a#\xa2\xd4\x18\x1f#GameUI_Disconnec\
    t_LeavingSplit\x12X\n'NETWORK_DISCONNECT_DIFFERENTCLASSTABLES\x10\x20\
    \x1a+\xa2\xd4\x18'#GameUI_Disconnect_DifferentClassTables\x12P\n#NETWORK\
    _DISCONNECT_BADRELAYPASSWORD\x10!\x1a'\xa2\xd4\x18##GameUI_Disconnect_Ba\
    dRelayPassword\x12X\n'NETWORK_DISCONNECT_BADSPECTATORPASSWORD\x10\"\x1a+\
    \xa2\xd4\x18'#GameUI_Disconnect_BadSpectatorPassword\x12L\n!NETWORK_DISC\
    ONNECT_HLTVRESTRICTED\x10#\x1a%\xa2\xd4\x18!#GameUI_Disconnect_HLTVRestr\
    icted\x12H\n\x1fNETWORK_DISCONNECT_NOSPECTATORS\x10$\x1a#\xa2\xd4\x18\
    \x1f#GameUI_Disconnect_NoSpectators\x12N\n\"NETWORK_DISCONNECT_HLTVUNAVA\
    ILABLE\x10%\x1a&\xa2\xd4\x18\"#GameUI_Disconnect_HLTVUnavailable\x12@\n\
    \x1bNETWORK_DISCONNECT_HLTVSTOP\x10&\x1a\x1f\xa2\xd4\x18\x1b#GameUI_Disc\
    onnect_HLTVStop\x12<\n\x19NETWORK_DISCONNECT_KICKED\x10'\x1a\x1d\xa2\xd4\
    \x18\x19#GameUI_Disconnect_Kicked\x12@\n\x1bNETWORK_DISCONNECT_BANADDED\
    \x10(\x1a\x1f\xa2\xd4\x18\x1b#GameUI_Disconnect_BanAdded\x12H\n\x1fNETWO\
    RK_DISCONNECT_KICKBANADDED\x10)\x1a#\xa2\xd4\x18\x1f#GameUI_Disconnect_K\
    ickBanAdded\x12D\n\x1dNETWORK_DISCONNECT_HLTVDIRECT\x10*\x1a!\xa2\xd4\
    \x18\x1d#GameUI_Disconnect_HLTVDirect\x12\\\n)NETWORK_DISCONNECT_PURESER\
    VER_CLIENTEXTRA\x10+\x1a-\xa2\xd4\x18)#GameUI_Disconnect_PureServer_Clie\
    ntExtra\x12V\n&NETWORK_DISCONNECT_PURESERVER_MISMATCH\x10,\x1a*\xa2\xd4\
    \x18&#GameUI_Disconnect_PureServer_Mismatch\x12>\n\x1aNETWORK_DISCONNECT\
    _USERCMD\x10-\x1a\x1e\xa2\xd4\x18\x1a#GameUI_Disconnect_UserCmd\x12N\n#N\
    ETWORK_DISCONNECT_REJECTED_BY_GAME\x10.\x1a%\xa2\xd4\x18!#GameUI_Disconn\
    ect_RejectedByGame\x12T\n&NETWORK_DISCONNECT_MESSAGE_PARSE_ERROR\x10/\
    \x1a(\xa2\xd4\x18$#GameUI_Disconnect_MessageParseError\x12X\n(NETWORK_DI\
    SCONNECT_INVALID_MESSAGE_ERROR\x100\x1a*\xa2\xd4\x18&#GameUI_Disconnect_\
    InvalidMessageError\x12T\n&NETWORK_DISCONNECT_BAD_SERVER_PASSWORD\x101\
    \x1a(\xa2\xd4\x18$#GameUI_Disconnect_BadServerPassword\x121\n-NETWORK_DI\
    SCONNECT_DIRECT_CONNECT_RESERVATION\x102\x12S\n%NETWORK_DISCONNECT_CONNE\
    CTION_FAILURE\x103\x1a(\xa2\xd4\x18$#GameUI_Disconnect_ConnectionFailure\
    \x12Y\n)NETWORK_DISCONNECT_NO_PEER_GROUP_HANDLERS\x104\x1a*\xa2\xd4\x18&\
    #GameUI_Disconnect_NoPeerGroupHandlers\x12#\n\x1fNETWORK_DISCONNECT_RECO\
    NNECTION\x105\x12H\n\x1fNETWORK_DISCONNECT_LOOPSHUTDOWN\x106\x1a#\xa2\
    \xd4\x18\x1f#GameUI_Disconnect_LoopShutdown\x12L\n!NETWORK_DISCONNECT_LO\
    OPDEACTIVATE\x107\x1a%\xa2\xd4\x18!#GameUI_Disconnect_LoopDeactivate\x12\
    H\n\x1fNETWORK_DISCONNECT_HOST_ENDGAME\x108\x1a#\xa2\xd4\x18\x1f#GameUI_\
    Disconnect_Host_EndGame\x12\\\n*NETWORK_DISCONNECT_LOOP_LEVELLOAD_ACTIVA\
    TE\x109\x1a,\xa2\xd4\x18(#GameUI_Disconnect_LoopLevelLoadActivate\x12V\n\
    'NETWORK_DISCONNECT_CREATE_SERVER_FAILED\x10:\x1a)\xa2\xd4\x18%#GameUI_D\
    isconnect_CreateServerFailed\x12D\n\x1aNETWORK_DISCONNECT_EXITING\x10;\
    \x1a$\xa2\xd4\x18\x20#GameUI_Disconnect_ExitingEngine\x12T\n)NETWORK_DIS\
    CONNECT_REQUEST_HOSTSTATE_IDLE\x10<\x1a%\xa2\xd4\x18!#GameUI_Disconnect_\
    Request_HSIdle\x12\\\n.NETWORK_DISCONNECT_REQUEST_HOSTSTATE_HLTVRELAY\
    \x10=\x1a(\xa2\xd4\x18$#GameUI_Disconnect_Request_HLTVRelay\x12Q\n*NETWO\
    RK_DISCONNECT_CLIENT_CONSISTENCY_FAIL\x10>\x1a!\xa2\xd4\x18\x1d#GameUI_C\
    lientConsistencyFail\x12Q\n+NETWORK_DISCONNECT_CLIENT_UNABLE_TO_CRC_MAP\
    \x10?\x1a\x20\xa2\xd4\x18\x1c#GameUI_ClientUnableToCRCMap\x12=\n\x20NETW\
    ORK_DISCONNECT_CLIENT_NO_MAP\x10@\x1a\x17\xa2\xd4\x18\x13#GameUI_ClientN\
    oMap\x12K\n'NETWORK_DISCONNECT_CLIENT_DIFFERENT_MAP\x10A\x1a\x1e\xa2\xd4\
    \x18\x1a#GameUI_ClientDifferentMap\x12M\n(NETWORK_DISCONNECT_SERVER_REQU\
    IRES_STEAM\x10B\x1a\x1f\xa2\xd4\x18\x1b#GameUI_ServerRequireSteams\x12M\
    \n\"NETWORK_DISCONNECT_STEAM_DENY_MISC\x10C\x1a%\xa2\xd4\x18!#GameUI_Dis\
    connect_SteamDeny_Misc\x12_\n,NETWORK_DISCONNECT_STEAM_DENY_BAD_ANTI_CHE\
    AT\x10D\x1a-\xa2\xd4\x18)#GameUI_Disconnect_SteamDeny_BadAntiCheat\x12M\
    \n\"NETWORK_DISCONNECT_SERVER_SHUTDOWN\x10E\x1a%\xa2\xd4\x18!#GameUI_Dis\
    connect_ServerShutdown\x12b\n,NETWORK_DISCONNECT_SPLITPACKET_SEND_OVERFL\
    OW\x10F\x1a0\xa2\xd4\x18,#GameUI_Disconnect_Splitpacket_Send_Overflow\
    \x12U\n&NETWORK_DISCONNECT_REPLAY_INCOMPATIBLE\x10G\x1a)\xa2\xd4\x18%#Ga\
    meUI_Disconnect_ReplayIncompatible\x12Z\n+NETWORK_DISCONNECT_CONNECT_REQ\
    UEST_TIMEDOUT\x10H\x1a)\xa2\xd4\x18%#GameUI_Disconnect_ConnectionTimedou\
    t\x12U\n&NETWORK_DISCONNECT_SERVER_INCOMPATIBLE\x10I\x1a)\xa2\xd4\x18%#G\
    ameUI_Disconnect_ServerIncompatible\x12^\n*NETWORK_DISCONNECT_LOCALPROBL\
    EM_MANYRELAYS\x10J\x1a.\xa2\xd4\x18*#GameUI_Disconnect_LocalProblem_Many\
    Relays\x12z\n8NETWORK_DISCONNECT_LOCALPROBLEM_HOSTEDSERVERPRIMARYRELAY\
    \x10K\x1a<\xa2\xd4\x188#GameUI_Disconnect_LocalProblem_HostedServerPrima\
    ryRelay\x12d\n-NETWORK_DISCONNECT_LOCALPROBLEM_NETWORKCONFIG\x10L\x1a1\
    \xa2\xd4\x18-#GameUI_Disconnect_LocalProblem_NetworkConfig\x12T\n%NETWOR\
    K_DISCONNECT_LOCALPROBLEM_OTHER\x10M\x1a)\xa2\xd4\x18%#GameUI_Disconnect\
    _LocalProblem_Other\x12S\n!NETWORK_DISCONNECT_REMOTE_TIMEOUT\x10O\x1a,\
    \xa2\xd4\x18(#GameUI_Disconnect_RemoteProblem_Timeout\x12h\n,NETWORK_DIS\
    CONNECT_REMOTE_TIMEOUT_CONNECTING\x10P\x1a6\xa2\xd4\x182#GameUI_Disconne\
    ct_RemoteProblem_TimeoutConnecting\x12O\n\x1fNETWORK_DISCONNECT_REMOTE_O\
    THER\x10Q\x1a*\xa2\xd4\x18&#GameUI_Disconnect_RemoteProblem_Other\x12U\n\
    \"NETWORK_DISCONNECT_REMOTE_BADCRYPT\x10R\x1a-\xa2\xd4\x18)#GameUI_Disco\
    nnect_RemoteProblem_BadCrypt\x12a\n(NETWORK_DISCONNECT_REMOTE_CERTNOTTRU\
    STED\x10S\x1a3\xa2\xd4\x18/#GameUI_Disconnect_RemoteProblem_CertNotTrust\
    ed\x12>\n\x1aNETWORK_DISCONNECT_UNUSUAL\x10T\x1a\x1e\xa2\xd4\x18\x1a#Gam\
    eUI_Disconnect_Unusual\x12K\n!NETWORK_DISCONNECT_INTERNAL_ERROR\x10U\x1a\
    $\xa2\xd4\x18\x20#GameUI_Disconnect_InternalError\x12Q\n&NETWORK_DISCONN\
    ECT_REJECT_BADCHALLENGE\x10\x80\x01\x1a$\xa2\xd4\x18\x20#GameUI_ServerRe\
    jectBadChallenge\x12A\n!NETWORK_DISCONNECT_REJECT_NOLOBBY\x10\x81\x01\
    \x1a\x19\xa2\xd4\x18\x15#GameUI_ServerNoLobby\x12O\n(NETWORK_DISCONNECT_\
    REJECT_BACKGROUND_MAP\x10\x82\x01\x1a\x20\xa2\xd4\x18\x1c#Valve_Reject_B\
    ackground_Map\x12M\n'NETWORK_DISCONNECT_REJECT_SINGLE_PLAYER\x10\x83\x01\
    \x1a\x1f\xa2\xd4\x18\x1b#Valve_Reject_Single_Player\x12I\n%NETWORK_DISCO\
    NNECT_REJECT_HIDDEN_GAME\x10\x84\x01\x1a\x1d\xa2\xd4\x18\x19#Valve_Rejec\
    t_Hidden_Game\x12O\n%NETWORK_DISCONNECT_REJECT_LANRESTRICT\x10\x85\x01\
    \x1a#\xa2\xd4\x18\x1f#GameUI_ServerRejectLANRestrict\x12O\n%NETWORK_DISC\
    ONNECT_REJECT_BADPASSWORD\x10\x86\x01\x1a#\xa2\xd4\x18\x1f#GameUI_Server\
    RejectBadPassword\x12M\n$NETWORK_DISCONNECT_REJECT_SERVERFULL\x10\x87\
    \x01\x1a\"\xa2\xd4\x18\x1e#GameUI_ServerRejectServerFull\x12]\n,NETWORK_\
    DISCONNECT_REJECT_INVALIDRESERVATION\x10\x88\x01\x1a*\xa2\xd4\x18&#GameU\
    I_ServerRejectInvalidReservation\x12S\n'NETWORK_DISCONNECT_REJECT_FAILED\
    CHANNEL\x10\x89\x01\x1a%\xa2\xd4\x18!#GameUI_ServerRejectFailedChannel\
    \x12W\n,NETWORK_DISCONNECT_REJECT_CONNECT_FROM_LOBBY\x10\x8a\x01\x1a$\
    \xa2\xd4\x18\x20#Valve_Reject_Connect_From_Lobby\x12W\n,NETWORK_DISCONNE\
    CT_REJECT_RESERVED_FOR_LOBBY\x10\x8b\x01\x1a$\xa2\xd4\x18\x20#Valve_Reje\
    ct_Reserved_For_Lobby\x12Z\n*NETWORK_DISCONNECT_REJECT_INVALIDKEYLENGTH\
    \x10\x8c\x01\x1a)\xa2\xd4\x18%#GameUI_ServerReject_InvalidKeyLength\x12O\
    \n%NETWORK_DISCONNECT_REJECT_OLDPROTOCOL\x10\x8d\x01\x1a#\xa2\xd4\x18\
    \x1f#GameUI_ServerRejectOldProtocol\x12O\n%NETWORK_DISCONNECT_REJECT_NEW\
    PROTOCOL\x10\x8e\x01\x1a#\xa2\xd4\x18\x1f#GameUI_ServerRejectNewProtocol\
    \x12[\n+NETWORK_DISCONNECT_REJECT_INVALIDCONNECTION\x10\x8f\x01\x1a)\xa2\
    \xd4\x18%#GameUI_ServerRejectInvalidConnection\x12U\n(NETWORK_DISCONNECT\
    _REJECT_INVALIDCERTLEN\x10\x90\x01\x1a&\xa2\xd4\x18\"#GameUI_ServerRejec\
    tInvalidCertLen\x12_\n-NETWORK_DISCONNECT_REJECT_INVALIDSTEAMCERTLEN\x10\
    \x91\x01\x1a+\xa2\xd4\x18'#GameUI_ServerRejectInvalidSteamCertLen\x12C\n\
    \x1fNETWORK_DISCONNECT_REJECT_STEAM\x10\x92\x01\x1a\x1d\xa2\xd4\x18\x19#\
    GameUI_ServerRejectSteam\x12Q\n,NETWORK_DISCONNECT_REJECT_SERVERAUTHDISA\
    BLED\x10\x93\x01\x1a\x1e\xa2\xd4\x18\x1a#GameUI_ServerAuthDisabled\x12Y\
    \n0NETWORK_DISCONNECT_REJECT_SERVERCDKEYAUTHINVALID\x10\x94\x01\x1a\"\
    \xa2\xd4\x18\x1e#GameUI_ServerCDKeyAuthInvalid\x12E\n\x20NETWORK_DISCONN\
    ECT_REJECT_BANNED\x10\x95\x01\x1a\x1e\xa2\xd4\x18\x1a#GameUI_ServerRejec\
    tBanned:]\n\x18network_connection_token\x18\xc4\x8a\x03\x20\x01(\t\x12!.\
    google.protobuf.EnumValueOptionsR\x16networkConnectionTokenB\x03\x80\x01\
    \0\
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
