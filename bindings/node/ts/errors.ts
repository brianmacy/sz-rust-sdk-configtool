/**
 * The one error class of this binding (see bindings/CONTRACT.md, Errors).
 *
 * Mirrors the community `@senzing/configtool` `SzConfigError` idiom
 * (`errorType`), but carries the stable wire reason code as `code`. The
 * reason code IS the error kind: `code`, `reasonCode`, `errorType` and `kind`
 * are the same string.
 */
import { REASON_CODES, type ReasonCode } from "./generated/reason-codes.js";

/** One field-level failure in {@link ValidationDetails}. */
export interface ValidationFailureDetail {
  field: string;
  reasonCode: string;
  offendingValue: string | null;
}

/**
 * Parsed `VALIDATION_ERRORS` details (`sz-configtool.validation-errors/v1`),
 * as returned by {@link SzConfigToolError.validationDetails}.
 */
export interface ValidationDetails {
  schema: string;
  failures: ValidationFailureDetail[];
}

/** A failure from the configuration library or the wire boundary. */
export class SzConfigToolError extends Error {
  /** The wire reason code (one of {@link REASON_CODES}). */
  readonly code: ReasonCode;
  /**
   * Validation details as JSON TEXT (only for `VALIDATION_ERRORS`), like every
   * other binding (bindings/CONTRACT.md, Errors). Use
   * {@link SzConfigToolError.validationDetails} for the parsed object.
   */
  readonly details: string | undefined;

  constructor(
    code: ReasonCode,
    message: string,
    details?: string,
    options?: { cause?: unknown },
  ) {
    super(message, options);
    this.name = "SzConfigToolError";
    this.code = code;
    this.details = details;
  }

  /** Alias of {@link SzConfigToolError.code} (community `SzConfigError` idiom). */
  get errorType(): ReasonCode {
    return this.code;
  }

  /** Alias of {@link SzConfigToolError.code}. */
  get reasonCode(): ReasonCode {
    return this.code;
  }

  /** The error kind: the reason code itself (same as {@link SzConfigToolError.code}). */
  get kind(): ReasonCode {
    return this.code;
  }

  /** {@link SzConfigToolError.details} parsed, or `undefined` when there are none. */
  validationDetails(): ValidationDetails | undefined {
    return this.details === undefined ? undefined : (JSON.parse(this.details) as ValidationDetails);
  }
}

/** True when `value` is one of the wire reason codes. */
export function isReasonCode(value: unknown): value is ReasonCode {
  return (REASON_CODES as readonly unknown[]).includes(value);
}

interface NativeErrorShape {
  message?: unknown;
  reasonCode?: unknown;
  details?: unknown;
}

function detailsText(details: unknown): string | undefined {
  return typeof details === "string" ? details : undefined;
}

/**
 * Convert anything thrown at the native boundary. Native errors keep their
 * reason code; anything else (a JS value napi cannot convert, an args object
 * `JSON.stringify` rejects) is caller input: `INVALID_INPUT`.
 */
export function toSzConfigToolError(err: unknown): SzConfigToolError {
  if (err instanceof SzConfigToolError) return err;
  const e = (typeof err === "object" && err !== null ? err : {}) as NativeErrorShape;
  const message = typeof e.message === "string" ? e.message : String(err);
  if (isReasonCode(e.reasonCode)) {
    return new SzConfigToolError(e.reasonCode, message, detailsText(e.details), { cause: err });
  }
  return new SzConfigToolError("INVALID_INPUT", message, undefined, { cause: err });
}
