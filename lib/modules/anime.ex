defmodule TougouBot.Modules.Anime do
  @moduledoc """
  The Anime module is in charge of handling searches for anime by using the 
  myanimelist api (https://myanimelist.net/modules.php?go=api)
  """
  use Alchemy.Cogs

  Cogs.set_parser(:anime, &TougouBot.Util.Parsers.plus_parser/1)
  Cogs.def anime(term) do
    results = search(term, "anime")
    Cogs.say(results)
  end
  Cogs.set_parser(:manga, &TougouBot.Util.Parsers.plus_parser/1)
  Cogs.def manga(term) do
    results = search(term, "manga")
    Cogs.say(results)
  end

  defp search(term, type) do
    [username, password] = 
      case File.read("mal") do
        {:ok, body} ->
          String.split(body)
        {:error, e} ->
          IO.inspect(e)
          ["", ""]
      end 
    HTTPoison.start()
    case HTTPoison.get("https://"<>username<>":"<>password<>"@myanimelist.net/api/"<>type<>"/search.xml?q="<>term) do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        id = Floki.find(body, "entry")
            |> Floki.find("id")
            |> Enum.map(fn ({_, _, v}) -> v end)
            |> List.first
            |> List.first
        "https://myanimelist.net/"<>type<>"/"<>id
      {:ok, %HTTPoison.Response{status_code: 204, body: _}} ->
        "それは居ない"#that doesn't exist
      {:ok, %HTTPoison.Response{status_code: status, body: body, headers: headers}} ->
        TougouBot.Util.Error_Handler.handle_http_error(status, body, headers)
        "何かが壊れちゃった… Details logged."
      {:error, %HTTPoison.Error{reason: e}} ->
        IO.inspect(e)
        "got a bad error, check log"#todo, flavour text.
    end
  end
end