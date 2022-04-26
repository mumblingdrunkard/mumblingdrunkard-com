FROM node as build
WORKDIR /app
COPY package*.json ./
RUN  npm ci
COPY src src
COPY public public
COPY *.json *.js ./
COPY Makefile inject.sh ./
RUN  make

# reduce number of requests needed
RUN  mv public/app/static/build/main-*.js public/app/static/build/main.js

FROM nginx as production
COPY nginx.conf /etc/nginx
COPY --from=build /app/public /public
RUN  nginx -t
EXPOSE 8081
