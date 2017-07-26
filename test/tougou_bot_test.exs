defmodule TougouBotTest do
  use ExUnit.Case
  doctest TougouBot

  test "greets the world" do
    assert TougouBot.hello() == :world
  end
end
