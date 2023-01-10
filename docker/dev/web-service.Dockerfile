FROM node:16-alpine AS build

WORKDIR /app

ARG SERVICE
ARG BACKEND

ENV SERVICE ${SERVICE}
ENV REACT_APP_API_ADDRESS ${BACKEND}

COPY ./services/${SERVICE}/package.json .
COPY ./services/${SERVICE}/package-lock.json .

RUN --mount=type=cache,target=/app/node_modules npm i --silent

COPY ./services/${SERVICE}/.eslintrc.json .
COPY ./services/${SERVICE}/tsconfig.json .

COPY ./services/${SERVICE}/src ./src
COPY ./services/${SERVICE}/public ./public

RUN --mount=type=cache,target=/app/node_modules npm run build

FROM nginx as production

WORKDIR /usr/share/nginx/html

COPY --from=build /app/build .
COPY LICENSE.md .

# redirect to index.html if the file or directory isn't found
RUN sed -i "s/index  index.html index.htm;/index  index.html index.htm;\n        try_files \$uri \$uri\/ \/index.html?\$args;/" /etc/nginx/conf.d/default.conf