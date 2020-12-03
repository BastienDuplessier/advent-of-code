defmodule A2020.Three do
  @tree "#"

  def problem do
    map = "./assets/3" |> File.read!() |> String.split("\n") |> Enum.filter(&(&1 != ""))

    # {_x, trees} =
    #   Enum.reduce(map, {3, 0}, fn line, {x, trees} ->
    #     pos = Enum.at(stream, x)

    #     case String.at(line, pos) do
    #       @tree -> {x + 3, trees + 1}
    #       _ -> {x + 3, trees}
    #     end
    #   end)

    estimate_trees(map, 3, 1)
  end

  def problem_bis do
    map = "./assets/3" |> File.read!() |> String.split("\n") |> Enum.filter(&(&1 != ""))

    estimate_trees(map, 1, 1) * estimate_trees(map, 3, 1) * estimate_trees(map, 5, 1) *
      estimate_trees(map, 7, 1) * estimate_trees(map, 1, 2)
  end

  # defp estimate_trees(right, 1) do
  #   {_x, trees} =
  #     Enum.reduce(map, {right, 0}, fn line, {x, trees} ->
  #       pos = Enum.at(stream, x)

  #       case String.at(line, pos) do
  #         @tree -> {x + right, trees + 1}
  #         _ -> {x + right, trees}
  #       end
  #     end)

  #   trees
  # end

  defp estimate_trees(map, right, down) do
    imax = (map |> List.first() |> String.length()) - 1
    stream = Stream.cycle(0..imax)

    IO.inspect("=== #{right};#{down} =======================================")

    {_x, trees} =
      map
      |> Stream.drop(down)
      |> Stream.take_every(down)
      |> Enum.reduce({right, 0}, fn line, {x, trees} ->
        pos = Enum.at(stream, x)
        IO.inspect("line: #{line}")
        IO.inspect("pos: #{pos} (#{x})")

        case String.at(line, pos) |> IO.inspect() do
          @tree -> {x + right, trees + 1}
          _ -> {x + right, trees}
        end
      end)

    IO.inspect("Result --")
    trees |> IO.inspect()
  end
end
