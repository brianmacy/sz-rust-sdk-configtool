/**
 * sz-configtool — typed Node.js binding for Senzing configuration JSON.
 *
 * Every function is `f(config, options?)`: `config` is the configuration JSON
 * text (opaque). A config-changing function returns the new config string
 * (so calls chain); when it also produces a record (e.g. the new row), its
 * companion `<name>Result` (same options) returns that record as JSON text,
 * or a `<Fn>Record` of JSON texts for named results. Other functions return
 * JSON text. Errors are {@link SzConfigToolError}.
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
  type InvokeEnvelope,
  type ReturnKind,
  type WireArgs,
} from "./runtime.js";
export type { JsonValue } from "./json.js";
export { NATIVE_PATH_ENV, nativePath, platformTag } from "./native.js";
