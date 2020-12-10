defmodule A2020.Ten do
  def problem do
    {a, _, b} =
      "./assets/10"
      |> File.read!()
      |> String.split("\n")
      |> Enum.map(&String.to_integer/1)
      |> List.insert_at(0, 0)
      |> Enum.sort()
      |> to_zipper()
      |> count_gaps()

    a * b
  end

  def problem_bis do
    "./assets/10"
    |> File.read!()
    |> String.split("\n")
    |> Enum.map(&String.to_integer/1)
    |> Enum.sort()
    |> count_combinaisons()
    |> elem(0)
  end

  defp count_gaps(zipper), do: count_gaps(zipper |> move(), {0, 0, 1})

  defp count_gaps(%{left: []}, result), do: result

  defp count_gaps(%{left: [i | _], right: [j | _]} = zipper, {a, b, c}) do
    case i - j do
      1 -> zipper |> move() |> count_gaps({a + 1, b, c})
      2 -> zipper |> move() |> count_gaps({a, b + 1, c})
      3 -> zipper |> move() |> count_gaps({a, b, c + 1})
    end
  end

  defp to_zipper(list) do
    %{left: list, right: []}
  end

  def move(%{left: [], right: _right} = zipper), do: zipper

  def move(%{left: [v | left], right: right}) do
    %{left: left, right: [v | right]}
  end

  defp count_combinaisons(list), do: count_combinaisons([0 | list], %{})

  defp count_combinaisons([], memo), do: {0, memo}
  defp count_combinaisons([a], memo), do: {1, Map.put(memo, a, 1)}

  defp count_combinaisons([h | t], memo) do
    candidates = [
      Enum.drop(t, 2),
      Enum.drop(t, 1),
      t
    ]

    {count, new_memo} =
      candidates
      |> Enum.filter(fn
        [] -> false
        [x | _] -> x <= h + 3
      end)
      |> Enum.reduce({0, memo}, fn
        [], acc ->
          acc

        list, {count, memo} ->
          {res, new_memo} = get_count_or_compute(list, memo)
          {res + count, new_memo}
      end)

    {count, Map.put(new_memo, h, count)}
  end

  defp get_count_or_compute([h | t], memo) do
    case Map.fetch(memo, h) do
      {:ok, count} -> {count, memo}
      :error -> count_combinaisons([h | t], memo)
    end
  end
end
