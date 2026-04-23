package com.aion;

public final class AionNative {
    static {
        System.loadLibrary("aion_jni");
    }

    public static native int aionRun(String cmd, String[] args);
}
