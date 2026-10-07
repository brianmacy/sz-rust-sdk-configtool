/**
 * Any JSON value (what a manifest `json` argument accepts). A `bigint` is
 * written as its exact digits (a JSON integer beyond `Number.MAX_SAFE_INTEGER`).
 */
export type JsonValue =
  | string
  | number
  | bigint
  | boolean
  | null
  | JsonValue[]
  | { [key: string]: JsonValue };
