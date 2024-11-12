defmodule Calions.Rustler.MixProject do
  use Mix.Project

  def project do
    [
      app: :calions_rustler,
      compilers: [:rustler] ++ Mix.compilers(),
      deps: deps(),
      elixir: "~> 1.10",
      rustler_crates: [
        calions_rustler: [
          mode: if(Mix.env() == :prod, do: :release, else: :debug)
        ]
      ],
      start_permanent: Mix.env() == :prod,
      version: "0.3.1"
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:benchee, "~> 1.0", only: :dev},
      {:earmark, "~> 1.4", only: :dev},
      {:ex_doc, "~> 0.22.2", only: :dev},
      {:rustler, "~> 0.21.1"}
    ]
  end
end
