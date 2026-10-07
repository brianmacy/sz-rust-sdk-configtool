namespace Sz.ConfigTool
{
    /// <summary>
    /// Result of a <c>config_and_json</c> function: the modified configuration
    /// and the JSON record describing the change (e.g. the new row).
    /// </summary>
    /// <param name="Config">The modified configuration JSON (opaque, byte-exact from the library).</param>
    /// <param name="Json">The record, as JSON text.</param>
    public record ConfigAndJson(string Config, string Json);

    /// <summary>
    /// Raw result of <see cref="SzConfigTool.Invoke"/>: the envelope fields.
    /// </summary>
    /// <param name="Kind">One of <c>config</c>, <c>json</c>, <c>config_and_json</c>, <c>int</c>, <c>unit</c>.</param>
    /// <param name="Config">The modified configuration (config kinds only), byte-exact.</param>
    /// <param name="Result">The result value as JSON text (json, record and int kinds), or null.</param>
    public record InvokeResult(string Kind, string? Config, string? Result);
}
