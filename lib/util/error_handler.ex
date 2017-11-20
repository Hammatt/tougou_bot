defmodule TougouBot.Util.Error_Handler do
  @moduledoc """
  A separate module to handle different types of errors.
  I found that I was often repeating myself (especially when it came to handling http errors), 
  so this functionality has been modularised.
  """

  #Function to report an error into the console, in our live version we pipe 
  # all output into a log txt file so we just output like normal here and it'll all be logged.
  def handle_http_error(status, body, headers) do
    IO.puts("-----HTTP Error Report Start-----")

    IO.puts("HTTP Status: "<>Integer.to_string(status))

    IO.puts("---HTTP Headers Start---")
    IO.inspect(headers)
    IO.puts("---HTTP Headers End---")
    
    IO.puts("---HTTP Body Start---")
    IO.inspect(body)
    IO.puts("---HTTP Body End---")

    IO.puts("-----HTTP Error Report End-----")
  end
end