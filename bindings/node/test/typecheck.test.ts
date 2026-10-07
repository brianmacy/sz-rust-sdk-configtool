/**
 * The whole package type-checks under tsconfig.test.json, including
 * test/compile/*.ts whose `@ts-expect-error` lines pin that wrong option
 * shapes do NOT compile (an unused directive is a tsc error).
 */
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { test } from "node:test";

import { packageDir } from "./helpers.ts";

test("tsc -p tsconfig.test.json passes (wrong option shapes fail to compile)", () => {
  const require = createRequire(join(packageDir, "package.json"));
  const tsc = join(dirname(require.resolve("typescript/package.json")), "bin", "tsc");
  const run = spawnSync(process.execPath, [tsc, "-p", join(packageDir, "tsconfig.test.json")], {
    encoding: "utf8",
    cwd: packageDir,
  });
  assert.equal(run.status, 0, `${run.stdout}${run.stderr}`);
});
