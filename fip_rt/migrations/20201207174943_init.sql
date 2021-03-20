-- Add migration script here
INSERT INTO rt (
        claims_aud,
        claims_exp,
        claims_iat,
        claims_iss,
        claims_jti,
        claims_nbf,
        claims_sub,
        token_blocked,
        token_blocked_description
    )
VALUES (
        "fip_api",
        2607285997,
        1607285997,
        "fip_rt",
        "E9E752B9-C342-4D3D-BDCC-13FA8E79FA0B",
        1607285997,
        "E5D86E1D-31A4-4964-9881-F160FC5B1073",
        false,
        ""
    )