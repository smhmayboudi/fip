# TODO

BENCHMARK
$ cargo install critcmp
$ cargo install criterion

CHANGELOG
$ cargo install conventional_commits_linter
$ cargo install conventional_commits_next_version

CI/CD
DOCKER
TEST

AT ->
    Service
        -> TokenBlock
        -> TokenUnblock

RT ->
    Service
        -> TokenBlock
        -> TokenUnblock

Log ->
    Health
        -> Fix implementation
    Tracing
        -> Elastic APM
        -> opentelemetry_prometheus

Transport ->
    Kafka
