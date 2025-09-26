FROM elixir:1.14
WORKDIR /app
COPY . /app
RUN mix do local.hex --force, local.rebar --force
RUN mix deps.get
RUN mix compile
EXPOSE 4000
CMD ["mix", "run", "--no-halt"]
