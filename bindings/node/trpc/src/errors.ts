/**
 * Maps `SzConfigToolError` reason codes to tRPC error codes (the
 * `@senzing/trpc` `toTRPCError` idiom), keeping the original error as `cause`.
 */
import { TRPCError } from "@trpc/server";
import { SzConfigToolError, type ReasonCode } from "sz-configtool";

export type TRPCErrorCode = ConstructorParameters<typeof TRPCError>[0]["code"];

/**
 * Exhaustive over the 14 wire reason codes (adding one is a compile error
 * here): caller mistakes are BAD_REQUEST, missing references NOT_FOUND,
 * duplicates CONFLICT, a structurally unusable config UNPROCESSABLE_CONTENT.
 */
export const TRPC_CODE_BY_REASON: Readonly<Record<ReasonCode, TRPCErrorCode>> = {
  JSON_PARSE: "BAD_REQUEST",
  NOT_FOUND: "NOT_FOUND",
  NOT_ON_CALL: "NOT_FOUND",
  NOT_IN_FEATURE: "NOT_FOUND",
  ALREADY_EXISTS: "CONFLICT",
  ALREADY_PRESENT: "CONFLICT",
  INVALID_INPUT: "BAD_REQUEST",
  MISSING_SECTION: "UNPROCESSABLE_CONTENT",
  INVALID_STRUCTURE: "UNPROCESSABLE_CONTENT",
  MISSING_FIELD: "BAD_REQUEST",
  INVALID_CONFIG: "UNPROCESSABLE_CONTENT",
  NOT_IMPLEMENTED: "NOT_IMPLEMENTED",
  VALIDATION_ERRORS: "BAD_REQUEST",
  INTERNAL: "INTERNAL_SERVER_ERROR",
};

/** Wrap any thrown value in a TRPCError (SzConfigToolError → mapped code). */
export function toTRPCError(err: unknown): TRPCError {
  if (err instanceof TRPCError) return err;
  if (err instanceof SzConfigToolError) {
    return new TRPCError({ code: TRPC_CODE_BY_REASON[err.code], message: err.message, cause: err });
  }
  const message = err instanceof Error ? err.message : String(err);
  return new TRPCError({ code: "INTERNAL_SERVER_ERROR", message, cause: err });
}

/** The `data.szConfigTool` payload clients receive for a library error. */
export interface SzConfigToolErrorData {
  reasonCode: ReasonCode;
  /** The error kind: the reason code itself. */
  kind: ReasonCode;
  /** `VALIDATION_ERRORS` details as JSON text (`sz-configtool.validation-errors/v1`), else null. */
  details: string | null;
}

/** Extract {@link SzConfigToolErrorData} from a TRPCError's cause, if any. */
export function errorData(error: TRPCError): SzConfigToolErrorData | null {
  const cause = error.cause;
  if (!(cause instanceof SzConfigToolError)) return null;
  return { reasonCode: cause.code, kind: cause.kind, details: cause.details ?? null };
}
