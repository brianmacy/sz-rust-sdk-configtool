/** The .node must expose only the napi seam: no `SzConfigTool_*` C ABI symbol. */
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import { test } from "node:test";

import { nativePath } from "../dist/native.js";

function nmArgs(platform: string): string[] | undefined {
  switch (platform) {
    case "darwin":
      return ["-gU"];
    case "linux":
      return ["-D", "--defined-only"];
    default:
      return undefined;
  }
}

const args = nmArgs(process.platform);

test("no SzConfigTool_* symbol is exported", { skip: args === undefined }, () => {
  const file = nativePath();
  assert.ok(existsSync(file), `missing ${file}; run npm run build`);
  const symbols = execFileSync("nm", [...(args ?? []), file], { encoding: "utf8" });
  assert.match(symbols, /napi_register_module_v1/);
  assert.doesNotMatch(symbols, /SzConfigTool_/);
});
