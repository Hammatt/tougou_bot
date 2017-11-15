defmodule TougouBot.Modules.Wiki do
  @moduledoc """
  This module uses the MediaWiki action API running on https://en.wikipedia.org/
  """
  use Alchemy.Cogs

  Cogs.set_parser(:wiki, &TougouBot.Util.Parsers.plus_parser/1)
  Cogs.def wiki(term) do
    result = search(term)
    Cogs.say(result)
  end

  defp search(term) do
    HTTPoison.start()
    case HTTPoison.get("https://en.wikipedia.org/w/api.php?action=query&generator=search&gsrsearch="<>term<>"&format=json&gsrprop=snippet&prop=info&inprop=url") do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        data = Poison.decode!(body)
        case get_in(data, ["query"]) do
          nil ->
            "それは居ない"#that doesnt exist
          _ ->
            articles = get_in(data, ["query"])
            |> get_in(["pages"])
            |> Enum.find(fn {key, value} ->
              case Integer.parse(key) do
                :error -> false
                _ -> #Find the "index" of 1 for the result that should come out first.
                  case value["index"] do
                    1 -> 
                      key
                    _ ->
                      false
                  end
              end
            end)
            {_, article} = articles
            Map.fetch!(article, "fullurl")
        end
      {:ok, %HTTPoison.Response{status_code: status, body: body}} ->
        IO.inspect(body)
        "何かが壊れちゃった…: Encountered a "<>Integer.to_string(status)<>" error. Details logged."
      {:error, %HTTPoison.Error{reason: e}} ->
        IO.inspect(e)
        "なにかが壊れた"#that doesn't exist
    end
  end
end