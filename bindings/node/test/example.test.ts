/** The README quickstart must run to completion on the real fixture. */
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { join } from "node:path";
import { test } from "node:test";

import { conformance, packageDir } from "./helpers.ts";
import { WORKSPACE_ROOT } from "./generated/paths.ts";

test("examples/quickstart.ts succeeds", () => {
  const fixturePath = join(packageDir, WORKSPACE_ROOT, conformance.fixture);
  const out = execFileSync(
    process.execPath,
    ["--experimental-strip-types", join(packageDir, "examples", "quickstart.ts"), fixturePath],
    { encoding: "utf8" },
  );
  assert.match(out, /"dataSource":"CUSTOMERS"/);
  assert.match(out, /duplicate rejected: ALREADY_EXISTS\n/);
});
