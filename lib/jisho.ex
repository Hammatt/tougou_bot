defmodule TougouBot.Jisho do
  use Alchemy.Cogs
  alias Alchemy.Embed
  import Embed

  @jisho_colour_embed %Embed{color: 0x56D926}
  @jisho_error_embed %Embed{color: 0xff0000}

  Cogs.def jisho(term) do
    case search(term) do
      {:ok, response} ->
        #turn data into a list of maps, each map represents a different result.
        response = get_in(response, ["data"])
        case length(response) do
          0 ->
            @jisho_colour_embed
            |> field("それは居ない", "馬鹿")
            |> Embed.send
          _ ->
            #strip away all the data we don't need and turn our results into a
            # list of tuples of related data.
            results = Enum.map(response, fn(x) ->
              {x["japanese"], x["senses"]}
            end)
            IO.inspect(results)

            @jisho_colour_embed
            |> field("Jisho result for", term)
            |> Embed.send
        end
      {:error, response} ->
        @jisho_error_embed
        |> field("エラーが発生しました", response)
    end
  end

  def search(term) do
    HTTPoison.start
    case HTTPoison.get("http://jisho.org/api/v1/search/words?keyword="<>term) do
      {:ok, %HTTPoison.Response{status_code: 200, body: result, headers: _}} -> 
        {:ok, Poison.decode!(result)}
      {:ok, %HTTPoison.Response{status_code: 404}} ->
        IO.puts("jisho api 404")
        {:error, 404}
      {:error, %HTTPoison.Error{reason: e}} ->
        IO.inspect(e)
        {:error, "HTTPoison Error."}
    end
  end

end