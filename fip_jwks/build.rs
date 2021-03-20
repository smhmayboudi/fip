fn main() {
    tonic_build::configure()
        .build_client(false)
        .compile(&["jwks.proto"], &["../fip_jwks/proto"])
        .unwrap();
}
