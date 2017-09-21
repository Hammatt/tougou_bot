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
        IO.inspect(response)#TODO debug line
        case length(response) do
          0 ->
            @jisho_colour_embed
            |> field("それは居ない", "馬鹿")
            |> Embed.send
          _ ->
            #TODO: finish working out better way of parsing the results
            # just need some way to extract the contents of each "japanese" key 
            # and also each english_definitions key.
            # need to think about what to do for wikipedia definitions, they seem 
            # to be under the "links" key.
            Enum.map(response, fn(x) ->
              #one result
              #need to turn this result into a heading (the first reading/word)
              # and a list of definitions and possibly alternate readings/forms.
            end)

            @jisho_colour_embed
            |> field("Jisho result for", term)
            #|> field("Jisho result for "<>term<>":", search(term))
            #|> field("details", )
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