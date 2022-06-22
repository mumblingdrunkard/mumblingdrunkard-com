FROM nginx as production
COPY nginx.conf /etc/nginx
COPY public public
RUN  nginx -t
EXPOSE 8082
