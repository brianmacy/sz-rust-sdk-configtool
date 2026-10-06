using Sz.ConfigTool.Native;

namespace Sz.ConfigTool
{
    /// <summary>
    /// Functional, stateless operations on a Senzing configuration JSON
    /// document: every method takes the configuration and returns the result
    /// (a new configuration, JSON, or both). Configurations are opaque strings,
    /// passed to and returned from the native library byte-exact.
    /// </summary>
    /// <remarks>
    /// The typed methods are generated from <c>api/manifest</c> (see
    /// <c>Generated/SzConfigTool.g.cs</c>). This class is thread-safe: errors are
    /// captured per thread immediately after each native call.
    /// </remarks>
    public static partial class SzConfigTool
    {
        /// <summary>
        /// Call any manifest function by its snake_case wire name (the raw seam;
        /// it also reaches functions the typed API omits, such as
        /// not-implemented placeholders).
        /// </summary>
        /// <param name="name">The manifest function name, e.g. <c>add_data_source</c>.</param>
        /// <param name="configJson">The configuration JSON (opaque).</param>
        /// <param name="argsJson">A JSON object keyed by the manifest argument names, or null for none.</param>
        /// <returns>The decoded envelope.</returns>
        /// <exception cref="SzConfigToolException">The call failed.</exception>
        public static InvokeResult Invoke(string name, string configJson, string? argsJson = null) =>
            NativeCall.Invoke(name, configJson, argsJson);

        /// <summary>The native library version, e.g. <c>4.4.0-1</c>.</summary>
        public static string LibraryVersion
        {
            get
            {
                NativeLoader.EnsureInstalled();
                return NativeCall.Require(Utf8.FromNative(NativeMethods.SzConfigTool_getLibraryVersion()), "SzConfigTool_getLibraryVersion returned NULL");
            }
        }

        /// <summary>The C ABI version of the loaded native library.</summary>
        public static int AbiVersion
        {
            get
            {
                NativeLoader.EnsureInstalled();
                return NativeMethods.SzConfigTool_getAbiVersion();
            }
        }
    }
}
