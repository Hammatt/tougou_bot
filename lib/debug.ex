defmodule TougouBot.Debug do
  use Alchemy.Cogs

  Cogs.def ping do
    Cogs.say "pong!"
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
    seconds = seconds - (minutes * m)
    to_string(days)<>"d, "<>to_string(hours)<>"h, "<>to_string(minutes)<>"m"
  end

  Cogs.def status do
    vsn = Mix.Project.config[:version]
    memory = to_string(:erlang.memory()[:total] /1000000)
    { { _, io_in }, { _, io_out } } = :erlang.statistics(:io)
    io_in = to_string(io_in/1000000)
    io_out = to_string(io_out/1000000)
    Cogs.say("Tougou Version: "<>vsn<>" reporting in!\nUptime: "<>uptime()<>"\nMemory: "<>memory<>
            "Mb\nIO in: "<>io_in<>"Mb\nIO out: "<>io_out<>"Mb")
  end

  Cogs.def help do
    cmds = Enum.map(Cogs.all_commands, fn ({k, _}) -> k end)
    Cogs.say "```"<>descriptions_from_cmds(cmds)<>"```"
  end

  defp descriptions_from_cmds([]) do
    ""
  end
  defp descriptions_from_cmds([head | tail]) do
    case command_descriptions[head] do
      nil ->
        "Developer forgot to describe "<>head<>",\n"<>descriptions_from_cmds(tail)
      d ->
        d<>"\n"<>descriptions_from_cmds(tail)
    end
  end

  defp command_descriptions do
    descriptions = 
    %{
      "ping" => "!ping: tougou-chan should reply with pong!",
      "status" => "!status: tougou-chan will tell you about her running version, "<>
                  "her uptime, and her memory/io stats.",
      "jisho" => "!jisho <word>: tougou-chan will check jisho for the first word "<>
                  "you give her and tell you its reading/meaning.",
      "vndb" => "!vndb <term>: tougou-chan will give you the most popular vn "<>
                "that she can find using the term on vndb.",
      "vndbrng" => "!vndbrng: tougou-chan will give you a random vn from vndb",
      "tag" => "!tag <tag>: tougou-chan will attempt to recall the content "<>
                "associated with the given tag",
      "ntag" => "!ntag <tag> <content>: tougou-chan will learn and remember"<>
                " a new tag->content pair.",
      "dtag" => "!dtag <tag>: tougou-chan will forget the specified tag",
      "atags" => "!atags: gives a list of all `tags` that tougou-chan knows",
      "help" => "!help: gives a list of all commands and their descriptions"
    }
  end
end