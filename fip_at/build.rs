//! TODO: documentation

fn main() {
    tonic_build::configure()
        .build_client(false)
        .client_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .client_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        // .extern_path(".fip.at", "server")
        // .field_attribute(".", "")
        // .proto_path("crate::api::proto")
        .server_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .server_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        .type_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .compile(&["at.proto"], &["../fip_at/proto"])
        .unwrap();
}
