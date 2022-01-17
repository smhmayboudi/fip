# fip

[![Build Status](__badge_image__)](__badge_url__)

## Help


```shell
$ cargo watch -c --exec "run | bunyan"

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
    -d '{"token":"4D47BC37-F744-4026-ADE2-BE17EEB5660B"}' \
    -import-path fip_at/proto \
    -import-path fip_api/proto \
    -import-path fip_jwks/proto \
    -import-path fip_rt/proto \
    -import-path fip_user/proto \
    -plaintext \
    -proto api_auth.proto \
    127.0.0.1:50050 fip.api.auth.Auth/Token

$ grpcurl \
    -H 'authorization: JWT eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IkFEODZENEQzLUEzNkMtNDkyNC04NDJDLTA0RjhCNzRBMjBDQiJ9.eyJhdWQiOiJmaXBfYXBpIiwiZXhwIjoyNjM3MzU4Mjc3LCJpYXQiOjE2MzM3NTgyNzcsImlzcyI6ImZpcF9hcGkiLCJqdGkiOiI0RDgxQ0ZBMy1FRTRDLTQ3QkEtQTkyMC00MjMxMkNFMjYwMDgiLCJuYmYiOjE2MzM3NTgyNzcsInN1YiI6IkU1RDg2RTFELTMxQTQtNDk2NC05ODgxLUYxNjBGQzVCMTA3MyJ9.tnxJLIzENWokyk53yYJRr0v1KZsqflMkW-I3HOEDq8oBhdZIcpS1NzNV9CAAb4otEdZcaFWU4FW4py66ALV2evcDdn5cF2Cg3dwwVp-8KKp1GFADusKvtJ95FhVliWWv-IeH0f05nBNAE5QLv_8CvbXcTU0FkHSAmBtwByRi0PXwtoK3OJ-mgZ9Nzzcapv90nya7D1CR2hv8Mbx7DAFlucD12CSeTg0qnCOctQ6u7gc8gkFFD7YiSMwNtjBHazajBKnth0ZlOgOtYTpiEuQKhRJH_0BT_g8lzY8KlZuCp3MLpUc8tKoSM9zcBOftV-gG6erRghJ--egpNZJdziRdyGLVX1zaZUphg6d1IIhLVZcAVoNf46N1ENm3PLIQM_dKpzGrIRAas8z8HqDsLA0rr871N933z_8waKil8FfLIhbp4kxBhDGU1GHrxlitNauLPWePVx9kNuQX8puhUfHgVGxTlAdmq9DeL8elZ9y2_afMNw5QU0KhCR92xRNZwcGgK-Snly6toTaVY6MnORtslN7uXlEJD921rdop8_ZPJy5JmnaUj4etS8rZzoyb1YFfTQGhrnEeDaTnDnab2TesoiU4bjWyz761HGWiV6ebebt-FBEeBd0x62ssvW-e-_ghq7k8mMBWUm9QobCpL6gurJOHgGm-6-8SKC-1OJpHMMQ' \
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
    -H 'authorization: JWT eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IkFEODZENEQzLUEzNkMtNDkyNC04NDJDLTA0RjhCNzRBMjBDQiJ9.eyJhdWQiOiJmaXBfYXBpIiwiZXhwIjoyNjM3MzU4Mjc3LCJpYXQiOjE2MzM3NTgyNzcsImlzcyI6ImZpcF9hcGkiLCJqdGkiOiI0RDgxQ0ZBMy1FRTRDLTQ3QkEtQTkyMC00MjMxMkNFMjYwMDgiLCJuYmYiOjE2MzM3NTgyNzcsInN1YiI6IkU1RDg2RTFELTMxQTQtNDk2NC05ODgxLUYxNjBGQzVCMTA3MyJ9.tnxJLIzENWokyk53yYJRr0v1KZsqflMkW-I3HOEDq8oBhdZIcpS1NzNV9CAAb4otEdZcaFWU4FW4py66ALV2evcDdn5cF2Cg3dwwVp-8KKp1GFADusKvtJ95FhVliWWv-IeH0f05nBNAE5QLv_8CvbXcTU0FkHSAmBtwByRi0PXwtoK3OJ-mgZ9Nzzcapv90nya7D1CR2hv8Mbx7DAFlucD12CSeTg0qnCOctQ6u7gc8gkFFD7YiSMwNtjBHazajBKnth0ZlOgOtYTpiEuQKhRJH_0BT_g8lzY8KlZuCp3MLpUc8tKoSM9zcBOftV-gG6erRghJ--egpNZJdziRdyGLVX1zaZUphg6d1IIhLVZcAVoNf46N1ENm3PLIQM_dKpzGrIRAas8z8HqDsLA0rr871N933z_8waKil8FfLIhbp4kxBhDGU1GHrxlitNauLPWePVx9kNuQX8puhUfHgVGxTlAdmq9DeL8elZ9y2_afMNw5QU0KhCR92xRNZwcGgK-Snly6toTaVY6MnORtslN7uXlEJD921rdop8_ZPJy5JmnaUj4etS8rZzoyb1YFfTQGhrnEeDaTnDnab2TesoiU4bjWyz761HGWiV6ebebt-FBEeBd0x62ssvW-e-_ghq7k8mMBWUm9QobCpL6gurJOHgGm-6-8SKC-1OJpHMMQ' \
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
