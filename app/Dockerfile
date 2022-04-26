FROM node as build
WORKDIR /app
COPY package*.json ./
RUN  npm ci
COPY src src
COPY public public
COPY *.json *.js ./
COPY Makefile inject.sh ./
RUN  make

FROM nginx as production
COPY nginx.conf /etc/nginx
COPY --from=build /app/public /public
RUN  nginx -t
EXPOSE 8081
