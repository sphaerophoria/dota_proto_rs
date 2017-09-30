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
pub enum EDOTAGCMsg {
    k_EMsgGCDOTABase = 7000,
    k_EMsgGCGeneralResponse = 7001,
    k_EMsgGCGameMatchSignOut = 7004,
    k_EMsgGCGameMatchSignOutResponse = 7005,
    k_EMsgGCJoinChatChannel = 7009,
    k_EMsgGCJoinChatChannelResponse = 7010,
    k_EMsgGCOtherJoinedChannel = 7013,
    k_EMsgGCOtherLeftChannel = 7014,
    k_EMsgGCMatchHistoryList = 7017,
    k_EMsgServerToGCRequestStatus = 7026,
    k_EMsgGCGetRecentMatches = 7027,
    k_EMsgGCRecentMatchesResponse = 7028,
    k_EMsgGCStartFindingMatch = 7033,
    k_EMsgGCConnectedPlayers = 7034,
    k_EMsgGCAbandonCurrentGame = 7035,
    k_EMsgGCStopFindingMatch = 7036,
    k_EMsgGCPracticeLobbyCreate = 7038,
    k_EMsgGCPracticeLobbyLeave = 7040,
    k_EMsgGCPracticeLobbyLaunch = 7041,
    k_EMsgGCPracticeLobbyList = 7042,
    k_EMsgGCPracticeLobbyListResponse = 7043,
    k_EMsgGCPracticeLobbyJoin = 7044,
    k_EMsgGCPracticeLobbySetDetails = 7046,
    k_EMsgGCPracticeLobbySetTeamSlot = 7047,
    k_EMsgGCInitialQuestionnaireResponse = 7049,
    k_EMsgGCPracticeLobbyResponse = 7055,
    k_EMsgGCBroadcastNotification = 7056,
    k_EMsgGCLiveScoreboardUpdate = 7057,
    k_EMsgGCRequestChatChannelList = 7060,
    k_EMsgGCRequestChatChannelListResponse = 7061,
    k_EMsgGCRequestMatches = 7064,
    k_EMsgGCRequestMatchesResponse = 7065,
    k_EMsgGCRequestPlayerResources = 7068,
    k_EMsgGCRequestPlayerResourcesResponse = 7069,
    k_EMsgGCReadyUp = 7070,
    k_EMsgGCKickedFromMatchmakingQueue = 7071,
    k_EMsgGCLeaverDetected = 7072,
    k_EMsgGCSpectateFriendGame = 7073,
    k_EMsgGCSpectateFriendGameResponse = 7074,
    k_EMsgGCPlayerReports = 7075,
    k_EMsgGCReportsRemainingRequest = 7076,
    k_EMsgGCReportsRemainingResponse = 7077,
    k_EMsgGCSubmitPlayerReport = 7078,
    k_EMsgGCSubmitPlayerReportResponse = 7079,
    k_EMsgGCPracticeLobbyKick = 7081,
    k_EMsgGCReportCountsRequest = 7082,
    k_EMsgGCReportCountsResponse = 7083,
    k_EMsgGCRequestSaveGames = 7084,
    k_EMsgGCRequestSaveGamesServer = 7085,
    k_EMsgGCRequestSaveGamesResponse = 7086,
    k_EMsgGCLeaverDetectedResponse = 7087,
    k_EMsgGCPlayerFailedToConnect = 7088,
    k_EMsgGCGCToRelayConnect = 7089,
    k_EMsgGCGCToRelayConnectresponse = 7090,
    k_EMsgGCWatchGame = 7091,
    k_EMsgGCWatchGameResponse = 7092,
    k_EMsgGCBanStatusRequest = 7093,
    k_EMsgGCBanStatusResponse = 7094,
    k_EMsgGCMatchDetailsRequest = 7095,
    k_EMsgGCMatchDetailsResponse = 7096,
    k_EMsgGCCancelWatchGame = 7097,
    k_EMsgGCProfileRequest = 7098,
    k_EMsgGCProfileResponse = 7099,
    k_EMsgGCPopup = 7102,
    k_EMsgGCDOTAClearNotifySuccessfulReport = 7104,
    k_EMsgGCFriendPracticeLobbyListRequest = 7111,
    k_EMsgGCFriendPracticeLobbyListResponse = 7112,
    k_EMsgGCPracticeLobbyJoinResponse = 7113,
    k_EMsgClientEconNotification_Job = 7114,
    k_EMsgGCCreateTeam = 7115,
    k_EMsgGCCreateTeamResponse = 7116,
    k_EMsgGCTeamData = 7121,
    k_EMsgGCTeamInvite_InviterToGC = 7122,
    k_EMsgGCTeamInvite_GCImmediateResponseToInviter = 7123,
    k_EMsgGCTeamInvite_GCRequestToInvitee = 7124,
    k_EMsgGCTeamInvite_InviteeResponseToGC = 7125,
    k_EMsgGCTeamInvite_GCResponseToInviter = 7126,
    k_EMsgGCTeamInvite_GCResponseToInvitee = 7127,
    k_EMsgGCKickTeamMember = 7128,
    k_EMsgGCKickTeamMemberResponse = 7129,
    k_EMsgGCLeaveTeam = 7130,
    k_EMsgGCLeaveTeamResponse = 7131,
    k_EMsgGCSuggestTeamMatchmaking = 7132,
    k_EMsgGCPlayerHeroesFavoritesAdd = 7133,
    k_EMsgGCPlayerHeroesFavoritesRemove = 7134,
    k_EMsgGCSetShowcaseHero = 7141,
    k_EMsgGCApplyTeamToPracticeLobby = 7142,
    k_EMsgGCRequestInternatinalTicketEmail = 7143,
    k_EMsgGCTransferTeamAdmin = 7144,
    k_EMsgRequestLeagueInfo = 7147,
    k_EMsgResponseLeagueInfo = 7148,
    k_EMsgGCPracticeLobbyJoinBroadcastChannel = 7149,
    k_EMsgGC_TournamentItemEvent = 7150,
    k_EMsgGC_TournamentItemEventResponse = 7151,
    k_EMsgCastMatchVote = 7152,
    k_EMsgCastMatchVoteResponse = 7153,
    k_EMsgRetrieveMatchVote = 7154,
    k_EMsgRetrieveMatchVoteResponse = 7155,
    k_EMsgTeamFanfare = 7156,
    k_EMsgResponseTeamFanfare = 7157,
    k_EMsgGC_GameServerUploadSaveGame = 7158,
    k_EMsgGC_GameServerSaveGameResult = 7159,
    k_EMsgGC_GameServerGetLoadGame = 7160,
    k_EMsgGC_GameServerGetLoadGameResult = 7161,
    k_EMsgGCEditTeamDetails = 7166,
    k_EMsgGCEditTeamDetailsResponse = 7167,
    k_EMsgGCProTeamListRequest = 7168,
    k_EMsgGCProTeamListResponse = 7169,
    k_EMsgGCReadyUpStatus = 7170,
    k_EMsgGCHallOfFame = 7171,
    k_EMsgGCHallOfFameRequest = 7172,
    k_EMsgGCHallOfFameResponse = 7173,
    k_EMsgGCGenerateDiretidePrizeList = 7174,
    k_EMsgGCRewardDiretidePrizes = 7176,
    k_EMsgGCDiretidePrizesRewardedResponse = 7177,
    k_EMsgGCHalloweenHighScoreRequest = 7178,
    k_EMsgGCHalloweenHighScoreResponse = 7179,
    k_EMsgGCGenerateDiretidePrizeListResponse = 7180,
    k_EMsgGCStorePromoPagesRequest = 7182,
    k_EMsgGCStorePromoPagesResponse = 7183,
    k_EMsgGCToGCMatchCompleted = 7186,
    k_EMsgGCBalancedShuffleLobby = 7188,
    k_EMsgGCToGCCheckLeaguePermission = 7189,
    k_EMsgGCToGCCheckLeaguePermissionResponse = 7190,
    k_EMsgGCLeagueScheduleRequest = 7191,
    k_EMsgGCLeagueScheduleResponse = 7192,
    k_EMsgGCLeagueScheduleEdit = 7193,
    k_EMsgGCLeagueScheduleEditResponse = 7194,
    k_EMsgGCLeaguesInMonthRequest = 7195,
    k_EMsgGCLeaguesInMonthResponse = 7196,
    k_EMsgGCMatchmakingStatsRequest = 7197,
    k_EMsgGCMatchmakingStatsResponse = 7198,
    k_EMsgGCBotGameCreate = 7199,
    k_EMsgGCSetMatchHistoryAccess = 7200,
    k_EMsgGCSetMatchHistoryAccessResponse = 7201,
    k_EMsgUpgradeLeagueItem = 7203,
    k_EMsgUpgradeLeagueItemResponse = 7204,
    k_EMsgGCTeamMemberProfileRequest = 7205,
    k_EMsgGCWatchDownloadedReplay = 7206,
    k_EMsgGCSetMapLocationState = 7207,
    k_EMsgGCSetMapLocationStateResponse = 7208,
    k_EMsgGCResetMapLocations = 7209,
    k_EMsgGCResetMapLocationsResponse = 7210,
    k_EMsgGCSetFeaturedItems = 7212,
    k_EMsgGCFeaturedItems = 7215,
    k_EMsgRefreshPartnerAccountLink = 7216,
    k_EMsgClientsRejoinChatChannels = 7217,
    k_EMsgGCToGCGetUserChatInfo = 7218,
    k_EMsgGCToGCGetUserChatInfoResponse = 7219,
    k_EMsgGCToGCLeaveAllChatChannels = 7220,
    k_EMsgGCToGCUpdateAccountChatBan = 7221,
    k_EMsgGCGuildCreateRequest = 7222,
    k_EMsgGCGuildCreateResponse = 7223,
    k_EMsgGCGuildSetAccountRoleRequest = 7224,
    k_EMsgGCGuildSetAccountRoleResponse = 7225,
    k_EMsgGCRequestGuildData = 7226,
    k_EMsgGCGuildData = 7227,
    k_EMsgGCGuildInviteAccountRequest = 7228,
    k_EMsgGCGuildInviteAccountResponse = 7229,
    k_EMsgGCGuildCancelInviteRequest = 7230,
    k_EMsgGCGuildCancelInviteResponse = 7231,
    k_EMsgGCGuildUpdateDetailsRequest = 7232,
    k_EMsgGCGuildUpdateDetailsResponse = 7233,
    k_EMsgGCToGCCanInviteUserToTeam = 7234,
    k_EMsgGCToGCCanInviteUserToTeamResponse = 7235,
    k_EMsgGCToGCGetUserRank = 7236,
    k_EMsgGCToGCGetUserRankResponse = 7237,
    k_EMsgGCToGCUpdateTeamStats = 7240,
    k_EMsgGCToGCGetTeamRank = 7241,
    k_EMsgGCToGCGetTeamRankResponse = 7242,
    k_EMsgGCPassportDataRequest = 7248,
    k_EMsgGCPassportDataResponse = 7249,
    k_EMsgGCNotInGuildData = 7251,
    k_EMsgGCGuildInviteData = 7254,
    k_EMsgGCToGCGetLeagueAdmin = 7255,
    k_EMsgGCToGCGetLeagueAdminResponse = 7256,
    k_EMsgGCRequestLeaguePrizePool = 7258,
    k_EMsgGCRequestLeaguePrizePoolResponse = 7259,
    k_EMsgGCToGCUpdateOpenGuildPartyRequest = 7261,
    k_EMsgGCToGCUpdateOpenGuildPartyResponse = 7262,
    k_EMsgGCToGCDestroyOpenGuildPartyRequest = 7263,
    k_EMsgGCToGCDestroyOpenGuildPartyResponse = 7264,
    k_EMsgGCGuildUpdateMessage = 7265,
    k_EMsgGCPartySetOpenGuildRequest = 7266,
    k_EMsgGCPartySetOpenGuildResponse = 7267,
    k_EMsgGCGuildOpenPartyRefresh = 7268,
    k_EMsgGCJoinOpenGuildPartyRequest = 7269,
    k_EMsgGCJoinOpenGuildPartyResponse = 7270,
    k_EMsgGCLeaveChatChannel = 7272,
    k_EMsgGCChatMessage = 7273,
    k_EMsgGCGetHeroStandings = 7274,
    k_EMsgGCGetHeroStandingsResponse = 7275,
    k_EMsgGCGuildEditLogoRequest = 7279,
    k_EMsgGCGuildEditLogoResponse = 7280,
    k_EMsgGCGuildmatePracticeLobbyListRequest = 7281,
    k_EMsgGCGuildmatePracticeLobbyListResponse = 7282,
    k_EMsgGCItemEditorReservationsRequest = 7283,
    k_EMsgGCItemEditorReservationsResponse = 7284,
    k_EMsgGCItemEditorReserveItemDef = 7285,
    k_EMsgGCItemEditorReserveItemDefResponse = 7286,
    k_EMsgGCItemEditorReleaseReservation = 7287,
    k_EMsgGCItemEditorReleaseReservationResponse = 7288,
    k_EMsgGCRewardTutorialPrizes = 7289,
    k_EMsgGCLastHitChallengeHighScorePost = 7290,
    k_EMsgGCLastHitChallengeHighScoreRequest = 7291,
    k_EMsgGCLastHitChallengeHighScoreResponse = 7292,
    k_EMsgGCCreateFantasyLeagueRequest = 7293,
    k_EMsgGCCreateFantasyLeagueResponse = 7294,
    k_EMsgGCFantasyLeagueInfoRequest = 7297,
    k_EMsgGCFantasyLeagueInfoResponse = 7298,
    k_EMsgGCFantasyLeagueInfo = 7299,
    k_EMsgGCCreateFantasyTeamRequest = 7300,
    k_EMsgGCCreateFantasyTeamResponse = 7301,
    k_EMsgGCEditFantasyTeamRequest = 7302,
    k_EMsgGCEditFantasyTeamResponse = 7303,
    k_EMsgGCFantasyTeamInfoRequestByFantasyLeagueID = 7304,
    k_EMsgGCFantasyTeamInfoRequestByOwnerAccountID = 7305,
    k_EMsgGCFantasyTeamInfoResponse = 7306,
    k_EMsgGCFantasyTeamInfo = 7307,
    k_EMsgGCFantasyLivePlayerStats = 7308,
    k_EMsgGCFantasyFinalPlayerStats = 7309,
    k_EMsgGCFantasyMatch = 7310,
    k_EMsgGCFantasyTeamScoreRequest = 7312,
    k_EMsgGCFantasyTeamScoreResponse = 7313,
    k_EMsgGCFantasyTeamStandingsRequest = 7314,
    k_EMsgGCFantasyTeamStandingsResponse = 7315,
    k_EMsgGCFantasyPlayerScoreRequest = 7316,
    k_EMsgGCFantasyPlayerScoreResponse = 7317,
    k_EMsgGCFantasyPlayerStandingsRequest = 7318,
    k_EMsgGCFantasyPlayerStandingsResponse = 7319,
    k_EMsgGCFlipLobbyTeams = 7320,
    k_EMsgGCCustomGameCreate = 7321,
    k_EMsgGCFantasyPlayerInfoRequest = 7322,
    k_EMsgGCFantasyPlayerInfoResponse = 7323,
    k_EMsgGCToGCProcessPlayerReportForTarget = 7324,
    k_EMsgGCToGCProcessReportSuccess = 7325,
    k_EMsgGCNotifyAccountFlagsChange = 7326,
    k_EMsgGCSetProfilePrivacy = 7327,
    k_EMsgGCSetProfilePrivacyResponse = 7328,
    k_EMsgGCSteamProfileRequest = 7329,
    k_EMsgGCSteamProfileRequestResponse = 7330,
    k_EMsgGCFantasyLeagueCreateInfoRequest = 7331,
    k_EMsgGCFantasyLeagueCreateInfoResponse = 7332,
    k_EMsgGCFantasyLeagueInviteInfoRequest = 7333,
    k_EMsgGCFantasyLeagueInviteInfoResponse = 7334,
    k_EMsgGCClientIgnoredUser = 7335,
    k_EMsgGCFantasyLeagueCreateRequest = 7336,
    k_EMsgGCFantasyLeagueCreateResponse = 7337,
    k_EMsgGCFantasyTeamCreateRequest = 7338,
    k_EMsgGCFantasyTeamCreateResponse = 7339,
    k_EMsgGCFantasyLeagueFriendJoinListRequest = 7340,
    k_EMsgGCFantasyLeagueFriendJoinListResponse = 7341,
    k_EMsgGCClientSuspended = 7342,
    k_EMsgGCPartyMemberSetCoach = 7343,
    k_EMsgGCFantasyLeagueEditInvitesRequest = 7344,
    k_EMsgGCFantasyLeagueEditInvitesResponse = 7345,
    k_EMsgGCPracticeLobbySetCoach = 7346,
    k_EMsgGCFantasyLeagueEditInfoRequest = 7347,
    k_EMsgGCFantasyLeagueEditInfoResponse = 7348,
    k_EMsgGCFantasyLeagueDraftStatusRequest = 7349,
    k_EMsgGCFantasyLeagueDraftStatus = 7350,
    k_EMsgGCFantasyLeagueDraftPlayerRequest = 7351,
    k_EMsgGCFantasyLeagueDraftPlayerResponse = 7352,
    k_EMsgGCFantasyLeagueMatchupsRequest = 7353,
    k_EMsgGCFantasyLeagueMatchupsResponse = 7354,
    k_EMsgGCFantasyTeamRosterSwapRequest = 7355,
    k_EMsgGCFantasyTeamRosterSwapResponse = 7356,
    k_EMsgGCFantasyTeamRosterRequest = 7357,
    k_EMsgGCFantasyTeamRosterResponse = 7358,
    k_EMsgGCNexonPartnerUpdate = 7359,
    k_EMsgGCToGCProcessPCBangRewardPoints = 7360,
    k_EMsgGCFantasyTeamRosterAddDropRequest = 7361,
    k_EMsgGCFantasyTeamRosterAddDropResponse = 7362,
    k_EMsgPresentedClientTerminateDlg = 7363,
    k_EMsgGCFantasyPlayerHisoricalStatsRequest = 7364,
    k_EMsgGCFantasyPlayerHisoricalStatsResponse = 7365,
    k_EMsgGCPCBangTimedRewardMessage = 7366,
    k_EMsgGCLobbyUpdateBroadcastChannelInfo = 7367,
    k_EMsgGCFantasyTeamTradesRequest = 7368,
    k_EMsgGCFantasyTeamTradesResponse = 7369,
    k_EMsgGCFantasyTeamTradeCancelRequest = 7370,
    k_EMsgGCFantasyTeamTradeCancelResponse = 7371,
    k_EMsgGCToGCGrantTournamentItem = 7372,
    k_EMsgGCProcessFantasyScheduledEvent = 7373,
    k_EMsgGCToGCGrantPCBangRewardItem = 7374,
    k_EMsgGCToGCUpgradeTwitchViewerItems = 7375,
    k_EMsgGCToGCGetLiveMatchAffiliates = 7376,
    k_EMsgGCToGCGetLiveMatchAffiliatesResponse = 7377,
    k_EMsgGCToGCUpdatePlayerPennantCounts = 7378,
    k_EMsgGCToGCGetPlayerPennantCounts = 7379,
    k_EMsgGCToGCGetPlayerPennantCountsResponse = 7380,
    k_EMsgGCGameMatchSignOutPermissionRequest = 7381,
    k_EMsgGCGameMatchSignOutPermissionResponse = 7382,
    k_EMsgDOTAChatChannelMemberUpdate = 7383,
    k_EMsgDOTAAwardEventPoints = 7384,
    k_EMsgDOTAGetEventPoints = 7387,
    k_EMsgDOTAGetEventPointsResponse = 7388,
    k_EMsgGCToGCSignoutAwardEventPoints = 7390,
    k_EMsgDOTASendFriendRecruits = 7393,
    k_EMsgDOTAFriendRecruitsRequest = 7394,
    k_EMsgDOTAFriendRecruitsResponse = 7395,
    k_EMsgDOTAFriendRecruitInviteAcceptDecline = 7396,
    k_EMsgGCPartyLeaderWatchGamePrompt = 7397,
    k_EMsgDOTAFrostivusTimeElapsed = 7398,
    k_EMsgDOTALiveLeagueGameUpdate = 7402,
    k_EMsgDOTAChatGetUserList = 7403,
    k_EMsgDOTAChatGetUserListResponse = 7404,
    k_EMsgGCCompendiumSetSelection = 7405,
    k_EMsgGCCompendiumDataRequest = 7406,
    k_EMsgGCCompendiumDataResponse = 7407,
    k_EMsgDOTAGetPlayerMatchHistory = 7408,
    k_EMsgDOTAGetPlayerMatchHistoryResponse = 7409,
    k_EMsgGCToGCMatchmakingAddParty = 7410,
    k_EMsgGCToGCMatchmakingRemoveParty = 7411,
    k_EMsgGCToGCMatchmakingRemoveAllParties = 7412,
    k_EMsgGCToGCMatchmakingMatchFound = 7413,
    k_EMsgGCToGCUpdateMatchManagementStats = 7414,
    k_EMsgGCToGCUpdateMatchmakingStats = 7415,
    k_EMsgGCToServerPingRequest = 7416,
    k_EMsgGCToServerPingResponse = 7417,
    k_EMsgGCToServerConsoleCommand = 7418,
    k_EMsgGCToGCUpdateLiveLeagueGameInfo = 7420,
    k_EMsgGCMakeOffering = 7423,
    k_EMsgGCRequestOfferings = 7424,
    k_EMsgGCRequestOfferingsResponse = 7425,
    k_EMsgGCToGCProcessMatchLeaver = 7426,
    k_EMsgGCNotificationsRequest = 7427,
    k_EMsgGCNotificationsResponse = 7428,
    k_EMsgGCToGCModifyNotification = 7429,
    k_EMsgGCToGCSetNewNotifications = 7430,
    k_EMsgGCToGCSetIsLeagueAdmin = 7431,
    k_EMsgGCLeagueAdminState = 7432,
    k_EMsgGCToGCSendLeagueAdminState = 7433,
    k_EMsgGCLeagueAdminList = 7434,
    k_EMsgGCNotificationsMarkReadRequest = 7435,
    k_EMsgGCFantasyMessageAdd = 7436,
    k_EMsgGCFantasyMessagesRequest = 7437,
    k_EMsgGCFantasyMessagesResponse = 7438,
    k_EMsgGCFantasyScheduledMatchesRequest = 7439,
    k_EMsgGCFantasyScheduledMatchesResponse = 7440,
    k_EMsgGCToGCGrantLeagueAccess = 7441,
    k_EMsgGCEventGameCreate = 7443,
    k_EMsgGCPerfectWorldUserLookupRequest = 7444,
    k_EMsgGCPerfectWorldUserLookupResponse = 7445,
    k_EMsgGCFantasyRemoveOwner = 7448,
    k_EMsgGCFantasyRemoveOwnerResponse = 7449,
    k_EMsgGCRequestBatchPlayerResources = 7450,
    k_EMsgGCRequestBatchPlayerResourcesResponse = 7451,
    k_EMsgGCToGCSendUpdateLeagues = 7452,
    k_EMsgGCCompendiumSetSelectionResponse = 7453,
    k_EMsgGCPlayerInfoRequest = 7454,
    k_EMsgGCPlayerInfo = 7455,
    k_EMsgGCPlayerInfoSubmit = 7456,
    k_EMsgGCPlayerInfoSubmitResponse = 7457,
    k_EMsgGCToGCGetAccountLevel = 7458,
    k_EMsgGCToGCGetAccountLevelResponse = 7459,
    k_EMsgGCToGCGetAccountPartner = 7460,
    k_EMsgGCToGCGetAccountPartnerResponse = 7461,
    k_EMsgGCToGCGetAccountProfile = 7462,
    k_EMsgGCToGCGetAccountProfileResponse = 7463,
    k_EMsgDOTAGetWeekendTourneySchedule = 7464,
    k_EMsgDOTAWeekendTourneySchedule = 7465,
    k_EMsgGCJoinableCustomGameModesRequest = 7466,
    k_EMsgGCJoinableCustomGameModesResponse = 7467,
    k_EMsgGCJoinableCustomLobbiesRequest = 7468,
    k_EMsgGCJoinableCustomLobbiesResponse = 7469,
    k_EMsgGCQuickJoinCustomLobby = 7470,
    k_EMsgGCQuickJoinCustomLobbyResponse = 7471,
    k_EMsgGCToGCGrantEventPointAction = 7472,
    k_EMsgServerGetEventPoints = 7473,
    k_EMsgServerGetEventPointsResponse = 7474,
    k_EMsgServerGrantSurveyPermission = 7475,
    k_EMsgServerGrantSurveyPermissionResponse = 7476,
    k_EMsgClientProvideSurveyResult = 7477,
    k_EMsgGCToGCSetCompendiumSelection = 7478,
    k_EMsgGCToGCUpdateTI4HeroQuest = 7480,
    k_EMsgGCCompendiumDataChanged = 7481,
    k_EMsgDOTAFantasyLeagueFindRequest = 7482,
    k_EMsgDOTAFantasyLeagueFindResponse = 7483,
    k_EMsgGCHasItemQuery = 7484,
    k_EMsgGCHasItemResponse = 7485,
    k_EMsgGCConsumeFantasyTicket = 7486,
    k_EMsgGCConsumeFantasyTicketFailure = 7487,
    k_EMsgGCToGCGrantEventPointActionMsg = 7488,
    k_EMsgClientToGCTrackDialogResult = 7489,
    k_EMsgGCFantasyLeaveLeagueRequest = 7490,
    k_EMsgGCFantasyLeaveLeagueResponse = 7491,
    k_EMsgGCToGCGetCompendiumSelections = 7492,
    k_EMsgGCToGCGetCompendiumSelectionsResponse = 7493,
    k_EMsgServerToGCMatchConnectionStats = 7494,
    k_EMsgGCToClientTournamentItemDrop = 7495,
    k_EMsgSQLDelayedGrantLeagueDrop = 7496,
    k_EMsgServerGCUpdateSpectatorCount = 7497,
    k_EMsgGCFantasyPlayerScoreDetailsRequest = 7499,
    k_EMsgGCFantasyPlayerScoreDetailsResponse = 7500,
    k_EMsgGCToGCEmoticonUnlock = 7501,
    k_EMsgSignOutDraftInfo = 7502,
    k_EMsgClientToGCEmoticonDataRequest = 7503,
    k_EMsgGCToClientEmoticonData = 7504,
    k_EMsgGCPracticeLobbyToggleBroadcastChannelCameramanStatus = 7505,
    k_EMsgGCToGCCreateWeekendTourneyRequest = 7506,
    k_EMsgGCToGCCreateWeekendTourneyResponse = 7507,
    k_EMsgClientToGCSetAdditionalEquips = 7513,
    k_EMsgClientToGCGetAdditionalEquips = 7514,
    k_EMsgClientToGCGetAdditionalEquipsResponse = 7515,
    k_EMsgServerToGCGetAdditionalEquips = 7516,
    k_EMsgServerToGCGetAdditionalEquipsResponse = 7517,
    k_EMsgDOTARedeemItem = 7518,
    k_EMsgDOTARedeemItemResponse = 7519,
    k_EMsgSQLGCToGCGrantAllHeroProgress = 7520,
    k_EMsgClientToGCGetAllHeroProgress = 7521,
    k_EMsgClientToGCGetAllHeroProgressResponse = 7522,
    k_EMsgGCToGCGetServerForClient = 7523,
    k_EMsgGCToGCGetServerForClientResponse = 7524,
    k_EMsgSQLProcessTournamentGameOutcome = 7525,
    k_EMsgSQLGrantTrophyToAccount = 7526,
    k_EMsgClientToGCGetTrophyList = 7527,
    k_EMsgClientToGCGetTrophyListResponse = 7528,
    k_EMsgGCToClientTrophyAwarded = 7529,
    k_EMsgGCGameBotMatchSignOut = 7530,
    k_EMsgGCGameBotMatchSignOutPermissionRequest = 7531,
    k_EMsgSignOutBotInfo = 7532,
    k_EMsgGCToGCUpdateProfileCards = 7533,
    k_EMsgClientToGCGetProfileCard = 7534,
    k_EMsgClientToGCGetProfileCardResponse = 7535,
    k_EMsgServerToGCGetProfileCard = 7536,
    k_EMsgServerToGCGetProfileCardResponse = 7537,
    k_EMsgClientToGCSetProfileCardSlots = 7538,
    k_EMsgGCToClientProfileCardUpdated = 7539,
    k_EMsgServerToGCVictoryPredictions = 7540,
    k_EMsgClientToGCMarkNotificationListRead = 7542,
    k_EMsgGCToClientNewNotificationAdded = 7543,
    k_EMsgServerToGCSuspiciousActivity = 7544,
    k_EMsgSignOutCommunicationSummary = 7545,
    k_EMsgServerToGCRequestStatus_Response = 7546,
    k_EMsgClientToGCCreateHeroStatue = 7547,
    k_EMsgGCToClientHeroStatueCreateResult = 7548,
    k_EMsgGCGCToLANServerRelayConnect = 7549,
    k_EMsgServerToGCGetIngameEventData = 7551,
    k_EMsgGCToGCUpdateIngameEventDataBroadcast = 7552,
    k_EMsgGCToServerIngameEventData_OraclePA = 7553,
    k_EMsgServerToGCReportKillSummaries = 7554,
    k_EMsgGCToGCReportKillSummaries = 7555,
    k_EMsgGCToGCUpdateAssassinMinigame = 7556,
    k_EMsgGCToGCFantasySetMatchLeague = 7557,
    k_EMsgClientToGCRecordCompendiumStats = 7558,
    k_EMsgGCItemEditorRequestLeagueInfo = 7559,
    k_EMsgGCItemEditorLeagueInfoResponse = 7560,
    k_EMsgGCToGCUpdatePlayerPredictions = 7561,
    k_EMsgGCToServerPredictionResult = 7562,
    k_EMsgServerToGCSignoutAwardAdditionalDrops = 7563,
    k_EMsgGCToGCSignoutAwardAdditionalDrops = 7564,
    k_EMsgGCToClientEventStatusChanged = 7565,
    k_EMsgGCHasItemDefsQuery = 7566,
    k_EMsgGCHasItemDefsResponse = 7567,
    k_EMsgGCToGCReplayMonitorValidateReplay = 7569,
    k_EMsgLobbyEventPoints = 7572,
    k_EMsgGCToGCGetCustomGameTickets = 7573,
    k_EMsgGCToGCGetCustomGameTicketsResponse = 7574,
    k_EMsgGCToGCCustomGamePlayed = 7576,
    k_EMsgGCToGCGrantEventPointsToUser = 7577,
    k_EMsgGCToGCSetEventMMPanicFlushTime = 7578,
    k_EMsgGameserverCrashReport = 7579,
    k_EMsgGameserverCrashReportResponse = 7580,
    k_EMsgGCToClientSteamDatagramTicket = 7581,
    k_EMsgGCToGCGrantEventOwnership = 7582,
    k_EMsgGCToGCSendAccountsEventPoints = 7583,
    k_EMsgClientToGCRerollPlayerChallenge = 7584,
    k_EMsgServerToGCRerollPlayerChallenge = 7585,
    k_EMsgGCRerollPlayerChallengeResponse = 7586,
    k_EMsgSignOutUpdatePlayerChallenge = 7587,
    k_EMsgClientToGCSetPartyLeader = 7588,
    k_EMsgClientToGCCancelPartyInvites = 7589,
    k_EMsgGCToGCMasterReloadAccount = 7590,
    k_EMsgSQLGrantLeagueMatchToTicketHolders = 7592,
    k_EMsgClientToGCSetAdditionalEquipsResponse = 7593,
    k_EMsgGCToGCEmoticonUnlockNoRollback = 7594,
    k_EMsgGCToGCGetCompendiumFanfare = 7595,
    k_EMsgServerToGCHoldEventPoints = 7596,
    k_EMsgSignOutReleaseEventPointHolds = 7597,
    k_EMsgGCToGCChatNewUserSession = 7598,
    k_EMsgClientToGCGetLeagueSeries = 7599,
    k_EMsgClientToGCGetLeagueSeriesResponse = 7600,
    k_EMsgSQLGCToGCSignoutUpdateLeagueSchedule = 7601,
    k_EMsgGCToServerUpdateBroadcastCheers = 7602,
    k_EMsgClientToGCApplyGemCombiner = 7603,
    k_EMsgClientToGCDOTACreateStaticRecipe = 7604,
    k_EMsgClientToGCDOTACreateStaticRecipeResponse = 7605,
    k_EMsgClientToGCGetAllHeroOrder = 7606,
    k_EMsgClientToGCGetAllHeroOrderResponse = 7607,
    k_EMsgSQLGCToGCGrantBadgePoints = 7608,
    k_EMsgGCToGCGetAccountMatchStatus = 7609,
    k_EMsgGCToGCGetAccountMatchStatusResponse = 7610,
    k_EMsgGCToGCCheckOwnsEntireEmoticonRange = 7611,
    k_EMsgGCToGCCheckOwnsEntireEmoticonRangeResponse = 7612,
    k_EMsgGCDev_GrantWarKill = 8001,
    k_EMsgServerToGCLockCharmTrading = 8004,
    k_EMsgClientToGCPlayerStatsRequest = 8006,
    k_EMsgGCToClientPlayerStatsResponse = 8007,
    k_EMsgGCClearPracticeLobbyTeam = 8008,
    k_EMsgClientToGCFindTopSourceTVGames = 8009,
    k_EMsgGCToClientFindTopSourceTVGamesResponse = 8010,
    k_EMsgGCLobbyList = 8011,
    k_EMsgGCLobbyListResponse = 8012,
    k_EMsgGCPlayerStatsMatchSignOut = 8013,
    k_EMsgClientToGCCustomGamePlayerCountRequest = 8014,
    k_EMsgGCToClientCustomGamePlayerCountResponse = 8015,
    k_EMsgClientToGCSocialFeedPostCommentRequest = 8016,
    k_EMsgGCToClientSocialFeedPostCommentResponse = 8017,
    k_EMsgClientToGCCustomGamesFriendsPlayedRequest = 8018,
    k_EMsgGCToClientCustomGamesFriendsPlayedResponse = 8019,
    k_EMsgClientToGCFriendsPlayedCustomGameRequest = 8020,
    k_EMsgGCToClientFriendsPlayedCustomGameResponse = 8021,
    k_EMsgClientToGCFeaturedHeroesRequest = 8022,
    k_EMsgGCToClientFeaturedHeroesResponse = 8023,
    k_EMsgGCTopCustomGamesList = 8024,
    k_EMsgClientToGCSocialMatchPostCommentRequest = 8025,
    k_EMsgGCToClientSocialMatchPostCommentResponse = 8026,
    k_EMsgClientToGCSocialMatchDetailsRequest = 8027,
    k_EMsgGCToClientSocialMatchDetailsResponse = 8028,
    k_EMsgClientToGCSetPartyOpen = 8029,
    k_EMsgClientToGCMergePartyInvite = 8030,
    k_EMsgGCToClientMergeGroupInviteReply = 8031,
    k_EMsgClientToGCMergePartyResponse = 8032,
    k_EMsgGCToClientMergePartyResponseReply = 8033,
    k_EMsgClientToGCGetProfileCardStats = 8034,
    k_EMsgClientToGCGetProfileCardStatsResponse = 8035,
    k_EMsgClientToGCTopLeagueMatchesRequest = 8036,
    k_EMsgClientToGCTopFriendMatchesRequest = 8037,
    k_EMsgGCToClientProfileCardStatsUpdated = 8040,
    k_EMsgServerToGCRealtimeStats = 8041,
    k_EMsgGCToServerRealtimeStatsStartStop = 8042,
    k_EMsgGCToGCGetServersForClients = 8045,
    k_EMsgGCToGCGetServersForClientsResponse = 8046,
    k_EMsgGCPracticeLobbyKickFromTeam = 8047,
    k_EMsgDOTAChatGetMemberCount = 8048,
    k_EMsgDOTAChatGetMemberCountResponse = 8049,
    k_EMsgClientToGCSocialFeedPostMessageRequest = 8050,
    k_EMsgGCToClientSocialFeedPostMessageResponse = 8051,
    k_EMsgCustomGameListenServerStartedLoading = 8052,
    k_EMsgCustomGameClientFinishedLoading = 8053,
    k_EMsgGCPracticeLobbyCloseBroadcastChannel = 8054,
    k_EMsgGCStartFindingMatchResponse = 8055,
    k_EMsgSQLGCToGCGrantAccountFlag = 8057,
    k_EMsgGCToGCGetAccountFlags = 8058,
    k_EMsgGCToGCGetAccountFlagsResponse = 8059,
    k_EMsgSignOutWagerStats = 8060,
    k_EMsgGCToClientTopLeagueMatchesResponse = 8061,
    k_EMsgGCToClientTopFriendMatchesResponse = 8062,
    k_EMsgClientToGCMatchesMinimalRequest = 8063,
    k_EMsgClientToGCMatchesMinimalResponse = 8064,
    k_EMsgGCToGCGetProfileBadgePoints = 8065,
    k_EMsgGCToGCGetProfileBadgePointsResponse = 8066,
    k_EMsgGCToClientChatRegionsEnabled = 8067,
    k_EMsgClientToGCPingData = 8068,
    k_EMsgServerToGCMatchDetailsRequest = 8069,
    k_EMsgGCToServerMatchDetailsResponse = 8070,
    k_EMsgGCToGCEnsureAccountInParty = 8071,
    k_EMsgGCToGCEnsureAccountInPartyResponse = 8072,
    k_EMsgClientToGCGetProfileTickets = 8073,
    k_EMsgClientToGCGetProfileTicketsResponse = 8074,
    k_EMsgGCToClientMatchGroupsVersion = 8075,
    k_EMsgClientToGCH264Unsupported = 8076,
    k_EMsgClientToGCRequestH264Support = 8077,
    k_EMsgClientToGCGetQuestProgress = 8078,
    k_EMsgClientToGCGetQuestProgressResponse = 8079,
    k_EMsgSignOutXPCoins = 8080,
    k_EMsgGCToClientMatchSignedOut = 8081,
    k_EMsgGCGetHeroStatsHistory = 8082,
    k_EMsgGCGetHeroStatsHistoryResponse = 8083,
    k_EMsgClientToGCPrivateChatInvite = 8084,
    k_EMsgClientToGCPrivateChatKick = 8088,
    k_EMsgClientToGCPrivateChatPromote = 8089,
    k_EMsgClientToGCPrivateChatDemote = 8090,
    k_EMsgGCToClientPrivateChatResponse = 8091,
    k_EMsgClientToGCPrivateChatInfoRequest = 8092,
    k_EMsgGCToClientPrivateChatInfoResponse = 8093,
    k_EMsgClientToGCLatestConductScorecardRequest = 8095,
    k_EMsgClientToGCLatestConductScorecard = 8096,
    k_EMsgServerToGCPostMatchTip = 8097,
    k_EMsgServerToGCPostMatchTipResponse = 8098,
    k_EMsgClientToGCWageringRequest = 8099,
    k_EMsgGCToClientWageringResponse = 8100,
    k_EMsgClientToGCEventGoalsRequest = 8103,
    k_EMsgClientToGCEventGoalsResponse = 8104,
    k_EMsgClientToGCLeaguePredictions = 8106,
    k_EMsgGCToClientLeaguePredictionsResponse = 8107,
    k_EMsgGCToGCLeaguePredictionsUpdate = 8108,
    k_EMsgClientToGCSuspiciousActivity = 8109,
    k_EMsgGCToGCAddUserToPostGameChat = 8110,
    k_EMsgClientToGCHasPlayerVotedForMVP = 8111,
    k_EMsgClientToGCHasPlayerVotedForMVPResponse = 8112,
    k_EMsgClientToGCVoteForMVP = 8113,
    k_EMsgClientToGCVoteForMVPResponse = 8114,
    k_EMsgGCToGCGetEventOwnership = 8115,
    k_EMsgGCToGCGetEventOwnershipResponse = 8116,
    k_EMsgGCToClientAutomatedTournamentStateChange = 8117,
    k_EMsgClientToGCWeekendTourneyOpts = 8118,
    k_EMsgClientToGCWeekendTourneyOptsResponse = 8119,
    k_EMsgClientToGCWeekendTourneyLeave = 8120,
    k_EMsgClientToGCWeekendTourneyLeaveResponse = 8121,
    k_EMsgClientToGCTeammateStatsRequest = 8124,
    k_EMsgClientToGCTeammateStatsResponse = 8125,
    k_EMsgClientToGCGetGiftPermissions = 8126,
    k_EMsgClientToGCGetGiftPermissionsResponse = 8127,
    k_EMsgClientToGCVoteForArcana = 8128,
    k_EMsgClientToGCVoteForArcanaResponse = 8129,
    k_EMsgClientToGCRequestArcanaVotesRemaining = 8130,
    k_EMsgClientToGCRequestArcanaVotesRemainingResponse = 8131,
    k_EMsgGCTransferTeamAdminResponse = 8132,
    k_EMsgGCChangeTeamSub = 8133,
    k_EMsgGCChangeTeamSubResponse = 8134,
    k_EMsgGCToClientTeamInfo = 8135,
    k_EMsgGCToClientTeamsInfo = 8136,
    k_EMsgClientToGCMyTeamInfoRequest = 8137,
    k_EMsgClientToGCRequestEventPointLog = 8138,
    k_EMsgClientToGCRequestEventPointLogResponse = 8139,
    k_EMsgClientToGCPublishUserStat = 8140,
    k_EMsgGCToGCSignoutSpendWager = 8141,
    k_EMsgGCSubmitLobbyMVPVote = 8144,
    k_EMsgGCSubmitLobbyMVPVoteResponse = 8145,
    k_EMsgClientToGCRequestLinaPlaysRemaining = 8146,
    k_EMsgClientToGCRequestLinaPlaysRemainingResponse = 8147,
    k_EMsgClientToGCRequestLinaGameResult = 8148,
    k_EMsgClientToGCRequestLinaGameResultResponse = 8149,
    k_EMsgSignOutCommunityGoalProgress = 8150,
    k_EMsgGCToClientLobbyMVPNotifyRecipient = 8151,
    k_EMsgGCToClientLobbyMVPAwarded = 8152,
    k_EMsgGCToClientQuestProgressUpdated = 8153,
    k_EMsgGCToClientWageringUpdate = 8154,
    k_EMsgGCToClientArcanaVotesUpdate = 8155,
    k_EMsgClientToGCAddTI6TreeProgress = 8156,
    k_EMsgClientToGCSetSpectatorLobbyDetails = 8157,
    k_EMsgClientToGCSetSpectatorLobbyDetailsResponse = 8158,
    k_EMsgClientToGCCreateSpectatorLobby = 8159,
    k_EMsgClientToGCCreateSpectatorLobbyResponse = 8160,
    k_EMsgClientToGCSpectatorLobbyList = 8161,
    k_EMsgClientToGCSpectatorLobbyListResponse = 8162,
    k_EMsgSpectatorLobbyGameDetails = 8163,
    k_EMsgServerToGCStartCompendiumInGamePredictions = 8164,
    k_EMsgServerToGCEndCompendiumInGamePredictions = 8165,
    k_EMsgServerToGCCompendiumInGamePredictionResults = 8166,
    k_EMsgServerToGCCloseCompendiumInGamePredictionVoting = 8167,
    k_EMsgClientToGCOpenPlayerCardPack = 8168,
    k_EMsgClientToGCOpenPlayerCardPackResponse = 8169,
    k_EMsgClientToGCSelectCompendiumInGamePrediction = 8170,
    k_EMsgClientToGCSelectCompendiumInGamePredictionResponse = 8171,
    k_EMsgClientToGCWeekendTourneyGetPlayerStats = 8172,
    k_EMsgClientToGCWeekendTourneyGetPlayerStatsResponse = 8173,
    k_EMsgClientToGCRecyclePlayerCard = 8174,
    k_EMsgClientToGCRecyclePlayerCardResponse = 8175,
    k_EMsgClientToGCCreatePlayerCardPack = 8176,
    k_EMsgClientToGCCreatePlayerCardPackResponse = 8177,
    k_EMsgClientToGCGetPlayerCardRosterRequest = 8178,
    k_EMsgClientToGCGetPlayerCardRosterResponse = 8179,
    k_EMsgClientToGCSetPlayerCardRosterRequest = 8180,
    k_EMsgClientToGCSetPlayerCardRosterResponse = 8181,
    k_EMsgServerToGCStartCompendiumInGamePredictionsResponse = 8182,
    k_EMsgServerToGCCloseCompendiumInGamePredictionVotingResponse = 8183,
    k_EMsgServerToGCEndCompendiumInGamePredictionsResponse = 8184,
    k_EMsgServerToGCCompendiumInGamePredictionResultsResponse = 8185,
    k_EMsgLobbyBattleCupVictory = 8186,
    k_EMsgGCGetPlayerCardItemInfo = 8187,
    k_EMsgGCGetPlayerCardItemInfoResponse = 8188,
    k_EMsgClientToGCRequestSteamDatagramTicket = 8189,
    k_EMsgClientToGCRequestSteamDatagramTicketResponse = 8190,
    k_EMsgGCToClientBattlePassRollupRequest = 8191,
    k_EMsgGCToClientBattlePassRollupResponse = 8192,
    k_EMsgClientToGCTransferSeasonalMMRRequest = 8193,
    k_EMsgClientToGCTransferSeasonalMMRResponse = 8194,
    k_EMsgGCToGCPublicChatCommunicationBan = 8195,
    k_EMsgGCToGCUpdateAccountPublicChatBan = 8196,
    k_EMsgGCChatReportPublicSpam = 8197,
    k_EMsgClientToGCSetPartyBuilderOptions = 8198,
    k_EMsgClientToGCSetPartyBuilderOptionsResponse = 8199,
    k_EMsgGCToClientPlaytestStatus = 8200,
    k_EMsgClientToGCJoinPlaytest = 8201,
    k_EMsgClientToGCJoinPlaytestResponse = 8202,
    k_EMsgLobbyPlaytestDetails = 8203,
    k_EMsgDOTASetFavoriteTeam = 8204,
    k_EMsgGCToClientBattlePassRollupListRequest = 8205,
    k_EMsgGCToClientBattlePassRollupListResponse = 8206,
    k_EMsgGCIsProQuery = 8207,
    k_EMsgGCIsProResponse = 8208,
    k_EMsgDOTAClaimEventAction = 8209,
    k_EMsgDOTAClaimEventActionResponse = 8210,
    k_EMsgDOTAGetPeriodicResource = 8211,
    k_EMsgDOTAGetPeriodicResourceResponse = 8212,
    k_EMsgDOTAPeriodicResourceUpdated = 8213,
    k_EMsgServerToGCSpendWager = 8214,
    k_EMsgGCToGCSignoutSpendWagerToken = 8215,
    k_EMsgSubmitTriviaQuestionAnswer = 8216,
    k_EMsgSubmitTriviaQuestionAnswerResponse = 8217,
    k_EMsgClientToGCGiveTip = 8218,
    k_EMsgClientToGCGiveTipResponse = 8219,
    k_EMsgStartTriviaSession = 8220,
    k_EMsgStartTriviaSessionResponse = 8221,
    k_EMsgAnchorPhoneNumberRequest = 8222,
    k_EMsgAnchorPhoneNumberResponse = 8223,
    k_EMsgUnanchorPhoneNumberRequest = 8224,
    k_EMsgUnanchorPhoneNumberResponse = 8225,
    k_EMsgGCToClientTipNotification = 8226,
    k_EMsgClientToGCRequestSlarkGameResult = 8227,
    k_EMsgClientToGCRequestSlarkGameResultResponse = 8228,
    k_EMsgGCToGCSignoutSpendRankWager = 8229,
    k_EMsgGCToGCGetFavoriteTeam = 8230,
    k_EMsgGCToGCGetFavoriteTeamResponse = 8231,
    k_EMsgSignOutEventGameData = 8232,
    k_EMsgGCToClientAllStarVotesRequest = 8233,
    k_EMsgGCToClientAllStarVotesReply = 8234,
    k_EMsgGCToClientAllStarVotesSubmit = 8236,
    k_EMsgGCToClientAllStarVotesSubmitReply = 8237,
    k_EMsgClientToGCQuickStatsRequest = 8238,
    k_EMsgClientToGCQuickStatsResponse = 8239,
    k_EMsgGCToGCSubtractEventPointsFromUser = 8240,
    k_EMsgSelectionPriorityChoiceRequest = 8241,
    k_EMsgSelectionPriorityChoiceResponse = 8242,
    k_EMsgGCToGCCompendiumInGamePredictionResults = 8243,
    k_EMsgGameAutographReward = 8244,
    k_EMsgGameAutographRewardResponse = 8245,
    k_EMsgServerToGCMatchPlayerItemPurchaseHistory = 8250,
}

