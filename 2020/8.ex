defmodule A2020.Eight do
  def problem do
    instructions =
      "./assets/8"
      |> File.read!()
      |> String.split("\n")
      |> Enum.reduce({0, %{}}, fn line, {i, map} ->
        {i + 1, Map.put(map, i, parse(line))}
      end)
      |> elem(1)
      |> loop(0, {0, []}, 0)
      |> elem(1)
  end

  def bench_problem_bis do
    t1 = Time.utc_now()
    result = problem_bis
    t2 = Time.utc_now()
    time = Time.diff(t2, t1)
    IO.inspect("Result : #{result} in #{time}ms")
  end

  def problem_bis do
    instructions =
      "./assets/8"
      |> File.read!()
      |> String.split("\n")
      |> Enum.reduce({0, %{}}, fn line, {i, map} ->
        {i + 1, Map.put(map, i, parse(line))}
      end)
      |> elem(1)

    instructions
    |> Stream.filter(fn {_, {cmd, _}} -> cmd == :nop || cmd == :jmp end)
    |> Stream.map(fn {i, cmd} ->
      {i, Map.put(instructions, i, reverse(cmd))}
    end)
    |> Enum.find(fn {i, instructions} ->
      case loop(instructions, 0, {0, []}, 0) do
        {:end, _acc} -> true
        _ -> false
      end
    end)
    |> elem(1)
    |> loop(0, {0, []}, 0)
    |> elem(1)
  end

  defp parse("nop " <> int) do
    {:nop, String.to_integer(int)}
  end

  defp parse("acc " <> int) do
    {:acc, String.to_integer(int)}
  end

  defp parse("jmp " <> int) do
    {:jmp, String.to_integer(int)}
  end

  defp parse(oth) do
    # IO.inspect("unknown instruction #{oth}")
    {:nop, 0}
  end

  defp loop(instructions, i, {_acc, _visited}, 1000) do
    # IO.inspect("stack overflow")
    :error
  end

  defp loop(_instructions, :end, {acc, _visited}, _stack) do
    # IO.inspect("finished program")
    {:end, acc}
  end

  defp loop(instructions, i, {acc, visited}, stack) do
    # IO.inspect("i = #{i}, visited = #{inspect(visited)}")

    case i in visited do
      true ->
        # IO.inspect("loop detected")
        {:loop, acc}

      false ->
        {i2, acc} = instructions |> Map.get(i) |> execute(i, acc)
        loop(instructions, i2, {acc, [i | visited]}, stack + 1)
    end
  end

  defp execute({:nop, _arg}, i, acc) do
    # IO.inspect("nop, i = #{i}, acc = #{acc}")
    {i + 1, acc}
  end

  defp execute({:acc, arg}, i, acc) do
    # IO.inspect("acc #{arg}, i = #{i}, acc = #{acc}")
    {i + 1, acc + arg}
  end

  defp execute({:jmp, arg}, i, acc) do
    # IO.inspect("jmp #{arg}, i = #{i}, acc = #{acc}")
    {i + arg, acc}
  end

  defp execute(_, i, acc) do
    # IO.inspect("end, i = #{i}, acc = #{acc}")
    {:end, acc}
  end

  def reverse({:nop, arg}), do: {:jmp, arg}
  def reverse({:jmp, arg}), do: {:nop, arg}
end
