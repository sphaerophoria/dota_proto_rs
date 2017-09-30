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
pub struct CMsgBotWorldState {
    // message fields
    team_id: ::std::option::Option<u32>,
    game_time: ::std::option::Option<f32>,
    dota_time: ::std::option::Option<f32>,
    game_state: ::std::option::Option<u32>,
    hero_pick_state: ::std::option::Option<u32>,
    time_of_day: ::std::option::Option<f32>,
    glyph_cooldown: ::std::option::Option<f32>,
    glyph_cooldown_enemy: ::std::option::Option<u32>,
    players: ::protobuf::RepeatedField<CMsgBotWorldState_Player>,
    units: ::protobuf::RepeatedField<CMsgBotWorldState_Unit>,
    dropped_items: ::protobuf::RepeatedField<CMsgBotWorldState_DroppedItem>,
    rune_infos: ::protobuf::RepeatedField<CMsgBotWorldState_RuneInfo>,
    incoming_teleports: ::protobuf::RepeatedField<CMsgBotWorldState_TeleportInfo>,
    linear_projectiles: ::protobuf::RepeatedField<CMsgBotWorldState_LinearProjectile>,
    avoidance_zones: ::protobuf::RepeatedField<CMsgBotWorldState_AvoidanceZone>,
    ability_events: ::protobuf::RepeatedField<CMsgBotWorldState_EventAbility>,
    damage_events: ::protobuf::RepeatedField<CMsgBotWorldState_EventDamage>,
    courier_killed_events: ::protobuf::RepeatedField<CMsgBotWorldState_EventCourierKilled>,
    roshan_killed_events: ::protobuf::RepeatedField<CMsgBotWorldState_EventRoshanKilled>,
    tree_events: ::protobuf::RepeatedField<CMsgBotWorldState_EventTree>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState {}

impl CMsgBotWorldState {
    pub fn new() -> CMsgBotWorldState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState,
        };
        unsafe {
            instance.get(CMsgBotWorldState::new)
        }
    }

    // optional uint32 team_id = 1;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
    }

    // optional float game_time = 2;

    pub fn clear_game_time(&mut self) {
        self.game_time = ::std::option::Option::None;
    }

    pub fn has_game_time(&self) -> bool {
        self.game_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_time(&mut self, v: f32) {
        self.game_time = ::std::option::Option::Some(v);
    }

    pub fn get_game_time(&self) -> f32 {
        self.game_time.unwrap_or(0.)
    }

    fn get_game_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.game_time
    }

    fn mut_game_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.game_time
    }

    // optional float dota_time = 3;

    pub fn clear_dota_time(&mut self) {
        self.dota_time = ::std::option::Option::None;
    }

    pub fn has_dota_time(&self) -> bool {
        self.dota_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota_time(&mut self, v: f32) {
        self.dota_time = ::std::option::Option::Some(v);
    }

    pub fn get_dota_time(&self) -> f32 {
        self.dota_time.unwrap_or(0.)
    }

    fn get_dota_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.dota_time
    }

    fn mut_dota_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.dota_time
    }

    // optional uint32 game_state = 4;

    pub fn clear_game_state(&mut self) {
        self.game_state = ::std::option::Option::None;
    }

    pub fn has_game_state(&self) -> bool {
        self.game_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_state(&mut self, v: u32) {
        self.game_state = ::std::option::Option::Some(v);
    }

    pub fn get_game_state(&self) -> u32 {
        self.game_state.unwrap_or(0)
    }

    fn get_game_state_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.game_state
    }

    fn mut_game_state_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.game_state
    }

    // optional uint32 hero_pick_state = 5;

    pub fn clear_hero_pick_state(&mut self) {
        self.hero_pick_state = ::std::option::Option::None;
    }

    pub fn has_hero_pick_state(&self) -> bool {
        self.hero_pick_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hero_pick_state(&mut self, v: u32) {
        self.hero_pick_state = ::std::option::Option::Some(v);
    }

    pub fn get_hero_pick_state(&self) -> u32 {
        self.hero_pick_state.unwrap_or(0)
    }

    fn get_hero_pick_state_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.hero_pick_state
    }

    fn mut_hero_pick_state_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.hero_pick_state
    }

    // optional float time_of_day = 6;

    pub fn clear_time_of_day(&mut self) {
        self.time_of_day = ::std::option::Option::None;
    }

    pub fn has_time_of_day(&self) -> bool {
        self.time_of_day.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_of_day(&mut self, v: f32) {
        self.time_of_day = ::std::option::Option::Some(v);
    }

    pub fn get_time_of_day(&self) -> f32 {
        self.time_of_day.unwrap_or(0.)
    }

    fn get_time_of_day_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.time_of_day
    }

    fn mut_time_of_day_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.time_of_day
    }

    // optional float glyph_cooldown = 7;

    pub fn clear_glyph_cooldown(&mut self) {
        self.glyph_cooldown = ::std::option::Option::None;
    }

    pub fn has_glyph_cooldown(&self) -> bool {
        self.glyph_cooldown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_glyph_cooldown(&mut self, v: f32) {
        self.glyph_cooldown = ::std::option::Option::Some(v);
    }

    pub fn get_glyph_cooldown(&self) -> f32 {
        self.glyph_cooldown.unwrap_or(0.)
    }

    fn get_glyph_cooldown_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.glyph_cooldown
    }

    fn mut_glyph_cooldown_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.glyph_cooldown
    }

    // optional uint32 glyph_cooldown_enemy = 8;

    pub fn clear_glyph_cooldown_enemy(&mut self) {
        self.glyph_cooldown_enemy = ::std::option::Option::None;
    }

    pub fn has_glyph_cooldown_enemy(&self) -> bool {
        self.glyph_cooldown_enemy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_glyph_cooldown_enemy(&mut self, v: u32) {
        self.glyph_cooldown_enemy = ::std::option::Option::Some(v);
    }

    pub fn get_glyph_cooldown_enemy(&self) -> u32 {
        self.glyph_cooldown_enemy.unwrap_or(0)
    }

    fn get_glyph_cooldown_enemy_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.glyph_cooldown_enemy
    }

    fn mut_glyph_cooldown_enemy_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.glyph_cooldown_enemy
    }

    // repeated .CMsgBotWorldState.Player players = 10;

    pub fn clear_players(&mut self) {
        self.players.clear();
    }

    // Param is passed by value, moved
    pub fn set_players(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_Player>) {
        self.players = v;
    }

    // Mutable pointer to the field.
    pub fn mut_players(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_Player> {
        &mut self.players
    }

    // Take field
    pub fn take_players(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_Player> {
        ::std::mem::replace(&mut self.players, ::protobuf::RepeatedField::new())
    }

    pub fn get_players(&self) -> &[CMsgBotWorldState_Player] {
        &self.players
    }

    fn get_players_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_Player> {
        &self.players
    }

    fn mut_players_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_Player> {
        &mut self.players
    }

    // repeated .CMsgBotWorldState.Unit units = 11;

    pub fn clear_units(&mut self) {
        self.units.clear();
    }

    // Param is passed by value, moved
    pub fn set_units(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_Unit>) {
        self.units = v;
    }

    // Mutable pointer to the field.
    pub fn mut_units(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_Unit> {
        &mut self.units
    }

    // Take field
    pub fn take_units(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_Unit> {
        ::std::mem::replace(&mut self.units, ::protobuf::RepeatedField::new())
    }

    pub fn get_units(&self) -> &[CMsgBotWorldState_Unit] {
        &self.units
    }

    fn get_units_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_Unit> {
        &self.units
    }

    fn mut_units_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_Unit> {
        &mut self.units
    }

    // repeated .CMsgBotWorldState.DroppedItem dropped_items = 12;

    pub fn clear_dropped_items(&mut self) {
        self.dropped_items.clear();
    }

    // Param is passed by value, moved
    pub fn set_dropped_items(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_DroppedItem>) {
        self.dropped_items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dropped_items(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_DroppedItem> {
        &mut self.dropped_items
    }

    // Take field
    pub fn take_dropped_items(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_DroppedItem> {
        ::std::mem::replace(&mut self.dropped_items, ::protobuf::RepeatedField::new())
    }

    pub fn get_dropped_items(&self) -> &[CMsgBotWorldState_DroppedItem] {
        &self.dropped_items
    }

    fn get_dropped_items_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_DroppedItem> {
        &self.dropped_items
    }

    fn mut_dropped_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_DroppedItem> {
        &mut self.dropped_items
    }

    // repeated .CMsgBotWorldState.RuneInfo rune_infos = 13;

    pub fn clear_rune_infos(&mut self) {
        self.rune_infos.clear();
    }

    // Param is passed by value, moved
    pub fn set_rune_infos(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_RuneInfo>) {
        self.rune_infos = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rune_infos(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_RuneInfo> {
        &mut self.rune_infos
    }

    // Take field
    pub fn take_rune_infos(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_RuneInfo> {
        ::std::mem::replace(&mut self.rune_infos, ::protobuf::RepeatedField::new())
    }

    pub fn get_rune_infos(&self) -> &[CMsgBotWorldState_RuneInfo] {
        &self.rune_infos
    }

    fn get_rune_infos_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_RuneInfo> {
        &self.rune_infos
    }

    fn mut_rune_infos_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_RuneInfo> {
        &mut self.rune_infos
    }

    // repeated .CMsgBotWorldState.TeleportInfo incoming_teleports = 14;

    pub fn clear_incoming_teleports(&mut self) {
        self.incoming_teleports.clear();
    }

    // Param is passed by value, moved
    pub fn set_incoming_teleports(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_TeleportInfo>) {
        self.incoming_teleports = v;
    }

    // Mutable pointer to the field.
    pub fn mut_incoming_teleports(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_TeleportInfo> {
        &mut self.incoming_teleports
    }

    // Take field
    pub fn take_incoming_teleports(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_TeleportInfo> {
        ::std::mem::replace(&mut self.incoming_teleports, ::protobuf::RepeatedField::new())
    }

    pub fn get_incoming_teleports(&self) -> &[CMsgBotWorldState_TeleportInfo] {
        &self.incoming_teleports
    }

    fn get_incoming_teleports_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_TeleportInfo> {
        &self.incoming_teleports
    }

    fn mut_incoming_teleports_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_TeleportInfo> {
        &mut self.incoming_teleports
    }

    // repeated .CMsgBotWorldState.LinearProjectile linear_projectiles = 15;

    pub fn clear_linear_projectiles(&mut self) {
        self.linear_projectiles.clear();
    }

    // Param is passed by value, moved
    pub fn set_linear_projectiles(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_LinearProjectile>) {
        self.linear_projectiles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_linear_projectiles(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_LinearProjectile> {
        &mut self.linear_projectiles
    }

    // Take field
    pub fn take_linear_projectiles(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_LinearProjectile> {
        ::std::mem::replace(&mut self.linear_projectiles, ::protobuf::RepeatedField::new())
    }

    pub fn get_linear_projectiles(&self) -> &[CMsgBotWorldState_LinearProjectile] {
        &self.linear_projectiles
    }

    fn get_linear_projectiles_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_LinearProjectile> {
        &self.linear_projectiles
    }

    fn mut_linear_projectiles_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_LinearProjectile> {
        &mut self.linear_projectiles
    }

    // repeated .CMsgBotWorldState.AvoidanceZone avoidance_zones = 16;

    pub fn clear_avoidance_zones(&mut self) {
        self.avoidance_zones.clear();
    }

    // Param is passed by value, moved
    pub fn set_avoidance_zones(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_AvoidanceZone>) {
        self.avoidance_zones = v;
    }

    // Mutable pointer to the field.
    pub fn mut_avoidance_zones(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_AvoidanceZone> {
        &mut self.avoidance_zones
    }

    // Take field
    pub fn take_avoidance_zones(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_AvoidanceZone> {
        ::std::mem::replace(&mut self.avoidance_zones, ::protobuf::RepeatedField::new())
    }

    pub fn get_avoidance_zones(&self) -> &[CMsgBotWorldState_AvoidanceZone] {
        &self.avoidance_zones
    }

    fn get_avoidance_zones_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_AvoidanceZone> {
        &self.avoidance_zones
    }

    fn mut_avoidance_zones_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_AvoidanceZone> {
        &mut self.avoidance_zones
    }

    // repeated .CMsgBotWorldState.EventAbility ability_events = 20;

    pub fn clear_ability_events(&mut self) {
        self.ability_events.clear();
    }

    // Param is passed by value, moved
    pub fn set_ability_events(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_EventAbility>) {
        self.ability_events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ability_events(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_EventAbility> {
        &mut self.ability_events
    }

    // Take field
    pub fn take_ability_events(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_EventAbility> {
        ::std::mem::replace(&mut self.ability_events, ::protobuf::RepeatedField::new())
    }

    pub fn get_ability_events(&self) -> &[CMsgBotWorldState_EventAbility] {
        &self.ability_events
    }

    fn get_ability_events_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_EventAbility> {
        &self.ability_events
    }

    fn mut_ability_events_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_EventAbility> {
        &mut self.ability_events
    }

    // repeated .CMsgBotWorldState.EventDamage damage_events = 21;

    pub fn clear_damage_events(&mut self) {
        self.damage_events.clear();
    }

    // Param is passed by value, moved
    pub fn set_damage_events(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_EventDamage>) {
        self.damage_events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_damage_events(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_EventDamage> {
        &mut self.damage_events
    }

    // Take field
    pub fn take_damage_events(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_EventDamage> {
        ::std::mem::replace(&mut self.damage_events, ::protobuf::RepeatedField::new())
    }

    pub fn get_damage_events(&self) -> &[CMsgBotWorldState_EventDamage] {
        &self.damage_events
    }

    fn get_damage_events_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_EventDamage> {
        &self.damage_events
    }

    fn mut_damage_events_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_EventDamage> {
        &mut self.damage_events
    }

    // repeated .CMsgBotWorldState.EventCourierKilled courier_killed_events = 22;

    pub fn clear_courier_killed_events(&mut self) {
        self.courier_killed_events.clear();
    }

    // Param is passed by value, moved
    pub fn set_courier_killed_events(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_EventCourierKilled>) {
        self.courier_killed_events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_courier_killed_events(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_EventCourierKilled> {
        &mut self.courier_killed_events
    }

    // Take field
    pub fn take_courier_killed_events(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_EventCourierKilled> {
        ::std::mem::replace(&mut self.courier_killed_events, ::protobuf::RepeatedField::new())
    }

    pub fn get_courier_killed_events(&self) -> &[CMsgBotWorldState_EventCourierKilled] {
        &self.courier_killed_events
    }

    fn get_courier_killed_events_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_EventCourierKilled> {
        &self.courier_killed_events
    }

    fn mut_courier_killed_events_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_EventCourierKilled> {
        &mut self.courier_killed_events
    }

    // repeated .CMsgBotWorldState.EventRoshanKilled roshan_killed_events = 23;

    pub fn clear_roshan_killed_events(&mut self) {
        self.roshan_killed_events.clear();
    }

    // Param is passed by value, moved
    pub fn set_roshan_killed_events(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_EventRoshanKilled>) {
        self.roshan_killed_events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_roshan_killed_events(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_EventRoshanKilled> {
        &mut self.roshan_killed_events
    }

    // Take field
    pub fn take_roshan_killed_events(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_EventRoshanKilled> {
        ::std::mem::replace(&mut self.roshan_killed_events, ::protobuf::RepeatedField::new())
    }

    pub fn get_roshan_killed_events(&self) -> &[CMsgBotWorldState_EventRoshanKilled] {
        &self.roshan_killed_events
    }

    fn get_roshan_killed_events_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_EventRoshanKilled> {
        &self.roshan_killed_events
    }

    fn mut_roshan_killed_events_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_EventRoshanKilled> {
        &mut self.roshan_killed_events
    }

    // repeated .CMsgBotWorldState.EventTree tree_events = 24;

    pub fn clear_tree_events(&mut self) {
        self.tree_events.clear();
    }

    // Param is passed by value, moved
    pub fn set_tree_events(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_EventTree>) {
        self.tree_events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tree_events(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_EventTree> {
        &mut self.tree_events
    }

    // Take field
    pub fn take_tree_events(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_EventTree> {
        ::std::mem::replace(&mut self.tree_events, ::protobuf::RepeatedField::new())
    }

    pub fn get_tree_events(&self) -> &[CMsgBotWorldState_EventTree] {
        &self.tree_events
    }

    fn get_tree_events_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_EventTree> {
        &self.tree_events
    }

    fn mut_tree_events_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_EventTree> {
        &mut self.tree_events
    }
}

impl ::protobuf::Message for CMsgBotWorldState {
    fn is_initialized(&self) -> bool {
        for v in &self.players {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.units {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.dropped_items {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rune_infos {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.incoming_teleports {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.linear_projectiles {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.avoidance_zones {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.ability_events {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.damage_events {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.courier_killed_events {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.roshan_killed_events {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tree_events {
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
                    let tmp = is.read_uint32()?;
                    self.team_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.game_time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.dota_time = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.game_state = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.hero_pick_state = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.time_of_day = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.glyph_cooldown = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.glyph_cooldown_enemy = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.players)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.units)?;
                },
                12 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.dropped_items)?;
                },
                13 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rune_infos)?;
                },
                14 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.incoming_teleports)?;
                },
                15 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.linear_projectiles)?;
                },
                16 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.avoidance_zones)?;
                },
                20 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ability_events)?;
                },
                21 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.damage_events)?;
                },
                22 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.courier_killed_events)?;
                },
                23 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.roshan_killed_events)?;
                },
                24 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tree_events)?;
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
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.game_time {
            my_size += 5;
        }
        if let Some(v) = self.dota_time {
            my_size += 5;
        }
        if let Some(v) = self.game_state {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.hero_pick_state {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.time_of_day {
            my_size += 5;
        }
        if let Some(v) = self.glyph_cooldown {
            my_size += 5;
        }
        if let Some(v) = self.glyph_cooldown_enemy {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.players {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.units {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.dropped_items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.rune_infos {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.incoming_teleports {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.linear_projectiles {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.avoidance_zones {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.ability_events {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.damage_events {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.courier_killed_events {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.roshan_killed_events {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.tree_events {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.game_time {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.dota_time {
            os.write_float(3, v)?;
        }
        if let Some(v) = self.game_state {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.hero_pick_state {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.time_of_day {
            os.write_float(6, v)?;
        }
        if let Some(v) = self.glyph_cooldown {
            os.write_float(7, v)?;
        }
        if let Some(v) = self.glyph_cooldown_enemy {
            os.write_uint32(8, v)?;
        }
        for v in &self.players {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.units {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.dropped_items {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.rune_infos {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.incoming_teleports {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.linear_projectiles {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.avoidance_zones {
            os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.ability_events {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.damage_events {
            os.write_tag(21, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.courier_killed_events {
            os.write_tag(22, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.roshan_killed_events {
            os.write_tag(23, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.tree_events {
            os.write_tag(24, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CMsgBotWorldState {
    fn new() -> CMsgBotWorldState {
        CMsgBotWorldState::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgBotWorldState::get_team_id_for_reflect,
                    CMsgBotWorldState::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "game_time",
                    CMsgBotWorldState::get_game_time_for_reflect,
                    CMsgBotWorldState::mut_game_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "dota_time",
                    CMsgBotWorldState::get_dota_time_for_reflect,
                    CMsgBotWorldState::mut_dota_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_state",
                    CMsgBotWorldState::get_game_state_for_reflect,
                    CMsgBotWorldState::mut_game_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hero_pick_state",
                    CMsgBotWorldState::get_hero_pick_state_for_reflect,
                    CMsgBotWorldState::mut_hero_pick_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "time_of_day",
                    CMsgBotWorldState::get_time_of_day_for_reflect,
                    CMsgBotWorldState::mut_time_of_day_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "glyph_cooldown",
                    CMsgBotWorldState::get_glyph_cooldown_for_reflect,
                    CMsgBotWorldState::mut_glyph_cooldown_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "glyph_cooldown_enemy",
                    CMsgBotWorldState::get_glyph_cooldown_enemy_for_reflect,
                    CMsgBotWorldState::mut_glyph_cooldown_enemy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_Player>>(
                    "players",
                    CMsgBotWorldState::get_players_for_reflect,
                    CMsgBotWorldState::mut_players_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_Unit>>(
                    "units",
                    CMsgBotWorldState::get_units_for_reflect,
                    CMsgBotWorldState::mut_units_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_DroppedItem>>(
                    "dropped_items",
                    CMsgBotWorldState::get_dropped_items_for_reflect,
                    CMsgBotWorldState::mut_dropped_items_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_RuneInfo>>(
                    "rune_infos",
                    CMsgBotWorldState::get_rune_infos_for_reflect,
                    CMsgBotWorldState::mut_rune_infos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_TeleportInfo>>(
                    "incoming_teleports",
                    CMsgBotWorldState::get_incoming_teleports_for_reflect,
                    CMsgBotWorldState::mut_incoming_teleports_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_LinearProjectile>>(
                    "linear_projectiles",
                    CMsgBotWorldState::get_linear_projectiles_for_reflect,
                    CMsgBotWorldState::mut_linear_projectiles_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_AvoidanceZone>>(
                    "avoidance_zones",
                    CMsgBotWorldState::get_avoidance_zones_for_reflect,
                    CMsgBotWorldState::mut_avoidance_zones_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_EventAbility>>(
                    "ability_events",
                    CMsgBotWorldState::get_ability_events_for_reflect,
                    CMsgBotWorldState::mut_ability_events_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_EventDamage>>(
                    "damage_events",
                    CMsgBotWorldState::get_damage_events_for_reflect,
                    CMsgBotWorldState::mut_damage_events_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_EventCourierKilled>>(
                    "courier_killed_events",
                    CMsgBotWorldState::get_courier_killed_events_for_reflect,
                    CMsgBotWorldState::mut_courier_killed_events_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_EventRoshanKilled>>(
                    "roshan_killed_events",
                    CMsgBotWorldState::get_roshan_killed_events_for_reflect,
                    CMsgBotWorldState::mut_roshan_killed_events_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_EventTree>>(
                    "tree_events",
                    CMsgBotWorldState::get_tree_events_for_reflect,
                    CMsgBotWorldState::mut_tree_events_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState>(
                    "CMsgBotWorldState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState {
    fn clear(&mut self) {
        self.clear_team_id();
        self.clear_game_time();
        self.clear_dota_time();
        self.clear_game_state();
        self.clear_hero_pick_state();
        self.clear_time_of_day();
        self.clear_glyph_cooldown();
        self.clear_glyph_cooldown_enemy();
        self.clear_players();
        self.clear_units();
        self.clear_dropped_items();
        self.clear_rune_infos();
        self.clear_incoming_teleports();
        self.clear_linear_projectiles();
        self.clear_avoidance_zones();
        self.clear_ability_events();
        self.clear_damage_events();
        self.clear_courier_killed_events();
        self.clear_roshan_killed_events();
        self.clear_tree_events();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_Vector {
    // message fields
    x: ::std::option::Option<i32>,
    y: ::std::option::Option<i32>,
    z: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_Vector {}

impl CMsgBotWorldState_Vector {
    pub fn new() -> CMsgBotWorldState_Vector {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_Vector {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_Vector> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_Vector,
        };
        unsafe {
            instance.get(CMsgBotWorldState_Vector::new)
        }
    }

    // required int32 x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: i32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> i32 {
        self.x.unwrap_or(0)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.x
    }

    // required int32 y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: i32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> i32 {
        self.y.unwrap_or(0)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.y
    }

    // required int32 z = 3;

    pub fn clear_z(&mut self) {
        self.z = ::std::option::Option::None;
    }

    pub fn has_z(&self) -> bool {
        self.z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_z(&mut self, v: i32) {
        self.z = ::std::option::Option::Some(v);
    }

    pub fn get_z(&self) -> i32 {
        self.z.unwrap_or(0)
    }

    fn get_z_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.z
    }

    fn mut_z_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.z
    }
}

impl ::protobuf::Message for CMsgBotWorldState_Vector {
    fn is_initialized(&self) -> bool {
        if self.x.is_none() {
            return false;
        }
        if self.y.is_none() {
            return false;
        }
        if self.z.is_none() {
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
                    let tmp = is.read_int32()?;
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.z = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.x {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.y {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.z {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.z {
            os.write_int32(3, v)?;
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_Vector {
    fn new() -> CMsgBotWorldState_Vector {
        CMsgBotWorldState_Vector::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_Vector>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "x",
                    CMsgBotWorldState_Vector::get_x_for_reflect,
                    CMsgBotWorldState_Vector::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "y",
                    CMsgBotWorldState_Vector::get_y_for_reflect,
                    CMsgBotWorldState_Vector::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "z",
                    CMsgBotWorldState_Vector::get_z_for_reflect,
                    CMsgBotWorldState_Vector::mut_z_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_Vector>(
                    "CMsgBotWorldState_Vector",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_Vector {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_z();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_Vector {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_Vector {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_Player {
    // message fields
    player_id: ::std::option::Option<u32>,
    hero_id: ::std::option::Option<u32>,
    is_alive: ::std::option::Option<bool>,
    respawn_time: ::std::option::Option<f32>,
    kills: ::std::option::Option<u32>,
    deaths: ::std::option::Option<u32>,
    assists: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_Player {}

impl CMsgBotWorldState_Player {
    pub fn new() -> CMsgBotWorldState_Player {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_Player {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_Player> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_Player,
        };
        unsafe {
            instance.get(CMsgBotWorldState_Player::new)
        }
    }

    // optional uint32 player_id = 1;

    pub fn clear_player_id(&mut self) {
        self.player_id = ::std::option::Option::None;
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: u32) {
        self.player_id = ::std::option::Option::Some(v);
    }

    pub fn get_player_id(&self) -> u32 {
        self.player_id.unwrap_or(0)
    }

    fn get_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.player_id
    }

    fn mut_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.player_id
    }

    // optional uint32 hero_id = 2;

    pub fn clear_hero_id(&mut self) {
        self.hero_id = ::std::option::Option::None;
    }

    pub fn has_hero_id(&self) -> bool {
        self.hero_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hero_id(&mut self, v: u32) {
        self.hero_id = ::std::option::Option::Some(v);
    }

    pub fn get_hero_id(&self) -> u32 {
        self.hero_id.unwrap_or(0)
    }

    fn get_hero_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.hero_id
    }

    fn mut_hero_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.hero_id
    }

    // optional bool is_alive = 3;

    pub fn clear_is_alive(&mut self) {
        self.is_alive = ::std::option::Option::None;
    }

    pub fn has_is_alive(&self) -> bool {
        self.is_alive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_alive(&mut self, v: bool) {
        self.is_alive = ::std::option::Option::Some(v);
    }

    pub fn get_is_alive(&self) -> bool {
        self.is_alive.unwrap_or(false)
    }

    fn get_is_alive_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_alive
    }

    fn mut_is_alive_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_alive
    }

    // optional float respawn_time = 4;

    pub fn clear_respawn_time(&mut self) {
        self.respawn_time = ::std::option::Option::None;
    }

    pub fn has_respawn_time(&self) -> bool {
        self.respawn_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_respawn_time(&mut self, v: f32) {
        self.respawn_time = ::std::option::Option::Some(v);
    }

    pub fn get_respawn_time(&self) -> f32 {
        self.respawn_time.unwrap_or(0.)
    }

    fn get_respawn_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.respawn_time
    }

    fn mut_respawn_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.respawn_time
    }

    // optional uint32 kills = 5;

    pub fn clear_kills(&mut self) {
        self.kills = ::std::option::Option::None;
    }

    pub fn has_kills(&self) -> bool {
        self.kills.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kills(&mut self, v: u32) {
        self.kills = ::std::option::Option::Some(v);
    }

    pub fn get_kills(&self) -> u32 {
        self.kills.unwrap_or(0)
    }

    fn get_kills_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.kills
    }

    fn mut_kills_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.kills
    }

    // optional uint32 deaths = 6;

    pub fn clear_deaths(&mut self) {
        self.deaths = ::std::option::Option::None;
    }

    pub fn has_deaths(&self) -> bool {
        self.deaths.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deaths(&mut self, v: u32) {
        self.deaths = ::std::option::Option::Some(v);
    }

    pub fn get_deaths(&self) -> u32 {
        self.deaths.unwrap_or(0)
    }

    fn get_deaths_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.deaths
    }

    fn mut_deaths_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.deaths
    }

    // optional uint32 assists = 7;

    pub fn clear_assists(&mut self) {
        self.assists = ::std::option::Option::None;
    }

    pub fn has_assists(&self) -> bool {
        self.assists.is_some()
    }

    // Param is passed by value, moved
    pub fn set_assists(&mut self, v: u32) {
        self.assists = ::std::option::Option::Some(v);
    }

    pub fn get_assists(&self) -> u32 {
        self.assists.unwrap_or(0)
    }

    fn get_assists_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.assists
    }

    fn mut_assists_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.assists
    }
}

impl ::protobuf::Message for CMsgBotWorldState_Player {
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
                    self.player_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.hero_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_alive = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.respawn_time = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.kills = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.deaths = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.assists = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.player_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.hero_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_alive {
            my_size += 2;
        }
        if let Some(v) = self.respawn_time {
            my_size += 5;
        }
        if let Some(v) = self.kills {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.deaths {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.assists {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.hero_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.is_alive {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.respawn_time {
            os.write_float(4, v)?;
        }
        if let Some(v) = self.kills {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.deaths {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.assists {
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_Player {
    fn new() -> CMsgBotWorldState_Player {
        CMsgBotWorldState_Player::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_Player>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_id",
                    CMsgBotWorldState_Player::get_player_id_for_reflect,
                    CMsgBotWorldState_Player::mut_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hero_id",
                    CMsgBotWorldState_Player::get_hero_id_for_reflect,
                    CMsgBotWorldState_Player::mut_hero_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_alive",
                    CMsgBotWorldState_Player::get_is_alive_for_reflect,
                    CMsgBotWorldState_Player::mut_is_alive_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "respawn_time",
                    CMsgBotWorldState_Player::get_respawn_time_for_reflect,
                    CMsgBotWorldState_Player::mut_respawn_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "kills",
                    CMsgBotWorldState_Player::get_kills_for_reflect,
                    CMsgBotWorldState_Player::mut_kills_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "deaths",
                    CMsgBotWorldState_Player::get_deaths_for_reflect,
                    CMsgBotWorldState_Player::mut_deaths_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "assists",
                    CMsgBotWorldState_Player::get_assists_for_reflect,
                    CMsgBotWorldState_Player::mut_assists_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_Player>(
                    "CMsgBotWorldState_Player",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_Player {
    fn clear(&mut self) {
        self.clear_player_id();
        self.clear_hero_id();
        self.clear_is_alive();
        self.clear_respawn_time();
        self.clear_kills();
        self.clear_deaths();
        self.clear_assists();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_Player {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_Player {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_Ability {
    // message fields
    handle: ::std::option::Option<u32>,
    ability_id: ::std::option::Option<u32>,
    slot: ::std::option::Option<u32>,
    caster_handle: ::std::option::Option<u32>,
    level: ::std::option::Option<u32>,
    cast_range: ::std::option::Option<u32>,
    channel_time: ::std::option::Option<f32>,
    cooldown_remaining: ::std::option::Option<f32>,
    is_activated: ::std::option::Option<bool>,
    is_toggled: ::std::option::Option<bool>,
    is_in_ability_phase: ::std::option::Option<bool>,
    is_channeling: ::std::option::Option<bool>,
    is_stolen: ::std::option::Option<bool>,
    charges: ::std::option::Option<u32>,
    secondary_charges: ::std::option::Option<u32>,
    is_combined_locked: ::std::option::Option<bool>,
    power_treads_stat: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_Ability {}

impl CMsgBotWorldState_Ability {
    pub fn new() -> CMsgBotWorldState_Ability {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_Ability {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_Ability> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_Ability,
        };
        unsafe {
            instance.get(CMsgBotWorldState_Ability::new)
        }
    }

    // optional uint32 handle = 1;

    pub fn clear_handle(&mut self) {
        self.handle = ::std::option::Option::None;
    }

    pub fn has_handle(&self) -> bool {
        self.handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_handle(&mut self, v: u32) {
        self.handle = ::std::option::Option::Some(v);
    }

    pub fn get_handle(&self) -> u32 {
        self.handle.unwrap_or(0)
    }

    fn get_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.handle
    }

    fn mut_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.handle
    }

    // optional uint32 ability_id = 2;

    pub fn clear_ability_id(&mut self) {
        self.ability_id = ::std::option::Option::None;
    }

    pub fn has_ability_id(&self) -> bool {
        self.ability_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_id(&mut self, v: u32) {
        self.ability_id = ::std::option::Option::Some(v);
    }

    pub fn get_ability_id(&self) -> u32 {
        self.ability_id.unwrap_or(0)
    }

    fn get_ability_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_id
    }

    fn mut_ability_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_id
    }

    // optional uint32 slot = 3;

    pub fn clear_slot(&mut self) {
        self.slot = ::std::option::Option::None;
    }

    pub fn has_slot(&self) -> bool {
        self.slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slot(&mut self, v: u32) {
        self.slot = ::std::option::Option::Some(v);
    }

    pub fn get_slot(&self) -> u32 {
        self.slot.unwrap_or(0)
    }

    fn get_slot_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.slot
    }

    fn mut_slot_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.slot
    }

    // optional uint32 caster_handle = 5;

    pub fn clear_caster_handle(&mut self) {
        self.caster_handle = ::std::option::Option::None;
    }

    pub fn has_caster_handle(&self) -> bool {
        self.caster_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_caster_handle(&mut self, v: u32) {
        self.caster_handle = ::std::option::Option::Some(v);
    }

    pub fn get_caster_handle(&self) -> u32 {
        self.caster_handle.unwrap_or(0)
    }

    fn get_caster_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.caster_handle
    }

    fn mut_caster_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.caster_handle
    }

    // optional uint32 level = 6;

    pub fn clear_level(&mut self) {
        self.level = ::std::option::Option::None;
    }

    pub fn has_level(&self) -> bool {
        self.level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_level(&mut self, v: u32) {
        self.level = ::std::option::Option::Some(v);
    }

    pub fn get_level(&self) -> u32 {
        self.level.unwrap_or(0)
    }

    fn get_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.level
    }

    fn mut_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.level
    }

    // optional uint32 cast_range = 10;

    pub fn clear_cast_range(&mut self) {
        self.cast_range = ::std::option::Option::None;
    }

    pub fn has_cast_range(&self) -> bool {
        self.cast_range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cast_range(&mut self, v: u32) {
        self.cast_range = ::std::option::Option::Some(v);
    }

    pub fn get_cast_range(&self) -> u32 {
        self.cast_range.unwrap_or(0)
    }

    fn get_cast_range_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.cast_range
    }

    fn mut_cast_range_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.cast_range
    }

    // optional float channel_time = 11;

    pub fn clear_channel_time(&mut self) {
        self.channel_time = ::std::option::Option::None;
    }

    pub fn has_channel_time(&self) -> bool {
        self.channel_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_time(&mut self, v: f32) {
        self.channel_time = ::std::option::Option::Some(v);
    }

    pub fn get_channel_time(&self) -> f32 {
        self.channel_time.unwrap_or(0.)
    }

    fn get_channel_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.channel_time
    }

    fn mut_channel_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.channel_time
    }

    // optional float cooldown_remaining = 12;

    pub fn clear_cooldown_remaining(&mut self) {
        self.cooldown_remaining = ::std::option::Option::None;
    }

    pub fn has_cooldown_remaining(&self) -> bool {
        self.cooldown_remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cooldown_remaining(&mut self, v: f32) {
        self.cooldown_remaining = ::std::option::Option::Some(v);
    }

    pub fn get_cooldown_remaining(&self) -> f32 {
        self.cooldown_remaining.unwrap_or(0.)
    }

    fn get_cooldown_remaining_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.cooldown_remaining
    }

    fn mut_cooldown_remaining_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.cooldown_remaining
    }

    // optional bool is_activated = 20;

    pub fn clear_is_activated(&mut self) {
        self.is_activated = ::std::option::Option::None;
    }

    pub fn has_is_activated(&self) -> bool {
        self.is_activated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_activated(&mut self, v: bool) {
        self.is_activated = ::std::option::Option::Some(v);
    }

    pub fn get_is_activated(&self) -> bool {
        self.is_activated.unwrap_or(false)
    }

    fn get_is_activated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_activated
    }

    fn mut_is_activated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_activated
    }

    // optional bool is_toggled = 21;

    pub fn clear_is_toggled(&mut self) {
        self.is_toggled = ::std::option::Option::None;
    }

    pub fn has_is_toggled(&self) -> bool {
        self.is_toggled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_toggled(&mut self, v: bool) {
        self.is_toggled = ::std::option::Option::Some(v);
    }

    pub fn get_is_toggled(&self) -> bool {
        self.is_toggled.unwrap_or(false)
    }

    fn get_is_toggled_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_toggled
    }

    fn mut_is_toggled_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_toggled
    }

    // optional bool is_in_ability_phase = 22;

    pub fn clear_is_in_ability_phase(&mut self) {
        self.is_in_ability_phase = ::std::option::Option::None;
    }

    pub fn has_is_in_ability_phase(&self) -> bool {
        self.is_in_ability_phase.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_in_ability_phase(&mut self, v: bool) {
        self.is_in_ability_phase = ::std::option::Option::Some(v);
    }

    pub fn get_is_in_ability_phase(&self) -> bool {
        self.is_in_ability_phase.unwrap_or(false)
    }

    fn get_is_in_ability_phase_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_in_ability_phase
    }

    fn mut_is_in_ability_phase_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_in_ability_phase
    }

    // optional bool is_channeling = 23;

    pub fn clear_is_channeling(&mut self) {
        self.is_channeling = ::std::option::Option::None;
    }

    pub fn has_is_channeling(&self) -> bool {
        self.is_channeling.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_channeling(&mut self, v: bool) {
        self.is_channeling = ::std::option::Option::Some(v);
    }

    pub fn get_is_channeling(&self) -> bool {
        self.is_channeling.unwrap_or(false)
    }

    fn get_is_channeling_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_channeling
    }

    fn mut_is_channeling_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_channeling
    }

    // optional bool is_stolen = 24;

    pub fn clear_is_stolen(&mut self) {
        self.is_stolen = ::std::option::Option::None;
    }

    pub fn has_is_stolen(&self) -> bool {
        self.is_stolen.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_stolen(&mut self, v: bool) {
        self.is_stolen = ::std::option::Option::Some(v);
    }

    pub fn get_is_stolen(&self) -> bool {
        self.is_stolen.unwrap_or(false)
    }

    fn get_is_stolen_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_stolen
    }

    fn mut_is_stolen_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_stolen
    }

    // optional uint32 charges = 30;

    pub fn clear_charges(&mut self) {
        self.charges = ::std::option::Option::None;
    }

    pub fn has_charges(&self) -> bool {
        self.charges.is_some()
    }

    // Param is passed by value, moved
    pub fn set_charges(&mut self, v: u32) {
        self.charges = ::std::option::Option::Some(v);
    }

    pub fn get_charges(&self) -> u32 {
        self.charges.unwrap_or(0)
    }

    fn get_charges_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.charges
    }

    fn mut_charges_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.charges
    }

    // optional uint32 secondary_charges = 31;

    pub fn clear_secondary_charges(&mut self) {
        self.secondary_charges = ::std::option::Option::None;
    }

    pub fn has_secondary_charges(&self) -> bool {
        self.secondary_charges.is_some()
    }

    // Param is passed by value, moved
    pub fn set_secondary_charges(&mut self, v: u32) {
        self.secondary_charges = ::std::option::Option::Some(v);
    }

    pub fn get_secondary_charges(&self) -> u32 {
        self.secondary_charges.unwrap_or(0)
    }

    fn get_secondary_charges_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.secondary_charges
    }

    fn mut_secondary_charges_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.secondary_charges
    }

    // optional bool is_combined_locked = 40;

    pub fn clear_is_combined_locked(&mut self) {
        self.is_combined_locked = ::std::option::Option::None;
    }

    pub fn has_is_combined_locked(&self) -> bool {
        self.is_combined_locked.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_combined_locked(&mut self, v: bool) {
        self.is_combined_locked = ::std::option::Option::Some(v);
    }

    pub fn get_is_combined_locked(&self) -> bool {
        self.is_combined_locked.unwrap_or(false)
    }

    fn get_is_combined_locked_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_combined_locked
    }

    fn mut_is_combined_locked_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_combined_locked
    }

    // optional uint32 power_treads_stat = 50;

    pub fn clear_power_treads_stat(&mut self) {
        self.power_treads_stat = ::std::option::Option::None;
    }

    pub fn has_power_treads_stat(&self) -> bool {
        self.power_treads_stat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_power_treads_stat(&mut self, v: u32) {
        self.power_treads_stat = ::std::option::Option::Some(v);
    }

    pub fn get_power_treads_stat(&self) -> u32 {
        self.power_treads_stat.unwrap_or(0)
    }

    fn get_power_treads_stat_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.power_treads_stat
    }

    fn mut_power_treads_stat_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.power_treads_stat
    }
}

impl ::protobuf::Message for CMsgBotWorldState_Ability {
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
                    self.handle = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ability_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.slot = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.caster_handle = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.level = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.cast_range = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.channel_time = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.cooldown_remaining = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_activated = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_toggled = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_in_ability_phase = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_channeling = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_stolen = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.charges = ::std::option::Option::Some(tmp);
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.secondary_charges = ::std::option::Option::Some(tmp);
                },
                40 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_combined_locked = ::std::option::Option::Some(tmp);
                },
                50 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.power_treads_stat = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.handle {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ability_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.slot {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.caster_handle {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.level {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.cast_range {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.channel_time {
            my_size += 5;
        }
        if let Some(v) = self.cooldown_remaining {
            my_size += 5;
        }
        if let Some(v) = self.is_activated {
            my_size += 3;
        }
        if let Some(v) = self.is_toggled {
            my_size += 3;
        }
        if let Some(v) = self.is_in_ability_phase {
            my_size += 3;
        }
        if let Some(v) = self.is_channeling {
            my_size += 3;
        }
        if let Some(v) = self.is_stolen {
            my_size += 3;
        }
        if let Some(v) = self.charges {
            my_size += ::protobuf::rt::value_size(30, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.secondary_charges {
            my_size += ::protobuf::rt::value_size(31, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_combined_locked {
            my_size += 3;
        }
        if let Some(v) = self.power_treads_stat {
            my_size += ::protobuf::rt::value_size(50, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.handle {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.ability_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.slot {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.caster_handle {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.level {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.cast_range {
            os.write_uint32(10, v)?;
        }
        if let Some(v) = self.channel_time {
            os.write_float(11, v)?;
        }
        if let Some(v) = self.cooldown_remaining {
            os.write_float(12, v)?;
        }
        if let Some(v) = self.is_activated {
            os.write_bool(20, v)?;
        }
        if let Some(v) = self.is_toggled {
            os.write_bool(21, v)?;
        }
        if let Some(v) = self.is_in_ability_phase {
            os.write_bool(22, v)?;
        }
        if let Some(v) = self.is_channeling {
            os.write_bool(23, v)?;
        }
        if let Some(v) = self.is_stolen {
            os.write_bool(24, v)?;
        }
        if let Some(v) = self.charges {
            os.write_uint32(30, v)?;
        }
        if let Some(v) = self.secondary_charges {
            os.write_uint32(31, v)?;
        }
        if let Some(v) = self.is_combined_locked {
            os.write_bool(40, v)?;
        }
        if let Some(v) = self.power_treads_stat {
            os.write_uint32(50, v)?;
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_Ability {
    fn new() -> CMsgBotWorldState_Ability {
        CMsgBotWorldState_Ability::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_Ability>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "handle",
                    CMsgBotWorldState_Ability::get_handle_for_reflect,
                    CMsgBotWorldState_Ability::mut_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_id",
                    CMsgBotWorldState_Ability::get_ability_id_for_reflect,
                    CMsgBotWorldState_Ability::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "slot",
                    CMsgBotWorldState_Ability::get_slot_for_reflect,
                    CMsgBotWorldState_Ability::mut_slot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "caster_handle",
                    CMsgBotWorldState_Ability::get_caster_handle_for_reflect,
                    CMsgBotWorldState_Ability::mut_caster_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "level",
                    CMsgBotWorldState_Ability::get_level_for_reflect,
                    CMsgBotWorldState_Ability::mut_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cast_range",
                    CMsgBotWorldState_Ability::get_cast_range_for_reflect,
                    CMsgBotWorldState_Ability::mut_cast_range_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "channel_time",
                    CMsgBotWorldState_Ability::get_channel_time_for_reflect,
                    CMsgBotWorldState_Ability::mut_channel_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "cooldown_remaining",
                    CMsgBotWorldState_Ability::get_cooldown_remaining_for_reflect,
                    CMsgBotWorldState_Ability::mut_cooldown_remaining_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_activated",
                    CMsgBotWorldState_Ability::get_is_activated_for_reflect,
                    CMsgBotWorldState_Ability::mut_is_activated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_toggled",
                    CMsgBotWorldState_Ability::get_is_toggled_for_reflect,
                    CMsgBotWorldState_Ability::mut_is_toggled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_in_ability_phase",
                    CMsgBotWorldState_Ability::get_is_in_ability_phase_for_reflect,
                    CMsgBotWorldState_Ability::mut_is_in_ability_phase_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_channeling",
                    CMsgBotWorldState_Ability::get_is_channeling_for_reflect,
                    CMsgBotWorldState_Ability::mut_is_channeling_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_stolen",
                    CMsgBotWorldState_Ability::get_is_stolen_for_reflect,
                    CMsgBotWorldState_Ability::mut_is_stolen_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "charges",
                    CMsgBotWorldState_Ability::get_charges_for_reflect,
                    CMsgBotWorldState_Ability::mut_charges_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "secondary_charges",
                    CMsgBotWorldState_Ability::get_secondary_charges_for_reflect,
                    CMsgBotWorldState_Ability::mut_secondary_charges_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_combined_locked",
                    CMsgBotWorldState_Ability::get_is_combined_locked_for_reflect,
                    CMsgBotWorldState_Ability::mut_is_combined_locked_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "power_treads_stat",
                    CMsgBotWorldState_Ability::get_power_treads_stat_for_reflect,
                    CMsgBotWorldState_Ability::mut_power_treads_stat_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_Ability>(
                    "CMsgBotWorldState_Ability",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_Ability {
    fn clear(&mut self) {
        self.clear_handle();
        self.clear_ability_id();
        self.clear_slot();
        self.clear_caster_handle();
        self.clear_level();
        self.clear_cast_range();
        self.clear_channel_time();
        self.clear_cooldown_remaining();
        self.clear_is_activated();
        self.clear_is_toggled();
        self.clear_is_in_ability_phase();
        self.clear_is_channeling();
        self.clear_is_stolen();
        self.clear_charges();
        self.clear_secondary_charges();
        self.clear_is_combined_locked();
        self.clear_power_treads_stat();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_Ability {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_Ability {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_DroppedItem {
    // message fields
    item_id: ::std::option::Option<u32>,
    location: ::protobuf::SingularPtrField<CMsgBotWorldState_Vector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_DroppedItem {}

impl CMsgBotWorldState_DroppedItem {
    pub fn new() -> CMsgBotWorldState_DroppedItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_DroppedItem {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_DroppedItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_DroppedItem,
        };
        unsafe {
            instance.get(CMsgBotWorldState_DroppedItem::new)
        }
    }

    // optional uint32 item_id = 1;

    pub fn clear_item_id(&mut self) {
        self.item_id = ::std::option::Option::None;
    }

    pub fn has_item_id(&self) -> bool {
        self.item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_id(&mut self, v: u32) {
        self.item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_id(&self) -> u32 {
        self.item_id.unwrap_or(0)
    }

    fn get_item_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_id
    }

    fn mut_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_id
    }

    // optional .CMsgBotWorldState.Vector location = 2;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: CMsgBotWorldState_Vector) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut CMsgBotWorldState_Vector {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> CMsgBotWorldState_Vector {
        self.location.take().unwrap_or_else(|| CMsgBotWorldState_Vector::new())
    }

    pub fn get_location(&self) -> &CMsgBotWorldState_Vector {
        self.location.as_ref().unwrap_or_else(|| CMsgBotWorldState_Vector::default_instance())
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &mut self.location
    }
}

impl ::protobuf::Message for CMsgBotWorldState_DroppedItem {
    fn is_initialized(&self) -> bool {
        for v in &self.location {
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
                    let tmp = is.read_uint32()?;
                    self.item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location)?;
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
        if let Some(v) = self.item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.location.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_DroppedItem {
    fn new() -> CMsgBotWorldState_DroppedItem {
        CMsgBotWorldState_DroppedItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_DroppedItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_id",
                    CMsgBotWorldState_DroppedItem::get_item_id_for_reflect,
                    CMsgBotWorldState_DroppedItem::mut_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_Vector>>(
                    "location",
                    CMsgBotWorldState_DroppedItem::get_location_for_reflect,
                    CMsgBotWorldState_DroppedItem::mut_location_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_DroppedItem>(
                    "CMsgBotWorldState_DroppedItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_DroppedItem {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_location();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_DroppedItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_DroppedItem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_RuneInfo {
    // message fields
    field_type: ::std::option::Option<i32>,
    location: ::protobuf::SingularPtrField<CMsgBotWorldState_Vector>,
    status: ::std::option::Option<u32>,
    time_since_seen: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_RuneInfo {}

impl CMsgBotWorldState_RuneInfo {
    pub fn new() -> CMsgBotWorldState_RuneInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_RuneInfo {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_RuneInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_RuneInfo,
        };
        unsafe {
            instance.get(CMsgBotWorldState_RuneInfo::new)
        }
    }

    // optional int32 type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: i32) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> i32 {
        self.field_type.unwrap_or(0)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.field_type
    }

    // optional .CMsgBotWorldState.Vector location = 2;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: CMsgBotWorldState_Vector) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut CMsgBotWorldState_Vector {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> CMsgBotWorldState_Vector {
        self.location.take().unwrap_or_else(|| CMsgBotWorldState_Vector::new())
    }

    pub fn get_location(&self) -> &CMsgBotWorldState_Vector {
        self.location.as_ref().unwrap_or_else(|| CMsgBotWorldState_Vector::default_instance())
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &mut self.location
    }

    // optional uint32 status = 3;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: u32) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> u32 {
        self.status.unwrap_or(0)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.status
    }

    // optional float time_since_seen = 4;

    pub fn clear_time_since_seen(&mut self) {
        self.time_since_seen = ::std::option::Option::None;
    }

    pub fn has_time_since_seen(&self) -> bool {
        self.time_since_seen.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_since_seen(&mut self, v: f32) {
        self.time_since_seen = ::std::option::Option::Some(v);
    }

    pub fn get_time_since_seen(&self) -> f32 {
        self.time_since_seen.unwrap_or(0.)
    }

    fn get_time_since_seen_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.time_since_seen
    }

    fn mut_time_since_seen_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.time_since_seen
    }
}

impl ::protobuf::Message for CMsgBotWorldState_RuneInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.location {
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
                    let tmp = is.read_int32()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.time_since_seen = ::std::option::Option::Some(tmp);
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
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.time_since_seen {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.location.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.status {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.time_since_seen {
            os.write_float(4, v)?;
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_RuneInfo {
    fn new() -> CMsgBotWorldState_RuneInfo {
        CMsgBotWorldState_RuneInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_RuneInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "type",
                    CMsgBotWorldState_RuneInfo::get_field_type_for_reflect,
                    CMsgBotWorldState_RuneInfo::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_Vector>>(
                    "location",
                    CMsgBotWorldState_RuneInfo::get_location_for_reflect,
                    CMsgBotWorldState_RuneInfo::mut_location_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "status",
                    CMsgBotWorldState_RuneInfo::get_status_for_reflect,
                    CMsgBotWorldState_RuneInfo::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "time_since_seen",
                    CMsgBotWorldState_RuneInfo::get_time_since_seen_for_reflect,
                    CMsgBotWorldState_RuneInfo::mut_time_since_seen_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_RuneInfo>(
                    "CMsgBotWorldState_RuneInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_RuneInfo {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_location();
        self.clear_status();
        self.clear_time_since_seen();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_RuneInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_RuneInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_TeleportInfo {
    // message fields
    player_id: ::std::option::Option<u32>,
    location: ::protobuf::SingularPtrField<CMsgBotWorldState_Vector>,
    time_remaning: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_TeleportInfo {}

impl CMsgBotWorldState_TeleportInfo {
    pub fn new() -> CMsgBotWorldState_TeleportInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_TeleportInfo {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_TeleportInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_TeleportInfo,
        };
        unsafe {
            instance.get(CMsgBotWorldState_TeleportInfo::new)
        }
    }

    // optional uint32 player_id = 1;

    pub fn clear_player_id(&mut self) {
        self.player_id = ::std::option::Option::None;
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: u32) {
        self.player_id = ::std::option::Option::Some(v);
    }

    pub fn get_player_id(&self) -> u32 {
        self.player_id.unwrap_or(0)
    }

    fn get_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.player_id
    }

    fn mut_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.player_id
    }

    // optional .CMsgBotWorldState.Vector location = 2;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: CMsgBotWorldState_Vector) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut CMsgBotWorldState_Vector {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> CMsgBotWorldState_Vector {
        self.location.take().unwrap_or_else(|| CMsgBotWorldState_Vector::new())
    }

    pub fn get_location(&self) -> &CMsgBotWorldState_Vector {
        self.location.as_ref().unwrap_or_else(|| CMsgBotWorldState_Vector::default_instance())
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &mut self.location
    }

    // optional float time_remaning = 3;

    pub fn clear_time_remaning(&mut self) {
        self.time_remaning = ::std::option::Option::None;
    }

    pub fn has_time_remaning(&self) -> bool {
        self.time_remaning.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_remaning(&mut self, v: f32) {
        self.time_remaning = ::std::option::Option::Some(v);
    }

    pub fn get_time_remaning(&self) -> f32 {
        self.time_remaning.unwrap_or(0.)
    }

    fn get_time_remaning_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.time_remaning
    }

    fn mut_time_remaning_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.time_remaning
    }
}

impl ::protobuf::Message for CMsgBotWorldState_TeleportInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.location {
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
                    let tmp = is.read_uint32()?;
                    self.player_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.time_remaning = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.player_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.time_remaning {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.location.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.time_remaning {
            os.write_float(3, v)?;
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_TeleportInfo {
    fn new() -> CMsgBotWorldState_TeleportInfo {
        CMsgBotWorldState_TeleportInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_TeleportInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_id",
                    CMsgBotWorldState_TeleportInfo::get_player_id_for_reflect,
                    CMsgBotWorldState_TeleportInfo::mut_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_Vector>>(
                    "location",
                    CMsgBotWorldState_TeleportInfo::get_location_for_reflect,
                    CMsgBotWorldState_TeleportInfo::mut_location_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "time_remaning",
                    CMsgBotWorldState_TeleportInfo::get_time_remaning_for_reflect,
                    CMsgBotWorldState_TeleportInfo::mut_time_remaning_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_TeleportInfo>(
                    "CMsgBotWorldState_TeleportInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_TeleportInfo {
    fn clear(&mut self) {
        self.clear_player_id();
        self.clear_location();
        self.clear_time_remaning();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_TeleportInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_TeleportInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_Modifier {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    stack_count: ::std::option::Option<u32>,
    ability_handle: ::std::option::Option<u32>,
    ability_id: ::std::option::Option<u32>,
    remaining_duration: ::std::option::Option<f32>,
    auxiliary_units_handles: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_Modifier {}

impl CMsgBotWorldState_Modifier {
    pub fn new() -> CMsgBotWorldState_Modifier {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_Modifier {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_Modifier> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_Modifier,
        };
        unsafe {
            instance.get(CMsgBotWorldState_Modifier::new)
        }
    }

    // optional string name = 1;

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

    // optional uint32 stack_count = 2;

    pub fn clear_stack_count(&mut self) {
        self.stack_count = ::std::option::Option::None;
    }

    pub fn has_stack_count(&self) -> bool {
        self.stack_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stack_count(&mut self, v: u32) {
        self.stack_count = ::std::option::Option::Some(v);
    }

    pub fn get_stack_count(&self) -> u32 {
        self.stack_count.unwrap_or(0)
    }

    fn get_stack_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.stack_count
    }

    fn mut_stack_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.stack_count
    }

    // optional uint32 ability_handle = 3;

    pub fn clear_ability_handle(&mut self) {
        self.ability_handle = ::std::option::Option::None;
    }

    pub fn has_ability_handle(&self) -> bool {
        self.ability_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_handle(&mut self, v: u32) {
        self.ability_handle = ::std::option::Option::Some(v);
    }

    pub fn get_ability_handle(&self) -> u32 {
        self.ability_handle.unwrap_or(0)
    }

    fn get_ability_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_handle
    }

    fn mut_ability_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_handle
    }

    // optional uint32 ability_id = 4;

    pub fn clear_ability_id(&mut self) {
        self.ability_id = ::std::option::Option::None;
    }

    pub fn has_ability_id(&self) -> bool {
        self.ability_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_id(&mut self, v: u32) {
        self.ability_id = ::std::option::Option::Some(v);
    }

    pub fn get_ability_id(&self) -> u32 {
        self.ability_id.unwrap_or(0)
    }

    fn get_ability_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_id
    }

    fn mut_ability_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_id
    }

    // optional float remaining_duration = 5;

    pub fn clear_remaining_duration(&mut self) {
        self.remaining_duration = ::std::option::Option::None;
    }

    pub fn has_remaining_duration(&self) -> bool {
        self.remaining_duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remaining_duration(&mut self, v: f32) {
        self.remaining_duration = ::std::option::Option::Some(v);
    }

    pub fn get_remaining_duration(&self) -> f32 {
        self.remaining_duration.unwrap_or(0.)
    }

    fn get_remaining_duration_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.remaining_duration
    }

    fn mut_remaining_duration_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.remaining_duration
    }

    // repeated uint32 auxiliary_units_handles = 6;

    pub fn clear_auxiliary_units_handles(&mut self) {
        self.auxiliary_units_handles.clear();
    }

    // Param is passed by value, moved
    pub fn set_auxiliary_units_handles(&mut self, v: ::std::vec::Vec<u32>) {
        self.auxiliary_units_handles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_auxiliary_units_handles(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.auxiliary_units_handles
    }

    // Take field
    pub fn take_auxiliary_units_handles(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.auxiliary_units_handles, ::std::vec::Vec::new())
    }

    pub fn get_auxiliary_units_handles(&self) -> &[u32] {
        &self.auxiliary_units_handles
    }

    fn get_auxiliary_units_handles_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.auxiliary_units_handles
    }

    fn mut_auxiliary_units_handles_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.auxiliary_units_handles
    }
}

impl ::protobuf::Message for CMsgBotWorldState_Modifier {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.stack_count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ability_handle = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ability_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.remaining_duration = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.auxiliary_units_handles)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.stack_count {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ability_handle {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ability_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.remaining_duration {
            my_size += 5;
        }
        for value in &self.auxiliary_units_handles {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.stack_count {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.ability_handle {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.ability_id {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.remaining_duration {
            os.write_float(5, v)?;
        }
        for v in &self.auxiliary_units_handles {
            os.write_uint32(6, *v)?;
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_Modifier {
    fn new() -> CMsgBotWorldState_Modifier {
        CMsgBotWorldState_Modifier::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_Modifier>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgBotWorldState_Modifier::get_name_for_reflect,
                    CMsgBotWorldState_Modifier::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "stack_count",
                    CMsgBotWorldState_Modifier::get_stack_count_for_reflect,
                    CMsgBotWorldState_Modifier::mut_stack_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_handle",
                    CMsgBotWorldState_Modifier::get_ability_handle_for_reflect,
                    CMsgBotWorldState_Modifier::mut_ability_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_id",
                    CMsgBotWorldState_Modifier::get_ability_id_for_reflect,
                    CMsgBotWorldState_Modifier::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "remaining_duration",
                    CMsgBotWorldState_Modifier::get_remaining_duration_for_reflect,
                    CMsgBotWorldState_Modifier::mut_remaining_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "auxiliary_units_handles",
                    CMsgBotWorldState_Modifier::get_auxiliary_units_handles_for_reflect,
                    CMsgBotWorldState_Modifier::mut_auxiliary_units_handles_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_Modifier>(
                    "CMsgBotWorldState_Modifier",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_Modifier {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_stack_count();
        self.clear_ability_handle();
        self.clear_ability_id();
        self.clear_remaining_duration();
        self.clear_auxiliary_units_handles();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_Modifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_Modifier {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_LinearProjectile {
    // message fields
    handle: ::std::option::Option<u32>,
    caster_handle: ::std::option::Option<u32>,
    caster_player_id: ::std::option::Option<i32>,
    ability_handle: ::std::option::Option<u32>,
    ability_id: ::std::option::Option<u32>,
    location: ::protobuf::SingularPtrField<CMsgBotWorldState_Vector>,
    velocity: ::protobuf::SingularPtrField<CMsgBotWorldState_Vector>,
    radius: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_LinearProjectile {}

impl CMsgBotWorldState_LinearProjectile {
    pub fn new() -> CMsgBotWorldState_LinearProjectile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_LinearProjectile {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_LinearProjectile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_LinearProjectile,
        };
        unsafe {
            instance.get(CMsgBotWorldState_LinearProjectile::new)
        }
    }

    // optional uint32 handle = 1;

    pub fn clear_handle(&mut self) {
        self.handle = ::std::option::Option::None;
    }

    pub fn has_handle(&self) -> bool {
        self.handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_handle(&mut self, v: u32) {
        self.handle = ::std::option::Option::Some(v);
    }

    pub fn get_handle(&self) -> u32 {
        self.handle.unwrap_or(0)
    }

    fn get_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.handle
    }

    fn mut_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.handle
    }

    // optional uint32 caster_handle = 2;

    pub fn clear_caster_handle(&mut self) {
        self.caster_handle = ::std::option::Option::None;
    }

    pub fn has_caster_handle(&self) -> bool {
        self.caster_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_caster_handle(&mut self, v: u32) {
        self.caster_handle = ::std::option::Option::Some(v);
    }

    pub fn get_caster_handle(&self) -> u32 {
        self.caster_handle.unwrap_or(0)
    }

    fn get_caster_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.caster_handle
    }

    fn mut_caster_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.caster_handle
    }

    // optional int32 caster_player_id = 3;

    pub fn clear_caster_player_id(&mut self) {
        self.caster_player_id = ::std::option::Option::None;
    }

    pub fn has_caster_player_id(&self) -> bool {
        self.caster_player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_caster_player_id(&mut self, v: i32) {
        self.caster_player_id = ::std::option::Option::Some(v);
    }

    pub fn get_caster_player_id(&self) -> i32 {
        self.caster_player_id.unwrap_or(0)
    }

    fn get_caster_player_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.caster_player_id
    }

    fn mut_caster_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.caster_player_id
    }

    // optional uint32 ability_handle = 4;

    pub fn clear_ability_handle(&mut self) {
        self.ability_handle = ::std::option::Option::None;
    }

    pub fn has_ability_handle(&self) -> bool {
        self.ability_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_handle(&mut self, v: u32) {
        self.ability_handle = ::std::option::Option::Some(v);
    }

    pub fn get_ability_handle(&self) -> u32 {
        self.ability_handle.unwrap_or(0)
    }

    fn get_ability_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_handle
    }

    fn mut_ability_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_handle
    }

    // optional uint32 ability_id = 5;

    pub fn clear_ability_id(&mut self) {
        self.ability_id = ::std::option::Option::None;
    }

    pub fn has_ability_id(&self) -> bool {
        self.ability_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_id(&mut self, v: u32) {
        self.ability_id = ::std::option::Option::Some(v);
    }

    pub fn get_ability_id(&self) -> u32 {
        self.ability_id.unwrap_or(0)
    }

    fn get_ability_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_id
    }

    fn mut_ability_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_id
    }

    // optional .CMsgBotWorldState.Vector location = 6;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: CMsgBotWorldState_Vector) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut CMsgBotWorldState_Vector {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> CMsgBotWorldState_Vector {
        self.location.take().unwrap_or_else(|| CMsgBotWorldState_Vector::new())
    }

    pub fn get_location(&self) -> &CMsgBotWorldState_Vector {
        self.location.as_ref().unwrap_or_else(|| CMsgBotWorldState_Vector::default_instance())
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &mut self.location
    }

    // optional .CMsgBotWorldState.Vector velocity = 7;

    pub fn clear_velocity(&mut self) {
        self.velocity.clear();
    }

    pub fn has_velocity(&self) -> bool {
        self.velocity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_velocity(&mut self, v: CMsgBotWorldState_Vector) {
        self.velocity = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_velocity(&mut self) -> &mut CMsgBotWorldState_Vector {
        if self.velocity.is_none() {
            self.velocity.set_default();
        }
        self.velocity.as_mut().unwrap()
    }

    // Take field
    pub fn take_velocity(&mut self) -> CMsgBotWorldState_Vector {
        self.velocity.take().unwrap_or_else(|| CMsgBotWorldState_Vector::new())
    }

    pub fn get_velocity(&self) -> &CMsgBotWorldState_Vector {
        self.velocity.as_ref().unwrap_or_else(|| CMsgBotWorldState_Vector::default_instance())
    }

    fn get_velocity_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &self.velocity
    }

    fn mut_velocity_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &mut self.velocity
    }

    // optional uint32 radius = 8;

    pub fn clear_radius(&mut self) {
        self.radius = ::std::option::Option::None;
    }

    pub fn has_radius(&self) -> bool {
        self.radius.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radius(&mut self, v: u32) {
        self.radius = ::std::option::Option::Some(v);
    }

    pub fn get_radius(&self) -> u32 {
        self.radius.unwrap_or(0)
    }

    fn get_radius_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.radius
    }

    fn mut_radius_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.radius
    }
}

impl ::protobuf::Message for CMsgBotWorldState_LinearProjectile {
    fn is_initialized(&self) -> bool {
        for v in &self.location {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.velocity {
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
                    let tmp = is.read_uint32()?;
                    self.handle = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.caster_handle = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.caster_player_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ability_handle = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ability_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.velocity)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.radius = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.handle {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.caster_handle {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.caster_player_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ability_handle {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ability_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.velocity.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.radius {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.handle {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.caster_handle {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.caster_player_id {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.ability_handle {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.ability_id {
            os.write_uint32(5, v)?;
        }
        if let Some(ref v) = self.location.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.velocity.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.radius {
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_LinearProjectile {
    fn new() -> CMsgBotWorldState_LinearProjectile {
        CMsgBotWorldState_LinearProjectile::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_LinearProjectile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "handle",
                    CMsgBotWorldState_LinearProjectile::get_handle_for_reflect,
                    CMsgBotWorldState_LinearProjectile::mut_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "caster_handle",
                    CMsgBotWorldState_LinearProjectile::get_caster_handle_for_reflect,
                    CMsgBotWorldState_LinearProjectile::mut_caster_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "caster_player_id",
                    CMsgBotWorldState_LinearProjectile::get_caster_player_id_for_reflect,
                    CMsgBotWorldState_LinearProjectile::mut_caster_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_handle",
                    CMsgBotWorldState_LinearProjectile::get_ability_handle_for_reflect,
                    CMsgBotWorldState_LinearProjectile::mut_ability_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_id",
                    CMsgBotWorldState_LinearProjectile::get_ability_id_for_reflect,
                    CMsgBotWorldState_LinearProjectile::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_Vector>>(
                    "location",
                    CMsgBotWorldState_LinearProjectile::get_location_for_reflect,
                    CMsgBotWorldState_LinearProjectile::mut_location_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_Vector>>(
                    "velocity",
                    CMsgBotWorldState_LinearProjectile::get_velocity_for_reflect,
                    CMsgBotWorldState_LinearProjectile::mut_velocity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "radius",
                    CMsgBotWorldState_LinearProjectile::get_radius_for_reflect,
                    CMsgBotWorldState_LinearProjectile::mut_radius_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_LinearProjectile>(
                    "CMsgBotWorldState_LinearProjectile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_LinearProjectile {
    fn clear(&mut self) {
        self.clear_handle();
        self.clear_caster_handle();
        self.clear_caster_player_id();
        self.clear_ability_handle();
        self.clear_ability_id();
        self.clear_location();
        self.clear_velocity();
        self.clear_radius();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_LinearProjectile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_LinearProjectile {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_TrackingProjectile {
    // message fields
    caster_handle: ::std::option::Option<u32>,
    caster_player_id: ::std::option::Option<i32>,
    ability_handle: ::std::option::Option<u32>,
    ability_id: ::std::option::Option<u32>,
    location: ::protobuf::SingularPtrField<CMsgBotWorldState_Vector>,
    velocity: ::std::option::Option<u32>,
    is_dodgeable: ::std::option::Option<bool>,
    is_attack: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_TrackingProjectile {}

impl CMsgBotWorldState_TrackingProjectile {
    pub fn new() -> CMsgBotWorldState_TrackingProjectile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_TrackingProjectile {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_TrackingProjectile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_TrackingProjectile,
        };
        unsafe {
            instance.get(CMsgBotWorldState_TrackingProjectile::new)
        }
    }

    // optional uint32 caster_handle = 1;

    pub fn clear_caster_handle(&mut self) {
        self.caster_handle = ::std::option::Option::None;
    }

    pub fn has_caster_handle(&self) -> bool {
        self.caster_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_caster_handle(&mut self, v: u32) {
        self.caster_handle = ::std::option::Option::Some(v);
    }

    pub fn get_caster_handle(&self) -> u32 {
        self.caster_handle.unwrap_or(0)
    }

    fn get_caster_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.caster_handle
    }

    fn mut_caster_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.caster_handle
    }

    // optional int32 caster_player_id = 2;

    pub fn clear_caster_player_id(&mut self) {
        self.caster_player_id = ::std::option::Option::None;
    }

    pub fn has_caster_player_id(&self) -> bool {
        self.caster_player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_caster_player_id(&mut self, v: i32) {
        self.caster_player_id = ::std::option::Option::Some(v);
    }

    pub fn get_caster_player_id(&self) -> i32 {
        self.caster_player_id.unwrap_or(0)
    }

    fn get_caster_player_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.caster_player_id
    }

    fn mut_caster_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.caster_player_id
    }

    // optional uint32 ability_handle = 3;

    pub fn clear_ability_handle(&mut self) {
        self.ability_handle = ::std::option::Option::None;
    }

    pub fn has_ability_handle(&self) -> bool {
        self.ability_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_handle(&mut self, v: u32) {
        self.ability_handle = ::std::option::Option::Some(v);
    }

    pub fn get_ability_handle(&self) -> u32 {
        self.ability_handle.unwrap_or(0)
    }

    fn get_ability_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_handle
    }

    fn mut_ability_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_handle
    }

    // optional uint32 ability_id = 4;

    pub fn clear_ability_id(&mut self) {
        self.ability_id = ::std::option::Option::None;
    }

    pub fn has_ability_id(&self) -> bool {
        self.ability_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_id(&mut self, v: u32) {
        self.ability_id = ::std::option::Option::Some(v);
    }

    pub fn get_ability_id(&self) -> u32 {
        self.ability_id.unwrap_or(0)
    }

    fn get_ability_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_id
    }

    fn mut_ability_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_id
    }

    // optional .CMsgBotWorldState.Vector location = 5;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: CMsgBotWorldState_Vector) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut CMsgBotWorldState_Vector {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> CMsgBotWorldState_Vector {
        self.location.take().unwrap_or_else(|| CMsgBotWorldState_Vector::new())
    }

    pub fn get_location(&self) -> &CMsgBotWorldState_Vector {
        self.location.as_ref().unwrap_or_else(|| CMsgBotWorldState_Vector::default_instance())
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &mut self.location
    }

    // optional uint32 velocity = 6;

    pub fn clear_velocity(&mut self) {
        self.velocity = ::std::option::Option::None;
    }

    pub fn has_velocity(&self) -> bool {
        self.velocity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_velocity(&mut self, v: u32) {
        self.velocity = ::std::option::Option::Some(v);
    }

    pub fn get_velocity(&self) -> u32 {
        self.velocity.unwrap_or(0)
    }

    fn get_velocity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.velocity
    }

    fn mut_velocity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.velocity
    }

    // optional bool is_dodgeable = 7;

    pub fn clear_is_dodgeable(&mut self) {
        self.is_dodgeable = ::std::option::Option::None;
    }

    pub fn has_is_dodgeable(&self) -> bool {
        self.is_dodgeable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_dodgeable(&mut self, v: bool) {
        self.is_dodgeable = ::std::option::Option::Some(v);
    }

    pub fn get_is_dodgeable(&self) -> bool {
        self.is_dodgeable.unwrap_or(false)
    }

    fn get_is_dodgeable_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_dodgeable
    }

    fn mut_is_dodgeable_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_dodgeable
    }

    // optional bool is_attack = 8;

    pub fn clear_is_attack(&mut self) {
        self.is_attack = ::std::option::Option::None;
    }

    pub fn has_is_attack(&self) -> bool {
        self.is_attack.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_attack(&mut self, v: bool) {
        self.is_attack = ::std::option::Option::Some(v);
    }

    pub fn get_is_attack(&self) -> bool {
        self.is_attack.unwrap_or(false)
    }

    fn get_is_attack_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_attack
    }

    fn mut_is_attack_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_attack
    }
}

impl ::protobuf::Message for CMsgBotWorldState_TrackingProjectile {
    fn is_initialized(&self) -> bool {
        for v in &self.location {
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
                    let tmp = is.read_uint32()?;
                    self.caster_handle = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.caster_player_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ability_handle = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ability_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.velocity = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_dodgeable = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_attack = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.caster_handle {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.caster_player_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ability_handle {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ability_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.velocity {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_dodgeable {
            my_size += 2;
        }
        if let Some(v) = self.is_attack {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.caster_handle {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.caster_player_id {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.ability_handle {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.ability_id {
            os.write_uint32(4, v)?;
        }
        if let Some(ref v) = self.location.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.velocity {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.is_dodgeable {
            os.write_bool(7, v)?;
        }
        if let Some(v) = self.is_attack {
            os.write_bool(8, v)?;
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_TrackingProjectile {
    fn new() -> CMsgBotWorldState_TrackingProjectile {
        CMsgBotWorldState_TrackingProjectile::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_TrackingProjectile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "caster_handle",
                    CMsgBotWorldState_TrackingProjectile::get_caster_handle_for_reflect,
                    CMsgBotWorldState_TrackingProjectile::mut_caster_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "caster_player_id",
                    CMsgBotWorldState_TrackingProjectile::get_caster_player_id_for_reflect,
                    CMsgBotWorldState_TrackingProjectile::mut_caster_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_handle",
                    CMsgBotWorldState_TrackingProjectile::get_ability_handle_for_reflect,
                    CMsgBotWorldState_TrackingProjectile::mut_ability_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_id",
                    CMsgBotWorldState_TrackingProjectile::get_ability_id_for_reflect,
                    CMsgBotWorldState_TrackingProjectile::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_Vector>>(
                    "location",
                    CMsgBotWorldState_TrackingProjectile::get_location_for_reflect,
                    CMsgBotWorldState_TrackingProjectile::mut_location_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "velocity",
                    CMsgBotWorldState_TrackingProjectile::get_velocity_for_reflect,
                    CMsgBotWorldState_TrackingProjectile::mut_velocity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_dodgeable",
                    CMsgBotWorldState_TrackingProjectile::get_is_dodgeable_for_reflect,
                    CMsgBotWorldState_TrackingProjectile::mut_is_dodgeable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_attack",
                    CMsgBotWorldState_TrackingProjectile::get_is_attack_for_reflect,
                    CMsgBotWorldState_TrackingProjectile::mut_is_attack_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_TrackingProjectile>(
                    "CMsgBotWorldState_TrackingProjectile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_TrackingProjectile {
    fn clear(&mut self) {
        self.clear_caster_handle();
        self.clear_caster_player_id();
        self.clear_ability_handle();
        self.clear_ability_id();
        self.clear_location();
        self.clear_velocity();
        self.clear_is_dodgeable();
        self.clear_is_attack();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_TrackingProjectile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_TrackingProjectile {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_AvoidanceZone {
    // message fields
    location: ::protobuf::SingularPtrField<CMsgBotWorldState_Vector>,
    caster_handle: ::std::option::Option<u32>,
    caster_player_id: ::std::option::Option<i32>,
    ability_handle: ::std::option::Option<u32>,
    ability_id: ::std::option::Option<u32>,
    radius: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_AvoidanceZone {}

impl CMsgBotWorldState_AvoidanceZone {
    pub fn new() -> CMsgBotWorldState_AvoidanceZone {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_AvoidanceZone {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_AvoidanceZone> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_AvoidanceZone,
        };
        unsafe {
            instance.get(CMsgBotWorldState_AvoidanceZone::new)
        }
    }

    // optional .CMsgBotWorldState.Vector location = 1;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: CMsgBotWorldState_Vector) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut CMsgBotWorldState_Vector {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> CMsgBotWorldState_Vector {
        self.location.take().unwrap_or_else(|| CMsgBotWorldState_Vector::new())
    }

    pub fn get_location(&self) -> &CMsgBotWorldState_Vector {
        self.location.as_ref().unwrap_or_else(|| CMsgBotWorldState_Vector::default_instance())
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &mut self.location
    }

    // optional uint32 caster_handle = 2;

    pub fn clear_caster_handle(&mut self) {
        self.caster_handle = ::std::option::Option::None;
    }

    pub fn has_caster_handle(&self) -> bool {
        self.caster_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_caster_handle(&mut self, v: u32) {
        self.caster_handle = ::std::option::Option::Some(v);
    }

    pub fn get_caster_handle(&self) -> u32 {
        self.caster_handle.unwrap_or(0)
    }

    fn get_caster_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.caster_handle
    }

    fn mut_caster_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.caster_handle
    }

    // optional int32 caster_player_id = 3;

    pub fn clear_caster_player_id(&mut self) {
        self.caster_player_id = ::std::option::Option::None;
    }

    pub fn has_caster_player_id(&self) -> bool {
        self.caster_player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_caster_player_id(&mut self, v: i32) {
        self.caster_player_id = ::std::option::Option::Some(v);
    }

    pub fn get_caster_player_id(&self) -> i32 {
        self.caster_player_id.unwrap_or(0)
    }

    fn get_caster_player_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.caster_player_id
    }

    fn mut_caster_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.caster_player_id
    }

    // optional uint32 ability_handle = 4;

    pub fn clear_ability_handle(&mut self) {
        self.ability_handle = ::std::option::Option::None;
    }

    pub fn has_ability_handle(&self) -> bool {
        self.ability_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_handle(&mut self, v: u32) {
        self.ability_handle = ::std::option::Option::Some(v);
    }

    pub fn get_ability_handle(&self) -> u32 {
        self.ability_handle.unwrap_or(0)
    }

    fn get_ability_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_handle
    }

    fn mut_ability_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_handle
    }

    // optional uint32 ability_id = 5;

    pub fn clear_ability_id(&mut self) {
        self.ability_id = ::std::option::Option::None;
    }

    pub fn has_ability_id(&self) -> bool {
        self.ability_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_id(&mut self, v: u32) {
        self.ability_id = ::std::option::Option::Some(v);
    }

    pub fn get_ability_id(&self) -> u32 {
        self.ability_id.unwrap_or(0)
    }

    fn get_ability_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_id
    }

    fn mut_ability_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_id
    }

    // optional uint32 radius = 6;

    pub fn clear_radius(&mut self) {
        self.radius = ::std::option::Option::None;
    }

    pub fn has_radius(&self) -> bool {
        self.radius.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radius(&mut self, v: u32) {
        self.radius = ::std::option::Option::Some(v);
    }

    pub fn get_radius(&self) -> u32 {
        self.radius.unwrap_or(0)
    }

    fn get_radius_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.radius
    }

    fn mut_radius_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.radius
    }
}

impl ::protobuf::Message for CMsgBotWorldState_AvoidanceZone {
    fn is_initialized(&self) -> bool {
        for v in &self.location {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.caster_handle = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.caster_player_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ability_handle = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ability_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.radius = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.caster_handle {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.caster_player_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ability_handle {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ability_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.radius {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.location.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.caster_handle {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.caster_player_id {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.ability_handle {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.ability_id {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.radius {
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_AvoidanceZone {
    fn new() -> CMsgBotWorldState_AvoidanceZone {
        CMsgBotWorldState_AvoidanceZone::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_AvoidanceZone>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_Vector>>(
                    "location",
                    CMsgBotWorldState_AvoidanceZone::get_location_for_reflect,
                    CMsgBotWorldState_AvoidanceZone::mut_location_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "caster_handle",
                    CMsgBotWorldState_AvoidanceZone::get_caster_handle_for_reflect,
                    CMsgBotWorldState_AvoidanceZone::mut_caster_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "caster_player_id",
                    CMsgBotWorldState_AvoidanceZone::get_caster_player_id_for_reflect,
                    CMsgBotWorldState_AvoidanceZone::mut_caster_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_handle",
                    CMsgBotWorldState_AvoidanceZone::get_ability_handle_for_reflect,
                    CMsgBotWorldState_AvoidanceZone::mut_ability_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_id",
                    CMsgBotWorldState_AvoidanceZone::get_ability_id_for_reflect,
                    CMsgBotWorldState_AvoidanceZone::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "radius",
                    CMsgBotWorldState_AvoidanceZone::get_radius_for_reflect,
                    CMsgBotWorldState_AvoidanceZone::mut_radius_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_AvoidanceZone>(
                    "CMsgBotWorldState_AvoidanceZone",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_AvoidanceZone {
    fn clear(&mut self) {
        self.clear_location();
        self.clear_caster_handle();
        self.clear_caster_player_id();
        self.clear_ability_handle();
        self.clear_ability_id();
        self.clear_radius();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_AvoidanceZone {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_AvoidanceZone {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_EventAbility {
    // message fields
    ability_id: ::std::option::Option<u32>,
    player_id: ::std::option::Option<u32>,
    unit_handle: ::std::option::Option<u32>,
    location: ::protobuf::SingularPtrField<CMsgBotWorldState_Vector>,
    is_channel_start: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_EventAbility {}

impl CMsgBotWorldState_EventAbility {
    pub fn new() -> CMsgBotWorldState_EventAbility {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_EventAbility {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_EventAbility> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_EventAbility,
        };
        unsafe {
            instance.get(CMsgBotWorldState_EventAbility::new)
        }
    }

    // optional uint32 ability_id = 1;

    pub fn clear_ability_id(&mut self) {
        self.ability_id = ::std::option::Option::None;
    }

    pub fn has_ability_id(&self) -> bool {
        self.ability_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_id(&mut self, v: u32) {
        self.ability_id = ::std::option::Option::Some(v);
    }

    pub fn get_ability_id(&self) -> u32 {
        self.ability_id.unwrap_or(0)
    }

    fn get_ability_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_id
    }

    fn mut_ability_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_id
    }

    // optional uint32 player_id = 2;

    pub fn clear_player_id(&mut self) {
        self.player_id = ::std::option::Option::None;
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: u32) {
        self.player_id = ::std::option::Option::Some(v);
    }

    pub fn get_player_id(&self) -> u32 {
        self.player_id.unwrap_or(0)
    }

    fn get_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.player_id
    }

    fn mut_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.player_id
    }

    // optional uint32 unit_handle = 3;

    pub fn clear_unit_handle(&mut self) {
        self.unit_handle = ::std::option::Option::None;
    }

    pub fn has_unit_handle(&self) -> bool {
        self.unit_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_handle(&mut self, v: u32) {
        self.unit_handle = ::std::option::Option::Some(v);
    }

    pub fn get_unit_handle(&self) -> u32 {
        self.unit_handle.unwrap_or(0)
    }

    fn get_unit_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.unit_handle
    }

    fn mut_unit_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.unit_handle
    }

    // optional .CMsgBotWorldState.Vector location = 4;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: CMsgBotWorldState_Vector) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut CMsgBotWorldState_Vector {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> CMsgBotWorldState_Vector {
        self.location.take().unwrap_or_else(|| CMsgBotWorldState_Vector::new())
    }

    pub fn get_location(&self) -> &CMsgBotWorldState_Vector {
        self.location.as_ref().unwrap_or_else(|| CMsgBotWorldState_Vector::default_instance())
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &mut self.location
    }

    // optional bool is_channel_start = 5;

    pub fn clear_is_channel_start(&mut self) {
        self.is_channel_start = ::std::option::Option::None;
    }

    pub fn has_is_channel_start(&self) -> bool {
        self.is_channel_start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_channel_start(&mut self, v: bool) {
        self.is_channel_start = ::std::option::Option::Some(v);
    }

    pub fn get_is_channel_start(&self) -> bool {
        self.is_channel_start.unwrap_or(false)
    }

    fn get_is_channel_start_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_channel_start
    }

    fn mut_is_channel_start_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_channel_start
    }
}

impl ::protobuf::Message for CMsgBotWorldState_EventAbility {
    fn is_initialized(&self) -> bool {
        for v in &self.location {
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
                    let tmp = is.read_uint32()?;
                    self.ability_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.player_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.unit_handle = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_channel_start = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.ability_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.player_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.unit_handle {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.is_channel_start {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ability_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.player_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.unit_handle {
            os.write_uint32(3, v)?;
        }
        if let Some(ref v) = self.location.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.is_channel_start {
            os.write_bool(5, v)?;
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_EventAbility {
    fn new() -> CMsgBotWorldState_EventAbility {
        CMsgBotWorldState_EventAbility::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_EventAbility>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_id",
                    CMsgBotWorldState_EventAbility::get_ability_id_for_reflect,
                    CMsgBotWorldState_EventAbility::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_id",
                    CMsgBotWorldState_EventAbility::get_player_id_for_reflect,
                    CMsgBotWorldState_EventAbility::mut_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "unit_handle",
                    CMsgBotWorldState_EventAbility::get_unit_handle_for_reflect,
                    CMsgBotWorldState_EventAbility::mut_unit_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_Vector>>(
                    "location",
                    CMsgBotWorldState_EventAbility::get_location_for_reflect,
                    CMsgBotWorldState_EventAbility::mut_location_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_channel_start",
                    CMsgBotWorldState_EventAbility::get_is_channel_start_for_reflect,
                    CMsgBotWorldState_EventAbility::mut_is_channel_start_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_EventAbility>(
                    "CMsgBotWorldState_EventAbility",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_EventAbility {
    fn clear(&mut self) {
        self.clear_ability_id();
        self.clear_player_id();
        self.clear_unit_handle();
        self.clear_location();
        self.clear_is_channel_start();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_EventAbility {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_EventAbility {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_EventDamage {
    // message fields
    damage: ::std::option::Option<u32>,
    victim_player_id: ::std::option::Option<u32>,
    victim_unit_handle: ::std::option::Option<u32>,
    attacker_player_id: ::std::option::Option<u32>,
    attacker_unit_handle: ::std::option::Option<u32>,
    ability_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_EventDamage {}

impl CMsgBotWorldState_EventDamage {
    pub fn new() -> CMsgBotWorldState_EventDamage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_EventDamage {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_EventDamage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_EventDamage,
        };
        unsafe {
            instance.get(CMsgBotWorldState_EventDamage::new)
        }
    }

    // optional uint32 damage = 1;

    pub fn clear_damage(&mut self) {
        self.damage = ::std::option::Option::None;
    }

    pub fn has_damage(&self) -> bool {
        self.damage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_damage(&mut self, v: u32) {
        self.damage = ::std::option::Option::Some(v);
    }

    pub fn get_damage(&self) -> u32 {
        self.damage.unwrap_or(0)
    }

    fn get_damage_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.damage
    }

    fn mut_damage_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.damage
    }

    // optional uint32 victim_player_id = 2;

    pub fn clear_victim_player_id(&mut self) {
        self.victim_player_id = ::std::option::Option::None;
    }

    pub fn has_victim_player_id(&self) -> bool {
        self.victim_player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_victim_player_id(&mut self, v: u32) {
        self.victim_player_id = ::std::option::Option::Some(v);
    }

    pub fn get_victim_player_id(&self) -> u32 {
        self.victim_player_id.unwrap_or(0)
    }

    fn get_victim_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.victim_player_id
    }

    fn mut_victim_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.victim_player_id
    }

    // optional uint32 victim_unit_handle = 3;

    pub fn clear_victim_unit_handle(&mut self) {
        self.victim_unit_handle = ::std::option::Option::None;
    }

    pub fn has_victim_unit_handle(&self) -> bool {
        self.victim_unit_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_victim_unit_handle(&mut self, v: u32) {
        self.victim_unit_handle = ::std::option::Option::Some(v);
    }

    pub fn get_victim_unit_handle(&self) -> u32 {
        self.victim_unit_handle.unwrap_or(0)
    }

    fn get_victim_unit_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.victim_unit_handle
    }

    fn mut_victim_unit_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.victim_unit_handle
    }

    // optional uint32 attacker_player_id = 4;

    pub fn clear_attacker_player_id(&mut self) {
        self.attacker_player_id = ::std::option::Option::None;
    }

    pub fn has_attacker_player_id(&self) -> bool {
        self.attacker_player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attacker_player_id(&mut self, v: u32) {
        self.attacker_player_id = ::std::option::Option::Some(v);
    }

    pub fn get_attacker_player_id(&self) -> u32 {
        self.attacker_player_id.unwrap_or(0)
    }

    fn get_attacker_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.attacker_player_id
    }

    fn mut_attacker_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.attacker_player_id
    }

    // optional uint32 attacker_unit_handle = 5;

    pub fn clear_attacker_unit_handle(&mut self) {
        self.attacker_unit_handle = ::std::option::Option::None;
    }

    pub fn has_attacker_unit_handle(&self) -> bool {
        self.attacker_unit_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attacker_unit_handle(&mut self, v: u32) {
        self.attacker_unit_handle = ::std::option::Option::Some(v);
    }

    pub fn get_attacker_unit_handle(&self) -> u32 {
        self.attacker_unit_handle.unwrap_or(0)
    }

    fn get_attacker_unit_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.attacker_unit_handle
    }

    fn mut_attacker_unit_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.attacker_unit_handle
    }

    // optional uint32 ability_id = 6;

    pub fn clear_ability_id(&mut self) {
        self.ability_id = ::std::option::Option::None;
    }

    pub fn has_ability_id(&self) -> bool {
        self.ability_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_id(&mut self, v: u32) {
        self.ability_id = ::std::option::Option::Some(v);
    }

    pub fn get_ability_id(&self) -> u32 {
        self.ability_id.unwrap_or(0)
    }

    fn get_ability_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_id
    }

    fn mut_ability_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_id
    }
}

impl ::protobuf::Message for CMsgBotWorldState_EventDamage {
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
                    self.damage = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.victim_player_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.victim_unit_handle = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.attacker_player_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.attacker_unit_handle = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ability_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.damage {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.victim_player_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.victim_unit_handle {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.attacker_player_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.attacker_unit_handle {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ability_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.damage {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.victim_player_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.victim_unit_handle {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.attacker_player_id {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.attacker_unit_handle {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.ability_id {
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_EventDamage {
    fn new() -> CMsgBotWorldState_EventDamage {
        CMsgBotWorldState_EventDamage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_EventDamage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "damage",
                    CMsgBotWorldState_EventDamage::get_damage_for_reflect,
                    CMsgBotWorldState_EventDamage::mut_damage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "victim_player_id",
                    CMsgBotWorldState_EventDamage::get_victim_player_id_for_reflect,
                    CMsgBotWorldState_EventDamage::mut_victim_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "victim_unit_handle",
                    CMsgBotWorldState_EventDamage::get_victim_unit_handle_for_reflect,
                    CMsgBotWorldState_EventDamage::mut_victim_unit_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "attacker_player_id",
                    CMsgBotWorldState_EventDamage::get_attacker_player_id_for_reflect,
                    CMsgBotWorldState_EventDamage::mut_attacker_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "attacker_unit_handle",
                    CMsgBotWorldState_EventDamage::get_attacker_unit_handle_for_reflect,
                    CMsgBotWorldState_EventDamage::mut_attacker_unit_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_id",
                    CMsgBotWorldState_EventDamage::get_ability_id_for_reflect,
                    CMsgBotWorldState_EventDamage::mut_ability_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_EventDamage>(
                    "CMsgBotWorldState_EventDamage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_EventDamage {
    fn clear(&mut self) {
        self.clear_damage();
        self.clear_victim_player_id();
        self.clear_victim_unit_handle();
        self.clear_attacker_player_id();
        self.clear_attacker_unit_handle();
        self.clear_ability_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_EventDamage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_EventDamage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_EventCourierKilled {
    // message fields
    team_id: ::std::option::Option<u32>,
    courier_unit_handle: ::std::option::Option<u32>,
    killer_player_id: ::std::option::Option<u32>,
    killer_unit_handle: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_EventCourierKilled {}

impl CMsgBotWorldState_EventCourierKilled {
    pub fn new() -> CMsgBotWorldState_EventCourierKilled {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_EventCourierKilled {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_EventCourierKilled> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_EventCourierKilled,
        };
        unsafe {
            instance.get(CMsgBotWorldState_EventCourierKilled::new)
        }
    }

    // optional uint32 team_id = 1;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
    }

    // optional uint32 courier_unit_handle = 2;

    pub fn clear_courier_unit_handle(&mut self) {
        self.courier_unit_handle = ::std::option::Option::None;
    }

    pub fn has_courier_unit_handle(&self) -> bool {
        self.courier_unit_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_courier_unit_handle(&mut self, v: u32) {
        self.courier_unit_handle = ::std::option::Option::Some(v);
    }

    pub fn get_courier_unit_handle(&self) -> u32 {
        self.courier_unit_handle.unwrap_or(0)
    }

    fn get_courier_unit_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.courier_unit_handle
    }

    fn mut_courier_unit_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.courier_unit_handle
    }

    // optional uint32 killer_player_id = 3;

    pub fn clear_killer_player_id(&mut self) {
        self.killer_player_id = ::std::option::Option::None;
    }

    pub fn has_killer_player_id(&self) -> bool {
        self.killer_player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_killer_player_id(&mut self, v: u32) {
        self.killer_player_id = ::std::option::Option::Some(v);
    }

    pub fn get_killer_player_id(&self) -> u32 {
        self.killer_player_id.unwrap_or(0)
    }

    fn get_killer_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.killer_player_id
    }

    fn mut_killer_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.killer_player_id
    }

    // optional uint32 killer_unit_handle = 4;

    pub fn clear_killer_unit_handle(&mut self) {
        self.killer_unit_handle = ::std::option::Option::None;
    }

    pub fn has_killer_unit_handle(&self) -> bool {
        self.killer_unit_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_killer_unit_handle(&mut self, v: u32) {
        self.killer_unit_handle = ::std::option::Option::Some(v);
    }

    pub fn get_killer_unit_handle(&self) -> u32 {
        self.killer_unit_handle.unwrap_or(0)
    }

    fn get_killer_unit_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.killer_unit_handle
    }

    fn mut_killer_unit_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.killer_unit_handle
    }
}

impl ::protobuf::Message for CMsgBotWorldState_EventCourierKilled {
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
                    self.team_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.courier_unit_handle = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.killer_player_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.killer_unit_handle = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.courier_unit_handle {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.killer_player_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.killer_unit_handle {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.courier_unit_handle {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.killer_player_id {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.killer_unit_handle {
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_EventCourierKilled {
    fn new() -> CMsgBotWorldState_EventCourierKilled {
        CMsgBotWorldState_EventCourierKilled::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_EventCourierKilled>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgBotWorldState_EventCourierKilled::get_team_id_for_reflect,
                    CMsgBotWorldState_EventCourierKilled::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "courier_unit_handle",
                    CMsgBotWorldState_EventCourierKilled::get_courier_unit_handle_for_reflect,
                    CMsgBotWorldState_EventCourierKilled::mut_courier_unit_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "killer_player_id",
                    CMsgBotWorldState_EventCourierKilled::get_killer_player_id_for_reflect,
                    CMsgBotWorldState_EventCourierKilled::mut_killer_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "killer_unit_handle",
                    CMsgBotWorldState_EventCourierKilled::get_killer_unit_handle_for_reflect,
                    CMsgBotWorldState_EventCourierKilled::mut_killer_unit_handle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_EventCourierKilled>(
                    "CMsgBotWorldState_EventCourierKilled",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_EventCourierKilled {
    fn clear(&mut self) {
        self.clear_team_id();
        self.clear_courier_unit_handle();
        self.clear_killer_player_id();
        self.clear_killer_unit_handle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_EventCourierKilled {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_EventCourierKilled {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_EventRoshanKilled {
    // message fields
    killer_player_id: ::std::option::Option<u32>,
    killer_unit_handle: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_EventRoshanKilled {}

impl CMsgBotWorldState_EventRoshanKilled {
    pub fn new() -> CMsgBotWorldState_EventRoshanKilled {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_EventRoshanKilled {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_EventRoshanKilled> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_EventRoshanKilled,
        };
        unsafe {
            instance.get(CMsgBotWorldState_EventRoshanKilled::new)
        }
    }

    // optional uint32 killer_player_id = 1;

    pub fn clear_killer_player_id(&mut self) {
        self.killer_player_id = ::std::option::Option::None;
    }

    pub fn has_killer_player_id(&self) -> bool {
        self.killer_player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_killer_player_id(&mut self, v: u32) {
        self.killer_player_id = ::std::option::Option::Some(v);
    }

    pub fn get_killer_player_id(&self) -> u32 {
        self.killer_player_id.unwrap_or(0)
    }

    fn get_killer_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.killer_player_id
    }

    fn mut_killer_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.killer_player_id
    }

    // optional uint32 killer_unit_handle = 2;

    pub fn clear_killer_unit_handle(&mut self) {
        self.killer_unit_handle = ::std::option::Option::None;
    }

    pub fn has_killer_unit_handle(&self) -> bool {
        self.killer_unit_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_killer_unit_handle(&mut self, v: u32) {
        self.killer_unit_handle = ::std::option::Option::Some(v);
    }

    pub fn get_killer_unit_handle(&self) -> u32 {
        self.killer_unit_handle.unwrap_or(0)
    }

    fn get_killer_unit_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.killer_unit_handle
    }

    fn mut_killer_unit_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.killer_unit_handle
    }
}

impl ::protobuf::Message for CMsgBotWorldState_EventRoshanKilled {
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
                    self.killer_player_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.killer_unit_handle = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.killer_player_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.killer_unit_handle {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.killer_player_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.killer_unit_handle {
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_EventRoshanKilled {
    fn new() -> CMsgBotWorldState_EventRoshanKilled {
        CMsgBotWorldState_EventRoshanKilled::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_EventRoshanKilled>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "killer_player_id",
                    CMsgBotWorldState_EventRoshanKilled::get_killer_player_id_for_reflect,
                    CMsgBotWorldState_EventRoshanKilled::mut_killer_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "killer_unit_handle",
                    CMsgBotWorldState_EventRoshanKilled::get_killer_unit_handle_for_reflect,
                    CMsgBotWorldState_EventRoshanKilled::mut_killer_unit_handle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_EventRoshanKilled>(
                    "CMsgBotWorldState_EventRoshanKilled",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_EventRoshanKilled {
    fn clear(&mut self) {
        self.clear_killer_player_id();
        self.clear_killer_unit_handle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_EventRoshanKilled {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_EventRoshanKilled {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_EventTree {
    // message fields
    tree_id: ::std::option::Option<u32>,
    destroyed: ::std::option::Option<bool>,
    respawned: ::std::option::Option<bool>,
    location: ::protobuf::SingularPtrField<CMsgBotWorldState_Vector>,
    delayed: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_EventTree {}

impl CMsgBotWorldState_EventTree {
    pub fn new() -> CMsgBotWorldState_EventTree {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_EventTree {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_EventTree> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_EventTree,
        };
        unsafe {
            instance.get(CMsgBotWorldState_EventTree::new)
        }
    }

    // optional uint32 tree_id = 1;

    pub fn clear_tree_id(&mut self) {
        self.tree_id = ::std::option::Option::None;
    }

    pub fn has_tree_id(&self) -> bool {
        self.tree_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tree_id(&mut self, v: u32) {
        self.tree_id = ::std::option::Option::Some(v);
    }

    pub fn get_tree_id(&self) -> u32 {
        self.tree_id.unwrap_or(0)
    }

    fn get_tree_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tree_id
    }

    fn mut_tree_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tree_id
    }

    // optional bool destroyed = 2;

    pub fn clear_destroyed(&mut self) {
        self.destroyed = ::std::option::Option::None;
    }

    pub fn has_destroyed(&self) -> bool {
        self.destroyed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destroyed(&mut self, v: bool) {
        self.destroyed = ::std::option::Option::Some(v);
    }

    pub fn get_destroyed(&self) -> bool {
        self.destroyed.unwrap_or(false)
    }

    fn get_destroyed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.destroyed
    }

    fn mut_destroyed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.destroyed
    }

    // optional bool respawned = 3;

    pub fn clear_respawned(&mut self) {
        self.respawned = ::std::option::Option::None;
    }

    pub fn has_respawned(&self) -> bool {
        self.respawned.is_some()
    }

    // Param is passed by value, moved
    pub fn set_respawned(&mut self, v: bool) {
        self.respawned = ::std::option::Option::Some(v);
    }

    pub fn get_respawned(&self) -> bool {
        self.respawned.unwrap_or(false)
    }

    fn get_respawned_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.respawned
    }

    fn mut_respawned_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.respawned
    }

    // optional .CMsgBotWorldState.Vector location = 4;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: CMsgBotWorldState_Vector) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut CMsgBotWorldState_Vector {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> CMsgBotWorldState_Vector {
        self.location.take().unwrap_or_else(|| CMsgBotWorldState_Vector::new())
    }

    pub fn get_location(&self) -> &CMsgBotWorldState_Vector {
        self.location.as_ref().unwrap_or_else(|| CMsgBotWorldState_Vector::default_instance())
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &mut self.location
    }

    // optional bool delayed = 5;

    pub fn clear_delayed(&mut self) {
        self.delayed = ::std::option::Option::None;
    }

    pub fn has_delayed(&self) -> bool {
        self.delayed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delayed(&mut self, v: bool) {
        self.delayed = ::std::option::Option::Some(v);
    }

    pub fn get_delayed(&self) -> bool {
        self.delayed.unwrap_or(false)
    }

    fn get_delayed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.delayed
    }

    fn mut_delayed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.delayed
    }
}

impl ::protobuf::Message for CMsgBotWorldState_EventTree {
    fn is_initialized(&self) -> bool {
        for v in &self.location {
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
                    let tmp = is.read_uint32()?;
                    self.tree_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.destroyed = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.respawned = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.delayed = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.tree_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.destroyed {
            my_size += 2;
        }
        if let Some(v) = self.respawned {
            my_size += 2;
        }
        if let Some(ref v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.delayed {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tree_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.destroyed {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.respawned {
            os.write_bool(3, v)?;
        }
        if let Some(ref v) = self.location.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.delayed {
            os.write_bool(5, v)?;
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_EventTree {
    fn new() -> CMsgBotWorldState_EventTree {
        CMsgBotWorldState_EventTree::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_EventTree>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tree_id",
                    CMsgBotWorldState_EventTree::get_tree_id_for_reflect,
                    CMsgBotWorldState_EventTree::mut_tree_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "destroyed",
                    CMsgBotWorldState_EventTree::get_destroyed_for_reflect,
                    CMsgBotWorldState_EventTree::mut_destroyed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "respawned",
                    CMsgBotWorldState_EventTree::get_respawned_for_reflect,
                    CMsgBotWorldState_EventTree::mut_respawned_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_Vector>>(
                    "location",
                    CMsgBotWorldState_EventTree::get_location_for_reflect,
                    CMsgBotWorldState_EventTree::mut_location_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "delayed",
                    CMsgBotWorldState_EventTree::get_delayed_for_reflect,
                    CMsgBotWorldState_EventTree::mut_delayed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_EventTree>(
                    "CMsgBotWorldState_EventTree",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_EventTree {
    fn clear(&mut self) {
        self.clear_tree_id();
        self.clear_destroyed();
        self.clear_respawned();
        self.clear_location();
        self.clear_delayed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_EventTree {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_EventTree {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotWorldState_Unit {
    // message fields
    handle: ::std::option::Option<u32>,
    unit_type: ::std::option::Option<CMsgBotWorldState_UnitType>,
    name: ::protobuf::SingularField<::std::string::String>,
    team_id: ::std::option::Option<u32>,
    level: ::std::option::Option<u32>,
    location: ::protobuf::SingularPtrField<CMsgBotWorldState_Vector>,
    is_alive: ::std::option::Option<bool>,
    bounding_radius: ::std::option::Option<u32>,
    facing: ::std::option::Option<u32>,
    ground_height: ::std::option::Option<u32>,
    vision_range_daytime: ::std::option::Option<u32>,
    vision_range_nighttime: ::std::option::Option<u32>,
    health: ::std::option::Option<u32>,
    health_max: ::std::option::Option<u32>,
    health_regen: ::std::option::Option<f32>,
    mana: ::std::option::Option<u32>,
    mana_max: ::std::option::Option<u32>,
    mana_regen: ::std::option::Option<f32>,
    base_movement_speed: ::std::option::Option<u32>,
    current_movement_speed: ::std::option::Option<u32>,
    anim_activity: ::std::option::Option<i32>,
    anim_cycle: ::std::option::Option<f32>,
    base_damage: ::std::option::Option<u32>,
    base_damage_variance: ::std::option::Option<u32>,
    bonus_damage: ::std::option::Option<u32>,
    attack_damage: ::std::option::Option<u32>,
    attack_range: ::std::option::Option<u32>,
    attack_speed: ::std::option::Option<u32>,
    attack_anim_point: ::std::option::Option<f32>,
    attack_acquisition_range: ::std::option::Option<u32>,
    attack_projectile_speed: ::std::option::Option<u32>,
    attack_target_handle: ::std::option::Option<u32>,
    bounty_xp: ::std::option::Option<u32>,
    bounty_gold_min: ::std::option::Option<u32>,
    bounty_gold_max: ::std::option::Option<u32>,
    is_channeling: ::std::option::Option<bool>,
    active_ability_handle: ::std::option::Option<u32>,
    is_attack_immune: ::std::option::Option<bool>,
    is_blind: ::std::option::Option<bool>,
    is_block_disabled: ::std::option::Option<bool>,
    is_disarmed: ::std::option::Option<bool>,
    is_dominated: ::std::option::Option<bool>,
    is_evade_disabled: ::std::option::Option<bool>,
    is_hexed: ::std::option::Option<bool>,
    is_invisible: ::std::option::Option<bool>,
    is_invulnerable: ::std::option::Option<bool>,
    is_magic_immune: ::std::option::Option<bool>,
    is_muted: ::std::option::Option<bool>,
    is_nightmared: ::std::option::Option<bool>,
    is_rooted: ::std::option::Option<bool>,
    is_silenced: ::std::option::Option<bool>,
    is_specially_deniable: ::std::option::Option<bool>,
    is_stunned: ::std::option::Option<bool>,
    is_unable_to_miss: ::std::option::Option<bool>,
    has_scepter: ::std::option::Option<bool>,
    abilities: ::protobuf::RepeatedField<CMsgBotWorldState_Ability>,
    items: ::protobuf::RepeatedField<CMsgBotWorldState_Ability>,
    modifiers: ::protobuf::RepeatedField<CMsgBotWorldState_Modifier>,
    incoming_tracking_projectiles: ::protobuf::RepeatedField<CMsgBotWorldState_TrackingProjectile>,
    action_type: ::std::option::Option<u32>,
    ability_target_handle: ::std::option::Option<u32>,
    primary_attribute: ::std::option::Option<u32>,
    is_illusion: ::std::option::Option<bool>,
    respawn_time: ::std::option::Option<u32>,
    buyback_cost: ::std::option::Option<u32>,
    buyback_cooldown: ::std::option::Option<u32>,
    spell_amplifiction: ::std::option::Option<f32>,
    armor: ::std::option::Option<u32>,
    magic_resist: ::std::option::Option<f32>,
    evasion: ::std::option::Option<f32>,
    xp_needed_to_level: ::std::option::Option<u32>,
    ability_points: ::std::option::Option<u32>,
    reliable_gold: ::std::option::Option<u32>,
    unreliable_gold: ::std::option::Option<u32>,
    last_hits: ::std::option::Option<u32>,
    denies: ::std::option::Option<u32>,
    net_worth: ::std::option::Option<u32>,
    remaining_lifespan: ::std::option::Option<f32>,
    flying_courier: ::std::option::Option<bool>,
    shrine_cooldown: ::std::option::Option<f32>,
    is_shrine_healing: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotWorldState_Unit {}

impl CMsgBotWorldState_Unit {
    pub fn new() -> CMsgBotWorldState_Unit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotWorldState_Unit {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotWorldState_Unit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotWorldState_Unit,
        };
        unsafe {
            instance.get(CMsgBotWorldState_Unit::new)
        }
    }

    // optional uint32 handle = 1;

    pub fn clear_handle(&mut self) {
        self.handle = ::std::option::Option::None;
    }

    pub fn has_handle(&self) -> bool {
        self.handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_handle(&mut self, v: u32) {
        self.handle = ::std::option::Option::Some(v);
    }

    pub fn get_handle(&self) -> u32 {
        self.handle.unwrap_or(0)
    }

    fn get_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.handle
    }

    fn mut_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.handle
    }

    // optional .CMsgBotWorldState.UnitType unit_type = 2;

    pub fn clear_unit_type(&mut self) {
        self.unit_type = ::std::option::Option::None;
    }

    pub fn has_unit_type(&self) -> bool {
        self.unit_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_type(&mut self, v: CMsgBotWorldState_UnitType) {
        self.unit_type = ::std::option::Option::Some(v);
    }

    pub fn get_unit_type(&self) -> CMsgBotWorldState_UnitType {
        self.unit_type.unwrap_or(CMsgBotWorldState_UnitType::INVALID)
    }

    fn get_unit_type_for_reflect(&self) -> &::std::option::Option<CMsgBotWorldState_UnitType> {
        &self.unit_type
    }

    fn mut_unit_type_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgBotWorldState_UnitType> {
        &mut self.unit_type
    }

    // optional string name = 3;

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

    // optional uint32 team_id = 4;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
    }

    // optional uint32 level = 5;

    pub fn clear_level(&mut self) {
        self.level = ::std::option::Option::None;
    }

    pub fn has_level(&self) -> bool {
        self.level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_level(&mut self, v: u32) {
        self.level = ::std::option::Option::Some(v);
    }

    pub fn get_level(&self) -> u32 {
        self.level.unwrap_or(0)
    }

    fn get_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.level
    }

    fn mut_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.level
    }

    // optional .CMsgBotWorldState.Vector location = 6;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: CMsgBotWorldState_Vector) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut CMsgBotWorldState_Vector {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> CMsgBotWorldState_Vector {
        self.location.take().unwrap_or_else(|| CMsgBotWorldState_Vector::new())
    }

    pub fn get_location(&self) -> &CMsgBotWorldState_Vector {
        self.location.as_ref().unwrap_or_else(|| CMsgBotWorldState_Vector::default_instance())
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgBotWorldState_Vector> {
        &mut self.location
    }

    // optional bool is_alive = 7;

    pub fn clear_is_alive(&mut self) {
        self.is_alive = ::std::option::Option::None;
    }

    pub fn has_is_alive(&self) -> bool {
        self.is_alive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_alive(&mut self, v: bool) {
        self.is_alive = ::std::option::Option::Some(v);
    }

    pub fn get_is_alive(&self) -> bool {
        self.is_alive.unwrap_or(false)
    }

    fn get_is_alive_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_alive
    }

    fn mut_is_alive_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_alive
    }

    // optional uint32 bounding_radius = 10;

    pub fn clear_bounding_radius(&mut self) {
        self.bounding_radius = ::std::option::Option::None;
    }

    pub fn has_bounding_radius(&self) -> bool {
        self.bounding_radius.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bounding_radius(&mut self, v: u32) {
        self.bounding_radius = ::std::option::Option::Some(v);
    }

    pub fn get_bounding_radius(&self) -> u32 {
        self.bounding_radius.unwrap_or(0)
    }

    fn get_bounding_radius_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.bounding_radius
    }

    fn mut_bounding_radius_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.bounding_radius
    }

    // optional uint32 facing = 11;

    pub fn clear_facing(&mut self) {
        self.facing = ::std::option::Option::None;
    }

    pub fn has_facing(&self) -> bool {
        self.facing.is_some()
    }

    // Param is passed by value, moved
    pub fn set_facing(&mut self, v: u32) {
        self.facing = ::std::option::Option::Some(v);
    }

    pub fn get_facing(&self) -> u32 {
        self.facing.unwrap_or(0)
    }

    fn get_facing_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.facing
    }

    fn mut_facing_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.facing
    }

    // optional uint32 ground_height = 12;

    pub fn clear_ground_height(&mut self) {
        self.ground_height = ::std::option::Option::None;
    }

    pub fn has_ground_height(&self) -> bool {
        self.ground_height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ground_height(&mut self, v: u32) {
        self.ground_height = ::std::option::Option::Some(v);
    }

    pub fn get_ground_height(&self) -> u32 {
        self.ground_height.unwrap_or(0)
    }

    fn get_ground_height_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ground_height
    }

    fn mut_ground_height_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ground_height
    }

    // optional uint32 vision_range_daytime = 15;

    pub fn clear_vision_range_daytime(&mut self) {
        self.vision_range_daytime = ::std::option::Option::None;
    }

    pub fn has_vision_range_daytime(&self) -> bool {
        self.vision_range_daytime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vision_range_daytime(&mut self, v: u32) {
        self.vision_range_daytime = ::std::option::Option::Some(v);
    }

    pub fn get_vision_range_daytime(&self) -> u32 {
        self.vision_range_daytime.unwrap_or(0)
    }

    fn get_vision_range_daytime_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.vision_range_daytime
    }

    fn mut_vision_range_daytime_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.vision_range_daytime
    }

    // optional uint32 vision_range_nighttime = 16;

    pub fn clear_vision_range_nighttime(&mut self) {
        self.vision_range_nighttime = ::std::option::Option::None;
    }

    pub fn has_vision_range_nighttime(&self) -> bool {
        self.vision_range_nighttime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vision_range_nighttime(&mut self, v: u32) {
        self.vision_range_nighttime = ::std::option::Option::Some(v);
    }

    pub fn get_vision_range_nighttime(&self) -> u32 {
        self.vision_range_nighttime.unwrap_or(0)
    }

    fn get_vision_range_nighttime_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.vision_range_nighttime
    }

    fn mut_vision_range_nighttime_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.vision_range_nighttime
    }

    // optional uint32 health = 20;

    pub fn clear_health(&mut self) {
        self.health = ::std::option::Option::None;
    }

    pub fn has_health(&self) -> bool {
        self.health.is_some()
    }

    // Param is passed by value, moved
    pub fn set_health(&mut self, v: u32) {
        self.health = ::std::option::Option::Some(v);
    }

    pub fn get_health(&self) -> u32 {
        self.health.unwrap_or(0)
    }

    fn get_health_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.health
    }

    fn mut_health_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.health
    }

    // optional uint32 health_max = 21;

    pub fn clear_health_max(&mut self) {
        self.health_max = ::std::option::Option::None;
    }

    pub fn has_health_max(&self) -> bool {
        self.health_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_health_max(&mut self, v: u32) {
        self.health_max = ::std::option::Option::Some(v);
    }

    pub fn get_health_max(&self) -> u32 {
        self.health_max.unwrap_or(0)
    }

    fn get_health_max_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.health_max
    }

    fn mut_health_max_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.health_max
    }

    // optional float health_regen = 22;

    pub fn clear_health_regen(&mut self) {
        self.health_regen = ::std::option::Option::None;
    }

    pub fn has_health_regen(&self) -> bool {
        self.health_regen.is_some()
    }

    // Param is passed by value, moved
    pub fn set_health_regen(&mut self, v: f32) {
        self.health_regen = ::std::option::Option::Some(v);
    }

    pub fn get_health_regen(&self) -> f32 {
        self.health_regen.unwrap_or(0.)
    }

    fn get_health_regen_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.health_regen
    }

    fn mut_health_regen_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.health_regen
    }

    // optional uint32 mana = 25;

    pub fn clear_mana(&mut self) {
        self.mana = ::std::option::Option::None;
    }

    pub fn has_mana(&self) -> bool {
        self.mana.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mana(&mut self, v: u32) {
        self.mana = ::std::option::Option::Some(v);
    }

    pub fn get_mana(&self) -> u32 {
        self.mana.unwrap_or(0)
    }

    fn get_mana_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.mana
    }

    fn mut_mana_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.mana
    }

    // optional uint32 mana_max = 26;

    pub fn clear_mana_max(&mut self) {
        self.mana_max = ::std::option::Option::None;
    }

    pub fn has_mana_max(&self) -> bool {
        self.mana_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mana_max(&mut self, v: u32) {
        self.mana_max = ::std::option::Option::Some(v);
    }

    pub fn get_mana_max(&self) -> u32 {
        self.mana_max.unwrap_or(0)
    }

    fn get_mana_max_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.mana_max
    }

    fn mut_mana_max_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.mana_max
    }

    // optional float mana_regen = 27;

    pub fn clear_mana_regen(&mut self) {
        self.mana_regen = ::std::option::Option::None;
    }

    pub fn has_mana_regen(&self) -> bool {
        self.mana_regen.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mana_regen(&mut self, v: f32) {
        self.mana_regen = ::std::option::Option::Some(v);
    }

    pub fn get_mana_regen(&self) -> f32 {
        self.mana_regen.unwrap_or(0.)
    }

    fn get_mana_regen_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.mana_regen
    }

    fn mut_mana_regen_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.mana_regen
    }

    // optional uint32 base_movement_speed = 30;

    pub fn clear_base_movement_speed(&mut self) {
        self.base_movement_speed = ::std::option::Option::None;
    }

    pub fn has_base_movement_speed(&self) -> bool {
        self.base_movement_speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_movement_speed(&mut self, v: u32) {
        self.base_movement_speed = ::std::option::Option::Some(v);
    }

    pub fn get_base_movement_speed(&self) -> u32 {
        self.base_movement_speed.unwrap_or(0)
    }

    fn get_base_movement_speed_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.base_movement_speed
    }

    fn mut_base_movement_speed_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.base_movement_speed
    }

    // optional uint32 current_movement_speed = 31;

    pub fn clear_current_movement_speed(&mut self) {
        self.current_movement_speed = ::std::option::Option::None;
    }

    pub fn has_current_movement_speed(&self) -> bool {
        self.current_movement_speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current_movement_speed(&mut self, v: u32) {
        self.current_movement_speed = ::std::option::Option::Some(v);
    }

    pub fn get_current_movement_speed(&self) -> u32 {
        self.current_movement_speed.unwrap_or(0)
    }

    fn get_current_movement_speed_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.current_movement_speed
    }

    fn mut_current_movement_speed_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.current_movement_speed
    }

    // optional int32 anim_activity = 35;

    pub fn clear_anim_activity(&mut self) {
        self.anim_activity = ::std::option::Option::None;
    }

    pub fn has_anim_activity(&self) -> bool {
        self.anim_activity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_anim_activity(&mut self, v: i32) {
        self.anim_activity = ::std::option::Option::Some(v);
    }

    pub fn get_anim_activity(&self) -> i32 {
        self.anim_activity.unwrap_or(0)
    }

    fn get_anim_activity_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.anim_activity
    }

    fn mut_anim_activity_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.anim_activity
    }

    // optional float anim_cycle = 36;

    pub fn clear_anim_cycle(&mut self) {
        self.anim_cycle = ::std::option::Option::None;
    }

    pub fn has_anim_cycle(&self) -> bool {
        self.anim_cycle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_anim_cycle(&mut self, v: f32) {
        self.anim_cycle = ::std::option::Option::Some(v);
    }

    pub fn get_anim_cycle(&self) -> f32 {
        self.anim_cycle.unwrap_or(0.)
    }

    fn get_anim_cycle_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.anim_cycle
    }

    fn mut_anim_cycle_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.anim_cycle
    }

    // optional uint32 base_damage = 40;

    pub fn clear_base_damage(&mut self) {
        self.base_damage = ::std::option::Option::None;
    }

    pub fn has_base_damage(&self) -> bool {
        self.base_damage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_damage(&mut self, v: u32) {
        self.base_damage = ::std::option::Option::Some(v);
    }

    pub fn get_base_damage(&self) -> u32 {
        self.base_damage.unwrap_or(0)
    }

    fn get_base_damage_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.base_damage
    }

    fn mut_base_damage_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.base_damage
    }

    // optional uint32 base_damage_variance = 41;

    pub fn clear_base_damage_variance(&mut self) {
        self.base_damage_variance = ::std::option::Option::None;
    }

    pub fn has_base_damage_variance(&self) -> bool {
        self.base_damage_variance.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_damage_variance(&mut self, v: u32) {
        self.base_damage_variance = ::std::option::Option::Some(v);
    }

    pub fn get_base_damage_variance(&self) -> u32 {
        self.base_damage_variance.unwrap_or(0)
    }

    fn get_base_damage_variance_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.base_damage_variance
    }

    fn mut_base_damage_variance_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.base_damage_variance
    }

    // optional uint32 bonus_damage = 42;

    pub fn clear_bonus_damage(&mut self) {
        self.bonus_damage = ::std::option::Option::None;
    }

    pub fn has_bonus_damage(&self) -> bool {
        self.bonus_damage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bonus_damage(&mut self, v: u32) {
        self.bonus_damage = ::std::option::Option::Some(v);
    }

    pub fn get_bonus_damage(&self) -> u32 {
        self.bonus_damage.unwrap_or(0)
    }

    fn get_bonus_damage_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.bonus_damage
    }

    fn mut_bonus_damage_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.bonus_damage
    }

    // optional uint32 attack_damage = 43;

    pub fn clear_attack_damage(&mut self) {
        self.attack_damage = ::std::option::Option::None;
    }

    pub fn has_attack_damage(&self) -> bool {
        self.attack_damage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attack_damage(&mut self, v: u32) {
        self.attack_damage = ::std::option::Option::Some(v);
    }

    pub fn get_attack_damage(&self) -> u32 {
        self.attack_damage.unwrap_or(0)
    }

    fn get_attack_damage_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.attack_damage
    }

    fn mut_attack_damage_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.attack_damage
    }

    // optional uint32 attack_range = 44;

    pub fn clear_attack_range(&mut self) {
        self.attack_range = ::std::option::Option::None;
    }

    pub fn has_attack_range(&self) -> bool {
        self.attack_range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attack_range(&mut self, v: u32) {
        self.attack_range = ::std::option::Option::Some(v);
    }

    pub fn get_attack_range(&self) -> u32 {
        self.attack_range.unwrap_or(0)
    }

    fn get_attack_range_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.attack_range
    }

    fn mut_attack_range_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.attack_range
    }

    // optional uint32 attack_speed = 45;

    pub fn clear_attack_speed(&mut self) {
        self.attack_speed = ::std::option::Option::None;
    }

    pub fn has_attack_speed(&self) -> bool {
        self.attack_speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attack_speed(&mut self, v: u32) {
        self.attack_speed = ::std::option::Option::Some(v);
    }

    pub fn get_attack_speed(&self) -> u32 {
        self.attack_speed.unwrap_or(0)
    }

    fn get_attack_speed_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.attack_speed
    }

    fn mut_attack_speed_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.attack_speed
    }

    // optional float attack_anim_point = 46;

    pub fn clear_attack_anim_point(&mut self) {
        self.attack_anim_point = ::std::option::Option::None;
    }

    pub fn has_attack_anim_point(&self) -> bool {
        self.attack_anim_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attack_anim_point(&mut self, v: f32) {
        self.attack_anim_point = ::std::option::Option::Some(v);
    }

    pub fn get_attack_anim_point(&self) -> f32 {
        self.attack_anim_point.unwrap_or(0.)
    }

    fn get_attack_anim_point_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.attack_anim_point
    }

    fn mut_attack_anim_point_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.attack_anim_point
    }

    // optional uint32 attack_acquisition_range = 47;

    pub fn clear_attack_acquisition_range(&mut self) {
        self.attack_acquisition_range = ::std::option::Option::None;
    }

    pub fn has_attack_acquisition_range(&self) -> bool {
        self.attack_acquisition_range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attack_acquisition_range(&mut self, v: u32) {
        self.attack_acquisition_range = ::std::option::Option::Some(v);
    }

    pub fn get_attack_acquisition_range(&self) -> u32 {
        self.attack_acquisition_range.unwrap_or(0)
    }

    fn get_attack_acquisition_range_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.attack_acquisition_range
    }

    fn mut_attack_acquisition_range_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.attack_acquisition_range
    }

    // optional uint32 attack_projectile_speed = 48;

    pub fn clear_attack_projectile_speed(&mut self) {
        self.attack_projectile_speed = ::std::option::Option::None;
    }

    pub fn has_attack_projectile_speed(&self) -> bool {
        self.attack_projectile_speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attack_projectile_speed(&mut self, v: u32) {
        self.attack_projectile_speed = ::std::option::Option::Some(v);
    }

    pub fn get_attack_projectile_speed(&self) -> u32 {
        self.attack_projectile_speed.unwrap_or(0)
    }

    fn get_attack_projectile_speed_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.attack_projectile_speed
    }

    fn mut_attack_projectile_speed_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.attack_projectile_speed
    }

    // optional uint32 attack_target_handle = 49;

    pub fn clear_attack_target_handle(&mut self) {
        self.attack_target_handle = ::std::option::Option::None;
    }

    pub fn has_attack_target_handle(&self) -> bool {
        self.attack_target_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attack_target_handle(&mut self, v: u32) {
        self.attack_target_handle = ::std::option::Option::Some(v);
    }

    pub fn get_attack_target_handle(&self) -> u32 {
        self.attack_target_handle.unwrap_or(0)
    }

    fn get_attack_target_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.attack_target_handle
    }

    fn mut_attack_target_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.attack_target_handle
    }

    // optional uint32 bounty_xp = 60;

    pub fn clear_bounty_xp(&mut self) {
        self.bounty_xp = ::std::option::Option::None;
    }

    pub fn has_bounty_xp(&self) -> bool {
        self.bounty_xp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bounty_xp(&mut self, v: u32) {
        self.bounty_xp = ::std::option::Option::Some(v);
    }

    pub fn get_bounty_xp(&self) -> u32 {
        self.bounty_xp.unwrap_or(0)
    }

    fn get_bounty_xp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.bounty_xp
    }

    fn mut_bounty_xp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.bounty_xp
    }

    // optional uint32 bounty_gold_min = 61;

    pub fn clear_bounty_gold_min(&mut self) {
        self.bounty_gold_min = ::std::option::Option::None;
    }

    pub fn has_bounty_gold_min(&self) -> bool {
        self.bounty_gold_min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bounty_gold_min(&mut self, v: u32) {
        self.bounty_gold_min = ::std::option::Option::Some(v);
    }

    pub fn get_bounty_gold_min(&self) -> u32 {
        self.bounty_gold_min.unwrap_or(0)
    }

    fn get_bounty_gold_min_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.bounty_gold_min
    }

    fn mut_bounty_gold_min_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.bounty_gold_min
    }

    // optional uint32 bounty_gold_max = 62;

    pub fn clear_bounty_gold_max(&mut self) {
        self.bounty_gold_max = ::std::option::Option::None;
    }

    pub fn has_bounty_gold_max(&self) -> bool {
        self.bounty_gold_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bounty_gold_max(&mut self, v: u32) {
        self.bounty_gold_max = ::std::option::Option::Some(v);
    }

    pub fn get_bounty_gold_max(&self) -> u32 {
        self.bounty_gold_max.unwrap_or(0)
    }

    fn get_bounty_gold_max_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.bounty_gold_max
    }

    fn mut_bounty_gold_max_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.bounty_gold_max
    }

    // optional bool is_channeling = 65;

    pub fn clear_is_channeling(&mut self) {
        self.is_channeling = ::std::option::Option::None;
    }

    pub fn has_is_channeling(&self) -> bool {
        self.is_channeling.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_channeling(&mut self, v: bool) {
        self.is_channeling = ::std::option::Option::Some(v);
    }

    pub fn get_is_channeling(&self) -> bool {
        self.is_channeling.unwrap_or(false)
    }

    fn get_is_channeling_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_channeling
    }

    fn mut_is_channeling_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_channeling
    }

    // optional uint32 active_ability_handle = 66;

    pub fn clear_active_ability_handle(&mut self) {
        self.active_ability_handle = ::std::option::Option::None;
    }

    pub fn has_active_ability_handle(&self) -> bool {
        self.active_ability_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_active_ability_handle(&mut self, v: u32) {
        self.active_ability_handle = ::std::option::Option::Some(v);
    }

    pub fn get_active_ability_handle(&self) -> u32 {
        self.active_ability_handle.unwrap_or(0)
    }

    fn get_active_ability_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.active_ability_handle
    }

    fn mut_active_ability_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.active_ability_handle
    }

    // optional bool is_attack_immune = 70;

    pub fn clear_is_attack_immune(&mut self) {
        self.is_attack_immune = ::std::option::Option::None;
    }

    pub fn has_is_attack_immune(&self) -> bool {
        self.is_attack_immune.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_attack_immune(&mut self, v: bool) {
        self.is_attack_immune = ::std::option::Option::Some(v);
    }

    pub fn get_is_attack_immune(&self) -> bool {
        self.is_attack_immune.unwrap_or(false)
    }

    fn get_is_attack_immune_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_attack_immune
    }

    fn mut_is_attack_immune_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_attack_immune
    }

    // optional bool is_blind = 71;

    pub fn clear_is_blind(&mut self) {
        self.is_blind = ::std::option::Option::None;
    }

    pub fn has_is_blind(&self) -> bool {
        self.is_blind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_blind(&mut self, v: bool) {
        self.is_blind = ::std::option::Option::Some(v);
    }

    pub fn get_is_blind(&self) -> bool {
        self.is_blind.unwrap_or(false)
    }

    fn get_is_blind_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_blind
    }

    fn mut_is_blind_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_blind
    }

    // optional bool is_block_disabled = 72;

    pub fn clear_is_block_disabled(&mut self) {
        self.is_block_disabled = ::std::option::Option::None;
    }

    pub fn has_is_block_disabled(&self) -> bool {
        self.is_block_disabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_block_disabled(&mut self, v: bool) {
        self.is_block_disabled = ::std::option::Option::Some(v);
    }

    pub fn get_is_block_disabled(&self) -> bool {
        self.is_block_disabled.unwrap_or(false)
    }

    fn get_is_block_disabled_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_block_disabled
    }

    fn mut_is_block_disabled_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_block_disabled
    }

    // optional bool is_disarmed = 73;

    pub fn clear_is_disarmed(&mut self) {
        self.is_disarmed = ::std::option::Option::None;
    }

    pub fn has_is_disarmed(&self) -> bool {
        self.is_disarmed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_disarmed(&mut self, v: bool) {
        self.is_disarmed = ::std::option::Option::Some(v);
    }

    pub fn get_is_disarmed(&self) -> bool {
        self.is_disarmed.unwrap_or(false)
    }

    fn get_is_disarmed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_disarmed
    }

    fn mut_is_disarmed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_disarmed
    }

    // optional bool is_dominated = 74;

    pub fn clear_is_dominated(&mut self) {
        self.is_dominated = ::std::option::Option::None;
    }

    pub fn has_is_dominated(&self) -> bool {
        self.is_dominated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_dominated(&mut self, v: bool) {
        self.is_dominated = ::std::option::Option::Some(v);
    }

    pub fn get_is_dominated(&self) -> bool {
        self.is_dominated.unwrap_or(false)
    }

    fn get_is_dominated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_dominated
    }

    fn mut_is_dominated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_dominated
    }

    // optional bool is_evade_disabled = 75;

    pub fn clear_is_evade_disabled(&mut self) {
        self.is_evade_disabled = ::std::option::Option::None;
    }

    pub fn has_is_evade_disabled(&self) -> bool {
        self.is_evade_disabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_evade_disabled(&mut self, v: bool) {
        self.is_evade_disabled = ::std::option::Option::Some(v);
    }

    pub fn get_is_evade_disabled(&self) -> bool {
        self.is_evade_disabled.unwrap_or(false)
    }

    fn get_is_evade_disabled_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_evade_disabled
    }

    fn mut_is_evade_disabled_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_evade_disabled
    }

    // optional bool is_hexed = 76;

    pub fn clear_is_hexed(&mut self) {
        self.is_hexed = ::std::option::Option::None;
    }

    pub fn has_is_hexed(&self) -> bool {
        self.is_hexed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_hexed(&mut self, v: bool) {
        self.is_hexed = ::std::option::Option::Some(v);
    }

    pub fn get_is_hexed(&self) -> bool {
        self.is_hexed.unwrap_or(false)
    }

    fn get_is_hexed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_hexed
    }

    fn mut_is_hexed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_hexed
    }

    // optional bool is_invisible = 77;

    pub fn clear_is_invisible(&mut self) {
        self.is_invisible = ::std::option::Option::None;
    }

    pub fn has_is_invisible(&self) -> bool {
        self.is_invisible.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_invisible(&mut self, v: bool) {
        self.is_invisible = ::std::option::Option::Some(v);
    }

    pub fn get_is_invisible(&self) -> bool {
        self.is_invisible.unwrap_or(false)
    }

    fn get_is_invisible_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_invisible
    }

    fn mut_is_invisible_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_invisible
    }

    // optional bool is_invulnerable = 78;

    pub fn clear_is_invulnerable(&mut self) {
        self.is_invulnerable = ::std::option::Option::None;
    }

    pub fn has_is_invulnerable(&self) -> bool {
        self.is_invulnerable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_invulnerable(&mut self, v: bool) {
        self.is_invulnerable = ::std::option::Option::Some(v);
    }

    pub fn get_is_invulnerable(&self) -> bool {
        self.is_invulnerable.unwrap_or(false)
    }

    fn get_is_invulnerable_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_invulnerable
    }

    fn mut_is_invulnerable_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_invulnerable
    }

    // optional bool is_magic_immune = 79;

    pub fn clear_is_magic_immune(&mut self) {
        self.is_magic_immune = ::std::option::Option::None;
    }

    pub fn has_is_magic_immune(&self) -> bool {
        self.is_magic_immune.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_magic_immune(&mut self, v: bool) {
        self.is_magic_immune = ::std::option::Option::Some(v);
    }

    pub fn get_is_magic_immune(&self) -> bool {
        self.is_magic_immune.unwrap_or(false)
    }

    fn get_is_magic_immune_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_magic_immune
    }

    fn mut_is_magic_immune_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_magic_immune
    }

    // optional bool is_muted = 80;

    pub fn clear_is_muted(&mut self) {
        self.is_muted = ::std::option::Option::None;
    }

    pub fn has_is_muted(&self) -> bool {
        self.is_muted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_muted(&mut self, v: bool) {
        self.is_muted = ::std::option::Option::Some(v);
    }

    pub fn get_is_muted(&self) -> bool {
        self.is_muted.unwrap_or(false)
    }

    fn get_is_muted_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_muted
    }

    fn mut_is_muted_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_muted
    }

    // optional bool is_nightmared = 82;

    pub fn clear_is_nightmared(&mut self) {
        self.is_nightmared = ::std::option::Option::None;
    }

    pub fn has_is_nightmared(&self) -> bool {
        self.is_nightmared.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_nightmared(&mut self, v: bool) {
        self.is_nightmared = ::std::option::Option::Some(v);
    }

    pub fn get_is_nightmared(&self) -> bool {
        self.is_nightmared.unwrap_or(false)
    }

    fn get_is_nightmared_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_nightmared
    }

    fn mut_is_nightmared_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_nightmared
    }

    // optional bool is_rooted = 83;

    pub fn clear_is_rooted(&mut self) {
        self.is_rooted = ::std::option::Option::None;
    }

    pub fn has_is_rooted(&self) -> bool {
        self.is_rooted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_rooted(&mut self, v: bool) {
        self.is_rooted = ::std::option::Option::Some(v);
    }

    pub fn get_is_rooted(&self) -> bool {
        self.is_rooted.unwrap_or(false)
    }

    fn get_is_rooted_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_rooted
    }

    fn mut_is_rooted_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_rooted
    }

    // optional bool is_silenced = 84;

    pub fn clear_is_silenced(&mut self) {
        self.is_silenced = ::std::option::Option::None;
    }

    pub fn has_is_silenced(&self) -> bool {
        self.is_silenced.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_silenced(&mut self, v: bool) {
        self.is_silenced = ::std::option::Option::Some(v);
    }

    pub fn get_is_silenced(&self) -> bool {
        self.is_silenced.unwrap_or(false)
    }

    fn get_is_silenced_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_silenced
    }

    fn mut_is_silenced_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_silenced
    }

    // optional bool is_specially_deniable = 85;

    pub fn clear_is_specially_deniable(&mut self) {
        self.is_specially_deniable = ::std::option::Option::None;
    }

    pub fn has_is_specially_deniable(&self) -> bool {
        self.is_specially_deniable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_specially_deniable(&mut self, v: bool) {
        self.is_specially_deniable = ::std::option::Option::Some(v);
    }

    pub fn get_is_specially_deniable(&self) -> bool {
        self.is_specially_deniable.unwrap_or(false)
    }

    fn get_is_specially_deniable_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_specially_deniable
    }

    fn mut_is_specially_deniable_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_specially_deniable
    }

    // optional bool is_stunned = 86;

    pub fn clear_is_stunned(&mut self) {
        self.is_stunned = ::std::option::Option::None;
    }

    pub fn has_is_stunned(&self) -> bool {
        self.is_stunned.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_stunned(&mut self, v: bool) {
        self.is_stunned = ::std::option::Option::Some(v);
    }

    pub fn get_is_stunned(&self) -> bool {
        self.is_stunned.unwrap_or(false)
    }

    fn get_is_stunned_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_stunned
    }

    fn mut_is_stunned_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_stunned
    }

    // optional bool is_unable_to_miss = 87;

    pub fn clear_is_unable_to_miss(&mut self) {
        self.is_unable_to_miss = ::std::option::Option::None;
    }

    pub fn has_is_unable_to_miss(&self) -> bool {
        self.is_unable_to_miss.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_unable_to_miss(&mut self, v: bool) {
        self.is_unable_to_miss = ::std::option::Option::Some(v);
    }

    pub fn get_is_unable_to_miss(&self) -> bool {
        self.is_unable_to_miss.unwrap_or(false)
    }

    fn get_is_unable_to_miss_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_unable_to_miss
    }

    fn mut_is_unable_to_miss_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_unable_to_miss
    }

    // optional bool has_scepter = 88;

    pub fn clear_has_scepter(&mut self) {
        self.has_scepter = ::std::option::Option::None;
    }

    pub fn has_has_scepter(&self) -> bool {
        self.has_scepter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_has_scepter(&mut self, v: bool) {
        self.has_scepter = ::std::option::Option::Some(v);
    }

    pub fn get_has_scepter(&self) -> bool {
        self.has_scepter.unwrap_or(false)
    }

    fn get_has_scepter_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.has_scepter
    }

    fn mut_has_scepter_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.has_scepter
    }

    // repeated .CMsgBotWorldState.Ability abilities = 90;

    pub fn clear_abilities(&mut self) {
        self.abilities.clear();
    }

    // Param is passed by value, moved
    pub fn set_abilities(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_Ability>) {
        self.abilities = v;
    }

    // Mutable pointer to the field.
    pub fn mut_abilities(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_Ability> {
        &mut self.abilities
    }

    // Take field
    pub fn take_abilities(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_Ability> {
        ::std::mem::replace(&mut self.abilities, ::protobuf::RepeatedField::new())
    }

    pub fn get_abilities(&self) -> &[CMsgBotWorldState_Ability] {
        &self.abilities
    }

    fn get_abilities_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_Ability> {
        &self.abilities
    }

    fn mut_abilities_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_Ability> {
        &mut self.abilities
    }

    // repeated .CMsgBotWorldState.Ability items = 91;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_Ability>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_Ability> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_Ability> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[CMsgBotWorldState_Ability] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_Ability> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_Ability> {
        &mut self.items
    }

    // repeated .CMsgBotWorldState.Modifier modifiers = 92;

    pub fn clear_modifiers(&mut self) {
        self.modifiers.clear();
    }

    // Param is passed by value, moved
    pub fn set_modifiers(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_Modifier>) {
        self.modifiers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_modifiers(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_Modifier> {
        &mut self.modifiers
    }

    // Take field
    pub fn take_modifiers(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_Modifier> {
        ::std::mem::replace(&mut self.modifiers, ::protobuf::RepeatedField::new())
    }

    pub fn get_modifiers(&self) -> &[CMsgBotWorldState_Modifier] {
        &self.modifiers
    }

    fn get_modifiers_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_Modifier> {
        &self.modifiers
    }

    fn mut_modifiers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_Modifier> {
        &mut self.modifiers
    }

    // repeated .CMsgBotWorldState.TrackingProjectile incoming_tracking_projectiles = 93;

    pub fn clear_incoming_tracking_projectiles(&mut self) {
        self.incoming_tracking_projectiles.clear();
    }

    // Param is passed by value, moved
    pub fn set_incoming_tracking_projectiles(&mut self, v: ::protobuf::RepeatedField<CMsgBotWorldState_TrackingProjectile>) {
        self.incoming_tracking_projectiles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_incoming_tracking_projectiles(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_TrackingProjectile> {
        &mut self.incoming_tracking_projectiles
    }

    // Take field
    pub fn take_incoming_tracking_projectiles(&mut self) -> ::protobuf::RepeatedField<CMsgBotWorldState_TrackingProjectile> {
        ::std::mem::replace(&mut self.incoming_tracking_projectiles, ::protobuf::RepeatedField::new())
    }

    pub fn get_incoming_tracking_projectiles(&self) -> &[CMsgBotWorldState_TrackingProjectile] {
        &self.incoming_tracking_projectiles
    }

    fn get_incoming_tracking_projectiles_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgBotWorldState_TrackingProjectile> {
        &self.incoming_tracking_projectiles
    }

    fn mut_incoming_tracking_projectiles_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgBotWorldState_TrackingProjectile> {
        &mut self.incoming_tracking_projectiles
    }

    // optional uint32 action_type = 100;

    pub fn clear_action_type(&mut self) {
        self.action_type = ::std::option::Option::None;
    }

    pub fn has_action_type(&self) -> bool {
        self.action_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action_type(&mut self, v: u32) {
        self.action_type = ::std::option::Option::Some(v);
    }

    pub fn get_action_type(&self) -> u32 {
        self.action_type.unwrap_or(0)
    }

    fn get_action_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.action_type
    }

    fn mut_action_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.action_type
    }

    // optional uint32 ability_target_handle = 101;

    pub fn clear_ability_target_handle(&mut self) {
        self.ability_target_handle = ::std::option::Option::None;
    }

    pub fn has_ability_target_handle(&self) -> bool {
        self.ability_target_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_target_handle(&mut self, v: u32) {
        self.ability_target_handle = ::std::option::Option::Some(v);
    }

    pub fn get_ability_target_handle(&self) -> u32 {
        self.ability_target_handle.unwrap_or(0)
    }

    fn get_ability_target_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_target_handle
    }

    fn mut_ability_target_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_target_handle
    }

    // optional uint32 primary_attribute = 110;

    pub fn clear_primary_attribute(&mut self) {
        self.primary_attribute = ::std::option::Option::None;
    }

    pub fn has_primary_attribute(&self) -> bool {
        self.primary_attribute.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary_attribute(&mut self, v: u32) {
        self.primary_attribute = ::std::option::Option::Some(v);
    }

    pub fn get_primary_attribute(&self) -> u32 {
        self.primary_attribute.unwrap_or(0)
    }

    fn get_primary_attribute_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.primary_attribute
    }

    fn mut_primary_attribute_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.primary_attribute
    }

    // optional bool is_illusion = 111;

    pub fn clear_is_illusion(&mut self) {
        self.is_illusion = ::std::option::Option::None;
    }

    pub fn has_is_illusion(&self) -> bool {
        self.is_illusion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_illusion(&mut self, v: bool) {
        self.is_illusion = ::std::option::Option::Some(v);
    }

    pub fn get_is_illusion(&self) -> bool {
        self.is_illusion.unwrap_or(false)
    }

    fn get_is_illusion_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_illusion
    }

    fn mut_is_illusion_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_illusion
    }

    // optional uint32 respawn_time = 112;

    pub fn clear_respawn_time(&mut self) {
        self.respawn_time = ::std::option::Option::None;
    }

    pub fn has_respawn_time(&self) -> bool {
        self.respawn_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_respawn_time(&mut self, v: u32) {
        self.respawn_time = ::std::option::Option::Some(v);
    }

    pub fn get_respawn_time(&self) -> u32 {
        self.respawn_time.unwrap_or(0)
    }

    fn get_respawn_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.respawn_time
    }

    fn mut_respawn_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.respawn_time
    }

    // optional uint32 buyback_cost = 113;

    pub fn clear_buyback_cost(&mut self) {
        self.buyback_cost = ::std::option::Option::None;
    }

    pub fn has_buyback_cost(&self) -> bool {
        self.buyback_cost.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buyback_cost(&mut self, v: u32) {
        self.buyback_cost = ::std::option::Option::Some(v);
    }

    pub fn get_buyback_cost(&self) -> u32 {
        self.buyback_cost.unwrap_or(0)
    }

    fn get_buyback_cost_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.buyback_cost
    }

    fn mut_buyback_cost_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.buyback_cost
    }

    // optional uint32 buyback_cooldown = 114;

    pub fn clear_buyback_cooldown(&mut self) {
        self.buyback_cooldown = ::std::option::Option::None;
    }

    pub fn has_buyback_cooldown(&self) -> bool {
        self.buyback_cooldown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buyback_cooldown(&mut self, v: u32) {
        self.buyback_cooldown = ::std::option::Option::Some(v);
    }

    pub fn get_buyback_cooldown(&self) -> u32 {
        self.buyback_cooldown.unwrap_or(0)
    }

    fn get_buyback_cooldown_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.buyback_cooldown
    }

    fn mut_buyback_cooldown_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.buyback_cooldown
    }

    // optional float spell_amplifiction = 115;

    pub fn clear_spell_amplifiction(&mut self) {
        self.spell_amplifiction = ::std::option::Option::None;
    }

    pub fn has_spell_amplifiction(&self) -> bool {
        self.spell_amplifiction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spell_amplifiction(&mut self, v: f32) {
        self.spell_amplifiction = ::std::option::Option::Some(v);
    }

    pub fn get_spell_amplifiction(&self) -> f32 {
        self.spell_amplifiction.unwrap_or(0.)
    }

    fn get_spell_amplifiction_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.spell_amplifiction
    }

    fn mut_spell_amplifiction_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.spell_amplifiction
    }

    // optional uint32 armor = 116;

    pub fn clear_armor(&mut self) {
        self.armor = ::std::option::Option::None;
    }

    pub fn has_armor(&self) -> bool {
        self.armor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_armor(&mut self, v: u32) {
        self.armor = ::std::option::Option::Some(v);
    }

    pub fn get_armor(&self) -> u32 {
        self.armor.unwrap_or(0)
    }

    fn get_armor_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.armor
    }

    fn mut_armor_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.armor
    }

    // optional float magic_resist = 117;

    pub fn clear_magic_resist(&mut self) {
        self.magic_resist = ::std::option::Option::None;
    }

    pub fn has_magic_resist(&self) -> bool {
        self.magic_resist.is_some()
    }

    // Param is passed by value, moved
    pub fn set_magic_resist(&mut self, v: f32) {
        self.magic_resist = ::std::option::Option::Some(v);
    }

    pub fn get_magic_resist(&self) -> f32 {
        self.magic_resist.unwrap_or(0.)
    }

    fn get_magic_resist_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.magic_resist
    }

    fn mut_magic_resist_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.magic_resist
    }

    // optional float evasion = 118;

    pub fn clear_evasion(&mut self) {
        self.evasion = ::std::option::Option::None;
    }

    pub fn has_evasion(&self) -> bool {
        self.evasion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_evasion(&mut self, v: f32) {
        self.evasion = ::std::option::Option::Some(v);
    }

    pub fn get_evasion(&self) -> f32 {
        self.evasion.unwrap_or(0.)
    }

    fn get_evasion_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.evasion
    }

    fn mut_evasion_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.evasion
    }

    // optional uint32 xp_needed_to_level = 120;

    pub fn clear_xp_needed_to_level(&mut self) {
        self.xp_needed_to_level = ::std::option::Option::None;
    }

    pub fn has_xp_needed_to_level(&self) -> bool {
        self.xp_needed_to_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xp_needed_to_level(&mut self, v: u32) {
        self.xp_needed_to_level = ::std::option::Option::Some(v);
    }

    pub fn get_xp_needed_to_level(&self) -> u32 {
        self.xp_needed_to_level.unwrap_or(0)
    }

    fn get_xp_needed_to_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.xp_needed_to_level
    }

    fn mut_xp_needed_to_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.xp_needed_to_level
    }

    // optional uint32 ability_points = 121;

    pub fn clear_ability_points(&mut self) {
        self.ability_points = ::std::option::Option::None;
    }

    pub fn has_ability_points(&self) -> bool {
        self.ability_points.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_points(&mut self, v: u32) {
        self.ability_points = ::std::option::Option::Some(v);
    }

    pub fn get_ability_points(&self) -> u32 {
        self.ability_points.unwrap_or(0)
    }

    fn get_ability_points_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_points
    }

    fn mut_ability_points_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_points
    }

    // optional uint32 reliable_gold = 122;

    pub fn clear_reliable_gold(&mut self) {
        self.reliable_gold = ::std::option::Option::None;
    }

    pub fn has_reliable_gold(&self) -> bool {
        self.reliable_gold.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reliable_gold(&mut self, v: u32) {
        self.reliable_gold = ::std::option::Option::Some(v);
    }

    pub fn get_reliable_gold(&self) -> u32 {
        self.reliable_gold.unwrap_or(0)
    }

    fn get_reliable_gold_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.reliable_gold
    }

    fn mut_reliable_gold_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.reliable_gold
    }

    // optional uint32 unreliable_gold = 123;

    pub fn clear_unreliable_gold(&mut self) {
        self.unreliable_gold = ::std::option::Option::None;
    }

    pub fn has_unreliable_gold(&self) -> bool {
        self.unreliable_gold.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unreliable_gold(&mut self, v: u32) {
        self.unreliable_gold = ::std::option::Option::Some(v);
    }

    pub fn get_unreliable_gold(&self) -> u32 {
        self.unreliable_gold.unwrap_or(0)
    }

    fn get_unreliable_gold_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.unreliable_gold
    }

    fn mut_unreliable_gold_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.unreliable_gold
    }

    // optional uint32 last_hits = 124;

    pub fn clear_last_hits(&mut self) {
        self.last_hits = ::std::option::Option::None;
    }

    pub fn has_last_hits(&self) -> bool {
        self.last_hits.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_hits(&mut self, v: u32) {
        self.last_hits = ::std::option::Option::Some(v);
    }

    pub fn get_last_hits(&self) -> u32 {
        self.last_hits.unwrap_or(0)
    }

    fn get_last_hits_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.last_hits
    }

    fn mut_last_hits_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.last_hits
    }

    // optional uint32 denies = 125;

    pub fn clear_denies(&mut self) {
        self.denies = ::std::option::Option::None;
    }

    pub fn has_denies(&self) -> bool {
        self.denies.is_some()
    }

    // Param is passed by value, moved
    pub fn set_denies(&mut self, v: u32) {
        self.denies = ::std::option::Option::Some(v);
    }

    pub fn get_denies(&self) -> u32 {
        self.denies.unwrap_or(0)
    }

    fn get_denies_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.denies
    }

    fn mut_denies_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.denies
    }

    // optional uint32 net_worth = 126;

    pub fn clear_net_worth(&mut self) {
        self.net_worth = ::std::option::Option::None;
    }

    pub fn has_net_worth(&self) -> bool {
        self.net_worth.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_worth(&mut self, v: u32) {
        self.net_worth = ::std::option::Option::Some(v);
    }

    pub fn get_net_worth(&self) -> u32 {
        self.net_worth.unwrap_or(0)
    }

    fn get_net_worth_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.net_worth
    }

    fn mut_net_worth_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.net_worth
    }

    // optional float remaining_lifespan = 130;

    pub fn clear_remaining_lifespan(&mut self) {
        self.remaining_lifespan = ::std::option::Option::None;
    }

    pub fn has_remaining_lifespan(&self) -> bool {
        self.remaining_lifespan.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remaining_lifespan(&mut self, v: f32) {
        self.remaining_lifespan = ::std::option::Option::Some(v);
    }

    pub fn get_remaining_lifespan(&self) -> f32 {
        self.remaining_lifespan.unwrap_or(0.)
    }

    fn get_remaining_lifespan_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.remaining_lifespan
    }

    fn mut_remaining_lifespan_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.remaining_lifespan
    }

    // optional bool flying_courier = 140;

    pub fn clear_flying_courier(&mut self) {
        self.flying_courier = ::std::option::Option::None;
    }

    pub fn has_flying_courier(&self) -> bool {
        self.flying_courier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flying_courier(&mut self, v: bool) {
        self.flying_courier = ::std::option::Option::Some(v);
    }

    pub fn get_flying_courier(&self) -> bool {
        self.flying_courier.unwrap_or(false)
    }

    fn get_flying_courier_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.flying_courier
    }

    fn mut_flying_courier_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.flying_courier
    }

    // optional float shrine_cooldown = 150;

    pub fn clear_shrine_cooldown(&mut self) {
        self.shrine_cooldown = ::std::option::Option::None;
    }

    pub fn has_shrine_cooldown(&self) -> bool {
        self.shrine_cooldown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shrine_cooldown(&mut self, v: f32) {
        self.shrine_cooldown = ::std::option::Option::Some(v);
    }

    pub fn get_shrine_cooldown(&self) -> f32 {
        self.shrine_cooldown.unwrap_or(0.)
    }

    fn get_shrine_cooldown_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.shrine_cooldown
    }

    fn mut_shrine_cooldown_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.shrine_cooldown
    }

    // optional bool is_shrine_healing = 151;

    pub fn clear_is_shrine_healing(&mut self) {
        self.is_shrine_healing = ::std::option::Option::None;
    }

    pub fn has_is_shrine_healing(&self) -> bool {
        self.is_shrine_healing.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_shrine_healing(&mut self, v: bool) {
        self.is_shrine_healing = ::std::option::Option::Some(v);
    }

    pub fn get_is_shrine_healing(&self) -> bool {
        self.is_shrine_healing.unwrap_or(false)
    }

    fn get_is_shrine_healing_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_shrine_healing
    }

    fn mut_is_shrine_healing_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_shrine_healing
    }
}

impl ::protobuf::Message for CMsgBotWorldState_Unit {
    fn is_initialized(&self) -> bool {
        for v in &self.location {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.abilities {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.items {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.modifiers {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.incoming_tracking_projectiles {
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
                    let tmp = is.read_uint32()?;
                    self.handle = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.unit_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.level = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_alive = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.bounding_radius = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.facing = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ground_height = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.vision_range_daytime = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.vision_range_nighttime = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.health = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.health_max = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.health_regen = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.mana = ::std::option::Option::Some(tmp);
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.mana_max = ::std::option::Option::Some(tmp);
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.mana_regen = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.base_movement_speed = ::std::option::Option::Some(tmp);
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.current_movement_speed = ::std::option::Option::Some(tmp);
                },
                35 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.anim_activity = ::std::option::Option::Some(tmp);
                },
                36 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.anim_cycle = ::std::option::Option::Some(tmp);
                },
                40 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.base_damage = ::std::option::Option::Some(tmp);
                },
                41 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.base_damage_variance = ::std::option::Option::Some(tmp);
                },
                42 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.bonus_damage = ::std::option::Option::Some(tmp);
                },
                43 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.attack_damage = ::std::option::Option::Some(tmp);
                },
                44 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.attack_range = ::std::option::Option::Some(tmp);
                },
                45 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.attack_speed = ::std::option::Option::Some(tmp);
                },
                46 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.attack_anim_point = ::std::option::Option::Some(tmp);
                },
                47 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.attack_acquisition_range = ::std::option::Option::Some(tmp);
                },
                48 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.attack_projectile_speed = ::std::option::Option::Some(tmp);
                },
                49 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.attack_target_handle = ::std::option::Option::Some(tmp);
                },
                60 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.bounty_xp = ::std::option::Option::Some(tmp);
                },
                61 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.bounty_gold_min = ::std::option::Option::Some(tmp);
                },
                62 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.bounty_gold_max = ::std::option::Option::Some(tmp);
                },
                65 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_channeling = ::std::option::Option::Some(tmp);
                },
                66 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.active_ability_handle = ::std::option::Option::Some(tmp);
                },
                70 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_attack_immune = ::std::option::Option::Some(tmp);
                },
                71 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_blind = ::std::option::Option::Some(tmp);
                },
                72 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_block_disabled = ::std::option::Option::Some(tmp);
                },
                73 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_disarmed = ::std::option::Option::Some(tmp);
                },
                74 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_dominated = ::std::option::Option::Some(tmp);
                },
                75 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_evade_disabled = ::std::option::Option::Some(tmp);
                },
                76 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_hexed = ::std::option::Option::Some(tmp);
                },
                77 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_invisible = ::std::option::Option::Some(tmp);
                },
                78 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_invulnerable = ::std::option::Option::Some(tmp);
                },
                79 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_magic_immune = ::std::option::Option::Some(tmp);
                },
                80 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_muted = ::std::option::Option::Some(tmp);
                },
                82 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_nightmared = ::std::option::Option::Some(tmp);
                },
                83 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_rooted = ::std::option::Option::Some(tmp);
                },
                84 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_silenced = ::std::option::Option::Some(tmp);
                },
                85 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_specially_deniable = ::std::option::Option::Some(tmp);
                },
                86 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_stunned = ::std::option::Option::Some(tmp);
                },
                87 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_unable_to_miss = ::std::option::Option::Some(tmp);
                },
                88 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.has_scepter = ::std::option::Option::Some(tmp);
                },
                90 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.abilities)?;
                },
                91 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
                },
                92 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.modifiers)?;
                },
                93 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.incoming_tracking_projectiles)?;
                },
                100 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.action_type = ::std::option::Option::Some(tmp);
                },
                101 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ability_target_handle = ::std::option::Option::Some(tmp);
                },
                110 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.primary_attribute = ::std::option::Option::Some(tmp);
                },
                111 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_illusion = ::std::option::Option::Some(tmp);
                },
                112 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.respawn_time = ::std::option::Option::Some(tmp);
                },
                113 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.buyback_cost = ::std::option::Option::Some(tmp);
                },
                114 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.buyback_cooldown = ::std::option::Option::Some(tmp);
                },
                115 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.spell_amplifiction = ::std::option::Option::Some(tmp);
                },
                116 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.armor = ::std::option::Option::Some(tmp);
                },
                117 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.magic_resist = ::std::option::Option::Some(tmp);
                },
                118 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.evasion = ::std::option::Option::Some(tmp);
                },
                120 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.xp_needed_to_level = ::std::option::Option::Some(tmp);
                },
                121 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ability_points = ::std::option::Option::Some(tmp);
                },
                122 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.reliable_gold = ::std::option::Option::Some(tmp);
                },
                123 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.unreliable_gold = ::std::option::Option::Some(tmp);
                },
                124 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.last_hits = ::std::option::Option::Some(tmp);
                },
                125 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.denies = ::std::option::Option::Some(tmp);
                },
                126 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.net_worth = ::std::option::Option::Some(tmp);
                },
                130 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.remaining_lifespan = ::std::option::Option::Some(tmp);
                },
                140 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.flying_courier = ::std::option::Option::Some(tmp);
                },
                150 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.shrine_cooldown = ::std::option::Option::Some(tmp);
                },
                151 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_shrine_healing = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.handle {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.unit_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.level {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.is_alive {
            my_size += 2;
        }
        if let Some(v) = self.bounding_radius {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.facing {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ground_height {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.vision_range_daytime {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.vision_range_nighttime {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.health {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.health_max {
            my_size += ::protobuf::rt::value_size(21, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.health_regen {
            my_size += 6;
        }
        if let Some(v) = self.mana {
            my_size += ::protobuf::rt::value_size(25, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.mana_max {
            my_size += ::protobuf::rt::value_size(26, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.mana_regen {
            my_size += 6;
        }
        if let Some(v) = self.base_movement_speed {
            my_size += ::protobuf::rt::value_size(30, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.current_movement_speed {
            my_size += ::protobuf::rt::value_size(31, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.anim_activity {
            my_size += ::protobuf::rt::value_size(35, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.anim_cycle {
            my_size += 6;
        }
        if let Some(v) = self.base_damage {
            my_size += ::protobuf::rt::value_size(40, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.base_damage_variance {
            my_size += ::protobuf::rt::value_size(41, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.bonus_damage {
            my_size += ::protobuf::rt::value_size(42, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.attack_damage {
            my_size += ::protobuf::rt::value_size(43, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.attack_range {
            my_size += ::protobuf::rt::value_size(44, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.attack_speed {
            my_size += ::protobuf::rt::value_size(45, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.attack_anim_point {
            my_size += 6;
        }
        if let Some(v) = self.attack_acquisition_range {
            my_size += ::protobuf::rt::value_size(47, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.attack_projectile_speed {
            my_size += ::protobuf::rt::value_size(48, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.attack_target_handle {
            my_size += ::protobuf::rt::value_size(49, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.bounty_xp {
            my_size += ::protobuf::rt::value_size(60, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.bounty_gold_min {
            my_size += ::protobuf::rt::value_size(61, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.bounty_gold_max {
            my_size += ::protobuf::rt::value_size(62, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_channeling {
            my_size += 3;
        }
        if let Some(v) = self.active_ability_handle {
            my_size += ::protobuf::rt::value_size(66, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_attack_immune {
            my_size += 3;
        }
        if let Some(v) = self.is_blind {
            my_size += 3;
        }
        if let Some(v) = self.is_block_disabled {
            my_size += 3;
        }
        if let Some(v) = self.is_disarmed {
            my_size += 3;
        }
        if let Some(v) = self.is_dominated {
            my_size += 3;
        }
        if let Some(v) = self.is_evade_disabled {
            my_size += 3;
        }
        if let Some(v) = self.is_hexed {
            my_size += 3;
        }
        if let Some(v) = self.is_invisible {
            my_size += 3;
        }
        if let Some(v) = self.is_invulnerable {
            my_size += 3;
        }
        if let Some(v) = self.is_magic_immune {
            my_size += 3;
        }
        if let Some(v) = self.is_muted {
            my_size += 3;
        }
        if let Some(v) = self.is_nightmared {
            my_size += 3;
        }
        if let Some(v) = self.is_rooted {
            my_size += 3;
        }
        if let Some(v) = self.is_silenced {
            my_size += 3;
        }
        if let Some(v) = self.is_specially_deniable {
            my_size += 3;
        }
        if let Some(v) = self.is_stunned {
            my_size += 3;
        }
        if let Some(v) = self.is_unable_to_miss {
            my_size += 3;
        }
        if let Some(v) = self.has_scepter {
            my_size += 3;
        }
        for value in &self.abilities {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.items {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.modifiers {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.incoming_tracking_projectiles {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.action_type {
            my_size += ::protobuf::rt::value_size(100, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ability_target_handle {
            my_size += ::protobuf::rt::value_size(101, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.primary_attribute {
            my_size += ::protobuf::rt::value_size(110, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_illusion {
            my_size += 3;
        }
        if let Some(v) = self.respawn_time {
            my_size += ::protobuf::rt::value_size(112, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.buyback_cost {
            my_size += ::protobuf::rt::value_size(113, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.buyback_cooldown {
            my_size += ::protobuf::rt::value_size(114, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.spell_amplifiction {
            my_size += 6;
        }
        if let Some(v) = self.armor {
            my_size += ::protobuf::rt::value_size(116, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.magic_resist {
            my_size += 6;
        }
        if let Some(v) = self.evasion {
            my_size += 6;
        }
        if let Some(v) = self.xp_needed_to_level {
            my_size += ::protobuf::rt::value_size(120, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ability_points {
            my_size += ::protobuf::rt::value_size(121, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.reliable_gold {
            my_size += ::protobuf::rt::value_size(122, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.unreliable_gold {
            my_size += ::protobuf::rt::value_size(123, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.last_hits {
            my_size += ::protobuf::rt::value_size(124, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.denies {
            my_size += ::protobuf::rt::value_size(125, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.net_worth {
            my_size += ::protobuf::rt::value_size(126, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.remaining_lifespan {
            my_size += 6;
        }
        if let Some(v) = self.flying_courier {
            my_size += 3;
        }
        if let Some(v) = self.shrine_cooldown {
            my_size += 6;
        }
        if let Some(v) = self.is_shrine_healing {
            my_size += 3;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.handle {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.unit_type {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.team_id {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.level {
            os.write_uint32(5, v)?;
        }
        if let Some(ref v) = self.location.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.is_alive {
            os.write_bool(7, v)?;
        }
        if let Some(v) = self.bounding_radius {
            os.write_uint32(10, v)?;
        }
        if let Some(v) = self.facing {
            os.write_uint32(11, v)?;
        }
        if let Some(v) = self.ground_height {
            os.write_uint32(12, v)?;
        }
        if let Some(v) = self.vision_range_daytime {
            os.write_uint32(15, v)?;
        }
        if let Some(v) = self.vision_range_nighttime {
            os.write_uint32(16, v)?;
        }
        if let Some(v) = self.health {
            os.write_uint32(20, v)?;
        }
        if let Some(v) = self.health_max {
            os.write_uint32(21, v)?;
        }
        if let Some(v) = self.health_regen {
            os.write_float(22, v)?;
        }
        if let Some(v) = self.mana {
            os.write_uint32(25, v)?;
        }
        if let Some(v) = self.mana_max {
            os.write_uint32(26, v)?;
        }
        if let Some(v) = self.mana_regen {
            os.write_float(27, v)?;
        }
        if let Some(v) = self.base_movement_speed {
            os.write_uint32(30, v)?;
        }
        if let Some(v) = self.current_movement_speed {
            os.write_uint32(31, v)?;
        }
        if let Some(v) = self.anim_activity {
            os.write_int32(35, v)?;
        }
        if let Some(v) = self.anim_cycle {
            os.write_float(36, v)?;
        }
        if let Some(v) = self.base_damage {
            os.write_uint32(40, v)?;
        }
        if let Some(v) = self.base_damage_variance {
            os.write_uint32(41, v)?;
        }
        if let Some(v) = self.bonus_damage {
            os.write_uint32(42, v)?;
        }
        if let Some(v) = self.attack_damage {
            os.write_uint32(43, v)?;
        }
        if let Some(v) = self.attack_range {
            os.write_uint32(44, v)?;
        }
        if let Some(v) = self.attack_speed {
            os.write_uint32(45, v)?;
        }
        if let Some(v) = self.attack_anim_point {
            os.write_float(46, v)?;
        }
        if let Some(v) = self.attack_acquisition_range {
            os.write_uint32(47, v)?;
        }
        if let Some(v) = self.attack_projectile_speed {
            os.write_uint32(48, v)?;
        }
        if let Some(v) = self.attack_target_handle {
            os.write_uint32(49, v)?;
        }
        if let Some(v) = self.bounty_xp {
            os.write_uint32(60, v)?;
        }
        if let Some(v) = self.bounty_gold_min {
            os.write_uint32(61, v)?;
        }
        if let Some(v) = self.bounty_gold_max {
            os.write_uint32(62, v)?;
        }
        if let Some(v) = self.is_channeling {
            os.write_bool(65, v)?;
        }
        if let Some(v) = self.active_ability_handle {
            os.write_uint32(66, v)?;
        }
        if let Some(v) = self.is_attack_immune {
            os.write_bool(70, v)?;
        }
        if let Some(v) = self.is_blind {
            os.write_bool(71, v)?;
        }
        if let Some(v) = self.is_block_disabled {
            os.write_bool(72, v)?;
        }
        if let Some(v) = self.is_disarmed {
            os.write_bool(73, v)?;
        }
        if let Some(v) = self.is_dominated {
            os.write_bool(74, v)?;
        }
        if let Some(v) = self.is_evade_disabled {
            os.write_bool(75, v)?;
        }
        if let Some(v) = self.is_hexed {
            os.write_bool(76, v)?;
        }
        if let Some(v) = self.is_invisible {
            os.write_bool(77, v)?;
        }
        if let Some(v) = self.is_invulnerable {
            os.write_bool(78, v)?;
        }
        if let Some(v) = self.is_magic_immune {
            os.write_bool(79, v)?;
        }
        if let Some(v) = self.is_muted {
            os.write_bool(80, v)?;
        }
        if let Some(v) = self.is_nightmared {
            os.write_bool(82, v)?;
        }
        if let Some(v) = self.is_rooted {
            os.write_bool(83, v)?;
        }
        if let Some(v) = self.is_silenced {
            os.write_bool(84, v)?;
        }
        if let Some(v) = self.is_specially_deniable {
            os.write_bool(85, v)?;
        }
        if let Some(v) = self.is_stunned {
            os.write_bool(86, v)?;
        }
        if let Some(v) = self.is_unable_to_miss {
            os.write_bool(87, v)?;
        }
        if let Some(v) = self.has_scepter {
            os.write_bool(88, v)?;
        }
        for v in &self.abilities {
            os.write_tag(90, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.items {
            os.write_tag(91, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.modifiers {
            os.write_tag(92, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.incoming_tracking_projectiles {
            os.write_tag(93, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.action_type {
            os.write_uint32(100, v)?;
        }
        if let Some(v) = self.ability_target_handle {
            os.write_uint32(101, v)?;
        }
        if let Some(v) = self.primary_attribute {
            os.write_uint32(110, v)?;
        }
        if let Some(v) = self.is_illusion {
            os.write_bool(111, v)?;
        }
        if let Some(v) = self.respawn_time {
            os.write_uint32(112, v)?;
        }
        if let Some(v) = self.buyback_cost {
            os.write_uint32(113, v)?;
        }
        if let Some(v) = self.buyback_cooldown {
            os.write_uint32(114, v)?;
        }
        if let Some(v) = self.spell_amplifiction {
            os.write_float(115, v)?;
        }
        if let Some(v) = self.armor {
            os.write_uint32(116, v)?;
        }
        if let Some(v) = self.magic_resist {
            os.write_float(117, v)?;
        }
        if let Some(v) = self.evasion {
            os.write_float(118, v)?;
        }
        if let Some(v) = self.xp_needed_to_level {
            os.write_uint32(120, v)?;
        }
        if let Some(v) = self.ability_points {
            os.write_uint32(121, v)?;
        }
        if let Some(v) = self.reliable_gold {
            os.write_uint32(122, v)?;
        }
        if let Some(v) = self.unreliable_gold {
            os.write_uint32(123, v)?;
        }
        if let Some(v) = self.last_hits {
            os.write_uint32(124, v)?;
        }
        if let Some(v) = self.denies {
            os.write_uint32(125, v)?;
        }
        if let Some(v) = self.net_worth {
            os.write_uint32(126, v)?;
        }
        if let Some(v) = self.remaining_lifespan {
            os.write_float(130, v)?;
        }
        if let Some(v) = self.flying_courier {
            os.write_bool(140, v)?;
        }
        if let Some(v) = self.shrine_cooldown {
            os.write_float(150, v)?;
        }
        if let Some(v) = self.is_shrine_healing {
            os.write_bool(151, v)?;
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

impl ::protobuf::MessageStatic for CMsgBotWorldState_Unit {
    fn new() -> CMsgBotWorldState_Unit {
        CMsgBotWorldState_Unit::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotWorldState_Unit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "handle",
                    CMsgBotWorldState_Unit::get_handle_for_reflect,
                    CMsgBotWorldState_Unit::mut_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgBotWorldState_UnitType>>(
                    "unit_type",
                    CMsgBotWorldState_Unit::get_unit_type_for_reflect,
                    CMsgBotWorldState_Unit::mut_unit_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgBotWorldState_Unit::get_name_for_reflect,
                    CMsgBotWorldState_Unit::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgBotWorldState_Unit::get_team_id_for_reflect,
                    CMsgBotWorldState_Unit::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "level",
                    CMsgBotWorldState_Unit::get_level_for_reflect,
                    CMsgBotWorldState_Unit::mut_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_Vector>>(
                    "location",
                    CMsgBotWorldState_Unit::get_location_for_reflect,
                    CMsgBotWorldState_Unit::mut_location_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_alive",
                    CMsgBotWorldState_Unit::get_is_alive_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_alive_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bounding_radius",
                    CMsgBotWorldState_Unit::get_bounding_radius_for_reflect,
                    CMsgBotWorldState_Unit::mut_bounding_radius_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "facing",
                    CMsgBotWorldState_Unit::get_facing_for_reflect,
                    CMsgBotWorldState_Unit::mut_facing_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ground_height",
                    CMsgBotWorldState_Unit::get_ground_height_for_reflect,
                    CMsgBotWorldState_Unit::mut_ground_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "vision_range_daytime",
                    CMsgBotWorldState_Unit::get_vision_range_daytime_for_reflect,
                    CMsgBotWorldState_Unit::mut_vision_range_daytime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "vision_range_nighttime",
                    CMsgBotWorldState_Unit::get_vision_range_nighttime_for_reflect,
                    CMsgBotWorldState_Unit::mut_vision_range_nighttime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "health",
                    CMsgBotWorldState_Unit::get_health_for_reflect,
                    CMsgBotWorldState_Unit::mut_health_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "health_max",
                    CMsgBotWorldState_Unit::get_health_max_for_reflect,
                    CMsgBotWorldState_Unit::mut_health_max_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "health_regen",
                    CMsgBotWorldState_Unit::get_health_regen_for_reflect,
                    CMsgBotWorldState_Unit::mut_health_regen_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "mana",
                    CMsgBotWorldState_Unit::get_mana_for_reflect,
                    CMsgBotWorldState_Unit::mut_mana_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "mana_max",
                    CMsgBotWorldState_Unit::get_mana_max_for_reflect,
                    CMsgBotWorldState_Unit::mut_mana_max_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "mana_regen",
                    CMsgBotWorldState_Unit::get_mana_regen_for_reflect,
                    CMsgBotWorldState_Unit::mut_mana_regen_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "base_movement_speed",
                    CMsgBotWorldState_Unit::get_base_movement_speed_for_reflect,
                    CMsgBotWorldState_Unit::mut_base_movement_speed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "current_movement_speed",
                    CMsgBotWorldState_Unit::get_current_movement_speed_for_reflect,
                    CMsgBotWorldState_Unit::mut_current_movement_speed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "anim_activity",
                    CMsgBotWorldState_Unit::get_anim_activity_for_reflect,
                    CMsgBotWorldState_Unit::mut_anim_activity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "anim_cycle",
                    CMsgBotWorldState_Unit::get_anim_cycle_for_reflect,
                    CMsgBotWorldState_Unit::mut_anim_cycle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "base_damage",
                    CMsgBotWorldState_Unit::get_base_damage_for_reflect,
                    CMsgBotWorldState_Unit::mut_base_damage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "base_damage_variance",
                    CMsgBotWorldState_Unit::get_base_damage_variance_for_reflect,
                    CMsgBotWorldState_Unit::mut_base_damage_variance_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bonus_damage",
                    CMsgBotWorldState_Unit::get_bonus_damage_for_reflect,
                    CMsgBotWorldState_Unit::mut_bonus_damage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "attack_damage",
                    CMsgBotWorldState_Unit::get_attack_damage_for_reflect,
                    CMsgBotWorldState_Unit::mut_attack_damage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "attack_range",
                    CMsgBotWorldState_Unit::get_attack_range_for_reflect,
                    CMsgBotWorldState_Unit::mut_attack_range_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "attack_speed",
                    CMsgBotWorldState_Unit::get_attack_speed_for_reflect,
                    CMsgBotWorldState_Unit::mut_attack_speed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "attack_anim_point",
                    CMsgBotWorldState_Unit::get_attack_anim_point_for_reflect,
                    CMsgBotWorldState_Unit::mut_attack_anim_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "attack_acquisition_range",
                    CMsgBotWorldState_Unit::get_attack_acquisition_range_for_reflect,
                    CMsgBotWorldState_Unit::mut_attack_acquisition_range_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "attack_projectile_speed",
                    CMsgBotWorldState_Unit::get_attack_projectile_speed_for_reflect,
                    CMsgBotWorldState_Unit::mut_attack_projectile_speed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "attack_target_handle",
                    CMsgBotWorldState_Unit::get_attack_target_handle_for_reflect,
                    CMsgBotWorldState_Unit::mut_attack_target_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bounty_xp",
                    CMsgBotWorldState_Unit::get_bounty_xp_for_reflect,
                    CMsgBotWorldState_Unit::mut_bounty_xp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bounty_gold_min",
                    CMsgBotWorldState_Unit::get_bounty_gold_min_for_reflect,
                    CMsgBotWorldState_Unit::mut_bounty_gold_min_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bounty_gold_max",
                    CMsgBotWorldState_Unit::get_bounty_gold_max_for_reflect,
                    CMsgBotWorldState_Unit::mut_bounty_gold_max_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_channeling",
                    CMsgBotWorldState_Unit::get_is_channeling_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_channeling_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "active_ability_handle",
                    CMsgBotWorldState_Unit::get_active_ability_handle_for_reflect,
                    CMsgBotWorldState_Unit::mut_active_ability_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_attack_immune",
                    CMsgBotWorldState_Unit::get_is_attack_immune_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_attack_immune_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_blind",
                    CMsgBotWorldState_Unit::get_is_blind_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_blind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_block_disabled",
                    CMsgBotWorldState_Unit::get_is_block_disabled_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_block_disabled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_disarmed",
                    CMsgBotWorldState_Unit::get_is_disarmed_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_disarmed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_dominated",
                    CMsgBotWorldState_Unit::get_is_dominated_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_dominated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_evade_disabled",
                    CMsgBotWorldState_Unit::get_is_evade_disabled_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_evade_disabled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_hexed",
                    CMsgBotWorldState_Unit::get_is_hexed_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_hexed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_invisible",
                    CMsgBotWorldState_Unit::get_is_invisible_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_invisible_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_invulnerable",
                    CMsgBotWorldState_Unit::get_is_invulnerable_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_invulnerable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_magic_immune",
                    CMsgBotWorldState_Unit::get_is_magic_immune_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_magic_immune_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_muted",
                    CMsgBotWorldState_Unit::get_is_muted_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_muted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_nightmared",
                    CMsgBotWorldState_Unit::get_is_nightmared_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_nightmared_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_rooted",
                    CMsgBotWorldState_Unit::get_is_rooted_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_rooted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_silenced",
                    CMsgBotWorldState_Unit::get_is_silenced_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_silenced_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_specially_deniable",
                    CMsgBotWorldState_Unit::get_is_specially_deniable_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_specially_deniable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_stunned",
                    CMsgBotWorldState_Unit::get_is_stunned_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_stunned_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_unable_to_miss",
                    CMsgBotWorldState_Unit::get_is_unable_to_miss_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_unable_to_miss_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_scepter",
                    CMsgBotWorldState_Unit::get_has_scepter_for_reflect,
                    CMsgBotWorldState_Unit::mut_has_scepter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_Ability>>(
                    "abilities",
                    CMsgBotWorldState_Unit::get_abilities_for_reflect,
                    CMsgBotWorldState_Unit::mut_abilities_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_Ability>>(
                    "items",
                    CMsgBotWorldState_Unit::get_items_for_reflect,
                    CMsgBotWorldState_Unit::mut_items_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_Modifier>>(
                    "modifiers",
                    CMsgBotWorldState_Unit::get_modifiers_for_reflect,
                    CMsgBotWorldState_Unit::mut_modifiers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgBotWorldState_TrackingProjectile>>(
                    "incoming_tracking_projectiles",
                    CMsgBotWorldState_Unit::get_incoming_tracking_projectiles_for_reflect,
                    CMsgBotWorldState_Unit::mut_incoming_tracking_projectiles_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "action_type",
                    CMsgBotWorldState_Unit::get_action_type_for_reflect,
                    CMsgBotWorldState_Unit::mut_action_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_target_handle",
                    CMsgBotWorldState_Unit::get_ability_target_handle_for_reflect,
                    CMsgBotWorldState_Unit::mut_ability_target_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "primary_attribute",
                    CMsgBotWorldState_Unit::get_primary_attribute_for_reflect,
                    CMsgBotWorldState_Unit::mut_primary_attribute_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_illusion",
                    CMsgBotWorldState_Unit::get_is_illusion_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_illusion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "respawn_time",
                    CMsgBotWorldState_Unit::get_respawn_time_for_reflect,
                    CMsgBotWorldState_Unit::mut_respawn_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "buyback_cost",
                    CMsgBotWorldState_Unit::get_buyback_cost_for_reflect,
                    CMsgBotWorldState_Unit::mut_buyback_cost_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "buyback_cooldown",
                    CMsgBotWorldState_Unit::get_buyback_cooldown_for_reflect,
                    CMsgBotWorldState_Unit::mut_buyback_cooldown_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "spell_amplifiction",
                    CMsgBotWorldState_Unit::get_spell_amplifiction_for_reflect,
                    CMsgBotWorldState_Unit::mut_spell_amplifiction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "armor",
                    CMsgBotWorldState_Unit::get_armor_for_reflect,
                    CMsgBotWorldState_Unit::mut_armor_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "magic_resist",
                    CMsgBotWorldState_Unit::get_magic_resist_for_reflect,
                    CMsgBotWorldState_Unit::mut_magic_resist_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "evasion",
                    CMsgBotWorldState_Unit::get_evasion_for_reflect,
                    CMsgBotWorldState_Unit::mut_evasion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "xp_needed_to_level",
                    CMsgBotWorldState_Unit::get_xp_needed_to_level_for_reflect,
                    CMsgBotWorldState_Unit::mut_xp_needed_to_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_points",
                    CMsgBotWorldState_Unit::get_ability_points_for_reflect,
                    CMsgBotWorldState_Unit::mut_ability_points_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "reliable_gold",
                    CMsgBotWorldState_Unit::get_reliable_gold_for_reflect,
                    CMsgBotWorldState_Unit::mut_reliable_gold_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "unreliable_gold",
                    CMsgBotWorldState_Unit::get_unreliable_gold_for_reflect,
                    CMsgBotWorldState_Unit::mut_unreliable_gold_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "last_hits",
                    CMsgBotWorldState_Unit::get_last_hits_for_reflect,
                    CMsgBotWorldState_Unit::mut_last_hits_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "denies",
                    CMsgBotWorldState_Unit::get_denies_for_reflect,
                    CMsgBotWorldState_Unit::mut_denies_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "net_worth",
                    CMsgBotWorldState_Unit::get_net_worth_for_reflect,
                    CMsgBotWorldState_Unit::mut_net_worth_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "remaining_lifespan",
                    CMsgBotWorldState_Unit::get_remaining_lifespan_for_reflect,
                    CMsgBotWorldState_Unit::mut_remaining_lifespan_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "flying_courier",
                    CMsgBotWorldState_Unit::get_flying_courier_for_reflect,
                    CMsgBotWorldState_Unit::mut_flying_courier_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "shrine_cooldown",
                    CMsgBotWorldState_Unit::get_shrine_cooldown_for_reflect,
                    CMsgBotWorldState_Unit::mut_shrine_cooldown_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_shrine_healing",
                    CMsgBotWorldState_Unit::get_is_shrine_healing_for_reflect,
                    CMsgBotWorldState_Unit::mut_is_shrine_healing_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotWorldState_Unit>(
                    "CMsgBotWorldState_Unit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotWorldState_Unit {
    fn clear(&mut self) {
        self.clear_handle();
        self.clear_unit_type();
        self.clear_name();
        self.clear_team_id();
        self.clear_level();
        self.clear_location();
        self.clear_is_alive();
        self.clear_bounding_radius();
        self.clear_facing();
        self.clear_ground_height();
        self.clear_vision_range_daytime();
        self.clear_vision_range_nighttime();
        self.clear_health();
        self.clear_health_max();
        self.clear_health_regen();
        self.clear_mana();
        self.clear_mana_max();
        self.clear_mana_regen();
        self.clear_base_movement_speed();
        self.clear_current_movement_speed();
        self.clear_anim_activity();
        self.clear_anim_cycle();
        self.clear_base_damage();
        self.clear_base_damage_variance();
        self.clear_bonus_damage();
        self.clear_attack_damage();
        self.clear_attack_range();
        self.clear_attack_speed();
        self.clear_attack_anim_point();
        self.clear_attack_acquisition_range();
        self.clear_attack_projectile_speed();
        self.clear_attack_target_handle();
        self.clear_bounty_xp();
        self.clear_bounty_gold_min();
        self.clear_bounty_gold_max();
        self.clear_is_channeling();
        self.clear_active_ability_handle();
        self.clear_is_attack_immune();
        self.clear_is_blind();
        self.clear_is_block_disabled();
        self.clear_is_disarmed();
        self.clear_is_dominated();
        self.clear_is_evade_disabled();
        self.clear_is_hexed();
        self.clear_is_invisible();
        self.clear_is_invulnerable();
        self.clear_is_magic_immune();
        self.clear_is_muted();
        self.clear_is_nightmared();
        self.clear_is_rooted();
        self.clear_is_silenced();
        self.clear_is_specially_deniable();
        self.clear_is_stunned();
        self.clear_is_unable_to_miss();
        self.clear_has_scepter();
        self.clear_abilities();
        self.clear_items();
        self.clear_modifiers();
        self.clear_incoming_tracking_projectiles();
        self.clear_action_type();
        self.clear_ability_target_handle();
        self.clear_primary_attribute();
        self.clear_is_illusion();
        self.clear_respawn_time();
        self.clear_buyback_cost();
        self.clear_buyback_cooldown();
        self.clear_spell_amplifiction();
        self.clear_armor();
        self.clear_magic_resist();
        self.clear_evasion();
        self.clear_xp_needed_to_level();
        self.clear_ability_points();
        self.clear_reliable_gold();
        self.clear_unreliable_gold();
        self.clear_last_hits();
        self.clear_denies();
        self.clear_net_worth();
        self.clear_remaining_lifespan();
        self.clear_flying_courier();
        self.clear_shrine_cooldown();
        self.clear_is_shrine_healing();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotWorldState_Unit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_Unit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgBotWorldState_UnitType {
    INVALID = 0,
    HERO = 1,
    CREEP_HERO = 2,
    LANE_CREEP = 3,
    JUNGLE_CREEP = 4,
    ROSHAN = 5,
    TOWER = 6,
    BARRACKS = 7,
    SHRINE = 8,
    FORT = 9,
    BUILDING = 10,
    COURIER = 11,
    WARD = 12,
}

impl ::protobuf::ProtobufEnum for CMsgBotWorldState_UnitType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgBotWorldState_UnitType> {
        match value {
            0 => ::std::option::Option::Some(CMsgBotWorldState_UnitType::INVALID),
            1 => ::std::option::Option::Some(CMsgBotWorldState_UnitType::HERO),
            2 => ::std::option::Option::Some(CMsgBotWorldState_UnitType::CREEP_HERO),
            3 => ::std::option::Option::Some(CMsgBotWorldState_UnitType::LANE_CREEP),
            4 => ::std::option::Option::Some(CMsgBotWorldState_UnitType::JUNGLE_CREEP),
            5 => ::std::option::Option::Some(CMsgBotWorldState_UnitType::ROSHAN),
            6 => ::std::option::Option::Some(CMsgBotWorldState_UnitType::TOWER),
            7 => ::std::option::Option::Some(CMsgBotWorldState_UnitType::BARRACKS),
            8 => ::std::option::Option::Some(CMsgBotWorldState_UnitType::SHRINE),
            9 => ::std::option::Option::Some(CMsgBotWorldState_UnitType::FORT),
            10 => ::std::option::Option::Some(CMsgBotWorldState_UnitType::BUILDING),
            11 => ::std::option::Option::Some(CMsgBotWorldState_UnitType::COURIER),
            12 => ::std::option::Option::Some(CMsgBotWorldState_UnitType::WARD),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgBotWorldState_UnitType] = &[
            CMsgBotWorldState_UnitType::INVALID,
            CMsgBotWorldState_UnitType::HERO,
            CMsgBotWorldState_UnitType::CREEP_HERO,
            CMsgBotWorldState_UnitType::LANE_CREEP,
            CMsgBotWorldState_UnitType::JUNGLE_CREEP,
            CMsgBotWorldState_UnitType::ROSHAN,
            CMsgBotWorldState_UnitType::TOWER,
            CMsgBotWorldState_UnitType::BARRACKS,
            CMsgBotWorldState_UnitType::SHRINE,
            CMsgBotWorldState_UnitType::FORT,
            CMsgBotWorldState_UnitType::BUILDING,
            CMsgBotWorldState_UnitType::COURIER,
            CMsgBotWorldState_UnitType::WARD,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CMsgBotWorldState_UnitType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgBotWorldState_UnitType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgBotWorldState_UnitType {
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotWorldState_UnitType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'dota_gcmessages_common_bot_script.proto\"\xfd<\n\x11CMsgBotWorldState\
    \x12\x17\n\x07team_id\x18\x01\x20\x01(\rR\x06teamId\x12\x1b\n\tgame_time\
    \x18\x02\x20\x01(\x02R\x08gameTime\x12\x1b\n\tdota_time\x18\x03\x20\x01(\
    \x02R\x08dotaTime\x12\x1d\n\ngame_state\x18\x04\x20\x01(\rR\tgameState\
    \x12&\n\x0fhero_pick_state\x18\x05\x20\x01(\rR\rheroPickState\x12\x1e\n\
    \x0btime_of_day\x18\x06\x20\x01(\x02R\ttimeOfDay\x12%\n\x0eglyph_cooldow\
    n\x18\x07\x20\x01(\x02R\rglyphCooldown\x120\n\x14glyph_cooldown_enemy\
    \x18\x08\x20\x01(\rR\x12glyphCooldownEnemy\x123\n\x07players\x18\n\x20\
    \x03(\x0b2\x19.CMsgBotWorldState.PlayerR\x07players\x12-\n\x05units\x18\
    \x0b\x20\x03(\x0b2\x17.CMsgBotWorldState.UnitR\x05units\x12C\n\rdropped_\
    items\x18\x0c\x20\x03(\x0b2\x1e.CMsgBotWorldState.DroppedItemR\x0cdroppe\
    dItems\x12:\n\nrune_infos\x18\r\x20\x03(\x0b2\x1b.CMsgBotWorldState.Rune\
    InfoR\truneInfos\x12N\n\x12incoming_teleports\x18\x0e\x20\x03(\x0b2\x1f.\
    CMsgBotWorldState.TeleportInfoR\x11incomingTeleports\x12R\n\x12linear_pr\
    ojectiles\x18\x0f\x20\x03(\x0b2#.CMsgBotWorldState.LinearProjectileR\x11\
    linearProjectiles\x12I\n\x0favoidance_zones\x18\x10\x20\x03(\x0b2\x20.CM\
    sgBotWorldState.AvoidanceZoneR\x0eavoidanceZones\x12F\n\x0eability_event\
    s\x18\x14\x20\x03(\x0b2\x1f.CMsgBotWorldState.EventAbilityR\rabilityEven\
    ts\x12C\n\rdamage_events\x18\x15\x20\x03(\x0b2\x1e.CMsgBotWorldState.Eve\
    ntDamageR\x0cdamageEvents\x12Y\n\x15courier_killed_events\x18\x16\x20\
    \x03(\x0b2%.CMsgBotWorldState.EventCourierKilledR\x13courierKilledEvents\
    \x12V\n\x14roshan_killed_events\x18\x17\x20\x03(\x0b2$.CMsgBotWorldState\
    .EventRoshanKilledR\x12roshanKilledEvents\x12=\n\x0btree_events\x18\x18\
    \x20\x03(\x0b2\x1c.CMsgBotWorldState.EventTreeR\ntreeEvents\x1a2\n\x06Ve\
    ctor\x12\x0c\n\x01x\x18\x01\x20\x02(\x05R\x01x\x12\x0c\n\x01y\x18\x02\
    \x20\x02(\x05R\x01y\x12\x0c\n\x01z\x18\x03\x20\x02(\x05R\x01z\x1a\xc4\
    \x01\n\x06Player\x12\x1b\n\tplayer_id\x18\x01\x20\x01(\rR\x08playerId\
    \x12\x17\n\x07hero_id\x18\x02\x20\x01(\rR\x06heroId\x12\x19\n\x08is_aliv\
    e\x18\x03\x20\x01(\x08R\x07isAlive\x12!\n\x0crespawn_time\x18\x04\x20\
    \x01(\x02R\x0brespawnTime\x12\x14\n\x05kills\x18\x05\x20\x01(\rR\x05kill\
    s\x12\x16\n\x06deaths\x18\x06\x20\x01(\rR\x06deaths\x12\x18\n\x07assists\
    \x18\x07\x20\x01(\rR\x07assists\x1a\xd4\x04\n\x07Ability\x12\x16\n\x06ha\
    ndle\x18\x01\x20\x01(\rR\x06handle\x12\x1d\n\nability_id\x18\x02\x20\x01\
    (\rR\tabilityId\x12\x12\n\x04slot\x18\x03\x20\x01(\rR\x04slot\x12#\n\rca\
    ster_handle\x18\x05\x20\x01(\rR\x0ccasterHandle\x12\x14\n\x05level\x18\
    \x06\x20\x01(\rR\x05level\x12\x1d\n\ncast_range\x18\n\x20\x01(\rR\tcastR\
    ange\x12!\n\x0cchannel_time\x18\x0b\x20\x01(\x02R\x0bchannelTime\x12-\n\
    \x12cooldown_remaining\x18\x0c\x20\x01(\x02R\x11cooldownRemaining\x12!\n\
    \x0cis_activated\x18\x14\x20\x01(\x08R\x0bisActivated\x12\x1d\n\nis_togg\
    led\x18\x15\x20\x01(\x08R\tisToggled\x12-\n\x13is_in_ability_phase\x18\
    \x16\x20\x01(\x08R\x10isInAbilityPhase\x12#\n\ris_channeling\x18\x17\x20\
    \x01(\x08R\x0cisChanneling\x12\x1b\n\tis_stolen\x18\x18\x20\x01(\x08R\
    \x08isStolen\x12\x18\n\x07charges\x18\x1e\x20\x01(\rR\x07charges\x12+\n\
    \x11secondary_charges\x18\x1f\x20\x01(\rR\x10secondaryCharges\x12,\n\x12\
    is_combined_locked\x18(\x20\x01(\x08R\x10isCombinedLocked\x12*\n\x11powe\
    r_treads_stat\x182\x20\x01(\rR\x0fpowerTreadsStat\x1a]\n\x0bDroppedItem\
    \x12\x17\n\x07item_id\x18\x01\x20\x01(\rR\x06itemId\x125\n\x08location\
    \x18\x02\x20\x01(\x0b2\x19.CMsgBotWorldState.VectorR\x08location\x1a\x95\
    \x01\n\x08RuneInfo\x12\x12\n\x04type\x18\x01\x20\x01(\x05R\x04type\x125\
    \n\x08location\x18\x02\x20\x01(\x0b2\x19.CMsgBotWorldState.VectorR\x08lo\
    cation\x12\x16\n\x06status\x18\x03\x20\x01(\rR\x06status\x12&\n\x0ftime_\
    since_seen\x18\x04\x20\x01(\x02R\rtimeSinceSeen\x1a\x87\x01\n\x0cTelepor\
    tInfo\x12\x1b\n\tplayer_id\x18\x01\x20\x01(\rR\x08playerId\x125\n\x08loc\
    ation\x18\x02\x20\x01(\x0b2\x19.CMsgBotWorldState.VectorR\x08location\
    \x12#\n\rtime_remaning\x18\x03\x20\x01(\x02R\x0ctimeRemaning\x1a\xec\x01\
    \n\x08Modifier\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x1f\n\
    \x0bstack_count\x18\x02\x20\x01(\rR\nstackCount\x12%\n\x0eability_handle\
    \x18\x03\x20\x01(\rR\rabilityHandle\x12\x1d\n\nability_id\x18\x04\x20\
    \x01(\rR\tabilityId\x12-\n\x12remaining_duration\x18\x05\x20\x01(\x02R\
    \x11remainingDuration\x126\n\x17auxiliary_units_handles\x18\x06\x20\x03(\
    \rR\x15auxiliaryUnitsHandles\x1a\xc5\x02\n\x10LinearProjectile\x12\x16\n\
    \x06handle\x18\x01\x20\x01(\rR\x06handle\x12#\n\rcaster_handle\x18\x02\
    \x20\x01(\rR\x0ccasterHandle\x12(\n\x10caster_player_id\x18\x03\x20\x01(\
    \x05R\x0ecasterPlayerId\x12%\n\x0eability_handle\x18\x04\x20\x01(\rR\rab\
    ilityHandle\x12\x1d\n\nability_id\x18\x05\x20\x01(\rR\tabilityId\x125\n\
    \x08location\x18\x06\x20\x01(\x0b2\x19.CMsgBotWorldState.VectorR\x08loca\
    tion\x125\n\x08velocity\x18\x07\x20\x01(\x0b2\x19.CMsgBotWorldState.Vect\
    orR\x08velocity\x12\x16\n\x06radius\x18\x08\x20\x01(\rR\x06radius\x1a\
    \xbc\x02\n\x12TrackingProjectile\x12#\n\rcaster_handle\x18\x01\x20\x01(\
    \rR\x0ccasterHandle\x12(\n\x10caster_player_id\x18\x02\x20\x01(\x05R\x0e\
    casterPlayerId\x12%\n\x0eability_handle\x18\x03\x20\x01(\rR\rabilityHand\
    le\x12\x1d\n\nability_id\x18\x04\x20\x01(\rR\tabilityId\x125\n\x08locati\
    on\x18\x05\x20\x01(\x0b2\x19.CMsgBotWorldState.VectorR\x08location\x12\
    \x1a\n\x08velocity\x18\x06\x20\x01(\rR\x08velocity\x12!\n\x0cis_dodgeabl\
    e\x18\x07\x20\x01(\x08R\x0bisDodgeable\x12\x1b\n\tis_attack\x18\x08\x20\
    \x01(\x08R\x08isAttack\x1a\xf3\x01\n\rAvoidanceZone\x125\n\x08location\
    \x18\x01\x20\x01(\x0b2\x19.CMsgBotWorldState.VectorR\x08location\x12#\n\
    \rcaster_handle\x18\x02\x20\x01(\rR\x0ccasterHandle\x12(\n\x10caster_pla\
    yer_id\x18\x03\x20\x01(\x05R\x0ecasterPlayerId\x12%\n\x0eability_handle\
    \x18\x04\x20\x01(\rR\rabilityHandle\x12\x1d\n\nability_id\x18\x05\x20\
    \x01(\rR\tabilityId\x12\x16\n\x06radius\x18\x06\x20\x01(\rR\x06radius\
    \x1a\xcc\x01\n\x0cEventAbility\x12\x1d\n\nability_id\x18\x01\x20\x01(\rR\
    \tabilityId\x12\x1b\n\tplayer_id\x18\x02\x20\x01(\rR\x08playerId\x12\x1f\
    \n\x0bunit_handle\x18\x03\x20\x01(\rR\nunitHandle\x125\n\x08location\x18\
    \x04\x20\x01(\x0b2\x19.CMsgBotWorldState.VectorR\x08location\x12(\n\x10i\
    s_channel_start\x18\x05\x20\x01(\x08R\x0eisChannelStart\x1a\xfc\x01\n\
    \x0bEventDamage\x12\x16\n\x06damage\x18\x01\x20\x01(\rR\x06damage\x12(\n\
    \x10victim_player_id\x18\x02\x20\x01(\rR\x0evictimPlayerId\x12,\n\x12vic\
    tim_unit_handle\x18\x03\x20\x01(\rR\x10victimUnitHandle\x12,\n\x12attack\
    er_player_id\x18\x04\x20\x01(\rR\x10attackerPlayerId\x120\n\x14attacker_\
    unit_handle\x18\x05\x20\x01(\rR\x12attackerUnitHandle\x12\x1d\n\nability\
    _id\x18\x06\x20\x01(\rR\tabilityId\x1a\xb5\x01\n\x12EventCourierKilled\
    \x12\x17\n\x07team_id\x18\x01\x20\x01(\rR\x06teamId\x12.\n\x13courier_un\
    it_handle\x18\x02\x20\x01(\rR\x11courierUnitHandle\x12(\n\x10killer_play\
    er_id\x18\x03\x20\x01(\rR\x0ekillerPlayerId\x12,\n\x12killer_unit_handle\
    \x18\x04\x20\x01(\rR\x10killerUnitHandle\x1ak\n\x11EventRoshanKilled\x12\
    (\n\x10killer_player_id\x18\x01\x20\x01(\rR\x0ekillerPlayerId\x12,\n\x12\
    killer_unit_handle\x18\x02\x20\x01(\rR\x10killerUnitHandle\x1a\xb1\x01\n\
    \tEventTree\x12\x17\n\x07tree_id\x18\x01\x20\x01(\rR\x06treeId\x12\x1c\n\
    \tdestroyed\x18\x02\x20\x01(\x08R\tdestroyed\x12\x1c\n\trespawned\x18\
    \x03\x20\x01(\x08R\trespawned\x125\n\x08location\x18\x04\x20\x01(\x0b2\
    \x19.CMsgBotWorldState.VectorR\x08location\x12\x18\n\x07delayed\x18\x05\
    \x20\x01(\x08R\x07delayed\x1a\xe5\x18\n\x04Unit\x12\x16\n\x06handle\x18\
    \x01\x20\x01(\rR\x06handle\x12A\n\tunit_type\x18\x02\x20\x01(\x0e2\x1b.C\
    MsgBotWorldState.UnitType:\x07INVALIDR\x08unitType\x12\x12\n\x04name\x18\
    \x03\x20\x01(\tR\x04name\x12\x17\n\x07team_id\x18\x04\x20\x01(\rR\x06tea\
    mId\x12\x14\n\x05level\x18\x05\x20\x01(\rR\x05level\x125\n\x08location\
    \x18\x06\x20\x01(\x0b2\x19.CMsgBotWorldState.VectorR\x08location\x12\x19\
    \n\x08is_alive\x18\x07\x20\x01(\x08R\x07isAlive\x12'\n\x0fbounding_radiu\
    s\x18\n\x20\x01(\rR\x0eboundingRadius\x12\x16\n\x06facing\x18\x0b\x20\
    \x01(\rR\x06facing\x12#\n\rground_height\x18\x0c\x20\x01(\rR\x0cgroundHe\
    ight\x120\n\x14vision_range_daytime\x18\x0f\x20\x01(\rR\x12visionRangeDa\
    ytime\x124\n\x16vision_range_nighttime\x18\x10\x20\x01(\rR\x14visionRang\
    eNighttime\x12\x16\n\x06health\x18\x14\x20\x01(\rR\x06health\x12\x1d\n\n\
    health_max\x18\x15\x20\x01(\rR\thealthMax\x12!\n\x0chealth_regen\x18\x16\
    \x20\x01(\x02R\x0bhealthRegen\x12\x12\n\x04mana\x18\x19\x20\x01(\rR\x04m\
    ana\x12\x19\n\x08mana_max\x18\x1a\x20\x01(\rR\x07manaMax\x12\x1d\n\nmana\
    _regen\x18\x1b\x20\x01(\x02R\tmanaRegen\x12.\n\x13base_movement_speed\
    \x18\x1e\x20\x01(\rR\x11baseMovementSpeed\x124\n\x16current_movement_spe\
    ed\x18\x1f\x20\x01(\rR\x14currentMovementSpeed\x12#\n\ranim_activity\x18\
    #\x20\x01(\x05R\x0canimActivity\x12\x1d\n\nanim_cycle\x18$\x20\x01(\x02R\
    \tanimCycle\x12\x1f\n\x0bbase_damage\x18(\x20\x01(\rR\nbaseDamage\x120\n\
    \x14base_damage_variance\x18)\x20\x01(\rR\x12baseDamageVariance\x12!\n\
    \x0cbonus_damage\x18*\x20\x01(\rR\x0bbonusDamage\x12#\n\rattack_damage\
    \x18+\x20\x01(\rR\x0cattackDamage\x12!\n\x0cattack_range\x18,\x20\x01(\r\
    R\x0battackRange\x12!\n\x0cattack_speed\x18-\x20\x01(\rR\x0battackSpeed\
    \x12*\n\x11attack_anim_point\x18.\x20\x01(\x02R\x0fattackAnimPoint\x128\
    \n\x18attack_acquisition_range\x18/\x20\x01(\rR\x16attackAcquisitionRang\
    e\x126\n\x17attack_projectile_speed\x180\x20\x01(\rR\x15attackProjectile\
    Speed\x120\n\x14attack_target_handle\x181\x20\x01(\rR\x12attackTargetHan\
    dle\x12\x1b\n\tbounty_xp\x18<\x20\x01(\rR\x08bountyXp\x12&\n\x0fbounty_g\
    old_min\x18=\x20\x01(\rR\rbountyGoldMin\x12&\n\x0fbounty_gold_max\x18>\
    \x20\x01(\rR\rbountyGoldMax\x12#\n\ris_channeling\x18A\x20\x01(\x08R\x0c\
    isChanneling\x122\n\x15active_ability_handle\x18B\x20\x01(\rR\x13activeA\
    bilityHandle\x12(\n\x10is_attack_immune\x18F\x20\x01(\x08R\x0eisAttackIm\
    mune\x12\x19\n\x08is_blind\x18G\x20\x01(\x08R\x07isBlind\x12*\n\x11is_bl\
    ock_disabled\x18H\x20\x01(\x08R\x0fisBlockDisabled\x12\x1f\n\x0bis_disar\
    med\x18I\x20\x01(\x08R\nisDisarmed\x12!\n\x0cis_dominated\x18J\x20\x01(\
    \x08R\x0bisDominated\x12*\n\x11is_evade_disabled\x18K\x20\x01(\x08R\x0fi\
    sEvadeDisabled\x12\x19\n\x08is_hexed\x18L\x20\x01(\x08R\x07isHexed\x12!\
    \n\x0cis_invisible\x18M\x20\x01(\x08R\x0bisInvisible\x12'\n\x0fis_invuln\
    erable\x18N\x20\x01(\x08R\x0eisInvulnerable\x12&\n\x0fis_magic_immune\
    \x18O\x20\x01(\x08R\risMagicImmune\x12\x19\n\x08is_muted\x18P\x20\x01(\
    \x08R\x07isMuted\x12#\n\ris_nightmared\x18R\x20\x01(\x08R\x0cisNightmare\
    d\x12\x1b\n\tis_rooted\x18S\x20\x01(\x08R\x08isRooted\x12\x1f\n\x0bis_si\
    lenced\x18T\x20\x01(\x08R\nisSilenced\x122\n\x15is_specially_deniable\
    \x18U\x20\x01(\x08R\x13isSpeciallyDeniable\x12\x1d\n\nis_stunned\x18V\
    \x20\x01(\x08R\tisStunned\x12)\n\x11is_unable_to_miss\x18W\x20\x01(\x08R\
    \x0eisUnableToMiss\x12\x1f\n\x0bhas_scepter\x18X\x20\x01(\x08R\nhasScept\
    er\x128\n\tabilities\x18Z\x20\x03(\x0b2\x1a.CMsgBotWorldState.AbilityR\t\
    abilities\x120\n\x05items\x18[\x20\x03(\x0b2\x1a.CMsgBotWorldState.Abili\
    tyR\x05items\x129\n\tmodifiers\x18\\\x20\x03(\x0b2\x1b.CMsgBotWorldState\
    .ModifierR\tmodifiers\x12i\n\x1dincoming_tracking_projectiles\x18]\x20\
    \x03(\x0b2%.CMsgBotWorldState.TrackingProjectileR\x1bincomingTrackingPro\
    jectiles\x12\x1f\n\x0baction_type\x18d\x20\x01(\rR\nactionType\x122\n\
    \x15ability_target_handle\x18e\x20\x01(\rR\x13abilityTargetHandle\x12+\n\
    \x11primary_attribute\x18n\x20\x01(\rR\x10primaryAttribute\x12\x1f\n\x0b\
    is_illusion\x18o\x20\x01(\x08R\nisIllusion\x12!\n\x0crespawn_time\x18p\
    \x20\x01(\rR\x0brespawnTime\x12!\n\x0cbuyback_cost\x18q\x20\x01(\rR\x0bb\
    uybackCost\x12)\n\x10buyback_cooldown\x18r\x20\x01(\rR\x0fbuybackCooldow\
    n\x12-\n\x12spell_amplifiction\x18s\x20\x01(\x02R\x11spellAmplifiction\
    \x12\x14\n\x05armor\x18t\x20\x01(\rR\x05armor\x12!\n\x0cmagic_resist\x18\
    u\x20\x01(\x02R\x0bmagicResist\x12\x18\n\x07evasion\x18v\x20\x01(\x02R\
    \x07evasion\x12+\n\x12xp_needed_to_level\x18x\x20\x01(\rR\x0fxpNeededToL\
    evel\x12%\n\x0eability_points\x18y\x20\x01(\rR\rabilityPoints\x12#\n\rre\
    liable_gold\x18z\x20\x01(\rR\x0creliableGold\x12'\n\x0funreliable_gold\
    \x18{\x20\x01(\rR\x0eunreliableGold\x12\x1b\n\tlast_hits\x18|\x20\x01(\r\
    R\x08lastHits\x12\x16\n\x06denies\x18}\x20\x01(\rR\x06denies\x12\x1b\n\t\
    net_worth\x18~\x20\x01(\rR\x08netWorth\x12.\n\x12remaining_lifespan\x18\
    \x82\x01\x20\x01(\x02R\x11remainingLifespan\x12&\n\x0eflying_courier\x18\
    \x8c\x01\x20\x01(\x08R\rflyingCourier\x12(\n\x0fshrine_cooldown\x18\x96\
    \x01\x20\x01(\x02R\x0eshrineCooldown\x12+\n\x11is_shrine_healing\x18\x97\
    \x01\x20\x01(\x08R\x0fisShrineHealing\"\xb3\x01\n\x08UnitType\x12\x0b\n\
    \x07INVALID\x10\0\x12\x08\n\x04HERO\x10\x01\x12\x0e\n\nCREEP_HERO\x10\
    \x02\x12\x0e\n\nLANE_CREEP\x10\x03\x12\x10\n\x0cJUNGLE_CREEP\x10\x04\x12\
    \n\n\x06ROSHAN\x10\x05\x12\t\n\x05TOWER\x10\x06\x12\x0c\n\x08BARRACKS\
    \x10\x07\x12\n\n\x06SHRINE\x10\x08\x12\x08\n\x04FORT\x10\t\x12\x0c\n\x08\
    BUILDING\x10\n\x12\x0b\n\x07COURIER\x10\x0b\x12\x08\n\x04WARD\x10\x0cB\
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
