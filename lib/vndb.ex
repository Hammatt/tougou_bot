defmodule TougouBot.VNDB do
  use Alchemy.Cogs

  Cogs.set_parser(:vndb, &TougouBot.VNDB.custom_parser/1)
  Cogs.def vndb(term) do
    result = search(term)
    Cogs.say result
  end
  def rebuild_string([head | []]) do
    head
  end
  def rebuild_string([head | tail]) do
    head<>"+"<>rebuild_string(tail)
  end
  def custom_parser(args) do
    args = String.split(args)
    args = rebuild_string(args)
    List.wrap(args)
  end


  def search(term) do
    HTTPoison.start
    case HTTPoison.get("https://vndb.org/v/all?sq="<>term<>";o=d;s=rating") do
      {:ok, %HTTPoison.Response{status_code: 307, body: _, headers: headers}} -> 
        get_first_location_header(headers)
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} -> 
        data = Floki.find(body, "table.stripe")
        data = Floki.find(extract_top_table(data), "td.tc1")
        data = Floki.find(data, "a")
        data = extract_indexed_result(data, 2)
      {:ok, %HTTPoison.Response{status_code: 404}} ->
        IO.puts("VNDB search 404")
      {:error, %HTTPoison.Error{reason: e}} ->
        IO.inspect(e)
    end
  end

  defp get_first_location_header([{"Location", url} | tail]) do
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
end