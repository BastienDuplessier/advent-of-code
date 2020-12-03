defmodule AdventOfCode do
  # Imports <+>
  import MyOperators

  def problem_2 do
    rx = ~r/(\d+)\-(\d+) (\w): (\w+)/

    input =
      rx
      |> Regex.scan(File.read!("./assets/2"))

    Enum.reduce(input, 0, fn [_, min, max, l, pwd], acc ->
      [code] = String.to_charlist(l)
      count = pwd |> String.to_charlist() |> Enum.filter(&(&1 == code)) |> Enum.count()

      case count >= String.to_integer(min) && count <= String.to_integer(max) do
        true -> acc + 1
        false -> acc
      end
    end)
  end

  def problem_2_bis do
    rx = ~r/(\d+)\-(\d+) (\w): (\w+)/

    input =
      rx
      |> Regex.scan(File.read!("./assets/2"))

    Enum.reduce(input, 0, fn [_, sa, sb, l, pwd], acc ->
      a = String.to_integer(sa) - 1
      b = String.to_integer(sb) - 1

      case l == String.at(pwd, a) != (l == String.at(pwd, b)) do
        true -> acc + 1
        false -> acc
      end
    end)
  end

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
