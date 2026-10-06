// GENERATED — do not edit. Source: api/manifest/*.yaml; regenerate with `cargo run -p sz-configtool-codegen`.

/** One wire reason code. */
export type ReasonCode = (typeof REASON_CODES)[number];

/** The complete wire error taxonomy (`project.yaml` `reason_codes`). */
export const REASON_CODES = [
  "JSON_PARSE",
  "NOT_FOUND",
  "NOT_ON_CALL",
  "NOT_IN_FEATURE",
  "ALREADY_EXISTS",
  "ALREADY_PRESENT",
  "INVALID_INPUT",
  "MISSING_SECTION",
  "INVALID_STRUCTURE",
  "MISSING_FIELD",
  "INVALID_CONFIG",
  "NOT_IMPLEMENTED",
  "VALIDATION_ERRORS",
  "INTERNAL",
] as const;
