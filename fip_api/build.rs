//! TODO: documentation

fn build_client() {
    tonic_build::configure()
        .build_server(false)
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
                .unwrap_or_else(|err| {
            panic!("{:?}", err);
        });

    tonic_build::configure()
        .build_server(false)
        .client_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .client_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        // .extern_path(".fip.jwks", "server")
        // .field_attribute(".", "")
        // .proto_path("crate::api::proto")
        .server_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .server_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        .type_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .compile(&["jwks.proto"], &["../fip_jwks/proto"])
                .unwrap_or_else(|err| {
            panic!("{:?}", err);
        });

    tonic_build::configure()
        .build_server(false)
        .client_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .client_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        // .extern_path(".fip.rt", "server")
        // .field_attribute(".", "")
        // .proto_path("crate::api::proto")
        .server_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .server_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        .type_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .compile(&["rt.proto"], &["../fip_rt/proto"])
                .unwrap_or_else(|err| {
            panic!("{:?}", err);
        });

    tonic_build::configure()
        .build_server(false)
        .client_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .client_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        // .extern_path(".fip.user", "server")
        // .field_attribute(".", "")
        // .proto_path("crate::api::proto")
        .server_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .server_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        .type_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .compile(&["user.proto"], &["../fip_user/proto"])
                .unwrap_or_else(|err| {
            panic!("{:?}", err);
        });
}

fn build_server() {
    tonic_build::configure()
        .build_client(false)
        .client_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .client_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        // .extern_path(".fip.api", "client")
        // .field_attribute(".", "")
        // .proto_path("crate::api::proto")
        .server_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .server_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        .type_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .compile(&["api_api.proto"], &["../fip_api/proto"])
                .unwrap_or_else(|err| {
            panic!("{:?}", err);
        });

    tonic_build::configure()
        .build_client(false)
        .client_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .client_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        // .extern_path(".fip.auth", "client")
        // .field_attribute(".", "")
        // .proto_path("crate::auth::proto")
        .server_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .server_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        .type_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .compile(&["api_auth.proto"], &["../fip_api/proto"])
                .unwrap_or_else(|err| {
            panic!("{:?}", err);
        });
}

fn build_server_2() {
    tonic_build::configure()
        .build_client(false)
        .client_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .client_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        .extern_path(".fip.at", "client")
        // .field_attribute(".", "")
        .proto_path("crate::at::proto")
        .server_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .server_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        .type_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .compile(&["api_at.proto"], &["proto", "../fip_at/proto"])
                .unwrap_or_else(|err| {
            panic!("{:?}", err);
        });

    tonic_build::configure()
        .build_client(false)
        .client_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .client_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        .extern_path(".fip.jwks", "client")
        // .field_attribute(".", "")
        .proto_path("crate::jwks::proto")
        .server_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .server_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        .type_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .compile(&["api_jwks.proto"], &["proto", "../fip_jwks/proto"])
                .unwrap_or_else(|err| {
            panic!("{:?}", err);
        });

    tonic_build::configure()
        .build_client(false)
        .client_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .client_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        .extern_path(".fip.rt", "client")
        // .field_attribute(".", "")
        .proto_path("crate::rt::proto")
        .server_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .server_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        .type_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .compile(&["api_rt.proto"], &["proto", "../fip_rt/proto"])
        .unwrap_or_else(|err| {
            panic!("{:?}", err);
        });

    tonic_build::configure()
        .build_client(false)
        .client_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .client_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        .extern_path(".fip.user", "client")
        // .field_attribute(".", "")
        .proto_path("crate::user::proto")
        .server_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .server_mod_attribute(
            ".",
            "#[allow(clippy::future_not_send, clippy::missing_errors_doc, clippy::module_name_repetitions, clippy::similar_names, clippy::too_many_lines, clippy::wildcard_imports)]",
        )
        .type_attribute(".", "#[allow(missing_copy_implementations, missing_docs)]")
        .compile(&["api_user.proto"], &["proto", "../fip_user/proto"])
                .unwrap_or_else(|err| {
            panic!("{:?}", err);
        });
}

fn main() {
    build_server();
    build_server_2();
    build_client();
}
