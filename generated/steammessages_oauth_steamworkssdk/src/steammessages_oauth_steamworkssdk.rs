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
pub struct COAuthToken_ImplicitGrantNoPrompt_Request {
    // message fields
    clientid: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for COAuthToken_ImplicitGrantNoPrompt_Request {}

impl COAuthToken_ImplicitGrantNoPrompt_Request {
    pub fn new() -> COAuthToken_ImplicitGrantNoPrompt_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static COAuthToken_ImplicitGrantNoPrompt_Request {
        static mut instance: ::protobuf::lazy::Lazy<COAuthToken_ImplicitGrantNoPrompt_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const COAuthToken_ImplicitGrantNoPrompt_Request,
        };
        unsafe {
            instance.get(COAuthToken_ImplicitGrantNoPrompt_Request::new)
        }
    }

    // optional string clientid = 1;

    pub fn clear_clientid(&mut self) {
        self.clientid.clear();
    }

    pub fn has_clientid(&self) -> bool {
        self.clientid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientid(&mut self, v: ::std::string::String) {
        self.clientid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientid(&mut self) -> &mut ::std::string::String {
        if self.clientid.is_none() {
            self.clientid.set_default();
        }
        self.clientid.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientid(&mut self) -> ::std::string::String {
        self.clientid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_clientid(&self) -> &str {
        match self.clientid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_clientid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.clientid
    }

    fn mut_clientid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.clientid
    }
}

impl ::protobuf::Message for COAuthToken_ImplicitGrantNoPrompt_Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.clientid)?;
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
        if let Some(ref v) = self.clientid.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.clientid.as_ref() {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for COAuthToken_ImplicitGrantNoPrompt_Request {
    fn new() -> COAuthToken_ImplicitGrantNoPrompt_Request {
        COAuthToken_ImplicitGrantNoPrompt_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<COAuthToken_ImplicitGrantNoPrompt_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clientid",
                    COAuthToken_ImplicitGrantNoPrompt_Request::get_clientid_for_reflect,
                    COAuthToken_ImplicitGrantNoPrompt_Request::mut_clientid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<COAuthToken_ImplicitGrantNoPrompt_Request>(
                    "COAuthToken_ImplicitGrantNoPrompt_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for COAuthToken_ImplicitGrantNoPrompt_Request {
    fn clear(&mut self) {
        self.clear_clientid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for COAuthToken_ImplicitGrantNoPrompt_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for COAuthToken_ImplicitGrantNoPrompt_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct COAuthToken_ImplicitGrantNoPrompt_Response {
    // message fields
    access_token: ::protobuf::SingularField<::std::string::String>,
    redirect_uri: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for COAuthToken_ImplicitGrantNoPrompt_Response {}

impl COAuthToken_ImplicitGrantNoPrompt_Response {
    pub fn new() -> COAuthToken_ImplicitGrantNoPrompt_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static COAuthToken_ImplicitGrantNoPrompt_Response {
        static mut instance: ::protobuf::lazy::Lazy<COAuthToken_ImplicitGrantNoPrompt_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const COAuthToken_ImplicitGrantNoPrompt_Response,
        };
        unsafe {
            instance.get(COAuthToken_ImplicitGrantNoPrompt_Response::new)
        }
    }

    // optional string access_token = 1;

    pub fn clear_access_token(&mut self) {
        self.access_token.clear();
    }

    pub fn has_access_token(&self) -> bool {
        self.access_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_access_token(&mut self, v: ::std::string::String) {
        self.access_token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_access_token(&mut self) -> &mut ::std::string::String {
        if self.access_token.is_none() {
            self.access_token.set_default();
        }
        self.access_token.as_mut().unwrap()
    }

    // Take field
    pub fn take_access_token(&mut self) -> ::std::string::String {
        self.access_token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_access_token(&self) -> &str {
        match self.access_token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_access_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.access_token
    }

    fn mut_access_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.access_token
    }

    // optional string redirect_uri = 2;

    pub fn clear_redirect_uri(&mut self) {
        self.redirect_uri.clear();
    }

    pub fn has_redirect_uri(&self) -> bool {
        self.redirect_uri.is_some()
    }

    // Param is passed by value, moved
    pub fn set_redirect_uri(&mut self, v: ::std::string::String) {
        self.redirect_uri = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_redirect_uri(&mut self) -> &mut ::std::string::String {
        if self.redirect_uri.is_none() {
            self.redirect_uri.set_default();
        }
        self.redirect_uri.as_mut().unwrap()
    }

    // Take field
    pub fn take_redirect_uri(&mut self) -> ::std::string::String {
        self.redirect_uri.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_redirect_uri(&self) -> &str {
        match self.redirect_uri.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_redirect_uri_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.redirect_uri
    }

    fn mut_redirect_uri_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.redirect_uri
    }
}

impl ::protobuf::Message for COAuthToken_ImplicitGrantNoPrompt_Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.access_token)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.redirect_uri)?;
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
        if let Some(ref v) = self.access_token.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.redirect_uri.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.access_token.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.redirect_uri.as_ref() {
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

impl ::protobuf::MessageStatic for COAuthToken_ImplicitGrantNoPrompt_Response {
    fn new() -> COAuthToken_ImplicitGrantNoPrompt_Response {
        COAuthToken_ImplicitGrantNoPrompt_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<COAuthToken_ImplicitGrantNoPrompt_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "access_token",
                    COAuthToken_ImplicitGrantNoPrompt_Response::get_access_token_for_reflect,
                    COAuthToken_ImplicitGrantNoPrompt_Response::mut_access_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "redirect_uri",
                    COAuthToken_ImplicitGrantNoPrompt_Response::get_redirect_uri_for_reflect,
                    COAuthToken_ImplicitGrantNoPrompt_Response::mut_redirect_uri_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<COAuthToken_ImplicitGrantNoPrompt_Response>(
                    "COAuthToken_ImplicitGrantNoPrompt_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for COAuthToken_ImplicitGrantNoPrompt_Response {
    fn clear(&mut self) {
        self.clear_access_token();
        self.clear_redirect_uri();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for COAuthToken_ImplicitGrantNoPrompt_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for COAuthToken_ImplicitGrantNoPrompt_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'steammessages_oauth.steamworkssdk.proto\x1a.steammessages_unified_bas\
    e.steamworkssdk.proto\"\x85\x01\n)COAuthToken_ImplicitGrantNoPrompt_Requ\
    est\x12X\n\x08clientid\x18\x01\x20\x01(\tR\x08clientidB<\x82\xb5\x188Cli\
    ent\x20ID\x20for\x20which\x20to\x20count\x20the\x20number\x20of\x20issue\
    d\x20tokens\"\xd1\x01\n*COAuthToken_ImplicitGrantNoPrompt_Response\x12F\
    \n\x0caccess_token\x18\x01\x20\x01(\tR\x0baccessTokenB#\x82\xb5\x18\x1fO\
    Auth\x20Token,\x20granted\x20on\x20success\x12[\n\x0credirect_uri\x18\
    \x02\x20\x01(\tR\x0bredirectUriB8\x82\xb5\x184Redirection\x20URI\x20prov\
    ided\x20during\x20client\x20registration.2\xb1\x02\n\nOAuthToken\x12\xeb\
    \x01\n\x15ImplicitGrantNoPrompt\x12*.COAuthToken_ImplicitGrantNoPrompt_R\
    equest\x1a+.COAuthToken_ImplicitGrantNoPrompt_Response\"y\x82\xb5\x18uGr\
    ants\x20an\x20implicit\x20OAuth\x20token\x20(grant\x20type\x20'token')\
    \x20for\x20the\x20specified\x20client\x20ID\x20on\x20behalf\x20of\x20a\
    \x20user\x20without\x20prompting\x1a5\x82\xb5\x181Service\x20containing\
    \x20methods\x20to\x20manage\x20OAuth\x20tokens\
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
