using System;

namespace Sz.ConfigTool
{
    /// <summary>
    /// The single error type raised by <see cref="SzConfigTool"/>. Discriminate
    /// on <see cref="Kind"/> (or the stable <see cref="ReasonCode"/> string);
    /// <see cref="Details"/> carries the structured validation failures when
    /// <see cref="Kind"/> is <see cref="SzConfigToolErrorKind.ValidationErrors"/>.
    /// </summary>
    public sealed class SzConfigToolException : Exception
    {
        private const string InternalReasonCode = "INTERNAL";

        /// <summary>Create an exception.</summary>
        /// <param name="reasonCode">The stable reason code; null (none reported) becomes <c>INTERNAL</c>.</param>
        /// <param name="message">The library's message.</param>
        /// <param name="details">Structured details JSON, or null.</param>
        /// <param name="returnCode">The native return code (diagnostic only; not the taxonomy).</param>
        public SzConfigToolException(string? reasonCode, string message, string? details = null, long returnCode = 0)
            : base(message)
        {
            ReasonCode = reasonCode ?? InternalReasonCode;
            Kind = SzConfigToolErrorKinds.FromReasonCode(ReasonCode);
            Details = details;
            ReturnCode = returnCode;
        }

        /// <summary>
        /// The error kind: it has the reason code's identity (one enum constant
        /// per wire reason code, so <c>SzConfigToolErrorKinds.ToReasonCode(Kind) == ReasonCode</c>;
        /// <see cref="SzConfigToolErrorKind.Unknown"/> only when an unrecognized code was reported).
        /// </summary>
        public SzConfigToolErrorKind Kind { get; }

        /// <summary>
        /// The stable reason code (e.g. <c>NOT_FOUND</c>); never null — <c>INTERNAL</c>
        /// when the native layer reported none (same as Java's <c>getReasonCode()</c>).
        /// </summary>
        public string ReasonCode { get; }

        /// <summary>
        /// Versioned details JSON (<c>sz-configtool.validation-errors/v1</c>) for
        /// validation errors; null otherwise.
        /// </summary>
        public string? Details { get; }

        /// <summary>The native return code (0 for errors raised by this binding itself).</summary>
        public long ReturnCode { get; }
    }
}
