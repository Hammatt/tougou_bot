defmodule TougouBot.Util.Parsers do
  @moduledoc """
  A separate module to store the parsers that we use in for different commands 
  to support code re-use.
  """

  #function returns all arguments concatinated with a +
  def plus_parser(args) do
    args = String.split(args)
    args = rebuild_string(args, "+")
    List.wrap(args)
  end

  #function returns all arguments concatinated by a space.
  def space_parser(args) do
    args = String.split(args)
    args = rebuild_string(args, " ")
    List.wrap(args)
  end

  #parser used by the tags module to extract the first word as one argument 
  # and then the rest all as one space separated string, resulting in two args.
  def tags_parser(args) do
    args = String.split(args)
    arg0 = Enum.at(args, 0)
    args = Enum.drop(args, 1)
    args = rebuild_string(args, " ")
    List.wrap(arg0) ++ List.wrap(args)
  end

  #helper functions to re-build our list as a string with a given concatinator
  def rebuild_string([head | []], concatenator) do
    head
  end
  def rebuild_string([head | tail], concatenator) do
    head<>concatenator<>rebuild_string(tail, concatenator)
  end
end