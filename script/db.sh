#!/bin/bash

cd ../fip_at
rm -fr at.db*
sqlx --database-url sqlite:at.db db create
sqlx --database-url sqlite:at.db migrate run
cd ../fip_jwks
rm -fr jwks.db*
sqlx --database-url sqlite:jwks.db db create
sqlx --database-url sqlite:jwks.db migrate run
cd ../fip_rt
rm -fr rt.db*
sqlx --database-url sqlite:rt.db db create
sqlx --database-url sqlite:rt.db migrate run
cd ../fip_user
rm -fr user.db*
sqlx --database-url sqlite:user.db db create
sqlx --database-url sqlite:user.db migrate run
cd ..