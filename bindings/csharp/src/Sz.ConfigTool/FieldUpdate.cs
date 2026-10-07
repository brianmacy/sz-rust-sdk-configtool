using System;
using System.Collections.Generic;

namespace Sz.ConfigTool
{
    /// <summary>The three states of a <see cref="FieldUpdate{T}"/>.</summary>
    public enum FieldUpdateState
    {
        /// <summary>Leave the field unchanged (the argument is omitted).</summary>
        Leave = 0,

        /// <summary>Clear the field (the argument is sent as JSON <c>null</c>).</summary>
        Clear = 1,

        /// <summary>Set the field to <see cref="FieldUpdate{T}.Value"/>.</summary>
        Set = 2,
    }

    /// <summary>
    /// A tri-state argument: leave the field unchanged, clear it, or set it.
    /// <c>default(FieldUpdate&lt;T&gt;)</c> is <see cref="Leave"/>, so an
    /// omitted optional parameter leaves the field as it is.
    /// </summary>
    /// <typeparam name="T">The value type (<see cref="string"/> or <see cref="long"/>).</typeparam>
    public readonly struct FieldUpdate<T> : IEquatable<FieldUpdate<T>>
    {
        private readonly T _value;

        private FieldUpdate(FieldUpdateState state, T value)
        {
            State = state;
            _value = value;
        }

        /// <summary>Leave the field unchanged (same as <c>default</c>).</summary>
        public static FieldUpdate<T> Leave => default;

        /// <summary>Clear the field.</summary>
        public static FieldUpdate<T> Clear => new FieldUpdate<T>(FieldUpdateState.Clear, default!);

        /// <summary>Set the field to <paramref name="value"/>.</summary>
        /// <exception cref="ArgumentNullException"><paramref name="value"/> is null (use <see cref="Clear"/>).</exception>
        public static FieldUpdate<T> Set(T value)
        {
            if (value is null)
            {
                throw new ArgumentNullException(nameof(value), "Use FieldUpdate<T>.Clear to clear a field.");
            }

            return new FieldUpdate<T>(FieldUpdateState.Set, value);
        }

        /// <summary>Shorthand for <see cref="Set(T)"/>.</summary>
        public static implicit operator FieldUpdate<T>(T value) => Set(value);

        /// <summary>Which of leave / clear / set this is.</summary>
        public FieldUpdateState State { get; }

        /// <summary>True when the field is left unchanged.</summary>
        public bool IsLeave => State == FieldUpdateState.Leave;

        /// <summary>True when the field is cleared.</summary>
        public bool IsClear => State == FieldUpdateState.Clear;

        /// <summary>True when the field is set.</summary>
        public bool IsSet => State == FieldUpdateState.Set;

        /// <summary>The value to set.</summary>
        /// <exception cref="InvalidOperationException">The state is not <see cref="FieldUpdateState.Set"/>.</exception>
        public T Value => IsSet ? _value : throw new InvalidOperationException($"FieldUpdate is {State}, not Set.");

        /// <inheritdoc />
        public bool Equals(FieldUpdate<T> other) =>
            State == other.State && EqualityComparer<T>.Default.Equals(_value, other._value);

        /// <inheritdoc />
        public override bool Equals(object? obj) => obj is FieldUpdate<T> other && Equals(other);

        /// <inheritdoc />
        public override int GetHashCode() =>
            ((int)State * 397) ^ (IsSet ? EqualityComparer<T>.Default.GetHashCode(_value!) : 0);

        /// <inheritdoc />
        public override string ToString() => IsSet ? $"Set({_value})" : State.ToString();

        /// <summary>Equality.</summary>
        public static bool operator ==(FieldUpdate<T> left, FieldUpdate<T> right) => left.Equals(right);

        /// <summary>Inequality.</summary>
        public static bool operator !=(FieldUpdate<T> left, FieldUpdate<T> right) => !left.Equals(right);
    }
}
