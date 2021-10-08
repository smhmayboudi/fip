#!/bin/bash

cd ../fip_at
rm -fr at.db*
sqlx database create --database-url sqlite:at.db
sqlx migrate run --database-url sqlite:at.db
cd ../fip_jwks
rm -fr jwks.db*
sqlx database create --database-url sqlite:jwks.db
sqlx migrate run  --database-url sqlite:jwks.db
cd ../fip_rt
rm -fr rt.db*
sqlx database create --database-url sqlite:rt.db
sqlx migrate run --database-url sqlite:rt.db
cd ../fip_user
rm -fr user.db*
sqlx database create --database-url sqlite:user.db
sqlx migrate run --database-url sqlite:user.db
cd ..