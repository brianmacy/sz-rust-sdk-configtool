using System;
using System.Runtime.InteropServices;
using System.Text;

namespace Sz.ConfigTool.Native
{
    /// <summary>UTF-8 marshalling helpers (netstandard2.0 has no UTF-8 string marshaller).</summary>
    internal static class Utf8
    {
        // Strict: a lone surrogate in a .NET string, or invalid UTF-8 from the
        // library, throws instead of being silently replaced with U+FFFD.
        private static readonly UTF8Encoding Strict = new UTF8Encoding(false, true);

        /// <summary>NUL-terminated UTF-8 bytes of <paramref name="value"/>.</summary>
        /// <exception cref="SzConfigToolException">
        /// <c>INVALID_INPUT</c>: the string contains a NUL character (a C string
        /// cannot carry it; same as the C++ binding), or is not valid UTF-16 (a
        /// lone surrogate has no UTF-8 form; same as the Java/TS/Python bindings).
        /// </exception>
        public static byte[] ToNative(string value, string param)
        {
            if (value.IndexOf('\0') >= 0)
            {
                throw new SzConfigToolException(
                    "INVALID_INPUT", param + " contains a NUL character, which the C ABI cannot carry");
            }

            byte[] bytes;
            try
            {
                bytes = Strict.GetBytes(value);
            }
            catch (EncoderFallbackException)
            {
                throw new SzConfigToolException(
                    "INVALID_INPUT", param + " is not valid UTF-16 (lone surrogate); it has no UTF-8 form");
            }

            byte[] terminated = new byte[bytes.Length + 1];
            Buffer.BlockCopy(bytes, 0, terminated, 0, bytes.Length);
            return terminated;
        }

        /// <summary>Decode a NUL-terminated UTF-8 string, or null for a null pointer.</summary>
        public static string? FromNative(IntPtr ptr)
        {
            if (ptr == IntPtr.Zero)
            {
                return null;
            }

            int len = 0;
            while (Marshal.ReadByte(ptr, len) != 0)
            {
                len++;
            }

            byte[] bytes = new byte[len];
            Marshal.Copy(ptr, bytes, 0, len);
            return Strict.GetString(bytes);
        }
    }
}
