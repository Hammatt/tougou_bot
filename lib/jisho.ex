defmodule TougouBot.Jisho do
  use Alchemy.Cogs
  alias Alchemy.Embed
  import Embed

  @jisho_colour_embed %Embed{color: 0x56D926}

  Cogs.def jisho(term) do
    #todo: rework here to have readings and details parse into separate fields.
    @jisho_colour_embed
    |> field("Jisho result for "<>term<>":", search(term))
    |> Embed.send
  end

  def search(term) do
    HTTPoison.start
    case HTTPoison.get("http://jisho.org/api/v1/search/words?keyword="<>term) do
      {:ok, %HTTPoison.Response{status_code: 200, body: result, headers: _}} -> 
        data = Poison.decode!(result)
        parse_results_to_string_data(data["data"])
      {:ok, %HTTPoison.Response{status_code: 404}} ->
        IO.puts("jisho api 404")
      {:error, %HTTPoison.Error{reason: e}} ->
        IO.inspect(e)
    end
  end

  #TODO: is there a way to make this parser more pretty?
  #only take the first jisho result
  defp parse_results_to_string_data([head | _]) do
    pretty = "**Reading(s):**  \n\t"<>parse_results_to_string_japanese(head["japanese"])
    pretty = pretty<>"**Definition(s):** \n"<>parse_results_to_string_senses(head["senses"], 1)
    pretty
  end
  defp parse_results_to_string_data([]) do
    "Couldn't find any results"#TODO make line more tougouesque
  end
  defp parse_results_to_string_japanese([%{"word" => _, "reading" => _} = head | tail]) do
    head["word"]<>"("<>head["reading"]<>")  \n"<>parse_results_to_string_japanese(tail)
  end
  defp parse_results_to_string_japanese([%{"reading" => _} = head | tail]) do
    head["reading"]<>"  \n"<>parse_results_to_string_japanese(tail)
  end
  defp parse_results_to_string_japanese([]) do
    "  \n"
  end
  defp parse_results_to_string_eng([head | []]) do
    "\t"<>head<>"\n"
  end
  defp parse_results_to_string_eng([head | tail]) do
    "\t"<>head<>", "<>parse_results_to_string_eng(tail)
  end
  defp parse_results_to_string_senses([head | tail], index) do
    "**"<>to_string(index)<>":**"<>parse_results_to_string_eng(head["english_definitions"])<>parse_results_to_string_senses(tail, index+1)
  end
  defp parse_results_to_string_senses([], _) do
    "  \n"
  end
end