defmodule A2020.Eleven do
  def problem do
    "./assets/11"
    |> File.read!()
    |> String.split("\n")
    |> Enum.map(&String.graphemes/1)
    |> to_2d_zipper()
    |> simulate()
    |> count_occupied()
  end

  def problem_bis do
    "./assets/11"
    |> File.read!()
    |> String.split("\n")
    |> Enum.map(&String.graphemes/1)
    |> to_2d_zipper()
    |> simulate_plus()
    |> count_occupied()
  end

  defp simulate(zipper) do
    # IO.puts("====================================")
    state = string(zipper)

    # print(zipper)
    new_zipper = zipper |> evolve() |> reset()

    new_state = string(new_zipper)

    case state == new_state do
      true -> new_zipper
      false -> simulate(new_zipper)
    end
  end

  def evolve(world) do
    new_world = to_2d_zipper([[]])

    Enum.reduce(world.up, {world, new_world}, fn line, acc ->
      {world, new_world} =
        Enum.reduce(line.left, acc, fn
          ".", {world, new_world} ->
            {
              move(world, :right),
              new_world |> put(".") |> move(:right)
            }

          cell, {world, new_world} ->
            adjacents =
              Enum.map(
                [:up, :up_left, :up_right, :down, :down_left, :down_right, :left, :right],
                &get(&1, world)
              )

            new_cell = evolve_cell(cell, adjacents)

            {
              move(world, :right),
              new_world |> put(new_cell) |> move(:right)
            }
        end)

      {
        world |> move(:right) |> move(:down),
        new_world |> move(:right) |> move(:down)
      }
    end)
    |> elem(1)
  end

  defp simulate_plus(zipper) do
    # IO.puts("====================================")
    state = string(zipper)

    # print(zipper)
    new_zipper = zipper |> evolve_plus() |> reset()

    new_state = string(new_zipper)

    case state == new_state do
      true -> new_zipper
      false -> simulate_plus(new_zipper)
    end
  end

  def evolve_plus(world) do
    new_world = to_2d_zipper([[]])

    Enum.reduce(world.up, {world, new_world}, fn line, acc ->
      {world, new_world} =
        Enum.reduce(line.left, acc, fn
          ".", {world, new_world} ->
            {
              move(world, :right),
              new_world |> put(".") |> move(:right)
            }

          cell, {world, new_world} ->
            adjacents =
              Enum.map(
                [:up, :up_left, :up_right, :down, :down_left, :down_right, :left, :right],
                &get_seat(&1, world)
              )

            new_cell = evolve_cell_plus(cell, adjacents)

            {
              move(world, :right),
              new_world |> put(new_cell) |> move(:right)
            }
        end)

      {
        world |> move(:right) |> move(:down),
        new_world |> move(:right) |> move(:down)
      }
    end)
    |> elem(1)
  end

  def evolve_cell("L", adjacents) do
    # If a seat is empty (L) and there are no occupied seats adjacent to it
    #     the seat becomes occupied.
    case Enum.all?(adjacents, &(&1 != "#")) do
      true -> "#"
      false -> "L"
    end
  end

  def evolve_cell("#", adjacents) do
    # If a seat is occupied (#) and four or more seats adjacent to it are also occupied
    #     the seat becomes empty.
    case adjacents |> Enum.filter(&(&1 == "#")) |> Enum.count() do
      count when count >= 4 -> "L"
      _ -> "#"
    end
  end

  def evolve_cell(cell, _adjacents), do: cell

  def evolve_cell_plus("L", adjacents) do
    # If a seat is empty (L) and there are no occupied seats adjacent to it
    #     the seat becomes occupied.
    case Enum.all?(adjacents, &(&1 != "#")) do
      true -> "#"
      false -> "L"
    end
  end

  def evolve_cell_plus("#", adjacents) do
    # If a seat is occupied (#) and four or more seats adjacent to it are also occupied
    #     the seat becomes empty.
    case adjacents |> Enum.filter(&(&1 == "#")) |> Enum.count() do
      count when count >= 5 -> "L"
      _ -> "#"
    end
  end

  def evolve_cell_plus(cell, _adjacents), do: cell

  def put(%{up: []} = zipper, val), do: Map.put(zipper, :up, [%{left: [val], right: []}])
  def put(%{up: [h | t]} = zipper, val), do: Map.put(zipper, :up, [put(h, val) | t])

  def put(%{left: []} = zipper, val), do: Map.put(zipper, :left, [val])
  def put(%{left: list} = zipper, val), do: Map.put(zipper, :left, [val | list])

  def to_2d_zipper(list) do
    %{
      up: Enum.map(list, &to_zipper/1),
      down: []
    }
  end

  def to_zipper(list) do
    %{
      left: list,
      right: []
    }
  end

  def reset(%{up: up, down: down}) do
    reseted_up = Enum.map(up, &reset/1)
    reseted_down = Enum.map(down, &reset/1)

    %{
      up: Enum.reduce(reseted_down, reseted_up, fn x, acc -> [x | acc] end),
      down: []
    }
  end

  def reset(%{left: left, right: right}) do
    %{
      left: Enum.reduce(right, left, fn x, acc -> [x | acc] end),
      right: []
    }
  end

  def move(world, dir), do: move({dir, world})

  def move({:up, %{up: up, down: [h | t]}}), do: %{up: [h | up], down: t}
  def move({:down, %{up: [], down: down}}), do: %{up: Enum.reverse(down), down: []}
  def move({:down, %{up: [h | t], down: down}}), do: %{up: t, down: [h | down]}
  def move({:right, %{left: [], right: right}}), do: %{left: Enum.reverse(right), right: []}

  def move({:left, %{left: left, right: [h | t]}}), do: %{left: [h | left], right: t}

  def move({:left, %{up: up, down: down}}) do
    %{
      up: Enum.map(up, &move(&1, :left)),
      down: Enum.map(down, &move(&1, :left))
    }
  end

  def move({:right, %{left: [h | t], right: right}}), do: %{left: t, right: [h | right]}

  def move({:right, %{up: up, down: down}}) do
    %{
      up: Enum.map(up, &move(&1, :right)),
      down: Enum.map(down, &move(&1, :right))
    }
  end

  def move({:up_left, zipper}), do: zipper |> move(:up) |> move(:left)
  def move({:up_right, zipper}), do: zipper |> move(:up) |> move(:right)
  def move({:down_left, zipper}), do: zipper |> move(:down) |> move(:left)
  def move({:down_right, zipper}), do: zipper |> move(:down) |> move(:right)

  def move({_dir, zipper}), do: zipper

  def get(%{up: [%{left: [h | _]} | _]}), do: h
  def get(_), do: nil

  def get(:up, %{down: [%{left: [h | _]} | _]}), do: h
  def get(:up_left, %{down: [%{right: [h | _]} | _]}), do: h
  def get(:up_right, %{down: [%{left: [_, h | _]} | _]}), do: h
  def get(:down, %{up: [_, %{left: [h | _]} | _]}), do: h
  def get(:down_left, %{up: [_, %{right: [h | _]} | _]}), do: h
  def get(:down_right, %{up: [_, %{left: [_, h | _]} | _]}), do: h
  def get(:left, %{up: [%{right: [h | _]} | _]}), do: h
  def get(:right, %{up: [%{left: [_, h | _]} | _]}), do: h
  def get(_, _), do: nil

  def get_seat(dir, zipper) do
    case get(dir, zipper) do
      "." ->
        get_seat(dir, move(zipper, dir))

      seat ->
        seat
    end
  end

  def count_occupied(zipper) do
    zipper
    |> reset()
    |> Map.get(:up)
    |> Enum.reduce(0, fn line, acc ->
      acc + Enum.count(line.left, &(&1 == "#"))
    end)
  end

  def print(%{up: up, down: down}) do
    down |> Enum.map(&print/1) |> Enum.reverse() |> Enum.each(&IO.puts/1)
    up |> Enum.map(&print/1) |> Enum.each(&IO.puts/1)
  end

  def print(%{left: left, right: right}) do
    (Enum.reverse(right) ++ left) |> Enum.join()
  end

  def string(%{up: up, down: down}) do
    "#{Enum.map(up, &string/1)}#{Enum.map(down, &string/1)}"
  end

  def string(%{left: left, right: right}) do
    "#{Enum.join(left)}#{Enum.join(right)}"
  end
end
