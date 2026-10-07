# sz-configtool — Java binding

Unofficial Java (JDK 17+) binding of `sz_configtool_lib`: stateless, typed
operations on Senzing configuration JSON documents (`g2config.json`). No
runtime dependencies; not a replacement for (and no dependency on) the
official Senzing Java SDK — no engine is involved.

> **Unofficial.** This is an unofficial library: Senzing does not publicly
> document most configuration functions and parameters. Use it only with
> Senzing-provided guidance on what to change and when.

```
SzConfigTool (generated, static camelCase methods)
   -> NativeBridge.invoke(name, config, argsJson)     one JNI method
   -> bindings/jni (Rust cdylib szconfigtool_jni)     -> sz_configtool_api::invoke
```

## Install (GitHub Releases only)

Nothing is published to Maven Central. Download `sz-configtool-<version>.jar`
from the repository's GitHub Release and either put it on the classpath or
install it into your local Maven repository:

```bash
mvn install:install-file -Dfile=sz-configtool-<version>.jar \
  -DgroupId=io.github.brianmacy -DartifactId=sz-configtool -Dversion=<version> -Dpackaging=jar
```

## Build from source

```bash
# from the repository root
cargo build -p sz-configtool-jni --release          # -> target/release/libszconfigtool_jni.{dylib,so} / szconfigtool_jni.dll
cd bindings/java && mvn package                     # tests + jar with this host's native bundled
```

`mvn` picks the library up from `${CARGO_TARGET_DIR:-../../target}/release`
(`-Dnative.profile=debug` for a debug build) and bundles it as
`natives/<os>-<arch>/<lib>` (`macos-aarch64`, `macos-x86_64`, `linux-x86_64`,
`linux-aarch64`, `windows-x86_64`, `windows-aarch64`).

**Multi-platform jar:** drop prebuilt libraries into
`bindings/java/natives/<os>-<arch>/<lib>` before `mvn package`; that tree is
copied into the jar as-is.

## Loading the native library

`NativeBridge` loads it once per JVM, in this order:

| Step | How |
|---|---|
| 1 | `-Dszconfigtool.native.path=/abs/libszconfigtool_jni.so` — `System.load` of that file (explicit override) |
| 2 | Bundled `natives/<os>-<arch>/<lib>`, extracted to `<dir>/<version>-<sha256 prefix>/<lib>` via temp file + atomic rename (safe for concurrent JVMs; an identical existing file — e.g. a Windows DLL locked by another JVM — is reused) |
| 3 | `System.loadLibrary("szconfigtool_jni")` (name: `-Dszconfigtool.native.name`) from `java.library.path` |

`<dir>` is `-Dszconfigtool.native.dir` or
`${java.io.tmpdir}/sz-configtool-jni-${user.name}`. If the temp directory is
mounted `noexec`, point `szconfigtool.native.dir` at an exec-allowed directory.

## Usage

Runnable example ([examples/Example.java](examples/Example.java)), from
`bindings/java` after `mvn package` (which writes `target/sz-configtool-<version>.jar`;
with a release download, use that jar instead):

```bash
java -cp target/sz-configtool-<version>.jar examples/Example.java ../../tests/fixtures/g2config_template.json
```

```java
String config = Files.readString(Path.of("g2config.json"));
config = SzConfigTool.addDataSource(config, "CUSTOMERS");
var attr = new SzConfigTool.AddAttributeOptions().internal("No").id(5000);
String row = SzConfigTool.addAttributeResult(config, "CUST_NAME", "NAME", "FULL_NAME", "NAME",
        attr);                                  // the CFG_ATTR row it adds, JSON text
config = SzConfigTool.addAttribute(config, "CUST_NAME", "NAME", "FULL_NAME", "NAME",
        attr);                                  // modified config (opaque string)
config = SzConfigTool.setFragment(config, "SNAME_SSTAB",       // clear ERFRAG_DESC
        new SzConfigTool.SetFragmentOptions().description(FieldUpdate.clear()));
```

API rules (see `bindings/CONTRACT.md`; generated from `api/manifest`):

* Method names are the manifest names in camelCase (`add_data_source` →
  `addDataSource`); args colliding with Java keywords get `Value`
  (`class` → `classValue`). Javadoc carries the manifest's doc, semantics,
  notes and reason codes.
* Required args (including manifest `required: true`) are positional;
  optional args go in the per-function `*Options` builder (an overload without
  it exists, except for the manifest's `requires_options` functions, which the
  library rejects without an optional arg: `addExpressionCall`,
  `addStandardizeCall`, `setFeature`). Unset = omitted, so the LIBRARY default
  applies.
* Tri-state args take `FieldUpdate<T>`: `leave()` / `clear()` / `set(v)`.
* Returns: every config-changing method (`config`, `config_and_json`) →
  the modified config `String`, so calls chain. A `config_and_json` function
  also has a companion `<name>Result` (same arguments and overloads) → the
  record as JSON text (e.g. `addAttributeResult` → the new `CFG_ATTR` row).
  The companion RE-RUNS the operation (a second run): call it with the SAME
  input config you gave the primary, never the config the primary returned
  (on that config `addAttributeResult` fails with `ALREADY_EXISTS`,
  `delete*Result` with `NOT_FOUND`, and `setGenericPlanResult` reports
  `wasCreated` = `"false"`).
  `json` → JSON text `String`; `tuple_names` → a `*Record` record with one
  JSON-text component per name (e.g. `setGenericPlanResult` →
  `SetGenericPlanRecord(planId, wasCreated)` = `("3", "true")`;
  `VerifyCompatibilityVersionRecord(currentVersion, matches)` =
  `("\"11\"", "true")`); `int` → `long`.
* `json`-typed args are JSON text (validated before the call).
* `int_or_str` args (call selectors) have two overloads: `long` = call id,
  `String` = feature code, e.g. `getStandardizeCall(config, 2L)` /
  `getStandardizeCall(config, "DOB")`. A numeric `String` is a feature code.
* The config is never parsed or re-serialized by the binding.
* Failures throw the checked `SzConfigToolException`: `getReasonCode()`
  (one of the 14 codes), `getKind()` (`SzConfigToolErrorKind`, the same code
  as an enum: `getKind().name().equals(getReasonCode())`), `getMessage()`,
  `getDetails()` (`sz-configtool.validation-errors/v1` JSON for
  `VALIDATION_ERRORS`). `null` arguments throw `NullPointerException`.
* Functions marked `not_implemented` have no typed method; they remain
  reachable via `NativeBridge.invoke`.

## Tests

`mvn test` runs, against the real native library: every case of
`api/manifest/generated/conformance.json` through the generated typed methods
(wire-only and stub steps through `NativeBridge.invoke`), plus naming,
optional/required/tri-state args, error mapping, byte-exact config opacity,
extraction (threads and concurrent JVMs, overrides) and an `nm` check that the
library exports only `Java_*` symbols (no `SzConfigTool_*`).

Coverage: `mvn -Pcoverage test` writes a JaCoCo report
(`target/site/jacoco/`); `packaging/coverage.sh rust java` (repository root)
runs it against an instrumented JNI library (see `packaging/README.md`, Coverage).

## Regenerating

`SzConfigTool.java`, `SzConfigToolErrorKind.java` and the test
`TypedDispatch.java` are generated by `tools/codegen/src/lang/java.rs`:

```bash
cargo run -p sz-configtool-codegen            # regenerate
cargo run -p sz-configtool-codegen -- --check # CI: fail if stale
```
