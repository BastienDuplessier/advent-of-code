defmodule AdventOfCode do
  def problem_1 do
    numbers =
      File.read!("./assets/1")
      |> String.split("\n")
      |> Enum.filter(&(&1 != ""))

    {_, x, _, y} =
      numbers
      |> Enum.map(fn sx ->
        {x, _} = Integer.parse(sx)
        y = 2020 - x
        {sx, x, "#{y}", y}
      end)
      |> Enum.find(fn {_, _, sy, _} -> sy in numbers end)
  end

  def problem_1_bis do
    numbers =
      File.read!("./assets/1")
      |> String.split("\n")
      |> Enum.filter(&(&1 != ""))
      |> Enum.map(&String.to_integer/1)

    full_numbers = Enum.flat_map(numbers, fn x -> Enum.map(numbers, &{x, &1, &1 + x}) end)

    {x, y, z} =
      full_numbers
      |> Enum.map(fn {x, y, xy} ->
        z = 2020 - xy
        {x, y, z}
      end)
      |> Enum.find(fn {_, _, z} -> z in numbers end)

    x * y * z
  end
end
