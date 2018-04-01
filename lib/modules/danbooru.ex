defmodule TougouBot.Modules.Danbooru do
  @moduledoc """
  The Danbooru module is in charge of the !pic commands which use the danbooru 
  api to get a random image according to the tags given (or from all images if none given.)
  """
  use Alchemy.Cogs
  alias Alchemy.Embed
  import Embed

  Cogs.def pic do
    # random pic
    Embed.send(search(""))
  end

  Cogs.set_parser(:pic, &TougouBot.Util.Parsers.plus_parser/1)
  Cogs.def pic(args) do
    #pic based on args
    Embed.send(search(args))
  end

  @danbooru_colour_embed %Embed{color: 0xADD8E6}
  @danbooru_error_embed %Embed{color: 0xff0000}

  defp search(tags) do
    HTTPoison.start()
    case HTTPoison.get("https://danbooru.donmai.us/posts.json?search&random=true&limit=1&tags="<>tags) do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        data = Poison.decode!(body)
        case List.first(data) == nil do
          true ->
            @danbooru_error_embed
            |> title("それは居ない")#that doesnt exist
          _ ->
            IO.puts(List.first(data)["file_url"])
            @danbooru_colour_embed
            |> title("<"<>"https://danbooru.donmai.us/posts/"<>Integer.to_string(List.first(data)["id"])<>">")
            |> image(List.first(data)["file_url"])
        end
      {:ok, %HTTPoison.Response{status_code: 500, body: body, headers: headers}} ->
        TougouBot.Util.Error_Handler.handle_http_error(500, body, headers)
        @danbooru_error_embed
        |> title("何かが壊れちゃった…")
        |> description("Possibly because Danbooru has a 2 tag search limit. Details logged.")
      {:ok, %HTTPoison.Response{status_code: status, body: body, headers: headers}} ->
        TougouBot.Util.Error_Handler.handle_http_error(status, body, headers)
        @danbooru_error_embed
        |> title("何かが壊れちゃった…")
        |> description("Encountered a "<>Integer.to_string(status)<>" error. Details logged.")
      {:error, e} ->
        IO.inspect(e);
        @danbooru_error_embed
        |> title("何かが壊れちゃった…")
        |> description("got a bad error, check log")
    end
  end

end