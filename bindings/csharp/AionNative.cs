using System.Runtime.InteropServices;

public static class AionNative
{
    [DllImport("aion", CallingConvention = CallingConvention.Cdecl)]
    public static extern int aion_telemetry_set_enabled(byte enabled);
}
