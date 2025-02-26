FROM nginx:latest

COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE  80

CMD ["/usr/sbin/nginx","-g","daemon off;"]

