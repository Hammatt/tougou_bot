defmodule TougouBot.Mixfile do
  use Mix.Project

  def project do
    [
      app: :tougou_bot,
      version: "0.3.0",
      elixir: "~> 1.5",
      start_permanent: Mix.env == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
      [extra_applications: [:logger]]
      [mod: {TougouBot, []}]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:poison, "~> 3.1"},
      {:httpoison, "~> 0.10.0"},
      {:alchemy, "~> 0.6.0", hex: :discord_alchemy},
      {:floki, "~> 0.17.0"}
    ]
  end
end
