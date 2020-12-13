defmodule A2020.Twelve do
  def problem do
    {_dir, x, y} =
      "./assets/12"
      |> File.read!()
      |> String.split("\n")
      |> Enum.map(&parse_instruction/1)
      |> execute({:east, 0, 0})

    abs(x) + abs(y)
  end

  def problem_bis do
    {_wpt, x, y} =
      "./assets/12"
      |> File.read!()
      |> String.split("\n")
      |> Enum.map(&parse_instruction/1)
      |> execute_wpt({{10, 1}, 0, 0})

    abs(x) + abs(y)
  end

  defp parse_instruction("N" <> val), do: {:north, String.to_integer(val)}
  defp parse_instruction("S" <> val), do: {:south, String.to_integer(val)}
  defp parse_instruction("E" <> val), do: {:east, String.to_integer(val)}
  defp parse_instruction("W" <> val), do: {:west, String.to_integer(val)}
  defp parse_instruction("L" <> val), do: {:left, String.to_integer(val)}
  defp parse_instruction("R" <> val), do: {:right, String.to_integer(val)}
  defp parse_instruction("F" <> val), do: {:forward, String.to_integer(val)}

  def execute([], state), do: state

  def execute([instruction | rest], state) do
    # IO.inspect("=====================")
    # IO.inspect(instruction)
    # IO.inspect(state)
    new_state = navigate(instruction, state)
    execute(rest, new_state)
  end

  def navigate({:forward, val}, {dir, _x, _y} = acc), do: move(acc, dir, val)
  def navigate({cmd, val}, acc) when cmd == :left or cmd == :right, do: rotate(acc, cmd, val)
  def navigate({dir, val}, acc), do: move(acc, dir, val)

  def move({dir, x, y}, :north, val), do: {dir, x, y + val}
  def move({dir, x, y}, :south, val), do: {dir, x, y - val}
  def move({dir, x, y}, :east, val), do: {dir, x + val, y}
  def move({dir, x, y}, :west, val), do: {dir, x - val, y}

  def rotate({dir, x, y}, target_dir, val), do: {rotate(dir, target_dir, val), x, y}

  def rotate(dir, target_dir, val) do
    val |> to_changes(target_dir) |> shift(dir) |> get_direction()
  end

  def shift(val, :north), do: val
  def shift(val, :east), do: val + 1
  def shift(val, :south), do: val + 2
  def shift(val, :west), do: val + 3

  @directions [:north, :east, :south, :west]
  def get_direction(drop) do
    @directions |> Stream.cycle() |> Stream.drop(drop) |> Enum.take(1) |> List.first()
  end

  def to_changes(90, :left), do: 3
  def to_changes(180, :left), do: 2
  def to_changes(270, :left), do: 1
  def to_changes(90, :right), do: 1
  def to_changes(180, :right), do: 2
  def to_changes(270, :right), do: 3

  def execute_wpt([], state), do: state

  def execute_wpt([instruction | rest], state) do
    IO.inspect("=====================")
    IO.inspect(instruction)
    IO.inspect(state)
    new_state = navigate_wpt(instruction, state)
    execute_wpt(rest, new_state)
  end

  def navigate_wpt({:forward, val}, {{xx, yy} = wpt, x, y}) do
    {wpt, x + xx * val, y + yy * val}
  end

  def navigate_wpt({cmd, val}, {wpt, x, y}) when cmd == :left or cmd == :right do
    {rotate_wpt(wpt, cmd, val), x, y}
  end

  def navigate_wpt({dir, val}, {wpt, x, y}), do: {move_wpt(wpt, dir, val), x, y}

  def move_wpt({x, y}, :north, val), do: {x, y + val}
  def move_wpt({x, y}, :south, val), do: {x, y - val}
  def move_wpt({x, y}, :east, val), do: {x + val, y}
  def move_wpt({x, y}, :west, val), do: {x - val, y}

  def rotate_wpt({x, y}, _dir, 180), do: {-x, -y}
  def rotate_wpt({x, y}, :right, 90), do: {y, -x}
  def rotate_wpt({x, y}, :right, 270), do: {-y, x}
  def rotate_wpt({x, y}, :left, 90), do: {-y, x}
  def rotate_wpt({x, y}, :left, 270), do: {y, -x}
end
