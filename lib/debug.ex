defmodule TougouBot.Debug do
  use Alchemy.Cogs

  Cogs.def ping do
    Cogs.say "pong!"
  end

  def uptime do
    {seconds, _} = :erlang.statistics(:wall_clock)

    seconds = seconds + (60*60*1000)#DEBUG LINE

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
end