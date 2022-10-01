FROM ubuntu:latest
RUN apt-get update && apt-get install -y ca-certificates libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*

COPY target/release/backend /usr/local/bin/
RUN chmod +x /usr/local/bin/backend
RUN ls -l /usr/local/bin/backend

EXPOSE 8080

CMD ["backend"]