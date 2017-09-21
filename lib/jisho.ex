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

            #only take the top result.
            {readings, senses} = List.first(results)
            readings_str = to_readings_str(readings)
            definitions_str = to_definitions_str(1, senses)
            @jisho_colour_embed
            |> field("Reading(s):", readings_str)
            |> field("Definition(s):", definitions_str)
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

  defp to_readings_str([h | []]) do
    to_readings_str_helper(h)
  end
  defp to_readings_str([h | t]) do
    to_readings_str_helper(h)<>", "<>to_readings_str(t)
  end
  defp to_readings_str_helper(h) do
    case h["word"] do
      nil ->
        h["reading"]
      _ ->
        h["word"]<>"("<>h["reading"]<>")"
    end
  end

  defp to_definitions_str(_, []) do
    ""#parts of speach and english definitions are lists. need to fix.
  end
  defp to_definitions_str(1, [h | t]) do
    "1. "<>Enum.join(h["parts_of_speech"], ", ")<>": "<>Enum.join(h["english_definitions"], ", ")<>to_definitions_str(1+1, t)
  end
  defp to_definitions_str(i, [h | t]) do
    "\n"<>Integer.to_string(i)<>". "<>Enum.join(h["english_definitions"], ", ")<>to_definitions_str(i+1, t)
  end

end