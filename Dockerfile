FROM debian:bullseye
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && update-ca-certificates \    
    && rm -rf /var/lib/apt/lists/*

COPY target/release/rust_api_send_message_telegram /usr/local/bin/

EXPOSE 7000

CMD ["rust_api_send_message_telegram"]
