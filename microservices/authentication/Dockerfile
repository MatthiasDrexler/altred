FROM jboss/keycloak:15.0.2

COPY authentication/altred-main-realm.json /opt/jboss/keycloak/imports/altred-main-realm.json
COPY authentication/run-with-decoded-database-url.sh ./
ENTRYPOINT ["sh", "run-with-decoded-database-url.sh"]
