defmodule TougouBotTest do
  use ExUnit.Case
  doctest TougouBot

  test "util plus parser" do
    assert TougouBot.Util.Parsers.plus_parser("1 23 4_5") == ["1+23+4_5"]
  end
  test "util space parser" do
    assert TougouBot.Util.Parsers.space_parser("1 23 4_5") == ["1 23 4_5"]
  end
  test "util tags parser" do
    assert TougouBot.Util.Parsers.tags_parser("1 23 4_5") == ["1", "23 4_5"]
  end
end
