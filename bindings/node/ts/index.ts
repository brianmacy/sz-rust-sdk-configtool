/**
 * sz-configtool — typed Node.js binding for Senzing configuration JSON.
 *
 * Every function is `f(config, options?)`: `config` is the configuration JSON
 * text (opaque); the result is a new config string, JSON text, a
 * `{ config, json }` record, or a `<Fn>Result` of JSON texts for named
 * results. Errors are {@link SzConfigToolError}.
 */
export * from "./generated/functions.js";
export { REASON_CODES, type ReasonCode } from "./generated/reason-codes.js";
export {
  SzConfigToolError,
  isReasonCode,
  type ValidationDetails,
  type ValidationFailureDetail,
} from "./errors.js";
export {
  abiVersion,
  invoke,
  libraryVersion,
  type ConfigAndJson,
  type InvokeEnvelope,
  type ReturnKind,
  type WireArgs,
} from "./runtime.js";
export type { JsonValue } from "./json.js";
export { NATIVE_PATH_ENV, nativePath, platformTag } from "./native.js";
