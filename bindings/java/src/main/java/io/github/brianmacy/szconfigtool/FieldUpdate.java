package io.github.brianmacy.szconfigtool;

import java.util.Objects;

/**
 * A tri-state update of one field: {@link #leave() leave} it unchanged,
 * {@link #clear() clear} it, or {@link #set(Object) set} it to a value.
 *
 * <p>On the wire (see {@code bindings/CONTRACT.md}) leave omits the argument,
 * clear sends {@code null} and set sends the value.
 *
 * @param <T> the field's value type ({@code String} or {@code Long})
 */
public final class FieldUpdate<T> {
    /** The three update states. */
    public enum State {
        /** Leave the field unchanged (argument omitted). */
        LEAVE,
        /** Clear the field (argument sent as {@code null}). */
        CLEAR,
        /** Set the field to {@link FieldUpdate#getValue()}. */
        SET
    }

    private static final FieldUpdate<?> LEAVE = new FieldUpdate<>(State.LEAVE, null);
    private static final FieldUpdate<?> CLEAR = new FieldUpdate<>(State.CLEAR, null);

    private final State state;
    private final T value;

    private FieldUpdate(State state, T value) {
        this.state = state;
        this.value = value;
    }

    /**
     * Leave the field unchanged.
     *
     * @param <T> value type
     * @return the shared LEAVE instance
     */
    @SuppressWarnings("unchecked")
    public static <T> FieldUpdate<T> leave() {
        return (FieldUpdate<T>) LEAVE;
    }

    /**
     * Clear the field.
     *
     * @param <T> value type
     * @return the shared CLEAR instance
     */
    @SuppressWarnings("unchecked")
    public static <T> FieldUpdate<T> clear() {
        return (FieldUpdate<T>) CLEAR;
    }

    /**
     * Set the field.
     *
     * @param <T> value type
     * @param value the new value (non-null; use {@link #clear()} to clear)
     * @return a SET update
     * @throws NullPointerException if {@code value} is null
     */
    public static <T> FieldUpdate<T> set(T value) {
        return new FieldUpdate<>(State.SET, Objects.requireNonNull(value, "value"));
    }

    /** @return this update's state */
    public State getState() {
        return state;
    }

    /** @return true for LEAVE */
    public boolean isLeave() {
        return state == State.LEAVE;
    }

    /** @return true for CLEAR */
    public boolean isClear() {
        return state == State.CLEAR;
    }

    /** @return true for SET */
    public boolean isSet() {
        return state == State.SET;
    }

    /**
     * The value of a SET update.
     *
     * @return the value
     * @throws IllegalStateException if this is not a SET update
     */
    public T getValue() {
        if (state != State.SET) {
            throw new IllegalStateException("FieldUpdate is " + state + ", not SET");
        }
        return value;
    }

    @Override
    public boolean equals(Object o) {
        return o instanceof FieldUpdate<?> other && state == other.state
                && Objects.equals(value, other.value);
    }

    @Override
    public int hashCode() {
        return Objects.hash(state, value);
    }

    @Override
    public String toString() {
        return state == State.SET ? "FieldUpdate.set(" + value + ")" : "FieldUpdate." + state;
    }
}
