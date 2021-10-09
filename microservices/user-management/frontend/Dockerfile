FROM node:16-alpine as builder
WORKDIR /svelte/src/user-management
COPY src/ ./src/
COPY static/ ./static/
COPY package.json ./
COPY .babelrc ./
COPY svelte.config.js ./
COPY tsconfig.json ./
RUN yarn install
RUN yarn build

# FROM gcr.io/distroless/nodejs:16
# COPY --from=builder /svelte/src/user-management/target/ ./
FROM node:16-alpine
WORKDIR /svelte/user-management
COPY --from=builder /svelte/src/user-management/target/ ./
RUN addgroup -S nonrootgroup && adduser -S nonrootuser -G nonrootgroup
USER nonrootuser
CMD ["sh"]
# CMD ["node", "index.js"]
