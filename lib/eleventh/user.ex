defmodule Eleventh.User do
  defstruct [:id, :last_claim_at, :inventory, :server]

  def new(id, server) do
    %__MODULE__{id: id, last_claim_at: nil, inventory: [], server: server}
  end
end
