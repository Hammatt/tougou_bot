defmodule TougouBot.Jisho do
  use Alchemy.Cogs

  Cogs.def jisho(term) do
    Cogs.say search(term)
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
    pretty = "**Japanese:** "<>parse_results_to_string_japanese(head["japanese"])
    pretty = pretty<>"**Details:** \n"<>parse_results_to_string_senses(head["senses"])
    #pretty = pretty<>parse_results_to_string_data(tail) only give the first result.
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
  defp parse_results_to_string_eng([head | tail]) do
    "\t"<>head<>"\n"<>parse_results_to_string_eng(tail)
  end
  defp parse_results_to_string_eng([]) do
    "  \n"
  end
  defp parse_results_to_string_senses([head | tail]) do
    parse_results_to_string_eng(head["english_definitions"])<>parse_results_to_string_senses(tail)
  end
  defp parse_results_to_string_senses([]) do
    "  \n"
  end
end