Benchee.run(%{
  "ulid" => fn -> Calions.Rustler.ulid(1_000_000) end,
  "uuid_v4" => fn -> Calions.Rustler.uuid_v4(1_000_000) end
})
