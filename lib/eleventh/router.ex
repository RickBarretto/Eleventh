defmodule Eleventh.Router do
  use Plug.Router

  plug(Plug.Parsers, parsers: [:json], json_decoder: Jason)
  plug(:match)
  plug(:dispatch)

  get "/health" do
    send_resp(conn, 200, "ok")
  end

  post "/join" do
    id = UUID.uuid4()
    server = to_string(node())
    user = Eleventh.User.new(id, server)
    Registry.register(Eleventh.UserRegistry, id, user)
    send_resp(conn, 201, Jason.encode!(%{id: id}))
  end

  post "/claim" do
    %{"user_id" => user_id} = conn.body_params

    case Registry.lookup(Eleventh.UserRegistry, user_id) do
      [{pid, user}] ->
        last = user.last_claim_at
        now = DateTime.utc_now()
        allowed = is_nil(last) or DateTime.diff(now, last, :second) >= 24 * 3600

        if allowed do
          case Eleventh.PackManager.claim_pack(user_id) do
            {:ok, pack} ->
              new_user = %{user | last_claim_at: now, inventory: [pack | user.inventory]}
              Registry.update_value(Eleventh.UserRegistry, user_id, fn _ -> new_user end)
              send_resp(conn, 200, Jason.encode!(%{pack: pack}))

            {:error, :no_packs} ->
              send_resp(conn, 409, Jason.encode!(%{error: "no_packs"}))
          end
        else
          send_resp(conn, 429, Jason.encode!(%{error: "too_soon"}))
        end

      [] ->
        send_resp(conn, 404, Jason.encode!(%{error: "user_not_found"}))
    end
  end

  post "/match" do
    %{"host_id" => host, "guest_id" => guest} = conn.body_params

    case Eleventh.MatchManager.create_match(host, guest) do
      {:ok, match} ->
        send_resp(conn, 201, Jason.encode!(%{match: match}))
    end
  end

  post "/seed" do
    # Accepts optional { "packs": [...] } payload. If missing, seed default soccer packs.
    packs = Map.get(conn.body_params || %{}, "packs")

    packs_to_seed =
      cond do
        is_list(packs) ->
          packs

        true ->
          Enum.map(1..10, fn i ->
            %{
              "id" => "pack_#{i}",
              "name" => "Soccer Pack #{i}",
              "cards" => [
                %{
                  "id" => "card_#{i}_1",
                  "player" => "Player #{i} A",
                  "rating" => Enum.random(60..99)
                },
                %{
                  "id" => "card_#{i}_2",
                  "player" => "Player #{i} B",
                  "rating" => Enum.random(60..99)
                },
                %{
                  "id" => "card_#{i}_3",
                  "player" => "Player #{i} C",
                  "rating" => Enum.random(60..99)
                }
              ]
            }
          end)
      end

    :ok = Eleventh.PackManager.seed(packs_to_seed)
    send_resp(conn, 201, Jason.encode!(%{status: "seeded", count: length(packs_to_seed)}))
  end

  post "/trade" do
    %{"from_id" => from, "to_id" => to} = conn.body_params

    case Registry.lookup(Eleventh.UserRegistry, from) do
      [{_pid, from_user}] ->
        case from_user.inventory do
          [card | rest] ->
            new_from = %{from_user | inventory: rest}
            Registry.update_value(Eleventh.UserRegistry, from, fn _ -> new_from end)

            case Registry.lookup(Eleventh.UserRegistry, to) do
              [{_p2, to_user}] ->
                new_to = %{to_user | inventory: [card | to_user.inventory]}
                Registry.update_value(Eleventh.UserRegistry, to, fn _ -> new_to end)
                send_resp(conn, 200, Jason.encode!(%{status: "ok", card: card}))

              [] ->
                Registry.update_value(Eleventh.UserRegistry, from, fn _ -> from_user end)
                send_resp(conn, 404, Jason.encode!(%{error: "recipient_not_found"}))
            end

          [] ->
            send_resp(conn, 409, Jason.encode!(%{error: "no_cards"}))
        end

      [] ->
        send_resp(conn, 404, Jason.encode!(%{error: "from_not_found"}))
    end
  end

  match _ do
    send_resp(conn, 404, "not found")
  end
end
