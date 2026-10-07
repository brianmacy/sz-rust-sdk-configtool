/** Run a handler, converting thrown errors to TRPCErrors. */
import { toTRPCError } from "./errors.js";

export function szCall<T>(fn: () => T): T {
  try {
    return fn();
  } catch (err) {
    throw toTRPCError(err);
  }
}
