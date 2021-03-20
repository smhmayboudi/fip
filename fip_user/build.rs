fn main() {
    tonic_build::configure()
        .build_client(false)
        .compile(&["user.proto"], &["../fip_user/proto"])
        .unwrap();
}