impl ::protobuf::ProtobufEnum for EDOTAGCMsg {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EDOTAGCMsg> {
        match value {
            7000 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCDOTABase),
            7001 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGeneralResponse),
            7004 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGameMatchSignOut),
            7005 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGameMatchSignOutResponse),
            7009 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCJoinChatChannel),
            7010 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCJoinChatChannelResponse),
            7013 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCOtherJoinedChannel),
            7014 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCOtherLeftChannel),
            7017 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCMatchHistoryList),
            7026 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCRequestStatus),
            7027 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGetRecentMatches),
            7028 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRecentMatchesResponse),
            7033 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCStartFindingMatch),
            7034 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCConnectedPlayers),
            7035 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCAbandonCurrentGame),
            7036 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCStopFindingMatch),
            7038 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbyCreate),
            7040 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbyLeave),
            7041 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbyLaunch),
            7042 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbyList),
            7043 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbyListResponse),
            7044 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbyJoin),
            7046 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbySetDetails),
            7047 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbySetTeamSlot),
            7049 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCInitialQuestionnaireResponse),
            7055 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbyResponse),
            7056 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCBroadcastNotification),
            7057 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLiveScoreboardUpdate),
            7060 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestChatChannelList),
            7061 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestChatChannelListResponse),
            7064 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestMatches),
            7065 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestMatchesResponse),
            7068 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestPlayerResources),
            7069 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestPlayerResourcesResponse),
            7070 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCReadyUp),
            7071 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCKickedFromMatchmakingQueue),
            7072 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLeaverDetected),
            7073 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSpectateFriendGame),
            7074 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSpectateFriendGameResponse),
            7075 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPlayerReports),
            7076 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCReportsRemainingRequest),
            7077 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCReportsRemainingResponse),
            7078 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSubmitPlayerReport),
            7079 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSubmitPlayerReportResponse),
            7081 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbyKick),
            7082 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCReportCountsRequest),
            7083 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCReportCountsResponse),
            7084 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestSaveGames),
            7085 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestSaveGamesServer),
            7086 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestSaveGamesResponse),
            7087 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLeaverDetectedResponse),
            7088 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPlayerFailedToConnect),
            7089 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGCToRelayConnect),
            7090 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGCToRelayConnectresponse),
            7091 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCWatchGame),
            7092 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCWatchGameResponse),
            7093 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCBanStatusRequest),
            7094 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCBanStatusResponse),
            7095 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCMatchDetailsRequest),
            7096 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCMatchDetailsResponse),
            7097 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCCancelWatchGame),
            7098 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCProfileRequest),
            7099 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCProfileResponse),
            7102 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPopup),
            7104 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCDOTAClearNotifySuccessfulReport),
            7111 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFriendPracticeLobbyListRequest),
            7112 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFriendPracticeLobbyListResponse),
            7113 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbyJoinResponse),
            7114 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientEconNotification_Job),
            7115 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCCreateTeam),
            7116 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCCreateTeamResponse),
            7121 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCTeamData),
            7122 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCTeamInvite_InviterToGC),
            7123 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCTeamInvite_GCImmediateResponseToInviter),
            7124 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCTeamInvite_GCRequestToInvitee),
            7125 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCTeamInvite_InviteeResponseToGC),
            7126 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCTeamInvite_GCResponseToInviter),
            7127 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCTeamInvite_GCResponseToInvitee),
            7128 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCKickTeamMember),
            7129 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCKickTeamMemberResponse),
            7130 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLeaveTeam),
            7131 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLeaveTeamResponse),
            7132 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSuggestTeamMatchmaking),
            7133 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPlayerHeroesFavoritesAdd),
            7134 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPlayerHeroesFavoritesRemove),
            7141 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSetShowcaseHero),
            7142 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCApplyTeamToPracticeLobby),
            7143 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestInternatinalTicketEmail),
            7144 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCTransferTeamAdmin),
            7147 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgRequestLeagueInfo),
            7148 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgResponseLeagueInfo),
            7149 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbyJoinBroadcastChannel),
            7150 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGC_TournamentItemEvent),
            7151 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGC_TournamentItemEventResponse),
            7152 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgCastMatchVote),
            7153 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgCastMatchVoteResponse),
            7154 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgRetrieveMatchVote),
            7155 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgRetrieveMatchVoteResponse),
            7156 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgTeamFanfare),
            7157 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgResponseTeamFanfare),
            7158 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGC_GameServerUploadSaveGame),
            7159 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGC_GameServerSaveGameResult),
            7160 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGC_GameServerGetLoadGame),
            7161 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGC_GameServerGetLoadGameResult),
            7166 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCEditTeamDetails),
            7167 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCEditTeamDetailsResponse),
            7168 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCProTeamListRequest),
            7169 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCProTeamListResponse),
            7170 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCReadyUpStatus),
            7171 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCHallOfFame),
            7172 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCHallOfFameRequest),
            7173 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCHallOfFameResponse),
            7174 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGenerateDiretidePrizeList),
            7176 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRewardDiretidePrizes),
            7177 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCDiretidePrizesRewardedResponse),
            7178 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCHalloweenHighScoreRequest),
            7179 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCHalloweenHighScoreResponse),
            7180 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGenerateDiretidePrizeListResponse),
            7182 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCStorePromoPagesRequest),
            7183 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCStorePromoPagesResponse),
            7186 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCMatchCompleted),
            7188 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCBalancedShuffleLobby),
            7189 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCCheckLeaguePermission),
            7190 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCCheckLeaguePermissionResponse),
            7191 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLeagueScheduleRequest),
            7192 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLeagueScheduleResponse),
            7193 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLeagueScheduleEdit),
            7194 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLeagueScheduleEditResponse),
            7195 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLeaguesInMonthRequest),
            7196 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLeaguesInMonthResponse),
            7197 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCMatchmakingStatsRequest),
            7198 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCMatchmakingStatsResponse),
            7199 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCBotGameCreate),
            7200 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSetMatchHistoryAccess),
            7201 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSetMatchHistoryAccessResponse),
            7203 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgUpgradeLeagueItem),
            7204 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgUpgradeLeagueItemResponse),
            7205 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCTeamMemberProfileRequest),
            7206 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCWatchDownloadedReplay),
            7207 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSetMapLocationState),
            7208 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSetMapLocationStateResponse),
            7209 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCResetMapLocations),
            7210 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCResetMapLocationsResponse),
            7212 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSetFeaturedItems),
            7215 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFeaturedItems),
            7216 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgRefreshPartnerAccountLink),
            7217 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientsRejoinChatChannels),
            7218 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetUserChatInfo),
            7219 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetUserChatInfoResponse),
            7220 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCLeaveAllChatChannels),
            7221 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCUpdateAccountChatBan),
            7222 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildCreateRequest),
            7223 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildCreateResponse),
            7224 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildSetAccountRoleRequest),
            7225 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildSetAccountRoleResponse),
            7226 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestGuildData),
            7227 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildData),
            7228 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildInviteAccountRequest),
            7229 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildInviteAccountResponse),
            7230 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildCancelInviteRequest),
            7231 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildCancelInviteResponse),
            7232 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildUpdateDetailsRequest),
            7233 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildUpdateDetailsResponse),
            7234 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCCanInviteUserToTeam),
            7235 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCCanInviteUserToTeamResponse),
            7236 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetUserRank),
            7237 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetUserRankResponse),
            7240 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCUpdateTeamStats),
            7241 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetTeamRank),
            7242 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetTeamRankResponse),
            7248 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPassportDataRequest),
            7249 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPassportDataResponse),
            7251 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCNotInGuildData),
            7254 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildInviteData),
            7255 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetLeagueAdmin),
            7256 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetLeagueAdminResponse),
            7258 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestLeaguePrizePool),
            7259 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestLeaguePrizePoolResponse),
            7261 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCUpdateOpenGuildPartyRequest),
            7262 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCUpdateOpenGuildPartyResponse),
            7263 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCDestroyOpenGuildPartyRequest),
            7264 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCDestroyOpenGuildPartyResponse),
            7265 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildUpdateMessage),
            7266 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPartySetOpenGuildRequest),
            7267 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPartySetOpenGuildResponse),
            7268 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildOpenPartyRefresh),
            7269 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCJoinOpenGuildPartyRequest),
            7270 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCJoinOpenGuildPartyResponse),
            7272 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLeaveChatChannel),
            7273 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCChatMessage),
            7274 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGetHeroStandings),
            7275 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGetHeroStandingsResponse),
            7279 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildEditLogoRequest),
            7280 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildEditLogoResponse),
            7281 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildmatePracticeLobbyListRequest),
            7282 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGuildmatePracticeLobbyListResponse),
            7283 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCItemEditorReservationsRequest),
            7284 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCItemEditorReservationsResponse),
            7285 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCItemEditorReserveItemDef),
            7286 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCItemEditorReserveItemDefResponse),
            7287 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCItemEditorReleaseReservation),
            7288 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCItemEditorReleaseReservationResponse),
            7289 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRewardTutorialPrizes),
            7290 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLastHitChallengeHighScorePost),
            7291 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLastHitChallengeHighScoreRequest),
            7292 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLastHitChallengeHighScoreResponse),
            7293 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCCreateFantasyLeagueRequest),
            7294 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCCreateFantasyLeagueResponse),
            7297 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueInfoRequest),
            7298 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueInfoResponse),
            7299 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueInfo),
            7300 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCCreateFantasyTeamRequest),
            7301 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCCreateFantasyTeamResponse),
            7302 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCEditFantasyTeamRequest),
            7303 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCEditFantasyTeamResponse),
            7304 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamInfoRequestByFantasyLeagueID),
            7305 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamInfoRequestByOwnerAccountID),
            7306 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamInfoResponse),
            7307 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamInfo),
            7308 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLivePlayerStats),
            7309 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyFinalPlayerStats),
            7310 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyMatch),
            7312 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamScoreRequest),
            7313 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamScoreResponse),
            7314 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamStandingsRequest),
            7315 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamStandingsResponse),
            7316 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyPlayerScoreRequest),
            7317 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyPlayerScoreResponse),
            7318 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyPlayerStandingsRequest),
            7319 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyPlayerStandingsResponse),
            7320 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFlipLobbyTeams),
            7321 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCCustomGameCreate),
            7322 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyPlayerInfoRequest),
            7323 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyPlayerInfoResponse),
            7324 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCProcessPlayerReportForTarget),
            7325 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCProcessReportSuccess),
            7326 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCNotifyAccountFlagsChange),
            7327 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSetProfilePrivacy),
            7328 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSetProfilePrivacyResponse),
            7329 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSteamProfileRequest),
            7330 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSteamProfileRequestResponse),
            7331 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueCreateInfoRequest),
            7332 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueCreateInfoResponse),
            7333 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueInviteInfoRequest),
            7334 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueInviteInfoResponse),
            7335 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCClientIgnoredUser),
            7336 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueCreateRequest),
            7337 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueCreateResponse),
            7338 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamCreateRequest),
            7339 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamCreateResponse),
            7340 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueFriendJoinListRequest),
            7341 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueFriendJoinListResponse),
            7342 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCClientSuspended),
            7343 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPartyMemberSetCoach),
            7344 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueEditInvitesRequest),
            7345 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueEditInvitesResponse),
            7346 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbySetCoach),
            7347 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueEditInfoRequest),
            7348 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueEditInfoResponse),
            7349 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueDraftStatusRequest),
            7350 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueDraftStatus),
            7351 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueDraftPlayerRequest),
            7352 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueDraftPlayerResponse),
            7353 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueMatchupsRequest),
            7354 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeagueMatchupsResponse),
            7355 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamRosterSwapRequest),
            7356 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamRosterSwapResponse),
            7357 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamRosterRequest),
            7358 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamRosterResponse),
            7359 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCNexonPartnerUpdate),
            7360 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCProcessPCBangRewardPoints),
            7361 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamRosterAddDropRequest),
            7362 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamRosterAddDropResponse),
            7363 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgPresentedClientTerminateDlg),
            7364 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyPlayerHisoricalStatsRequest),
            7365 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyPlayerHisoricalStatsResponse),
            7366 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPCBangTimedRewardMessage),
            7367 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLobbyUpdateBroadcastChannelInfo),
            7368 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamTradesRequest),
            7369 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamTradesResponse),
            7370 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamTradeCancelRequest),
            7371 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyTeamTradeCancelResponse),
            7372 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGrantTournamentItem),
            7373 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCProcessFantasyScheduledEvent),
            7374 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGrantPCBangRewardItem),
            7375 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCUpgradeTwitchViewerItems),
            7376 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetLiveMatchAffiliates),
            7377 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetLiveMatchAffiliatesResponse),
            7378 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCUpdatePlayerPennantCounts),
            7379 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetPlayerPennantCounts),
            7380 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetPlayerPennantCountsResponse),
            7381 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGameMatchSignOutPermissionRequest),
            7382 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGameMatchSignOutPermissionResponse),
            7383 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAChatChannelMemberUpdate),
            7384 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAAwardEventPoints),
            7387 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAGetEventPoints),
            7388 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAGetEventPointsResponse),
            7390 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCSignoutAwardEventPoints),
            7393 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTASendFriendRecruits),
            7394 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAFriendRecruitsRequest),
            7395 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAFriendRecruitsResponse),
            7396 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAFriendRecruitInviteAcceptDecline),
            7397 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPartyLeaderWatchGamePrompt),
            7398 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAFrostivusTimeElapsed),
            7402 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTALiveLeagueGameUpdate),
            7403 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAChatGetUserList),
            7404 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAChatGetUserListResponse),
            7405 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCCompendiumSetSelection),
            7406 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCCompendiumDataRequest),
            7407 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCCompendiumDataResponse),
            7408 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAGetPlayerMatchHistory),
            7409 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAGetPlayerMatchHistoryResponse),
            7410 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCMatchmakingAddParty),
            7411 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCMatchmakingRemoveParty),
            7412 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCMatchmakingRemoveAllParties),
            7413 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCMatchmakingMatchFound),
            7414 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCUpdateMatchManagementStats),
            7415 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCUpdateMatchmakingStats),
            7416 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToServerPingRequest),
            7417 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToServerPingResponse),
            7418 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToServerConsoleCommand),
            7420 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCUpdateLiveLeagueGameInfo),
            7423 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCMakeOffering),
            7424 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestOfferings),
            7425 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestOfferingsResponse),
            7426 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCProcessMatchLeaver),
            7427 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCNotificationsRequest),
            7428 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCNotificationsResponse),
            7429 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCModifyNotification),
            7430 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCSetNewNotifications),
            7431 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCSetIsLeagueAdmin),
            7432 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLeagueAdminState),
            7433 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCSendLeagueAdminState),
            7434 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLeagueAdminList),
            7435 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCNotificationsMarkReadRequest),
            7436 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyMessageAdd),
            7437 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyMessagesRequest),
            7438 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyMessagesResponse),
            7439 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyScheduledMatchesRequest),
            7440 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyScheduledMatchesResponse),
            7441 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGrantLeagueAccess),
            7443 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCEventGameCreate),
            7444 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPerfectWorldUserLookupRequest),
            7445 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPerfectWorldUserLookupResponse),
            7448 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyRemoveOwner),
            7449 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyRemoveOwnerResponse),
            7450 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestBatchPlayerResources),
            7451 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRequestBatchPlayerResourcesResponse),
            7452 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCSendUpdateLeagues),
            7453 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCCompendiumSetSelectionResponse),
            7454 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPlayerInfoRequest),
            7455 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPlayerInfo),
            7456 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPlayerInfoSubmit),
            7457 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPlayerInfoSubmitResponse),
            7458 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetAccountLevel),
            7459 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetAccountLevelResponse),
            7460 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetAccountPartner),
            7461 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetAccountPartnerResponse),
            7462 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetAccountProfile),
            7463 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetAccountProfileResponse),
            7464 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAGetWeekendTourneySchedule),
            7465 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAWeekendTourneySchedule),
            7466 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCJoinableCustomGameModesRequest),
            7467 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCJoinableCustomGameModesResponse),
            7468 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCJoinableCustomLobbiesRequest),
            7469 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCJoinableCustomLobbiesResponse),
            7470 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCQuickJoinCustomLobby),
            7471 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCQuickJoinCustomLobbyResponse),
            7472 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGrantEventPointAction),
            7473 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerGetEventPoints),
            7474 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerGetEventPointsResponse),
            7475 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerGrantSurveyPermission),
            7476 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerGrantSurveyPermissionResponse),
            7477 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientProvideSurveyResult),
            7478 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCSetCompendiumSelection),
            7480 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCUpdateTI4HeroQuest),
            7481 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCCompendiumDataChanged),
            7482 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAFantasyLeagueFindRequest),
            7483 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAFantasyLeagueFindResponse),
            7484 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCHasItemQuery),
            7485 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCHasItemResponse),
            7486 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCConsumeFantasyTicket),
            7487 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCConsumeFantasyTicketFailure),
            7488 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGrantEventPointActionMsg),
            7489 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCTrackDialogResult),
            7490 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeaveLeagueRequest),
            7491 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyLeaveLeagueResponse),
            7492 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetCompendiumSelections),
            7493 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetCompendiumSelectionsResponse),
            7494 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCMatchConnectionStats),
            7495 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientTournamentItemDrop),
            7496 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSQLDelayedGrantLeagueDrop),
            7497 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerGCUpdateSpectatorCount),
            7499 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyPlayerScoreDetailsRequest),
            7500 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCFantasyPlayerScoreDetailsResponse),
            7501 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCEmoticonUnlock),
            7502 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSignOutDraftInfo),
            7503 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCEmoticonDataRequest),
            7504 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientEmoticonData),
            7505 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbyToggleBroadcastChannelCameramanStatus),
            7506 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCCreateWeekendTourneyRequest),
            7507 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCCreateWeekendTourneyResponse),
            7513 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSetAdditionalEquips),
            7514 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetAdditionalEquips),
            7515 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetAdditionalEquipsResponse),
            7516 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCGetAdditionalEquips),
            7517 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCGetAdditionalEquipsResponse),
            7518 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTARedeemItem),
            7519 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTARedeemItemResponse),
            7520 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSQLGCToGCGrantAllHeroProgress),
            7521 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetAllHeroProgress),
            7522 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetAllHeroProgressResponse),
            7523 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetServerForClient),
            7524 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetServerForClientResponse),
            7525 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSQLProcessTournamentGameOutcome),
            7526 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSQLGrantTrophyToAccount),
            7527 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetTrophyList),
            7528 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetTrophyListResponse),
            7529 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientTrophyAwarded),
            7530 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGameBotMatchSignOut),
            7531 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGameBotMatchSignOutPermissionRequest),
            7532 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSignOutBotInfo),
            7533 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCUpdateProfileCards),
            7534 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetProfileCard),
            7535 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetProfileCardResponse),
            7536 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCGetProfileCard),
            7537 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCGetProfileCardResponse),
            7538 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSetProfileCardSlots),
            7539 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientProfileCardUpdated),
            7540 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCVictoryPredictions),
            7542 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCMarkNotificationListRead),
            7543 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientNewNotificationAdded),
            7544 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCSuspiciousActivity),
            7545 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSignOutCommunicationSummary),
            7546 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCRequestStatus_Response),
            7547 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCCreateHeroStatue),
            7548 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientHeroStatueCreateResult),
            7549 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGCToLANServerRelayConnect),
            7551 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCGetIngameEventData),
            7552 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCUpdateIngameEventDataBroadcast),
            7553 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToServerIngameEventData_OraclePA),
            7554 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCReportKillSummaries),
            7555 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCReportKillSummaries),
            7556 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCUpdateAssassinMinigame),
            7557 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCFantasySetMatchLeague),
            7558 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRecordCompendiumStats),
            7559 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCItemEditorRequestLeagueInfo),
            7560 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCItemEditorLeagueInfoResponse),
            7561 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCUpdatePlayerPredictions),
            7562 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToServerPredictionResult),
            7563 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCSignoutAwardAdditionalDrops),
            7564 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCSignoutAwardAdditionalDrops),
            7565 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientEventStatusChanged),
            7566 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCHasItemDefsQuery),
            7567 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCHasItemDefsResponse),
            7569 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCReplayMonitorValidateReplay),
            7572 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgLobbyEventPoints),
            7573 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetCustomGameTickets),
            7574 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetCustomGameTicketsResponse),
            7576 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCCustomGamePlayed),
            7577 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGrantEventPointsToUser),
            7578 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCSetEventMMPanicFlushTime),
            7579 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGameserverCrashReport),
            7580 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGameserverCrashReportResponse),
            7581 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientSteamDatagramTicket),
            7582 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGrantEventOwnership),
            7583 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCSendAccountsEventPoints),
            7584 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRerollPlayerChallenge),
            7585 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCRerollPlayerChallenge),
            7586 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCRerollPlayerChallengeResponse),
            7587 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSignOutUpdatePlayerChallenge),
            7588 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSetPartyLeader),
            7589 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCCancelPartyInvites),
            7590 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCMasterReloadAccount),
            7592 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSQLGrantLeagueMatchToTicketHolders),
            7593 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSetAdditionalEquipsResponse),
            7594 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCEmoticonUnlockNoRollback),
            7595 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetCompendiumFanfare),
            7596 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCHoldEventPoints),
            7597 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSignOutReleaseEventPointHolds),
            7598 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCChatNewUserSession),
            7599 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetLeagueSeries),
            7600 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetLeagueSeriesResponse),
            7601 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSQLGCToGCSignoutUpdateLeagueSchedule),
            7602 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToServerUpdateBroadcastCheers),
            7603 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCApplyGemCombiner),
            7604 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCDOTACreateStaticRecipe),
            7605 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCDOTACreateStaticRecipeResponse),
            7606 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetAllHeroOrder),
            7607 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetAllHeroOrderResponse),
            7608 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSQLGCToGCGrantBadgePoints),
            7609 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetAccountMatchStatus),
            7610 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetAccountMatchStatusResponse),
            7611 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCCheckOwnsEntireEmoticonRange),
            7612 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCCheckOwnsEntireEmoticonRangeResponse),
            8001 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCDev_GrantWarKill),
            8004 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCLockCharmTrading),
            8006 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCPlayerStatsRequest),
            8007 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientPlayerStatsResponse),
            8008 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCClearPracticeLobbyTeam),
            8009 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCFindTopSourceTVGames),
            8010 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientFindTopSourceTVGamesResponse),
            8011 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLobbyList),
            8012 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCLobbyListResponse),
            8013 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPlayerStatsMatchSignOut),
            8014 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCCustomGamePlayerCountRequest),
            8015 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientCustomGamePlayerCountResponse),
            8016 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSocialFeedPostCommentRequest),
            8017 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientSocialFeedPostCommentResponse),
            8018 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCCustomGamesFriendsPlayedRequest),
            8019 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientCustomGamesFriendsPlayedResponse),
            8020 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCFriendsPlayedCustomGameRequest),
            8021 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientFriendsPlayedCustomGameResponse),
            8022 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCFeaturedHeroesRequest),
            8023 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientFeaturedHeroesResponse),
            8024 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCTopCustomGamesList),
            8025 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSocialMatchPostCommentRequest),
            8026 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientSocialMatchPostCommentResponse),
            8027 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSocialMatchDetailsRequest),
            8028 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientSocialMatchDetailsResponse),
            8029 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSetPartyOpen),
            8030 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCMergePartyInvite),
            8031 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientMergeGroupInviteReply),
            8032 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCMergePartyResponse),
            8033 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientMergePartyResponseReply),
            8034 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetProfileCardStats),
            8035 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetProfileCardStatsResponse),
            8036 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCTopLeagueMatchesRequest),
            8037 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCTopFriendMatchesRequest),
            8040 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientProfileCardStatsUpdated),
            8041 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCRealtimeStats),
            8042 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToServerRealtimeStatsStartStop),
            8045 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetServersForClients),
            8046 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetServersForClientsResponse),
            8047 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbyKickFromTeam),
            8048 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAChatGetMemberCount),
            8049 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAChatGetMemberCountResponse),
            8050 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSocialFeedPostMessageRequest),
            8051 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientSocialFeedPostMessageResponse),
            8052 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgCustomGameListenServerStartedLoading),
            8053 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgCustomGameClientFinishedLoading),
            8054 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCPracticeLobbyCloseBroadcastChannel),
            8055 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCStartFindingMatchResponse),
            8057 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSQLGCToGCGrantAccountFlag),
            8058 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetAccountFlags),
            8059 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetAccountFlagsResponse),
            8060 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSignOutWagerStats),
            8061 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientTopLeagueMatchesResponse),
            8062 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientTopFriendMatchesResponse),
            8063 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCMatchesMinimalRequest),
            8064 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCMatchesMinimalResponse),
            8065 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetProfileBadgePoints),
            8066 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetProfileBadgePointsResponse),
            8067 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientChatRegionsEnabled),
            8068 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCPingData),
            8069 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCMatchDetailsRequest),
            8070 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToServerMatchDetailsResponse),
            8071 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCEnsureAccountInParty),
            8072 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCEnsureAccountInPartyResponse),
            8073 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetProfileTickets),
            8074 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetProfileTicketsResponse),
            8075 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientMatchGroupsVersion),
            8076 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCH264Unsupported),
            8077 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRequestH264Support),
            8078 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetQuestProgress),
            8079 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetQuestProgressResponse),
            8080 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSignOutXPCoins),
            8081 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientMatchSignedOut),
            8082 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGetHeroStatsHistory),
            8083 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGetHeroStatsHistoryResponse),
            8084 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCPrivateChatInvite),
            8088 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCPrivateChatKick),
            8089 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCPrivateChatPromote),
            8090 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCPrivateChatDemote),
            8091 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientPrivateChatResponse),
            8092 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCPrivateChatInfoRequest),
            8093 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientPrivateChatInfoResponse),
            8095 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCLatestConductScorecardRequest),
            8096 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCLatestConductScorecard),
            8097 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCPostMatchTip),
            8098 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCPostMatchTipResponse),
            8099 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCWageringRequest),
            8100 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientWageringResponse),
            8103 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCEventGoalsRequest),
            8104 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCEventGoalsResponse),
            8106 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCLeaguePredictions),
            8107 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientLeaguePredictionsResponse),
            8108 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCLeaguePredictionsUpdate),
            8109 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSuspiciousActivity),
            8110 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCAddUserToPostGameChat),
            8111 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCHasPlayerVotedForMVP),
            8112 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCHasPlayerVotedForMVPResponse),
            8113 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCVoteForMVP),
            8114 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCVoteForMVPResponse),
            8115 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetEventOwnership),
            8116 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetEventOwnershipResponse),
            8117 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientAutomatedTournamentStateChange),
            8118 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCWeekendTourneyOpts),
            8119 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCWeekendTourneyOptsResponse),
            8120 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCWeekendTourneyLeave),
            8121 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCWeekendTourneyLeaveResponse),
            8124 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCTeammateStatsRequest),
            8125 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCTeammateStatsResponse),
            8126 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetGiftPermissions),
            8127 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetGiftPermissionsResponse),
            8128 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCVoteForArcana),
            8129 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCVoteForArcanaResponse),
            8130 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRequestArcanaVotesRemaining),
            8131 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRequestArcanaVotesRemainingResponse),
            8132 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCTransferTeamAdminResponse),
            8133 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCChangeTeamSub),
            8134 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCChangeTeamSubResponse),
            8135 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientTeamInfo),
            8136 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientTeamsInfo),
            8137 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCMyTeamInfoRequest),
            8138 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRequestEventPointLog),
            8139 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRequestEventPointLogResponse),
            8140 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCPublishUserStat),
            8141 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCSignoutSpendWager),
            8144 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSubmitLobbyMVPVote),
            8145 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCSubmitLobbyMVPVoteResponse),
            8146 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRequestLinaPlaysRemaining),
            8147 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRequestLinaPlaysRemainingResponse),
            8148 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRequestLinaGameResult),
            8149 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRequestLinaGameResultResponse),
            8150 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSignOutCommunityGoalProgress),
            8151 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientLobbyMVPNotifyRecipient),
            8152 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientLobbyMVPAwarded),
            8153 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientQuestProgressUpdated),
            8154 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientWageringUpdate),
            8155 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientArcanaVotesUpdate),
            8156 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCAddTI6TreeProgress),
            8157 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSetSpectatorLobbyDetails),
            8158 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSetSpectatorLobbyDetailsResponse),
            8159 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCCreateSpectatorLobby),
            8160 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCCreateSpectatorLobbyResponse),
            8161 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSpectatorLobbyList),
            8162 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSpectatorLobbyListResponse),
            8163 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSpectatorLobbyGameDetails),
            8164 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCStartCompendiumInGamePredictions),
            8165 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCEndCompendiumInGamePredictions),
            8166 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCCompendiumInGamePredictionResults),
            8167 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCCloseCompendiumInGamePredictionVoting),
            8168 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCOpenPlayerCardPack),
            8169 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCOpenPlayerCardPackResponse),
            8170 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSelectCompendiumInGamePrediction),
            8171 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSelectCompendiumInGamePredictionResponse),
            8172 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCWeekendTourneyGetPlayerStats),
            8173 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCWeekendTourneyGetPlayerStatsResponse),
            8174 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRecyclePlayerCard),
            8175 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRecyclePlayerCardResponse),
            8176 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCCreatePlayerCardPack),
            8177 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCCreatePlayerCardPackResponse),
            8178 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetPlayerCardRosterRequest),
            8179 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGetPlayerCardRosterResponse),
            8180 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSetPlayerCardRosterRequest),
            8181 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSetPlayerCardRosterResponse),
            8182 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCStartCompendiumInGamePredictionsResponse),
            8183 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCCloseCompendiumInGamePredictionVotingResponse),
            8184 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCEndCompendiumInGamePredictionsResponse),
            8185 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCCompendiumInGamePredictionResultsResponse),
            8186 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgLobbyBattleCupVictory),
            8187 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGetPlayerCardItemInfo),
            8188 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCGetPlayerCardItemInfoResponse),
            8189 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRequestSteamDatagramTicket),
            8190 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRequestSteamDatagramTicketResponse),
            8191 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientBattlePassRollupRequest),
            8192 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientBattlePassRollupResponse),
            8193 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCTransferSeasonalMMRRequest),
            8194 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCTransferSeasonalMMRResponse),
            8195 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCPublicChatCommunicationBan),
            8196 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCUpdateAccountPublicChatBan),
            8197 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCChatReportPublicSpam),
            8198 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSetPartyBuilderOptions),
            8199 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCSetPartyBuilderOptionsResponse),
            8200 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientPlaytestStatus),
            8201 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCJoinPlaytest),
            8202 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCJoinPlaytestResponse),
            8203 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgLobbyPlaytestDetails),
            8204 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTASetFavoriteTeam),
            8205 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientBattlePassRollupListRequest),
            8206 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientBattlePassRollupListResponse),
            8207 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCIsProQuery),
            8208 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCIsProResponse),
            8209 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAClaimEventAction),
            8210 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAClaimEventActionResponse),
            8211 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAGetPeriodicResource),
            8212 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAGetPeriodicResourceResponse),
            8213 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgDOTAPeriodicResourceUpdated),
            8214 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCSpendWager),
            8215 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCSignoutSpendWagerToken),
            8216 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSubmitTriviaQuestionAnswer),
            8217 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSubmitTriviaQuestionAnswerResponse),
            8218 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGiveTip),
            8219 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCGiveTipResponse),
            8220 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgStartTriviaSession),
            8221 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgStartTriviaSessionResponse),
            8222 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgAnchorPhoneNumberRequest),
            8223 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgAnchorPhoneNumberResponse),
            8224 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgUnanchorPhoneNumberRequest),
            8225 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgUnanchorPhoneNumberResponse),
            8226 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientTipNotification),
            8227 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRequestSlarkGameResult),
            8228 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCRequestSlarkGameResultResponse),
            8229 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCSignoutSpendRankWager),
            8230 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetFavoriteTeam),
            8231 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCGetFavoriteTeamResponse),
            8232 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSignOutEventGameData),
            8233 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientAllStarVotesRequest),
            8234 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientAllStarVotesReply),
            8236 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientAllStarVotesSubmit),
            8237 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToClientAllStarVotesSubmitReply),
            8238 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCQuickStatsRequest),
            8239 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgClientToGCQuickStatsResponse),
            8240 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCSubtractEventPointsFromUser),
            8241 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSelectionPriorityChoiceRequest),
            8242 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgSelectionPriorityChoiceResponse),
            8243 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGCToGCCompendiumInGamePredictionResults),
            8244 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGameAutographReward),
            8245 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgGameAutographRewardResponse),
            8250 => ::std::option::Option::Some(EDOTAGCMsg::k_EMsgServerToGCMatchPlayerItemPurchaseHistory),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EDOTAGCMsg] = &[
            EDOTAGCMsg::k_EMsgGCDOTABase,
            EDOTAGCMsg::k_EMsgGCGeneralResponse,
            EDOTAGCMsg::k_EMsgGCGameMatchSignOut,
            EDOTAGCMsg::k_EMsgGCGameMatchSignOutResponse,
            EDOTAGCMsg::k_EMsgGCJoinChatChannel,
            EDOTAGCMsg::k_EMsgGCJoinChatChannelResponse,
            EDOTAGCMsg::k_EMsgGCOtherJoinedChannel,
            EDOTAGCMsg::k_EMsgGCOtherLeftChannel,
            EDOTAGCMsg::k_EMsgGCMatchHistoryList,
            EDOTAGCMsg::k_EMsgServerToGCRequestStatus,
            EDOTAGCMsg::k_EMsgGCGetRecentMatches,
            EDOTAGCMsg::k_EMsgGCRecentMatchesResponse,
            EDOTAGCMsg::k_EMsgGCStartFindingMatch,
            EDOTAGCMsg::k_EMsgGCConnectedPlayers,
            EDOTAGCMsg::k_EMsgGCAbandonCurrentGame,
            EDOTAGCMsg::k_EMsgGCStopFindingMatch,
            EDOTAGCMsg::k_EMsgGCPracticeLobbyCreate,
            EDOTAGCMsg::k_EMsgGCPracticeLobbyLeave,
            EDOTAGCMsg::k_EMsgGCPracticeLobbyLaunch,
            EDOTAGCMsg::k_EMsgGCPracticeLobbyList,
            EDOTAGCMsg::k_EMsgGCPracticeLobbyListResponse,
            EDOTAGCMsg::k_EMsgGCPracticeLobbyJoin,
            EDOTAGCMsg::k_EMsgGCPracticeLobbySetDetails,
            EDOTAGCMsg::k_EMsgGCPracticeLobbySetTeamSlot,
            EDOTAGCMsg::k_EMsgGCInitialQuestionnaireResponse,
            EDOTAGCMsg::k_EMsgGCPracticeLobbyResponse,
            EDOTAGCMsg::k_EMsgGCBroadcastNotification,
            EDOTAGCMsg::k_EMsgGCLiveScoreboardUpdate,
            EDOTAGCMsg::k_EMsgGCRequestChatChannelList,
            EDOTAGCMsg::k_EMsgGCRequestChatChannelListResponse,
            EDOTAGCMsg::k_EMsgGCRequestMatches,
            EDOTAGCMsg::k_EMsgGCRequestMatchesResponse,
            EDOTAGCMsg::k_EMsgGCRequestPlayerResources,
            EDOTAGCMsg::k_EMsgGCRequestPlayerResourcesResponse,
            EDOTAGCMsg::k_EMsgGCReadyUp,
            EDOTAGCMsg::k_EMsgGCKickedFromMatchmakingQueue,
            EDOTAGCMsg::k_EMsgGCLeaverDetected,
            EDOTAGCMsg::k_EMsgGCSpectateFriendGame,
            EDOTAGCMsg::k_EMsgGCSpectateFriendGameResponse,
            EDOTAGCMsg::k_EMsgGCPlayerReports,
            EDOTAGCMsg::k_EMsgGCReportsRemainingRequest,
            EDOTAGCMsg::k_EMsgGCReportsRemainingResponse,
            EDOTAGCMsg::k_EMsgGCSubmitPlayerReport,
            EDOTAGCMsg::k_EMsgGCSubmitPlayerReportResponse,
            EDOTAGCMsg::k_EMsgGCPracticeLobbyKick,
            EDOTAGCMsg::k_EMsgGCReportCountsRequest,
            EDOTAGCMsg::k_EMsgGCReportCountsResponse,
            EDOTAGCMsg::k_EMsgGCRequestSaveGames,
            EDOTAGCMsg::k_EMsgGCRequestSaveGamesServer,
            EDOTAGCMsg::k_EMsgGCRequestSaveGamesResponse,
            EDOTAGCMsg::k_EMsgGCLeaverDetectedResponse,
            EDOTAGCMsg::k_EMsgGCPlayerFailedToConnect,
            EDOTAGCMsg::k_EMsgGCGCToRelayConnect,
            EDOTAGCMsg::k_EMsgGCGCToRelayConnectresponse,
            EDOTAGCMsg::k_EMsgGCWatchGame,
            EDOTAGCMsg::k_EMsgGCWatchGameResponse,
            EDOTAGCMsg::k_EMsgGCBanStatusRequest,
            EDOTAGCMsg::k_EMsgGCBanStatusResponse,
            EDOTAGCMsg::k_EMsgGCMatchDetailsRequest,
            EDOTAGCMsg::k_EMsgGCMatchDetailsResponse,
            EDOTAGCMsg::k_EMsgGCCancelWatchGame,
            EDOTAGCMsg::k_EMsgGCProfileRequest,
            EDOTAGCMsg::k_EMsgGCProfileResponse,
            EDOTAGCMsg::k_EMsgGCPopup,
            EDOTAGCMsg::k_EMsgGCDOTAClearNotifySuccessfulReport,
            EDOTAGCMsg::k_EMsgGCFriendPracticeLobbyListRequest,
            EDOTAGCMsg::k_EMsgGCFriendPracticeLobbyListResponse,
            EDOTAGCMsg::k_EMsgGCPracticeLobbyJoinResponse,
            EDOTAGCMsg::k_EMsgClientEconNotification_Job,
            EDOTAGCMsg::k_EMsgGCCreateTeam,
            EDOTAGCMsg::k_EMsgGCCreateTeamResponse,
            EDOTAGCMsg::k_EMsgGCTeamData,
            EDOTAGCMsg::k_EMsgGCTeamInvite_InviterToGC,
            EDOTAGCMsg::k_EMsgGCTeamInvite_GCImmediateResponseToInviter,
            EDOTAGCMsg::k_EMsgGCTeamInvite_GCRequestToInvitee,
            EDOTAGCMsg::k_EMsgGCTeamInvite_InviteeResponseToGC,
            EDOTAGCMsg::k_EMsgGCTeamInvite_GCResponseToInviter,
            EDOTAGCMsg::k_EMsgGCTeamInvite_GCResponseToInvitee,
            EDOTAGCMsg::k_EMsgGCKickTeamMember,
            EDOTAGCMsg::k_EMsgGCKickTeamMemberResponse,
            EDOTAGCMsg::k_EMsgGCLeaveTeam,
            EDOTAGCMsg::k_EMsgGCLeaveTeamResponse,
            EDOTAGCMsg::k_EMsgGCSuggestTeamMatchmaking,
            EDOTAGCMsg::k_EMsgGCPlayerHeroesFavoritesAdd,
            EDOTAGCMsg::k_EMsgGCPlayerHeroesFavoritesRemove,
            EDOTAGCMsg::k_EMsgGCSetShowcaseHero,
            EDOTAGCMsg::k_EMsgGCApplyTeamToPracticeLobby,
            EDOTAGCMsg::k_EMsgGCRequestInternatinalTicketEmail,
            EDOTAGCMsg::k_EMsgGCTransferTeamAdmin,
            EDOTAGCMsg::k_EMsgRequestLeagueInfo,
            EDOTAGCMsg::k_EMsgResponseLeagueInfo,
            EDOTAGCMsg::k_EMsgGCPracticeLobbyJoinBroadcastChannel,
            EDOTAGCMsg::k_EMsgGC_TournamentItemEvent,
            EDOTAGCMsg::k_EMsgGC_TournamentItemEventResponse,
            EDOTAGCMsg::k_EMsgCastMatchVote,
            EDOTAGCMsg::k_EMsgCastMatchVoteResponse,
            EDOTAGCMsg::k_EMsgRetrieveMatchVote,
            EDOTAGCMsg::k_EMsgRetrieveMatchVoteResponse,
            EDOTAGCMsg::k_EMsgTeamFanfare,
            EDOTAGCMsg::k_EMsgResponseTeamFanfare,
            EDOTAGCMsg::k_EMsgGC_GameServerUploadSaveGame,
            EDOTAGCMsg::k_EMsgGC_GameServerSaveGameResult,
            EDOTAGCMsg::k_EMsgGC_GameServerGetLoadGame,
            EDOTAGCMsg::k_EMsgGC_GameServerGetLoadGameResult,
            EDOTAGCMsg::k_EMsgGCEditTeamDetails,
            EDOTAGCMsg::k_EMsgGCEditTeamDetailsResponse,
            EDOTAGCMsg::k_EMsgGCProTeamListRequest,
            EDOTAGCMsg::k_EMsgGCProTeamListResponse,
            EDOTAGCMsg::k_EMsgGCReadyUpStatus,
            EDOTAGCMsg::k_EMsgGCHallOfFame,
            EDOTAGCMsg::k_EMsgGCHallOfFameRequest,
            EDOTAGCMsg::k_EMsgGCHallOfFameResponse,
            EDOTAGCMsg::k_EMsgGCGenerateDiretidePrizeList,
            EDOTAGCMsg::k_EMsgGCRewardDiretidePrizes,
            EDOTAGCMsg::k_EMsgGCDiretidePrizesRewardedResponse,
            EDOTAGCMsg::k_EMsgGCHalloweenHighScoreRequest,
            EDOTAGCMsg::k_EMsgGCHalloweenHighScoreResponse,
            EDOTAGCMsg::k_EMsgGCGenerateDiretidePrizeListResponse,
            EDOTAGCMsg::k_EMsgGCStorePromoPagesRequest,
            EDOTAGCMsg::k_EMsgGCStorePromoPagesResponse,
            EDOTAGCMsg::k_EMsgGCToGCMatchCompleted,
            EDOTAGCMsg::k_EMsgGCBalancedShuffleLobby,
            EDOTAGCMsg::k_EMsgGCToGCCheckLeaguePermission,
            EDOTAGCMsg::k_EMsgGCToGCCheckLeaguePermissionResponse,
            EDOTAGCMsg::k_EMsgGCLeagueScheduleRequest,
            EDOTAGCMsg::k_EMsgGCLeagueScheduleResponse,
            EDOTAGCMsg::k_EMsgGCLeagueScheduleEdit,
            EDOTAGCMsg::k_EMsgGCLeagueScheduleEditResponse,
            EDOTAGCMsg::k_EMsgGCLeaguesInMonthRequest,
            EDOTAGCMsg::k_EMsgGCLeaguesInMonthResponse,
            EDOTAGCMsg::k_EMsgGCMatchmakingStatsRequest,
            EDOTAGCMsg::k_EMsgGCMatchmakingStatsResponse,
            EDOTAGCMsg::k_EMsgGCBotGameCreate,
            EDOTAGCMsg::k_EMsgGCSetMatchHistoryAccess,
            EDOTAGCMsg::k_EMsgGCSetMatchHistoryAccessResponse,
            EDOTAGCMsg::k_EMsgUpgradeLeagueItem,
            EDOTAGCMsg::k_EMsgUpgradeLeagueItemResponse,
            EDOTAGCMsg::k_EMsgGCTeamMemberProfileRequest,
            EDOTAGCMsg::k_EMsgGCWatchDownloadedReplay,
            EDOTAGCMsg::k_EMsgGCSetMapLocationState,
            EDOTAGCMsg::k_EMsgGCSetMapLocationStateResponse,
            EDOTAGCMsg::k_EMsgGCResetMapLocations,
            EDOTAGCMsg::k_EMsgGCResetMapLocationsResponse,
            EDOTAGCMsg::k_EMsgGCSetFeaturedItems,
            EDOTAGCMsg::k_EMsgGCFeaturedItems,
            EDOTAGCMsg::k_EMsgRefreshPartnerAccountLink,
            EDOTAGCMsg::k_EMsgClientsRejoinChatChannels,
            EDOTAGCMsg::k_EMsgGCToGCGetUserChatInfo,
            EDOTAGCMsg::k_EMsgGCToGCGetUserChatInfoResponse,
            EDOTAGCMsg::k_EMsgGCToGCLeaveAllChatChannels,
            EDOTAGCMsg::k_EMsgGCToGCUpdateAccountChatBan,
            EDOTAGCMsg::k_EMsgGCGuildCreateRequest,
            EDOTAGCMsg::k_EMsgGCGuildCreateResponse,
            EDOTAGCMsg::k_EMsgGCGuildSetAccountRoleRequest,
            EDOTAGCMsg::k_EMsgGCGuildSetAccountRoleResponse,
            EDOTAGCMsg::k_EMsgGCRequestGuildData,
            EDOTAGCMsg::k_EMsgGCGuildData,
            EDOTAGCMsg::k_EMsgGCGuildInviteAccountRequest,
            EDOTAGCMsg::k_EMsgGCGuildInviteAccountResponse,
            EDOTAGCMsg::k_EMsgGCGuildCancelInviteRequest,
            EDOTAGCMsg::k_EMsgGCGuildCancelInviteResponse,
            EDOTAGCMsg::k_EMsgGCGuildUpdateDetailsRequest,
            EDOTAGCMsg::k_EMsgGCGuildUpdateDetailsResponse,
            EDOTAGCMsg::k_EMsgGCToGCCanInviteUserToTeam,
            EDOTAGCMsg::k_EMsgGCToGCCanInviteUserToTeamResponse,
            EDOTAGCMsg::k_EMsgGCToGCGetUserRank,
            EDOTAGCMsg::k_EMsgGCToGCGetUserRankResponse,
            EDOTAGCMsg::k_EMsgGCToGCUpdateTeamStats,
            EDOTAGCMsg::k_EMsgGCToGCGetTeamRank,
            EDOTAGCMsg::k_EMsgGCToGCGetTeamRankResponse,
            EDOTAGCMsg::k_EMsgGCPassportDataRequest,
            EDOTAGCMsg::k_EMsgGCPassportDataResponse,
            EDOTAGCMsg::k_EMsgGCNotInGuildData,
            EDOTAGCMsg::k_EMsgGCGuildInviteData,
            EDOTAGCMsg::k_EMsgGCToGCGetLeagueAdmin,
            EDOTAGCMsg::k_EMsgGCToGCGetLeagueAdminResponse,
            EDOTAGCMsg::k_EMsgGCRequestLeaguePrizePool,
            EDOTAGCMsg::k_EMsgGCRequestLeaguePrizePoolResponse,
            EDOTAGCMsg::k_EMsgGCToGCUpdateOpenGuildPartyRequest,
            EDOTAGCMsg::k_EMsgGCToGCUpdateOpenGuildPartyResponse,
            EDOTAGCMsg::k_EMsgGCToGCDestroyOpenGuildPartyRequest,
            EDOTAGCMsg::k_EMsgGCToGCDestroyOpenGuildPartyResponse,
            EDOTAGCMsg::k_EMsgGCGuildUpdateMessage,
            EDOTAGCMsg::k_EMsgGCPartySetOpenGuildRequest,
            EDOTAGCMsg::k_EMsgGCPartySetOpenGuildResponse,
            EDOTAGCMsg::k_EMsgGCGuildOpenPartyRefresh,
            EDOTAGCMsg::k_EMsgGCJoinOpenGuildPartyRequest,
            EDOTAGCMsg::k_EMsgGCJoinOpenGuildPartyResponse,
            EDOTAGCMsg::k_EMsgGCLeaveChatChannel,
            EDOTAGCMsg::k_EMsgGCChatMessage,
            EDOTAGCMsg::k_EMsgGCGetHeroStandings,
            EDOTAGCMsg::k_EMsgGCGetHeroStandingsResponse,
            EDOTAGCMsg::k_EMsgGCGuildEditLogoRequest,
            EDOTAGCMsg::k_EMsgGCGuildEditLogoResponse,
            EDOTAGCMsg::k_EMsgGCGuildmatePracticeLobbyListRequest,
            EDOTAGCMsg::k_EMsgGCGuildmatePracticeLobbyListResponse,
            EDOTAGCMsg::k_EMsgGCItemEditorReservationsRequest,
            EDOTAGCMsg::k_EMsgGCItemEditorReservationsResponse,
            EDOTAGCMsg::k_EMsgGCItemEditorReserveItemDef,
            EDOTAGCMsg::k_EMsgGCItemEditorReserveItemDefResponse,
            EDOTAGCMsg::k_EMsgGCItemEditorReleaseReservation,
            EDOTAGCMsg::k_EMsgGCItemEditorReleaseReservationResponse,
            EDOTAGCMsg::k_EMsgGCRewardTutorialPrizes,
            EDOTAGCMsg::k_EMsgGCLastHitChallengeHighScorePost,
            EDOTAGCMsg::k_EMsgGCLastHitChallengeHighScoreRequest,
            EDOTAGCMsg::k_EMsgGCLastHitChallengeHighScoreResponse,
            EDOTAGCMsg::k_EMsgGCCreateFantasyLeagueRequest,
            EDOTAGCMsg::k_EMsgGCCreateFantasyLeagueResponse,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueInfoRequest,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueInfoResponse,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueInfo,
            EDOTAGCMsg::k_EMsgGCCreateFantasyTeamRequest,
            EDOTAGCMsg::k_EMsgGCCreateFantasyTeamResponse,
            EDOTAGCMsg::k_EMsgGCEditFantasyTeamRequest,
            EDOTAGCMsg::k_EMsgGCEditFantasyTeamResponse,
            EDOTAGCMsg::k_EMsgGCFantasyTeamInfoRequestByFantasyLeagueID,
            EDOTAGCMsg::k_EMsgGCFantasyTeamInfoRequestByOwnerAccountID,
            EDOTAGCMsg::k_EMsgGCFantasyTeamInfoResponse,
            EDOTAGCMsg::k_EMsgGCFantasyTeamInfo,
            EDOTAGCMsg::k_EMsgGCFantasyLivePlayerStats,
            EDOTAGCMsg::k_EMsgGCFantasyFinalPlayerStats,
            EDOTAGCMsg::k_EMsgGCFantasyMatch,
            EDOTAGCMsg::k_EMsgGCFantasyTeamScoreRequest,
            EDOTAGCMsg::k_EMsgGCFantasyTeamScoreResponse,
            EDOTAGCMsg::k_EMsgGCFantasyTeamStandingsRequest,
            EDOTAGCMsg::k_EMsgGCFantasyTeamStandingsResponse,
            EDOTAGCMsg::k_EMsgGCFantasyPlayerScoreRequest,
            EDOTAGCMsg::k_EMsgGCFantasyPlayerScoreResponse,
            EDOTAGCMsg::k_EMsgGCFantasyPlayerStandingsRequest,
            EDOTAGCMsg::k_EMsgGCFantasyPlayerStandingsResponse,
            EDOTAGCMsg::k_EMsgGCFlipLobbyTeams,
            EDOTAGCMsg::k_EMsgGCCustomGameCreate,
            EDOTAGCMsg::k_EMsgGCFantasyPlayerInfoRequest,
            EDOTAGCMsg::k_EMsgGCFantasyPlayerInfoResponse,
            EDOTAGCMsg::k_EMsgGCToGCProcessPlayerReportForTarget,
            EDOTAGCMsg::k_EMsgGCToGCProcessReportSuccess,
            EDOTAGCMsg::k_EMsgGCNotifyAccountFlagsChange,
            EDOTAGCMsg::k_EMsgGCSetProfilePrivacy,
            EDOTAGCMsg::k_EMsgGCSetProfilePrivacyResponse,
            EDOTAGCMsg::k_EMsgGCSteamProfileRequest,
            EDOTAGCMsg::k_EMsgGCSteamProfileRequestResponse,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueCreateInfoRequest,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueCreateInfoResponse,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueInviteInfoRequest,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueInviteInfoResponse,
            EDOTAGCMsg::k_EMsgGCClientIgnoredUser,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueCreateRequest,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueCreateResponse,
            EDOTAGCMsg::k_EMsgGCFantasyTeamCreateRequest,
            EDOTAGCMsg::k_EMsgGCFantasyTeamCreateResponse,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueFriendJoinListRequest,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueFriendJoinListResponse,
            EDOTAGCMsg::k_EMsgGCClientSuspended,
            EDOTAGCMsg::k_EMsgGCPartyMemberSetCoach,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueEditInvitesRequest,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueEditInvitesResponse,
            EDOTAGCMsg::k_EMsgGCPracticeLobbySetCoach,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueEditInfoRequest,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueEditInfoResponse,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueDraftStatusRequest,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueDraftStatus,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueDraftPlayerRequest,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueDraftPlayerResponse,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueMatchupsRequest,
            EDOTAGCMsg::k_EMsgGCFantasyLeagueMatchupsResponse,
            EDOTAGCMsg::k_EMsgGCFantasyTeamRosterSwapRequest,
            EDOTAGCMsg::k_EMsgGCFantasyTeamRosterSwapResponse,
            EDOTAGCMsg::k_EMsgGCFantasyTeamRosterRequest,
            EDOTAGCMsg::k_EMsgGCFantasyTeamRosterResponse,
            EDOTAGCMsg::k_EMsgGCNexonPartnerUpdate,
            EDOTAGCMsg::k_EMsgGCToGCProcessPCBangRewardPoints,
            EDOTAGCMsg::k_EMsgGCFantasyTeamRosterAddDropRequest,
            EDOTAGCMsg::k_EMsgGCFantasyTeamRosterAddDropResponse,
            EDOTAGCMsg::k_EMsgPresentedClientTerminateDlg,
            EDOTAGCMsg::k_EMsgGCFantasyPlayerHisoricalStatsRequest,
            EDOTAGCMsg::k_EMsgGCFantasyPlayerHisoricalStatsResponse,
            EDOTAGCMsg::k_EMsgGCPCBangTimedRewardMessage,
            EDOTAGCMsg::k_EMsgGCLobbyUpdateBroadcastChannelInfo,
            EDOTAGCMsg::k_EMsgGCFantasyTeamTradesRequest,
            EDOTAGCMsg::k_EMsgGCFantasyTeamTradesResponse,
            EDOTAGCMsg::k_EMsgGCFantasyTeamTradeCancelRequest,
            EDOTAGCMsg::k_EMsgGCFantasyTeamTradeCancelResponse,
            EDOTAGCMsg::k_EMsgGCToGCGrantTournamentItem,
            EDOTAGCMsg::k_EMsgGCProcessFantasyScheduledEvent,
            EDOTAGCMsg::k_EMsgGCToGCGrantPCBangRewardItem,
            EDOTAGCMsg::k_EMsgGCToGCUpgradeTwitchViewerItems,
            EDOTAGCMsg::k_EMsgGCToGCGetLiveMatchAffiliates,
            EDOTAGCMsg::k_EMsgGCToGCGetLiveMatchAffiliatesResponse,
            EDOTAGCMsg::k_EMsgGCToGCUpdatePlayerPennantCounts,
            EDOTAGCMsg::k_EMsgGCToGCGetPlayerPennantCounts,
            EDOTAGCMsg::k_EMsgGCToGCGetPlayerPennantCountsResponse,
            EDOTAGCMsg::k_EMsgGCGameMatchSignOutPermissionRequest,
            EDOTAGCMsg::k_EMsgGCGameMatchSignOutPermissionResponse,
            EDOTAGCMsg::k_EMsgDOTAChatChannelMemberUpdate,
            EDOTAGCMsg::k_EMsgDOTAAwardEventPoints,
            EDOTAGCMsg::k_EMsgDOTAGetEventPoints,
            EDOTAGCMsg::k_EMsgDOTAGetEventPointsResponse,
            EDOTAGCMsg::k_EMsgGCToGCSignoutAwardEventPoints,
            EDOTAGCMsg::k_EMsgDOTASendFriendRecruits,
            EDOTAGCMsg::k_EMsgDOTAFriendRecruitsRequest,
            EDOTAGCMsg::k_EMsgDOTAFriendRecruitsResponse,
            EDOTAGCMsg::k_EMsgDOTAFriendRecruitInviteAcceptDecline,
            EDOTAGCMsg::k_EMsgGCPartyLeaderWatchGamePrompt,
            EDOTAGCMsg::k_EMsgDOTAFrostivusTimeElapsed,
            EDOTAGCMsg::k_EMsgDOTALiveLeagueGameUpdate,
            EDOTAGCMsg::k_EMsgDOTAChatGetUserList,
            EDOTAGCMsg::k_EMsgDOTAChatGetUserListResponse,
            EDOTAGCMsg::k_EMsgGCCompendiumSetSelection,
            EDOTAGCMsg::k_EMsgGCCompendiumDataRequest,
            EDOTAGCMsg::k_EMsgGCCompendiumDataResponse,
            EDOTAGCMsg::k_EMsgDOTAGetPlayerMatchHistory,
            EDOTAGCMsg::k_EMsgDOTAGetPlayerMatchHistoryResponse,
            EDOTAGCMsg::k_EMsgGCToGCMatchmakingAddParty,
            EDOTAGCMsg::k_EMsgGCToGCMatchmakingRemoveParty,
            EDOTAGCMsg::k_EMsgGCToGCMatchmakingRemoveAllParties,
            EDOTAGCMsg::k_EMsgGCToGCMatchmakingMatchFound,
            EDOTAGCMsg::k_EMsgGCToGCUpdateMatchManagementStats,
            EDOTAGCMsg::k_EMsgGCToGCUpdateMatchmakingStats,
            EDOTAGCMsg::k_EMsgGCToServerPingRequest,
            EDOTAGCMsg::k_EMsgGCToServerPingResponse,
            EDOTAGCMsg::k_EMsgGCToServerConsoleCommand,
            EDOTAGCMsg::k_EMsgGCToGCUpdateLiveLeagueGameInfo,
            EDOTAGCMsg::k_EMsgGCMakeOffering,
            EDOTAGCMsg::k_EMsgGCRequestOfferings,
            EDOTAGCMsg::k_EMsgGCRequestOfferingsResponse,
            EDOTAGCMsg::k_EMsgGCToGCProcessMatchLeaver,
            EDOTAGCMsg::k_EMsgGCNotificationsRequest,
            EDOTAGCMsg::k_EMsgGCNotificationsResponse,
            EDOTAGCMsg::k_EMsgGCToGCModifyNotification,
            EDOTAGCMsg::k_EMsgGCToGCSetNewNotifications,
            EDOTAGCMsg::k_EMsgGCToGCSetIsLeagueAdmin,
            EDOTAGCMsg::k_EMsgGCLeagueAdminState,
            EDOTAGCMsg::k_EMsgGCToGCSendLeagueAdminState,
            EDOTAGCMsg::k_EMsgGCLeagueAdminList,
            EDOTAGCMsg::k_EMsgGCNotificationsMarkReadRequest,
            EDOTAGCMsg::k_EMsgGCFantasyMessageAdd,
            EDOTAGCMsg::k_EMsgGCFantasyMessagesRequest,
            EDOTAGCMsg::k_EMsgGCFantasyMessagesResponse,
            EDOTAGCMsg::k_EMsgGCFantasyScheduledMatchesRequest,
            EDOTAGCMsg::k_EMsgGCFantasyScheduledMatchesResponse,
            EDOTAGCMsg::k_EMsgGCToGCGrantLeagueAccess,
            EDOTAGCMsg::k_EMsgGCEventGameCreate,
            EDOTAGCMsg::k_EMsgGCPerfectWorldUserLookupRequest,
            EDOTAGCMsg::k_EMsgGCPerfectWorldUserLookupResponse,
            EDOTAGCMsg::k_EMsgGCFantasyRemoveOwner,
            EDOTAGCMsg::k_EMsgGCFantasyRemoveOwnerResponse,
            EDOTAGCMsg::k_EMsgGCRequestBatchPlayerResources,
            EDOTAGCMsg::k_EMsgGCRequestBatchPlayerResourcesResponse,
            EDOTAGCMsg::k_EMsgGCToGCSendUpdateLeagues,
            EDOTAGCMsg::k_EMsgGCCompendiumSetSelectionResponse,
            EDOTAGCMsg::k_EMsgGCPlayerInfoRequest,
            EDOTAGCMsg::k_EMsgGCPlayerInfo,
            EDOTAGCMsg::k_EMsgGCPlayerInfoSubmit,
            EDOTAGCMsg::k_EMsgGCPlayerInfoSubmitResponse,
            EDOTAGCMsg::k_EMsgGCToGCGetAccountLevel,
            EDOTAGCMsg::k_EMsgGCToGCGetAccountLevelResponse,
            EDOTAGCMsg::k_EMsgGCToGCGetAccountPartner,
            EDOTAGCMsg::k_EMsgGCToGCGetAccountPartnerResponse,
            EDOTAGCMsg::k_EMsgGCToGCGetAccountProfile,
            EDOTAGCMsg::k_EMsgGCToGCGetAccountProfileResponse,
            EDOTAGCMsg::k_EMsgDOTAGetWeekendTourneySchedule,
            EDOTAGCMsg::k_EMsgDOTAWeekendTourneySchedule,
            EDOTAGCMsg::k_EMsgGCJoinableCustomGameModesRequest,
            EDOTAGCMsg::k_EMsgGCJoinableCustomGameModesResponse,
            EDOTAGCMsg::k_EMsgGCJoinableCustomLobbiesRequest,
            EDOTAGCMsg::k_EMsgGCJoinableCustomLobbiesResponse,
            EDOTAGCMsg::k_EMsgGCQuickJoinCustomLobby,
            EDOTAGCMsg::k_EMsgGCQuickJoinCustomLobbyResponse,
            EDOTAGCMsg::k_EMsgGCToGCGrantEventPointAction,
            EDOTAGCMsg::k_EMsgServerGetEventPoints,
            EDOTAGCMsg::k_EMsgServerGetEventPointsResponse,
            EDOTAGCMsg::k_EMsgServerGrantSurveyPermission,
            EDOTAGCMsg::k_EMsgServerGrantSurveyPermissionResponse,
            EDOTAGCMsg::k_EMsgClientProvideSurveyResult,
            EDOTAGCMsg::k_EMsgGCToGCSetCompendiumSelection,
            EDOTAGCMsg::k_EMsgGCToGCUpdateTI4HeroQuest,
            EDOTAGCMsg::k_EMsgGCCompendiumDataChanged,
            EDOTAGCMsg::k_EMsgDOTAFantasyLeagueFindRequest,
            EDOTAGCMsg::k_EMsgDOTAFantasyLeagueFindResponse,
            EDOTAGCMsg::k_EMsgGCHasItemQuery,
            EDOTAGCMsg::k_EMsgGCHasItemResponse,
            EDOTAGCMsg::k_EMsgGCConsumeFantasyTicket,
            EDOTAGCMsg::k_EMsgGCConsumeFantasyTicketFailure,
            EDOTAGCMsg::k_EMsgGCToGCGrantEventPointActionMsg,
            EDOTAGCMsg::k_EMsgClientToGCTrackDialogResult,
            EDOTAGCMsg::k_EMsgGCFantasyLeaveLeagueRequest,
            EDOTAGCMsg::k_EMsgGCFantasyLeaveLeagueResponse,
            EDOTAGCMsg::k_EMsgGCToGCGetCompendiumSelections,
            EDOTAGCMsg::k_EMsgGCToGCGetCompendiumSelectionsResponse,
            EDOTAGCMsg::k_EMsgServerToGCMatchConnectionStats,
            EDOTAGCMsg::k_EMsgGCToClientTournamentItemDrop,
            EDOTAGCMsg::k_EMsgSQLDelayedGrantLeagueDrop,
            EDOTAGCMsg::k_EMsgServerGCUpdateSpectatorCount,
            EDOTAGCMsg::k_EMsgGCFantasyPlayerScoreDetailsRequest,
            EDOTAGCMsg::k_EMsgGCFantasyPlayerScoreDetailsResponse,
            EDOTAGCMsg::k_EMsgGCToGCEmoticonUnlock,
            EDOTAGCMsg::k_EMsgSignOutDraftInfo,
            EDOTAGCMsg::k_EMsgClientToGCEmoticonDataRequest,
            EDOTAGCMsg::k_EMsgGCToClientEmoticonData,
            EDOTAGCMsg::k_EMsgGCPracticeLobbyToggleBroadcastChannelCameramanStatus,
            EDOTAGCMsg::k_EMsgGCToGCCreateWeekendTourneyRequest,
            EDOTAGCMsg::k_EMsgGCToGCCreateWeekendTourneyResponse,
            EDOTAGCMsg::k_EMsgClientToGCSetAdditionalEquips,
            EDOTAGCMsg::k_EMsgClientToGCGetAdditionalEquips,
            EDOTAGCMsg::k_EMsgClientToGCGetAdditionalEquipsResponse,
            EDOTAGCMsg::k_EMsgServerToGCGetAdditionalEquips,
            EDOTAGCMsg::k_EMsgServerToGCGetAdditionalEquipsResponse,
            EDOTAGCMsg::k_EMsgDOTARedeemItem,
            EDOTAGCMsg::k_EMsgDOTARedeemItemResponse,
            EDOTAGCMsg::k_EMsgSQLGCToGCGrantAllHeroProgress,
            EDOTAGCMsg::k_EMsgClientToGCGetAllHeroProgress,
            EDOTAGCMsg::k_EMsgClientToGCGetAllHeroProgressResponse,
            EDOTAGCMsg::k_EMsgGCToGCGetServerForClient,
            EDOTAGCMsg::k_EMsgGCToGCGetServerForClientResponse,
            EDOTAGCMsg::k_EMsgSQLProcessTournamentGameOutcome,
            EDOTAGCMsg::k_EMsgSQLGrantTrophyToAccount,
            EDOTAGCMsg::k_EMsgClientToGCGetTrophyList,
            EDOTAGCMsg::k_EMsgClientToGCGetTrophyListResponse,
            EDOTAGCMsg::k_EMsgGCToClientTrophyAwarded,
            EDOTAGCMsg::k_EMsgGCGameBotMatchSignOut,
            EDOTAGCMsg::k_EMsgGCGameBotMatchSignOutPermissionRequest,
            EDOTAGCMsg::k_EMsgSignOutBotInfo,
            EDOTAGCMsg::k_EMsgGCToGCUpdateProfileCards,
            EDOTAGCMsg::k_EMsgClientToGCGetProfileCard,
            EDOTAGCMsg::k_EMsgClientToGCGetProfileCardResponse,
            EDOTAGCMsg::k_EMsgServerToGCGetProfileCard,
            EDOTAGCMsg::k_EMsgServerToGCGetProfileCardResponse,
            EDOTAGCMsg::k_EMsgClientToGCSetProfileCardSlots,
            EDOTAGCMsg::k_EMsgGCToClientProfileCardUpdated,
            EDOTAGCMsg::k_EMsgServerToGCVictoryPredictions,
            EDOTAGCMsg::k_EMsgClientToGCMarkNotificationListRead,
            EDOTAGCMsg::k_EMsgGCToClientNewNotificationAdded,
            EDOTAGCMsg::k_EMsgServerToGCSuspiciousActivity,
            EDOTAGCMsg::k_EMsgSignOutCommunicationSummary,
            EDOTAGCMsg::k_EMsgServerToGCRequestStatus_Response,
            EDOTAGCMsg::k_EMsgClientToGCCreateHeroStatue,
            EDOTAGCMsg::k_EMsgGCToClientHeroStatueCreateResult,
            EDOTAGCMsg::k_EMsgGCGCToLANServerRelayConnect,
            EDOTAGCMsg::k_EMsgServerToGCGetIngameEventData,
            EDOTAGCMsg::k_EMsgGCToGCUpdateIngameEventDataBroadcast,
            EDOTAGCMsg::k_EMsgGCToServerIngameEventData_OraclePA,
            EDOTAGCMsg::k_EMsgServerToGCReportKillSummaries,
            EDOTAGCMsg::k_EMsgGCToGCReportKillSummaries,
            EDOTAGCMsg::k_EMsgGCToGCUpdateAssassinMinigame,
            EDOTAGCMsg::k_EMsgGCToGCFantasySetMatchLeague,
            EDOTAGCMsg::k_EMsgClientToGCRecordCompendiumStats,
            EDOTAGCMsg::k_EMsgGCItemEditorRequestLeagueInfo,
            EDOTAGCMsg::k_EMsgGCItemEditorLeagueInfoResponse,
            EDOTAGCMsg::k_EMsgGCToGCUpdatePlayerPredictions,
            EDOTAGCMsg::k_EMsgGCToServerPredictionResult,
            EDOTAGCMsg::k_EMsgServerToGCSignoutAwardAdditionalDrops,
            EDOTAGCMsg::k_EMsgGCToGCSignoutAwardAdditionalDrops,
            EDOTAGCMsg::k_EMsgGCToClientEventStatusChanged,
            EDOTAGCMsg::k_EMsgGCHasItemDefsQuery,
            EDOTAGCMsg::k_EMsgGCHasItemDefsResponse,
            EDOTAGCMsg::k_EMsgGCToGCReplayMonitorValidateReplay,
            EDOTAGCMsg::k_EMsgLobbyEventPoints,
            EDOTAGCMsg::k_EMsgGCToGCGetCustomGameTickets,
            EDOTAGCMsg::k_EMsgGCToGCGetCustomGameTicketsResponse,
            EDOTAGCMsg::k_EMsgGCToGCCustomGamePlayed,
            EDOTAGCMsg::k_EMsgGCToGCGrantEventPointsToUser,
            EDOTAGCMsg::k_EMsgGCToGCSetEventMMPanicFlushTime,
            EDOTAGCMsg::k_EMsgGameserverCrashReport,
            EDOTAGCMsg::k_EMsgGameserverCrashReportResponse,
            EDOTAGCMsg::k_EMsgGCToClientSteamDatagramTicket,
            EDOTAGCMsg::k_EMsgGCToGCGrantEventOwnership,
            EDOTAGCMsg::k_EMsgGCToGCSendAccountsEventPoints,
            EDOTAGCMsg::k_EMsgClientToGCRerollPlayerChallenge,
            EDOTAGCMsg::k_EMsgServerToGCRerollPlayerChallenge,
            EDOTAGCMsg::k_EMsgGCRerollPlayerChallengeResponse,
            EDOTAGCMsg::k_EMsgSignOutUpdatePlayerChallenge,
            EDOTAGCMsg::k_EMsgClientToGCSetPartyLeader,
            EDOTAGCMsg::k_EMsgClientToGCCancelPartyInvites,
            EDOTAGCMsg::k_EMsgGCToGCMasterReloadAccount,
            EDOTAGCMsg::k_EMsgSQLGrantLeagueMatchToTicketHolders,
            EDOTAGCMsg::k_EMsgClientToGCSetAdditionalEquipsResponse,
            EDOTAGCMsg::k_EMsgGCToGCEmoticonUnlockNoRollback,
            EDOTAGCMsg::k_EMsgGCToGCGetCompendiumFanfare,
            EDOTAGCMsg::k_EMsgServerToGCHoldEventPoints,
            EDOTAGCMsg::k_EMsgSignOutReleaseEventPointHolds,
            EDOTAGCMsg::k_EMsgGCToGCChatNewUserSession,
            EDOTAGCMsg::k_EMsgClientToGCGetLeagueSeries,
            EDOTAGCMsg::k_EMsgClientToGCGetLeagueSeriesResponse,
            EDOTAGCMsg::k_EMsgSQLGCToGCSignoutUpdateLeagueSchedule,
            EDOTAGCMsg::k_EMsgGCToServerUpdateBroadcastCheers,
            EDOTAGCMsg::k_EMsgClientToGCApplyGemCombiner,
            EDOTAGCMsg::k_EMsgClientToGCDOTACreateStaticRecipe,
            EDOTAGCMsg::k_EMsgClientToGCDOTACreateStaticRecipeResponse,
            EDOTAGCMsg::k_EMsgClientToGCGetAllHeroOrder,
            EDOTAGCMsg::k_EMsgClientToGCGetAllHeroOrderResponse,
            EDOTAGCMsg::k_EMsgSQLGCToGCGrantBadgePoints,
            EDOTAGCMsg::k_EMsgGCToGCGetAccountMatchStatus,
            EDOTAGCMsg::k_EMsgGCToGCGetAccountMatchStatusResponse,
            EDOTAGCMsg::k_EMsgGCToGCCheckOwnsEntireEmoticonRange,
            EDOTAGCMsg::k_EMsgGCToGCCheckOwnsEntireEmoticonRangeResponse,
            EDOTAGCMsg::k_EMsgGCDev_GrantWarKill,
            EDOTAGCMsg::k_EMsgServerToGCLockCharmTrading,
            EDOTAGCMsg::k_EMsgClientToGCPlayerStatsRequest,
            EDOTAGCMsg::k_EMsgGCToClientPlayerStatsResponse,
            EDOTAGCMsg::k_EMsgGCClearPracticeLobbyTeam,
            EDOTAGCMsg::k_EMsgClientToGCFindTopSourceTVGames,
            EDOTAGCMsg::k_EMsgGCToClientFindTopSourceTVGamesResponse,
            EDOTAGCMsg::k_EMsgGCLobbyList,
            EDOTAGCMsg::k_EMsgGCLobbyListResponse,
            EDOTAGCMsg::k_EMsgGCPlayerStatsMatchSignOut,
            EDOTAGCMsg::k_EMsgClientToGCCustomGamePlayerCountRequest,
            EDOTAGCMsg::k_EMsgGCToClientCustomGamePlayerCountResponse,
            EDOTAGCMsg::k_EMsgClientToGCSocialFeedPostCommentRequest,
            EDOTAGCMsg::k_EMsgGCToClientSocialFeedPostCommentResponse,
            EDOTAGCMsg::k_EMsgClientToGCCustomGamesFriendsPlayedRequest,
            EDOTAGCMsg::k_EMsgGCToClientCustomGamesFriendsPlayedResponse,
            EDOTAGCMsg::k_EMsgClientToGCFriendsPlayedCustomGameRequest,
            EDOTAGCMsg::k_EMsgGCToClientFriendsPlayedCustomGameResponse,
            EDOTAGCMsg::k_EMsgClientToGCFeaturedHeroesRequest,
            EDOTAGCMsg::k_EMsgGCToClientFeaturedHeroesResponse,
            EDOTAGCMsg::k_EMsgGCTopCustomGamesList,
            EDOTAGCMsg::k_EMsgClientToGCSocialMatchPostCommentRequest,
            EDOTAGCMsg::k_EMsgGCToClientSocialMatchPostCommentResponse,
            EDOTAGCMsg::k_EMsgClientToGCSocialMatchDetailsRequest,
            EDOTAGCMsg::k_EMsgGCToClientSocialMatchDetailsResponse,
            EDOTAGCMsg::k_EMsgClientToGCSetPartyOpen,
            EDOTAGCMsg::k_EMsgClientToGCMergePartyInvite,
            EDOTAGCMsg::k_EMsgGCToClientMergeGroupInviteReply,
            EDOTAGCMsg::k_EMsgClientToGCMergePartyResponse,
            EDOTAGCMsg::k_EMsgGCToClientMergePartyResponseReply,
            EDOTAGCMsg::k_EMsgClientToGCGetProfileCardStats,
            EDOTAGCMsg::k_EMsgClientToGCGetProfileCardStatsResponse,
            EDOTAGCMsg::k_EMsgClientToGCTopLeagueMatchesRequest,
            EDOTAGCMsg::k_EMsgClientToGCTopFriendMatchesRequest,
            EDOTAGCMsg::k_EMsgGCToClientProfileCardStatsUpdated,
            EDOTAGCMsg::k_EMsgServerToGCRealtimeStats,
            EDOTAGCMsg::k_EMsgGCToServerRealtimeStatsStartStop,
            EDOTAGCMsg::k_EMsgGCToGCGetServersForClients,
            EDOTAGCMsg::k_EMsgGCToGCGetServersForClientsResponse,
            EDOTAGCMsg::k_EMsgGCPracticeLobbyKickFromTeam,
            EDOTAGCMsg::k_EMsgDOTAChatGetMemberCount,
            EDOTAGCMsg::k_EMsgDOTAChatGetMemberCountResponse,
            EDOTAGCMsg::k_EMsgClientToGCSocialFeedPostMessageRequest,
            EDOTAGCMsg::k_EMsgGCToClientSocialFeedPostMessageResponse,
            EDOTAGCMsg::k_EMsgCustomGameListenServerStartedLoading,
            EDOTAGCMsg::k_EMsgCustomGameClientFinishedLoading,
            EDOTAGCMsg::k_EMsgGCPracticeLobbyCloseBroadcastChannel,
            EDOTAGCMsg::k_EMsgGCStartFindingMatchResponse,
            EDOTAGCMsg::k_EMsgSQLGCToGCGrantAccountFlag,
            EDOTAGCMsg::k_EMsgGCToGCGetAccountFlags,
            EDOTAGCMsg::k_EMsgGCToGCGetAccountFlagsResponse,
            EDOTAGCMsg::k_EMsgSignOutWagerStats,
            EDOTAGCMsg::k_EMsgGCToClientTopLeagueMatchesResponse,
            EDOTAGCMsg::k_EMsgGCToClientTopFriendMatchesResponse,
            EDOTAGCMsg::k_EMsgClientToGCMatchesMinimalRequest,
            EDOTAGCMsg::k_EMsgClientToGCMatchesMinimalResponse,
            EDOTAGCMsg::k_EMsgGCToGCGetProfileBadgePoints,
            EDOTAGCMsg::k_EMsgGCToGCGetProfileBadgePointsResponse,
            EDOTAGCMsg::k_EMsgGCToClientChatRegionsEnabled,
            EDOTAGCMsg::k_EMsgClientToGCPingData,
            EDOTAGCMsg::k_EMsgServerToGCMatchDetailsRequest,
            EDOTAGCMsg::k_EMsgGCToServerMatchDetailsResponse,
            EDOTAGCMsg::k_EMsgGCToGCEnsureAccountInParty,
            EDOTAGCMsg::k_EMsgGCToGCEnsureAccountInPartyResponse,
            EDOTAGCMsg::k_EMsgClientToGCGetProfileTickets,
            EDOTAGCMsg::k_EMsgClientToGCGetProfileTicketsResponse,
            EDOTAGCMsg::k_EMsgGCToClientMatchGroupsVersion,
            EDOTAGCMsg::k_EMsgClientToGCH264Unsupported,
            EDOTAGCMsg::k_EMsgClientToGCRequestH264Support,
            EDOTAGCMsg::k_EMsgClientToGCGetQuestProgress,
            EDOTAGCMsg::k_EMsgClientToGCGetQuestProgressResponse,
            EDOTAGCMsg::k_EMsgSignOutXPCoins,
            EDOTAGCMsg::k_EMsgGCToClientMatchSignedOut,
            EDOTAGCMsg::k_EMsgGCGetHeroStatsHistory,
            EDOTAGCMsg::k_EMsgGCGetHeroStatsHistoryResponse,
            EDOTAGCMsg::k_EMsgClientToGCPrivateChatInvite,
            EDOTAGCMsg::k_EMsgClientToGCPrivateChatKick,
            EDOTAGCMsg::k_EMsgClientToGCPrivateChatPromote,
            EDOTAGCMsg::k_EMsgClientToGCPrivateChatDemote,
            EDOTAGCMsg::k_EMsgGCToClientPrivateChatResponse,
            EDOTAGCMsg::k_EMsgClientToGCPrivateChatInfoRequest,
            EDOTAGCMsg::k_EMsgGCToClientPrivateChatInfoResponse,
            EDOTAGCMsg::k_EMsgClientToGCLatestConductScorecardRequest,
            EDOTAGCMsg::k_EMsgClientToGCLatestConductScorecard,
            EDOTAGCMsg::k_EMsgServerToGCPostMatchTip,
            EDOTAGCMsg::k_EMsgServerToGCPostMatchTipResponse,
            EDOTAGCMsg::k_EMsgClientToGCWageringRequest,
            EDOTAGCMsg::k_EMsgGCToClientWageringResponse,
            EDOTAGCMsg::k_EMsgClientToGCEventGoalsRequest,
            EDOTAGCMsg::k_EMsgClientToGCEventGoalsResponse,
            EDOTAGCMsg::k_EMsgClientToGCLeaguePredictions,
            EDOTAGCMsg::k_EMsgGCToClientLeaguePredictionsResponse,
            EDOTAGCMsg::k_EMsgGCToGCLeaguePredictionsUpdate,
            EDOTAGCMsg::k_EMsgClientToGCSuspiciousActivity,
            EDOTAGCMsg::k_EMsgGCToGCAddUserToPostGameChat,
            EDOTAGCMsg::k_EMsgClientToGCHasPlayerVotedForMVP,
            EDOTAGCMsg::k_EMsgClientToGCHasPlayerVotedForMVPResponse,
            EDOTAGCMsg::k_EMsgClientToGCVoteForMVP,
            EDOTAGCMsg::k_EMsgClientToGCVoteForMVPResponse,
            EDOTAGCMsg::k_EMsgGCToGCGetEventOwnership,
            EDOTAGCMsg::k_EMsgGCToGCGetEventOwnershipResponse,
            EDOTAGCMsg::k_EMsgGCToClientAutomatedTournamentStateChange,
            EDOTAGCMsg::k_EMsgClientToGCWeekendTourneyOpts,
            EDOTAGCMsg::k_EMsgClientToGCWeekendTourneyOptsResponse,
            EDOTAGCMsg::k_EMsgClientToGCWeekendTourneyLeave,
            EDOTAGCMsg::k_EMsgClientToGCWeekendTourneyLeaveResponse,
            EDOTAGCMsg::k_EMsgClientToGCTeammateStatsRequest,
            EDOTAGCMsg::k_EMsgClientToGCTeammateStatsResponse,
            EDOTAGCMsg::k_EMsgClientToGCGetGiftPermissions,
            EDOTAGCMsg::k_EMsgClientToGCGetGiftPermissionsResponse,
            EDOTAGCMsg::k_EMsgClientToGCVoteForArcana,
            EDOTAGCMsg::k_EMsgClientToGCVoteForArcanaResponse,
            EDOTAGCMsg::k_EMsgClientToGCRequestArcanaVotesRemaining,
            EDOTAGCMsg::k_EMsgClientToGCRequestArcanaVotesRemainingResponse,
            EDOTAGCMsg::k_EMsgGCTransferTeamAdminResponse,
            EDOTAGCMsg::k_EMsgGCChangeTeamSub,
            EDOTAGCMsg::k_EMsgGCChangeTeamSubResponse,
            EDOTAGCMsg::k_EMsgGCToClientTeamInfo,
            EDOTAGCMsg::k_EMsgGCToClientTeamsInfo,
            EDOTAGCMsg::k_EMsgClientToGCMyTeamInfoRequest,
            EDOTAGCMsg::k_EMsgClientToGCRequestEventPointLog,
            EDOTAGCMsg::k_EMsgClientToGCRequestEventPointLogResponse,
            EDOTAGCMsg::k_EMsgClientToGCPublishUserStat,
            EDOTAGCMsg::k_EMsgGCToGCSignoutSpendWager,
            EDOTAGCMsg::k_EMsgGCSubmitLobbyMVPVote,
            EDOTAGCMsg::k_EMsgGCSubmitLobbyMVPVoteResponse,
            EDOTAGCMsg::k_EMsgClientToGCRequestLinaPlaysRemaining,
            EDOTAGCMsg::k_EMsgClientToGCRequestLinaPlaysRemainingResponse,
            EDOTAGCMsg::k_EMsgClientToGCRequestLinaGameResult,
            EDOTAGCMsg::k_EMsgClientToGCRequestLinaGameResultResponse,
            EDOTAGCMsg::k_EMsgSignOutCommunityGoalProgress,
            EDOTAGCMsg::k_EMsgGCToClientLobbyMVPNotifyRecipient,
            EDOTAGCMsg::k_EMsgGCToClientLobbyMVPAwarded,
            EDOTAGCMsg::k_EMsgGCToClientQuestProgressUpdated,
            EDOTAGCMsg::k_EMsgGCToClientWageringUpdate,
            EDOTAGCMsg::k_EMsgGCToClientArcanaVotesUpdate,
            EDOTAGCMsg::k_EMsgClientToGCAddTI6TreeProgress,
            EDOTAGCMsg::k_EMsgClientToGCSetSpectatorLobbyDetails,
            EDOTAGCMsg::k_EMsgClientToGCSetSpectatorLobbyDetailsResponse,
            EDOTAGCMsg::k_EMsgClientToGCCreateSpectatorLobby,
            EDOTAGCMsg::k_EMsgClientToGCCreateSpectatorLobbyResponse,
            EDOTAGCMsg::k_EMsgClientToGCSpectatorLobbyList,
            EDOTAGCMsg::k_EMsgClientToGCSpectatorLobbyListResponse,
            EDOTAGCMsg::k_EMsgSpectatorLobbyGameDetails,
            EDOTAGCMsg::k_EMsgServerToGCStartCompendiumInGamePredictions,
            EDOTAGCMsg::k_EMsgServerToGCEndCompendiumInGamePredictions,
            EDOTAGCMsg::k_EMsgServerToGCCompendiumInGamePredictionResults,
            EDOTAGCMsg::k_EMsgServerToGCCloseCompendiumInGamePredictionVoting,
            EDOTAGCMsg::k_EMsgClientToGCOpenPlayerCardPack,
            EDOTAGCMsg::k_EMsgClientToGCOpenPlayerCardPackResponse,
            EDOTAGCMsg::k_EMsgClientToGCSelectCompendiumInGamePrediction,
            EDOTAGCMsg::k_EMsgClientToGCSelectCompendiumInGamePredictionResponse,
            EDOTAGCMsg::k_EMsgClientToGCWeekendTourneyGetPlayerStats,
            EDOTAGCMsg::k_EMsgClientToGCWeekendTourneyGetPlayerStatsResponse,
            EDOTAGCMsg::k_EMsgClientToGCRecyclePlayerCard,
            EDOTAGCMsg::k_EMsgClientToGCRecyclePlayerCardResponse,
            EDOTAGCMsg::k_EMsgClientToGCCreatePlayerCardPack,
            EDOTAGCMsg::k_EMsgClientToGCCreatePlayerCardPackResponse,
            EDOTAGCMsg::k_EMsgClientToGCGetPlayerCardRosterRequest,
            EDOTAGCMsg::k_EMsgClientToGCGetPlayerCardRosterResponse,
            EDOTAGCMsg::k_EMsgClientToGCSetPlayerCardRosterRequest,
            EDOTAGCMsg::k_EMsgClientToGCSetPlayerCardRosterResponse,
            EDOTAGCMsg::k_EMsgServerToGCStartCompendiumInGamePredictionsResponse,
            EDOTAGCMsg::k_EMsgServerToGCCloseCompendiumInGamePredictionVotingResponse,
            EDOTAGCMsg::k_EMsgServerToGCEndCompendiumInGamePredictionsResponse,
            EDOTAGCMsg::k_EMsgServerToGCCompendiumInGamePredictionResultsResponse,
            EDOTAGCMsg::k_EMsgLobbyBattleCupVictory,
            EDOTAGCMsg::k_EMsgGCGetPlayerCardItemInfo,
            EDOTAGCMsg::k_EMsgGCGetPlayerCardItemInfoResponse,
            EDOTAGCMsg::k_EMsgClientToGCRequestSteamDatagramTicket,
            EDOTAGCMsg::k_EMsgClientToGCRequestSteamDatagramTicketResponse,
            EDOTAGCMsg::k_EMsgGCToClientBattlePassRollupRequest,
            EDOTAGCMsg::k_EMsgGCToClientBattlePassRollupResponse,
            EDOTAGCMsg::k_EMsgClientToGCTransferSeasonalMMRRequest,
            EDOTAGCMsg::k_EMsgClientToGCTransferSeasonalMMRResponse,
            EDOTAGCMsg::k_EMsgGCToGCPublicChatCommunicationBan,
            EDOTAGCMsg::k_EMsgGCToGCUpdateAccountPublicChatBan,
            EDOTAGCMsg::k_EMsgGCChatReportPublicSpam,
            EDOTAGCMsg::k_EMsgClientToGCSetPartyBuilderOptions,
            EDOTAGCMsg::k_EMsgClientToGCSetPartyBuilderOptionsResponse,
            EDOTAGCMsg::k_EMsgGCToClientPlaytestStatus,
            EDOTAGCMsg::k_EMsgClientToGCJoinPlaytest,
            EDOTAGCMsg::k_EMsgClientToGCJoinPlaytestResponse,
            EDOTAGCMsg::k_EMsgLobbyPlaytestDetails,
            EDOTAGCMsg::k_EMsgDOTASetFavoriteTeam,
            EDOTAGCMsg::k_EMsgGCToClientBattlePassRollupListRequest,
            EDOTAGCMsg::k_EMsgGCToClientBattlePassRollupListResponse,
            EDOTAGCMsg::k_EMsgGCIsProQuery,
            EDOTAGCMsg::k_EMsgGCIsProResponse,
            EDOTAGCMsg::k_EMsgDOTAClaimEventAction,
            EDOTAGCMsg::k_EMsgDOTAClaimEventActionResponse,
            EDOTAGCMsg::k_EMsgDOTAGetPeriodicResource,
            EDOTAGCMsg::k_EMsgDOTAGetPeriodicResourceResponse,
            EDOTAGCMsg::k_EMsgDOTAPeriodicResourceUpdated,
            EDOTAGCMsg::k_EMsgServerToGCSpendWager,
            EDOTAGCMsg::k_EMsgGCToGCSignoutSpendWagerToken,
            EDOTAGCMsg::k_EMsgSubmitTriviaQuestionAnswer,
            EDOTAGCMsg::k_EMsgSubmitTriviaQuestionAnswerResponse,
            EDOTAGCMsg::k_EMsgClientToGCGiveTip,
            EDOTAGCMsg::k_EMsgClientToGCGiveTipResponse,
            EDOTAGCMsg::k_EMsgStartTriviaSession,
            EDOTAGCMsg::k_EMsgStartTriviaSessionResponse,
            EDOTAGCMsg::k_EMsgAnchorPhoneNumberRequest,
            EDOTAGCMsg::k_EMsgAnchorPhoneNumberResponse,
            EDOTAGCMsg::k_EMsgUnanchorPhoneNumberRequest,
            EDOTAGCMsg::k_EMsgUnanchorPhoneNumberResponse,
            EDOTAGCMsg::k_EMsgGCToClientTipNotification,
            EDOTAGCMsg::k_EMsgClientToGCRequestSlarkGameResult,
            EDOTAGCMsg::k_EMsgClientToGCRequestSlarkGameResultResponse,
            EDOTAGCMsg::k_EMsgGCToGCSignoutSpendRankWager,
            EDOTAGCMsg::k_EMsgGCToGCGetFavoriteTeam,
            EDOTAGCMsg::k_EMsgGCToGCGetFavoriteTeamResponse,
            EDOTAGCMsg::k_EMsgSignOutEventGameData,
            EDOTAGCMsg::k_EMsgGCToClientAllStarVotesRequest,
            EDOTAGCMsg::k_EMsgGCToClientAllStarVotesReply,
            EDOTAGCMsg::k_EMsgGCToClientAllStarVotesSubmit,
            EDOTAGCMsg::k_EMsgGCToClientAllStarVotesSubmitReply,
            EDOTAGCMsg::k_EMsgClientToGCQuickStatsRequest,
            EDOTAGCMsg::k_EMsgClientToGCQuickStatsResponse,
            EDOTAGCMsg::k_EMsgGCToGCSubtractEventPointsFromUser,
            EDOTAGCMsg::k_EMsgSelectionPriorityChoiceRequest,
            EDOTAGCMsg::k_EMsgSelectionPriorityChoiceResponse,
            EDOTAGCMsg::k_EMsgGCToGCCompendiumInGamePredictionResults,
            EDOTAGCMsg::k_EMsgGameAutographReward,
            EDOTAGCMsg::k_EMsgGameAutographRewardResponse,
            EDOTAGCMsg::k_EMsgServerToGCMatchPlayerItemPurchaseHistory,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EDOTAGCMsg>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EDOTAGCMsg", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EDOTAGCMsg {
}

