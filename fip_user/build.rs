//! TODO: documentation

fn main() {
    tonic_build::configure()
        .build_client(false)
        .client_attribute(".", "#[doc = r\" TODO: documentation\"]")
        // .client_mod_attribute(".", "")
        // .extern_path(".fip.user", "server")
        .field_attribute(".", "#[doc = r\" TODO: documentation\"]")
        // .proto_path("crate::api::proto")
        .server_attribute(".", "#[doc = r\" TODO: documentation\"]")
        // .server_mod_attribute(".", "")
        .type_attribute(".", "#[doc = r\" TODO: documentation\"]")
        .compile(&["user.proto"], &["../fip_user/proto"])
        .unwrap();
}
