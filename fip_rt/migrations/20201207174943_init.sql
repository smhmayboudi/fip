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
        2637358277,
        1633758277,
        "fip_rt",
        "4D47BC37-F744-4026-ADE2-BE17EEB5660B",
        1633758277,
        "E5D86E1D-31A4-4964-9881-F160FC5B1073",
        false,
        ""
    )
