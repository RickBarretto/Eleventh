defmodule Eleventh.MatchManager do
  use GenServer

  def start_link(_opts) do
    GenServer.start_link(__MODULE__, %{}, name: __MODULE__)
  end

  def create_match(host_id, guest_id) do
    GenServer.call(__MODULE__, {:create, host_id, guest_id})
  end

  @impl true
  def init(_state) do
    {:ok, %{matches: %{}}}
  end

  @impl true
  def handle_call({:create, host, guest}, _from, state) do
    id = :crypto.strong_rand_bytes(8) |> Base.url_encode64()

    match = %{
      id: id,
      host: host,
      guest: guest,
      score: %{host: 1, guest: 0},
      started_at: DateTime.utc_now()
    }

    matches = Map.put(state.matches, id, match)
    {:reply, {:ok, match}, %{state | matches: matches}}
  end
end
