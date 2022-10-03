FROM ubuntu:latest
RUN apt-get update && apt-get install -y ca-certificates libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*

COPY target/release/backend /usr/local/bin/
RUN chmod +x /usr/local/bin/backend
COPY web_ui/web /opt/sepama

EXPOSE 8080

CMD ["backend"]