defmodule A2020.Two do
  def problem do
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

  def problem_bis do
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
end
