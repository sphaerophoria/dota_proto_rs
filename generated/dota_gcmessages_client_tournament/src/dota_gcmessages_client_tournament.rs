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
pub struct CMsgDOTATournamentInfo {
    // message fields
    league_id: ::std::option::Option<u32>,
    phase_list: ::protobuf::RepeatedField<CMsgDOTATournamentInfo_Phase>,
    teams_list: ::protobuf::RepeatedField<CMsgDOTATournamentInfo_Team>,
    upcoming_matches_list: ::protobuf::RepeatedField<CMsgDOTATournamentInfo_UpcomingMatch>,
    news_list: ::protobuf::RepeatedField<CMsgDOTATournamentInfo_News>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATournamentInfo {}

impl CMsgDOTATournamentInfo {
    pub fn new() -> CMsgDOTATournamentInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATournamentInfo {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATournamentInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATournamentInfo,
        };
        unsafe {
            instance.get(CMsgDOTATournamentInfo::new)
        }
    }

    // optional uint32 league_id = 1;

    pub fn clear_league_id(&mut self) {
        self.league_id = ::std::option::Option::None;
    }

    pub fn has_league_id(&self) -> bool {
        self.league_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_league_id(&mut self, v: u32) {
        self.league_id = ::std::option::Option::Some(v);
    }

    pub fn get_league_id(&self) -> u32 {
        self.league_id.unwrap_or(0)
    }

    fn get_league_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.league_id
    }

