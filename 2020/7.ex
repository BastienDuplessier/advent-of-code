defmodule A2020.Seven do
  def problem do
    File.read!("./assets/7")
    |> String.split("\n")
    |> Enum.filter(&(&1 != ""))
    |> Enum.reduce(%{}, fn line, map ->
      {color, rule} = parse_rule(line)
      Map.put(map, color, rule)
    end)
    |> find_parents("shiny gold")
    |> Enum.count()
  end

  def problem_bis do
    File.read!("./assets/7")
    |> String.split("\n")
    |> Enum.filter(&(&1 != ""))
    |> Enum.reduce(%{}, fn line, map ->
      {color, rule} = parse_rule(line)
      Map.put(map, color, rule)
    end)
    |> count_bags("shiny gold")
    |> Map.get("shiny gold")
  end

  defp parse_rule(str) do
    [color, "contain " <> rest] = String.split(str, " bags ", parts: 2)

    {color, parse_list(rest)}
  end

  defp parse_list("no other bags."), do: []

  defp parse_list(list) do
    list
    |> String.trim(".")
    |> String.split(", ")
    |> Enum.map(fn bags ->
      {count, " " <> rest} = Integer.parse(bags)
      [color | _] = rest |> String.split(" bag")

      {count, color}
    end)
  end

  # ["muted yellow", "bright white"]
  defp find_parents(rules, bag), do: find_parents(rules, bag, [], [bag])

  defp find_parents(_rules, nil, result, _visited), do: result

  defp find_parents(rules, bag, result, visited) do
    nodes =
      Enum.reduce(rules, [], fn {key, list}, acc ->
        Enum.reduce(list, acc, fn
          {_, ^bag}, acc -> [key | acc]
          _, acc -> acc
        end)
      end)

    new_result =
      Enum.reduce(nodes, result, fn node, result ->
        case node in result do
          true -> result
          false -> [node | result]
        end
      end)

    next_bag = List.first(new_result -- visited)
    find_parents(rules, next_bag, new_result, [bag | visited])
  end

  def count_bags(rules, bag), do: count_bags(rules, bag, %{})

  def count_bags(rules, bag, bag_sizes) do
    {size, new_bag_sizes} =
      rules
      |> Map.get(bag)
      |> Enum.reduce({0, bag_sizes}, fn {count, bag}, {sum, bag_sizes} ->
        {size, new_bag_sizes} =
          case Map.fetch(bag_sizes, bag) do
            {:ok, size} ->
              {size, bag_sizes}

            :error ->
              new_bag_sizes = count_bags(rules, bag, bag_sizes)
              size = Map.get(new_bag_sizes, bag)
              {size, new_bag_sizes}
          end

        {count * (1 + size) + sum, new_bag_sizes}
      end)

    Map.put(new_bag_sizes, bag, size)
  end
end
