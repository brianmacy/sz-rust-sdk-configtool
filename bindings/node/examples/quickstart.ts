/**
 * Quickstart: add a data source and an attribute, then list data sources.
 *
 *   node examples/quickstart.ts <path/to/g2config.json>
 */
import { readFileSync } from "node:fs";

import {
  addAttribute,
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
const attr = addAttribute(config, {
  attribute: "CUSTOMER_NAME",
  feature: "NAME",
  element: "FULL_NAME",
  class: "NAME",
});
config = attr.config;
console.log("new attribute:", attr.json);
console.log("data sources:", listDataSources(config));

try {
  addDataSource(config, { code: "CUSTOMERS" });
} catch (e) {
  if (!(e instanceof SzConfigToolError)) throw e;
  console.log(`duplicate rejected: ${e.code}`);
}