    fn mut_league_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.league_id
    }

    // repeated .CMsgDOTATournamentInfo.Phase phase_list = 2;

    pub fn clear_phase_list(&mut self) {
        self.phase_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_phase_list(&mut self, v: ::protobuf::RepeatedField<CMsgDOTATournamentInfo_Phase>) {
        self.phase_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_phase_list(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournamentInfo_Phase> {
        &mut self.phase_list
    }

    // Take field
    pub fn take_phase_list(&mut self) -> ::protobuf::RepeatedField<CMsgDOTATournamentInfo_Phase> {
        ::std::mem::replace(&mut self.phase_list, ::protobuf::RepeatedField::new())
    }

    pub fn get_phase_list(&self) -> &[CMsgDOTATournamentInfo_Phase] {
        &self.phase_list
    }

    fn get_phase_list_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTATournamentInfo_Phase> {
        &self.phase_list
    }

    fn mut_phase_list_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournamentInfo_Phase> {
        &mut self.phase_list
    }

    // repeated .CMsgDOTATournamentInfo.Team teams_list = 3;

    pub fn clear_teams_list(&mut self) {
        self.teams_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_teams_list(&mut self, v: ::protobuf::RepeatedField<CMsgDOTATournamentInfo_Team>) {
        self.teams_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_teams_list(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournamentInfo_Team> {
        &mut self.teams_list
    }

    // Take field
    pub fn take_teams_list(&mut self) -> ::protobuf::RepeatedField<CMsgDOTATournamentInfo_Team> {
        ::std::mem::replace(&mut self.teams_list, ::protobuf::RepeatedField::new())
    }

    pub fn get_teams_list(&self) -> &[CMsgDOTATournamentInfo_Team] {
        &self.teams_list
    }

    fn get_teams_list_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTATournamentInfo_Team> {
        &self.teams_list
    }

    fn mut_teams_list_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournamentInfo_Team> {
        &mut self.teams_list
    }

    // repeated .CMsgDOTATournamentInfo.UpcomingMatch upcoming_matches_list = 4;

    pub fn clear_upcoming_matches_list(&mut self) {
        self.upcoming_matches_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_upcoming_matches_list(&mut self, v: ::protobuf::RepeatedField<CMsgDOTATournamentInfo_UpcomingMatch>) {
        self.upcoming_matches_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_upcoming_matches_list(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournamentInfo_UpcomingMatch> {
        &mut self.upcoming_matches_list
    }

    // Take field
    pub fn take_upcoming_matches_list(&mut self) -> ::protobuf::RepeatedField<CMsgDOTATournamentInfo_UpcomingMatch> {
        ::std::mem::replace(&mut self.upcoming_matches_list, ::protobuf::RepeatedField::new())
    }

    pub fn get_upcoming_matches_list(&self) -> &[CMsgDOTATournamentInfo_UpcomingMatch] {
        &self.upcoming_matches_list
    }

    fn get_upcoming_matches_list_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTATournamentInfo_UpcomingMatch> {
        &self.upcoming_matches_list
    }

    fn mut_upcoming_matches_list_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournamentInfo_UpcomingMatch> {
        &mut self.upcoming_matches_list
    }

    // repeated .CMsgDOTATournamentInfo.News news_list = 5;

    pub fn clear_news_list(&mut self) {
        self.news_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_news_list(&mut self, v: ::protobuf::RepeatedField<CMsgDOTATournamentInfo_News>) {
        self.news_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_news_list(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournamentInfo_News> {
        &mut self.news_list
    }

    // Take field
    pub fn take_news_list(&mut self) -> ::protobuf::RepeatedField<CMsgDOTATournamentInfo_News> {
        ::std::mem::replace(&mut self.news_list, ::protobuf::RepeatedField::new())
    }

    pub fn get_news_list(&self) -> &[CMsgDOTATournamentInfo_News] {
        &self.news_list
    }

    fn get_news_list_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTATournamentInfo_News> {
        &self.news_list
    }

    fn mut_news_list_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournamentInfo_News> {
        &mut self.news_list
    }
}

impl ::protobuf::Message for CMsgDOTATournamentInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.phase_list {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.teams_list {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.upcoming_matches_list {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.news_list {
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
                    self.league_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.phase_list)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.teams_list)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.upcoming_matches_list)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.news_list)?;
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
        if let Some(v) = self.league_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.phase_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.teams_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.upcoming_matches_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.news_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.league_id {
            os.write_uint32(1, v)?;
        }
        for v in &self.phase_list {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.teams_list {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.upcoming_matches_list {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.news_list {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CMsgDOTATournamentInfo {
    fn new() -> CMsgDOTATournamentInfo {
        CMsgDOTATournamentInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATournamentInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_id",
                    CMsgDOTATournamentInfo::get_league_id_for_reflect,
                    CMsgDOTATournamentInfo::mut_league_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTATournamentInfo_Phase>>(
                    "phase_list",
                    CMsgDOTATournamentInfo::get_phase_list_for_reflect,
                    CMsgDOTATournamentInfo::mut_phase_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTATournamentInfo_Team>>(
                    "teams_list",
                    CMsgDOTATournamentInfo::get_teams_list_for_reflect,
                    CMsgDOTATournamentInfo::mut_teams_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTATournamentInfo_UpcomingMatch>>(
                    "upcoming_matches_list",
                    CMsgDOTATournamentInfo::get_upcoming_matches_list_for_reflect,
                    CMsgDOTATournamentInfo::mut_upcoming_matches_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTATournamentInfo_News>>(
                    "news_list",
                    CMsgDOTATournamentInfo::get_news_list_for_reflect,
                    CMsgDOTATournamentInfo::mut_news_list_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATournamentInfo>(
                    "CMsgDOTATournamentInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATournamentInfo {
    fn clear(&mut self) {
        self.clear_league_id();
        self.clear_phase_list();
        self.clear_teams_list();
        self.clear_upcoming_matches_list();
        self.clear_news_list();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATournamentInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATournamentInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATournamentInfo_PhaseGroup {
    // message fields
    group_id: ::std::option::Option<u32>,
    group_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATournamentInfo_PhaseGroup {}

impl CMsgDOTATournamentInfo_PhaseGroup {
    pub fn new() -> CMsgDOTATournamentInfo_PhaseGroup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATournamentInfo_PhaseGroup {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATournamentInfo_PhaseGroup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATournamentInfo_PhaseGroup,
        };
        unsafe {
            instance.get(CMsgDOTATournamentInfo_PhaseGroup::new)
        }
    }

    // optional uint32 group_id = 1;

    pub fn clear_group_id(&mut self) {
        self.group_id = ::std::option::Option::None;
    }

    pub fn has_group_id(&self) -> bool {
        self.group_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group_id(&mut self, v: u32) {
        self.group_id = ::std::option::Option::Some(v);
    }

    pub fn get_group_id(&self) -> u32 {
        self.group_id.unwrap_or(0)
    }

    fn get_group_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.group_id
    }

    fn mut_group_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.group_id
    }

    // optional string group_name = 2;

    pub fn clear_group_name(&mut self) {
        self.group_name.clear();
    }

    pub fn has_group_name(&self) -> bool {
        self.group_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group_name(&mut self, v: ::std::string::String) {
        self.group_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group_name(&mut self) -> &mut ::std::string::String {
        if self.group_name.is_none() {
            self.group_name.set_default();
        }
        self.group_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_group_name(&mut self) -> ::std::string::String {
        self.group_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_group_name(&self) -> &str {
        match self.group_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_group_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.group_name
    }

    fn mut_group_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.group_name
    }
}

impl ::protobuf::Message for CMsgDOTATournamentInfo_PhaseGroup {
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
                    self.group_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.group_name)?;
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
        if let Some(v) = self.group_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.group_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.group_id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.group_name.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgDOTATournamentInfo_PhaseGroup {
    fn new() -> CMsgDOTATournamentInfo_PhaseGroup {
        CMsgDOTATournamentInfo_PhaseGroup::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATournamentInfo_PhaseGroup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "group_id",
                    CMsgDOTATournamentInfo_PhaseGroup::get_group_id_for_reflect,
                    CMsgDOTATournamentInfo_PhaseGroup::mut_group_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group_name",
                    CMsgDOTATournamentInfo_PhaseGroup::get_group_name_for_reflect,
                    CMsgDOTATournamentInfo_PhaseGroup::mut_group_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATournamentInfo_PhaseGroup>(
                    "CMsgDOTATournamentInfo_PhaseGroup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATournamentInfo_PhaseGroup {
    fn clear(&mut self) {
        self.clear_group_id();
        self.clear_group_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATournamentInfo_PhaseGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATournamentInfo_PhaseGroup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATournamentInfo_Phase {
    // message fields
    phase_id: ::std::option::Option<u32>,
    phase_name: ::protobuf::SingularField<::std::string::String>,
    type_id: ::std::option::Option<u32>,
    iterations: ::std::option::Option<u32>,
    min_start_time: ::std::option::Option<u32>,
    max_start_time: ::std::option::Option<u32>,
    group_list: ::protobuf::RepeatedField<CMsgDOTATournamentInfo_PhaseGroup>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATournamentInfo_Phase {}

impl CMsgDOTATournamentInfo_Phase {
    pub fn new() -> CMsgDOTATournamentInfo_Phase {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATournamentInfo_Phase {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATournamentInfo_Phase> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATournamentInfo_Phase,
        };
        unsafe {
            instance.get(CMsgDOTATournamentInfo_Phase::new)
        }
    }

    // optional uint32 phase_id = 1;

    pub fn clear_phase_id(&mut self) {
        self.phase_id = ::std::option::Option::None;
    }

    pub fn has_phase_id(&self) -> bool {
        self.phase_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_phase_id(&mut self, v: u32) {
        self.phase_id = ::std::option::Option::Some(v);
    }

    pub fn get_phase_id(&self) -> u32 {
        self.phase_id.unwrap_or(0)
    }

    fn get_phase_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.phase_id
    }

    fn mut_phase_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.phase_id
    }

    // optional string phase_name = 2;

    pub fn clear_phase_name(&mut self) {
        self.phase_name.clear();
    }

    pub fn has_phase_name(&self) -> bool {
        self.phase_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_phase_name(&mut self, v: ::std::string::String) {
        self.phase_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_phase_name(&mut self) -> &mut ::std::string::String {
        if self.phase_name.is_none() {
            self.phase_name.set_default();
        }
        self.phase_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_phase_name(&mut self) -> ::std::string::String {
        self.phase_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_phase_name(&self) -> &str {
        match self.phase_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_phase_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.phase_name
    }

    fn mut_phase_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.phase_name
    }

    // optional uint32 type_id = 3;

    pub fn clear_type_id(&mut self) {
        self.type_id = ::std::option::Option::None;
    }

    pub fn has_type_id(&self) -> bool {
        self.type_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_type_id(&mut self, v: u32) {
        self.type_id = ::std::option::Option::Some(v);
    }

    pub fn get_type_id(&self) -> u32 {
        self.type_id.unwrap_or(0)
    }

    fn get_type_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.type_id
    }

    fn mut_type_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.type_id
    }

    // optional uint32 iterations = 4;

    pub fn clear_iterations(&mut self) {
        self.iterations = ::std::option::Option::None;
    }

    pub fn has_iterations(&self) -> bool {
        self.iterations.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iterations(&mut self, v: u32) {
        self.iterations = ::std::option::Option::Some(v);
    }

    pub fn get_iterations(&self) -> u32 {
        self.iterations.unwrap_or(0)
    }

    fn get_iterations_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.iterations
    }

    fn mut_iterations_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.iterations
    }

    // optional uint32 min_start_time = 5;

    pub fn clear_min_start_time(&mut self) {
        self.min_start_time = ::std::option::Option::None;
    }

    pub fn has_min_start_time(&self) -> bool {
        self.min_start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min_start_time(&mut self, v: u32) {
        self.min_start_time = ::std::option::Option::Some(v);
    }

    pub fn get_min_start_time(&self) -> u32 {
        self.min_start_time.unwrap_or(0)
    }

    fn get_min_start_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.min_start_time
    }

    fn mut_min_start_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.min_start_time
    }

    // optional uint32 max_start_time = 6;

    pub fn clear_max_start_time(&mut self) {
        self.max_start_time = ::std::option::Option::None;
    }

    pub fn has_max_start_time(&self) -> bool {
        self.max_start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_start_time(&mut self, v: u32) {
        self.max_start_time = ::std::option::Option::Some(v);
    }

    pub fn get_max_start_time(&self) -> u32 {
        self.max_start_time.unwrap_or(0)
    }

    fn get_max_start_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.max_start_time
    }

    fn mut_max_start_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.max_start_time
    }

    // repeated .CMsgDOTATournamentInfo.PhaseGroup group_list = 7;

    pub fn clear_group_list(&mut self) {
        self.group_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_group_list(&mut self, v: ::protobuf::RepeatedField<CMsgDOTATournamentInfo_PhaseGroup>) {
        self.group_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_group_list(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournamentInfo_PhaseGroup> {
        &mut self.group_list
    }

    // Take field
    pub fn take_group_list(&mut self) -> ::protobuf::RepeatedField<CMsgDOTATournamentInfo_PhaseGroup> {
        ::std::mem::replace(&mut self.group_list, ::protobuf::RepeatedField::new())
    }

    pub fn get_group_list(&self) -> &[CMsgDOTATournamentInfo_PhaseGroup] {
        &self.group_list
    }

    fn get_group_list_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTATournamentInfo_PhaseGroup> {
        &self.group_list
    }

    fn mut_group_list_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournamentInfo_PhaseGroup> {
        &mut self.group_list
    }
}

impl ::protobuf::Message for CMsgDOTATournamentInfo_Phase {
    fn is_initialized(&self) -> bool {
        for v in &self.group_list {
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
                    self.phase_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.phase_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.type_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.iterations = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.min_start_time = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.max_start_time = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.group_list)?;
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
        if let Some(v) = self.phase_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.phase_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.type_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.iterations {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.min_start_time {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.max_start_time {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.group_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.phase_id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.phase_name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.type_id {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.iterations {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.min_start_time {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.max_start_time {
            os.write_uint32(6, v)?;
        }
        for v in &self.group_list {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CMsgDOTATournamentInfo_Phase {
    fn new() -> CMsgDOTATournamentInfo_Phase {
        CMsgDOTATournamentInfo_Phase::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATournamentInfo_Phase>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "phase_id",
                    CMsgDOTATournamentInfo_Phase::get_phase_id_for_reflect,
                    CMsgDOTATournamentInfo_Phase::mut_phase_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "phase_name",
                    CMsgDOTATournamentInfo_Phase::get_phase_name_for_reflect,
                    CMsgDOTATournamentInfo_Phase::mut_phase_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "type_id",
                    CMsgDOTATournamentInfo_Phase::get_type_id_for_reflect,
                    CMsgDOTATournamentInfo_Phase::mut_type_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "iterations",
                    CMsgDOTATournamentInfo_Phase::get_iterations_for_reflect,
                    CMsgDOTATournamentInfo_Phase::mut_iterations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "min_start_time",
                    CMsgDOTATournamentInfo_Phase::get_min_start_time_for_reflect,
                    CMsgDOTATournamentInfo_Phase::mut_min_start_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "max_start_time",
                    CMsgDOTATournamentInfo_Phase::get_max_start_time_for_reflect,
                    CMsgDOTATournamentInfo_Phase::mut_max_start_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTATournamentInfo_PhaseGroup>>(
                    "group_list",
                    CMsgDOTATournamentInfo_Phase::get_group_list_for_reflect,
                    CMsgDOTATournamentInfo_Phase::mut_group_list_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATournamentInfo_Phase>(
                    "CMsgDOTATournamentInfo_Phase",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATournamentInfo_Phase {
    fn clear(&mut self) {
        self.clear_phase_id();
        self.clear_phase_name();
        self.clear_type_id();
        self.clear_iterations();
        self.clear_min_start_time();
        self.clear_max_start_time();
        self.clear_group_list();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATournamentInfo_Phase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATournamentInfo_Phase {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATournamentInfo_Team {
    // message fields
    team_id: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    tag: ::protobuf::SingularField<::std::string::String>,
    team_logo: ::std::option::Option<u64>,
    eliminated: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATournamentInfo_Team {}

impl CMsgDOTATournamentInfo_Team {
    pub fn new() -> CMsgDOTATournamentInfo_Team {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATournamentInfo_Team {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATournamentInfo_Team> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATournamentInfo_Team,
        };
        unsafe {
            instance.get(CMsgDOTATournamentInfo_Team::new)
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

    // optional string tag = 3;

    pub fn clear_tag(&mut self) {
        self.tag.clear();
    }

    pub fn has_tag(&self) -> bool {
        self.tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: ::std::string::String) {
        self.tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tag(&mut self) -> &mut ::std::string::String {
        if self.tag.is_none() {
            self.tag.set_default();
        }
        self.tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_tag(&mut self) -> ::std::string::String {
        self.tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_tag(&self) -> &str {
        match self.tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.tag
    }

    fn mut_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.tag
    }

    // optional uint64 team_logo = 4;

    pub fn clear_team_logo(&mut self) {
        self.team_logo = ::std::option::Option::None;
    }

    pub fn has_team_logo(&self) -> bool {
        self.team_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_logo(&mut self, v: u64) {
        self.team_logo = ::std::option::Option::Some(v);
    }

    pub fn get_team_logo(&self) -> u64 {
        self.team_logo.unwrap_or(0)
    }

    fn get_team_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team_logo
    }

    fn mut_team_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team_logo
    }

    // optional bool eliminated = 5;

    pub fn clear_eliminated(&mut self) {
        self.eliminated = ::std::option::Option::None;
    }

    pub fn has_eliminated(&self) -> bool {
        self.eliminated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eliminated(&mut self, v: bool) {
        self.eliminated = ::std::option::Option::Some(v);
    }

    pub fn get_eliminated(&self) -> bool {
        self.eliminated.unwrap_or(false)
    }

    fn get_eliminated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.eliminated
    }

    fn mut_eliminated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.eliminated
    }
}

impl ::protobuf::Message for CMsgDOTATournamentInfo_Team {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.tag)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.team_logo = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.eliminated = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.tag.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.team_logo {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.eliminated {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team_id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.tag.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.team_logo {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.eliminated {
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

impl ::protobuf::MessageStatic for CMsgDOTATournamentInfo_Team {
    fn new() -> CMsgDOTATournamentInfo_Team {
        CMsgDOTATournamentInfo_Team::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATournamentInfo_Team>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgDOTATournamentInfo_Team::get_team_id_for_reflect,
                    CMsgDOTATournamentInfo_Team::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgDOTATournamentInfo_Team::get_name_for_reflect,
                    CMsgDOTATournamentInfo_Team::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tag",
                    CMsgDOTATournamentInfo_Team::get_tag_for_reflect,
                    CMsgDOTATournamentInfo_Team::mut_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "team_logo",
                    CMsgDOTATournamentInfo_Team::get_team_logo_for_reflect,
                    CMsgDOTATournamentInfo_Team::mut_team_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "eliminated",
                    CMsgDOTATournamentInfo_Team::get_eliminated_for_reflect,
                    CMsgDOTATournamentInfo_Team::mut_eliminated_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATournamentInfo_Team>(
                    "CMsgDOTATournamentInfo_Team",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATournamentInfo_Team {
    fn clear(&mut self) {
        self.clear_team_id();
        self.clear_name();
        self.clear_tag();
        self.clear_team_logo();
        self.clear_eliminated();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATournamentInfo_Team {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATournamentInfo_Team {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATournamentInfo_UpcomingMatch {
    // message fields
    series_id: ::std::option::Option<u32>,
    team1_id: ::std::option::Option<u32>,
    team2_id: ::std::option::Option<u32>,
    bo: ::std::option::Option<u32>,
    stage_name: ::protobuf::SingularField<::std::string::String>,
    start_time: ::std::option::Option<u32>,
    winner_stage: ::protobuf::SingularField<::std::string::String>,
    loser_stage: ::protobuf::SingularField<::std::string::String>,
    team1_tag: ::protobuf::SingularField<::std::string::String>,
    team2_tag: ::protobuf::SingularField<::std::string::String>,
    team1_prev_opponent_tag: ::protobuf::SingularField<::std::string::String>,
    team2_prev_opponent_tag: ::protobuf::SingularField<::std::string::String>,
    team1_logo: ::std::option::Option<u64>,
    team2_logo: ::std::option::Option<u64>,
    team1_prev_opponent_logo: ::std::option::Option<u64>,
    team2_prev_opponent_logo: ::std::option::Option<u64>,
    team1_prev_opponent_id: ::std::option::Option<u32>,
    team2_prev_opponent_id: ::std::option::Option<u32>,
    team1_prev_match_score: ::std::option::Option<u32>,
    team1_prev_match_opponent_score: ::std::option::Option<u32>,
    team2_prev_match_score: ::std::option::Option<u32>,
    team2_prev_match_opponent_score: ::std::option::Option<u32>,
    phase_type: ::std::option::Option<u32>,
    team1_score: ::std::option::Option<u32>,
    team2_score: ::std::option::Option<u32>,
    phase_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATournamentInfo_UpcomingMatch {}

impl CMsgDOTATournamentInfo_UpcomingMatch {
    pub fn new() -> CMsgDOTATournamentInfo_UpcomingMatch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATournamentInfo_UpcomingMatch {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATournamentInfo_UpcomingMatch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATournamentInfo_UpcomingMatch,
        };
        unsafe {
            instance.get(CMsgDOTATournamentInfo_UpcomingMatch::new)
        }
    }

    // optional uint32 series_id = 1;

    pub fn clear_series_id(&mut self) {
        self.series_id = ::std::option::Option::None;
    }

    pub fn has_series_id(&self) -> bool {
        self.series_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_series_id(&mut self, v: u32) {
        self.series_id = ::std::option::Option::Some(v);
    }

    pub fn get_series_id(&self) -> u32 {
        self.series_id.unwrap_or(0)
    }

    fn get_series_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.series_id
    }

    fn mut_series_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.series_id
    }

    // optional uint32 team1_id = 2;

    pub fn clear_team1_id(&mut self) {
        self.team1_id = ::std::option::Option::None;
    }

    pub fn has_team1_id(&self) -> bool {
        self.team1_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team1_id(&mut self, v: u32) {
        self.team1_id = ::std::option::Option::Some(v);
    }

    pub fn get_team1_id(&self) -> u32 {
        self.team1_id.unwrap_or(0)
    }

    fn get_team1_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team1_id
    }

    fn mut_team1_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team1_id
    }

    // optional uint32 team2_id = 3;

    pub fn clear_team2_id(&mut self) {
        self.team2_id = ::std::option::Option::None;
    }

    pub fn has_team2_id(&self) -> bool {
        self.team2_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team2_id(&mut self, v: u32) {
        self.team2_id = ::std::option::Option::Some(v);
    }

    pub fn get_team2_id(&self) -> u32 {
        self.team2_id.unwrap_or(0)
    }

    fn get_team2_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team2_id
    }

    fn mut_team2_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team2_id
    }

    // optional uint32 bo = 4;

    pub fn clear_bo(&mut self) {
        self.bo = ::std::option::Option::None;
    }

    pub fn has_bo(&self) -> bool {
        self.bo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bo(&mut self, v: u32) {
        self.bo = ::std::option::Option::Some(v);
    }

    pub fn get_bo(&self) -> u32 {
        self.bo.unwrap_or(0)
    }

    fn get_bo_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.bo
    }

    fn mut_bo_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.bo
    }

    // optional string stage_name = 5;

    pub fn clear_stage_name(&mut self) {
        self.stage_name.clear();
    }

    pub fn has_stage_name(&self) -> bool {
        self.stage_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stage_name(&mut self, v: ::std::string::String) {
        self.stage_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stage_name(&mut self) -> &mut ::std::string::String {
        if self.stage_name.is_none() {
            self.stage_name.set_default();
        }
        self.stage_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_stage_name(&mut self) -> ::std::string::String {
        self.stage_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_stage_name(&self) -> &str {
        match self.stage_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_stage_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.stage_name
    }

    fn mut_stage_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.stage_name
    }

    // optional uint32 start_time = 6;

    pub fn clear_start_time(&mut self) {
        self.start_time = ::std::option::Option::None;
    }

    pub fn has_start_time(&self) -> bool {
        self.start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_time(&mut self, v: u32) {
        self.start_time = ::std::option::Option::Some(v);
    }

    pub fn get_start_time(&self) -> u32 {
        self.start_time.unwrap_or(0)
    }

    fn get_start_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.start_time
    }

    fn mut_start_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.start_time
    }

    // optional string winner_stage = 7;

    pub fn clear_winner_stage(&mut self) {
        self.winner_stage.clear();
    }

    pub fn has_winner_stage(&self) -> bool {
        self.winner_stage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_winner_stage(&mut self, v: ::std::string::String) {
        self.winner_stage = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_winner_stage(&mut self) -> &mut ::std::string::String {
        if self.winner_stage.is_none() {
            self.winner_stage.set_default();
        }
        self.winner_stage.as_mut().unwrap()
    }

    // Take field
    pub fn take_winner_stage(&mut self) -> ::std::string::String {
        self.winner_stage.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_winner_stage(&self) -> &str {
        match self.winner_stage.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_winner_stage_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.winner_stage
    }

    fn mut_winner_stage_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.winner_stage
    }

    // optional string loser_stage = 8;

    pub fn clear_loser_stage(&mut self) {
        self.loser_stage.clear();
    }

    pub fn has_loser_stage(&self) -> bool {
        self.loser_stage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_loser_stage(&mut self, v: ::std::string::String) {
        self.loser_stage = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_loser_stage(&mut self) -> &mut ::std::string::String {
        if self.loser_stage.is_none() {
            self.loser_stage.set_default();
        }
        self.loser_stage.as_mut().unwrap()
    }

    // Take field
    pub fn take_loser_stage(&mut self) -> ::std::string::String {
        self.loser_stage.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_loser_stage(&self) -> &str {
        match self.loser_stage.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_loser_stage_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.loser_stage
    }

    fn mut_loser_stage_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.loser_stage
    }

    // optional string team1_tag = 9;

    pub fn clear_team1_tag(&mut self) {
        self.team1_tag.clear();
    }

    pub fn has_team1_tag(&self) -> bool {
        self.team1_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team1_tag(&mut self, v: ::std::string::String) {
        self.team1_tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team1_tag(&mut self) -> &mut ::std::string::String {
        if self.team1_tag.is_none() {
            self.team1_tag.set_default();
        }
        self.team1_tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_team1_tag(&mut self) -> ::std::string::String {
        self.team1_tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_team1_tag(&self) -> &str {
        match self.team1_tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_team1_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.team1_tag
    }

    fn mut_team1_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.team1_tag
    }

    // optional string team2_tag = 10;

    pub fn clear_team2_tag(&mut self) {
        self.team2_tag.clear();
    }

    pub fn has_team2_tag(&self) -> bool {
        self.team2_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team2_tag(&mut self, v: ::std::string::String) {
        self.team2_tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team2_tag(&mut self) -> &mut ::std::string::String {
        if self.team2_tag.is_none() {
            self.team2_tag.set_default();
        }
        self.team2_tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_team2_tag(&mut self) -> ::std::string::String {
        self.team2_tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_team2_tag(&self) -> &str {
        match self.team2_tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_team2_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.team2_tag
    }

    fn mut_team2_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.team2_tag
    }

    // optional string team1_prev_opponent_tag = 11;

    pub fn clear_team1_prev_opponent_tag(&mut self) {
        self.team1_prev_opponent_tag.clear();
    }

    pub fn has_team1_prev_opponent_tag(&self) -> bool {
        self.team1_prev_opponent_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team1_prev_opponent_tag(&mut self, v: ::std::string::String) {
        self.team1_prev_opponent_tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team1_prev_opponent_tag(&mut self) -> &mut ::std::string::String {
        if self.team1_prev_opponent_tag.is_none() {
            self.team1_prev_opponent_tag.set_default();
        }
        self.team1_prev_opponent_tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_team1_prev_opponent_tag(&mut self) -> ::std::string::String {
        self.team1_prev_opponent_tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_team1_prev_opponent_tag(&self) -> &str {
        match self.team1_prev_opponent_tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_team1_prev_opponent_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.team1_prev_opponent_tag
    }

    fn mut_team1_prev_opponent_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.team1_prev_opponent_tag
    }

    // optional string team2_prev_opponent_tag = 12;

    pub fn clear_team2_prev_opponent_tag(&mut self) {
        self.team2_prev_opponent_tag.clear();
    }

    pub fn has_team2_prev_opponent_tag(&self) -> bool {
        self.team2_prev_opponent_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team2_prev_opponent_tag(&mut self, v: ::std::string::String) {
        self.team2_prev_opponent_tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team2_prev_opponent_tag(&mut self) -> &mut ::std::string::String {
        if self.team2_prev_opponent_tag.is_none() {
            self.team2_prev_opponent_tag.set_default();
        }
        self.team2_prev_opponent_tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_team2_prev_opponent_tag(&mut self) -> ::std::string::String {
        self.team2_prev_opponent_tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_team2_prev_opponent_tag(&self) -> &str {
        match self.team2_prev_opponent_tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_team2_prev_opponent_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.team2_prev_opponent_tag
    }

    fn mut_team2_prev_opponent_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.team2_prev_opponent_tag
    }

    // optional uint64 team1_logo = 13;

    pub fn clear_team1_logo(&mut self) {
        self.team1_logo = ::std::option::Option::None;
    }

    pub fn has_team1_logo(&self) -> bool {
        self.team1_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team1_logo(&mut self, v: u64) {
        self.team1_logo = ::std::option::Option::Some(v);
    }

    pub fn get_team1_logo(&self) -> u64 {
        self.team1_logo.unwrap_or(0)
    }

    fn get_team1_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team1_logo
    }

    fn mut_team1_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team1_logo
    }

    // optional uint64 team2_logo = 14;

    pub fn clear_team2_logo(&mut self) {
        self.team2_logo = ::std::option::Option::None;
    }

    pub fn has_team2_logo(&self) -> bool {
        self.team2_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team2_logo(&mut self, v: u64) {
        self.team2_logo = ::std::option::Option::Some(v);
    }

    pub fn get_team2_logo(&self) -> u64 {
        self.team2_logo.unwrap_or(0)
    }

    fn get_team2_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team2_logo
    }

    fn mut_team2_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team2_logo
    }

    // optional uint64 team1_prev_opponent_logo = 15;

    pub fn clear_team1_prev_opponent_logo(&mut self) {
        self.team1_prev_opponent_logo = ::std::option::Option::None;
    }

    pub fn has_team1_prev_opponent_logo(&self) -> bool {
        self.team1_prev_opponent_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team1_prev_opponent_logo(&mut self, v: u64) {
        self.team1_prev_opponent_logo = ::std::option::Option::Some(v);
    }

    pub fn get_team1_prev_opponent_logo(&self) -> u64 {
        self.team1_prev_opponent_logo.unwrap_or(0)
    }

    fn get_team1_prev_opponent_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team1_prev_opponent_logo
    }

    fn mut_team1_prev_opponent_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team1_prev_opponent_logo
    }

    // optional uint64 team2_prev_opponent_logo = 16;

    pub fn clear_team2_prev_opponent_logo(&mut self) {
        self.team2_prev_opponent_logo = ::std::option::Option::None;
    }

    pub fn has_team2_prev_opponent_logo(&self) -> bool {
        self.team2_prev_opponent_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team2_prev_opponent_logo(&mut self, v: u64) {
        self.team2_prev_opponent_logo = ::std::option::Option::Some(v);
    }

    pub fn get_team2_prev_opponent_logo(&self) -> u64 {
        self.team2_prev_opponent_logo.unwrap_or(0)
    }

    fn get_team2_prev_opponent_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team2_prev_opponent_logo
    }

    fn mut_team2_prev_opponent_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team2_prev_opponent_logo
    }

    // optional uint32 team1_prev_opponent_id = 17;

    pub fn clear_team1_prev_opponent_id(&mut self) {
        self.team1_prev_opponent_id = ::std::option::Option::None;
    }

    pub fn has_team1_prev_opponent_id(&self) -> bool {
        self.team1_prev_opponent_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team1_prev_opponent_id(&mut self, v: u32) {
        self.team1_prev_opponent_id = ::std::option::Option::Some(v);
    }

    pub fn get_team1_prev_opponent_id(&self) -> u32 {
        self.team1_prev_opponent_id.unwrap_or(0)
    }

    fn get_team1_prev_opponent_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team1_prev_opponent_id
    }

    fn mut_team1_prev_opponent_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team1_prev_opponent_id
    }

    // optional uint32 team2_prev_opponent_id = 18;

    pub fn clear_team2_prev_opponent_id(&mut self) {
        self.team2_prev_opponent_id = ::std::option::Option::None;
    }

    pub fn has_team2_prev_opponent_id(&self) -> bool {
        self.team2_prev_opponent_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team2_prev_opponent_id(&mut self, v: u32) {
        self.team2_prev_opponent_id = ::std::option::Option::Some(v);
    }

    pub fn get_team2_prev_opponent_id(&self) -> u32 {
        self.team2_prev_opponent_id.unwrap_or(0)
    }

    fn get_team2_prev_opponent_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team2_prev_opponent_id
    }

    fn mut_team2_prev_opponent_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team2_prev_opponent_id
    }

    // optional uint32 team1_prev_match_score = 19;

    pub fn clear_team1_prev_match_score(&mut self) {
        self.team1_prev_match_score = ::std::option::Option::None;
    }

    pub fn has_team1_prev_match_score(&self) -> bool {
        self.team1_prev_match_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team1_prev_match_score(&mut self, v: u32) {
        self.team1_prev_match_score = ::std::option::Option::Some(v);
    }

    pub fn get_team1_prev_match_score(&self) -> u32 {
        self.team1_prev_match_score.unwrap_or(0)
    }

    fn get_team1_prev_match_score_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team1_prev_match_score
    }

    fn mut_team1_prev_match_score_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team1_prev_match_score
    }

    // optional uint32 team1_prev_match_opponent_score = 20;

    pub fn clear_team1_prev_match_opponent_score(&mut self) {
        self.team1_prev_match_opponent_score = ::std::option::Option::None;
    }

    pub fn has_team1_prev_match_opponent_score(&self) -> bool {
        self.team1_prev_match_opponent_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team1_prev_match_opponent_score(&mut self, v: u32) {
        self.team1_prev_match_opponent_score = ::std::option::Option::Some(v);
    }

    pub fn get_team1_prev_match_opponent_score(&self) -> u32 {
        self.team1_prev_match_opponent_score.unwrap_or(0)
    }

    fn get_team1_prev_match_opponent_score_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team1_prev_match_opponent_score
    }

    fn mut_team1_prev_match_opponent_score_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team1_prev_match_opponent_score
    }

    // optional uint32 team2_prev_match_score = 21;

    pub fn clear_team2_prev_match_score(&mut self) {
        self.team2_prev_match_score = ::std::option::Option::None;
    }

    pub fn has_team2_prev_match_score(&self) -> bool {
        self.team2_prev_match_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team2_prev_match_score(&mut self, v: u32) {
        self.team2_prev_match_score = ::std::option::Option::Some(v);
    }

    pub fn get_team2_prev_match_score(&self) -> u32 {
        self.team2_prev_match_score.unwrap_or(0)
    }

    fn get_team2_prev_match_score_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team2_prev_match_score
    }

    fn mut_team2_prev_match_score_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team2_prev_match_score
    }

    // optional uint32 team2_prev_match_opponent_score = 22;

    pub fn clear_team2_prev_match_opponent_score(&mut self) {
        self.team2_prev_match_opponent_score = ::std::option::Option::None;
    }

    pub fn has_team2_prev_match_opponent_score(&self) -> bool {
        self.team2_prev_match_opponent_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team2_prev_match_opponent_score(&mut self, v: u32) {
        self.team2_prev_match_opponent_score = ::std::option::Option::Some(v);
    }

    pub fn get_team2_prev_match_opponent_score(&self) -> u32 {
        self.team2_prev_match_opponent_score.unwrap_or(0)
    }

    fn get_team2_prev_match_opponent_score_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team2_prev_match_opponent_score
    }

    fn mut_team2_prev_match_opponent_score_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team2_prev_match_opponent_score
    }

    // optional uint32 phase_type = 23;

    pub fn clear_phase_type(&mut self) {
        self.phase_type = ::std::option::Option::None;
    }

    pub fn has_phase_type(&self) -> bool {
        self.phase_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_phase_type(&mut self, v: u32) {
        self.phase_type = ::std::option::Option::Some(v);
    }

    pub fn get_phase_type(&self) -> u32 {
        self.phase_type.unwrap_or(0)
    }

    fn get_phase_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.phase_type
    }

    fn mut_phase_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.phase_type
    }

    // optional uint32 team1_score = 24;

    pub fn clear_team1_score(&mut self) {
        self.team1_score = ::std::option::Option::None;
    }

    pub fn has_team1_score(&self) -> bool {
        self.team1_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team1_score(&mut self, v: u32) {
        self.team1_score = ::std::option::Option::Some(v);
    }

    pub fn get_team1_score(&self) -> u32 {
        self.team1_score.unwrap_or(0)
    }

    fn get_team1_score_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team1_score
    }

    fn mut_team1_score_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team1_score
    }

    // optional uint32 team2_score = 25;

    pub fn clear_team2_score(&mut self) {
        self.team2_score = ::std::option::Option::None;
    }

    pub fn has_team2_score(&self) -> bool {
        self.team2_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team2_score(&mut self, v: u32) {
        self.team2_score = ::std::option::Option::Some(v);
    }

    pub fn get_team2_score(&self) -> u32 {
        self.team2_score.unwrap_or(0)
    }

    fn get_team2_score_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team2_score
    }

    fn mut_team2_score_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team2_score
    }

    // optional uint32 phase_id = 26;

    pub fn clear_phase_id(&mut self) {
        self.phase_id = ::std::option::Option::None;
    }

    pub fn has_phase_id(&self) -> bool {
        self.phase_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_phase_id(&mut self, v: u32) {
        self.phase_id = ::std::option::Option::Some(v);
    }

    pub fn get_phase_id(&self) -> u32 {
        self.phase_id.unwrap_or(0)
    }

    fn get_phase_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.phase_id
    }

    fn mut_phase_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.phase_id
    }
}

impl ::protobuf::Message for CMsgDOTATournamentInfo_UpcomingMatch {
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
                    self.series_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team1_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team2_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.bo = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.stage_name)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.start_time = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.winner_stage)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.loser_stage)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team1_tag)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team2_tag)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team1_prev_opponent_tag)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team2_prev_opponent_tag)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.team1_logo = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.team2_logo = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.team1_prev_opponent_logo = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.team2_prev_opponent_logo = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team1_prev_opponent_id = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team2_prev_opponent_id = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team1_prev_match_score = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team1_prev_match_opponent_score = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team2_prev_match_score = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team2_prev_match_opponent_score = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.phase_type = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team1_score = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team2_score = ::std::option::Option::Some(tmp);
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.phase_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.series_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team1_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team2_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.bo {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.stage_name.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(v) = self.start_time {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.winner_stage.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        if let Some(ref v) = self.loser_stage.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(ref v) = self.team1_tag.as_ref() {
            my_size += ::protobuf::rt::string_size(9, &v);
        }
        if let Some(ref v) = self.team2_tag.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        if let Some(ref v) = self.team1_prev_opponent_tag.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        }
        if let Some(ref v) = self.team2_prev_opponent_tag.as_ref() {
            my_size += ::protobuf::rt::string_size(12, &v);
        }
        if let Some(v) = self.team1_logo {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team2_logo {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team1_prev_opponent_logo {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team2_prev_opponent_logo {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team1_prev_opponent_id {
            my_size += ::protobuf::rt::value_size(17, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team2_prev_opponent_id {
            my_size += ::protobuf::rt::value_size(18, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team1_prev_match_score {
            my_size += ::protobuf::rt::value_size(19, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team1_prev_match_opponent_score {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team2_prev_match_score {
            my_size += ::protobuf::rt::value_size(21, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team2_prev_match_opponent_score {
            my_size += ::protobuf::rt::value_size(22, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.phase_type {
            my_size += ::protobuf::rt::value_size(23, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team1_score {
            my_size += ::protobuf::rt::value_size(24, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team2_score {
            my_size += ::protobuf::rt::value_size(25, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.phase_id {
            my_size += ::protobuf::rt::value_size(26, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.series_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.team1_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.team2_id {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.bo {
            os.write_uint32(4, v)?;
        }
        if let Some(ref v) = self.stage_name.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(v) = self.start_time {
            os.write_uint32(6, v)?;
        }
        if let Some(ref v) = self.winner_stage.as_ref() {
            os.write_string(7, &v)?;
        }
        if let Some(ref v) = self.loser_stage.as_ref() {
            os.write_string(8, &v)?;
        }
        if let Some(ref v) = self.team1_tag.as_ref() {
            os.write_string(9, &v)?;
        }
        if let Some(ref v) = self.team2_tag.as_ref() {
            os.write_string(10, &v)?;
        }
        if let Some(ref v) = self.team1_prev_opponent_tag.as_ref() {
            os.write_string(11, &v)?;
        }
        if let Some(ref v) = self.team2_prev_opponent_tag.as_ref() {
            os.write_string(12, &v)?;
        }
        if let Some(v) = self.team1_logo {
            os.write_uint64(13, v)?;
        }
        if let Some(v) = self.team2_logo {
            os.write_uint64(14, v)?;
        }
        if let Some(v) = self.team1_prev_opponent_logo {
            os.write_uint64(15, v)?;
        }
        if let Some(v) = self.team2_prev_opponent_logo {
            os.write_uint64(16, v)?;
        }
        if let Some(v) = self.team1_prev_opponent_id {
            os.write_uint32(17, v)?;
        }
        if let Some(v) = self.team2_prev_opponent_id {
            os.write_uint32(18, v)?;
        }
        if let Some(v) = self.team1_prev_match_score {
            os.write_uint32(19, v)?;
        }
        if let Some(v) = self.team1_prev_match_opponent_score {
            os.write_uint32(20, v)?;
        }
        if let Some(v) = self.team2_prev_match_score {
            os.write_uint32(21, v)?;
        }
        if let Some(v) = self.team2_prev_match_opponent_score {
            os.write_uint32(22, v)?;
        }
        if let Some(v) = self.phase_type {
            os.write_uint32(23, v)?;
        }
        if let Some(v) = self.team1_score {
            os.write_uint32(24, v)?;
        }
        if let Some(v) = self.team2_score {
            os.write_uint32(25, v)?;
        }
        if let Some(v) = self.phase_id {
            os.write_uint32(26, v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTATournamentInfo_UpcomingMatch {
    fn new() -> CMsgDOTATournamentInfo_UpcomingMatch {
        CMsgDOTATournamentInfo_UpcomingMatch::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATournamentInfo_UpcomingMatch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "series_id",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_series_id_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_series_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team1_id",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team1_id_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team1_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team2_id",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team2_id_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team2_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bo",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_bo_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_bo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "stage_name",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_stage_name_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_stage_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "start_time",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_start_time_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_start_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "winner_stage",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_winner_stage_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_winner_stage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "loser_stage",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_loser_stage_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_loser_stage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team1_tag",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team1_tag_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team1_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team2_tag",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team2_tag_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team2_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team1_prev_opponent_tag",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team1_prev_opponent_tag_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team1_prev_opponent_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team2_prev_opponent_tag",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team2_prev_opponent_tag_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team2_prev_opponent_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "team1_logo",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team1_logo_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team1_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "team2_logo",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team2_logo_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team2_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "team1_prev_opponent_logo",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team1_prev_opponent_logo_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team1_prev_opponent_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "team2_prev_opponent_logo",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team2_prev_opponent_logo_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team2_prev_opponent_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team1_prev_opponent_id",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team1_prev_opponent_id_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team1_prev_opponent_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team2_prev_opponent_id",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team2_prev_opponent_id_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team2_prev_opponent_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team1_prev_match_score",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team1_prev_match_score_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team1_prev_match_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team1_prev_match_opponent_score",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team1_prev_match_opponent_score_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team1_prev_match_opponent_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team2_prev_match_score",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team2_prev_match_score_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team2_prev_match_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team2_prev_match_opponent_score",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team2_prev_match_opponent_score_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team2_prev_match_opponent_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "phase_type",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_phase_type_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_phase_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team1_score",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team1_score_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team1_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team2_score",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_team2_score_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_team2_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "phase_id",
                    CMsgDOTATournamentInfo_UpcomingMatch::get_phase_id_for_reflect,
                    CMsgDOTATournamentInfo_UpcomingMatch::mut_phase_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATournamentInfo_UpcomingMatch>(
                    "CMsgDOTATournamentInfo_UpcomingMatch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATournamentInfo_UpcomingMatch {
    fn clear(&mut self) {
        self.clear_series_id();
        self.clear_team1_id();
        self.clear_team2_id();
        self.clear_bo();
        self.clear_stage_name();
        self.clear_start_time();
        self.clear_winner_stage();
        self.clear_loser_stage();
        self.clear_team1_tag();
        self.clear_team2_tag();
        self.clear_team1_prev_opponent_tag();
        self.clear_team2_prev_opponent_tag();
        self.clear_team1_logo();
        self.clear_team2_logo();
        self.clear_team1_prev_opponent_logo();
        self.clear_team2_prev_opponent_logo();
        self.clear_team1_prev_opponent_id();
        self.clear_team2_prev_opponent_id();
        self.clear_team1_prev_match_score();
        self.clear_team1_prev_match_opponent_score();
        self.clear_team2_prev_match_score();
        self.clear_team2_prev_match_opponent_score();
        self.clear_phase_type();
        self.clear_team1_score();
        self.clear_team2_score();
        self.clear_phase_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATournamentInfo_UpcomingMatch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATournamentInfo_UpcomingMatch {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATournamentInfo_News {
    // message fields
    link: ::protobuf::SingularField<::std::string::String>,
    title: ::protobuf::SingularField<::std::string::String>,
    image: ::protobuf::SingularField<::std::string::String>,
    timestamp: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATournamentInfo_News {}

impl CMsgDOTATournamentInfo_News {
    pub fn new() -> CMsgDOTATournamentInfo_News {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATournamentInfo_News {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATournamentInfo_News> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATournamentInfo_News,
        };
        unsafe {
            instance.get(CMsgDOTATournamentInfo_News::new)
        }
    }

    // optional string link = 1;

    pub fn clear_link(&mut self) {
        self.link.clear();
    }

    pub fn has_link(&self) -> bool {
        self.link.is_some()
    }

    // Param is passed by value, moved
    pub fn set_link(&mut self, v: ::std::string::String) {
        self.link = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_link(&mut self) -> &mut ::std::string::String {
        if self.link.is_none() {
            self.link.set_default();
        }
        self.link.as_mut().unwrap()
    }

    // Take field
    pub fn take_link(&mut self) -> ::std::string::String {
        self.link.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_link(&self) -> &str {
        match self.link.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_link_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.link
    }

    fn mut_link_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.link
    }

    // optional string title = 2;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    pub fn has_title(&self) -> bool {
        self.title.is_some()
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        if self.title.is_none() {
            self.title.set_default();
        }
        self.title.as_mut().unwrap()
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        self.title.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        match self.title.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_title_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.title
    }

    // optional string image = 3;

    pub fn clear_image(&mut self) {
        self.image.clear();
    }

    pub fn has_image(&self) -> bool {
        self.image.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: ::std::string::String) {
        self.image = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image(&mut self) -> &mut ::std::string::String {
        if self.image.is_none() {
            self.image.set_default();
        }
        self.image.as_mut().unwrap()
    }

    // Take field
    pub fn take_image(&mut self) -> ::std::string::String {
        self.image.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_image(&self) -> &str {
        match self.image.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_image_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.image
    }

    fn mut_image_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.image
    }

    // optional uint32 timestamp = 4;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u32) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> u32 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.timestamp
    }
}

impl ::protobuf::Message for CMsgDOTATournamentInfo_News {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.link)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.title)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.image)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.link.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.title.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.image.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.link.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.title.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.image.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.timestamp {
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

impl ::protobuf::MessageStatic for CMsgDOTATournamentInfo_News {
    fn new() -> CMsgDOTATournamentInfo_News {
        CMsgDOTATournamentInfo_News::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATournamentInfo_News>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "link",
                    CMsgDOTATournamentInfo_News::get_link_for_reflect,
                    CMsgDOTATournamentInfo_News::mut_link_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    CMsgDOTATournamentInfo_News::get_title_for_reflect,
                    CMsgDOTATournamentInfo_News::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "image",
                    CMsgDOTATournamentInfo_News::get_image_for_reflect,
                    CMsgDOTATournamentInfo_News::mut_image_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timestamp",
                    CMsgDOTATournamentInfo_News::get_timestamp_for_reflect,
                    CMsgDOTATournamentInfo_News::mut_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATournamentInfo_News>(
                    "CMsgDOTATournamentInfo_News",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATournamentInfo_News {
    fn clear(&mut self) {
        self.clear_link();
        self.clear_title();
        self.clear_image();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATournamentInfo_News {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATournamentInfo_News {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRequestWeekendTourneySchedule {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRequestWeekendTourneySchedule {}

impl CMsgRequestWeekendTourneySchedule {
    pub fn new() -> CMsgRequestWeekendTourneySchedule {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRequestWeekendTourneySchedule {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRequestWeekendTourneySchedule> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRequestWeekendTourneySchedule,
        };
        unsafe {
            instance.get(CMsgRequestWeekendTourneySchedule::new)
        }
    }
}

impl ::protobuf::Message for CMsgRequestWeekendTourneySchedule {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for CMsgRequestWeekendTourneySchedule {
    fn new() -> CMsgRequestWeekendTourneySchedule {
        CMsgRequestWeekendTourneySchedule::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRequestWeekendTourneySchedule>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRequestWeekendTourneySchedule>(
                    "CMsgRequestWeekendTourneySchedule",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRequestWeekendTourneySchedule {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRequestWeekendTourneySchedule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRequestWeekendTourneySchedule {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgWeekendTourneySchedule {
    // message fields
    divisions: ::protobuf::RepeatedField<CMsgWeekendTourneySchedule_Division>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgWeekendTourneySchedule {}

impl CMsgWeekendTourneySchedule {
    pub fn new() -> CMsgWeekendTourneySchedule {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgWeekendTourneySchedule {
        static mut instance: ::protobuf::lazy::Lazy<CMsgWeekendTourneySchedule> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgWeekendTourneySchedule,
        };
        unsafe {
            instance.get(CMsgWeekendTourneySchedule::new)
        }
    }

    // repeated .CMsgWeekendTourneySchedule.Division divisions = 1;

    pub fn clear_divisions(&mut self) {
        self.divisions.clear();
    }

    // Param is passed by value, moved
    pub fn set_divisions(&mut self, v: ::protobuf::RepeatedField<CMsgWeekendTourneySchedule_Division>) {
        self.divisions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_divisions(&mut self) -> &mut ::protobuf::RepeatedField<CMsgWeekendTourneySchedule_Division> {
        &mut self.divisions
    }

    // Take field
    pub fn take_divisions(&mut self) -> ::protobuf::RepeatedField<CMsgWeekendTourneySchedule_Division> {
        ::std::mem::replace(&mut self.divisions, ::protobuf::RepeatedField::new())
    }

    pub fn get_divisions(&self) -> &[CMsgWeekendTourneySchedule_Division] {
        &self.divisions
    }

    fn get_divisions_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgWeekendTourneySchedule_Division> {
        &self.divisions
    }

    fn mut_divisions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgWeekendTourneySchedule_Division> {
        &mut self.divisions
    }
}

impl ::protobuf::Message for CMsgWeekendTourneySchedule {
    fn is_initialized(&self) -> bool {
        for v in &self.divisions {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.divisions)?;
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
        for value in &self.divisions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.divisions {
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

impl ::protobuf::MessageStatic for CMsgWeekendTourneySchedule {
    fn new() -> CMsgWeekendTourneySchedule {
        CMsgWeekendTourneySchedule::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgWeekendTourneySchedule>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgWeekendTourneySchedule_Division>>(
                    "divisions",
                    CMsgWeekendTourneySchedule::get_divisions_for_reflect,
                    CMsgWeekendTourneySchedule::mut_divisions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgWeekendTourneySchedule>(
                    "CMsgWeekendTourneySchedule",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgWeekendTourneySchedule {
    fn clear(&mut self) {
        self.clear_divisions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgWeekendTourneySchedule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgWeekendTourneySchedule {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgWeekendTourneySchedule_Division {
    // message fields
    division_code: ::std::option::Option<u32>,
    time_window_open: ::std::option::Option<u32>,
    time_window_close: ::std::option::Option<u32>,
    time_window_open_next: ::std::option::Option<u32>,
    trophy_id: ::std::option::Option<u32>,
    free_weekend: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgWeekendTourneySchedule_Division {}

impl CMsgWeekendTourneySchedule_Division {
    pub fn new() -> CMsgWeekendTourneySchedule_Division {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgWeekendTourneySchedule_Division {
        static mut instance: ::protobuf::lazy::Lazy<CMsgWeekendTourneySchedule_Division> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgWeekendTourneySchedule_Division,
        };
        unsafe {
            instance.get(CMsgWeekendTourneySchedule_Division::new)
        }
    }

    // optional uint32 division_code = 1;

    pub fn clear_division_code(&mut self) {
        self.division_code = ::std::option::Option::None;
    }

    pub fn has_division_code(&self) -> bool {
        self.division_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_division_code(&mut self, v: u32) {
        self.division_code = ::std::option::Option::Some(v);
    }

    pub fn get_division_code(&self) -> u32 {
        self.division_code.unwrap_or(0)
    }

    fn get_division_code_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.division_code
    }

    fn mut_division_code_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.division_code
    }

    // optional uint32 time_window_open = 2;

    pub fn clear_time_window_open(&mut self) {
        self.time_window_open = ::std::option::Option::None;
    }

    pub fn has_time_window_open(&self) -> bool {
        self.time_window_open.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_window_open(&mut self, v: u32) {
        self.time_window_open = ::std::option::Option::Some(v);
    }

    pub fn get_time_window_open(&self) -> u32 {
        self.time_window_open.unwrap_or(0)
    }

    fn get_time_window_open_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_window_open
    }

    fn mut_time_window_open_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_window_open
    }

    // optional uint32 time_window_close = 3;

    pub fn clear_time_window_close(&mut self) {
        self.time_window_close = ::std::option::Option::None;
    }

    pub fn has_time_window_close(&self) -> bool {
        self.time_window_close.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_window_close(&mut self, v: u32) {
        self.time_window_close = ::std::option::Option::Some(v);
    }

    pub fn get_time_window_close(&self) -> u32 {
        self.time_window_close.unwrap_or(0)
    }

    fn get_time_window_close_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_window_close
    }

    fn mut_time_window_close_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_window_close
    }

    // optional uint32 time_window_open_next = 4;

    pub fn clear_time_window_open_next(&mut self) {
        self.time_window_open_next = ::std::option::Option::None;
    }

    pub fn has_time_window_open_next(&self) -> bool {
        self.time_window_open_next.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_window_open_next(&mut self, v: u32) {
        self.time_window_open_next = ::std::option::Option::Some(v);
    }

    pub fn get_time_window_open_next(&self) -> u32 {
        self.time_window_open_next.unwrap_or(0)
    }

    fn get_time_window_open_next_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_window_open_next
    }

    fn mut_time_window_open_next_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_window_open_next
    }

    // optional uint32 trophy_id = 5;

    pub fn clear_trophy_id(&mut self) {
        self.trophy_id = ::std::option::Option::None;
    }

    pub fn has_trophy_id(&self) -> bool {
        self.trophy_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trophy_id(&mut self, v: u32) {
        self.trophy_id = ::std::option::Option::Some(v);
    }

    pub fn get_trophy_id(&self) -> u32 {
        self.trophy_id.unwrap_or(0)
    }

    fn get_trophy_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.trophy_id
    }

    fn mut_trophy_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.trophy_id
    }

    // optional bool free_weekend = 6;

    pub fn clear_free_weekend(&mut self) {
        self.free_weekend = ::std::option::Option::None;
    }

    pub fn has_free_weekend(&self) -> bool {
        self.free_weekend.is_some()
    }

    // Param is passed by value, moved
    pub fn set_free_weekend(&mut self, v: bool) {
        self.free_weekend = ::std::option::Option::Some(v);
    }

    pub fn get_free_weekend(&self) -> bool {
        self.free_weekend.unwrap_or(false)
    }

    fn get_free_weekend_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.free_weekend
    }

    fn mut_free_weekend_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.free_weekend
    }
}

impl ::protobuf::Message for CMsgWeekendTourneySchedule_Division {
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
                    self.division_code = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.time_window_open = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.time_window_close = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.time_window_open_next = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.trophy_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.free_weekend = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.division_code {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.time_window_open {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.time_window_close {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.time_window_open_next {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.trophy_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.free_weekend {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.division_code {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.time_window_open {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.time_window_close {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.time_window_open_next {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.trophy_id {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.free_weekend {
            os.write_bool(6, v)?;
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

impl ::protobuf::MessageStatic for CMsgWeekendTourneySchedule_Division {
    fn new() -> CMsgWeekendTourneySchedule_Division {
        CMsgWeekendTourneySchedule_Division::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgWeekendTourneySchedule_Division>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "division_code",
                    CMsgWeekendTourneySchedule_Division::get_division_code_for_reflect,
                    CMsgWeekendTourneySchedule_Division::mut_division_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_window_open",
                    CMsgWeekendTourneySchedule_Division::get_time_window_open_for_reflect,
                    CMsgWeekendTourneySchedule_Division::mut_time_window_open_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_window_close",
                    CMsgWeekendTourneySchedule_Division::get_time_window_close_for_reflect,
                    CMsgWeekendTourneySchedule_Division::mut_time_window_close_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_window_open_next",
                    CMsgWeekendTourneySchedule_Division::get_time_window_open_next_for_reflect,
                    CMsgWeekendTourneySchedule_Division::mut_time_window_open_next_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "trophy_id",
                    CMsgWeekendTourneySchedule_Division::get_trophy_id_for_reflect,
                    CMsgWeekendTourneySchedule_Division::mut_trophy_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "free_weekend",
                    CMsgWeekendTourneySchedule_Division::get_free_weekend_for_reflect,
                    CMsgWeekendTourneySchedule_Division::mut_free_weekend_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgWeekendTourneySchedule_Division>(
                    "CMsgWeekendTourneySchedule_Division",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgWeekendTourneySchedule_Division {
    fn clear(&mut self) {
        self.clear_division_code();
        self.clear_time_window_open();
        self.clear_time_window_close();
        self.clear_time_window_open_next();
        self.clear_trophy_id();
        self.clear_free_weekend();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgWeekendTourneySchedule_Division {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgWeekendTourneySchedule_Division {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgWeekendTourneyOpts {
    // message fields
    participating: ::std::option::Option<bool>,
    division_id: ::std::option::Option<u32>,
    buyin: ::std::option::Option<u32>,
    skill_level: ::std::option::Option<u32>,
    match_groups: ::std::option::Option<u32>,
    team_id: ::std::option::Option<u32>,
    pickup_team_name: ::protobuf::SingularField<::std::string::String>,
    pickup_team_logo: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgWeekendTourneyOpts {}

impl CMsgWeekendTourneyOpts {
    pub fn new() -> CMsgWeekendTourneyOpts {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgWeekendTourneyOpts {
        static mut instance: ::protobuf::lazy::Lazy<CMsgWeekendTourneyOpts> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgWeekendTourneyOpts,
        };
        unsafe {
            instance.get(CMsgWeekendTourneyOpts::new)
        }
    }

    // optional bool participating = 1;

    pub fn clear_participating(&mut self) {
        self.participating = ::std::option::Option::None;
    }

    pub fn has_participating(&self) -> bool {
        self.participating.is_some()
    }

    // Param is passed by value, moved
    pub fn set_participating(&mut self, v: bool) {
        self.participating = ::std::option::Option::Some(v);
    }

    pub fn get_participating(&self) -> bool {
        self.participating.unwrap_or(false)
    }

    fn get_participating_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.participating
    }

    fn mut_participating_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.participating
    }

    // optional uint32 division_id = 2;

    pub fn clear_division_id(&mut self) {
        self.division_id = ::std::option::Option::None;
    }

    pub fn has_division_id(&self) -> bool {
        self.division_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_division_id(&mut self, v: u32) {
        self.division_id = ::std::option::Option::Some(v);
    }

    pub fn get_division_id(&self) -> u32 {
        self.division_id.unwrap_or(0)
    }

    fn get_division_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.division_id
    }

    fn mut_division_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.division_id
    }

    // optional uint32 buyin = 3;

    pub fn clear_buyin(&mut self) {
        self.buyin = ::std::option::Option::None;
    }

    pub fn has_buyin(&self) -> bool {
        self.buyin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buyin(&mut self, v: u32) {
        self.buyin = ::std::option::Option::Some(v);
    }

    pub fn get_buyin(&self) -> u32 {
        self.buyin.unwrap_or(0)
    }

    fn get_buyin_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.buyin
    }

    fn mut_buyin_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.buyin
    }

    // optional uint32 skill_level = 4;

    pub fn clear_skill_level(&mut self) {
        self.skill_level = ::std::option::Option::None;
    }

    pub fn has_skill_level(&self) -> bool {
        self.skill_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_skill_level(&mut self, v: u32) {
        self.skill_level = ::std::option::Option::Some(v);
    }

    pub fn get_skill_level(&self) -> u32 {
        self.skill_level.unwrap_or(0)
    }

    fn get_skill_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.skill_level
    }

    fn mut_skill_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.skill_level
    }

    // optional uint32 match_groups = 5;

    pub fn clear_match_groups(&mut self) {
        self.match_groups = ::std::option::Option::None;
    }

    pub fn has_match_groups(&self) -> bool {
        self.match_groups.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_groups(&mut self, v: u32) {
        self.match_groups = ::std::option::Option::Some(v);
    }

    pub fn get_match_groups(&self) -> u32 {
        self.match_groups.unwrap_or(0)
    }

    fn get_match_groups_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.match_groups
    }

    fn mut_match_groups_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.match_groups
    }

    // optional uint32 team_id = 6;

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

    // optional string pickup_team_name = 7;

    pub fn clear_pickup_team_name(&mut self) {
        self.pickup_team_name.clear();
    }

    pub fn has_pickup_team_name(&self) -> bool {
        self.pickup_team_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pickup_team_name(&mut self, v: ::std::string::String) {
        self.pickup_team_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pickup_team_name(&mut self) -> &mut ::std::string::String {
        if self.pickup_team_name.is_none() {
            self.pickup_team_name.set_default();
        }
        self.pickup_team_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_pickup_team_name(&mut self) -> ::std::string::String {
        self.pickup_team_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_pickup_team_name(&self) -> &str {
        match self.pickup_team_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_pickup_team_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.pickup_team_name
    }

    fn mut_pickup_team_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.pickup_team_name
    }

    // optional uint64 pickup_team_logo = 8;

    pub fn clear_pickup_team_logo(&mut self) {
        self.pickup_team_logo = ::std::option::Option::None;
    }

    pub fn has_pickup_team_logo(&self) -> bool {
        self.pickup_team_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pickup_team_logo(&mut self, v: u64) {
        self.pickup_team_logo = ::std::option::Option::Some(v);
    }

    pub fn get_pickup_team_logo(&self) -> u64 {
        self.pickup_team_logo.unwrap_or(0)
    }

    fn get_pickup_team_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.pickup_team_logo
    }

    fn mut_pickup_team_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.pickup_team_logo
    }
}

impl ::protobuf::Message for CMsgWeekendTourneyOpts {
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
                    self.participating = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.division_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.buyin = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.skill_level = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.match_groups = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team_id = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.pickup_team_name)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.pickup_team_logo = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.participating {
            my_size += 2;
        }
        if let Some(v) = self.division_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.buyin {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.skill_level {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.match_groups {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.pickup_team_name.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        if let Some(v) = self.pickup_team_logo {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.participating {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.division_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.buyin {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.skill_level {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.match_groups {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.team_id {
            os.write_uint32(6, v)?;
        }
        if let Some(ref v) = self.pickup_team_name.as_ref() {
            os.write_string(7, &v)?;
        }
        if let Some(v) = self.pickup_team_logo {
            os.write_uint64(8, v)?;
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

impl ::protobuf::MessageStatic for CMsgWeekendTourneyOpts {
    fn new() -> CMsgWeekendTourneyOpts {
        CMsgWeekendTourneyOpts::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgWeekendTourneyOpts>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "participating",
                    CMsgWeekendTourneyOpts::get_participating_for_reflect,
                    CMsgWeekendTourneyOpts::mut_participating_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "division_id",
                    CMsgWeekendTourneyOpts::get_division_id_for_reflect,
                    CMsgWeekendTourneyOpts::mut_division_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "buyin",
                    CMsgWeekendTourneyOpts::get_buyin_for_reflect,
                    CMsgWeekendTourneyOpts::mut_buyin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "skill_level",
                    CMsgWeekendTourneyOpts::get_skill_level_for_reflect,
                    CMsgWeekendTourneyOpts::mut_skill_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "match_groups",
                    CMsgWeekendTourneyOpts::get_match_groups_for_reflect,
                    CMsgWeekendTourneyOpts::mut_match_groups_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgWeekendTourneyOpts::get_team_id_for_reflect,
                    CMsgWeekendTourneyOpts::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pickup_team_name",
                    CMsgWeekendTourneyOpts::get_pickup_team_name_for_reflect,
                    CMsgWeekendTourneyOpts::mut_pickup_team_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "pickup_team_logo",
                    CMsgWeekendTourneyOpts::get_pickup_team_logo_for_reflect,
                    CMsgWeekendTourneyOpts::mut_pickup_team_logo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgWeekendTourneyOpts>(
                    "CMsgWeekendTourneyOpts",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgWeekendTourneyOpts {
    fn clear(&mut self) {
        self.clear_participating();
        self.clear_division_id();
        self.clear_buyin();
        self.clear_skill_level();
        self.clear_match_groups();
        self.clear_team_id();
        self.clear_pickup_team_name();
        self.clear_pickup_team_logo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgWeekendTourneyOpts {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgWeekendTourneyOpts {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgWeekendTourneyLeave {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgWeekendTourneyLeave {}

impl CMsgWeekendTourneyLeave {
    pub fn new() -> CMsgWeekendTourneyLeave {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgWeekendTourneyLeave {
        static mut instance: ::protobuf::lazy::Lazy<CMsgWeekendTourneyLeave> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgWeekendTourneyLeave,
        };
        unsafe {
            instance.get(CMsgWeekendTourneyLeave::new)
        }
    }
}

impl ::protobuf::Message for CMsgWeekendTourneyLeave {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for CMsgWeekendTourneyLeave {
    fn new() -> CMsgWeekendTourneyLeave {
        CMsgWeekendTourneyLeave::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgWeekendTourneyLeave>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgWeekendTourneyLeave>(
                    "CMsgWeekendTourneyLeave",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgWeekendTourneyLeave {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgWeekendTourneyLeave {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgWeekendTourneyLeave {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATournament {
    // message fields
    tournament_id: ::std::option::Option<u32>,
    division_id: ::std::option::Option<u32>,
    schedule_time: ::std::option::Option<u32>,
    skill_level: ::std::option::Option<u32>,
    tournament_template: ::std::option::Option<super::dota_client_enums::ETournamentTemplate>,
    state: ::std::option::Option<super::dota_client_enums::ETournamentState>,
    state_seq_num: ::std::option::Option<u32>,
    season_trophy_id: ::std::option::Option<u32>,
    teams: ::protobuf::RepeatedField<CMsgDOTATournament_Team>,
    games: ::protobuf::RepeatedField<CMsgDOTATournament_Game>,
    nodes: ::protobuf::RepeatedField<CMsgDOTATournament_Node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATournament {}

impl CMsgDOTATournament {
    pub fn new() -> CMsgDOTATournament {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATournament {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATournament> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATournament,
        };
        unsafe {
            instance.get(CMsgDOTATournament::new)
        }
    }

    // optional uint32 tournament_id = 1;

    pub fn clear_tournament_id(&mut self) {
        self.tournament_id = ::std::option::Option::None;
    }

    pub fn has_tournament_id(&self) -> bool {
        self.tournament_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tournament_id(&mut self, v: u32) {
        self.tournament_id = ::std::option::Option::Some(v);
    }

    pub fn get_tournament_id(&self) -> u32 {
        self.tournament_id.unwrap_or(0)
    }

    fn get_tournament_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tournament_id
    }

    fn mut_tournament_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tournament_id
    }

    // optional uint32 division_id = 2;

    pub fn clear_division_id(&mut self) {
        self.division_id = ::std::option::Option::None;
    }

    pub fn has_division_id(&self) -> bool {
        self.division_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_division_id(&mut self, v: u32) {
        self.division_id = ::std::option::Option::Some(v);
    }

    pub fn get_division_id(&self) -> u32 {
        self.division_id.unwrap_or(0)
    }

    fn get_division_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.division_id
    }

    fn mut_division_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.division_id
    }

    // optional uint32 schedule_time = 3;

    pub fn clear_schedule_time(&mut self) {
        self.schedule_time = ::std::option::Option::None;
    }

    pub fn has_schedule_time(&self) -> bool {
        self.schedule_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schedule_time(&mut self, v: u32) {
        self.schedule_time = ::std::option::Option::Some(v);
    }

    pub fn get_schedule_time(&self) -> u32 {
        self.schedule_time.unwrap_or(0)
    }

    fn get_schedule_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.schedule_time
    }

    fn mut_schedule_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.schedule_time
    }

    // optional uint32 skill_level = 4;

    pub fn clear_skill_level(&mut self) {
        self.skill_level = ::std::option::Option::None;
    }

    pub fn has_skill_level(&self) -> bool {
        self.skill_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_skill_level(&mut self, v: u32) {
        self.skill_level = ::std::option::Option::Some(v);
    }

    pub fn get_skill_level(&self) -> u32 {
        self.skill_level.unwrap_or(0)
    }

    fn get_skill_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.skill_level
    }

    fn mut_skill_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.skill_level
    }

    // optional .ETournamentTemplate tournament_template = 5;

    pub fn clear_tournament_template(&mut self) {
        self.tournament_template = ::std::option::Option::None;
    }

    pub fn has_tournament_template(&self) -> bool {
        self.tournament_template.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tournament_template(&mut self, v: super::dota_client_enums::ETournamentTemplate) {
        self.tournament_template = ::std::option::Option::Some(v);
    }

    pub fn get_tournament_template(&self) -> super::dota_client_enums::ETournamentTemplate {
        self.tournament_template.unwrap_or(super::dota_client_enums::ETournamentTemplate::k_ETournamentTemplate_None)
    }

    fn get_tournament_template_for_reflect(&self) -> &::std::option::Option<super::dota_client_enums::ETournamentTemplate> {
        &self.tournament_template
    }

    fn mut_tournament_template_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_client_enums::ETournamentTemplate> {
        &mut self.tournament_template
    }

    // optional .ETournamentState state = 6;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: super::dota_client_enums::ETournamentState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> super::dota_client_enums::ETournamentState {
        self.state.unwrap_or(super::dota_client_enums::ETournamentState::k_ETournamentState_Unknown)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<super::dota_client_enums::ETournamentState> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_client_enums::ETournamentState> {
        &mut self.state
    }

    // optional uint32 state_seq_num = 10;

    pub fn clear_state_seq_num(&mut self) {
        self.state_seq_num = ::std::option::Option::None;
    }

    pub fn has_state_seq_num(&self) -> bool {
        self.state_seq_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state_seq_num(&mut self, v: u32) {
        self.state_seq_num = ::std::option::Option::Some(v);
    }

    pub fn get_state_seq_num(&self) -> u32 {
        self.state_seq_num.unwrap_or(0)
    }

    fn get_state_seq_num_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.state_seq_num
    }

    fn mut_state_seq_num_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.state_seq_num
    }

    // optional uint32 season_trophy_id = 11;

    pub fn clear_season_trophy_id(&mut self) {
        self.season_trophy_id = ::std::option::Option::None;
    }

    pub fn has_season_trophy_id(&self) -> bool {
        self.season_trophy_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_season_trophy_id(&mut self, v: u32) {
        self.season_trophy_id = ::std::option::Option::Some(v);
    }

    pub fn get_season_trophy_id(&self) -> u32 {
        self.season_trophy_id.unwrap_or(0)
    }

    fn get_season_trophy_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.season_trophy_id
    }

    fn mut_season_trophy_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.season_trophy_id
    }

    // repeated .CMsgDOTATournament.Team teams = 7;

    pub fn clear_teams(&mut self) {
        self.teams.clear();
    }

    // Param is passed by value, moved
    pub fn set_teams(&mut self, v: ::protobuf::RepeatedField<CMsgDOTATournament_Team>) {
        self.teams = v;
    }

    // Mutable pointer to the field.
    pub fn mut_teams(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournament_Team> {
        &mut self.teams
    }

    // Take field
    pub fn take_teams(&mut self) -> ::protobuf::RepeatedField<CMsgDOTATournament_Team> {
        ::std::mem::replace(&mut self.teams, ::protobuf::RepeatedField::new())
    }

    pub fn get_teams(&self) -> &[CMsgDOTATournament_Team] {
        &self.teams
    }

    fn get_teams_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTATournament_Team> {
        &self.teams
    }

    fn mut_teams_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournament_Team> {
        &mut self.teams
    }

    // repeated .CMsgDOTATournament.Game games = 8;

    pub fn clear_games(&mut self) {
        self.games.clear();
    }

    // Param is passed by value, moved
    pub fn set_games(&mut self, v: ::protobuf::RepeatedField<CMsgDOTATournament_Game>) {
        self.games = v;
    }

    // Mutable pointer to the field.
    pub fn mut_games(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournament_Game> {
        &mut self.games
    }

    // Take field
    pub fn take_games(&mut self) -> ::protobuf::RepeatedField<CMsgDOTATournament_Game> {
        ::std::mem::replace(&mut self.games, ::protobuf::RepeatedField::new())
    }

    pub fn get_games(&self) -> &[CMsgDOTATournament_Game] {
        &self.games
    }

    fn get_games_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTATournament_Game> {
        &self.games
    }

    fn mut_games_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournament_Game> {
        &mut self.games
    }

    // repeated .CMsgDOTATournament.Node nodes = 9;

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::protobuf::RepeatedField<CMsgDOTATournament_Node>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournament_Node> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::protobuf::RepeatedField<CMsgDOTATournament_Node> {
        ::std::mem::replace(&mut self.nodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_nodes(&self) -> &[CMsgDOTATournament_Node] {
        &self.nodes
    }

    fn get_nodes_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTATournament_Node> {
        &self.nodes
    }

    fn mut_nodes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournament_Node> {
        &mut self.nodes
    }
}

impl ::protobuf::Message for CMsgDOTATournament {
    fn is_initialized(&self) -> bool {
        for v in &self.teams {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.games {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.nodes {
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
                    self.tournament_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.division_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.schedule_time = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.skill_level = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.tournament_template = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.state_seq_num = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.season_trophy_id = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.teams)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.games)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.nodes)?;
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
        if let Some(v) = self.tournament_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.division_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.schedule_time {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.skill_level {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.tournament_template {
            my_size += ::protobuf::rt::enum_size(5, v);
        }
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(6, v);
        }
        if let Some(v) = self.state_seq_num {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.season_trophy_id {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.teams {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.games {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.nodes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tournament_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.division_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.schedule_time {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.skill_level {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.tournament_template {
            os.write_enum(5, v.value())?;
        }
        if let Some(v) = self.state {
            os.write_enum(6, v.value())?;
        }
        if let Some(v) = self.state_seq_num {
            os.write_uint32(10, v)?;
        }
        if let Some(v) = self.season_trophy_id {
            os.write_uint32(11, v)?;
        }
        for v in &self.teams {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.games {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.nodes {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CMsgDOTATournament {
    fn new() -> CMsgDOTATournament {
        CMsgDOTATournament::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATournament>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tournament_id",
                    CMsgDOTATournament::get_tournament_id_for_reflect,
                    CMsgDOTATournament::mut_tournament_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "division_id",
                    CMsgDOTATournament::get_division_id_for_reflect,
                    CMsgDOTATournament::mut_division_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "schedule_time",
                    CMsgDOTATournament::get_schedule_time_for_reflect,
                    CMsgDOTATournament::mut_schedule_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "skill_level",
                    CMsgDOTATournament::get_skill_level_for_reflect,
                    CMsgDOTATournament::mut_skill_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_client_enums::ETournamentTemplate>>(
                    "tournament_template",
                    CMsgDOTATournament::get_tournament_template_for_reflect,
                    CMsgDOTATournament::mut_tournament_template_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_client_enums::ETournamentState>>(
                    "state",
                    CMsgDOTATournament::get_state_for_reflect,
                    CMsgDOTATournament::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "state_seq_num",
                    CMsgDOTATournament::get_state_seq_num_for_reflect,
                    CMsgDOTATournament::mut_state_seq_num_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "season_trophy_id",
                    CMsgDOTATournament::get_season_trophy_id_for_reflect,
                    CMsgDOTATournament::mut_season_trophy_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTATournament_Team>>(
                    "teams",
                    CMsgDOTATournament::get_teams_for_reflect,
                    CMsgDOTATournament::mut_teams_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTATournament_Game>>(
                    "games",
                    CMsgDOTATournament::get_games_for_reflect,
                    CMsgDOTATournament::mut_games_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTATournament_Node>>(
                    "nodes",
                    CMsgDOTATournament::get_nodes_for_reflect,
                    CMsgDOTATournament::mut_nodes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATournament>(
                    "CMsgDOTATournament",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATournament {
    fn clear(&mut self) {
        self.clear_tournament_id();
        self.clear_division_id();
        self.clear_schedule_time();
        self.clear_skill_level();
        self.clear_tournament_template();
        self.clear_state();
        self.clear_state_seq_num();
        self.clear_season_trophy_id();
        self.clear_teams();
        self.clear_games();
        self.clear_nodes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATournament {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATournament {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATournament_Team {
    // message fields
    team_gid: ::std::option::Option<u64>,
    node_or_state: ::std::option::Option<u32>,
    players: ::std::vec::Vec<u32>,
    player_buyin: ::std::vec::Vec<u32>,
    player_skill_level: ::std::vec::Vec<u32>,
    match_group_mask: ::std::option::Option<u32>,
    team_id: ::std::option::Option<u32>,
    team_name: ::protobuf::SingularField<::std::string::String>,
    team_base_logo: ::std::option::Option<u64>,
    team_ui_logo: ::std::option::Option<u64>,
    team_date: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATournament_Team {}

impl CMsgDOTATournament_Team {
    pub fn new() -> CMsgDOTATournament_Team {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATournament_Team {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATournament_Team> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATournament_Team,
        };
        unsafe {
            instance.get(CMsgDOTATournament_Team::new)
        }
    }

    // optional fixed64 team_gid = 1;

    pub fn clear_team_gid(&mut self) {
        self.team_gid = ::std::option::Option::None;
    }

    pub fn has_team_gid(&self) -> bool {
        self.team_gid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_gid(&mut self, v: u64) {
        self.team_gid = ::std::option::Option::Some(v);
    }

    pub fn get_team_gid(&self) -> u64 {
        self.team_gid.unwrap_or(0)
    }

    fn get_team_gid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team_gid
    }

    fn mut_team_gid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team_gid
    }

    // optional uint32 node_or_state = 2;

    pub fn clear_node_or_state(&mut self) {
        self.node_or_state = ::std::option::Option::None;
    }

    pub fn has_node_or_state(&self) -> bool {
        self.node_or_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_or_state(&mut self, v: u32) {
        self.node_or_state = ::std::option::Option::Some(v);
    }

    pub fn get_node_or_state(&self) -> u32 {
        self.node_or_state.unwrap_or(0)
    }

    fn get_node_or_state_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.node_or_state
    }

    fn mut_node_or_state_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.node_or_state
    }

    // repeated uint32 players = 3;

    pub fn clear_players(&mut self) {
        self.players.clear();
    }

    // Param is passed by value, moved
    pub fn set_players(&mut self, v: ::std::vec::Vec<u32>) {
        self.players = v;
    }

    // Mutable pointer to the field.
    pub fn mut_players(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.players
    }

    // Take field
    pub fn take_players(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.players, ::std::vec::Vec::new())
    }

    pub fn get_players(&self) -> &[u32] {
        &self.players
    }

    fn get_players_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.players
    }

    fn mut_players_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.players
    }

    // repeated uint32 player_buyin = 9;

    pub fn clear_player_buyin(&mut self) {
        self.player_buyin.clear();
    }

    // Param is passed by value, moved
    pub fn set_player_buyin(&mut self, v: ::std::vec::Vec<u32>) {
        self.player_buyin = v;
    }

    // Mutable pointer to the field.
    pub fn mut_player_buyin(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.player_buyin
    }

    // Take field
    pub fn take_player_buyin(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.player_buyin, ::std::vec::Vec::new())
    }

    pub fn get_player_buyin(&self) -> &[u32] {
        &self.player_buyin
    }

    fn get_player_buyin_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.player_buyin
    }

    fn mut_player_buyin_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.player_buyin
    }

    // repeated uint32 player_skill_level = 10;

    pub fn clear_player_skill_level(&mut self) {
        self.player_skill_level.clear();
    }

    // Param is passed by value, moved
    pub fn set_player_skill_level(&mut self, v: ::std::vec::Vec<u32>) {
        self.player_skill_level = v;
    }

    // Mutable pointer to the field.
    pub fn mut_player_skill_level(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.player_skill_level
    }

    // Take field
    pub fn take_player_skill_level(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.player_skill_level, ::std::vec::Vec::new())
    }

    pub fn get_player_skill_level(&self) -> &[u32] {
        &self.player_skill_level
    }

    fn get_player_skill_level_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.player_skill_level
    }

    fn mut_player_skill_level_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.player_skill_level
    }

    // optional uint32 match_group_mask = 12;

    pub fn clear_match_group_mask(&mut self) {
        self.match_group_mask = ::std::option::Option::None;
    }

    pub fn has_match_group_mask(&self) -> bool {
        self.match_group_mask.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_group_mask(&mut self, v: u32) {
        self.match_group_mask = ::std::option::Option::Some(v);
    }

    pub fn get_match_group_mask(&self) -> u32 {
        self.match_group_mask.unwrap_or(0)
    }

    fn get_match_group_mask_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.match_group_mask
    }

    fn mut_match_group_mask_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.match_group_mask
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

    // optional string team_name = 5;

    pub fn clear_team_name(&mut self) {
        self.team_name.clear();
    }

    pub fn has_team_name(&self) -> bool {
        self.team_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_name(&mut self, v: ::std::string::String) {
        self.team_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team_name(&mut self) -> &mut ::std::string::String {
        if self.team_name.is_none() {
            self.team_name.set_default();
        }
        self.team_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_team_name(&mut self) -> ::std::string::String {
        self.team_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_team_name(&self) -> &str {
        match self.team_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_team_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.team_name
    }

    fn mut_team_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.team_name
    }

    // optional uint64 team_base_logo = 7;

    pub fn clear_team_base_logo(&mut self) {
        self.team_base_logo = ::std::option::Option::None;
    }

    pub fn has_team_base_logo(&self) -> bool {
        self.team_base_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_base_logo(&mut self, v: u64) {
        self.team_base_logo = ::std::option::Option::Some(v);
    }

    pub fn get_team_base_logo(&self) -> u64 {
        self.team_base_logo.unwrap_or(0)
    }

    fn get_team_base_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team_base_logo
    }

    fn mut_team_base_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team_base_logo
    }

    // optional uint64 team_ui_logo = 8;

    pub fn clear_team_ui_logo(&mut self) {
        self.team_ui_logo = ::std::option::Option::None;
    }

    pub fn has_team_ui_logo(&self) -> bool {
        self.team_ui_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_ui_logo(&mut self, v: u64) {
        self.team_ui_logo = ::std::option::Option::Some(v);
    }

    pub fn get_team_ui_logo(&self) -> u64 {
        self.team_ui_logo.unwrap_or(0)
    }

    fn get_team_ui_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team_ui_logo
    }

    fn mut_team_ui_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team_ui_logo
    }

    // optional uint32 team_date = 11;

    pub fn clear_team_date(&mut self) {
        self.team_date = ::std::option::Option::None;
    }

    pub fn has_team_date(&self) -> bool {
        self.team_date.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_date(&mut self, v: u32) {
        self.team_date = ::std::option::Option::Some(v);
    }

    pub fn get_team_date(&self) -> u32 {
        self.team_date.unwrap_or(0)
    }

    fn get_team_date_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_date
    }

    fn mut_team_date_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_date
    }
}

impl ::protobuf::Message for CMsgDOTATournament_Team {
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
                    self.team_gid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.node_or_state = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.players)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.player_buyin)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.player_skill_level)?;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.match_group_mask = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team_name)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.team_base_logo = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.team_ui_logo = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team_date = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team_gid {
            my_size += 9;
        }
        if let Some(v) = self.node_or_state {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.players.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(3, &self.players);
        }
        if !self.player_buyin.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(9, &self.player_buyin);
        }
        if !self.player_skill_level.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(10, &self.player_skill_level);
        }
        if let Some(v) = self.match_group_mask {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.team_name.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(v) = self.team_base_logo {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team_ui_logo {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team_date {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team_gid {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.node_or_state {
            os.write_uint32(2, v)?;
        }
        if !self.players.is_empty() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.players))?;
            for v in &self.players {
                os.write_uint32_no_tag(*v)?;
            };
        }
        if !self.player_buyin.is_empty() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.player_buyin))?;
            for v in &self.player_buyin {
                os.write_uint32_no_tag(*v)?;
            };
        }
        if !self.player_skill_level.is_empty() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.player_skill_level))?;
            for v in &self.player_skill_level {
                os.write_uint32_no_tag(*v)?;
            };
        }
        if let Some(v) = self.match_group_mask {
            os.write_uint32(12, v)?;
        }
        if let Some(v) = self.team_id {
            os.write_uint32(4, v)?;
        }
        if let Some(ref v) = self.team_name.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(v) = self.team_base_logo {
            os.write_uint64(7, v)?;
        }
        if let Some(v) = self.team_ui_logo {
            os.write_uint64(8, v)?;
        }
        if let Some(v) = self.team_date {
            os.write_uint32(11, v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTATournament_Team {
    fn new() -> CMsgDOTATournament_Team {
        CMsgDOTATournament_Team::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATournament_Team>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "team_gid",
                    CMsgDOTATournament_Team::get_team_gid_for_reflect,
                    CMsgDOTATournament_Team::mut_team_gid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "node_or_state",
                    CMsgDOTATournament_Team::get_node_or_state_for_reflect,
                    CMsgDOTATournament_Team::mut_node_or_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "players",
                    CMsgDOTATournament_Team::get_players_for_reflect,
                    CMsgDOTATournament_Team::mut_players_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_buyin",
                    CMsgDOTATournament_Team::get_player_buyin_for_reflect,
                    CMsgDOTATournament_Team::mut_player_buyin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_skill_level",
                    CMsgDOTATournament_Team::get_player_skill_level_for_reflect,
                    CMsgDOTATournament_Team::mut_player_skill_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "match_group_mask",
                    CMsgDOTATournament_Team::get_match_group_mask_for_reflect,
                    CMsgDOTATournament_Team::mut_match_group_mask_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgDOTATournament_Team::get_team_id_for_reflect,
                    CMsgDOTATournament_Team::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team_name",
                    CMsgDOTATournament_Team::get_team_name_for_reflect,
                    CMsgDOTATournament_Team::mut_team_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "team_base_logo",
                    CMsgDOTATournament_Team::get_team_base_logo_for_reflect,
                    CMsgDOTATournament_Team::mut_team_base_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "team_ui_logo",
                    CMsgDOTATournament_Team::get_team_ui_logo_for_reflect,
                    CMsgDOTATournament_Team::mut_team_ui_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_date",
                    CMsgDOTATournament_Team::get_team_date_for_reflect,
                    CMsgDOTATournament_Team::mut_team_date_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATournament_Team>(
                    "CMsgDOTATournament_Team",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATournament_Team {
    fn clear(&mut self) {
        self.clear_team_gid();
        self.clear_node_or_state();
        self.clear_players();
        self.clear_player_buyin();
        self.clear_player_skill_level();
        self.clear_match_group_mask();
        self.clear_team_id();
        self.clear_team_name();
        self.clear_team_base_logo();
        self.clear_team_ui_logo();
        self.clear_team_date();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATournament_Team {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATournament_Team {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATournament_Game {
    // message fields
    node_idx: ::std::option::Option<u32>,
    lobby_id: ::std::option::Option<u64>,
    match_id: ::std::option::Option<u64>,
    team_a_good: ::std::option::Option<bool>,
    state: ::std::option::Option<super::dota_client_enums::ETournamentGameState>,
    start_time: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATournament_Game {}

impl CMsgDOTATournament_Game {
    pub fn new() -> CMsgDOTATournament_Game {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATournament_Game {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATournament_Game> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATournament_Game,
        };
        unsafe {
            instance.get(CMsgDOTATournament_Game::new)
        }
    }

    // optional uint32 node_idx = 1;

    pub fn clear_node_idx(&mut self) {
        self.node_idx = ::std::option::Option::None;
    }

    pub fn has_node_idx(&self) -> bool {
        self.node_idx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_idx(&mut self, v: u32) {
        self.node_idx = ::std::option::Option::Some(v);
    }

    pub fn get_node_idx(&self) -> u32 {
        self.node_idx.unwrap_or(0)
    }

    fn get_node_idx_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.node_idx
    }

    fn mut_node_idx_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.node_idx
    }

    // optional fixed64 lobby_id = 2;

    pub fn clear_lobby_id(&mut self) {
        self.lobby_id = ::std::option::Option::None;
    }

    pub fn has_lobby_id(&self) -> bool {
        self.lobby_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_id(&mut self, v: u64) {
        self.lobby_id = ::std::option::Option::Some(v);
    }

    pub fn get_lobby_id(&self) -> u64 {
        self.lobby_id.unwrap_or(0)
    }

    fn get_lobby_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lobby_id
    }

    fn mut_lobby_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lobby_id
    }

    // optional uint64 match_id = 3;

    pub fn clear_match_id(&mut self) {
        self.match_id = ::std::option::Option::None;
    }

    pub fn has_match_id(&self) -> bool {
        self.match_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_id(&mut self, v: u64) {
        self.match_id = ::std::option::Option::Some(v);
    }

    pub fn get_match_id(&self) -> u64 {
        self.match_id.unwrap_or(0)
    }

    fn get_match_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.match_id
    }

    fn mut_match_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.match_id
    }

    // optional bool team_a_good = 4;

    pub fn clear_team_a_good(&mut self) {
        self.team_a_good = ::std::option::Option::None;
    }

    pub fn has_team_a_good(&self) -> bool {
        self.team_a_good.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_a_good(&mut self, v: bool) {
        self.team_a_good = ::std::option::Option::Some(v);
    }

    pub fn get_team_a_good(&self) -> bool {
        self.team_a_good.unwrap_or(false)
    }

    fn get_team_a_good_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.team_a_good
    }

    fn mut_team_a_good_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.team_a_good
    }

    // optional .ETournamentGameState state = 5;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: super::dota_client_enums::ETournamentGameState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> super::dota_client_enums::ETournamentGameState {
        self.state.unwrap_or(super::dota_client_enums::ETournamentGameState::k_ETournamentGameState_Unknown)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<super::dota_client_enums::ETournamentGameState> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_client_enums::ETournamentGameState> {
        &mut self.state
    }

    // optional uint32 start_time = 6;

    pub fn clear_start_time(&mut self) {
        self.start_time = ::std::option::Option::None;
    }

    pub fn has_start_time(&self) -> bool {
        self.start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_time(&mut self, v: u32) {
        self.start_time = ::std::option::Option::Some(v);
    }

    pub fn get_start_time(&self) -> u32 {
        self.start_time.unwrap_or(0)
    }

    fn get_start_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.start_time
    }

    fn mut_start_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.start_time
    }
}

impl ::protobuf::Message for CMsgDOTATournament_Game {
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
                    self.node_idx = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.lobby_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.match_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.team_a_good = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.start_time = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.node_idx {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lobby_id {
            my_size += 9;
        }
        if let Some(v) = self.match_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team_a_good {
            my_size += 2;
        }
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(5, v);
        }
        if let Some(v) = self.start_time {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.node_idx {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.lobby_id {
            os.write_fixed64(2, v)?;
        }
        if let Some(v) = self.match_id {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.team_a_good {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.state {
            os.write_enum(5, v.value())?;
        }
        if let Some(v) = self.start_time {
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

impl ::protobuf::MessageStatic for CMsgDOTATournament_Game {
    fn new() -> CMsgDOTATournament_Game {
        CMsgDOTATournament_Game::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATournament_Game>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "node_idx",
                    CMsgDOTATournament_Game::get_node_idx_for_reflect,
                    CMsgDOTATournament_Game::mut_node_idx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "lobby_id",
                    CMsgDOTATournament_Game::get_lobby_id_for_reflect,
                    CMsgDOTATournament_Game::mut_lobby_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "match_id",
                    CMsgDOTATournament_Game::get_match_id_for_reflect,
                    CMsgDOTATournament_Game::mut_match_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "team_a_good",
                    CMsgDOTATournament_Game::get_team_a_good_for_reflect,
                    CMsgDOTATournament_Game::mut_team_a_good_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_client_enums::ETournamentGameState>>(
                    "state",
                    CMsgDOTATournament_Game::get_state_for_reflect,
                    CMsgDOTATournament_Game::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "start_time",
                    CMsgDOTATournament_Game::get_start_time_for_reflect,
                    CMsgDOTATournament_Game::mut_start_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATournament_Game>(
                    "CMsgDOTATournament_Game",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATournament_Game {
    fn clear(&mut self) {
        self.clear_node_idx();
        self.clear_lobby_id();
        self.clear_match_id();
        self.clear_team_a_good();
        self.clear_state();
        self.clear_start_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATournament_Game {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATournament_Game {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATournament_Node {
    // message fields
    node_id: ::std::option::Option<u32>,
    team_idx_a: ::std::option::Option<u32>,
    team_idx_b: ::std::option::Option<u32>,
    node_state: ::std::option::Option<super::dota_client_enums::ETournamentNodeState>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATournament_Node {}

impl CMsgDOTATournament_Node {
    pub fn new() -> CMsgDOTATournament_Node {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATournament_Node {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATournament_Node> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATournament_Node,
        };
        unsafe {
            instance.get(CMsgDOTATournament_Node::new)
        }
    }

    // optional uint32 node_id = 1;

    pub fn clear_node_id(&mut self) {
        self.node_id = ::std::option::Option::None;
    }

    pub fn has_node_id(&self) -> bool {
        self.node_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_id(&mut self, v: u32) {
        self.node_id = ::std::option::Option::Some(v);
    }

    pub fn get_node_id(&self) -> u32 {
        self.node_id.unwrap_or(0)
    }

    fn get_node_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.node_id
    }

    fn mut_node_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.node_id
    }

    // optional uint32 team_idx_a = 2;

    pub fn clear_team_idx_a(&mut self) {
        self.team_idx_a = ::std::option::Option::None;
    }

    pub fn has_team_idx_a(&self) -> bool {
        self.team_idx_a.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_idx_a(&mut self, v: u32) {
        self.team_idx_a = ::std::option::Option::Some(v);
    }

    pub fn get_team_idx_a(&self) -> u32 {
        self.team_idx_a.unwrap_or(0)
    }

    fn get_team_idx_a_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_idx_a
    }

    fn mut_team_idx_a_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_idx_a
    }

    // optional uint32 team_idx_b = 3;

    pub fn clear_team_idx_b(&mut self) {
        self.team_idx_b = ::std::option::Option::None;
    }

    pub fn has_team_idx_b(&self) -> bool {
        self.team_idx_b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_idx_b(&mut self, v: u32) {
        self.team_idx_b = ::std::option::Option::Some(v);
    }

    pub fn get_team_idx_b(&self) -> u32 {
        self.team_idx_b.unwrap_or(0)
    }

    fn get_team_idx_b_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_idx_b
    }

    fn mut_team_idx_b_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_idx_b
    }

    // optional .ETournamentNodeState node_state = 4;

    pub fn clear_node_state(&mut self) {
        self.node_state = ::std::option::Option::None;
    }

    pub fn has_node_state(&self) -> bool {
        self.node_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_state(&mut self, v: super::dota_client_enums::ETournamentNodeState) {
        self.node_state = ::std::option::Option::Some(v);
    }

    pub fn get_node_state(&self) -> super::dota_client_enums::ETournamentNodeState {
        self.node_state.unwrap_or(super::dota_client_enums::ETournamentNodeState::k_ETournamentNodeState_Unknown)
    }

    fn get_node_state_for_reflect(&self) -> &::std::option::Option<super::dota_client_enums::ETournamentNodeState> {
        &self.node_state
    }

    fn mut_node_state_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_client_enums::ETournamentNodeState> {
        &mut self.node_state
    }
}

impl ::protobuf::Message for CMsgDOTATournament_Node {
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
                    self.node_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team_idx_a = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team_idx_b = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.node_state = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.node_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team_idx_a {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team_idx_b {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.node_state {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.node_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.team_idx_a {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.team_idx_b {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.node_state {
            os.write_enum(4, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgDOTATournament_Node {
    fn new() -> CMsgDOTATournament_Node {
        CMsgDOTATournament_Node::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATournament_Node>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "node_id",
                    CMsgDOTATournament_Node::get_node_id_for_reflect,
                    CMsgDOTATournament_Node::mut_node_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_idx_a",
                    CMsgDOTATournament_Node::get_team_idx_a_for_reflect,
                    CMsgDOTATournament_Node::mut_team_idx_a_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_idx_b",
                    CMsgDOTATournament_Node::get_team_idx_b_for_reflect,
                    CMsgDOTATournament_Node::mut_team_idx_b_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_client_enums::ETournamentNodeState>>(
                    "node_state",
                    CMsgDOTATournament_Node::get_node_state_for_reflect,
                    CMsgDOTATournament_Node::mut_node_state_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATournament_Node>(
                    "CMsgDOTATournament_Node",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATournament_Node {
    fn clear(&mut self) {
        self.clear_node_id();
        self.clear_team_idx_a();
        self.clear_team_idx_b();
        self.clear_node_state();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATournament_Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATournament_Node {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATournamentStateChange {
    // message fields
    new_tournament_id: ::std::option::Option<u32>,
    event: ::std::option::Option<ETournamentEvent>,
    new_tournament_state: ::std::option::Option<super::dota_client_enums::ETournamentState>,
    game_changes: ::protobuf::RepeatedField<CMsgDOTATournamentStateChange_GameChange>,
    team_changes: ::protobuf::RepeatedField<CMsgDOTATournamentStateChange_TeamChange>,
    merged_tournament_ids: ::std::vec::Vec<u32>,
    state_seq_num: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATournamentStateChange {}

impl CMsgDOTATournamentStateChange {
    pub fn new() -> CMsgDOTATournamentStateChange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATournamentStateChange {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATournamentStateChange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATournamentStateChange,
        };
        unsafe {
            instance.get(CMsgDOTATournamentStateChange::new)
        }
    }

    // optional uint32 new_tournament_id = 1;

    pub fn clear_new_tournament_id(&mut self) {
        self.new_tournament_id = ::std::option::Option::None;
    }

    pub fn has_new_tournament_id(&self) -> bool {
        self.new_tournament_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_tournament_id(&mut self, v: u32) {
        self.new_tournament_id = ::std::option::Option::Some(v);
    }

    pub fn get_new_tournament_id(&self) -> u32 {
        self.new_tournament_id.unwrap_or(0)
    }

    fn get_new_tournament_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.new_tournament_id
    }

    fn mut_new_tournament_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.new_tournament_id
    }

    // optional .ETournamentEvent event = 2;

    pub fn clear_event(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_event(&self) -> bool {
        self.event.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event(&mut self, v: ETournamentEvent) {
        self.event = ::std::option::Option::Some(v);
    }

    pub fn get_event(&self) -> ETournamentEvent {
        self.event.unwrap_or(ETournamentEvent::k_ETournamentEvent_None)
    }

    fn get_event_for_reflect(&self) -> &::std::option::Option<ETournamentEvent> {
        &self.event
    }

    fn mut_event_for_reflect(&mut self) -> &mut ::std::option::Option<ETournamentEvent> {
        &mut self.event
    }

    // optional .ETournamentState new_tournament_state = 3;

    pub fn clear_new_tournament_state(&mut self) {
        self.new_tournament_state = ::std::option::Option::None;
    }

    pub fn has_new_tournament_state(&self) -> bool {
        self.new_tournament_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_tournament_state(&mut self, v: super::dota_client_enums::ETournamentState) {
        self.new_tournament_state = ::std::option::Option::Some(v);
    }

    pub fn get_new_tournament_state(&self) -> super::dota_client_enums::ETournamentState {
        self.new_tournament_state.unwrap_or(super::dota_client_enums::ETournamentState::k_ETournamentState_Unknown)
    }

    fn get_new_tournament_state_for_reflect(&self) -> &::std::option::Option<super::dota_client_enums::ETournamentState> {
        &self.new_tournament_state
    }

    fn mut_new_tournament_state_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_client_enums::ETournamentState> {
        &mut self.new_tournament_state
    }

    // repeated .CMsgDOTATournamentStateChange.GameChange game_changes = 4;

    pub fn clear_game_changes(&mut self) {
        self.game_changes.clear();
    }

    // Param is passed by value, moved
    pub fn set_game_changes(&mut self, v: ::protobuf::RepeatedField<CMsgDOTATournamentStateChange_GameChange>) {
        self.game_changes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_game_changes(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournamentStateChange_GameChange> {
        &mut self.game_changes
    }

    // Take field
    pub fn take_game_changes(&mut self) -> ::protobuf::RepeatedField<CMsgDOTATournamentStateChange_GameChange> {
        ::std::mem::replace(&mut self.game_changes, ::protobuf::RepeatedField::new())
    }

    pub fn get_game_changes(&self) -> &[CMsgDOTATournamentStateChange_GameChange] {
        &self.game_changes
    }

    fn get_game_changes_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTATournamentStateChange_GameChange> {
        &self.game_changes
    }

    fn mut_game_changes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournamentStateChange_GameChange> {
        &mut self.game_changes
    }

    // repeated .CMsgDOTATournamentStateChange.TeamChange team_changes = 5;

    pub fn clear_team_changes(&mut self) {
        self.team_changes.clear();
    }

    // Param is passed by value, moved
    pub fn set_team_changes(&mut self, v: ::protobuf::RepeatedField<CMsgDOTATournamentStateChange_TeamChange>) {
        self.team_changes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_team_changes(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournamentStateChange_TeamChange> {
        &mut self.team_changes
    }

    // Take field
    pub fn take_team_changes(&mut self) -> ::protobuf::RepeatedField<CMsgDOTATournamentStateChange_TeamChange> {
        ::std::mem::replace(&mut self.team_changes, ::protobuf::RepeatedField::new())
    }

    pub fn get_team_changes(&self) -> &[CMsgDOTATournamentStateChange_TeamChange] {
        &self.team_changes
    }

    fn get_team_changes_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTATournamentStateChange_TeamChange> {
        &self.team_changes
    }

    fn mut_team_changes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATournamentStateChange_TeamChange> {
        &mut self.team_changes
    }

    // repeated uint32 merged_tournament_ids = 6;

    pub fn clear_merged_tournament_ids(&mut self) {
        self.merged_tournament_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_merged_tournament_ids(&mut self, v: ::std::vec::Vec<u32>) {
        self.merged_tournament_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_merged_tournament_ids(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.merged_tournament_ids
    }

    // Take field
    pub fn take_merged_tournament_ids(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.merged_tournament_ids, ::std::vec::Vec::new())
    }

    pub fn get_merged_tournament_ids(&self) -> &[u32] {
        &self.merged_tournament_ids
    }

    fn get_merged_tournament_ids_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.merged_tournament_ids
    }

    fn mut_merged_tournament_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.merged_tournament_ids
    }

    // optional uint32 state_seq_num = 7;

    pub fn clear_state_seq_num(&mut self) {
        self.state_seq_num = ::std::option::Option::None;
    }

    pub fn has_state_seq_num(&self) -> bool {
        self.state_seq_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state_seq_num(&mut self, v: u32) {
        self.state_seq_num = ::std::option::Option::Some(v);
    }

    pub fn get_state_seq_num(&self) -> u32 {
        self.state_seq_num.unwrap_or(0)
    }

    fn get_state_seq_num_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.state_seq_num
    }

    fn mut_state_seq_num_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.state_seq_num
    }
}

impl ::protobuf::Message for CMsgDOTATournamentStateChange {
    fn is_initialized(&self) -> bool {
        for v in &self.game_changes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.team_changes {
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
                    self.new_tournament_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.event = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.new_tournament_state = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.game_changes)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.team_changes)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.merged_tournament_ids)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.state_seq_num = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.new_tournament_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.event {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.new_tournament_state {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        for value in &self.game_changes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.team_changes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.merged_tournament_ids.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(6, &self.merged_tournament_ids);
        }
        if let Some(v) = self.state_seq_num {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.new_tournament_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.event {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.new_tournament_state {
            os.write_enum(3, v.value())?;
        }
        for v in &self.game_changes {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.team_changes {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.merged_tournament_ids.is_empty() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.merged_tournament_ids))?;
            for v in &self.merged_tournament_ids {
                os.write_uint32_no_tag(*v)?;
            };
        }
        if let Some(v) = self.state_seq_num {
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

impl ::protobuf::MessageStatic for CMsgDOTATournamentStateChange {
    fn new() -> CMsgDOTATournamentStateChange {
        CMsgDOTATournamentStateChange::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATournamentStateChange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "new_tournament_id",
                    CMsgDOTATournamentStateChange::get_new_tournament_id_for_reflect,
                    CMsgDOTATournamentStateChange::mut_new_tournament_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ETournamentEvent>>(
                    "event",
                    CMsgDOTATournamentStateChange::get_event_for_reflect,
                    CMsgDOTATournamentStateChange::mut_event_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_client_enums::ETournamentState>>(
                    "new_tournament_state",
                    CMsgDOTATournamentStateChange::get_new_tournament_state_for_reflect,
                    CMsgDOTATournamentStateChange::mut_new_tournament_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTATournamentStateChange_GameChange>>(
                    "game_changes",
                    CMsgDOTATournamentStateChange::get_game_changes_for_reflect,
                    CMsgDOTATournamentStateChange::mut_game_changes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTATournamentStateChange_TeamChange>>(
                    "team_changes",
                    CMsgDOTATournamentStateChange::get_team_changes_for_reflect,
                    CMsgDOTATournamentStateChange::mut_team_changes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "merged_tournament_ids",
                    CMsgDOTATournamentStateChange::get_merged_tournament_ids_for_reflect,
                    CMsgDOTATournamentStateChange::mut_merged_tournament_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "state_seq_num",
                    CMsgDOTATournamentStateChange::get_state_seq_num_for_reflect,
                    CMsgDOTATournamentStateChange::mut_state_seq_num_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATournamentStateChange>(
                    "CMsgDOTATournamentStateChange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATournamentStateChange {
    fn clear(&mut self) {
        self.clear_new_tournament_id();
        self.clear_event();
        self.clear_new_tournament_state();
        self.clear_game_changes();
        self.clear_team_changes();
        self.clear_merged_tournament_ids();
        self.clear_state_seq_num();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATournamentStateChange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATournamentStateChange {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATournamentStateChange_GameChange {
    // message fields
    match_id: ::std::option::Option<u64>,
    new_state: ::std::option::Option<super::dota_client_enums::ETournamentGameState>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATournamentStateChange_GameChange {}

impl CMsgDOTATournamentStateChange_GameChange {
    pub fn new() -> CMsgDOTATournamentStateChange_GameChange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATournamentStateChange_GameChange {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATournamentStateChange_GameChange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATournamentStateChange_GameChange,
        };
        unsafe {
            instance.get(CMsgDOTATournamentStateChange_GameChange::new)
        }
    }

    // optional uint64 match_id = 1;

    pub fn clear_match_id(&mut self) {
        self.match_id = ::std::option::Option::None;
    }

    pub fn has_match_id(&self) -> bool {
        self.match_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_id(&mut self, v: u64) {
        self.match_id = ::std::option::Option::Some(v);
    }

    pub fn get_match_id(&self) -> u64 {
        self.match_id.unwrap_or(0)
    }

    fn get_match_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.match_id
    }

    fn mut_match_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.match_id
    }

    // optional .ETournamentGameState new_state = 2;

    pub fn clear_new_state(&mut self) {
        self.new_state = ::std::option::Option::None;
    }

    pub fn has_new_state(&self) -> bool {
        self.new_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_state(&mut self, v: super::dota_client_enums::ETournamentGameState) {
        self.new_state = ::std::option::Option::Some(v);
    }

    pub fn get_new_state(&self) -> super::dota_client_enums::ETournamentGameState {
        self.new_state.unwrap_or(super::dota_client_enums::ETournamentGameState::k_ETournamentGameState_Unknown)
    }

    fn get_new_state_for_reflect(&self) -> &::std::option::Option<super::dota_client_enums::ETournamentGameState> {
        &self.new_state
    }

    fn mut_new_state_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_client_enums::ETournamentGameState> {
        &mut self.new_state
    }
}

impl ::protobuf::Message for CMsgDOTATournamentStateChange_GameChange {
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
                    let tmp = is.read_uint64()?;
                    self.match_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.new_state = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.match_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.new_state {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.match_id {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.new_state {
            os.write_enum(2, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgDOTATournamentStateChange_GameChange {
    fn new() -> CMsgDOTATournamentStateChange_GameChange {
        CMsgDOTATournamentStateChange_GameChange::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATournamentStateChange_GameChange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "match_id",
                    CMsgDOTATournamentStateChange_GameChange::get_match_id_for_reflect,
                    CMsgDOTATournamentStateChange_GameChange::mut_match_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_client_enums::ETournamentGameState>>(
                    "new_state",
                    CMsgDOTATournamentStateChange_GameChange::get_new_state_for_reflect,
                    CMsgDOTATournamentStateChange_GameChange::mut_new_state_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATournamentStateChange_GameChange>(
                    "CMsgDOTATournamentStateChange_GameChange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATournamentStateChange_GameChange {
    fn clear(&mut self) {
        self.clear_match_id();
        self.clear_new_state();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATournamentStateChange_GameChange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATournamentStateChange_GameChange {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATournamentStateChange_TeamChange {
    // message fields
    team_gid: ::std::option::Option<u64>,
    new_node_or_state: ::std::option::Option<u32>,
    old_node_or_state: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATournamentStateChange_TeamChange {}

impl CMsgDOTATournamentStateChange_TeamChange {
    pub fn new() -> CMsgDOTATournamentStateChange_TeamChange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATournamentStateChange_TeamChange {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATournamentStateChange_TeamChange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATournamentStateChange_TeamChange,
        };
        unsafe {
            instance.get(CMsgDOTATournamentStateChange_TeamChange::new)
        }
    }

    // optional uint64 team_gid = 1;

    pub fn clear_team_gid(&mut self) {
        self.team_gid = ::std::option::Option::None;
    }

    pub fn has_team_gid(&self) -> bool {
        self.team_gid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_gid(&mut self, v: u64) {
        self.team_gid = ::std::option::Option::Some(v);
    }

    pub fn get_team_gid(&self) -> u64 {
        self.team_gid.unwrap_or(0)
    }

    fn get_team_gid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team_gid
    }

    fn mut_team_gid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team_gid
    }

    // optional uint32 new_node_or_state = 2;

    pub fn clear_new_node_or_state(&mut self) {
        self.new_node_or_state = ::std::option::Option::None;
    }

    pub fn has_new_node_or_state(&self) -> bool {
        self.new_node_or_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_node_or_state(&mut self, v: u32) {
        self.new_node_or_state = ::std::option::Option::Some(v);
    }

    pub fn get_new_node_or_state(&self) -> u32 {
        self.new_node_or_state.unwrap_or(0)
    }

    fn get_new_node_or_state_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.new_node_or_state
    }

    fn mut_new_node_or_state_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.new_node_or_state
    }

    // optional uint32 old_node_or_state = 3;

    pub fn clear_old_node_or_state(&mut self) {
        self.old_node_or_state = ::std::option::Option::None;
    }

    pub fn has_old_node_or_state(&self) -> bool {
        self.old_node_or_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_old_node_or_state(&mut self, v: u32) {
        self.old_node_or_state = ::std::option::Option::Some(v);
    }

    pub fn get_old_node_or_state(&self) -> u32 {
        self.old_node_or_state.unwrap_or(0)
    }

    fn get_old_node_or_state_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.old_node_or_state
    }

    fn mut_old_node_or_state_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.old_node_or_state
    }
}

impl ::protobuf::Message for CMsgDOTATournamentStateChange_TeamChange {
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
                    let tmp = is.read_uint64()?;
                    self.team_gid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.new_node_or_state = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.old_node_or_state = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team_gid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.new_node_or_state {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.old_node_or_state {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team_gid {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.new_node_or_state {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.old_node_or_state {
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

impl ::protobuf::MessageStatic for CMsgDOTATournamentStateChange_TeamChange {
    fn new() -> CMsgDOTATournamentStateChange_TeamChange {
        CMsgDOTATournamentStateChange_TeamChange::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATournamentStateChange_TeamChange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "team_gid",
                    CMsgDOTATournamentStateChange_TeamChange::get_team_gid_for_reflect,
                    CMsgDOTATournamentStateChange_TeamChange::mut_team_gid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "new_node_or_state",
                    CMsgDOTATournamentStateChange_TeamChange::get_new_node_or_state_for_reflect,
                    CMsgDOTATournamentStateChange_TeamChange::mut_new_node_or_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "old_node_or_state",
                    CMsgDOTATournamentStateChange_TeamChange::get_old_node_or_state_for_reflect,
                    CMsgDOTATournamentStateChange_TeamChange::mut_old_node_or_state_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATournamentStateChange_TeamChange>(
                    "CMsgDOTATournamentStateChange_TeamChange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATournamentStateChange_TeamChange {
    fn clear(&mut self) {
        self.clear_team_gid();
        self.clear_new_node_or_state();
        self.clear_old_node_or_state();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATournamentStateChange_TeamChange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATournamentStateChange_TeamChange {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATournamentRequest {
    // message fields
    tournament_id: ::std::option::Option<u32>,
    client_tournament_gid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATournamentRequest {}

impl CMsgDOTATournamentRequest {
    pub fn new() -> CMsgDOTATournamentRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATournamentRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATournamentRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATournamentRequest,
        };
        unsafe {
            instance.get(CMsgDOTATournamentRequest::new)
        }
    }

    // optional uint32 tournament_id = 1;

    pub fn clear_tournament_id(&mut self) {
        self.tournament_id = ::std::option::Option::None;
    }

    pub fn has_tournament_id(&self) -> bool {
        self.tournament_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tournament_id(&mut self, v: u32) {
        self.tournament_id = ::std::option::Option::Some(v);
    }

    pub fn get_tournament_id(&self) -> u32 {
        self.tournament_id.unwrap_or(0)
    }

    fn get_tournament_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tournament_id
    }

    fn mut_tournament_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tournament_id
    }

    // optional uint64 client_tournament_gid = 2;

    pub fn clear_client_tournament_gid(&mut self) {
        self.client_tournament_gid = ::std::option::Option::None;
    }

    pub fn has_client_tournament_gid(&self) -> bool {
        self.client_tournament_gid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_tournament_gid(&mut self, v: u64) {
        self.client_tournament_gid = ::std::option::Option::Some(v);
    }

    pub fn get_client_tournament_gid(&self) -> u64 {
        self.client_tournament_gid.unwrap_or(0)
    }

    fn get_client_tournament_gid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.client_tournament_gid
    }

    fn mut_client_tournament_gid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.client_tournament_gid
    }
}

impl ::protobuf::Message for CMsgDOTATournamentRequest {
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
                    self.tournament_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.client_tournament_gid = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.tournament_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.client_tournament_gid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tournament_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.client_tournament_gid {
            os.write_uint64(2, v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTATournamentRequest {
    fn new() -> CMsgDOTATournamentRequest {
        CMsgDOTATournamentRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATournamentRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tournament_id",
                    CMsgDOTATournamentRequest::get_tournament_id_for_reflect,
                    CMsgDOTATournamentRequest::mut_tournament_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "client_tournament_gid",
                    CMsgDOTATournamentRequest::get_client_tournament_gid_for_reflect,
                    CMsgDOTATournamentRequest::mut_client_tournament_gid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATournamentRequest>(
                    "CMsgDOTATournamentRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATournamentRequest {
    fn clear(&mut self) {
        self.clear_tournament_id();
        self.clear_client_tournament_gid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATournamentRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATournamentRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATournamentResponse {
    // message fields
    result: ::std::option::Option<u32>,
    tournament: ::protobuf::SingularPtrField<CMsgDOTATournament>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATournamentResponse {}

impl CMsgDOTATournamentResponse {
    pub fn new() -> CMsgDOTATournamentResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATournamentResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATournamentResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATournamentResponse,
        };
        unsafe {
            instance.get(CMsgDOTATournamentResponse::new)
        }
    }

    // optional uint32 result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: u32) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> u32 {
        self.result.unwrap_or(2u32)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.result
    }

    // optional .CMsgDOTATournament tournament = 2;

    pub fn clear_tournament(&mut self) {
        self.tournament.clear();
    }

    pub fn has_tournament(&self) -> bool {
        self.tournament.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tournament(&mut self, v: CMsgDOTATournament) {
        self.tournament = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tournament(&mut self) -> &mut CMsgDOTATournament {
        if self.tournament.is_none() {
            self.tournament.set_default();
        }
        self.tournament.as_mut().unwrap()
    }

    // Take field
    pub fn take_tournament(&mut self) -> CMsgDOTATournament {
        self.tournament.take().unwrap_or_else(|| CMsgDOTATournament::new())
    }

    pub fn get_tournament(&self) -> &CMsgDOTATournament {
        self.tournament.as_ref().unwrap_or_else(|| CMsgDOTATournament::default_instance())
    }

    fn get_tournament_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgDOTATournament> {
        &self.tournament
    }

    fn mut_tournament_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgDOTATournament> {
        &mut self.tournament
    }
}

impl ::protobuf::Message for CMsgDOTATournamentResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.tournament {
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
                    self.result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tournament)?;
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
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.tournament.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.tournament.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgDOTATournamentResponse {
    fn new() -> CMsgDOTATournamentResponse {
        CMsgDOTATournamentResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATournamentResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "result",
                    CMsgDOTATournamentResponse::get_result_for_reflect,
                    CMsgDOTATournamentResponse::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTATournament>>(
                    "tournament",
                    CMsgDOTATournamentResponse::get_tournament_for_reflect,
                    CMsgDOTATournamentResponse::mut_tournament_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATournamentResponse>(
                    "CMsgDOTATournamentResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATournamentResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_tournament();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATournamentResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATournamentResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAClearTournamentGame {
    // message fields
    tournament_id: ::std::option::Option<u32>,
    game_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAClearTournamentGame {}

impl CMsgDOTAClearTournamentGame {
    pub fn new() -> CMsgDOTAClearTournamentGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAClearTournamentGame {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAClearTournamentGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAClearTournamentGame,
        };
        unsafe {
            instance.get(CMsgDOTAClearTournamentGame::new)
        }
    }

    // optional uint32 tournament_id = 1;

    pub fn clear_tournament_id(&mut self) {
        self.tournament_id = ::std::option::Option::None;
    }

    pub fn has_tournament_id(&self) -> bool {
        self.tournament_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tournament_id(&mut self, v: u32) {
        self.tournament_id = ::std::option::Option::Some(v);
    }

    pub fn get_tournament_id(&self) -> u32 {
        self.tournament_id.unwrap_or(0)
    }

    fn get_tournament_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tournament_id
    }

    fn mut_tournament_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tournament_id
    }

    // optional uint32 game_id = 2;

    pub fn clear_game_id(&mut self) {
        self.game_id = ::std::option::Option::None;
    }

    pub fn has_game_id(&self) -> bool {
        self.game_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_id(&mut self, v: u32) {
        self.game_id = ::std::option::Option::Some(v);
    }

    pub fn get_game_id(&self) -> u32 {
        self.game_id.unwrap_or(0)
    }

    fn get_game_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.game_id
    }

    fn mut_game_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.game_id
    }
}

impl ::protobuf::Message for CMsgDOTAClearTournamentGame {
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
                    self.tournament_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.game_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.tournament_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.game_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tournament_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.game_id {
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

impl ::protobuf::MessageStatic for CMsgDOTAClearTournamentGame {
    fn new() -> CMsgDOTAClearTournamentGame {
        CMsgDOTAClearTournamentGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAClearTournamentGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tournament_id",
                    CMsgDOTAClearTournamentGame::get_tournament_id_for_reflect,
                    CMsgDOTAClearTournamentGame::mut_tournament_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_id",
                    CMsgDOTAClearTournamentGame::get_game_id_for_reflect,
                    CMsgDOTAClearTournamentGame::mut_game_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAClearTournamentGame>(
                    "CMsgDOTAClearTournamentGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAClearTournamentGame {
    fn clear(&mut self) {
        self.clear_tournament_id();
        self.clear_game_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAClearTournamentGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAClearTournamentGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAWeekendTourneyPlayerSkillLevelStats {
    // message fields
    skill_level: ::std::option::Option<u32>,
    times_won_0: ::std::option::Option<u32>,
    times_won_1: ::std::option::Option<u32>,
    times_won_2: ::std::option::Option<u32>,
    times_won_3: ::std::option::Option<u32>,
    times_bye_and_lost: ::std::option::Option<u32>,
    times_bye_and_won: ::std::option::Option<u32>,
    times_unusual_champ: ::std::option::Option<u32>,
    total_games_won: ::std::option::Option<u32>,
    score: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAWeekendTourneyPlayerSkillLevelStats {}

impl CMsgDOTAWeekendTourneyPlayerSkillLevelStats {
    pub fn new() -> CMsgDOTAWeekendTourneyPlayerSkillLevelStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAWeekendTourneyPlayerSkillLevelStats {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAWeekendTourneyPlayerSkillLevelStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAWeekendTourneyPlayerSkillLevelStats,
        };
        unsafe {
            instance.get(CMsgDOTAWeekendTourneyPlayerSkillLevelStats::new)
        }
    }

    // optional uint32 skill_level = 1;

    pub fn clear_skill_level(&mut self) {
        self.skill_level = ::std::option::Option::None;
    }

    pub fn has_skill_level(&self) -> bool {
        self.skill_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_skill_level(&mut self, v: u32) {
        self.skill_level = ::std::option::Option::Some(v);
    }

    pub fn get_skill_level(&self) -> u32 {
        self.skill_level.unwrap_or(0)
    }

    fn get_skill_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.skill_level
    }

    fn mut_skill_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.skill_level
    }

    // optional uint32 times_won_0 = 2;

    pub fn clear_times_won_0(&mut self) {
        self.times_won_0 = ::std::option::Option::None;
    }

    pub fn has_times_won_0(&self) -> bool {
        self.times_won_0.is_some()
    }

    // Param is passed by value, moved
    pub fn set_times_won_0(&mut self, v: u32) {
        self.times_won_0 = ::std::option::Option::Some(v);
    }

    pub fn get_times_won_0(&self) -> u32 {
        self.times_won_0.unwrap_or(0)
    }

    fn get_times_won_0_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.times_won_0
    }

    fn mut_times_won_0_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.times_won_0
    }

    // optional uint32 times_won_1 = 3;

    pub fn clear_times_won_1(&mut self) {
        self.times_won_1 = ::std::option::Option::None;
    }

    pub fn has_times_won_1(&self) -> bool {
        self.times_won_1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_times_won_1(&mut self, v: u32) {
        self.times_won_1 = ::std::option::Option::Some(v);
    }

    pub fn get_times_won_1(&self) -> u32 {
        self.times_won_1.unwrap_or(0)
    }

    fn get_times_won_1_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.times_won_1
    }

    fn mut_times_won_1_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.times_won_1
    }

    // optional uint32 times_won_2 = 4;

    pub fn clear_times_won_2(&mut self) {
        self.times_won_2 = ::std::option::Option::None;
    }

    pub fn has_times_won_2(&self) -> bool {
        self.times_won_2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_times_won_2(&mut self, v: u32) {
        self.times_won_2 = ::std::option::Option::Some(v);
    }

    pub fn get_times_won_2(&self) -> u32 {
        self.times_won_2.unwrap_or(0)
    }

    fn get_times_won_2_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.times_won_2
    }

    fn mut_times_won_2_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.times_won_2
    }

    // optional uint32 times_won_3 = 5;

    pub fn clear_times_won_3(&mut self) {
        self.times_won_3 = ::std::option::Option::None;
    }

    pub fn has_times_won_3(&self) -> bool {
        self.times_won_3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_times_won_3(&mut self, v: u32) {
        self.times_won_3 = ::std::option::Option::Some(v);
    }

    pub fn get_times_won_3(&self) -> u32 {
        self.times_won_3.unwrap_or(0)
    }

    fn get_times_won_3_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.times_won_3
    }

    fn mut_times_won_3_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.times_won_3
    }

    // optional uint32 times_bye_and_lost = 6;

    pub fn clear_times_bye_and_lost(&mut self) {
        self.times_bye_and_lost = ::std::option::Option::None;
    }

    pub fn has_times_bye_and_lost(&self) -> bool {
        self.times_bye_and_lost.is_some()
    }

    // Param is passed by value, moved
    pub fn set_times_bye_and_lost(&mut self, v: u32) {
        self.times_bye_and_lost = ::std::option::Option::Some(v);
    }

    pub fn get_times_bye_and_lost(&self) -> u32 {
        self.times_bye_and_lost.unwrap_or(0)
    }

    fn get_times_bye_and_lost_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.times_bye_and_lost
    }

    fn mut_times_bye_and_lost_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.times_bye_and_lost
    }

    // optional uint32 times_bye_and_won = 7;

    pub fn clear_times_bye_and_won(&mut self) {
        self.times_bye_and_won = ::std::option::Option::None;
    }

    pub fn has_times_bye_and_won(&self) -> bool {
        self.times_bye_and_won.is_some()
    }

    // Param is passed by value, moved
    pub fn set_times_bye_and_won(&mut self, v: u32) {
        self.times_bye_and_won = ::std::option::Option::Some(v);
    }

    pub fn get_times_bye_and_won(&self) -> u32 {
        self.times_bye_and_won.unwrap_or(0)
    }

    fn get_times_bye_and_won_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.times_bye_and_won
    }

    fn mut_times_bye_and_won_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.times_bye_and_won
    }

    // optional uint32 times_unusual_champ = 10;

    pub fn clear_times_unusual_champ(&mut self) {
        self.times_unusual_champ = ::std::option::Option::None;
    }

    pub fn has_times_unusual_champ(&self) -> bool {
        self.times_unusual_champ.is_some()
    }

    // Param is passed by value, moved
    pub fn set_times_unusual_champ(&mut self, v: u32) {
        self.times_unusual_champ = ::std::option::Option::Some(v);
    }

    pub fn get_times_unusual_champ(&self) -> u32 {
        self.times_unusual_champ.unwrap_or(0)
    }

    fn get_times_unusual_champ_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.times_unusual_champ
    }

    fn mut_times_unusual_champ_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.times_unusual_champ
    }

    // optional uint32 total_games_won = 8;

    pub fn clear_total_games_won(&mut self) {
        self.total_games_won = ::std::option::Option::None;
    }

    pub fn has_total_games_won(&self) -> bool {
        self.total_games_won.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_games_won(&mut self, v: u32) {
        self.total_games_won = ::std::option::Option::Some(v);
    }

    pub fn get_total_games_won(&self) -> u32 {
        self.total_games_won.unwrap_or(0)
    }

    fn get_total_games_won_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.total_games_won
    }

    fn mut_total_games_won_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.total_games_won
    }

    // optional uint32 score = 9;

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
}

impl ::protobuf::Message for CMsgDOTAWeekendTourneyPlayerSkillLevelStats {
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
                    self.skill_level = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.times_won_0 = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.times_won_1 = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.times_won_2 = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.times_won_3 = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.times_bye_and_lost = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.times_bye_and_won = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.times_unusual_champ = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.total_games_won = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.score = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.skill_level {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.times_won_0 {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.times_won_1 {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.times_won_2 {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.times_won_3 {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.times_bye_and_lost {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.times_bye_and_won {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.times_unusual_champ {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.total_games_won {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.score {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.skill_level {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.times_won_0 {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.times_won_1 {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.times_won_2 {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.times_won_3 {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.times_bye_and_lost {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.times_bye_and_won {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.times_unusual_champ {
            os.write_uint32(10, v)?;
        }
        if let Some(v) = self.total_games_won {
            os.write_uint32(8, v)?;
        }
        if let Some(v) = self.score {
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

impl ::protobuf::MessageStatic for CMsgDOTAWeekendTourneyPlayerSkillLevelStats {
    fn new() -> CMsgDOTAWeekendTourneyPlayerSkillLevelStats {
        CMsgDOTAWeekendTourneyPlayerSkillLevelStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAWeekendTourneyPlayerSkillLevelStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "skill_level",
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::get_skill_level_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::mut_skill_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "times_won_0",
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::get_times_won_0_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::mut_times_won_0_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "times_won_1",
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::get_times_won_1_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::mut_times_won_1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "times_won_2",
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::get_times_won_2_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::mut_times_won_2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "times_won_3",
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::get_times_won_3_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::mut_times_won_3_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "times_bye_and_lost",
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::get_times_bye_and_lost_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::mut_times_bye_and_lost_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "times_bye_and_won",
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::get_times_bye_and_won_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::mut_times_bye_and_won_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "times_unusual_champ",
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::get_times_unusual_champ_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::mut_times_unusual_champ_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "total_games_won",
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::get_total_games_won_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::mut_total_games_won_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "score",
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::get_score_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerSkillLevelStats::mut_score_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAWeekendTourneyPlayerSkillLevelStats>(
                    "CMsgDOTAWeekendTourneyPlayerSkillLevelStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAWeekendTourneyPlayerSkillLevelStats {
    fn clear(&mut self) {
        self.clear_skill_level();
        self.clear_times_won_0();
        self.clear_times_won_1();
        self.clear_times_won_2();
        self.clear_times_won_3();
        self.clear_times_bye_and_lost();
        self.clear_times_bye_and_won();
        self.clear_times_unusual_champ();
        self.clear_total_games_won();
        self.clear_score();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAWeekendTourneyPlayerSkillLevelStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAWeekendTourneyPlayerSkillLevelStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAWeekendTourneyPlayerStats {
    // message fields
    account_id: ::std::option::Option<u32>,
    season_trophy_id: ::std::option::Option<u32>,
    skill_levels: ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyPlayerSkillLevelStats>,
    current_tier: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAWeekendTourneyPlayerStats {}

impl CMsgDOTAWeekendTourneyPlayerStats {
    pub fn new() -> CMsgDOTAWeekendTourneyPlayerStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAWeekendTourneyPlayerStats {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAWeekendTourneyPlayerStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAWeekendTourneyPlayerStats,
        };
        unsafe {
            instance.get(CMsgDOTAWeekendTourneyPlayerStats::new)
        }
    }

    // optional uint32 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u32 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id
    }

    // optional uint32 season_trophy_id = 2;

    pub fn clear_season_trophy_id(&mut self) {
        self.season_trophy_id = ::std::option::Option::None;
    }

    pub fn has_season_trophy_id(&self) -> bool {
        self.season_trophy_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_season_trophy_id(&mut self, v: u32) {
        self.season_trophy_id = ::std::option::Option::Some(v);
    }

    pub fn get_season_trophy_id(&self) -> u32 {
        self.season_trophy_id.unwrap_or(0)
    }

    fn get_season_trophy_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.season_trophy_id
    }

    fn mut_season_trophy_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.season_trophy_id
    }

    // repeated .CMsgDOTAWeekendTourneyPlayerSkillLevelStats skill_levels = 3;

    pub fn clear_skill_levels(&mut self) {
        self.skill_levels.clear();
    }

    // Param is passed by value, moved
    pub fn set_skill_levels(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyPlayerSkillLevelStats>) {
        self.skill_levels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_skill_levels(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyPlayerSkillLevelStats> {
        &mut self.skill_levels
    }

    // Take field
    pub fn take_skill_levels(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyPlayerSkillLevelStats> {
        ::std::mem::replace(&mut self.skill_levels, ::protobuf::RepeatedField::new())
    }

    pub fn get_skill_levels(&self) -> &[CMsgDOTAWeekendTourneyPlayerSkillLevelStats] {
        &self.skill_levels
    }

    fn get_skill_levels_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAWeekendTourneyPlayerSkillLevelStats> {
        &self.skill_levels
    }

    fn mut_skill_levels_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyPlayerSkillLevelStats> {
        &mut self.skill_levels
    }

    // optional uint32 current_tier = 4;

    pub fn clear_current_tier(&mut self) {
        self.current_tier = ::std::option::Option::None;
    }

    pub fn has_current_tier(&self) -> bool {
        self.current_tier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current_tier(&mut self, v: u32) {
        self.current_tier = ::std::option::Option::Some(v);
    }

    pub fn get_current_tier(&self) -> u32 {
        self.current_tier.unwrap_or(0)
    }

    fn get_current_tier_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.current_tier
    }

    fn mut_current_tier_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.current_tier
    }
}

impl ::protobuf::Message for CMsgDOTAWeekendTourneyPlayerStats {
    fn is_initialized(&self) -> bool {
        for v in &self.skill_levels {
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
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.season_trophy_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.skill_levels)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.current_tier = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.season_trophy_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.skill_levels {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.current_tier {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.season_trophy_id {
            os.write_uint32(2, v)?;
        }
        for v in &self.skill_levels {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.current_tier {
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

impl ::protobuf::MessageStatic for CMsgDOTAWeekendTourneyPlayerStats {
    fn new() -> CMsgDOTAWeekendTourneyPlayerStats {
        CMsgDOTAWeekendTourneyPlayerStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAWeekendTourneyPlayerStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTAWeekendTourneyPlayerStats::get_account_id_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerStats::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "season_trophy_id",
                    CMsgDOTAWeekendTourneyPlayerStats::get_season_trophy_id_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerStats::mut_season_trophy_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAWeekendTourneyPlayerSkillLevelStats>>(
                    "skill_levels",
                    CMsgDOTAWeekendTourneyPlayerStats::get_skill_levels_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerStats::mut_skill_levels_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "current_tier",
                    CMsgDOTAWeekendTourneyPlayerStats::get_current_tier_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerStats::mut_current_tier_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAWeekendTourneyPlayerStats>(
                    "CMsgDOTAWeekendTourneyPlayerStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAWeekendTourneyPlayerStats {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_season_trophy_id();
        self.clear_skill_levels();
        self.clear_current_tier();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAWeekendTourneyPlayerStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAWeekendTourneyPlayerStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAWeekendTourneyPlayerStatsRequest {
    // message fields
    account_id: ::std::option::Option<u32>,
    season_trophy_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAWeekendTourneyPlayerStatsRequest {}

impl CMsgDOTAWeekendTourneyPlayerStatsRequest {
    pub fn new() -> CMsgDOTAWeekendTourneyPlayerStatsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAWeekendTourneyPlayerStatsRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAWeekendTourneyPlayerStatsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAWeekendTourneyPlayerStatsRequest,
        };
        unsafe {
            instance.get(CMsgDOTAWeekendTourneyPlayerStatsRequest::new)
        }
    }

    // optional uint32 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u32 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id
    }

    // optional uint32 season_trophy_id = 2;

    pub fn clear_season_trophy_id(&mut self) {
        self.season_trophy_id = ::std::option::Option::None;
    }

    pub fn has_season_trophy_id(&self) -> bool {
        self.season_trophy_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_season_trophy_id(&mut self, v: u32) {
        self.season_trophy_id = ::std::option::Option::Some(v);
    }

    pub fn get_season_trophy_id(&self) -> u32 {
        self.season_trophy_id.unwrap_or(0)
    }

    fn get_season_trophy_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.season_trophy_id
    }

    fn mut_season_trophy_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.season_trophy_id
    }
}

impl ::protobuf::Message for CMsgDOTAWeekendTourneyPlayerStatsRequest {
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
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.season_trophy_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.season_trophy_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.season_trophy_id {
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

impl ::protobuf::MessageStatic for CMsgDOTAWeekendTourneyPlayerStatsRequest {
    fn new() -> CMsgDOTAWeekendTourneyPlayerStatsRequest {
        CMsgDOTAWeekendTourneyPlayerStatsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAWeekendTourneyPlayerStatsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTAWeekendTourneyPlayerStatsRequest::get_account_id_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerStatsRequest::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "season_trophy_id",
                    CMsgDOTAWeekendTourneyPlayerStatsRequest::get_season_trophy_id_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerStatsRequest::mut_season_trophy_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAWeekendTourneyPlayerStatsRequest>(
                    "CMsgDOTAWeekendTourneyPlayerStatsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAWeekendTourneyPlayerStatsRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_season_trophy_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAWeekendTourneyPlayerStatsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAWeekendTourneyPlayerStatsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAWeekendTourneyPlayerHistoryRequest {
    // message fields
    account_id: ::std::option::Option<u32>,
    season_trophy_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAWeekendTourneyPlayerHistoryRequest {}

impl CMsgDOTAWeekendTourneyPlayerHistoryRequest {
    pub fn new() -> CMsgDOTAWeekendTourneyPlayerHistoryRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAWeekendTourneyPlayerHistoryRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAWeekendTourneyPlayerHistoryRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAWeekendTourneyPlayerHistoryRequest,
        };
        unsafe {
            instance.get(CMsgDOTAWeekendTourneyPlayerHistoryRequest::new)
        }
    }

    // optional uint32 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u32 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id
    }

    // optional uint32 season_trophy_id = 2;

    pub fn clear_season_trophy_id(&mut self) {
        self.season_trophy_id = ::std::option::Option::None;
    }

    pub fn has_season_trophy_id(&self) -> bool {
        self.season_trophy_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_season_trophy_id(&mut self, v: u32) {
        self.season_trophy_id = ::std::option::Option::Some(v);
    }

    pub fn get_season_trophy_id(&self) -> u32 {
        self.season_trophy_id.unwrap_or(0)
    }

    fn get_season_trophy_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.season_trophy_id
    }

    fn mut_season_trophy_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.season_trophy_id
    }
}

impl ::protobuf::Message for CMsgDOTAWeekendTourneyPlayerHistoryRequest {
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
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.season_trophy_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.season_trophy_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.season_trophy_id {
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

impl ::protobuf::MessageStatic for CMsgDOTAWeekendTourneyPlayerHistoryRequest {
    fn new() -> CMsgDOTAWeekendTourneyPlayerHistoryRequest {
        CMsgDOTAWeekendTourneyPlayerHistoryRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAWeekendTourneyPlayerHistoryRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTAWeekendTourneyPlayerHistoryRequest::get_account_id_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerHistoryRequest::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "season_trophy_id",
                    CMsgDOTAWeekendTourneyPlayerHistoryRequest::get_season_trophy_id_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerHistoryRequest::mut_season_trophy_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAWeekendTourneyPlayerHistoryRequest>(
                    "CMsgDOTAWeekendTourneyPlayerHistoryRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAWeekendTourneyPlayerHistoryRequest {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_season_trophy_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAWeekendTourneyPlayerHistoryRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAWeekendTourneyPlayerHistoryRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAWeekendTourneyPlayerHistory {
    // message fields
    account_id: ::std::option::Option<u32>,
    tournaments: ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyPlayerHistory_Tournament>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAWeekendTourneyPlayerHistory {}

impl CMsgDOTAWeekendTourneyPlayerHistory {
    pub fn new() -> CMsgDOTAWeekendTourneyPlayerHistory {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAWeekendTourneyPlayerHistory {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAWeekendTourneyPlayerHistory> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAWeekendTourneyPlayerHistory,
        };
        unsafe {
            instance.get(CMsgDOTAWeekendTourneyPlayerHistory::new)
        }
    }

    // optional uint32 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u32 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id
    }

    // repeated .CMsgDOTAWeekendTourneyPlayerHistory.Tournament tournaments = 3;

    pub fn clear_tournaments(&mut self) {
        self.tournaments.clear();
    }

    // Param is passed by value, moved
    pub fn set_tournaments(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyPlayerHistory_Tournament>) {
        self.tournaments = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tournaments(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyPlayerHistory_Tournament> {
        &mut self.tournaments
    }

    // Take field
    pub fn take_tournaments(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyPlayerHistory_Tournament> {
        ::std::mem::replace(&mut self.tournaments, ::protobuf::RepeatedField::new())
    }

    pub fn get_tournaments(&self) -> &[CMsgDOTAWeekendTourneyPlayerHistory_Tournament] {
        &self.tournaments
    }

    fn get_tournaments_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAWeekendTourneyPlayerHistory_Tournament> {
        &self.tournaments
    }

    fn mut_tournaments_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyPlayerHistory_Tournament> {
        &mut self.tournaments
    }
}

impl ::protobuf::Message for CMsgDOTAWeekendTourneyPlayerHistory {
    fn is_initialized(&self) -> bool {
        for v in &self.tournaments {
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
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tournaments)?;
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.tournaments {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        }
        for v in &self.tournaments {
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

impl ::protobuf::MessageStatic for CMsgDOTAWeekendTourneyPlayerHistory {
    fn new() -> CMsgDOTAWeekendTourneyPlayerHistory {
        CMsgDOTAWeekendTourneyPlayerHistory::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAWeekendTourneyPlayerHistory>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTAWeekendTourneyPlayerHistory::get_account_id_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerHistory::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAWeekendTourneyPlayerHistory_Tournament>>(
                    "tournaments",
                    CMsgDOTAWeekendTourneyPlayerHistory::get_tournaments_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerHistory::mut_tournaments_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAWeekendTourneyPlayerHistory>(
                    "CMsgDOTAWeekendTourneyPlayerHistory",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAWeekendTourneyPlayerHistory {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_tournaments();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAWeekendTourneyPlayerHistory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAWeekendTourneyPlayerHistory {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAWeekendTourneyPlayerHistory_Tournament {
    // message fields
    tournament_id: ::std::option::Option<u32>,
    start_time: ::std::option::Option<u32>,
    tournament_tier: ::std::option::Option<u32>,
    team_id: ::std::option::Option<u32>,
    team_date: ::std::option::Option<u32>,
    team_result: ::std::option::Option<u32>,
    account_id: ::std::vec::Vec<u32>,
    team_name: ::protobuf::SingularField<::std::string::String>,
    season_trophy_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAWeekendTourneyPlayerHistory_Tournament {}

impl CMsgDOTAWeekendTourneyPlayerHistory_Tournament {
    pub fn new() -> CMsgDOTAWeekendTourneyPlayerHistory_Tournament {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAWeekendTourneyPlayerHistory_Tournament {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAWeekendTourneyPlayerHistory_Tournament> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAWeekendTourneyPlayerHistory_Tournament,
        };
        unsafe {
            instance.get(CMsgDOTAWeekendTourneyPlayerHistory_Tournament::new)
        }
    }

    // optional uint32 tournament_id = 1;

    pub fn clear_tournament_id(&mut self) {
        self.tournament_id = ::std::option::Option::None;
    }

    pub fn has_tournament_id(&self) -> bool {
        self.tournament_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tournament_id(&mut self, v: u32) {
        self.tournament_id = ::std::option::Option::Some(v);
    }

    pub fn get_tournament_id(&self) -> u32 {
        self.tournament_id.unwrap_or(0)
    }

    fn get_tournament_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tournament_id
    }

    fn mut_tournament_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tournament_id
    }

    // optional uint32 start_time = 2;

    pub fn clear_start_time(&mut self) {
        self.start_time = ::std::option::Option::None;
    }

    pub fn has_start_time(&self) -> bool {
        self.start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_time(&mut self, v: u32) {
        self.start_time = ::std::option::Option::Some(v);
    }

    pub fn get_start_time(&self) -> u32 {
        self.start_time.unwrap_or(0)
    }

    fn get_start_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.start_time
    }

    fn mut_start_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.start_time
    }

    // optional uint32 tournament_tier = 3;

    pub fn clear_tournament_tier(&mut self) {
        self.tournament_tier = ::std::option::Option::None;
    }

    pub fn has_tournament_tier(&self) -> bool {
        self.tournament_tier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tournament_tier(&mut self, v: u32) {
        self.tournament_tier = ::std::option::Option::Some(v);
    }

    pub fn get_tournament_tier(&self) -> u32 {
        self.tournament_tier.unwrap_or(0)
    }

    fn get_tournament_tier_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tournament_tier
    }

    fn mut_tournament_tier_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tournament_tier
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

    // optional uint32 team_date = 5;

    pub fn clear_team_date(&mut self) {
        self.team_date = ::std::option::Option::None;
    }

    pub fn has_team_date(&self) -> bool {
        self.team_date.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_date(&mut self, v: u32) {
        self.team_date = ::std::option::Option::Some(v);
    }

    pub fn get_team_date(&self) -> u32 {
        self.team_date.unwrap_or(0)
    }

    fn get_team_date_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_date
    }

    fn mut_team_date_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_date
    }

    // optional uint32 team_result = 6;

    pub fn clear_team_result(&mut self) {
        self.team_result = ::std::option::Option::None;
    }

    pub fn has_team_result(&self) -> bool {
        self.team_result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_result(&mut self, v: u32) {
        self.team_result = ::std::option::Option::Some(v);
    }

    pub fn get_team_result(&self) -> u32 {
        self.team_result.unwrap_or(0)
    }

    fn get_team_result_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_result
    }

    fn mut_team_result_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_result
    }

    // repeated uint32 account_id = 7;

    pub fn clear_account_id(&mut self) {
        self.account_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: ::std::vec::Vec<u32>) {
        self.account_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_account_id(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.account_id
    }

    // Take field
    pub fn take_account_id(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.account_id, ::std::vec::Vec::new())
    }

    pub fn get_account_id(&self) -> &[u32] {
        &self.account_id
    }

    fn get_account_id_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.account_id
    }

    // optional string team_name = 8;

    pub fn clear_team_name(&mut self) {
        self.team_name.clear();
    }

    pub fn has_team_name(&self) -> bool {
        self.team_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_name(&mut self, v: ::std::string::String) {
        self.team_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team_name(&mut self) -> &mut ::std::string::String {
        if self.team_name.is_none() {
            self.team_name.set_default();
        }
        self.team_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_team_name(&mut self) -> ::std::string::String {
        self.team_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_team_name(&self) -> &str {
        match self.team_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_team_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.team_name
    }

    fn mut_team_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.team_name
    }

    // optional uint32 season_trophy_id = 9;

    pub fn clear_season_trophy_id(&mut self) {
        self.season_trophy_id = ::std::option::Option::None;
    }

    pub fn has_season_trophy_id(&self) -> bool {
        self.season_trophy_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_season_trophy_id(&mut self, v: u32) {
        self.season_trophy_id = ::std::option::Option::Some(v);
    }

    pub fn get_season_trophy_id(&self) -> u32 {
        self.season_trophy_id.unwrap_or(0)
    }

    fn get_season_trophy_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.season_trophy_id
    }

    fn mut_season_trophy_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.season_trophy_id
    }
}

impl ::protobuf::Message for CMsgDOTAWeekendTourneyPlayerHistory_Tournament {
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
                    self.tournament_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.start_time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.tournament_tier = ::std::option::Option::Some(tmp);
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
                    self.team_date = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team_result = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.account_id)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team_name)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.season_trophy_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.tournament_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.start_time {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.tournament_tier {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team_date {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team_result {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.account_id {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(ref v) = self.team_name.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(v) = self.season_trophy_id {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tournament_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.start_time {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.tournament_tier {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.team_id {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.team_date {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.team_result {
            os.write_uint32(6, v)?;
        }
        for v in &self.account_id {
            os.write_uint32(7, *v)?;
        };
        if let Some(ref v) = self.team_name.as_ref() {
            os.write_string(8, &v)?;
        }
        if let Some(v) = self.season_trophy_id {
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

impl ::protobuf::MessageStatic for CMsgDOTAWeekendTourneyPlayerHistory_Tournament {
    fn new() -> CMsgDOTAWeekendTourneyPlayerHistory_Tournament {
        CMsgDOTAWeekendTourneyPlayerHistory_Tournament::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAWeekendTourneyPlayerHistory_Tournament>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tournament_id",
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::get_tournament_id_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::mut_tournament_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "start_time",
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::get_start_time_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::mut_start_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tournament_tier",
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::get_tournament_tier_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::mut_tournament_tier_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::get_team_id_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_date",
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::get_team_date_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::mut_team_date_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_result",
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::get_team_result_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::mut_team_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::get_account_id_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team_name",
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::get_team_name_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::mut_team_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "season_trophy_id",
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::get_season_trophy_id_for_reflect,
                    CMsgDOTAWeekendTourneyPlayerHistory_Tournament::mut_season_trophy_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAWeekendTourneyPlayerHistory_Tournament>(
                    "CMsgDOTAWeekendTourneyPlayerHistory_Tournament",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAWeekendTourneyPlayerHistory_Tournament {
    fn clear(&mut self) {
        self.clear_tournament_id();
        self.clear_start_time();
        self.clear_tournament_tier();
        self.clear_team_id();
        self.clear_team_date();
        self.clear_team_result();
        self.clear_account_id();
        self.clear_team_name();
        self.clear_season_trophy_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAWeekendTourneyPlayerHistory_Tournament {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAWeekendTourneyPlayerHistory_Tournament {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAWeekendTourneyParticipationDetails {
    // message fields
    divisions: ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyParticipationDetails_Division>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAWeekendTourneyParticipationDetails {}

impl CMsgDOTAWeekendTourneyParticipationDetails {
    pub fn new() -> CMsgDOTAWeekendTourneyParticipationDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAWeekendTourneyParticipationDetails {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAWeekendTourneyParticipationDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAWeekendTourneyParticipationDetails,
        };
        unsafe {
            instance.get(CMsgDOTAWeekendTourneyParticipationDetails::new)
        }
    }

    // repeated .CMsgDOTAWeekendTourneyParticipationDetails.Division divisions = 1;

    pub fn clear_divisions(&mut self) {
        self.divisions.clear();
    }

    // Param is passed by value, moved
    pub fn set_divisions(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyParticipationDetails_Division>) {
        self.divisions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_divisions(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyParticipationDetails_Division> {
        &mut self.divisions
    }

    // Take field
    pub fn take_divisions(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyParticipationDetails_Division> {
        ::std::mem::replace(&mut self.divisions, ::protobuf::RepeatedField::new())
    }

    pub fn get_divisions(&self) -> &[CMsgDOTAWeekendTourneyParticipationDetails_Division] {
        &self.divisions
    }

    fn get_divisions_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAWeekendTourneyParticipationDetails_Division> {
        &self.divisions
    }

    fn mut_divisions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyParticipationDetails_Division> {
        &mut self.divisions
    }
}

impl ::protobuf::Message for CMsgDOTAWeekendTourneyParticipationDetails {
    fn is_initialized(&self) -> bool {
        for v in &self.divisions {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.divisions)?;
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
        for value in &self.divisions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.divisions {
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

impl ::protobuf::MessageStatic for CMsgDOTAWeekendTourneyParticipationDetails {
    fn new() -> CMsgDOTAWeekendTourneyParticipationDetails {
        CMsgDOTAWeekendTourneyParticipationDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAWeekendTourneyParticipationDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAWeekendTourneyParticipationDetails_Division>>(
                    "divisions",
                    CMsgDOTAWeekendTourneyParticipationDetails::get_divisions_for_reflect,
                    CMsgDOTAWeekendTourneyParticipationDetails::mut_divisions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAWeekendTourneyParticipationDetails>(
                    "CMsgDOTAWeekendTourneyParticipationDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAWeekendTourneyParticipationDetails {
    fn clear(&mut self) {
        self.clear_divisions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAWeekendTourneyParticipationDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAWeekendTourneyParticipationDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAWeekendTourneyParticipationDetails_Tier {
    // message fields
    tier: ::std::option::Option<u32>,
    players: ::std::option::Option<u32>,
    teams: ::std::option::Option<u32>,
    winning_teams: ::std::option::Option<u32>,
    players_streak_2: ::std::option::Option<u32>,
    players_streak_3: ::std::option::Option<u32>,
    players_streak_4: ::std::option::Option<u32>,
    players_streak_5: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAWeekendTourneyParticipationDetails_Tier {}

impl CMsgDOTAWeekendTourneyParticipationDetails_Tier {
    pub fn new() -> CMsgDOTAWeekendTourneyParticipationDetails_Tier {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAWeekendTourneyParticipationDetails_Tier {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAWeekendTourneyParticipationDetails_Tier> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAWeekendTourneyParticipationDetails_Tier,
        };
        unsafe {
            instance.get(CMsgDOTAWeekendTourneyParticipationDetails_Tier::new)
        }
    }

    // optional uint32 tier = 1;

    pub fn clear_tier(&mut self) {
        self.tier = ::std::option::Option::None;
    }

    pub fn has_tier(&self) -> bool {
        self.tier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tier(&mut self, v: u32) {
        self.tier = ::std::option::Option::Some(v);
    }

    pub fn get_tier(&self) -> u32 {
        self.tier.unwrap_or(0)
    }

    fn get_tier_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tier
    }

    fn mut_tier_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tier
    }

    // optional uint32 players = 2;

    pub fn clear_players(&mut self) {
        self.players = ::std::option::Option::None;
    }

    pub fn has_players(&self) -> bool {
        self.players.is_some()
    }

    // Param is passed by value, moved
    pub fn set_players(&mut self, v: u32) {
        self.players = ::std::option::Option::Some(v);
    }

    pub fn get_players(&self) -> u32 {
        self.players.unwrap_or(0)
    }

    fn get_players_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.players
    }

    fn mut_players_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.players
    }

    // optional uint32 teams = 3;

    pub fn clear_teams(&mut self) {
        self.teams = ::std::option::Option::None;
    }

    pub fn has_teams(&self) -> bool {
        self.teams.is_some()
    }

    // Param is passed by value, moved
    pub fn set_teams(&mut self, v: u32) {
        self.teams = ::std::option::Option::Some(v);
    }

    pub fn get_teams(&self) -> u32 {
        self.teams.unwrap_or(0)
    }

    fn get_teams_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.teams
    }

    fn mut_teams_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.teams
    }

    // optional uint32 winning_teams = 4;

    pub fn clear_winning_teams(&mut self) {
        self.winning_teams = ::std::option::Option::None;
    }

    pub fn has_winning_teams(&self) -> bool {
        self.winning_teams.is_some()
    }

    // Param is passed by value, moved
    pub fn set_winning_teams(&mut self, v: u32) {
        self.winning_teams = ::std::option::Option::Some(v);
    }

    pub fn get_winning_teams(&self) -> u32 {
        self.winning_teams.unwrap_or(0)
    }

    fn get_winning_teams_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.winning_teams
    }

    fn mut_winning_teams_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.winning_teams
    }

    // optional uint32 players_streak_2 = 5;

    pub fn clear_players_streak_2(&mut self) {
        self.players_streak_2 = ::std::option::Option::None;
    }

    pub fn has_players_streak_2(&self) -> bool {
        self.players_streak_2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_players_streak_2(&mut self, v: u32) {
        self.players_streak_2 = ::std::option::Option::Some(v);
    }

    pub fn get_players_streak_2(&self) -> u32 {
        self.players_streak_2.unwrap_or(0)
    }

    fn get_players_streak_2_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.players_streak_2
    }

    fn mut_players_streak_2_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.players_streak_2
    }

    // optional uint32 players_streak_3 = 6;

    pub fn clear_players_streak_3(&mut self) {
        self.players_streak_3 = ::std::option::Option::None;
    }

    pub fn has_players_streak_3(&self) -> bool {
        self.players_streak_3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_players_streak_3(&mut self, v: u32) {
        self.players_streak_3 = ::std::option::Option::Some(v);
    }

    pub fn get_players_streak_3(&self) -> u32 {
        self.players_streak_3.unwrap_or(0)
    }

    fn get_players_streak_3_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.players_streak_3
    }

    fn mut_players_streak_3_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.players_streak_3
    }

    // optional uint32 players_streak_4 = 7;

    pub fn clear_players_streak_4(&mut self) {
        self.players_streak_4 = ::std::option::Option::None;
    }

    pub fn has_players_streak_4(&self) -> bool {
        self.players_streak_4.is_some()
    }

    // Param is passed by value, moved
    pub fn set_players_streak_4(&mut self, v: u32) {
        self.players_streak_4 = ::std::option::Option::Some(v);
    }

    pub fn get_players_streak_4(&self) -> u32 {
        self.players_streak_4.unwrap_or(0)
    }

    fn get_players_streak_4_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.players_streak_4
    }

    fn mut_players_streak_4_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.players_streak_4
    }

    // optional uint32 players_streak_5 = 8;

    pub fn clear_players_streak_5(&mut self) {
        self.players_streak_5 = ::std::option::Option::None;
    }

    pub fn has_players_streak_5(&self) -> bool {
        self.players_streak_5.is_some()
    }

    // Param is passed by value, moved
    pub fn set_players_streak_5(&mut self, v: u32) {
        self.players_streak_5 = ::std::option::Option::Some(v);
    }

    pub fn get_players_streak_5(&self) -> u32 {
        self.players_streak_5.unwrap_or(0)
    }

    fn get_players_streak_5_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.players_streak_5
    }

    fn mut_players_streak_5_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.players_streak_5
    }
}

impl ::protobuf::Message for CMsgDOTAWeekendTourneyParticipationDetails_Tier {
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
                    self.tier = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.players = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.teams = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.winning_teams = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.players_streak_2 = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.players_streak_3 = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.players_streak_4 = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.players_streak_5 = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.tier {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.players {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.teams {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.winning_teams {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.players_streak_2 {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.players_streak_3 {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.players_streak_4 {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.players_streak_5 {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tier {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.players {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.teams {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.winning_teams {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.players_streak_2 {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.players_streak_3 {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.players_streak_4 {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.players_streak_5 {
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

impl ::protobuf::MessageStatic for CMsgDOTAWeekendTourneyParticipationDetails_Tier {
    fn new() -> CMsgDOTAWeekendTourneyParticipationDetails_Tier {
        CMsgDOTAWeekendTourneyParticipationDetails_Tier::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAWeekendTourneyParticipationDetails_Tier>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tier",
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::get_tier_for_reflect,
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::mut_tier_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "players",
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::get_players_for_reflect,
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::mut_players_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "teams",
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::get_teams_for_reflect,
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::mut_teams_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "winning_teams",
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::get_winning_teams_for_reflect,
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::mut_winning_teams_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "players_streak_2",
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::get_players_streak_2_for_reflect,
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::mut_players_streak_2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "players_streak_3",
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::get_players_streak_3_for_reflect,
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::mut_players_streak_3_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "players_streak_4",
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::get_players_streak_4_for_reflect,
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::mut_players_streak_4_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "players_streak_5",
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::get_players_streak_5_for_reflect,
                    CMsgDOTAWeekendTourneyParticipationDetails_Tier::mut_players_streak_5_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAWeekendTourneyParticipationDetails_Tier>(
                    "CMsgDOTAWeekendTourneyParticipationDetails_Tier",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAWeekendTourneyParticipationDetails_Tier {
    fn clear(&mut self) {
        self.clear_tier();
        self.clear_players();
        self.clear_teams();
        self.clear_winning_teams();
        self.clear_players_streak_2();
        self.clear_players_streak_3();
        self.clear_players_streak_4();
        self.clear_players_streak_5();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAWeekendTourneyParticipationDetails_Tier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAWeekendTourneyParticipationDetails_Tier {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAWeekendTourneyParticipationDetails_Division {
    // message fields
    division_id: ::std::option::Option<u32>,
    schedule_time: ::std::option::Option<u32>,
    tiers: ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyParticipationDetails_Tier>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAWeekendTourneyParticipationDetails_Division {}

impl CMsgDOTAWeekendTourneyParticipationDetails_Division {
    pub fn new() -> CMsgDOTAWeekendTourneyParticipationDetails_Division {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAWeekendTourneyParticipationDetails_Division {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAWeekendTourneyParticipationDetails_Division> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAWeekendTourneyParticipationDetails_Division,
        };
        unsafe {
            instance.get(CMsgDOTAWeekendTourneyParticipationDetails_Division::new)
        }
    }

    // optional uint32 division_id = 1;

    pub fn clear_division_id(&mut self) {
        self.division_id = ::std::option::Option::None;
    }

    pub fn has_division_id(&self) -> bool {
        self.division_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_division_id(&mut self, v: u32) {
        self.division_id = ::std::option::Option::Some(v);
    }

    pub fn get_division_id(&self) -> u32 {
        self.division_id.unwrap_or(0)
    }

    fn get_division_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.division_id
    }

    fn mut_division_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.division_id
    }

    // optional uint32 schedule_time = 2;

    pub fn clear_schedule_time(&mut self) {
        self.schedule_time = ::std::option::Option::None;
    }

    pub fn has_schedule_time(&self) -> bool {
        self.schedule_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schedule_time(&mut self, v: u32) {
        self.schedule_time = ::std::option::Option::Some(v);
    }

    pub fn get_schedule_time(&self) -> u32 {
        self.schedule_time.unwrap_or(0)
    }

    fn get_schedule_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.schedule_time
    }

    fn mut_schedule_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.schedule_time
    }

    // repeated .CMsgDOTAWeekendTourneyParticipationDetails.Tier tiers = 3;

    pub fn clear_tiers(&mut self) {
        self.tiers.clear();
    }

    // Param is passed by value, moved
    pub fn set_tiers(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyParticipationDetails_Tier>) {
        self.tiers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tiers(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyParticipationDetails_Tier> {
        &mut self.tiers
    }

    // Take field
    pub fn take_tiers(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyParticipationDetails_Tier> {
        ::std::mem::replace(&mut self.tiers, ::protobuf::RepeatedField::new())
    }

    pub fn get_tiers(&self) -> &[CMsgDOTAWeekendTourneyParticipationDetails_Tier] {
        &self.tiers
    }

    fn get_tiers_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAWeekendTourneyParticipationDetails_Tier> {
        &self.tiers
    }

    fn mut_tiers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAWeekendTourneyParticipationDetails_Tier> {
        &mut self.tiers
    }
}

impl ::protobuf::Message for CMsgDOTAWeekendTourneyParticipationDetails_Division {
    fn is_initialized(&self) -> bool {
        for v in &self.tiers {
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
                    self.division_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.schedule_time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tiers)?;
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
        if let Some(v) = self.division_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.schedule_time {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.tiers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.division_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.schedule_time {
            os.write_uint32(2, v)?;
        }
        for v in &self.tiers {
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

impl ::protobuf::MessageStatic for CMsgDOTAWeekendTourneyParticipationDetails_Division {
    fn new() -> CMsgDOTAWeekendTourneyParticipationDetails_Division {
        CMsgDOTAWeekendTourneyParticipationDetails_Division::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAWeekendTourneyParticipationDetails_Division>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "division_id",
                    CMsgDOTAWeekendTourneyParticipationDetails_Division::get_division_id_for_reflect,
                    CMsgDOTAWeekendTourneyParticipationDetails_Division::mut_division_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "schedule_time",
                    CMsgDOTAWeekendTourneyParticipationDetails_Division::get_schedule_time_for_reflect,
                    CMsgDOTAWeekendTourneyParticipationDetails_Division::mut_schedule_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAWeekendTourneyParticipationDetails_Tier>>(
                    "tiers",
                    CMsgDOTAWeekendTourneyParticipationDetails_Division::get_tiers_for_reflect,
                    CMsgDOTAWeekendTourneyParticipationDetails_Division::mut_tiers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAWeekendTourneyParticipationDetails_Division>(
                    "CMsgDOTAWeekendTourneyParticipationDetails_Division",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAWeekendTourneyParticipationDetails_Division {
    fn clear(&mut self) {
        self.clear_division_id();
        self.clear_schedule_time();
        self.clear_tiers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAWeekendTourneyParticipationDetails_Division {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAWeekendTourneyParticipationDetails_Division {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ETournamentEvent {
    k_ETournamentEvent_None = 0,
    k_ETournamentEvent_TournamentCreated = 1,
    k_ETournamentEvent_TournamentsMerged = 2,
    k_ETournamentEvent_GameOutcome = 3,
    k_ETournamentEvent_TeamGivenBye = 4,
    k_ETournamentEvent_TournamentCanceledByAdmin = 5,
    k_ETournamentEvent_TeamAbandoned = 6,
    k_ETournamentEvent_ScheduledGameStarted = 7,
    k_ETournamentEvent_Canceled = 8,
    k_ETournamentEvent_TeamParticipationTimedOut_EntryFeeRefund = 9,
    k_ETournamentEvent_TeamParticipationTimedOut_EntryFeeForfeit = 10,
    k_ETournamentEvent_TeamParticipationTimedOut_GrantedVictory = 11,
}

impl ::protobuf::ProtobufEnum for ETournamentEvent {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ETournamentEvent> {
        match value {
            0 => ::std::option::Option::Some(ETournamentEvent::k_ETournamentEvent_None),
            1 => ::std::option::Option::Some(ETournamentEvent::k_ETournamentEvent_TournamentCreated),
            2 => ::std::option::Option::Some(ETournamentEvent::k_ETournamentEvent_TournamentsMerged),
            3 => ::std::option::Option::Some(ETournamentEvent::k_ETournamentEvent_GameOutcome),
            4 => ::std::option::Option::Some(ETournamentEvent::k_ETournamentEvent_TeamGivenBye),
            5 => ::std::option::Option::Some(ETournamentEvent::k_ETournamentEvent_TournamentCanceledByAdmin),
            6 => ::std::option::Option::Some(ETournamentEvent::k_ETournamentEvent_TeamAbandoned),
            7 => ::std::option::Option::Some(ETournamentEvent::k_ETournamentEvent_ScheduledGameStarted),
            8 => ::std::option::Option::Some(ETournamentEvent::k_ETournamentEvent_Canceled),
            9 => ::std::option::Option::Some(ETournamentEvent::k_ETournamentEvent_TeamParticipationTimedOut_EntryFeeRefund),
            10 => ::std::option::Option::Some(ETournamentEvent::k_ETournamentEvent_TeamParticipationTimedOut_EntryFeeForfeit),
            11 => ::std::option::Option::Some(ETournamentEvent::k_ETournamentEvent_TeamParticipationTimedOut_GrantedVictory),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ETournamentEvent] = &[
            ETournamentEvent::k_ETournamentEvent_None,
            ETournamentEvent::k_ETournamentEvent_TournamentCreated,
            ETournamentEvent::k_ETournamentEvent_TournamentsMerged,
            ETournamentEvent::k_ETournamentEvent_GameOutcome,
            ETournamentEvent::k_ETournamentEvent_TeamGivenBye,
            ETournamentEvent::k_ETournamentEvent_TournamentCanceledByAdmin,
            ETournamentEvent::k_ETournamentEvent_TeamAbandoned,
            ETournamentEvent::k_ETournamentEvent_ScheduledGameStarted,
            ETournamentEvent::k_ETournamentEvent_Canceled,
            ETournamentEvent::k_ETournamentEvent_TeamParticipationTimedOut_EntryFeeRefund,
            ETournamentEvent::k_ETournamentEvent_TeamParticipationTimedOut_EntryFeeForfeit,
            ETournamentEvent::k_ETournamentEvent_TeamParticipationTimedOut_GrantedVictory,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ETournamentEvent>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ETournamentEvent", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ETournamentEvent {
}

impl ::protobuf::reflect::ProtobufValue for ETournamentEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'dota_gcmessages_client_tournament.proto\x1a\x17dota_client_enums.prot\
    o\"\xb0\x0f\n\x16CMsgDOTATournamentInfo\x12\x1b\n\tleague_id\x18\x01\x20\
    \x01(\rR\x08leagueId\x12<\n\nphase_list\x18\x02\x20\x03(\x0b2\x1d.CMsgDO\
    TATournamentInfo.PhaseR\tphaseList\x12;\n\nteams_list\x18\x03\x20\x03(\
    \x0b2\x1c.CMsgDOTATournamentInfo.TeamR\tteamsList\x12Y\n\x15upcoming_mat\
    ches_list\x18\x04\x20\x03(\x0b2%.CMsgDOTATournamentInfo.UpcomingMatchR\
    \x13upcomingMatchesList\x129\n\tnews_list\x18\x05\x20\x03(\x0b2\x1c.CMsg\
    DOTATournamentInfo.NewsR\x08newsList\x1aF\n\nPhaseGroup\x12\x19\n\x08gro\
    up_id\x18\x01\x20\x01(\rR\x07groupId\x12\x1d\n\ngroup_name\x18\x02\x20\
    \x01(\tR\tgroupName\x1a\x89\x02\n\x05Phase\x12\x19\n\x08phase_id\x18\x01\
    \x20\x01(\rR\x07phaseId\x12\x1d\n\nphase_name\x18\x02\x20\x01(\tR\tphase\
    Name\x12\x17\n\x07type_id\x18\x03\x20\x01(\rR\x06typeId\x12\x1e\n\nitera\
    tions\x18\x04\x20\x01(\rR\niterations\x12$\n\x0emin_start_time\x18\x05\
    \x20\x01(\rR\x0cminStartTime\x12$\n\x0emax_start_time\x18\x06\x20\x01(\r\
    R\x0cmaxStartTime\x12A\n\ngroup_list\x18\x07\x20\x03(\x0b2\".CMsgDOTATou\
    rnamentInfo.PhaseGroupR\tgroupList\x1a\x82\x01\n\x04Team\x12\x17\n\x07te\
    am_id\x18\x01\x20\x01(\rR\x06teamId\x12\x12\n\x04name\x18\x02\x20\x01(\t\
    R\x04name\x12\x10\n\x03tag\x18\x03\x20\x01(\tR\x03tag\x12\x1b\n\tteam_lo\
    go\x18\x04\x20\x01(\x04R\x08teamLogo\x12\x1e\n\neliminated\x18\x05\x20\
    \x01(\x08R\neliminated\x1a\xa8\x08\n\rUpcomingMatch\x12\x1b\n\tseries_id\
    \x18\x01\x20\x01(\rR\x08seriesId\x12\x19\n\x08team1_id\x18\x02\x20\x01(\
    \rR\x07team1Id\x12\x19\n\x08team2_id\x18\x03\x20\x01(\rR\x07team2Id\x12\
    \x0e\n\x02bo\x18\x04\x20\x01(\rR\x02bo\x12\x1d\n\nstage_name\x18\x05\x20\
    \x01(\tR\tstageName\x12\x1d\n\nstart_time\x18\x06\x20\x01(\rR\tstartTime\
    \x12!\n\x0cwinner_stage\x18\x07\x20\x01(\tR\x0bwinnerStage\x12\x1f\n\x0b\
    loser_stage\x18\x08\x20\x01(\tR\nloserStage\x12\x1b\n\tteam1_tag\x18\t\
    \x20\x01(\tR\x08team1Tag\x12\x1b\n\tteam2_tag\x18\n\x20\x01(\tR\x08team2\
    Tag\x125\n\x17team1_prev_opponent_tag\x18\x0b\x20\x01(\tR\x14team1PrevOp\
    ponentTag\x125\n\x17team2_prev_opponent_tag\x18\x0c\x20\x01(\tR\x14team2\
    PrevOpponentTag\x12\x1d\n\nteam1_logo\x18\r\x20\x01(\x04R\tteam1Logo\x12\
    \x1d\n\nteam2_logo\x18\x0e\x20\x01(\x04R\tteam2Logo\x127\n\x18team1_prev\
    _opponent_logo\x18\x0f\x20\x01(\x04R\x15team1PrevOpponentLogo\x127\n\x18\
    team2_prev_opponent_logo\x18\x10\x20\x01(\x04R\x15team2PrevOpponentLogo\
    \x123\n\x16team1_prev_opponent_id\x18\x11\x20\x01(\rR\x13team1PrevOppone\
    ntId\x123\n\x16team2_prev_opponent_id\x18\x12\x20\x01(\rR\x13team2PrevOp\
    ponentId\x123\n\x16team1_prev_match_score\x18\x13\x20\x01(\rR\x13team1Pr\
    evMatchScore\x12D\n\x1fteam1_prev_match_opponent_score\x18\x14\x20\x01(\
    \rR\x1bteam1PrevMatchOpponentScore\x123\n\x16team2_prev_match_score\x18\
    \x15\x20\x01(\rR\x13team2PrevMatchScore\x12D\n\x1fteam2_prev_match_oppon\
    ent_score\x18\x16\x20\x01(\rR\x1bteam2PrevMatchOpponentScore\x12\x1d\n\n\
    phase_type\x18\x17\x20\x01(\rR\tphaseType\x12\x1f\n\x0bteam1_score\x18\
    \x18\x20\x01(\rR\nteam1Score\x12\x1f\n\x0bteam2_score\x18\x19\x20\x01(\r\
    R\nteam2Score\x12\x19\n\x08phase_id\x18\x1a\x20\x01(\rR\x07phaseId\x1ad\
    \n\x04News\x12\x12\n\x04link\x18\x01\x20\x01(\tR\x04link\x12\x14\n\x05ti\
    tle\x18\x02\x20\x01(\tR\x05title\x12\x14\n\x05image\x18\x03\x20\x01(\tR\
    \x05image\x12\x1c\n\ttimestamp\x18\x04\x20\x01(\rR\ttimestamp\"#\n!CMsgR\
    equestWeekendTourneySchedule\"\xdb\x02\n\x1aCMsgWeekendTourneySchedule\
    \x12B\n\tdivisions\x18\x01\x20\x03(\x0b2$.CMsgWeekendTourneySchedule.Div\
    isionR\tdivisions\x1a\xf8\x01\n\x08Division\x12#\n\rdivision_code\x18\
    \x01\x20\x01(\rR\x0cdivisionCode\x12(\n\x10time_window_open\x18\x02\x20\
    \x01(\rR\x0etimeWindowOpen\x12*\n\x11time_window_close\x18\x03\x20\x01(\
    \rR\x0ftimeWindowClose\x121\n\x15time_window_open_next\x18\x04\x20\x01(\
    \rR\x12timeWindowOpenNext\x12\x1b\n\ttrophy_id\x18\x05\x20\x01(\rR\x08tr\
    ophyId\x12!\n\x0cfree_weekend\x18\x06\x20\x01(\x08R\x0bfreeWeekend\"\xa6\
    \x02\n\x16CMsgWeekendTourneyOpts\x12$\n\rparticipating\x18\x01\x20\x01(\
    \x08R\rparticipating\x12\x1f\n\x0bdivision_id\x18\x02\x20\x01(\rR\ndivis\
    ionId\x12\x14\n\x05buyin\x18\x03\x20\x01(\rR\x05buyin\x12\x1f\n\x0bskill\
    _level\x18\x04\x20\x01(\rR\nskillLevel\x12!\n\x0cmatch_groups\x18\x05\
    \x20\x01(\rR\x0bmatchGroups\x12\x17\n\x07team_id\x18\x06\x20\x01(\rR\x06\
    teamId\x12(\n\x10pickup_team_name\x18\x07\x20\x01(\tR\x0epickupTeamName\
    \x12(\n\x10pickup_team_logo\x18\x08\x20\x01(\x04R\x0epickupTeamLogo\"\
    \x19\n\x17CMsgWeekendTourneyLeave\"\xc4\n\n\x12CMsgDOTATournament\x12#\n\
    \rtournament_id\x18\x01\x20\x01(\rR\x0ctournamentId\x12\x1f\n\x0bdivisio\
    n_id\x18\x02\x20\x01(\rR\ndivisionId\x12#\n\rschedule_time\x18\x03\x20\
    \x01(\rR\x0cscheduleTime\x12\x1f\n\x0bskill_level\x18\x04\x20\x01(\rR\ns\
    killLevel\x12a\n\x13tournament_template\x18\x05\x20\x01(\x0e2\x14.ETourn\
    amentTemplate:\x1ak_ETournamentTemplate_NoneR\x12tournamentTemplate\x12C\
    \n\x05state\x18\x06\x20\x01(\x0e2\x11.ETournamentState:\x1ak_ETournament\
    State_UnknownR\x05state\x12\"\n\rstate_seq_num\x18\n\x20\x01(\rR\x0bstat\
    eSeqNum\x12(\n\x10season_trophy_id\x18\x0b\x20\x01(\rR\x0eseasonTrophyId\
    \x12.\n\x05teams\x18\x07\x20\x03(\x0b2\x18.CMsgDOTATournament.TeamR\x05t\
    eams\x12.\n\x05games\x18\x08\x20\x03(\x0b2\x18.CMsgDOTATournament.GameR\
    \x05games\x12.\n\x05nodes\x18\t\x20\x03(\x0b2\x18.CMsgDOTATournament.Nod\
    eR\x05nodes\x1a\x81\x03\n\x04Team\x12\x19\n\x08team_gid\x18\x01\x20\x01(\
    \x06R\x07teamGid\x12\"\n\rnode_or_state\x18\x02\x20\x01(\rR\x0bnodeOrSta\
    te\x12\x1c\n\x07players\x18\x03\x20\x03(\rR\x07playersB\x02\x10\x01\x12%\
    \n\x0cplayer_buyin\x18\t\x20\x03(\rR\x0bplayerBuyinB\x02\x10\x01\x120\n\
    \x12player_skill_level\x18\n\x20\x03(\rR\x10playerSkillLevelB\x02\x10\
    \x01\x12(\n\x10match_group_mask\x18\x0c\x20\x01(\rR\x0ematchGroupMask\
    \x12\x17\n\x07team_id\x18\x04\x20\x01(\rR\x06teamId\x12\x1b\n\tteam_name\
    \x18\x05\x20\x01(\tR\x08teamName\x12$\n\x0eteam_base_logo\x18\x07\x20\
    \x01(\x04R\x0cteamBaseLogo\x12\x20\n\x0cteam_ui_logo\x18\x08\x20\x01(\
    \x04R\nteamUiLogo\x12\x1b\n\tteam_date\x18\x0b\x20\x01(\rR\x08teamDate\
    \x1a\xe3\x01\n\x04Game\x12\x19\n\x08node_idx\x18\x01\x20\x01(\rR\x07node\
    Idx\x12\x19\n\x08lobby_id\x18\x02\x20\x01(\x06R\x07lobbyId\x12\x19\n\x08\
    match_id\x18\x03\x20\x01(\x04R\x07matchId\x12\x1e\n\x0bteam_a_good\x18\
    \x04\x20\x01(\x08R\tteamAGood\x12K\n\x05state\x18\x05\x20\x01(\x0e2\x15.\
    ETournamentGameState:\x1ek_ETournamentGameState_UnknownR\x05state\x12\
    \x1d\n\nstart_time\x18\x06\x20\x01(\rR\tstartTime\x1a\xb1\x01\n\x04Node\
    \x12\x17\n\x07node_id\x18\x01\x20\x01(\rR\x06nodeId\x12\x1c\n\nteam_idx_\
    a\x18\x02\x20\x01(\rR\x08teamIdxA\x12\x1c\n\nteam_idx_b\x18\x03\x20\x01(\
    \rR\x08teamIdxB\x12T\n\nnode_state\x18\x04\x20\x01(\x0e2\x15.ETournament\
    NodeState:\x1ek_ETournamentNodeState_UnknownR\tnodeState\"\xe2\x05\n\x1d\
    CMsgDOTATournamentStateChange\x12*\n\x11new_tournament_id\x18\x01\x20\
    \x01(\rR\x0fnewTournamentId\x12@\n\x05event\x18\x02\x20\x01(\x0e2\x11.ET\
    ournamentEvent:\x17k_ETournamentEvent_NoneR\x05event\x12_\n\x14new_tourn\
    ament_state\x18\x03\x20\x01(\x0e2\x11.ETournamentState:\x1ak_ETournament\
    State_UnknownR\x12newTournamentState\x12L\n\x0cgame_changes\x18\x04\x20\
    \x03(\x0b2).CMsgDOTATournamentStateChange.GameChangeR\x0bgameChanges\x12\
    L\n\x0cteam_changes\x18\x05\x20\x03(\x0b2).CMsgDOTATournamentStateChange\
    .TeamChangeR\x0bteamChanges\x126\n\x15merged_tournament_ids\x18\x06\x20\
    \x03(\rR\x13mergedTournamentIdsB\x02\x10\x01\x12\"\n\rstate_seq_num\x18\
    \x07\x20\x01(\rR\x0bstateSeqNum\x1a{\n\nGameChange\x12\x19\n\x08match_id\
    \x18\x01\x20\x01(\x04R\x07matchId\x12R\n\tnew_state\x18\x02\x20\x01(\x0e\
    2\x15.ETournamentGameState:\x1ek_ETournamentGameState_UnknownR\x08newSta\
    te\x1a}\n\nTeamChange\x12\x19\n\x08team_gid\x18\x01\x20\x01(\x04R\x07tea\
    mGid\x12)\n\x11new_node_or_state\x18\x02\x20\x01(\rR\x0enewNodeOrState\
    \x12)\n\x11old_node_or_state\x18\x03\x20\x01(\rR\x0eoldNodeOrState\"t\n\
    \x19CMsgDOTATournamentRequest\x12#\n\rtournament_id\x18\x01\x20\x01(\rR\
    \x0ctournamentId\x122\n\x15client_tournament_gid\x18\x02\x20\x01(\x04R\
    \x13clientTournamentGid\"l\n\x1aCMsgDOTATournamentResponse\x12\x19\n\x06\
    result\x18\x01\x20\x01(\r:\x012R\x06result\x123\n\ntournament\x18\x02\
    \x20\x01(\x0b2\x13.CMsgDOTATournamentR\ntournament\"[\n\x1bCMsgDOTAClear\
    TournamentGame\x12#\n\rtournament_id\x18\x01\x20\x01(\rR\x0ctournamentId\
    \x12\x17\n\x07game_id\x18\x02\x20\x01(\rR\x06gameId\"\x94\x03\n+CMsgDOTA\
    WeekendTourneyPlayerSkillLevelStats\x12\x1f\n\x0bskill_level\x18\x01\x20\
    \x01(\rR\nskillLevel\x12\x1e\n\x0btimes_won_0\x18\x02\x20\x01(\rR\ttimes\
    Won0\x12\x1e\n\x0btimes_won_1\x18\x03\x20\x01(\rR\ttimesWon1\x12\x1e\n\
    \x0btimes_won_2\x18\x04\x20\x01(\rR\ttimesWon2\x12\x1e\n\x0btimes_won_3\
    \x18\x05\x20\x01(\rR\ttimesWon3\x12+\n\x12times_bye_and_lost\x18\x06\x20\
    \x01(\rR\x0ftimesByeAndLost\x12)\n\x11times_bye_and_won\x18\x07\x20\x01(\
    \rR\x0etimesByeAndWon\x12.\n\x13times_unusual_champ\x18\n\x20\x01(\rR\
    \x11timesUnusualChamp\x12&\n\x0ftotal_games_won\x18\x08\x20\x01(\rR\rtot\
    alGamesWon\x12\x14\n\x05score\x18\t\x20\x01(\rR\x05score\"\xe0\x01\n!CMs\
    gDOTAWeekendTourneyPlayerStats\x12\x1d\n\naccount_id\x18\x01\x20\x01(\rR\
    \taccountId\x12(\n\x10season_trophy_id\x18\x02\x20\x01(\rR\x0eseasonTrop\
    hyId\x12O\n\x0cskill_levels\x18\x03\x20\x03(\x0b2,.CMsgDOTAWeekendTourne\
    yPlayerSkillLevelStatsR\x0bskillLevels\x12!\n\x0ccurrent_tier\x18\x04\
    \x20\x01(\rR\x0bcurrentTier\"s\n(CMsgDOTAWeekendTourneyPlayerStatsReques\
    t\x12\x1d\n\naccount_id\x18\x01\x20\x01(\rR\taccountId\x12(\n\x10season_\
    trophy_id\x18\x02\x20\x01(\rR\x0eseasonTrophyId\"u\n*CMsgDOTAWeekendTour\
    neyPlayerHistoryRequest\x12\x1d\n\naccount_id\x18\x01\x20\x01(\rR\taccou\
    ntId\x12(\n\x10season_trophy_id\x18\x02\x20\x01(\rR\x0eseasonTrophyId\"\
    \xd0\x03\n#CMsgDOTAWeekendTourneyPlayerHistory\x12\x1d\n\naccount_id\x18\
    \x01\x20\x01(\rR\taccountId\x12Q\n\x0btournaments\x18\x03\x20\x03(\x0b2/\
    .CMsgDOTAWeekendTourneyPlayerHistory.TournamentR\x0btournaments\x1a\xb6\
    \x02\n\nTournament\x12#\n\rtournament_id\x18\x01\x20\x01(\rR\x0ctourname\
    ntId\x12\x1d\n\nstart_time\x18\x02\x20\x01(\rR\tstartTime\x12'\n\x0ftour\
    nament_tier\x18\x03\x20\x01(\rR\x0etournamentTier\x12\x17\n\x07team_id\
    \x18\x04\x20\x01(\rR\x06teamId\x12\x1b\n\tteam_date\x18\x05\x20\x01(\rR\
    \x08teamDate\x12\x1f\n\x0bteam_result\x18\x06\x20\x01(\rR\nteamResult\
    \x12\x1d\n\naccount_id\x18\x07\x20\x03(\rR\taccountId\x12\x1b\n\tteam_na\
    me\x18\x08\x20\x01(\tR\x08teamName\x12(\n\x10season_trophy_id\x18\t\x20\
    \x01(\rR\x0eseasonTrophyId\"\xb5\x04\n*CMsgDOTAWeekendTourneyParticipati\
    onDetails\x12R\n\tdivisions\x18\x01\x20\x03(\x0b24.CMsgDOTAWeekendTourne\
    yParticipationDetails.DivisionR\tdivisions\x1a\x97\x02\n\x04Tier\x12\x12\
    \n\x04tier\x18\x01\x20\x01(\rR\x04tier\x12\x18\n\x07players\x18\x02\x20\
    \x01(\rR\x07players\x12\x14\n\x05teams\x18\x03\x20\x01(\rR\x05teams\x12#\
    \n\rwinning_teams\x18\x04\x20\x01(\rR\x0cwinningTeams\x12(\n\x10players_\
    streak_2\x18\x05\x20\x01(\rR\x0eplayersStreak2\x12(\n\x10players_streak_\
    3\x18\x06\x20\x01(\rR\x0eplayersStreak3\x12(\n\x10players_streak_4\x18\
    \x07\x20\x01(\rR\x0eplayersStreak4\x12(\n\x10players_streak_5\x18\x08\
    \x20\x01(\rR\x0eplayersStreak5\x1a\x98\x01\n\x08Division\x12\x1f\n\x0bdi\
    vision_id\x18\x01\x20\x01(\rR\ndivisionId\x12#\n\rschedule_time\x18\x02\
    \x20\x01(\rR\x0cscheduleTime\x12F\n\x05tiers\x18\x03\x20\x03(\x0b20.CMsg\
    DOTAWeekendTourneyParticipationDetails.TierR\x05tiers*\xb6\x04\n\x10ETou\
    rnamentEvent\x12\x1b\n\x17k_ETournamentEvent_None\x10\0\x12(\n$k_ETourna\
    mentEvent_TournamentCreated\x10\x01\x12(\n$k_ETournamentEvent_Tournament\
    sMerged\x10\x02\x12\"\n\x1ek_ETournamentEvent_GameOutcome\x10\x03\x12#\n\
    \x1fk_ETournamentEvent_TeamGivenBye\x10\x04\x120\n,k_ETournamentEvent_To\
    urnamentCanceledByAdmin\x10\x05\x12$\n\x20k_ETournamentEvent_TeamAbandon\
    ed\x10\x06\x12+\n'k_ETournamentEvent_ScheduledGameStarted\x10\x07\x12\
    \x1f\n\x1bk_ETournamentEvent_Canceled\x10\x08\x12?\n;k_ETournamentEvent_\
    TeamParticipationTimedOut_EntryFeeRefund\x10\t\x12@\n<k_ETournamentEvent\
    _TeamParticipationTimedOut_EntryFeeForfeit\x10\n\x12?\n;k_ETournamentEve\
    nt_TeamParticipationTimedOut_GrantedVictory\x10\x0bB\x05H\x01\x80\x01\0\
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
