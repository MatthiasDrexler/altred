FROM node:16-alpine as builder
WORKDIR /svelte/src/microservices/user-management/frontend
COPY user-management/frontend/ ./
RUN yarn install && yarn build

FROM node:16-alpine as packager
WORKDIR /svelte/src/microservices/user-management/frontend
COPY --from=builder /svelte/src/microservices/user-management/frontend/package.json ./
COPY --from=builder /svelte/src/microservices/user-management/frontend/target/ ./target/
RUN yarn install --production

FROM node:16-alpine as runner
WORKDIR /svelte/user-management
COPY --from=packager /svelte/src/microservices/user-management/frontend/package.json ./
COPY --from=packager /svelte/src/microservices/user-management/frontend/node_modules/ ./node_modules/
COPY --from=packager /svelte/src/microservices/user-management/frontend/target/ ./target/
RUN yarn install --production
RUN addgroup --system nonrootgroup && adduser --system nonrootuser --ingroup nonrootgroup
USER nonrootuser
CMD ["node", "target/index.js"]
