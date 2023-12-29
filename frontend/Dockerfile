FROM node:20 AS build
COPY . .
RUN npm ci
RUN npm run build
ARG NODE_ENV=production
ENV NODE_ENV $NODE_ENV
CMD ["node", "build"]