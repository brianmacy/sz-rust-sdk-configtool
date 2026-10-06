/**
 * sz-configtool-trpc — a tRPC router with one procedure per sz-configtool
 * function. Input = `{ config, ...camelCaseArgs }` (Zod-validated, strict);
 * output = the typed function's result. Config-changing functions are
 * mutations, read-only ones queries. Every request body carries the whole
 * configuration (~150–300KB): send queries with POST (see README).
 */
export { configToolRouter, type ConfigToolRouter } from "./generated/router.js";
export * as schemas from "./generated/schemas.js";
export {
  TRPC_CODE_BY_REASON,
  errorData,
  toTRPCError,
  type SzConfigToolErrorData,
  type TRPCErrorCode,
} from "./errors.js";
export { t } from "./trpc.js";
export { szCall } from "./sz-call.js";
