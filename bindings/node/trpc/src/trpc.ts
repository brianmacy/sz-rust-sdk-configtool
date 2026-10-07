/**
 * The shared tRPC instance. superjson is the transformer (as in
 * `@senzing/trpc`), and the error formatter adds `data.szConfigTool` with the
 * reason code, kind and validation details.
 */
import { initTRPC } from "@trpc/server";
import superjson from "superjson";

import { errorData } from "./errors.js";

export const t = initTRPC.create({
  transformer: superjson,
  errorFormatter({ shape, error }) {
    return { ...shape, data: { ...shape.data, szConfigTool: errorData(error) } };
  },
});
