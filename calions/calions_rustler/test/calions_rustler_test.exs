defmodule Calions.RustlerTest do
  use ExUnit.Case
  alias Calions.Rustler

  test "it cannot decrypt base64 decodable" do
    {:ok, key} = Rustler.gdpr_key()

    secret = "my_secret_password"
    {:ok, encrypted} = Rustler.gdpr_encrypt(secret, key)

    assert :error_value == Rustler.gdpr_decrypt("test", key)
  end

  test "it cannot decrypt with another key" do
    {:ok, key} = Rustler.gdpr_key()
    {:ok, other_key} = Rustler.gdpr_key()

    secret = "my_secret_password"
    {:ok, encrypted} = Rustler.gdpr_encrypt(secret, key)

    assert :error_decrypt == Rustler.gdpr_decrypt(encrypted, other_key)
  end

  test "it encrypts and decrypts with a custom key" do
    key = :crypto.strong_rand_bytes(44) |> :base64.encode()
    secret = "my_secret_password"
    {:ok, encrypted} = Rustler.gdpr_encrypt(secret, key)
    {:ok, decrypted} = Rustler.gdpr_decrypt(encrypted, key)

    assert is_bitstring(key)
    assert is_bitstring(encrypted)
    assert decrypted == secret
  end

  test "it encrypts and decrypts with generated key" do
    secret = "my_secret_password"
    {:ok, [key, encrypted]} = Rustler.gdpr_encrypt(secret)
    {:ok, decrypted} = Rustler.gdpr_decrypt(encrypted, key)

    assert is_bitstring(key)
    assert is_bitstring(encrypted)
    assert decrypted == secret
  end

  test "it generates a gdpr key" do
    {:ok, key} = Rustler.gdpr_key()

    assert is_bitstring(key)
  end

  test "it generates a monotonic ulid" do
    {:ok, [ulid]} = Rustler.ulid(1)

    assert String.match?(ulid, ~r/[0-9A-Z]{26}/)
  end

  test "it generates a v4 uuid" do
    [uuid] = Rustler.uuid_v4(1)

    assert String.match?(
             uuid,
             ~r/\b[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}\b/
           )
  end
end
