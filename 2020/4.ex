defmodule A2020.Four do
  # Required
  @required ~w(byr ecl eyr hcl hgt iyr pid)

  # byr (Birth Year)
  # iyr (Issue Year)
  # eyr (Expiration Year)
  # hgt (Height)
  # hcl (Hair Color)
  # ecl (Eye Color)
  # pid (Passport ID)

  # Optionnal
  # cid (Country ID)

  def problem do
    "./assets/4"
    |> File.read!()
    |> parse_passports()
    |> Enum.reduce(0, fn passport, count ->
      case validate_passport(passport) do
        true -> count + 1
        false -> count
      end
    end)
  end

  def problem_bis do
    "./assets/4"
    |> File.read!()
    |> parse_passports()
    |> Enum.reduce(0, fn passport, count ->
      case deep_validate_passport(passport) do
        %{"valid" => true} -> count + 1
        _ -> count
      end
    end)
  end

  def parse_passports(file) do
    file |> String.split("\n\n") |> Enum.filter(&(&1 != "")) |> Enum.map(&parse_passport/1)
  end

  def parse_passport(str) do
    str
    |> String.split(["\n", " "])
    |> Enum.reduce(%{"valid" => true}, fn data, passport ->
      case String.split(data, ":") do
        [key, val] -> Map.put(passport, key, val)
        _ -> passport
      end
    end)
  end

  def validate_passport(passport) do
    missing = Enum.find(@required, &(passport |> Map.get(&1) |> is_nil()))

    !missing
  end

  def deep_validate_passport(passport) do
    passport
    |> validate(:byr)
    |> validate(:iyr)
    |> validate(:eyr)
    |> validate(:hgt)
    |> validate(:hcl)
    |> validate(:ecl)
    |> validate(:pid)
  end

  def validate(%{"valid" => false} = passport, _field), do: passport

  def validate(passport, :byr) do
    with byr when not is_nil(byr) <- Map.get(passport, "byr"),
         {year, ""} <- Integer.parse(byr) |> IO.inspect(),
         true <- year >= 1920 && year <= 2002 do
      passport
    else
      _ -> invalidate(passport)
    end
  end

  def validate(passport, :iyr) do
    with iyr when not is_nil(iyr) <- Map.get(passport, "iyr"),
         {year, ""} <- Integer.parse(iyr),
         true <- year >= 2010 && year <= 2020 do
      passport
    else
      _ -> invalidate(passport)
    end
  end

  def validate(passport, :eyr) do
    with eyr when not is_nil(eyr) <- Map.get(passport, "eyr"),
         {year, ""} <- Integer.parse(eyr),
         true <- year >= 2020 && year <= 2030 do
      passport
    else
      _ -> invalidate(passport)
    end
  end

  def validate(passport, :hgt) do
    case passport |> Map.get("hgt", "") |> Integer.parse() do
      {hgt, "cm"} -> validate_height(passport, hgt, :cm)
      {hgt, "in"} -> validate_height(passport, hgt, :in)
      _ -> invalidate(passport)
    end
  end

  def validate(passport, :hcl) do
    with "#" <> hcl <- passport |> Map.get("hcl", ""),
         true <- String.length(hcl) == 6,
         hcl
         |> String.to_charlist()
         # 48 = 0, 57 = 9, 97 = a, 102 = f
         |> Enum.all?(&((&1 >= 48 && &1 <= 57) || (&1 >= 97 && &1 <= 102))) do
      passport
    else
      _ -> invalidate(passport)
    end
  end

  @authorized_colors ~w(amb blu brn gry grn hzl oth)
  def validate(passport, :ecl) do
    case Map.get(passport, "ecl") in @authorized_colors do
      true ->
        passport

      _ ->
        invalidate(passport)
    end
  end

  def validate(passport, :pid) do
    with pid <- Map.get(passport, "pid", ""),
         true <- String.length(pid) == 9,
         true <-
           pid
           |> String.to_charlist()
           # 48 = 0, 57 = 9
           |> Enum.all?(&(&1 >= 48 && &1 <= 57)) do
      passport
    else
      _ -> invalidate(passport)
    end
  end

  def validate(passport, _), do: invalidate(passport)

  def validate_height(passport, hgt, :cm) do
    case hgt >= 150 && hgt <= 193 do
      true -> passport
      false -> invalidate(passport)
    end
  end

  def validate_height(passport, hgt, :in) do
    case hgt >= 59 && hgt <= 76 do
      true -> passport
      false -> invalidate(passport)
    end
  end

  def invalidate(passport), do: %{passport | "valid" => false}
end
