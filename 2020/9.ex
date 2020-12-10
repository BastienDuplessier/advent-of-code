defmodule A2020.Nine do
  # @size 5
  @size 25

  def problem do
    File.read!("./assets/9")
    |> String.split("\n")
    |> Enum.map(&String.to_integer/1)
    |> check(@size)
    |> elem(1)
  end

  def problem_bis do
    list =
      File.read!("./assets/9")
      |> String.split("\n")
      |> Enum.map(&String.to_integer/1)

    {:ok, number} = check(list, @size)

    {a, b} = find_weakness(list, number)
    a + b
  end

  defp check(list, preamble_size) do
    check(Enum.drop(list, preamble_size), list, preamble_size)
  end

  defp check([], _checklist, _preamble_size), do: :error

  defp check([elt | list], checklist, preamble_size) do
    preamble = Enum.take(checklist, preamble_size)

    case check_sum(preamble, elt) do
      true -> check(list, Enum.drop(checklist, 1), preamble_size)
      false -> {:ok, elt}
    end
  end

  defp check_sum(list, target) do
    list |> combine |> Enum.member?(target)
  end

  defp combine(list) do
    Enum.reduce(list, [], fn i, acc -> Enum.reduce(list, acc, &[i + &1 | &2]) end)
  end

  defp find_weakness([], number), do: :error

  defp find_weakness(list, number) do
    # IO.inspect("====== #{List.first(list)} ======")

    case find_weakness(list, number, {0, []}) do
      {:ok, [h | t]} -> Enum.reduce(t, {h, h}, fn x, {mn, mx} -> {min(mn, x), max(mx, x)} end)
      :error -> find_weakness(Enum.drop(list, 1), number)
    end
  end

  defp find_weakness(_, number, {number, list}), do: {:ok, list}
  defp find_weakness([], _, _), do: :error

  defp find_weakness(list, number, {acc, ll}) when acc > number do
    # IO.puts("acc too high #{acc} ; #{inspect([0 | ll])}")
    :error
  end

  defp find_weakness([head | tail], number, {acc, acc_list}) do
    # IO.inspect("add #{head} to #{inspect([0 | acc_list])}, sum is #{acc + head}")
    find_weakness(tail, number, {acc + head, [head | acc_list]})
  end
end
