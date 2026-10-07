/**
 * Quickstart: add a data source and an attribute, then list data sources.
 *
 *   node examples/quickstart.ts <path/to/g2config.json>
 */
import { readFileSync } from "node:fs";

import {
  addAttribute,
  addAttributeResult,
  addDataSource,
  listDataSources,
  SzConfigToolError,
} from "../dist/index.js";

const path = process.argv[2];
if (!path) {
  console.error("usage: node examples/quickstart.ts <g2config.json>");
  process.exit(2);
}

let config = readFileSync(path, "utf8");
config = addDataSource(config, { code: "CUSTOMERS" });
const attribute = {
  attribute: "CUSTOMER_NAME",
  feature: "NAME",
  element: "FULL_NAME",
  class: "NAME",
};
// Config-changing functions return the new config; the companion
// `<name>Result` (same options) returns the row the operation creates.
console.log("new attribute:", addAttributeResult(config, attribute));
config = addAttribute(config, attribute);
console.log("data sources:", listDataSources(config));

try {
  addDataSource(config, { code: "CUSTOMERS" });
} catch (e) {
  if (!(e instanceof SzConfigToolError)) throw e;
  console.log(`duplicate rejected: ${e.code}`);
}
