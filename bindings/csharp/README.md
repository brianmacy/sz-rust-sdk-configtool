# Sz.ConfigTool (.NET)

Unofficial C# binding for the Senzing configuration library
(`libSzConfigTool`): functional, stateless edits of a `g2config.json` document.

> **Unofficial.** This is an unofficial library: Senzing does not publicly
> document most configuration functions and parameters. Use it only with
> Senzing-provided guidance on what to change and when.

```csharp
using Sz.ConfigTool;

string config = File.ReadAllText("g2config.json");
config = SzConfigTool.AddDataSource(config, "CUSTOMERS");
string sources = SzConfigTool.ListDataSources(config);   // JSON text
```

* Namespace `Sz.ConfigTool`, static class `SzConfigTool`, `netstandard2.0`.
* Every method takes the configuration JSON first and returns the result:
  every config-changing function (`config`, `config_and_json`) returns the new
  config string, so calls chain; `json` functions return JSON text (parse it
  with any JSON library). A `config_and_json` function also has a companion
  `<Name>Result` (same arguments and overloads) that returns the record the
  operation produces, as JSON text: `AddAttributeResult(...)` → the new
  `CFG_ATTR` row. The companion RE-RUNS the operation (a second run): call
  it with the SAME input config you gave the primary, never the config the
  primary returned (on that config `AddAttributeResult` fails with
  `ALREADY_EXISTS`, `Delete*Result` with `NOT_FOUND`, and
  `SetGenericPlanResult` reports `WasCreated` = `"false"`). Configs are
  opaque and byte-exact.
* Named results are a `<Name>Record` record whose fields are each member's
  JSON text: `SetGenericPlanResult(...)` → `SetGenericPlanRecord(PlanId,
  WasCreated)` (e.g. `"3"`, `"true"`), `VerifyCompatibilityVersion(...)` →
  `VerifyCompatibilityVersionRecord(CurrentVersion, Matches)` (e.g.
  `"\"11\""` — quotes included, `"true"`).
* Call selectors (`int_or_str` args, e.g. `call`) have `long` and `string`
  overloads: `GetComparisonCall(config, 34L)` selects by call id,
  `GetComparisonCall(config, "TAX_ID")` by feature code.
* Methods are PascalCase, arguments camelCase, generated from
  [`api/manifest`](../../api/manifest) (the XML docs come from it too).
  Placeholders marked `not_implemented` are not in the typed API; they remain
  callable with `SzConfigTool.Invoke(name, config, argsJson)`.
* Optional arguments are `null` by default (omitted; the library applies its
  own default). Tri-state arguments are `FieldUpdate<T>`:
  `default` = leave, `FieldUpdate<string>.Clear` = clear, a value (implicit) = set.
* Errors throw `SzConfigToolException` with `ReasonCode` (e.g. `NOT_FOUND`),
  `Kind` (`SzConfigToolErrorKind`, one constant per reason code: same identity),
  `Message` and `Details` (validation-errors
  JSON for `VALIDATION_ERRORS`). Errors are captured per thread; the class is
  thread-safe.
* A string with a lone surrogate (no UTF-8 form) is rejected before the call
  with `SzConfigToolException` `INVALID_INPUT`, as in every other binding; so
  is a NUL character in `name`, `config` or raw `args_json` (the C ABI takes
  NUL-terminated strings; a NUL in a typed string arg is JSON-escaped and
  passes unchanged). A null `name`/`config` throws `ArgumentNullException`.
* Windows: the bundled `SzConfigTool.dll` uses the static MSVC runtime (no VC++
  Redistributable needed).

See [`bindings/CONTRACT.md`](../CONTRACT.md) for the cross-language contract.

## Layout

| Path | Content |
|---|---|
| `src/Sz.ConfigTool/` | the library; `Generated/*.g.cs` are generated (do not edit) |
| `tests/Sz.ConfigTool.Tests/` | xUnit tests against the real native library |
| `examples/Sz.ConfigTool.Example/` | runnable example |
| `natives/<rid>/` | pack-time input: native libraries per RID (not checked in) |
| `natives.props` | supported RIDs and native file names |

## Build and test

Requires the .NET 8 SDK and Rust. From the repository root:

```bash
cargo build -p sz-configtool-ffi --release        # target/release/libSzConfigTool.*
cargo run -p sz-configtool-codegen -- --check     # generated C# is up to date
dotnet test bindings/csharp/Sz.ConfigTool.sln
```

The tests copy the native library from `target/release` (or
`$CARGO_TARGET_DIR/release`, or `$SZCONFIGTOOL_NATIVE_DIR`) to
`runtimes/<rid>/native/` in the test output and run every conformance case in
`api/manifest/generated/conformance.json` through the typed API.

Coverage (coverlet, `-p:CollectCoverage=true`): `packaging/coverage.sh rust dotnet`
from the repository root (see `packaging/README.md`, Coverage).

After editing `api/manifest/*.yaml`, regenerate with
`cargo run -p sz-configtool-codegen`.

## Pack

Place each RID's native library under `bindings/csharp/natives/<rid>/`
(names from `natives.props`), then pack:

```text
natives/linux-x64/libSzConfigTool.so
natives/linux-arm64/libSzConfigTool.so
natives/osx-arm64/libSzConfigTool.dylib
natives/win-x64/SzConfigTool.dll
```

```bash
dotnet pack bindings/csharp/src/Sz.ConfigTool -c Release -o out \
  -p:SzRequireAllNatives=true            # fail if any RID is missing
# other natives location: -p:SzNativesDir=/path/to/natives/
```

The `.nupkg` contains `lib/netstandard2.0/Sz.ConfigTool.dll` and
`runtimes/<rid>/native/<library>`. Without `SzRequireAllNatives=true` a missing
RID is a warning (useful for a local single-platform package). Nothing is
published to nuget.org; packages are attached to GitHub Releases.

## Consume a GitHub-Release package

Download `Sz.ConfigTool.<version>.nupkg` from the
[releases page](https://github.com/brianmacy/sz-rust-sdk-configtool/releases)
into a directory, register it as a local feed, and reference it:

```bash
mkdir -p ~/nuget-local && cp Sz.ConfigTool.*.nupkg ~/nuget-local/
dotnet new console -n ConfigDemo && cd ConfigDemo
dotnet nuget add source ~/nuget-local --name sz-local   # once
dotnet add package Sz.ConfigTool --source sz-local
```

Then `Program.cs`:

```csharp
using Sz.ConfigTool;

string config = File.ReadAllText(args[0]);
config = SzConfigTool.AddDataSource(config, "CUSTOMERS");
Console.WriteLine(SzConfigTool.ListDataSources(config));
try
{
    SzConfigTool.AddDataSource(config, "CUSTOMERS");
}
catch (SzConfigToolException e) when (e.Kind == SzConfigToolErrorKind.AlreadyExists)
{
    Console.WriteLine($"{e.ReasonCode}: {e.Message}");
}
```

```bash
dotnet run -- /path/to/g2config.json
```

The .NET SDK copies `runtimes/<rid>/native/` into the output; the library also
resolves `runtimes/<rid>/native/` next to `Sz.ConfigTool.dll` itself before
falling back to the default native loader (so a native library on the system
search path also works).

## Runnable example (in this repository)

```bash
dotnet run --project bindings/csharp/examples/Sz.ConfigTool.Example -- \
  tests/fixtures/g2config_template.json CUSTOMERS
```
