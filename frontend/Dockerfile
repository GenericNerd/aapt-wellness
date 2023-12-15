FROM node:lts-alpine AS build
COPY . .
RUN npm ci
RUN npm run build
FROM node:lts-alpine AS run
ARG NODE_ENV=production
ENV NODE_ENV $NODE_ENV
COPY --from=build ./build ./build
CMD ["node", "build"]