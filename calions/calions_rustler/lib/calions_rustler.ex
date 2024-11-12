defmodule Calions.Rustler do
  use Rustler, otp_app: :calions_rustler, crate: "calions_rustler"

  @doc ~S"""
  Decrypt an encrypted value with a key and return decrypted binary string

  Returns:
  - `{:ok, "decrypted_value"}` in case of success
  - `:error_decrypt` when decrypting encrypted value with the given key failed
  - `:error_value` when parsing encrypted value failed
  - `:error_key` when parsing key failed

  ## Examples

      iex> Calions.Rustler.gdpr_decrypt("secret", "my_key")
      {:ok, "secret"}

  """
  def gdpr_decrypt(_a, _b), do: :erlang.nif_error(:nif_not_loaded)

  @doc ~S"""
  Encrypt a binary string and return key with encrypted value

  Returns:
  - `{:ok, ["key", "encrypted_value"]}` in case of success
  - `:error_encrypt` when generating key or encrypting value

  ## Examples

      iex> Calions.Rustler.gdpr_encrypt("secret")
      {:ok, ["key", "encrypted_value"]}

  """
  def gdpr_encrypt(_a), do: :erlang.nif_error(:nif_not_loaded)

  @doc ~S"""
  Encrypt a binary string with a key and return encrypted value

  Returns:
  - `{:ok, "encrypted_value"}` in case of success
  - `:error_encrypt_with_key` when encrypting value with given key failed
  - `:error_key` when parsing key failed

  ## Examples

      iex> Calions.Rustler.gdpr_encrypt("secret", "my_key")
      {:ok, "encrypted_value"}

  """
  def gdpr_encrypt(_a, _b), do: :erlang.nif_error(:nif_not_loaded)

  @doc ~S"""
  Generate a GDPR key

  Returns:
  - `{:ok, "key"}` in case of success
  - `:error_key` when generating key failed

  ## Examples

      iex> Calions.Rustler.gdpr_key()
      {:ok, "my_key"}

  """
  def gdpr_key(), do: :erlang.nif_error(:nif_not_loaded)

  @doc ~S"""
  Generate a Monotonic ULID

  ## Examples

      iex> Calions.Rustler.ulid()
      {:ok, "01EB7ETY5EH3VSY8GP945XATN2"}

  """
  def ulid(_a), do: :erlang.nif_error(:nif_not_loaded)

  @doc ~S"""
  Generate a V4 UUID

  ## Examples

      iex> Calions.Rustler.uuid_v4()
      "42926399-b766-4b35-af5d-a71d820ceaf0"

  """
  def uuid_v4(_a), do: :erlang.nif_error(:nif_not_loaded)
end
