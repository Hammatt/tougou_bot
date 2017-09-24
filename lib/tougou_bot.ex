defmodule TougouBot do

  use Application
  alias Alchemy.Client

  def start(_type, _args) do
    {_, token} = File.read("token")
    token = String.trim(token)
    run = Client.start(token)
    load_modules()
    run
  end

  defp load_modules do
    use TougouBot.Modules.Debug
    use TougouBot.Modules.Jisho
    use TougouBot.Modules.VNDB
    use TougouBot.Modules.Tag
    use TougouBot.Modules.Wiki
    use TougouBot.Modules.Anime
  end
end
