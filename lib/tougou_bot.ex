defmodule TougouBot do

  use Application
  alias Alchemy.Client

  def start(_type, _args) do
    {_, token} = File.read("token")
    token = String.strip(token)
    run = Client.start(token)
    load_modules()
    run
  end

  defp load_modules do
    use TougouBot.Debug
    use TougouBot.Jisho
    use TougouBot.VNDB
    use TougouBot.Tag
  end
end
