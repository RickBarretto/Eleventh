FROM denoland/deno:alpine-1.38.3

WORKDIR /app

COPY . /app

EXPOSE 8000

CMD ["run", "--allow-net", "--allow-env", "src/server.ts"]
