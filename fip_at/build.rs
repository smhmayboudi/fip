fn main() {
    tonic_build::configure()
        .build_client(false)
        .compile(&["at.proto"], &["../fip_at/proto"])
        .unwrap();
}
