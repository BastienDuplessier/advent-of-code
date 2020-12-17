defmodule A2020.Fifteen do
  def problem do
    "./assets/15"
    |> File.read!()
    |> String.split(",")
    |> Enum.map(&String.to_integer/1)
    |> find_x(2020)
  end

  def problem_bis do
    "./assets/15"
    |> File.read!()
    |> String.split(",")
    |> Enum.map(&String.to_integer/1)
    |> find_x(30_000_000)
  end

  def find_x(init_list, max) do
    next_turn = Enum.count(init_list)

    init_list
    |> Enum.with_index()
    |> Enum.map(fn {v, i} ->
      # IO.puts("Turn #{i + 1} is #{v} =====================")
      {v, i}
    end)
    |> Enum.reduce(%{}, fn {v, i}, map -> Map.put(map, v, i) end)
    |> find_2020(max - 1, next_turn, 0)
  end

  def find_2020(_map, max, max, val) do
    # IO.puts("Turn 2020 is #{val} =====================")
    val
  end

  def find_2020(map, max, turn, val) do
    # IO.puts("Turn #{turn + 1} is #{val} =====================")

    new_val =
      case Map.fetch(map, val) do
        {:ok, last_turn} -> turn - last_turn
        :error -> 0
      end

    map |> Map.put(val, turn) |> find_2020(max, turn + 1, new_val)
  end
end
