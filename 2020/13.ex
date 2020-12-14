defmodule A2020.Thirteen do
  def problem do
    {bus, next_stop, time_to_wait} =
      "./assets/13" |> File.read!() |> String.split("\n") |> parse() |> get_next_bus()

    bus * time_to_wait
  end

  def problem_bis do
    busses = "./assets/13" |> File.read!() |> String.split("\n") |> parse_bis()
    # {compute(busses), compute_bis(busses)}
    next_sequence(busses)
  end

  defp parse([estimate, busses]) do
    {
      String.to_integer(estimate),
      busses |> String.split(",") |> Enum.filter(&(&1 != "x")) |> Enum.map(&String.to_integer/1)
    }
  end

  defp get_next_bus({time, busses}) do
    busses
    |> Enum.map(fn bus ->
      next_stop = (div(time, bus) + 1) * bus
      {bus, next_stop, next_stop - time}
    end)
    |> Enum.sort_by(fn {_, _, diff} -> diff end)
    |> List.first()
  end

  defp parse_bis([_estimate, busses]) do
    busses
    |> String.split(",")
    |> Enum.map(fn
      "x" -> "x"
      bus -> String.to_integer(bus)
    end)
  end

  defp compute(busses), do: compute(busses, 1)

  defp compute([first | busses], i) do
    time = first * i

    result =
      Enum.reduce(busses, 1, fn
        _bus, :error ->
          :error

        "x", shift ->
          shift + 1

        bus, shift ->
          case rem(time + shift, bus) do
            0 when time > bus -> shift + 1
            _ -> :error
          end
      end)

    case result do
      :error -> compute([first | busses], i + 1)
      _ -> time
    end
  end

  # https://elixirforum.com/t/advent-of-code-2020-day-13/36180/5
  def next_sequence(busses) do
    busses
    |> Enum.with_index()
    |> Enum.reduce({0, 1}, &add_to_sequence/2)
    |> elem(0)
  end

  defp add_to_sequence({"x", _index}, state), do: state

  defp add_to_sequence({bus, index}, {t, step}) do
    if rem(t + index, bus) == 0 do
      {t, lcm(step, bus)}
    else
      add_to_sequence({bus, index}, {t + step, step})
    end
  end

  defp lcm(a, b) do
    div(a * b, Integer.gcd(a, b))
  end
end
