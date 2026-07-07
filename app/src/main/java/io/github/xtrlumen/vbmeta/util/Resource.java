package io.github.xtrlumen.vbmeta.util;

public class Resource<T> {
    private final Status status;
    private final T data;
    private final Throwable error;

    private Resource(Status status, T data, Throwable error) {
        this.status = status;
        this.data = data;
        this.error = error;
    }

    public Status getStatus() {
        return status;
    }

    public T getData() {
        return data;
    }

    public Throwable getError() {
        return error;
    }

    public static <T> Resource<T> success(T data) {
        return new Resource<>(Status.SUCCESS, data, null);
    }

    public static <T> Resource<T> error(Throwable error, T data) {
        return new Resource<>(Status.ERROR, data, error);
    }

    public static <T> Resource<T> loading(T data) {
        return new Resource<>(Status.LOADING, data, null);
    }

    public enum Status {
        SUCCESS,
        ERROR,
        LOADING
    }
}
