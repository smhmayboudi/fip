fn main() {
    tonic_build::configure()
        .build_client(false)
        // .proto_path("crate::api::proto")
        // .extern_path(".fip.api", "client")
        .compile(&["api_api.proto"], &["../fip_api/proto"])
        .unwrap();

    tonic_build::configure()
        .build_client(false)
        // .proto_path("crate::auth::proto")
        // .extern_path(".fip.auth", "client")
        .compile(&["api_auth.proto"], &["../fip_api/proto"])
        .unwrap();

    tonic_build::configure()
        .build_client(false)
        .proto_path("crate::at::proto")
        .extern_path(".fip.at", "client")
        .compile(&["api_at.proto"], &["proto", "../fip_at/proto"])
        .unwrap();
    tonic_build::configure()
        .build_client(false)
        .proto_path("crate::jwks::proto")
        .extern_path(".fip.jwks", "client")
        .compile(&["api_jwks.proto"], &["proto", "../fip_jwks/proto"])
        .unwrap();
    tonic_build::configure()
        .build_client(false)
        .proto_path("crate::rt::proto")
        .extern_path(".fip.rt", "client")
        .compile(&["api_rt.proto"], &["proto", "../fip_rt/proto"])
        .unwrap();
    tonic_build::configure()
        .build_client(false)
        .proto_path("crate::user::proto")
        .extern_path(".fip.user", "client")
        .compile(&["api_user.proto"], &["proto", "../fip_user/proto"])
        .unwrap();

    tonic_build::configure()
        .build_server(false)
        .compile(&["at.proto"], &["../fip_at/proto"])
        .unwrap();
    tonic_build::configure()
        .build_server(false)
        .compile(&["jwks.proto"], &["../fip_jwks/proto"])
        .unwrap();
    tonic_build::configure()
        .build_server(false)
        .compile(&["rt.proto"], &["../fip_rt/proto"])
        .unwrap();
    tonic_build::configure()
        .build_server(false)
        .compile(&["user.proto"], &["../fip_user/proto"])
        .unwrap();
}
