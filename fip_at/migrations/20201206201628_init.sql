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
        2637358277,
        1633758277,
        "fip_at",
        "4D81CFA3-EE4C-47BA-A920-42312CE26008",
        1633758277,
        "E5D86E1D-31A4-4964-9881-F160FC5B1073",
        "JWT",
        "RS256",
        "",
        "",
        "AD86D4D3-A36C-4924-842C-04F8B74A20CB",
        "",
        "",
        false,
        ""
    )
