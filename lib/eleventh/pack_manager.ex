defmodule Eleventh.PackManager do
  use GenServer

  @table :packs_table

  # Public API
  def start_link(_opts) do
    GenServer.start_link(__MODULE__, %{}, name: __MODULE__)
  end

  def seed(packs) when is_list(packs) do
    GenServer.call(__MODULE__, {:seed, packs})
  end

  def claim_pack(user_id) do
    GenServer.call(__MODULE__, {:claim, user_id})
  end

  def list_unassigned do
    GenServer.call(__MODULE__, :list)
  end

  # Server
  @impl true
  def init(_state) do
    :ets.new(@table, [:named_table, :set, :public, read_concurrency: true])
    {:ok, %{assigned: %{}}}
  end

  @impl true
  def handle_call({:seed, packs}, _from, state) do
    Enum.each(packs, fn pack -> :ets.insert(@table, {pack["id"], pack}) end)
    {:reply, :ok, state}
  end

  def handle_call(:list, _from, state) do
    packs = :ets.tab2list(@table) |> Enum.map(fn {_k, v} -> v end)
    {:reply, packs, state}
  end

  def handle_call({:claim, user_id}, _from, state) do
    now = DateTime.utc_now()

    case :ets.first(@table) do
      :"$end_of_table" ->
        {:reply, {:error, :no_packs}, state}

      key ->
        [{^key, pack}] = :ets.lookup(@table, key)
        :ets.delete(@table, key)
        assigned = Map.put(state.assigned, pack["id"], %{user: user_id, at: now})
        {:reply, {:ok, pack}, %{state | assigned: assigned}}
    end
  end
end
