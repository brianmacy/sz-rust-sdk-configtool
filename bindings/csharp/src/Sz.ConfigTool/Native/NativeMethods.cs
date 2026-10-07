using System;
using System.Runtime.InteropServices;

namespace Sz.ConfigTool.Native
{
    /// <summary>
    /// <c>SzConfigTool_result</c> from <c>libSzConfigTool.h</c>:
    /// <c>{ char *response; int64_t returnCode; }</c>.
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    internal struct SzConfigToolResult
    {
        public IntPtr Response;
        public long ReturnCode;
    }

    /// <summary>
    /// Hand-written P/Invoke declarations for the subset of the C ABI this
    /// binding uses (see <c>ffi/include/libSzConfigTool.h</c>). Strings are
    /// passed as NUL-terminated UTF-8 byte arrays (netstandard2.0 has no UTF-8
    /// string marshaller); returned pointers are read by <see cref="Utf8"/>.
    /// Call through <see cref="NativeCall"/>, which installs the resolver first.
    /// </summary>
    internal static class NativeMethods
    {
        /// <summary>
        /// Logical library name; the runtime maps it to libSzConfigTool.so,
        /// libSzConfigTool.dylib or SzConfigTool.dll.
        /// </summary>
        internal const string LibraryName = "SzConfigTool";

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern SzConfigToolResult SzConfigTool_invoke(byte[] name, byte[] configJson, byte[] argsJson);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void SzConfigTool_free(IntPtr ptr);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern IntPtr SzConfigTool_getLastError();

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern IntPtr SzConfigTool_getLastErrorReasonCode();

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern IntPtr SzConfigTool_getLastErrorDetails();

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern IntPtr SzConfigTool_getLibraryVersion();

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern int SzConfigTool_getAbiVersion();
    }
}
