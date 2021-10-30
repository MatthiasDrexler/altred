#!/bin/sh

DB_VENDOR=$(echo $DATABASE_URL | grep --perl-regexp --only-matching '^[\w]*(?=://)')
export DB_VENDOR=$DB_VENDOR
# echo $DB_VENDOR

DB_USER=$(echo $DATABASE_URL | grep --perl-regexp --only-matching '(?<=://)[\w]+(?=:)')
export DB_USER=$DB_USER
# echo $DB_USER

DB_PASSWORD=$(echo $DATABASE_URL | grep --perl-regexp --only-matching '(?<=:)[\w]*(?=@)')
export DB_PASSWORD=$DB_PASSWORD
# echo $DB_PASSWORD

DB_ADDR=$(echo $DATABASE_URL | grep --perl-regexp --only-matching '(?<=@)[\S]*(?=:)')
export DB_ADDR=$DB_ADDR
# echo $DB_ADDR

DB_PORT=$(echo $DATABASE_URL | grep --perl-regexp --only-matching '(?<=:)[\d^:]*(?=/)')
export DB_PORT=$DB_PORT
# echo $DB_PORT

DB_DATABASE=$(echo $DATABASE_URL | grep --perl-regexp --only-matching '(?<=/)[^/]*$')
export DB_DATABASE=$DB_DATABASE
# echo $DB_DATABASE

HTTP_PORT=$PORT
HTTPS_PORT=$(expr $HTTP_PORT + 1)
MGMT_HTTP_PORT=$(expr $HTTP_PORT + 2)
MGMT_HTTPS_PORT=$(expr $HTTP_PORT + 3)
sed --in-place "s|jboss.http.port:8080|jboss.http.port:${HTTP_PORT}|g" /opt/jboss/keycloak/standalone/configuration/standalone-ha.xml
sed --in-place "s|jboss.https.port:8080|jboss.https.port:${HTTPS_PORT}|g" /opt/jboss/keycloak/standalone/configuration/standalone-ha.xml
sed --in-place "s|jboss.management.http.port:9990|jboss.management.http.port:${HTTPS_PORT}|g" /opt/jboss/keycloak/standalone/configuration/standalone-ha.xml
sed --in-place "s|jboss.management.https.port:9990|jboss.management.http.port:${HTTPS_PORT}|g" /opt/jboss/keycloak/standalone/configuration/standalone-ha.xml

sh /opt/jboss/tools/docker-entrypoint.sh -b 0.0.0.0
