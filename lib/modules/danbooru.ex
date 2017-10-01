defmodule TougouBot.Modules.Danbooru do
  @moduledoc """
  TODO: moduledoc
  """
  use Alchemy.Cogs

  Cogs.def pic do
    # random pic
    Cogs.say(search(""))
  end

  Cogs.set_parser(:pic, &TougouBot.Util.Parsers.plus_parser/1)
  Cogs.def pic(args) do
    #pic based on args
    Cogs.say(search(args))
  end

  defp search(tags) do
    HTTPoison.start()
    case HTTPoison.get("https://danbooru.donmai.us/posts.json?search&random=true&limit=1&tags="<>tags) do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        data = Poison.decode!(body)
        case data == nil do
          true ->
            "それは居ない"#that doesnt exist
          _ ->
            "https://danbooru.donmai.us/posts/"<>Integer.to_string(List.first(data)["id"])
        end
      {:error, e} ->
        IO.inspect(e);
        "got a bad error, check log"#todo, flavour text.
    end
  end

end