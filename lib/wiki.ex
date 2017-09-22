defmodule TougouBot.Wiki do
  use Alchemy.Cogs

  Cogs.def wiki(term) do
    result = search(term)
    Cogs.say result
  end

  Cogs.set_parser(:search, &TougouBot.Wiki.custom_parser/1)
  defp search(term) do
    HTTPoison.start
    case HTTPoison.get("https://en.wikipedia.org/w/api.php?action=query&generator=search&gsrsearch="<>term<>"&format=json&gsrprop=snippet&prop=info&inprop=url") do
      {:ok, %HTTPoison.Response{status_code: 404}} ->
        term<>"は404."
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
                    1 -> key
                    _ -> false
                  end
              end
            end)
            {_, article} = articles
            Map.fetch!(article, "fullurl")
        end
      {:error, %HTTPoison.Error{reason: e}} ->
        IO.inspect(e)
        "なにかが壊れた"#that doesn't exist
    end
  end
  #parser so that we search for not just the first word.
  def custom_parser(args) do
    args = String.split(args)
    args = rebuild_string(args)
    List.wrap(args)
  end
  def rebuild_string([head | []]) do
    head
  end
  def rebuild_string([head | tail]) do
    head<>" "<>rebuild_string(tail)
  end
end