/** The native loader: platform tag, default path, and the load failure. */
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { describe, test } from "node:test";

import { NATIVE_PATH_ENV, nativePath, platformTag } from "../dist/native.js";
import { packageDir } from "./helpers.ts";

describe("platformTag", () => {
  const cases: Array<[string, string, string]> = [
    ["linux", "x64", "linux-x64-gnu"],
    ["linux", "arm64", "linux-arm64-gnu"],
    ["win32", "x64", "win32-x64-msvc"],
    ["darwin", "arm64", "darwin-arm64"],
  ];
  for (const [platform, arch, want] of cases) {
    test(`${platform}/${arch} -> ${want}`, () => assert.equal(platformTag(platform, arch), want));
  }
  test("defaults to this process", () => {
    assert.equal(platformTag(), platformTag(process.platform, process.arch));
  });
});

describe("nativePath", () => {
  test(`without ${NATIVE_PATH_ENV}: <binaryName>.<platform tag>.node in the package`, () => {
    const pkg = JSON.parse(readFileSync(join(packageDir, "package.json"), "utf8")) as {
      napi: { binaryName: string };
    };
    const saved = process.env[NATIVE_PATH_ENV];
    delete process.env[NATIVE_PATH_ENV];
    try {
      assert.equal(nativePath(), join(packageDir, `${pkg.napi.binaryName}.${platformTag()}.node`));
    } finally {
      if (saved !== undefined) process.env[NATIVE_PATH_ENV] = saved;
    }
  });

  test(`${NATIVE_PATH_ENV} overrides the path`, () => {
    const saved = process.env[NATIVE_PATH_ENV];
    process.env[NATIVE_PATH_ENV] = "/explicit/override.node";
    try {
      assert.equal(nativePath(), "/explicit/override.node");
    } finally {
      if (saved === undefined) delete process.env[NATIVE_PATH_ENV];
      else process.env[NATIVE_PATH_ENV] = saved;
    }
  });
});

describe("loadNative", () => {
  // A fresh process: the loaded module is cached for the life of a process.
  test("a missing native file is a descriptive Error with the load error as cause", () => {
    const missing = join(packageDir, "dist", "no-such-native.node");
    const script =
      `import { loadNative } from ${JSON.stringify(pathToFileURL(join(packageDir, "dist", "native.js")).href)};\n` +
      "try { loadNative(); console.log('loaded'); } catch (e) {\n" +
      "  console.log(JSON.stringify({ message: e.message, cause: e.cause instanceof Error }));\n" +
      "}\n";
    const run = spawnSync(process.execPath, ["--input-type=module", "-e", script], {
      encoding: "utf8",
      env: { ...process.env, [NATIVE_PATH_ENV]: missing },
    });
    assert.equal(run.status, 0, run.stderr);
    const out = JSON.parse(run.stdout) as { message: string; cause: boolean };
    assert.equal(
      out.message,
      `sz-configtool: cannot load native binding '${missing}' (platform ${platformTag()}); ` +
        `build it with 'npm run build' or set ${NATIVE_PATH_ENV}`,
    );
    assert.equal(out.cause, true);
  });
});
