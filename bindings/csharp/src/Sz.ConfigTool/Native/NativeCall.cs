using System;
using System.Collections.Generic;
using Sz.ConfigTool.Json;

namespace Sz.ConfigTool.Native
{
    /// <summary>
    /// The native seam: <c>SzConfigTool_invoke(name, config, args_json)</c>
    /// plus envelope decoding and last-error mapping. Every typed wrapper goes
    /// through <see cref="Invoke"/>.
    /// </summary>
    internal static class NativeCall
    {
        /// <summary>Call <paramref name="name"/> and decode the success envelope.</summary>
        /// <exception cref="SzConfigToolException">The library (or the wire layer) reported an error.</exception>
        public static InvokeResult Invoke(string name, string config, string? argsJson)
        {
            NativeLoader.EnsureInstalled();
            byte[] nameBytes = Utf8.ToNative(name ?? throw new ArgumentNullException(nameof(name)), nameof(name));
            byte[] configBytes = Utf8.ToNative(config ?? throw new ArgumentNullException("configJson"), "configJson");
            byte[] argsBytes = Utf8.ToNative(argsJson ?? "{}", nameof(argsJson));

            SzConfigToolResult r = NativeMethods.SzConfigTool_invoke(nameBytes, configBytes, argsBytes);
            if (r.ReturnCode != 0)
            {
                // Read the thread-local error BEFORE any other SzConfigTool_* call.
                SzConfigToolException error = LastError(r.ReturnCode);
                NativeMethods.SzConfigTool_free(r.Response);
                throw error;
            }

            string? envelope;
            try
            {
                envelope = Utf8.FromNative(r.Response);
            }
            finally
            {
                NativeMethods.SzConfigTool_free(r.Response);
            }

            return ParseEnvelope(envelope ?? throw Protocol("null response with returnCode 0"));
        }

        /// <summary>Invoke and require the envelope kind to be <paramref name="kind"/>.</summary>
        public static InvokeResult Expect(string kind, string name, string config, string argsJson)
        {
            InvokeResult r = Invoke(name, config, argsJson);
            if (r.Kind != kind)
            {
                throw Protocol($"{name}: envelope kind '{r.Kind}', expected '{kind}'");
            }

            return r;
        }

        /// <summary>A <c>config</c> function's modified configuration.</summary>
        public static string Config(string name, string config, string argsJson) =>
            Expect("config", name, config, argsJson).Config ?? throw Protocol($"{name}: envelope without config");

        /// <summary>A <c>json</c> function's result, as JSON text.</summary>
        public static string Json(string name, string config, string argsJson) =>
            Expect("json", name, config, argsJson).Result ?? throw Protocol($"{name}: envelope without result");

        /// <summary>A <c>config_and_json</c> function's configuration and record.</summary>
        public static ConfigAndJson ConfigAndJson(string name, string config, string argsJson)
        {
            InvokeResult r = Expect("config_and_json", name, config, argsJson);
            return new ConfigAndJson(
                r.Config ?? throw Protocol($"{name}: envelope without config"),
                r.Result ?? throw Protocol($"{name}: envelope without result"));
        }

        /// <summary>An <c>int</c> function's result.</summary>
        public static long Int(string name, string config, string argsJson)
        {
            string raw = Expect("int", name, config, argsJson).Result ?? throw Protocol($"{name}: envelope without result");
            return long.TryParse(raw, System.Globalization.NumberStyles.AllowLeadingSign, System.Globalization.CultureInfo.InvariantCulture, out long v)
                ? v
                : throw Protocol($"{name}: int result '{raw}' is not an integer");
        }

        /// <summary>A <c>unit</c> function (success has no value).</summary>
        public static void Unit(string name, string config, string argsJson) => Expect("unit", name, config, argsJson);

        /// <summary>
        /// Raw JSON text of each member <paramref name="fields"/> of the JSON
        /// object <paramref name="json"/> (parsed once), in order.
        /// </summary>
        public static string[] Members(string json, params string[] fields)
        {
            IReadOnlyDictionary<string, string> members;
            try
            {
                members = JsonScanner.ObjectMembers(json);
            }
            catch (FormatException e)
            {
                throw Protocol($"record is not a JSON object: {e.Message}");
            }

            var values = new string[fields.Length];
            for (int i = 0; i < fields.Length; i++)
            {
                values[i] = members.TryGetValue(fields[i], out string? raw) ? raw : throw Protocol($"record has no field '{fields[i]}'");
            }

            return values;
        }

        private static InvokeResult ParseEnvelope(string envelope)
        {
            try
            {
                IReadOnlyDictionary<string, string> m = JsonScanner.ObjectMembers(envelope);
                string kind = m.TryGetValue("kind", out string? k) ? JsonScanner.DecodeString(k) : throw new FormatException("no 'kind'");
                string? config = m.TryGetValue("config", out string? c) ? JsonScanner.DecodeString(c) : null;
                string? result = m.TryGetValue("result", out string? res) ? res : null;
                return new InvokeResult(kind, config, result);
            }
            catch (FormatException e)
            {
                throw Protocol($"malformed envelope: {e.Message}");
            }
        }

        private static SzConfigToolException LastError(long returnCode)
        {
            string? reason = Utf8.FromNative(NativeMethods.SzConfigTool_getLastErrorReasonCode());
            string message = Utf8.FromNative(NativeMethods.SzConfigTool_getLastError()) ?? $"SzConfigTool_invoke failed ({returnCode})";
            string? details = Utf8.FromNative(NativeMethods.SzConfigTool_getLastErrorDetails());
            return new SzConfigToolException(reason, message, details, returnCode);
        }

        // A broken wire contract between this binding and the native library.
        private static SzConfigToolException Protocol(string message) =>
            new SzConfigToolException("INTERNAL", "Sz.ConfigTool protocol error: " + message);
    }
}
