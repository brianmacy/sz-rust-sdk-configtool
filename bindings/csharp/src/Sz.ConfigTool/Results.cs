namespace Sz.ConfigTool
{
    /// <summary>
    /// Raw result of <see cref="SzConfigTool.Invoke"/>: the envelope fields.
    /// </summary>
    /// <param name="Kind">One of <c>config</c>, <c>json</c>, <c>config_and_json</c>, <c>int</c>, <c>unit</c>.</param>
    /// <param name="Config">The modified configuration (config kinds only), byte-exact.</param>
    /// <param name="Result">The result value as JSON text (json, record and int kinds), or null.</param>
    public record InvokeResult(string Kind, string? Config, string? Result);
}
