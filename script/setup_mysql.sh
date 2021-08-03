#!/bin/sh

docker run --name courier-mysql\
	-v /var/db/courier-mysql:/var/lib/mysql\
	-p 3306:3306\
	-e MYSQL_ROOT_PASSWORD=my_root_pw\
	-e MYSQL_DATABASE=kvdb\
	-e MYSQL_USER=kvuser\
	-e MYSQL_PASSWORD=kvpassword\
	-d mysql

