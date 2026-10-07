# sz-configtool (Node.js / TypeScript)

Typed Node.js binding for manipulating Senzing configuration JSON
(`g2config.json`), plus a tRPC router over it (`trpc/`).

> **Unofficial.** This is an unofficial library: Senzing does not publicly
> document most configuration functions and parameters. Use it only with
> Senzing-provided guidance on what to change and when.

```
ts/generated/functions.ts   typed camelCase functions (GENERATED from api/manifest)
        |  invoke(name, config, argsJson)
sz-configtool.<platform>.node   napi-rs v3 seam (src/lib.rs) -> sz-configtool-api::invoke
        |
sz_configtool_lib (pure Rust)
```

The `.node` exports only the napi module entry point: no `SzConfigTool_*` C
symbol (asserted by `test/exports.test.ts`). It depends on
`sz-configtool-api`, never on the C FFI crate.

## Requirements

| Tool | Version |
|---|---|
| Rust | 1.88+ (workspace `rust-version`) |
| Node.js | 20+ to use the compiled package (`dist/`, what `engines` declares). 22.6+ to run `npm test` (the tests are `.ts` run with `--experimental-strip-types`, which Node 20 lacks) and the `.ts` example directly (type stripping is on by default from 22.18; 22.6-22.17 need `--experimental-strip-types`); on Node 20, use the compiled package only |
| npm | 10+ |

Prebuilt targets (`package.json` `napi.targets`): `aarch64-apple-darwin`,
`x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`,
`x86_64-pc-windows-msvc`. The file is named
`sz-configtool.<platform>-<arch>[-gnu|-msvc].node`
(e.g. `sz-configtool.linux-x64-gnu.node`).

## Build (single command)

```bash
cd bindings/node
npm ci
npm run build          # napi build --platform --release, then tsc -> dist/
npm test               # conformance + API + error + export tests vs the real .node (Node 22.6+)
npm run typecheck      # tsc --noEmit over sources, tests and examples
```

Coverage (Node's built-in `--experimental-test-coverage`, source-mapped to
the TypeScript, incl. `trpc/`): `packaging/coverage.sh rust node` from the
repository root (see `packaging/README.md`, Coverage).

Regenerate the typed wrappers, Zod schemas and router after a manifest change
(from the workspace root): `cargo run -p sz-configtool-codegen`
(`-- --check` fails when they are stale).

## Install from a GitHub Release

Nothing is published to npm. From the repository's GitHub Release, either:

