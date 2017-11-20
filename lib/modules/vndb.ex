defmodule TougouBot.Modules.VNDB do
  @moduledoc """
  This module uses the https://vndb.org/ api to search for visual novels.
  """
  use Alchemy.Cogs

  Cogs.set_parser(:vndb, &TougouBot.Util.Parsers.plus_parser/1)
  Cogs.def vndb(term) do
    result = search(term)
    Cogs.say(result)
  end

  Cogs.def vndbrng do
    Cogs.say(random_vn())
  end

  def search(term) do
    HTTPoison.start()
    case HTTPoison.get("https://vndb.org/v/all?sq="<>term<>";o=d;s=rating") do
      {:ok, %HTTPoison.Response{status_code: 307, body: _, headers: headers}} -> 
        get_first_location_header(headers)
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} -> 
        data = Floki.find(body, "table.stripe")
        data = Floki.find(extract_top_table(data), "td.tc1")
        data = Floki.find(data, "a")
        extract_indexed_result(data, 2)
      {:ok, %HTTPoison.Response{status_code: status, body: body, headers: headers}} ->
        TougouBot.Util.Error_Handler.handle_http_error(status, body, headers)
        "何かが壊れちゃった…: Encountered a "<>Integer.to_string(status)<>" error. Details logged."
      {:error, %HTTPoison.Error{reason: e}} ->
        IO.inspect(e)
    end
  end

  defp get_first_location_header([{"Location", url} | _tail]) do
    url
  end
  defp get_first_location_header([_ | tail]) do
    get_first_location_header(tail)
  end
  defp get_first_location_header([]) do
    IO.puts("Error, could not find location in vndb header")
  end
  defp extract_top_table([head | _]) do
    head
  end
  defp extract_indexed_result([], _) do
    "ばーーーか！"
  end
  defp extract_indexed_result([head | _], 0) do
    {_, [{"href", vnid}, {_, _}], _} = head
    "https://vndb.org"<>vnid
  end
  defp extract_indexed_result([_ | tail], count) do
    extract_indexed_result(tail, count-1)
  end

  def random_vn do
    HTTPoison.start()
    case HTTPoison.get("https://vndb.org/v/rand") do
      {:ok, %HTTPoison.Response{status_code: 307, body: _, headers: headers}} -> 
        get_first_location_header(headers)
      {:ok, %HTTPoison.Response{status_code: 404}} ->
        "VNDB seems to be down, got 404."
      {:error, %HTTPoison.Error{reason: e}} ->
        IO.inspect(e)
        "got a bad error, check log"
    end
  end
end