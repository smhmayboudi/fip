fn main() {
    tonic_build::configure()
        .build_client(false)
        .compile(&["rt.proto"], &["../fip_rt/proto"])
        .unwrap();
}