impl ::protobuf::reflect::ProtobufValue for EDOTAGCMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bdota_gcmessages_msgid.proto*\x8d\xe3\x01\n\nEDOTAGCMsg\x12\x15\n\
    \x10k_EMsgGCDOTABase\x10\xd86\x12\x1c\n\x17k_EMsgGCGeneralResponse\x10\
    \xd96\x12\x1d\n\x18k_EMsgGCGameMatchSignOut\x10\xdc6\x12%\n\x20k_EMsgGCG\
    ameMatchSignOutResponse\x10\xdd6\x12\x1c\n\x17k_EMsgGCJoinChatChannel\
    \x10\xe16\x12$\n\x1fk_EMsgGCJoinChatChannelResponse\x10\xe26\x12\x1f\n\
    \x1ak_EMsgGCOtherJoinedChannel\x10\xe56\x12\x1d\n\x18k_EMsgGCOtherLeftCh\
    annel\x10\xe66\x12\x1d\n\x18k_EMsgGCMatchHistoryList\x10\xe96\x12\"\n\
    \x1dk_EMsgServerToGCRequestStatus\x10\xf26\x12\x1d\n\x18k_EMsgGCGetRecen\
    tMatches\x10\xf36\x12\"\n\x1dk_EMsgGCRecentMatchesResponse\x10\xf46\x12\
    \x1e\n\x19k_EMsgGCStartFindingMatch\x10\xf96\x12\x1d\n\x18k_EMsgGCConnec\
    tedPlayers\x10\xfa6\x12\x1f\n\x1ak_EMsgGCAbandonCurrentGame\x10\xfb6\x12\
    \x1d\n\x18k_EMsgGCStopFindingMatch\x10\xfc6\x12\x20\n\x1bk_EMsgGCPractic\
    eLobbyCreate\x10\xfe6\x12\x1f\n\x1ak_EMsgGCPracticeLobbyLeave\x10\x807\
    \x12\x20\n\x1bk_EMsgGCPracticeLobbyLaunch\x10\x817\x12\x1e\n\x19k_EMsgGC\
    PracticeLobbyList\x10\x827\x12&\n!k_EMsgGCPracticeLobbyListResponse\x10\
    \x837\x12\x1e\n\x19k_EMsgGCPracticeLobbyJoin\x10\x847\x12$\n\x1fk_EMsgGC\
    PracticeLobbySetDetails\x10\x867\x12%\n\x20k_EMsgGCPracticeLobbySetTeamS\
    lot\x10\x877\x12)\n$k_EMsgGCInitialQuestionnaireResponse\x10\x897\x12\"\
    \n\x1dk_EMsgGCPracticeLobbyResponse\x10\x8f7\x12\"\n\x1dk_EMsgGCBroadcas\
    tNotification\x10\x907\x12!\n\x1ck_EMsgGCLiveScoreboardUpdate\x10\x917\
    \x12#\n\x1ek_EMsgGCRequestChatChannelList\x10\x947\x12+\n&k_EMsgGCReques\
    tChatChannelListResponse\x10\x957\x12\x1b\n\x16k_EMsgGCRequestMatches\
    \x10\x987\x12#\n\x1ek_EMsgGCRequestMatchesResponse\x10\x997\x12#\n\x1ek_\
    EMsgGCRequestPlayerResources\x10\x9c7\x12+\n&k_EMsgGCRequestPlayerResour\
    cesResponse\x10\x9d7\x12\x14\n\x0fk_EMsgGCReadyUp\x10\x9e7\x12'\n\"k_EMs\
    gGCKickedFromMatchmakingQueue\x10\x9f7\x12\x1b\n\x16k_EMsgGCLeaverDetect\
    ed\x10\xa07\x12\x1f\n\x1ak_EMsgGCSpectateFriendGame\x10\xa17\x12'\n\"k_E\
    MsgGCSpectateFriendGameResponse\x10\xa27\x12\x1a\n\x15k_EMsgGCPlayerRepo\
    rts\x10\xa37\x12$\n\x1fk_EMsgGCReportsRemainingRequest\x10\xa47\x12%\n\
    \x20k_EMsgGCReportsRemainingResponse\x10\xa57\x12\x1f\n\x1ak_EMsgGCSubmi\
    tPlayerReport\x10\xa67\x12'\n\"k_EMsgGCSubmitPlayerReportResponse\x10\
    \xa77\x12\x1e\n\x19k_EMsgGCPracticeLobbyKick\x10\xa97\x12\x20\n\x1bk_EMs\
    gGCReportCountsRequest\x10\xaa7\x12!\n\x1ck_EMsgGCReportCountsResponse\
    \x10\xab7\x12\x1d\n\x18k_EMsgGCRequestSaveGames\x10\xac7\x12#\n\x1ek_EMs\
    gGCRequestSaveGamesServer\x10\xad7\x12%\n\x20k_EMsgGCRequestSaveGamesRes\
    ponse\x10\xae7\x12#\n\x1ek_EMsgGCLeaverDetectedResponse\x10\xaf7\x12\"\n\
    \x1dk_EMsgGCPlayerFailedToConnect\x10\xb07\x12\x1d\n\x18k_EMsgGCGCToRela\
    yConnect\x10\xb17\x12%\n\x20k_EMsgGCGCToRelayConnectresponse\x10\xb27\
    \x12\x16\n\x11k_EMsgGCWatchGame\x10\xb37\x12\x1e\n\x19k_EMsgGCWatchGameR\
    esponse\x10\xb47\x12\x1d\n\x18k_EMsgGCBanStatusRequest\x10\xb57\x12\x1e\
    \n\x19k_EMsgGCBanStatusResponse\x10\xb67\x12\x20\n\x1bk_EMsgGCMatchDetai\
    lsRequest\x10\xb77\x12!\n\x1ck_EMsgGCMatchDetailsResponse\x10\xb87\x12\
    \x1c\n\x17k_EMsgGCCancelWatchGame\x10\xb97\x12\x1b\n\x16k_EMsgGCProfileR\
    equest\x10\xba7\x12\x1c\n\x17k_EMsgGCProfileResponse\x10\xbb7\x12\x12\n\
    \rk_EMsgGCPopup\x10\xbe7\x12,\n'k_EMsgGCDOTAClearNotifySuccessfulReport\
    \x10\xc07\x12+\n&k_EMsgGCFriendPracticeLobbyListRequest\x10\xc77\x12,\n'\
    k_EMsgGCFriendPracticeLobbyListResponse\x10\xc87\x12&\n!k_EMsgGCPractice\
    LobbyJoinResponse\x10\xc97\x12%\n\x20k_EMsgClientEconNotification_Job\
    \x10\xca7\x12\x17\n\x12k_EMsgGCCreateTeam\x10\xcb7\x12\x1f\n\x1ak_EMsgGC\
    CreateTeamResponse\x10\xcc7\x12\x15\n\x10k_EMsgGCTeamData\x10\xd17\x12#\
    \n\x1ek_EMsgGCTeamInvite_InviterToGC\x10\xd27\x124\n/k_EMsgGCTeamInvite_\
    GCImmediateResponseToInviter\x10\xd37\x12*\n%k_EMsgGCTeamInvite_GCReques\
    tToInvitee\x10\xd47\x12+\n&k_EMsgGCTeamInvite_InviteeResponseToGC\x10\
    \xd57\x12+\n&k_EMsgGCTeamInvite_GCResponseToInviter\x10\xd67\x12+\n&k_EM\
    sgGCTeamInvite_GCResponseToInvitee\x10\xd77\x12\x1b\n\x16k_EMsgGCKickTea\
    mMember\x10\xd87\x12#\n\x1ek_EMsgGCKickTeamMemberResponse\x10\xd97\x12\
    \x16\n\x11k_EMsgGCLeaveTeam\x10\xda7\x12\x1e\n\x19k_EMsgGCLeaveTeamRespo\
    nse\x10\xdb7\x12#\n\x1ek_EMsgGCSuggestTeamMatchmaking\x10\xdc7\x12%\n\
    \x20k_EMsgGCPlayerHeroesFavoritesAdd\x10\xdd7\x12(\n#k_EMsgGCPlayerHeroe\
    sFavoritesRemove\x10\xde7\x12\x1c\n\x17k_EMsgGCSetShowcaseHero\x10\xe57\
    \x12%\n\x20k_EMsgGCApplyTeamToPracticeLobby\x10\xe67\x12+\n&k_EMsgGCRequ\
    estInternatinalTicketEmail\x10\xe77\x12\x1e\n\x19k_EMsgGCTransferTeamAdm\
    in\x10\xe87\x12\x1c\n\x17k_EMsgRequestLeagueInfo\x10\xeb7\x12\x1d\n\x18k\
    _EMsgResponseLeagueInfo\x10\xec7\x12.\n)k_EMsgGCPracticeLobbyJoinBroadca\
    stChannel\x10\xed7\x12!\n\x1ck_EMsgGC_TournamentItemEvent\x10\xee7\x12)\
    \n$k_EMsgGC_TournamentItemEventResponse\x10\xef7\x12\x18\n\x13k_EMsgCast\
    MatchVote\x10\xf07\x12\x20\n\x1bk_EMsgCastMatchVoteResponse\x10\xf17\x12\
    \x1c\n\x17k_EMsgRetrieveMatchVote\x10\xf27\x12$\n\x1fk_EMsgRetrieveMatch\
    VoteResponse\x10\xf37\x12\x16\n\x11k_EMsgTeamFanfare\x10\xf47\x12\x1e\n\
    \x19k_EMsgResponseTeamFanfare\x10\xf57\x12&\n!k_EMsgGC_GameServerUploadS\
    aveGame\x10\xf67\x12&\n!k_EMsgGC_GameServerSaveGameResult\x10\xf77\x12#\
    \n\x1ek_EMsgGC_GameServerGetLoadGame\x10\xf87\x12)\n$k_EMsgGC_GameServer\
    GetLoadGameResult\x10\xf97\x12\x1c\n\x17k_EMsgGCEditTeamDetails\x10\xfe7\
    \x12$\n\x1fk_EMsgGCEditTeamDetailsResponse\x10\xff7\x12\x1f\n\x1ak_EMsgG\
    CProTeamListRequest\x10\x808\x12\x20\n\x1bk_EMsgGCProTeamListResponse\
    \x10\x818\x12\x1a\n\x15k_EMsgGCReadyUpStatus\x10\x828\x12\x17\n\x12k_EMs\
    gGCHallOfFame\x10\x838\x12\x1e\n\x19k_EMsgGCHallOfFameRequest\x10\x848\
    \x12\x1f\n\x1ak_EMsgGCHallOfFameResponse\x10\x858\x12&\n!k_EMsgGCGenerat\
    eDiretidePrizeList\x10\x868\x12!\n\x1ck_EMsgGCRewardDiretidePrizes\x10\
    \x888\x12+\n&k_EMsgGCDiretidePrizesRewardedResponse\x10\x898\x12&\n!k_EM\
    sgGCHalloweenHighScoreRequest\x10\x8a8\x12'\n\"k_EMsgGCHalloweenHighScor\
    eResponse\x10\x8b8\x12.\n)k_EMsgGCGenerateDiretidePrizeListResponse\x10\
    \x8c8\x12#\n\x1ek_EMsgGCStorePromoPagesRequest\x10\x8e8\x12$\n\x1fk_EMsg\
    GCStorePromoPagesResponse\x10\x8f8\x12\x1f\n\x1ak_EMsgGCToGCMatchComplet\
    ed\x10\x928\x12!\n\x1ck_EMsgGCBalancedShuffleLobby\x10\x948\x12&\n!k_EMs\
    gGCToGCCheckLeaguePermission\x10\x958\x12.\n)k_EMsgGCToGCCheckLeaguePerm\
    issionResponse\x10\x968\x12\"\n\x1dk_EMsgGCLeagueScheduleRequest\x10\x97\
    8\x12#\n\x1ek_EMsgGCLeagueScheduleResponse\x10\x988\x12\x1f\n\x1ak_EMsgG\
    CLeagueScheduleEdit\x10\x998\x12'\n\"k_EMsgGCLeagueScheduleEditResponse\
    \x10\x9a8\x12\"\n\x1dk_EMsgGCLeaguesInMonthRequest\x10\x9b8\x12#\n\x1ek_\
    EMsgGCLeaguesInMonthResponse\x10\x9c8\x12$\n\x1fk_EMsgGCMatchmakingStats\
    Request\x10\x9d8\x12%\n\x20k_EMsgGCMatchmakingStatsResponse\x10\x9e8\x12\
    \x1a\n\x15k_EMsgGCBotGameCreate\x10\x9f8\x12\"\n\x1dk_EMsgGCSetMatchHist\
    oryAccess\x10\xa08\x12*\n%k_EMsgGCSetMatchHistoryAccessResponse\x10\xa18\
    \x12\x1c\n\x17k_EMsgUpgradeLeagueItem\x10\xa38\x12$\n\x1fk_EMsgUpgradeLe\
    agueItemResponse\x10\xa48\x12%\n\x20k_EMsgGCTeamMemberProfileRequest\x10\
    \xa58\x12\"\n\x1dk_EMsgGCWatchDownloadedReplay\x10\xa68\x12\x20\n\x1bk_E\
    MsgGCSetMapLocationState\x10\xa78\x12(\n#k_EMsgGCSetMapLocationStateResp\
    onse\x10\xa88\x12\x1e\n\x19k_EMsgGCResetMapLocations\x10\xa98\x12&\n!k_E\
    MsgGCResetMapLocationsResponse\x10\xaa8\x12\x1d\n\x18k_EMsgGCSetFeatured\
    Items\x10\xac8\x12\x1a\n\x15k_EMsgGCFeaturedItems\x10\xaf8\x12$\n\x1fk_E\
    MsgRefreshPartnerAccountLink\x10\xb08\x12$\n\x1fk_EMsgClientsRejoinChatC\
    hannels\x10\xb18\x12\x20\n\x1bk_EMsgGCToGCGetUserChatInfo\x10\xb28\x12(\
    \n#k_EMsgGCToGCGetUserChatInfoResponse\x10\xb38\x12%\n\x20k_EMsgGCToGCLe\
    aveAllChatChannels\x10\xb48\x12%\n\x20k_EMsgGCToGCUpdateAccountChatBan\
    \x10\xb58\x12\x1f\n\x1ak_EMsgGCGuildCreateRequest\x10\xb68\x12\x20\n\x1b\
    k_EMsgGCGuildCreateResponse\x10\xb78\x12'\n\"k_EMsgGCGuildSetAccountRole\
    Request\x10\xb88\x12(\n#k_EMsgGCGuildSetAccountRoleResponse\x10\xb98\x12\
    \x1d\n\x18k_EMsgGCRequestGuildData\x10\xba8\x12\x16\n\x11k_EMsgGCGuildDa\
    ta\x10\xbb8\x12&\n!k_EMsgGCGuildInviteAccountRequest\x10\xbc8\x12'\n\"k_\
    EMsgGCGuildInviteAccountResponse\x10\xbd8\x12%\n\x20k_EMsgGCGuildCancelI\
    nviteRequest\x10\xbe8\x12&\n!k_EMsgGCGuildCancelInviteResponse\x10\xbf8\
    \x12&\n!k_EMsgGCGuildUpdateDetailsRequest\x10\xc08\x12'\n\"k_EMsgGCGuild\
    UpdateDetailsResponse\x10\xc18\x12$\n\x1fk_EMsgGCToGCCanInviteUserToTeam\
    \x10\xc28\x12,\n'k_EMsgGCToGCCanInviteUserToTeamResponse\x10\xc38\x12\
    \x1c\n\x17k_EMsgGCToGCGetUserRank\x10\xc48\x12$\n\x1fk_EMsgGCToGCGetUser\
    RankResponse\x10\xc58\x12\x20\n\x1bk_EMsgGCToGCUpdateTeamStats\x10\xc88\
    \x12\x1c\n\x17k_EMsgGCToGCGetTeamRank\x10\xc98\x12$\n\x1fk_EMsgGCToGCGet\
    TeamRankResponse\x10\xca8\x12\x20\n\x1bk_EMsgGCPassportDataRequest\x10\
    \xd08\x12!\n\x1ck_EMsgGCPassportDataResponse\x10\xd18\x12\x1b\n\x16k_EMs\
    gGCNotInGuildData\x10\xd38\x12\x1c\n\x17k_EMsgGCGuildInviteData\x10\xd68\
    \x12\x1f\n\x1ak_EMsgGCToGCGetLeagueAdmin\x10\xd78\x12'\n\"k_EMsgGCToGCGe\
    tLeagueAdminResponse\x10\xd88\x12#\n\x1ek_EMsgGCRequestLeaguePrizePool\
    \x10\xda8\x12+\n&k_EMsgGCRequestLeaguePrizePoolResponse\x10\xdb8\x12,\n'\
    k_EMsgGCToGCUpdateOpenGuildPartyRequest\x10\xdd8\x12-\n(k_EMsgGCToGCUpda\
    teOpenGuildPartyResponse\x10\xde8\x12-\n(k_EMsgGCToGCDestroyOpenGuildPar\
    tyRequest\x10\xdf8\x12.\n)k_EMsgGCToGCDestroyOpenGuildPartyResponse\x10\
    \xe08\x12\x1f\n\x1ak_EMsgGCGuildUpdateMessage\x10\xe18\x12%\n\x20k_EMsgG\
    CPartySetOpenGuildRequest\x10\xe28\x12&\n!k_EMsgGCPartySetOpenGuildRespo\
    nse\x10\xe38\x12\"\n\x1dk_EMsgGCGuildOpenPartyRefresh\x10\xe48\x12&\n!k_\
    EMsgGCJoinOpenGuildPartyRequest\x10\xe58\x12'\n\"k_EMsgGCJoinOpenGuildPa\
    rtyResponse\x10\xe68\x12\x1d\n\x18k_EMsgGCLeaveChatChannel\x10\xe88\x12\
    \x18\n\x13k_EMsgGCChatMessage\x10\xe98\x12\x1d\n\x18k_EMsgGCGetHeroStand\
    ings\x10\xea8\x12%\n\x20k_EMsgGCGetHeroStandingsResponse\x10\xeb8\x12!\n\
    \x1ck_EMsgGCGuildEditLogoRequest\x10\xef8\x12\"\n\x1dk_EMsgGCGuildEditLo\
    goResponse\x10\xf08\x12.\n)k_EMsgGCGuildmatePracticeLobbyListRequest\x10\
    \xf18\x12/\n*k_EMsgGCGuildmatePracticeLobbyListResponse\x10\xf28\x12*\n%\
    k_EMsgGCItemEditorReservationsRequest\x10\xf38\x12+\n&k_EMsgGCItemEditor\
    ReservationsResponse\x10\xf48\x12%\n\x20k_EMsgGCItemEditorReserveItemDef\
    \x10\xf58\x12-\n(k_EMsgGCItemEditorReserveItemDefResponse\x10\xf68\x12)\
    \n$k_EMsgGCItemEditorReleaseReservation\x10\xf78\x121\n,k_EMsgGCItemEdit\
    orReleaseReservationResponse\x10\xf88\x12!\n\x1ck_EMsgGCRewardTutorialPr\
    izes\x10\xf98\x12*\n%k_EMsgGCLastHitChallengeHighScorePost\x10\xfa8\x12-\
    \n(k_EMsgGCLastHitChallengeHighScoreRequest\x10\xfb8\x12.\n)k_EMsgGCLast\
    HitChallengeHighScoreResponse\x10\xfc8\x12'\n\"k_EMsgGCCreateFantasyLeag\
    ueRequest\x10\xfd8\x12(\n#k_EMsgGCCreateFantasyLeagueResponse\x10\xfe8\
    \x12%\n\x20k_EMsgGCFantasyLeagueInfoRequest\x10\x819\x12&\n!k_EMsgGCFant\
    asyLeagueInfoResponse\x10\x829\x12\x1e\n\x19k_EMsgGCFantasyLeagueInfo\
    \x10\x839\x12%\n\x20k_EMsgGCCreateFantasyTeamRequest\x10\x849\x12&\n!k_E\
    MsgGCCreateFantasyTeamResponse\x10\x859\x12#\n\x1ek_EMsgGCEditFantasyTea\
    mRequest\x10\x869\x12$\n\x1fk_EMsgGCEditFantasyTeamResponse\x10\x879\x12\
    4\n/k_EMsgGCFantasyTeamInfoRequestByFantasyLeagueID\x10\x889\x123\n.k_EM\
    sgGCFantasyTeamInfoRequestByOwnerAccountID\x10\x899\x12$\n\x1fk_EMsgGCFa\
    ntasyTeamInfoResponse\x10\x8a9\x12\x1c\n\x17k_EMsgGCFantasyTeamInfo\x10\
    \x8b9\x12#\n\x1ek_EMsgGCFantasyLivePlayerStats\x10\x8c9\x12$\n\x1fk_EMsg\
    GCFantasyFinalPlayerStats\x10\x8d9\x12\x19\n\x14k_EMsgGCFantasyMatch\x10\
    \x8e9\x12$\n\x1fk_EMsgGCFantasyTeamScoreRequest\x10\x909\x12%\n\x20k_EMs\
    gGCFantasyTeamScoreResponse\x10\x919\x12(\n#k_EMsgGCFantasyTeamStandings\
    Request\x10\x929\x12)\n$k_EMsgGCFantasyTeamStandingsResponse\x10\x939\
    \x12&\n!k_EMsgGCFantasyPlayerScoreRequest\x10\x949\x12'\n\"k_EMsgGCFanta\
    syPlayerScoreResponse\x10\x959\x12*\n%k_EMsgGCFantasyPlayerStandingsRequ\
    est\x10\x969\x12+\n&k_EMsgGCFantasyPlayerStandingsResponse\x10\x979\x12\
    \x1b\n\x16k_EMsgGCFlipLobbyTeams\x10\x989\x12\x1d\n\x18k_EMsgGCCustomGam\
    eCreate\x10\x999\x12%\n\x20k_EMsgGCFantasyPlayerInfoRequest\x10\x9a9\x12\
    &\n!k_EMsgGCFantasyPlayerInfoResponse\x10\x9b9\x12-\n(k_EMsgGCToGCProces\
    sPlayerReportForTarget\x10\x9c9\x12%\n\x20k_EMsgGCToGCProcessReportSucce\
    ss\x10\x9d9\x12%\n\x20k_EMsgGCNotifyAccountFlagsChange\x10\x9e9\x12\x1e\
    \n\x19k_EMsgGCSetProfilePrivacy\x10\x9f9\x12&\n!k_EMsgGCSetProfilePrivac\
    yResponse\x10\xa09\x12\x20\n\x1bk_EMsgGCSteamProfileRequest\x10\xa19\x12\
    (\n#k_EMsgGCSteamProfileRequestResponse\x10\xa29\x12+\n&k_EMsgGCFantasyL\
    eagueCreateInfoRequest\x10\xa39\x12,\n'k_EMsgGCFantasyLeagueCreateInfoRe\
    sponse\x10\xa49\x12+\n&k_EMsgGCFantasyLeagueInviteInfoRequest\x10\xa59\
    \x12,\n'k_EMsgGCFantasyLeagueInviteInfoResponse\x10\xa69\x12\x1e\n\x19k_\
    EMsgGCClientIgnoredUser\x10\xa79\x12'\n\"k_EMsgGCFantasyLeagueCreateRequ\
    est\x10\xa89\x12(\n#k_EMsgGCFantasyLeagueCreateResponse\x10\xa99\x12%\n\
    \x20k_EMsgGCFantasyTeamCreateRequest\x10\xaa9\x12&\n!k_EMsgGCFantasyTeam\
    CreateResponse\x10\xab9\x12/\n*k_EMsgGCFantasyLeagueFriendJoinListReques\
    t\x10\xac9\x120\n+k_EMsgGCFantasyLeagueFriendJoinListResponse\x10\xad9\
    \x12\x1c\n\x17k_EMsgGCClientSuspended\x10\xae9\x12\x20\n\x1bk_EMsgGCPart\
    yMemberSetCoach\x10\xaf9\x12,\n'k_EMsgGCFantasyLeagueEditInvitesRequest\
    \x10\xb09\x12-\n(k_EMsgGCFantasyLeagueEditInvitesResponse\x10\xb19\x12\"\
    \n\x1dk_EMsgGCPracticeLobbySetCoach\x10\xb29\x12)\n$k_EMsgGCFantasyLeagu\
    eEditInfoRequest\x10\xb39\x12*\n%k_EMsgGCFantasyLeagueEditInfoResponse\
    \x10\xb49\x12,\n'k_EMsgGCFantasyLeagueDraftStatusRequest\x10\xb59\x12%\n\
    \x20k_EMsgGCFantasyLeagueDraftStatus\x10\xb69\x12,\n'k_EMsgGCFantasyLeag\
    ueDraftPlayerRequest\x10\xb79\x12-\n(k_EMsgGCFantasyLeagueDraftPlayerRes\
    ponse\x10\xb89\x12)\n$k_EMsgGCFantasyLeagueMatchupsRequest\x10\xb99\x12*\
    \n%k_EMsgGCFantasyLeagueMatchupsResponse\x10\xba9\x12)\n$k_EMsgGCFantasy\
    TeamRosterSwapRequest\x10\xbb9\x12*\n%k_EMsgGCFantasyTeamRosterSwapRespo\
    nse\x10\xbc9\x12%\n\x20k_EMsgGCFantasyTeamRosterRequest\x10\xbd9\x12&\n!\
    k_EMsgGCFantasyTeamRosterResponse\x10\xbe9\x12\x1f\n\x1ak_EMsgGCNexonPar\
    tnerUpdate\x10\xbf9\x12*\n%k_EMsgGCToGCProcessPCBangRewardPoints\x10\xc0\
    9\x12,\n'k_EMsgGCFantasyTeamRosterAddDropRequest\x10\xc19\x12-\n(k_EMsgG\
    CFantasyTeamRosterAddDropResponse\x10\xc29\x12&\n!k_EMsgPresentedClientT\
    erminateDlg\x10\xc39\x12/\n*k_EMsgGCFantasyPlayerHisoricalStatsRequest\
    \x10\xc49\x120\n+k_EMsgGCFantasyPlayerHisoricalStatsResponse\x10\xc59\
    \x12%\n\x20k_EMsgGCPCBangTimedRewardMessage\x10\xc69\x12,\n'k_EMsgGCLobb\
    yUpdateBroadcastChannelInfo\x10\xc79\x12%\n\x20k_EMsgGCFantasyTeamTrades\
    Request\x10\xc89\x12&\n!k_EMsgGCFantasyTeamTradesResponse\x10\xc99\x12*\
    \n%k_EMsgGCFantasyTeamTradeCancelRequest\x10\xca9\x12+\n&k_EMsgGCFantasy\
    TeamTradeCancelResponse\x10\xcb9\x12$\n\x1fk_EMsgGCToGCGrantTournamentIt\
    em\x10\xcc9\x12)\n$k_EMsgGCProcessFantasyScheduledEvent\x10\xcd9\x12&\n!\
    k_EMsgGCToGCGrantPCBangRewardItem\x10\xce9\x12)\n$k_EMsgGCToGCUpgradeTwi\
    tchViewerItems\x10\xcf9\x12'\n\"k_EMsgGCToGCGetLiveMatchAffiliates\x10\
    \xd09\x12/\n*k_EMsgGCToGCGetLiveMatchAffiliatesResponse\x10\xd19\x12*\n%\
    k_EMsgGCToGCUpdatePlayerPennantCounts\x10\xd29\x12'\n\"k_EMsgGCToGCGetPl\
    ayerPennantCounts\x10\xd39\x12/\n*k_EMsgGCToGCGetPlayerPennantCountsResp\
    onse\x10\xd49\x12.\n)k_EMsgGCGameMatchSignOutPermissionRequest\x10\xd59\
    \x12/\n*k_EMsgGCGameMatchSignOutPermissionResponse\x10\xd69\x12&\n!k_EMs\
    gDOTAChatChannelMemberUpdate\x10\xd79\x12\x1f\n\x1ak_EMsgDOTAAwardEventP\
    oints\x10\xd89\x12\x1d\n\x18k_EMsgDOTAGetEventPoints\x10\xdb9\x12%\n\x20\
    k_EMsgDOTAGetEventPointsResponse\x10\xdc9\x12(\n#k_EMsgGCToGCSignoutAwar\
    dEventPoints\x10\xde9\x12!\n\x1ck_EMsgDOTASendFriendRecruits\x10\xe19\
    \x12$\n\x1fk_EMsgDOTAFriendRecruitsRequest\x10\xe29\x12%\n\x20k_EMsgDOTA\
    FriendRecruitsResponse\x10\xe39\x12/\n*k_EMsgDOTAFriendRecruitInviteAcce\
    ptDecline\x10\xe49\x12'\n\"k_EMsgGCPartyLeaderWatchGamePrompt\x10\xe59\
    \x12#\n\x1ek_EMsgDOTAFrostivusTimeElapsed\x10\xe69\x12#\n\x1ek_EMsgDOTAL\
    iveLeagueGameUpdate\x10\xea9\x12\x1e\n\x19k_EMsgDOTAChatGetUserList\x10\
    \xeb9\x12&\n!k_EMsgDOTAChatGetUserListResponse\x10\xec9\x12#\n\x1ek_EMsg\
    GCCompendiumSetSelection\x10\xed9\x12\"\n\x1dk_EMsgGCCompendiumDataReque\
    st\x10\xee9\x12#\n\x1ek_EMsgGCCompendiumDataResponse\x10\xef9\x12$\n\x1f\
    k_EMsgDOTAGetPlayerMatchHistory\x10\xf09\x12,\n'k_EMsgDOTAGetPlayerMatch\
    HistoryResponse\x10\xf19\x12$\n\x1fk_EMsgGCToGCMatchmakingAddParty\x10\
    \xf29\x12'\n\"k_EMsgGCToGCMatchmakingRemoveParty\x10\xf39\x12,\n'k_EMsgG\
    CToGCMatchmakingRemoveAllParties\x10\xf49\x12&\n!k_EMsgGCToGCMatchmaking\
    MatchFound\x10\xf59\x12+\n&k_EMsgGCToGCUpdateMatchManagementStats\x10\
    \xf69\x12'\n\"k_EMsgGCToGCUpdateMatchmakingStats\x10\xf79\x12\x20\n\x1bk\
    _EMsgGCToServerPingRequest\x10\xf89\x12!\n\x1ck_EMsgGCToServerPingRespon\
    se\x10\xf99\x12#\n\x1ek_EMsgGCToServerConsoleCommand\x10\xfa9\x12)\n$k_E\
    MsgGCToGCUpdateLiveLeagueGameInfo\x10\xfc9\x12\x19\n\x14k_EMsgGCMakeOffe\
    ring\x10\xff9\x12\x1d\n\x18k_EMsgGCRequestOfferings\x10\x80:\x12%\n\x20k\
    _EMsgGCRequestOfferingsResponse\x10\x81:\x12#\n\x1ek_EMsgGCToGCProcessMa\
    tchLeaver\x10\x82:\x12!\n\x1ck_EMsgGCNotificationsRequest\x10\x83:\x12\"\
    \n\x1dk_EMsgGCNotificationsResponse\x10\x84:\x12#\n\x1ek_EMsgGCToGCModif\
    yNotification\x10\x85:\x12$\n\x1fk_EMsgGCToGCSetNewNotifications\x10\x86\
    :\x12!\n\x1ck_EMsgGCToGCSetIsLeagueAdmin\x10\x87:\x12\x1d\n\x18k_EMsgGCL\
    eagueAdminState\x10\x88:\x12%\n\x20k_EMsgGCToGCSendLeagueAdminState\x10\
    \x89:\x12\x1c\n\x17k_EMsgGCLeagueAdminList\x10\x8a:\x12)\n$k_EMsgGCNotif\
    icationsMarkReadRequest\x10\x8b:\x12\x1e\n\x19k_EMsgGCFantasyMessageAdd\
    \x10\x8c:\x12#\n\x1ek_EMsgGCFantasyMessagesRequest\x10\x8d:\x12$\n\x1fk_\
    EMsgGCFantasyMessagesResponse\x10\x8e:\x12+\n&k_EMsgGCFantasyScheduledMa\
    tchesRequest\x10\x8f:\x12,\n'k_EMsgGCFantasyScheduledMatchesResponse\x10\
    \x90:\x12\"\n\x1dk_EMsgGCToGCGrantLeagueAccess\x10\x91:\x12\x1c\n\x17k_E\
    MsgGCEventGameCreate\x10\x93:\x12*\n%k_EMsgGCPerfectWorldUserLookupReque\
    st\x10\x94:\x12+\n&k_EMsgGCPerfectWorldUserLookupResponse\x10\x95:\x12\
    \x1f\n\x1ak_EMsgGCFantasyRemoveOwner\x10\x98:\x12'\n\"k_EMsgGCFantasyRem\
    oveOwnerResponse\x10\x99:\x12(\n#k_EMsgGCRequestBatchPlayerResources\x10\
    \x9a:\x120\n+k_EMsgGCRequestBatchPlayerResourcesResponse\x10\x9b:\x12\"\
    \n\x1dk_EMsgGCToGCSendUpdateLeagues\x10\x9c:\x12+\n&k_EMsgGCCompendiumSe\
    tSelectionResponse\x10\x9d:\x12\x1e\n\x19k_EMsgGCPlayerInfoRequest\x10\
    \x9e:\x12\x17\n\x12k_EMsgGCPlayerInfo\x10\x9f:\x12\x1d\n\x18k_EMsgGCPlay\
    erInfoSubmit\x10\xa0:\x12%\n\x20k_EMsgGCPlayerInfoSubmitResponse\x10\xa1\
    :\x12\x20\n\x1bk_EMsgGCToGCGetAccountLevel\x10\xa2:\x12(\n#k_EMsgGCToGCG\
    etAccountLevelResponse\x10\xa3:\x12\"\n\x1dk_EMsgGCToGCGetAccountPartner\
    \x10\xa4:\x12*\n%k_EMsgGCToGCGetAccountPartnerResponse\x10\xa5:\x12\"\n\
    \x1dk_EMsgGCToGCGetAccountProfile\x10\xa6:\x12*\n%k_EMsgGCToGCGetAccount\
    ProfileResponse\x10\xa7:\x12(\n#k_EMsgDOTAGetWeekendTourneySchedule\x10\
    \xa8:\x12%\n\x20k_EMsgDOTAWeekendTourneySchedule\x10\xa9:\x12+\n&k_EMsgG\
    CJoinableCustomGameModesRequest\x10\xaa:\x12,\n'k_EMsgGCJoinableCustomGa\
    meModesResponse\x10\xab:\x12)\n$k_EMsgGCJoinableCustomLobbiesRequest\x10\
    \xac:\x12*\n%k_EMsgGCJoinableCustomLobbiesResponse\x10\xad:\x12!\n\x1ck_\
    EMsgGCQuickJoinCustomLobby\x10\xae:\x12)\n$k_EMsgGCQuickJoinCustomLobbyR\
    esponse\x10\xaf:\x12&\n!k_EMsgGCToGCGrantEventPointAction\x10\xb0:\x12\
    \x1f\n\x1ak_EMsgServerGetEventPoints\x10\xb1:\x12'\n\"k_EMsgServerGetEve\
    ntPointsResponse\x10\xb2:\x12&\n!k_EMsgServerGrantSurveyPermission\x10\
    \xb3:\x12.\n)k_EMsgServerGrantSurveyPermissionResponse\x10\xb4:\x12$\n\
    \x1fk_EMsgClientProvideSurveyResult\x10\xb5:\x12'\n\"k_EMsgGCToGCSetComp\
    endiumSelection\x10\xb6:\x12#\n\x1ek_EMsgGCToGCUpdateTI4HeroQuest\x10\
    \xb8:\x12\"\n\x1dk_EMsgGCCompendiumDataChanged\x10\xb9:\x12'\n\"k_EMsgDO\
    TAFantasyLeagueFindRequest\x10\xba:\x12(\n#k_EMsgDOTAFantasyLeagueFindRe\
    sponse\x10\xbb:\x12\x19\n\x14k_EMsgGCHasItemQuery\x10\xbc:\x12\x1c\n\x17\
    k_EMsgGCHasItemResponse\x10\xbd:\x12!\n\x1ck_EMsgGCConsumeFantasyTicket\
    \x10\xbe:\x12(\n#k_EMsgGCConsumeFantasyTicketFailure\x10\xbf:\x12)\n$k_E\
    MsgGCToGCGrantEventPointActionMsg\x10\xc0:\x12&\n!k_EMsgClientToGCTrackD\
    ialogResult\x10\xc1:\x12&\n!k_EMsgGCFantasyLeaveLeagueRequest\x10\xc2:\
    \x12'\n\"k_EMsgGCFantasyLeaveLeagueResponse\x10\xc3:\x12(\n#k_EMsgGCToGC\
    GetCompendiumSelections\x10\xc4:\x120\n+k_EMsgGCToGCGetCompendiumSelecti\
    onsResponse\x10\xc5:\x12)\n$k_EMsgServerToGCMatchConnectionStats\x10\xc6\
    :\x12'\n\"k_EMsgGCToClientTournamentItemDrop\x10\xc7:\x12$\n\x1fk_EMsgSQ\
    LDelayedGrantLeagueDrop\x10\xc8:\x12'\n\"k_EMsgServerGCUpdateSpectatorCo\
    unt\x10\xc9:\x12-\n(k_EMsgGCFantasyPlayerScoreDetailsRequest\x10\xcb:\
    \x12.\n)k_EMsgGCFantasyPlayerScoreDetailsResponse\x10\xcc:\x12\x1f\n\x1a\
    k_EMsgGCToGCEmoticonUnlock\x10\xcd:\x12\x1b\n\x16k_EMsgSignOutDraftInfo\
    \x10\xce:\x12(\n#k_EMsgClientToGCEmoticonDataRequest\x10\xcf:\x12!\n\x1c\
    k_EMsgGCToClientEmoticonData\x10\xd0:\x12?\n:k_EMsgGCPracticeLobbyToggle\
    BroadcastChannelCameramanStatus\x10\xd1:\x12,\n'k_EMsgGCToGCCreateWeeken\
    dTourneyRequest\x10\xd2:\x12-\n(k_EMsgGCToGCCreateWeekendTourneyResponse\
    \x10\xd3:\x12(\n#k_EMsgClientToGCSetAdditionalEquips\x10\xd9:\x12(\n#k_E\
    MsgClientToGCGetAdditionalEquips\x10\xda:\x120\n+k_EMsgClientToGCGetAddi\
    tionalEquipsResponse\x10\xdb:\x12(\n#k_EMsgServerToGCGetAdditionalEquips\
    \x10\xdc:\x120\n+k_EMsgServerToGCGetAdditionalEquipsResponse\x10\xdd:\
    \x12\x19\n\x14k_EMsgDOTARedeemItem\x10\xde:\x12!\n\x1ck_EMsgDOTARedeemIt\
    emResponse\x10\xdf:\x12(\n#k_EMsgSQLGCToGCGrantAllHeroProgress\x10\xe0:\
    \x12'\n\"k_EMsgClientToGCGetAllHeroProgress\x10\xe1:\x12/\n*k_EMsgClient\
    ToGCGetAllHeroProgressResponse\x10\xe2:\x12#\n\x1ek_EMsgGCToGCGetServerF\
    orClient\x10\xe3:\x12+\n&k_EMsgGCToGCGetServerForClientResponse\x10\xe4:\
    \x12*\n%k_EMsgSQLProcessTournamentGameOutcome\x10\xe5:\x12\"\n\x1dk_EMsg\
    SQLGrantTrophyToAccount\x10\xe6:\x12\"\n\x1dk_EMsgClientToGCGetTrophyLis\
    t\x10\xe7:\x12*\n%k_EMsgClientToGCGetTrophyListResponse\x10\xe8:\x12\"\n\
    \x1dk_EMsgGCToClientTrophyAwarded\x10\xe9:\x12\x20\n\x1bk_EMsgGCGameBotM\
    atchSignOut\x10\xea:\x121\n,k_EMsgGCGameBotMatchSignOutPermissionRequest\
    \x10\xeb:\x12\x19\n\x14k_EMsgSignOutBotInfo\x10\xec:\x12#\n\x1ek_EMsgGCT\
    oGCUpdateProfileCards\x10\xed:\x12#\n\x1ek_EMsgClientToGCGetProfileCard\
    \x10\xee:\x12+\n&k_EMsgClientToGCGetProfileCardResponse\x10\xef:\x12#\n\
    \x1ek_EMsgServerToGCGetProfileCard\x10\xf0:\x12+\n&k_EMsgServerToGCGetPr\
    ofileCardResponse\x10\xf1:\x12(\n#k_EMsgClientToGCSetProfileCardSlots\
    \x10\xf2:\x12'\n\"k_EMsgGCToClientProfileCardUpdated\x10\xf3:\x12'\n\"k_\
    EMsgServerToGCVictoryPredictions\x10\xf4:\x12-\n(k_EMsgClientToGCMarkNot\
    ificationListRead\x10\xf6:\x12)\n$k_EMsgGCToClientNewNotificationAdded\
    \x10\xf7:\x12'\n\"k_EMsgServerToGCSuspiciousActivity\x10\xf8:\x12&\n!k_E\
    MsgSignOutCommunicationSummary\x10\xf9:\x12+\n&k_EMsgServerToGCRequestSt\
    atus_Response\x10\xfa:\x12%\n\x20k_EMsgClientToGCCreateHeroStatue\x10\
    \xfb:\x12+\n&k_EMsgGCToClientHeroStatueCreateResult\x10\xfc:\x12&\n!k_EM\
    sgGCGCToLANServerRelayConnect\x10\xfd:\x12'\n\"k_EMsgServerToGCGetIngame\
    EventData\x10\xff:\x12/\n*k_EMsgGCToGCUpdateIngameEventDataBroadcast\x10\
    \x80;\x12-\n(k_EMsgGCToServerIngameEventData_OraclePA\x10\x81;\x12(\n#k_\
    EMsgServerToGCReportKillSummaries\x10\x82;\x12$\n\x1fk_EMsgGCToGCReportK\
    illSummaries\x10\x83;\x12'\n\"k_EMsgGCToGCUpdateAssassinMinigame\x10\x84\
    ;\x12&\n!k_EMsgGCToGCFantasySetMatchLeague\x10\x85;\x12*\n%k_EMsgClientT\
    oGCRecordCompendiumStats\x10\x86;\x12(\n#k_EMsgGCItemEditorRequestLeague\
    Info\x10\x87;\x12)\n$k_EMsgGCItemEditorLeagueInfoResponse\x10\x88;\x12(\
    \n#k_EMsgGCToGCUpdatePlayerPredictions\x10\x89;\x12%\n\x20k_EMsgGCToServ\
    erPredictionResult\x10\x8a;\x120\n+k_EMsgServerToGCSignoutAwardAdditiona\
    lDrops\x10\x8b;\x12,\n'k_EMsgGCToGCSignoutAwardAdditionalDrops\x10\x8c;\
    \x12'\n\"k_EMsgGCToClientEventStatusChanged\x10\x8d;\x12\x1d\n\x18k_EMsg\
    GCHasItemDefsQuery\x10\x8e;\x12\x20\n\x1bk_EMsgGCHasItemDefsResponse\x10\
    \x8f;\x12,\n'k_EMsgGCToGCReplayMonitorValidateReplay\x10\x91;\x12\x1b\n\
    \x16k_EMsgLobbyEventPoints\x10\x94;\x12%\n\x20k_EMsgGCToGCGetCustomGameT\
    ickets\x10\x95;\x12-\n(k_EMsgGCToGCGetCustomGameTicketsResponse\x10\x96;\
    \x12!\n\x1ck_EMsgGCToGCCustomGamePlayed\x10\x98;\x12'\n\"k_EMsgGCToGCGra\
    ntEventPointsToUser\x10\x99;\x12)\n$k_EMsgGCToGCSetEventMMPanicFlushTime\
    \x10\x9a;\x12\x20\n\x1bk_EMsgGameserverCrashReport\x10\x9b;\x12(\n#k_EMs\
    gGameserverCrashReportResponse\x10\x9c;\x12(\n#k_EMsgGCToClientSteamData\
    gramTicket\x10\x9d;\x12$\n\x1fk_EMsgGCToGCGrantEventOwnership\x10\x9e;\
    \x12(\n#k_EMsgGCToGCSendAccountsEventPoints\x10\x9f;\x12*\n%k_EMsgClient\
    ToGCRerollPlayerChallenge\x10\xa0;\x12*\n%k_EMsgServerToGCRerollPlayerCh\
    allenge\x10\xa1;\x12*\n%k_EMsgGCRerollPlayerChallengeResponse\x10\xa2;\
    \x12'\n\"k_EMsgSignOutUpdatePlayerChallenge\x10\xa3;\x12#\n\x1ek_EMsgCli\
    entToGCSetPartyLeader\x10\xa4;\x12'\n\"k_EMsgClientToGCCancelPartyInvite\
    s\x10\xa5;\x12$\n\x1fk_EMsgGCToGCMasterReloadAccount\x10\xa6;\x12-\n(k_E\
    MsgSQLGrantLeagueMatchToTicketHolders\x10\xa8;\x120\n+k_EMsgClientToGCSe\
    tAdditionalEquipsResponse\x10\xa9;\x12)\n$k_EMsgGCToGCEmoticonUnlockNoRo\
    llback\x10\xaa;\x12%\n\x20k_EMsgGCToGCGetCompendiumFanfare\x10\xab;\x12$\
    \n\x1fk_EMsgServerToGCHoldEventPoints\x10\xac;\x12(\n#k_EMsgSignOutRelea\
    seEventPointHolds\x10\xad;\x12#\n\x1ek_EMsgGCToGCChatNewUserSession\x10\
    \xae;\x12$\n\x1fk_EMsgClientToGCGetLeagueSeries\x10\xaf;\x12,\n'k_EMsgCl\
    ientToGCGetLeagueSeriesResponse\x10\xb0;\x12/\n*k_EMsgSQLGCToGCSignoutUp\
    dateLeagueSchedule\x10\xb1;\x12*\n%k_EMsgGCToServerUpdateBroadcastCheers\
    \x10\xb2;\x12%\n\x20k_EMsgClientToGCApplyGemCombiner\x10\xb3;\x12+\n&k_E\
    MsgClientToGCDOTACreateStaticRecipe\x10\xb4;\x123\n.k_EMsgClientToGCDOTA\
    CreateStaticRecipeResponse\x10\xb5;\x12$\n\x1fk_EMsgClientToGCGetAllHero\
    Order\x10\xb6;\x12,\n'k_EMsgClientToGCGetAllHeroOrderResponse\x10\xb7;\
    \x12$\n\x1fk_EMsgSQLGCToGCGrantBadgePoints\x10\xb8;\x12&\n!k_EMsgGCToGCG\
    etAccountMatchStatus\x10\xb9;\x12.\n)k_EMsgGCToGCGetAccountMatchStatusRe\
    sponse\x10\xba;\x12-\n(k_EMsgGCToGCCheckOwnsEntireEmoticonRange\x10\xbb;\
    \x125\n0k_EMsgGCToGCCheckOwnsEntireEmoticonRangeResponse\x10\xbc;\x12\
    \x1d\n\x18k_EMsgGCDev_GrantWarKill\x10\xc1>\x12%\n\x20k_EMsgServerToGCLo\
    ckCharmTrading\x10\xc4>\x12'\n\"k_EMsgClientToGCPlayerStatsRequest\x10\
    \xc6>\x12(\n#k_EMsgGCToClientPlayerStatsResponse\x10\xc7>\x12#\n\x1ek_EM\
    sgGCClearPracticeLobbyTeam\x10\xc8>\x12)\n$k_EMsgClientToGCFindTopSource\
    TVGames\x10\xc9>\x121\n,k_EMsgGCToClientFindTopSourceTVGamesResponse\x10\
    \xca>\x12\x16\n\x11k_EMsgGCLobbyList\x10\xcb>\x12\x1e\n\x19k_EMsgGCLobby\
    ListResponse\x10\xcc>\x12$\n\x1fk_EMsgGCPlayerStatsMatchSignOut\x10\xcd>\
    \x121\n,k_EMsgClientToGCCustomGamePlayerCountRequest\x10\xce>\x122\n-k_E\
    MsgGCToClientCustomGamePlayerCountResponse\x10\xcf>\x121\n,k_EMsgClientT\
    oGCSocialFeedPostCommentRequest\x10\xd0>\x122\n-k_EMsgGCToClientSocialFe\
    edPostCommentResponse\x10\xd1>\x124\n/k_EMsgClientToGCCustomGamesFriends\
    PlayedRequest\x10\xd2>\x125\n0k_EMsgGCToClientCustomGamesFriendsPlayedRe\
    sponse\x10\xd3>\x123\n.k_EMsgClientToGCFriendsPlayedCustomGameRequest\
    \x10\xd4>\x124\n/k_EMsgGCToClientFriendsPlayedCustomGameResponse\x10\xd5\
    >\x12*\n%k_EMsgClientToGCFeaturedHeroesRequest\x10\xd6>\x12+\n&k_EMsgGCT\
    oClientFeaturedHeroesResponse\x10\xd7>\x12\x1f\n\x1ak_EMsgGCTopCustomGam\
    esList\x10\xd8>\x122\n-k_EMsgClientToGCSocialMatchPostCommentRequest\x10\
    \xd9>\x123\n.k_EMsgGCToClientSocialMatchPostCommentResponse\x10\xda>\x12\
    .\n)k_EMsgClientToGCSocialMatchDetailsRequest\x10\xdb>\x12/\n*k_EMsgGCTo\
    ClientSocialMatchDetailsResponse\x10\xdc>\x12!\n\x1ck_EMsgClientToGCSetP\
    artyOpen\x10\xdd>\x12%\n\x20k_EMsgClientToGCMergePartyInvite\x10\xde>\
    \x12*\n%k_EMsgGCToClientMergeGroupInviteReply\x10\xdf>\x12'\n\"k_EMsgCli\
    entToGCMergePartyResponse\x10\xe0>\x12,\n'k_EMsgGCToClientMergePartyResp\
    onseReply\x10\xe1>\x12(\n#k_EMsgClientToGCGetProfileCardStats\x10\xe2>\
    \x120\n+k_EMsgClientToGCGetProfileCardStatsResponse\x10\xe3>\x12,\n'k_EM\
    sgClientToGCTopLeagueMatchesRequest\x10\xe4>\x12,\n'k_EMsgClientToGCTopF\
    riendMatchesRequest\x10\xe5>\x12,\n'k_EMsgGCToClientProfileCardStatsUpda\
    ted\x10\xe8>\x12\"\n\x1dk_EMsgServerToGCRealtimeStats\x10\xe9>\x12+\n&k_\
    EMsgGCToServerRealtimeStatsStartStop\x10\xea>\x12%\n\x20k_EMsgGCToGCGetS\
    erversForClients\x10\xed>\x12-\n(k_EMsgGCToGCGetServersForClientsRespons\
    e\x10\xee>\x12&\n!k_EMsgGCPracticeLobbyKickFromTeam\x10\xef>\x12!\n\x1ck\
    _EMsgDOTAChatGetMemberCount\x10\xf0>\x12)\n$k_EMsgDOTAChatGetMemberCount\
    Response\x10\xf1>\x121\n,k_EMsgClientToGCSocialFeedPostMessageRequest\
    \x10\xf2>\x122\n-k_EMsgGCToClientSocialFeedPostMessageResponse\x10\xf3>\
    \x12/\n*k_EMsgCustomGameListenServerStartedLoading\x10\xf4>\x12*\n%k_EMs\
    gCustomGameClientFinishedLoading\x10\xf5>\x12/\n*k_EMsgGCPracticeLobbyCl\
    oseBroadcastChannel\x10\xf6>\x12&\n!k_EMsgGCStartFindingMatchResponse\
    \x10\xf7>\x12$\n\x1fk_EMsgSQLGCToGCGrantAccountFlag\x10\xf9>\x12\x20\n\
    \x1bk_EMsgGCToGCGetAccountFlags\x10\xfa>\x12(\n#k_EMsgGCToGCGetAccountFl\
    agsResponse\x10\xfb>\x12\x1c\n\x17k_EMsgSignOutWagerStats\x10\xfc>\x12-\
    \n(k_EMsgGCToClientTopLeagueMatchesResponse\x10\xfd>\x12-\n(k_EMsgGCToCl\
    ientTopFriendMatchesResponse\x10\xfe>\x12*\n%k_EMsgClientToGCMatchesMini\
    malRequest\x10\xff>\x12+\n&k_EMsgClientToGCMatchesMinimalResponse\x10\
    \x80?\x12&\n!k_EMsgGCToGCGetProfileBadgePoints\x10\x81?\x12.\n)k_EMsgGCT\
    oGCGetProfileBadgePointsResponse\x10\x82?\x12'\n\"k_EMsgGCToClientChatRe\
    gionsEnabled\x10\x83?\x12\x1d\n\x18k_EMsgClientToGCPingData\x10\x84?\x12\
    (\n#k_EMsgServerToGCMatchDetailsRequest\x10\x85?\x12)\n$k_EMsgGCToServer\
    MatchDetailsResponse\x10\x86?\x12%\n\x20k_EMsgGCToGCEnsureAccountInParty\
    \x10\x87?\x12-\n(k_EMsgGCToGCEnsureAccountInPartyResponse\x10\x88?\x12&\
    \n!k_EMsgClientToGCGetProfileTickets\x10\x89?\x12.\n)k_EMsgClientToGCGet\
    ProfileTicketsResponse\x10\x8a?\x12'\n\"k_EMsgGCToClientMatchGroupsVersi\
    on\x10\x8b?\x12$\n\x1fk_EMsgClientToGCH264Unsupported\x10\x8c?\x12'\n\"k\
    _EMsgClientToGCRequestH264Support\x10\x8d?\x12%\n\x20k_EMsgClientToGCGet\
    QuestProgress\x10\x8e?\x12-\n(k_EMsgClientToGCGetQuestProgressResponse\
    \x10\x8f?\x12\x19\n\x14k_EMsgSignOutXPCoins\x10\x90?\x12#\n\x1ek_EMsgGCT\
    oClientMatchSignedOut\x10\x91?\x12\x20\n\x1bk_EMsgGCGetHeroStatsHistory\
    \x10\x92?\x12(\n#k_EMsgGCGetHeroStatsHistoryResponse\x10\x93?\x12&\n!k_E\
    MsgClientToGCPrivateChatInvite\x10\x94?\x12$\n\x1fk_EMsgClientToGCPrivat\
    eChatKick\x10\x98?\x12'\n\"k_EMsgClientToGCPrivateChatPromote\x10\x99?\
    \x12&\n!k_EMsgClientToGCPrivateChatDemote\x10\x9a?\x12(\n#k_EMsgGCToClie\
    ntPrivateChatResponse\x10\x9b?\x12+\n&k_EMsgClientToGCPrivateChatInfoReq\
    uest\x10\x9c?\x12,\n'k_EMsgGCToClientPrivateChatInfoResponse\x10\x9d?\
    \x122\n-k_EMsgClientToGCLatestConductScorecardRequest\x10\x9f?\x12+\n&k_\
    EMsgClientToGCLatestConductScorecard\x10\xa0?\x12!\n\x1ck_EMsgServerToGC\
    PostMatchTip\x10\xa1?\x12)\n$k_EMsgServerToGCPostMatchTipResponse\x10\
    \xa2?\x12$\n\x1fk_EMsgClientToGCWageringRequest\x10\xa3?\x12%\n\x20k_EMs\
    gGCToClientWageringResponse\x10\xa4?\x12&\n!k_EMsgClientToGCEventGoalsRe\
    quest\x10\xa7?\x12'\n\"k_EMsgClientToGCEventGoalsResponse\x10\xa8?\x12&\
    \n!k_EMsgClientToGCLeaguePredictions\x10\xaa?\x12.\n)k_EMsgGCToClientLea\
    guePredictionsResponse\x10\xab?\x12(\n#k_EMsgGCToGCLeaguePredictionsUpda\
    te\x10\xac?\x12'\n\"k_EMsgClientToGCSuspiciousActivity\x10\xad?\x12&\n!k\
    _EMsgGCToGCAddUserToPostGameChat\x10\xae?\x12)\n$k_EMsgClientToGCHasPlay\
    erVotedForMVP\x10\xaf?\x121\n,k_EMsgClientToGCHasPlayerVotedForMVPRespon\
    se\x10\xb0?\x12\x1f\n\x1ak_EMsgClientToGCVoteForMVP\x10\xb1?\x12'\n\"k_E\
    MsgClientToGCVoteForMVPResponse\x10\xb2?\x12\"\n\x1dk_EMsgGCToGCGetEvent\
    Ownership\x10\xb3?\x12*\n%k_EMsgGCToGCGetEventOwnershipResponse\x10\xb4?\
    \x123\n.k_EMsgGCToClientAutomatedTournamentStateChange\x10\xb5?\x12'\n\"\
    k_EMsgClientToGCWeekendTourneyOpts\x10\xb6?\x12/\n*k_EMsgClientToGCWeeke\
    ndTourneyOptsResponse\x10\xb7?\x12(\n#k_EMsgClientToGCWeekendTourneyLeav\
    e\x10\xb8?\x120\n+k_EMsgClientToGCWeekendTourneyLeaveResponse\x10\xb9?\
    \x12)\n$k_EMsgClientToGCTeammateStatsRequest\x10\xbc?\x12*\n%k_EMsgClien\
    tToGCTeammateStatsResponse\x10\xbd?\x12'\n\"k_EMsgClientToGCGetGiftPermi\
    ssions\x10\xbe?\x12/\n*k_EMsgClientToGCGetGiftPermissionsResponse\x10\
    \xbf?\x12\"\n\x1dk_EMsgClientToGCVoteForArcana\x10\xc0?\x12*\n%k_EMsgCli\
    entToGCVoteForArcanaResponse\x10\xc1?\x120\n+k_EMsgClientToGCRequestArca\
    naVotesRemaining\x10\xc2?\x128\n3k_EMsgClientToGCRequestArcanaVotesRemai\
    ningResponse\x10\xc3?\x12&\n!k_EMsgGCTransferTeamAdminResponse\x10\xc4?\
    \x12\x1a\n\x15k_EMsgGCChangeTeamSub\x10\xc5?\x12\"\n\x1dk_EMsgGCChangeTe\
    amSubResponse\x10\xc6?\x12\x1d\n\x18k_EMsgGCToClientTeamInfo\x10\xc7?\
    \x12\x1e\n\x19k_EMsgGCToClientTeamsInfo\x10\xc8?\x12&\n!k_EMsgClientToGC\
    MyTeamInfoRequest\x10\xc9?\x12)\n$k_EMsgClientToGCRequestEventPointLog\
    \x10\xca?\x121\n,k_EMsgClientToGCRequestEventPointLogResponse\x10\xcb?\
    \x12$\n\x1fk_EMsgClientToGCPublishUserStat\x10\xcc?\x12\"\n\x1dk_EMsgGCT\
    oGCSignoutSpendWager\x10\xcd?\x12\x1f\n\x1ak_EMsgGCSubmitLobbyMVPVote\
    \x10\xd0?\x12'\n\"k_EMsgGCSubmitLobbyMVPVoteResponse\x10\xd1?\x12.\n)k_E\
    MsgClientToGCRequestLinaPlaysRemaining\x10\xd2?\x126\n1k_EMsgClientToGCR\
    equestLinaPlaysRemainingResponse\x10\xd3?\x12*\n%k_EMsgClientToGCRequest\
    LinaGameResult\x10\xd4?\x122\n-k_EMsgClientToGCRequestLinaGameResultResp\
    onse\x10\xd5?\x12'\n\"k_EMsgSignOutCommunityGoalProgress\x10\xd6?\x12,\n\
    'k_EMsgGCToClientLobbyMVPNotifyRecipient\x10\xd7?\x12$\n\x1fk_EMsgGCToCl\
    ientLobbyMVPAwarded\x10\xd8?\x12)\n$k_EMsgGCToClientQuestProgressUpdated\
    \x10\xd9?\x12#\n\x1ek_EMsgGCToClientWageringUpdate\x10\xda?\x12&\n!k_EMs\
    gGCToClientArcanaVotesUpdate\x10\xdb?\x12'\n\"k_EMsgClientToGCAddTI6Tree\
    Progress\x10\xdc?\x12-\n(k_EMsgClientToGCSetSpectatorLobbyDetails\x10\
    \xdd?\x125\n0k_EMsgClientToGCSetSpectatorLobbyDetailsResponse\x10\xde?\
    \x12)\n$k_EMsgClientToGCCreateSpectatorLobby\x10\xdf?\x121\n,k_EMsgClien\
    tToGCCreateSpectatorLobbyResponse\x10\xe0?\x12'\n\"k_EMsgClientToGCSpect\
    atorLobbyList\x10\xe1?\x12/\n*k_EMsgClientToGCSpectatorLobbyListResponse\
    \x10\xe2?\x12$\n\x1fk_EMsgSpectatorLobbyGameDetails\x10\xe3?\x125\n0k_EM\
    sgServerToGCStartCompendiumInGamePredictions\x10\xe4?\x123\n.k_EMsgServe\
    rToGCEndCompendiumInGamePredictions\x10\xe5?\x126\n1k_EMsgServerToGCComp\
    endiumInGamePredictionResults\x10\xe6?\x12:\n5k_EMsgServerToGCCloseCompe\
    ndiumInGamePredictionVoting\x10\xe7?\x12'\n\"k_EMsgClientToGCOpenPlayerC\
    ardPack\x10\xe8?\x12/\n*k_EMsgClientToGCOpenPlayerCardPackResponse\x10\
    \xe9?\x125\n0k_EMsgClientToGCSelectCompendiumInGamePrediction\x10\xea?\
    \x12=\n8k_EMsgClientToGCSelectCompendiumInGamePredictionResponse\x10\xeb\
    ?\x121\n,k_EMsgClientToGCWeekendTourneyGetPlayerStats\x10\xec?\x129\n4k_\
    EMsgClientToGCWeekendTourneyGetPlayerStatsResponse\x10\xed?\x12&\n!k_EMs\
    gClientToGCRecyclePlayerCard\x10\xee?\x12.\n)k_EMsgClientToGCRecyclePlay\
    erCardResponse\x10\xef?\x12)\n$k_EMsgClientToGCCreatePlayerCardPack\x10\
    \xf0?\x121\n,k_EMsgClientToGCCreatePlayerCardPackResponse\x10\xf1?\x12/\
    \n*k_EMsgClientToGCGetPlayerCardRosterRequest\x10\xf2?\x120\n+k_EMsgClie\
    ntToGCGetPlayerCardRosterResponse\x10\xf3?\x12/\n*k_EMsgClientToGCSetPla\
    yerCardRosterRequest\x10\xf4?\x120\n+k_EMsgClientToGCSetPlayerCardRoster\
    Response\x10\xf5?\x12=\n8k_EMsgServerToGCStartCompendiumInGamePrediction\
    sResponse\x10\xf6?\x12B\n=k_EMsgServerToGCCloseCompendiumInGamePredictio\
    nVotingResponse\x10\xf7?\x12;\n6k_EMsgServerToGCEndCompendiumInGamePredi\
    ctionsResponse\x10\xf8?\x12>\n9k_EMsgServerToGCCompendiumInGamePredictio\
    nResultsResponse\x10\xf9?\x12\x20\n\x1bk_EMsgLobbyBattleCupVictory\x10\
    \xfa?\x12\"\n\x1dk_EMsgGCGetPlayerCardItemInfo\x10\xfb?\x12*\n%k_EMsgGCG\
    etPlayerCardItemInfoResponse\x10\xfc?\x12/\n*k_EMsgClientToGCRequestStea\
    mDatagramTicket\x10\xfd?\x127\n2k_EMsgClientToGCRequestSteamDatagramTick\
    etResponse\x10\xfe?\x12,\n'k_EMsgGCToClientBattlePassRollupRequest\x10\
    \xff?\x12-\n(k_EMsgGCToClientBattlePassRollupResponse\x10\x80@\x12/\n*k_\
    EMsgClientToGCTransferSeasonalMMRRequest\x10\x81@\x120\n+k_EMsgClientToG\
    CTransferSeasonalMMRResponse\x10\x82@\x12+\n&k_EMsgGCToGCPublicChatCommu\
    nicationBan\x10\x83@\x12+\n&k_EMsgGCToGCUpdateAccountPublicChatBan\x10\
    \x84@\x12!\n\x1ck_EMsgGCChatReportPublicSpam\x10\x85@\x12+\n&k_EMsgClien\
    tToGCSetPartyBuilderOptions\x10\x86@\x123\n.k_EMsgClientToGCSetPartyBuil\
    derOptionsResponse\x10\x87@\x12#\n\x1ek_EMsgGCToClientPlaytestStatus\x10\
    \x88@\x12!\n\x1ck_EMsgClientToGCJoinPlaytest\x10\x89@\x12)\n$k_EMsgClien\
    tToGCJoinPlaytestResponse\x10\x8a@\x12\x1f\n\x1ak_EMsgLobbyPlaytestDetai\
    ls\x10\x8b@\x12\x1e\n\x19k_EMsgDOTASetFavoriteTeam\x10\x8c@\x120\n+k_EMs\
    gGCToClientBattlePassRollupListRequest\x10\x8d@\x121\n,k_EMsgGCToClientB\
    attlePassRollupListResponse\x10\x8e@\x12\x17\n\x12k_EMsgGCIsProQuery\x10\
    \x8f@\x12\x1a\n\x15k_EMsgGCIsProResponse\x10\x90@\x12\x1f\n\x1ak_EMsgDOT\
    AClaimEventAction\x10\x91@\x12'\n\"k_EMsgDOTAClaimEventActionResponse\
    \x10\x92@\x12\"\n\x1dk_EMsgDOTAGetPeriodicResource\x10\x93@\x12*\n%k_EMs\
    gDOTAGetPeriodicResourceResponse\x10\x94@\x12&\n!k_EMsgDOTAPeriodicResou\
    rceUpdated\x10\x95@\x12\x1f\n\x1ak_EMsgServerToGCSpendWager\x10\x96@\x12\
    '\n\"k_EMsgGCToGCSignoutSpendWagerToken\x10\x97@\x12%\n\x20k_EMsgSubmitT\
    riviaQuestionAnswer\x10\x98@\x12-\n(k_EMsgSubmitTriviaQuestionAnswerResp\
    onse\x10\x99@\x12\x1c\n\x17k_EMsgClientToGCGiveTip\x10\x9a@\x12$\n\x1fk_\
    EMsgClientToGCGiveTipResponse\x10\x9b@\x12\x1d\n\x18k_EMsgStartTriviaSes\
    sion\x10\x9c@\x12%\n\x20k_EMsgStartTriviaSessionResponse\x10\x9d@\x12#\n\
    \x1ek_EMsgAnchorPhoneNumberRequest\x10\x9e@\x12$\n\x1fk_EMsgAnchorPhoneN\
    umberResponse\x10\x9f@\x12%\n\x20k_EMsgUnanchorPhoneNumberRequest\x10\
    \xa0@\x12&\n!k_EMsgUnanchorPhoneNumberResponse\x10\xa1@\x12$\n\x1fk_EMsg\
    GCToClientTipNotification\x10\xa2@\x12+\n&k_EMsgClientToGCRequestSlarkGa\
    meResult\x10\xa3@\x123\n.k_EMsgClientToGCRequestSlarkGameResultResponse\
    \x10\xa4@\x12&\n!k_EMsgGCToGCSignoutSpendRankWager\x10\xa5@\x12\x20\n\
    \x1bk_EMsgGCToGCGetFavoriteTeam\x10\xa6@\x12(\n#k_EMsgGCToGCGetFavoriteT\
    eamResponse\x10\xa7@\x12\x1f\n\x1ak_EMsgSignOutEventGameData\x10\xa8@\
    \x12(\n#k_EMsgGCToClientAllStarVotesRequest\x10\xa9@\x12&\n!k_EMsgGCToCl\
    ientAllStarVotesReply\x10\xaa@\x12'\n\"k_EMsgGCToClientAllStarVotesSubmi\
    t\x10\xac@\x12,\n'k_EMsgGCToClientAllStarVotesSubmitReply\x10\xad@\x12&\
    \n!k_EMsgClientToGCQuickStatsRequest\x10\xae@\x12'\n\"k_EMsgClientToGCQu\
    ickStatsResponse\x10\xaf@\x12,\n'k_EMsgGCToGCSubtractEventPointsFromUser\
    \x10\xb0@\x12)\n$k_EMsgSelectionPriorityChoiceRequest\x10\xb1@\x12*\n%k_\
    EMsgSelectionPriorityChoiceResponse\x10\xb2@\x122\n-k_EMsgGCToGCCompendi\
    umInGamePredictionResults\x10\xb3@\x12\x1e\n\x19k_EMsgGameAutographRewar\
    d\x10\xb4@\x12&\n!k_EMsgGameAutographRewardResponse\x10\xb5@\x123\n.k_EM\
    sgServerToGCMatchPlayerItemPurchaseHistory\x10\xba@B\x05H\x01\x80\x01\0\
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
