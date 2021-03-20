-- Add migration script here
INSERT INTO at (
        claims_aud,
        claims_exp,
        claims_iat,
        claims_iss,
        claims_jti,
        claims_nbf,
        claims_sub,
        header_typ,
        header_alg,
        header_cty,
        header_jku,
        header_kid,
        header_x5u,
        header_x5t,
        token_blocked,
        token_blocked_description
    )
VALUES (
        "fip_api",
        2607285997,
        1607285997,
        "fip_at",
        "78B78FF8-0A28-4BAC-A68D-C87CBB97ACFF",
        1607285997,
        "E5D86E1D-31A4-4964-9881-F160FC5B1073",
        "JWT",
        "RS256",
        "",
        "",
        "",
        "",
        "",
        false,
        ""
    )