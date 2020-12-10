defmodule A2020.Six do
  def problem do
    "./assets/6"
    |> File.read!()
    |> String.split("\n\n")
    |> Enum.map(fn x ->
      x
      |> String.split("\n")
      |> Enum.flat_map(fn x -> String.graphemes(x) end)
      |> Enum.uniq()
      |> Enum.count()
    end)
    |> Enum.sum()
  end

  def problem_bis do
    "./assets/6"
    |> File.read!()
    |> String.split("\n\n")
    |> Enum.map(fn x ->
      x
      |> String.split("\n")
      |> Enum.map(fn x -> String.graphemes(x) end)
      |> check()
      |> Enum.count()
    end)
    |> Enum.sum()
  end

  defp check([ref | list]) do
    Enum.reduce(ref, [], fn x, acc ->
      case Enum.all?(list, &(x in &1)) do
        true -> [x | acc]
        false -> acc
      end
    end)
  end

  defp check(list), do: list
end
