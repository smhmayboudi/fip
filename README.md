# fip

[![Build Status](__badge_image__)](__badge_url__)

## Help


```shell
$ cargo watch -x "run | bunyan"
$ catflap -- cargo watch -x "run | bunyan"

$ grpcurl \
    -d '{"username":"smhmayboudi","password":"smhmayboudi"}' \
    -import-path fip_at/proto \
    -import-path fip_api/proto \
    -import-path fip_jwks/proto \
    -import-path fip_rt/proto \
    -import-path fip_user/proto \
    -plaintext \
    -proto api_auth.proto \
    127.0.0.1:50050 fip.api.auth.Auth/Login

$ grpcurl \
    -d '{"token":"E9E752B9-C342-4D3D-BDCC-13FA8E79FA0B"}' \
    -import-path fip_at/proto \
    -import-path fip_api/proto \
    -import-path fip_jwks/proto \
    -import-path fip_rt/proto \
    -import-path fip_user/proto \
    -plaintext \
    -proto api_auth.proto \
    127.0.0.1:50050 fip.api.auth.Auth/Token

$ grpcurl \
    -H 'authorization: JWT eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IkFEODZENEQzLUEzNkMtNDkyNC04NDJDLTA0RjhCNzRBMjBDQiJ9.eyJhdWQiOiJmaXBfYXBpIiwiZXhwIjoxNjI0MzMyMDY0LCJpYXQiOjE2MjA3MzIwNjQsImlzcyI6ImZpcF9hcGkiLCJqdGkiOiI0RDI5OTBGRC04OEY2LTQ1QTEtQTkwRS1ENzU5OENCQkRCMzQiLCJuYmYiOjE2MjA3MzIwNjQsInN1YiI6IkU1RDg2RTFELTMxQTQtNDk2NC05ODgxLUYxNjBGQzVCMTA3MyJ9.Rrrt0mqL827Olh9aaLPL4mp3RRv_CGsCdVqN6kbrNLvbczi88l72zIXvk832U6Nyu0xY0sOlkxKeu9gA4PhUcX8x7wfZWbTLY5YJHa6LWl067gVdItW7toU6YqRyCe9S33yNMD5x2UNkdYDI1QA-l0M2SuuQQnP1I_At27Z2NZsX9sQVt9K59xtBOM00TmVjleWjx5Dw357pTulbWGSafDPOWpY1ahW6UWnY0F4Y4Ah20pWCD04Ind4ioXobONt5w0YLAw56dkbIJFrTKqR4JbfcvySmTDxlWrKbYIF3UcW8FZl1017hlFgUHRtg0JOy3yputuR9NVl3D_qerv0aQfSH6qiJ8d2yiPDrbEuNWp3pa2cPXCAZ5BSXysxIvyYSC6i8bh-dzGUesB3dDNc2EWB76AUo8208Jdq1BoO7g9321ReDJvuymp031wMFfBdxPgRRQbsuodbVaWtVH9JjAL3YFqAKq-6AT-aux0nYSqob4AzbbEy1mdyl0jm18ahmOKWzNT_yfD6IaEV3dlT6y9w-HSxenK6ETlVsaLTwS-qd0jZAm9tiiFwqbgVk7EIIN4ut1m-KeP6T3e38JQdtzaec0g4nLUQ0wAfLOlA2F4mNukvEL9TMOdpDd0RFQWTmHVuXcPOD0ykBunurL2T2jnDWXNEsF9MnSjQl0sLZies' \
    -d '{"id":"E5D86E1D-31A4-4964-9881-F160FC5B1073"}' \
    -import-path fip_api/proto \
    -import-path fip_at/proto \
    -import-path fip_jwks/proto \
    -import-path fip_rt/proto \
    -import-path fip_user/proto \
    -plaintext \
    -proto api_user.proto \
    127.0.0.1:50050 fip.api.user.User/FindOne

$ grpcurl \
    -H 'authorization: JWT eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IkFEODZENEQzLUEzNkMtNDkyNC04NDJDLTA0RjhCNzRBMjBDQiJ9.eyJhdWQiOiJmaXBfYXBpIiwiZXhwIjoxNjI0MzMyMDY0LCJpYXQiOjE2MjA3MzIwNjQsImlzcyI6ImZpcF9hcGkiLCJqdGkiOiI0RDI5OTBGRC04OEY2LTQ1QTEtQTkwRS1ENzU5OENCQkRCMzQiLCJuYmYiOjE2MjA3MzIwNjQsInN1YiI6IkU1RDg2RTFELTMxQTQtNDk2NC05ODgxLUYxNjBGQzVCMTA3MyJ9.Rrrt0mqL827Olh9aaLPL4mp3RRv_CGsCdVqN6kbrNLvbczi88l72zIXvk832U6Nyu0xY0sOlkxKeu9gA4PhUcX8x7wfZWbTLY5YJHa6LWl067gVdItW7toU6YqRyCe9S33yNMD5x2UNkdYDI1QA-l0M2SuuQQnP1I_At27Z2NZsX9sQVt9K59xtBOM00TmVjleWjx5Dw357pTulbWGSafDPOWpY1ahW6UWnY0F4Y4Ah20pWCD04Ind4ioXobONt5w0YLAw56dkbIJFrTKqR4JbfcvySmTDxlWrKbYIF3UcW8FZl1017hlFgUHRtg0JOy3yputuR9NVl3D_qerv0aQfSH6qiJ8d2yiPDrbEuNWp3pa2cPXCAZ5BSXysxIvyYSC6i8bh-dzGUesB3dDNc2EWB76AUo8208Jdq1BoO7g9321ReDJvuymp031wMFfBdxPgRRQbsuodbVaWtVH9JjAL3YFqAKq-6AT-aux0nYSqob4AzbbEy1mdyl0jm18ahmOKWzNT_yfD6IaEV3dlT6y9w-HSxenK6ETlVsaLTwS-qd0jZAm9tiiFwqbgVk7EIIN4ut1m-KeP6T3e38JQdtzaec0g4nLUQ0wAfLOlA2F4mNukvEL9TMOdpDd0RFQWTmHVuXcPOD0ykBunurL2T2jnDWXNEsF9MnSjQl0sLZies' \
    -d '{"id":"E5D86E1D-31A4-4964-9881-F160FC5B1073"}' \
    -import-path fip_api/proto \
    -import-path fip_at/proto \
    -import-path fip_jwks/proto \
    -import-path fip_rt/proto \
    -import-path fip_user/proto \
    -plaintext \
    -proto api_api.proto \
    127.0.0.1:50050 fip.api.api.Api/FindOne
```
