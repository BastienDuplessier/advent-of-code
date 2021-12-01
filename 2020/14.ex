defmodule A2020.Fourteen do
  use Bitwise

  def problem do
    "./assets/14"
    |> File.read!()
    |> String.split("\n")
    |> parse()
    |> execute(%{}, [])
    |> Map.values()
    |> Enum.sum()
  end

  def problem_bis do
    "./assets/14"
    |> File.read!()
    |> String.split("\n")
    |> parse()
  end

  defp parse(list) do
    list
    |> Enum.reverse()
    |> Enum.reduce([], fn cmd, acc -> [parse_cmd(cmd) | acc] end)
  end

  defp parse_cmd("mask = " <> mask), do: {:chg_mask, String.graphemes(mask)}

  defp parse_cmd("mem[" <> rest) do
    {ptr, "] = " <> str_val} = Integer.parse(rest)
    {:chg_mem, {ptr, String.to_integer(str_val)}}
  end

  defp execute([], mem, _mask), do: mem

  defp execute([cmd | rest], mem, mask) do
    {mem, mask} = execute_cmd(cmd, mem, mask)

    execute(rest, mem, mask)
  end

  defp execute_cmd({:chg_mask, mask}, mem, _mask), do: {mem, mask}

  defp execute_cmd({:chg_mem, {ptr, raw_val}}, mem, mask) do
    val = apply_mask(mask, raw_val)
    {Map.put(mem, ptr, val), mask}
  end

  defp execute_cmd(_, mem, mask), do: {mem, mask}

  def apply_mask(mask, i) do
    mask
    |> deconstruct(i)
    |> Enum.reduce(0, fn
      {"X", i}, acc -> (acc + i) <<< 1
      {"1", _i}, acc -> (acc + 1) <<< 1
      {"0", _i}, acc -> acc <<< 1
    end) >>> 1
  end

  def apply_address_mask(mask, i) do
    mask
    |> deconstruct(i)
    |> Enum.reduce([], fn
      {"X", _i}, acc -> ["X" | acc]
      {"1", _i}, acc -> ["1" | acc]
      {"0", i}, acc -> ["#{i}" | acc]
    end)
    |> Enum.reverse()
    |> Enum.reduce([0], fn
      "0", acc -> Enum.map(acc, &(&1 <<< 1))
      "1", acc -> Enum.map(acc, &((&1 <<< 1) + 1))
      "X", acc -> Enum.map(acc, &((&1 <<< 1) + 1))
    end)
  end

  def deconstruct(mask, i), do: mask |> Enum.reverse() |> deconstruct(i, [])

  def deconstruct([], _i, acc), do: acc
  def deconstruct([h | t], i, acc), do: deconstruct(t, div(i, 2), [{h, rem(i, 2)} | acc])
end
