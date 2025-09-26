defmodule Eleventh.Application do
  use Application

  def start(_type, _args) do
    children = [
      {Registry, keys: :unique, name: Eleventh.UserRegistry},
      Eleventh.PackManager,
      Eleventh.MatchManager,
      {Plug.Cowboy, scheme: :http, plug: Eleventh.Router, options: [port: port()]}
    ]

    opts = [strategy: :one_for_one, name: Eleventh.Supervisor]
    Supervisor.start_link(children, opts)
  end

  defp port do
    case System.get_env("PORT") do
      nil -> 4000
      str -> String.to_integer(str)
    end
  end
end