1. **Tarball** — download the `sz-configtool-node-<version>-<os>-<arch>.tgz`
   built for your platform (package `sz-configtool`; it contains `dist/` and
   that platform's `.node`) and install it:

   ```bash
   npm install ./sz-configtool-node-4.4.0-2-linux-x64.tgz
   ```

   The tRPC router (`sz-configtool-trpc-<version>.tgz`, platform independent)
   declares `sz-configtool` as an exact-version **peer dependency**, so install
   it together with the platform tarball of the same version:

   ```bash
   npm install ./sz-configtool-node-4.4.0-2-linux-x64.tgz ./sz-configtool-trpc-4.4.0-2.tgz
   ```

2. **Bare `.node`** — install a tarball (any platform) or a source build, then
   point the loader at the downloaded binary:

   ```bash
   export SZ_CONFIGTOOL_NATIVE_PATH=/opt/senzing/sz-configtool.linux-x64-gnu.node
   ```

To produce a tarball yourself: `npm run build && npm pack`.

## Usage

```ts
import { readFileSync } from "node:fs";
import {
  addAttribute, addAttributeResult, addDataSource, listDataSources, SzConfigToolError,
} from "sz-configtool";

let config = readFileSync(process.argv[2]!, "utf8");
config = addDataSource(config, { code: "CUSTOMERS" });   // every config-changing
                                                         // function returns the config
const attribute = { attribute: "CUSTOMER_NAME", feature: "NAME", element: "FULL_NAME", class: "NAME" };
console.log(JSON.parse(addAttributeResult(config, attribute))); // the CFG_ATTR row it would add
config = addAttribute(config, attribute);                // new configuration
console.log(listDataSources(config));                    // JSON text

try {
  addDataSource(config, { code: "CUSTOMERS" });
} catch (e) {
  if (e instanceof SzConfigToolError) console.log(e.code); // ALREADY_EXISTS
}
```

Runnable: `node examples/quickstart.ts ../../tests/fixtures/g2config_template.json`
(after `npm run build`; executed by `test/example.test.ts`). Running a `.ts`
file directly needs Node 22.18+ (type stripping on by default; 22.6-22.17:
`node --experimental-strip-types ...`); on Node 20 compile it first.

### Conventions (`bindings/CONTRACT.md`)

| Topic | Rule |
|---|---|
| Names | manifest snake_case split on `_` → camelCase (`get_ftype_id` → `getFtypeId`); options properties likewise. |
| Shape | `fn(config, options)` — `options` omitted when there are no args, defaults to `{}` when all are optional. |
| Strict options | `options` must be a plain object (a string, number, boolean, array or `null` is `INVALID_INPUT` naming the function and the received type; a string gets a hint: `getSearchProfile(cfg, "SEARCH")` → `... got string; pass { code: "SEARCH" }`). Omitting it is allowed only when every option is optional. An unknown key is `INVALID_INPUT` before the native call, naming the key and the closest option (`unknown option 'candidate' for addSearchProfile; did you mean 'candidates'?`) or listing the valid ones; an option set to `undefined` is absent, not unknown. |
| Structured (`json`) options | Typed from the manifest `json_type` (`AddSearchProfileOptions.elements: ReadonlyArray<{ feature: string; flag: "Yes" \| "No" \| "Y" \| "N" }>`; `addFeature`'s `elementList`, `addExpressionCall`'s `elementList`, `addFragment`'s `fragmentConfig`, `addRule`'s `ruleConfig`), so a wrong shape fails `tsc`; at run time the same shape is checked before the native call: unknown keys inside an entry, wrong types and enum values are `INVALID_INPUT` naming the path (`addSearchProfile: elements[1].flag must be one of Yes, No, Y, N (got "Maybe")`), a missing required key is `MISSING_FIELD`. Free-form values (`setSetting` `value`, `setSystemParameter` `parameterValue`, `addConfigSectionField` `fieldValue`) stay `JsonValue`. The runtime check follows the declared type exactly: e.g. `flag` must be one of the four spellings (the library also accepts other casings via `invoke`). |
| `config` | Opaque string, passed and returned byte-exact (never parsed). A non-string `config` (e.g. a companion's record passed back) is `INVALID_INPUT` before the native call. |
| Optional | `name?: T`; `undefined` = absent (the library applies its documented default). |
| Tri-state | `name?: T \| null`: `undefined` = leave, `null` = clear, value = set. |
| `int` | `number \| bigint`. A `number` must be a safe integer (`\|n\| <= Number.MAX_SAFE_INTEGER`, else `INVALID_INPUT`: it was already rounded); pass a `bigint` for the full i64 range (`-(2n ** 63n)` .. `2n ** 63n - 1n`), written to the wire as its exact digits. A `bigint` outside i64 is `INVALID_INPUT`. `bigint` is also accepted anywhere inside a `json` arg. |
| `int_or_str` | `number \| bigint \| string` — a call selector: an integer call id or a feature code (`getComparisonCall(cfg, { call: 1 })` / `{ call: "NAME" }`). A non-integer number is `INVALID_INPUT`. |
| `required: true` | A required property even though Rust takes `Option` (absent → `MISSING_FIELD` via `invoke`). |
| Returns | Every config-changing function (`config`, `config_and_json`) → the new config `string`, so calls chain. A `config_and_json` function also has a companion `<fn>Result` (same options, same operation) → the record as JSON **text** (e.g. `addAttributeResult` → the new `CFG_ATTR` row); with `tuple_names` → `<Fn>Record` whose camelCase fields are each member's JSON **text** (e.g. `setGenericPlanResult` → `SetGenericPlanRecord { planId: "3", wasCreated: "true" }`; strings keep their quotes: `currentVersion: "\"11\""`). `json` → JSON **text** (`JSON.parse` it); `int` → `number`; `unit` → `void`. |
| Skipped | `status: not_implemented` placeholders; still callable via `invoke(name, config, args)`. |
| Docs | TSDoc on every function/option from the manifest `doc`, `semantics`, `notes`, `errors`. |

`invoke(name, config, args)` is the raw seam: snake_case `args` (object or
JSON text), returns `{ kind, config?, result? }` with `result` as JSON text.

### Errors

Every failure is `SzConfigToolError extends Error`:

| Property | Meaning |
|---|---|
| `code` (aliases `errorType`, `reasonCode`, `kind`) | One of the 14 `REASON_CODES` (`NOT_FOUND`, `ALREADY_EXISTS`, `VALIDATION_ERRORS`, …, `INTERNAL`). The reason code IS the error kind. |
| `details` | For `VALIDATION_ERRORS`: JSON **text** (like every binding) with schema `sz-configtool.validation-errors/v1`; `err.validationDetails()` returns it parsed as `ValidationDetails { schema, failures: [{ field, reasonCode, offendingValue }] }`. `undefined` otherwise. |
| `cause` | The native error. |

Errors name the **JS option**, not the wire field: a native `MISSING_FIELD` /
`INVALID_INPUT` message (and `VALIDATION_ERRORS` `details` fields) naming a
wire argument is translated, with the wire name in parentheses once for
traceability (`Missing required field: genericPlan (generic_plan)`;
`scoringCap (scoring_cap), candidateCap (candidate_cap), sendToRedo
(send_to_redo)`); the untranslated error is the `cause`. `invoke` is the raw
seam and keeps wire names.

A Rust panic becomes `INTERNAL`. A JS value the native layer cannot convert
(e.g. a number where a string is expected) or args that cannot be serialized
(e.g. a circular object) becomes `INVALID_INPUT`.

## tRPC router (`trpc/`)

`sz-configtool-trpc` exposes one procedure per typed function. Input is
`{ config, ...camelCaseArgs }`, validated by a strict Zod schema generated from
the manifest arg types (`str`→`z.string()`, `int`→`z.union([z.int(), z.bigint()])` (a `bigint` only reaches an in-process
caller: an HTTP JSON body carries numbers, so over HTTP ids are limited to safe
integers), `bool`,
`json`→ the typed `json_type` structure (`z.strictObject`s that reject
unknown keys, `z.enum`, `z.array`, `z.union`; `any` → `z.json()`), `str_list`→`z.array(z.string())`,
`int_or_str`→`z.union([z.int(), z.bigint(), z.string()])`; tri-state →
`.nullable().optional()`). Output is the typed function's result, with the
same types (config text, JSON text, `<Fn>Record` JSON texts). superjson is the
transformer.

* Config-changing functions are **mutations** (they return the config text);
  read-only ones and the `<fn>Result` companions (the record only) are **queries**.
* **Every request carries the whole configuration (~150–300KB)**. Send queries
  with POST (`methodOverride: "POST"` on the client, `allowMethodOverride: true`
  on the server) — a GET URL cannot hold it — and size any proxy/body limits
  accordingly.
* Errors become `TRPCError` (`TRPC_CODE_BY_REASON`): `NOT_FOUND`/`NOT_ON_CALL`/
  `NOT_IN_FEATURE` → `NOT_FOUND`; `ALREADY_EXISTS`/`ALREADY_PRESENT` →
  `CONFLICT`; `INVALID_INPUT`/`MISSING_FIELD`/`JSON_PARSE`/`VALIDATION_ERRORS` →
  `BAD_REQUEST`; `MISSING_SECTION`/`INVALID_STRUCTURE`/`INVALID_CONFIG` →
  `UNPROCESSABLE_CONTENT`; `NOT_IMPLEMENTED` → `NOT_IMPLEMENTED`; `INTERNAL` →
  `INTERNAL_SERVER_ERROR`. Clients get `error.data.szConfigTool =
  { reasonCode, kind, details }` (`kind` = `reasonCode`; `details` = JSON text
  or `null`).

```ts
// server
import { createHTTPServer } from "@trpc/server/adapters/standalone";
import { configToolRouter } from "sz-configtool-trpc";
createHTTPServer({ router: configToolRouter, allowMethodOverride: true }).listen(3000);

// client
import { createTRPCClient, httpLink } from "@trpc/client";
import superjson from "superjson";
import type { ConfigToolRouter } from "sz-configtool-trpc";
const sz = createTRPCClient<ConfigToolRouter>({
  links: [httpLink({ url: "http://localhost:3000", transformer: superjson, methodOverride: "POST" })],
});
const next = await sz.addDataSource.mutate({ config, code: "CUSTOMERS" });
const list = await sz.listDataSources.query({ config: next });
```

Build and test (after building the parent package):

```bash
cd bindings/node/trpc
npm ci && npm run build && npm test && npm run typecheck
```

Tests use a real in-process caller (`configToolRouter.createCaller({})`) and a
real HTTP round trip, both over the built `.node`.

## Relationship to `@senzing/configtool` (sz-napi)

The community `@senzing/configtool` package (in `sz-napi`) is a separate,
hand-written napi binding that pins an old `sz_configtool_lib` git revision
(pre-v0.4.0). This generated `sz-configtool` package supersedes it: it covers
every manifest function of the current library and is released with it. They
are not drop-in compatible: the error class is `SzConfigToolError` here
(`SzConfigError` there) and `code` / `kind` carry the wire reason code.

## Precedent

Idioms follow the community `sz-napi` packages where the contract allows:
napi-rs v3 with per-target `.node` files and the same `napi.targets`
(`@senzing/configtool`); options objects and camelCase; an `errorType`
property (`SzConfigError`); superjson, Zod inputs, a `toTRPCError` mapper and
a `szCall` wrapper (`@senzing/trpc`). Differences required by the contract:
one generated surface over a single `invoke` seam, `code` = wire reason code,
`json` results returned as JSON text.
