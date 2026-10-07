// Polyfill: netstandard2.0 lacks the marker type the compiler needs for
// `init` accessors (and therefore for records).
namespace System.Runtime.CompilerServices
{
    internal static class IsExternalInit
    {
    }
}
