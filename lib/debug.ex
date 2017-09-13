defmodule TougouBot.Debug do
  use Alchemy.Cogs
  alias Alchemy.Embed
  import Embed

  Cogs.def ping do
    Cogs.say "pong!"
  end

  defp rng(bound, limit) do
    case Integer.parse bound do
      :error ->
        "Lower bound not a number."#todo flavour text
      {i, _} ->
        case Integer.parse limit do
          :error ->
            "Upper limit not a number."#todo flavour text
          {j, _} ->
            (:rand.uniform(j-i))+i
        end
    end
  end
  Cogs.def roll do
    Cogs.say rng "0", "100"
  end
  Cogs.def roll(limit) do
    Cogs.say rng "0", limit
  end
  Cogs.def roll(bound, limit) do
    Cogs.say rng bound, limit
  end

  def uptime do
    {seconds, _} = :erlang.statistics(:wall_clock)
    d = (24*60*60*1000)
    days = div(seconds, d)
    seconds = seconds - (days * d)
    h = (60*60*1000)
    hours = div(seconds, h)
    seconds = seconds - (hours * h)
    m = (60*1000)
    minutes = div(seconds, m)
    #seconds = seconds - (minutes * m)
    to_string(days)<>"d, "<>to_string(hours)<>"h, "<>to_string(minutes)<>"m"
  end

  Cogs.def status do
    %Embed { color: 0xFFB6C1 }
    |> field("Version:", Mix.Project.config[:version])
    |> field("Uptime:", uptime())
    |> field("Memory use:", to_string(:erlang.memory()[:total] /1000000)<>"Mb")
    |> Embed.send
  end

  Cogs.def help do
    %Embed{ color: 0x8B4513, 
          fields: List.wrap(Enum.map(command_descriptions(), 
                              fn({k, v}) -> %Embed.Field{name: "!"<>k, value: v} end)) }
    |> Embed.send
  end

  defp command_descriptions() do
    %{
      "ping" => "Tougou-chan should reply with pong!",
      "status" => "Tougou-chan will tell you about her running version, "<>
                  "her uptime, and her memory/io stats.",
      "jisho" => "Takes one `word` as an argument. Tougou-chan will check jisho "<>
                  "for the first word you give her and tell you its reading/meaning.",
      "vndb" => "Takes one `term` as an argument. Tougou-chan will give you the most popular vn "<>
                "that she can find using the term on vndb.",
      "vndbrng" => "Tougou-chan will give you a random vn from vndb",
      "tag" => "Takes one `tag` as an argument Tougou-chan will attempt to "<>
                "recall the content associated with the given tag",
      "ntag" => "Takes one `tag`, then one `content` as arguments. Tougou-chan "<>
                "will learn and remember a new tag->content pair.",
      "dtag" => "Takes one `tag` as an argument. Tougou-chan will forget the specified tag",
      "atags" => "gives a list of all `tags` that Tougou-chan knows",
      "help" => "gives a list of all commands and their descriptions"
    }
  end
end