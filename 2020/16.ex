defmodule A2020.Sixteen do
  def problem do
    {rules, _, tickets} =
      "./assets/16"
      |> File.read!()
      |> String.split("\n\n")
      |> Enum.map(&String.split(&1, "\n"))
      |> parse()

    tickets |> find_invialid_values(rules) |> Enum.sum()
  end

  def problem_bis do
    {rules, ticket, tickets} =
      "./assets/16"
      |> File.read!()
      |> String.split("\n\n")
      |> Enum.map(&String.split(&1, "\n"))
      |> parse()

    ok_tickets = filter_invialid_tickets(tickets, rules)

    rules_order = compute_rule_order(tickets, rules)
  end

  defp parse([rules, [_, ticket], [_ | other_tickets]]) do
    {
      parse_rules(rules),
      parse_ticket(ticket),
      Enum.map(other_tickets, &parse_ticket/1)
    }
  end

  defp parse_rules(rules) do
    Enum.reduce(rules, %{}, fn rule, map ->
      [name, a, b, c, d] = String.split(rule, [": ", "-", " or "])

      values = Enum.map([a, b, c, d], &String.to_integer/1)
      Map.put(map, name, values)
    end)
  end

  defp parse_ticket(values), do: values |> String.split(",") |> Enum.map(&String.to_integer/1)

  defp find_invialid_values(tickets, rules) do
    tickets
    |> List.flatten()
    |> Enum.reduce([], fn value, acc ->
      valid =
        Enum.any?(rules, fn {_, [min1, max1, min2, max2]} ->
          (value >= min1 && value <= max1) || (value >= min2 && value <= max2)
        end)

      case valid do
        true -> acc
        false -> [value | acc] |> IO.inspect()
      end
    end)
  end

  defp filter_invialid_tickets(tickets, rules) do
    Enum.filter(tickets, fn ticket ->
      Enum.all?(ticket, fn value ->
        Enum.any?(rules, fn {_, [min1, max1, min2, max2]} ->
          (value >= min1 && value <= max1) || (value >= min2 && value <= max2)
        end)
      end)
    end)
  end

  defp compute_rule_order(tickets, rules) do
    rules_by_values =
      tickets
      |> Enum.map(fn ticket ->
        Enum.map(ticket, fn value ->
          rules
          |> Enum.filter(fn {_, [min1, max1, min2, max2]} ->
            (value >= min1 && value <= max1) || (value >= min2 && value <= max2)
          end)
          |> Enum.map(&elem(&1, 0))
        end)
      end)
  end

  defp transpose([head | tail]) do
    Enum.reduce(tail, Enum.map(head, &[&1]), fn line, acc ->
      line |> Enum.zip(acc) |> Enum.map(fn {a, b} -> [a | b] end)
    end)
  end
end
