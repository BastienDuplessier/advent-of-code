defmodule A2020.Five do
  require Integer

  def problem do
    "./assets/5"
    |> File.read!()
    |> String.split("\n")
    |> Enum.reduce(0, fn pass, max ->
      case get_seat(pass) do
        %{id: id} when id > max -> id
        _seat -> max
      end
    end)
  end

  def problem_bis do
    "./assets/5"
    |> File.read!()
    |> String.split("\n")
    |> Enum.map(&get_seat/1)
    |> Enum.sort_by(&{&1.column, &1.row})
    |> to_zipper()
    |> find_missing()
  end

  defp get_seat(pass), do: get_seat(pass, 0, 127)

  defp get_seat("F" <> rest, lower, upper) do
    new_upper = upper - center(lower, upper)
    get_seat(rest, lower, new_upper)
  end

  defp get_seat("B" <> rest, lower, upper) do
    new_lower = lower + center(lower, upper)
    get_seat(rest, new_lower, upper)
  end

  defp get_seat(pass, row, row) do
    column = get_column(pass, 0, 7)
    build_seat(row, column)
  end

  defp get_column("L" <> rest, lower, upper) do
    new_upper = upper - center(lower, upper)
    get_column(rest, lower, new_upper)
  end

  defp get_column("R" <> rest, lower, upper) do
    new_lower = lower + center(lower, upper)
    get_column(rest, new_lower, upper)
  end

  defp get_column("", column, column), do: column

  defp center(a, b), do: div(b - a + 1, 2)
  defp get_id(row, column), do: row * 8 + column
  defp build_seat(row, column), do: %{row: row, column: column, id: get_id(row, column)}

  defp to_zipper(list) do
    %{left: list, right: []}
  end

  defp find_missing(%{right: []} = zipper), do: zipper |> move() |> find_missing()
  defp find_missing(%{left: []}), do: :error

  defp find_missing(zipper) do
    %{left: [behind | _], right: [before | _]} = zipper

    with true <- before.column == behind.column, 2 <- behind.row - before.row do
      row = before.row + 1
      column = before.column
      build_seat(row, column)
    else
      _ -> zipper |> move() |> find_missing()
    end
  end

  def move(%{left: [], right: right} = zipper), do: zipper

  def move(%{left: [v | left], right: right}) do
    %{left: left, right: [v | right]}
  end
end
