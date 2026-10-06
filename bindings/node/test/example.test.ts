/** The README quickstart must run to completion on the real fixture. */
import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
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

test("examples/quickstart.ts without a config path prints usage and exits 2", () => {
  const run = spawnSync(
    process.execPath,
    ["--experimental-strip-types", join(packageDir, "examples", "quickstart.ts")],
    { encoding: "utf8" },
  );
  assert.equal(run.status, 2);
  assert.equal(run.stdout, "");
  assert.match(run.stderr, /^usage: node examples\/quickstart\.ts <g2config\.json>\n/);
});
