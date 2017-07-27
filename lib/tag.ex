defmodule TougouBot.Tag do
  @moduledoc """
  A `tag` is a simple mapping from a word to some contents.  

  Since the bot won't stay up 24/7 we need to keep this stored as a file.
  """
  use Alchemy.Cogs

  #pre-defined messages
  @tag_already "えー、でも、この言葉もう知てる"

  #at the moment, this will only remember one "word", 
  # anything after whitespace will be dropped
  Cogs.def ntag(tag, contents) do
    case all_tags[tag] do
      nil -> 
        write_new_tag(tag, contents)
        Cogs.say "あ〜、"<>tag<>" は "<>contents<>" 、なるほど"
      _ ->
        Cogs.say @tag_already
    end
  end
  Cogs.def ntag(tag) do
    case all_tags[tag] do
      nil ->
        Cogs.say tag<>"は何？ (!ntag <word> <meaning>)"
      _ ->
        Cogs.say @tag_already
    end
  end

  Cogs.def dtag(tag) do
    case all_tags[tag] do
      nil -> Cogs.say tag<>"は何？"
      _ ->
      delete_tag(tag)
      Cogs.say "はいよ、"<>tag<>"が忘れった"
    end
  end

  Cogs.def tag(tag) do
    Cogs.say tag_contents(tag, all_tags)
  end

  #output all tags
  Cogs.def atags do
    #turn our map into a list of key value pairs.
    tags = Enum.map(all_tags, fn({k, v}) -> {k, v} end)
    Cogs.say "```"<>tags_to_string(tags)<>"```"
  end

  defp tags_to_string([]) do
    ""
  end
  defp tags_to_string([{tag, contents} | tail]) do
    tag<>" は "<>contents<>"\n"<>tags_to_string(tail)
  end

  # The heavy lifting #

  @tag_file "tags.data"

  defp write_new_tag(tag, contents) do
    case File.read(@tag_file) do
      {:ok, ""} -> #tags data exists but is empty
        {_, new_data} = Poison.encode(%{tag => contents})
        File.write(@tag_file, new_data)
      {:ok, old_data} -> #add to existing tags data
        {_, data} = Poison.decode(old_data)
        {_, new_data} = Poison.encode(Map.merge(%{tag => contents}, data))
        File.write(@tag_file, new_data)
      {:error, :enoent} -> #file doesn't exist, write it and call self
        IO.puts("tags.data not found, creating file")
        File.write(@tag_file, "")
        write_new_tag(tag, contents)
    end
  end

  #get all of the tags as a map, remove the one, then write back.
  defp delete_tag(tag) do
    tags = all_tags
    tags = Map.drop(tags, [tag])
    {_, new_data} = Poison.encode(tags)
    File.write(@tag_file, new_data)
  end

  defp tag_contents(tag, tags) do
    case tags[tag] do
      nil -> tag<>"は何？"
      contents -> contents
    end
  end

  defp all_tags do
    case File.read(@tag_file) do
      {:ok, ""} -> #there are no tags
        %{}
      {:ok, data} ->
        {_, tags} = Poison.decode(data)
        tags
      {:error, _} -> #also no tags
        %{}
    end
  end
end