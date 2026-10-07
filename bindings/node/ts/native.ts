/**
 * Loads the napi-rs native seam (`<binaryName>.<platform>-<arch>[-abi].node`).
 *
 * The binary name comes from this package's `package.json` (`napi.binaryName`);
 * the environment variable {@link NATIVE_PATH_ENV} overrides the file path.
 */
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

/** The success envelope the native `invoke` returns. */
export interface NativeOutput {
  kind: string;
  config?: string | null;
  /** JSON text of the result value. */
  result?: string | null;
}

/** The native module's exports. */
export interface NativeBinding {
  invoke(name: string, config: string, argsJson?: string): NativeOutput;
  reasonCodes(): string[];
  libraryVersion(): string;
  abiVersion(): number;
}

/** Environment variable naming an explicit `.node` file to load. */
export const NATIVE_PATH_ENV = "SZ_CONFIGTOOL_NATIVE_PATH";

const PACKAGE_ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const requireNative = createRequire(import.meta.url);

/** napi-rs platform tag, e.g. `darwin-arm64`, `linux-x64-gnu`, `win32-x64-msvc`. */
export function platformTag(
  platform: string = process.platform,
  arch: string = process.arch,
): string {
  const abi = platform === "linux" ? "-gnu" : platform === "win32" ? "-msvc" : "";
  return `${platform}-${arch}${abi}`;
}

/** Absolute path of the native file this process would load. */
export function nativePath(): string {
  const override = process.env[NATIVE_PATH_ENV];
  if (override) return override;
  const pkg = requireNative(join(PACKAGE_ROOT, "package.json")) as {
    napi: { binaryName: string };
  };
  return join(PACKAGE_ROOT, `${pkg.napi.binaryName}.${platformTag()}.node`);
}

let cached: NativeBinding | undefined;

/** The loaded native module (loaded once). */
export function loadNative(): NativeBinding {
  if (cached) return cached;
  const path = nativePath();
  try {
    cached = requireNative(path) as NativeBinding;
  } catch (err) {
    throw new Error(
      `sz-configtool: cannot load native binding '${path}' (platform ${platformTag()}); ` +
        `build it with 'npm run build' or set ${NATIVE_PATH_ENV}`,
      { cause: err },
    );
  }
  return cached;
}
