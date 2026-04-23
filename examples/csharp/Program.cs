// AION C ABI pilot — build: cargo build -p aion-engine --features ffi
// Run: dotnet run (PATH must include ../../target/debug for aion_engine.dll)

using System;
using System.IO;
using System.Runtime.InteropServices;

internal static class Native
{
    private const string Lib = "aion_engine";

    [StructLayout(LayoutKind.Sequential)]
    public struct AionRunResult
    {
        public IntPtr stdout_data;
        public UIntPtr stdout_len;
        public IntPtr stderr_data;
        public UIntPtr stderr_len;
        public int exit_code;
        public ulong duration_ms;
        public IntPtr capsule_id;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct AionCapsule
    {
        public IntPtr model;
        public IntPtr prompt;
        public ulong seed;
        public IntPtr determinism_profile_json;
        public IntPtr token_trace_json;
        public IntPtr events_json;
        public IntPtr graph_json;
        public IntPtr why_report_json;
        public IntPtr drift_report_json;
        public IntPtr evidence_path;
    }

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int aion_capsule_save(ref AionCapsule capsule, IntPtr path);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int aion_replay_capsule(IntPtr path, ref AionRunResult out_result);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int aion_replay_symmetry_ok(IntPtr path, out byte out_ok);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern int aion_capsule_deterministic_hash_hex(IntPtr path, out IntPtr out_hex);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void aion_free_string(IntPtr s);

    [DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]
    public static extern void aion_free_run_result(ref AionRunResult r);
}

internal static class Program
{
    private static IntPtr Sz(string s) => Marshal.StringToCoTaskMemAnsi(s);

    public static void Main()
    {
        var tmp = Path.Combine(Path.GetTempPath(), "aion_csharp_capsule.aionai");
        if (File.Exists(tmp))
        {
            File.Delete(tmp);
        }

        var cap = new Native.AionCapsule
        {
            model = Sz("demo"),
            prompt = Sz("csharp pilot"),
            seed = 21,
            determinism_profile_json = Sz("{}"),
            token_trace_json = Sz("[]"),
            events_json = Sz("[]"),
            graph_json = Sz("{}"),
            why_report_json = Sz("{}"),
            drift_report_json = Sz("{}"),
            evidence_path = Sz(""),
        };
        var p = Sz(tmp);
        var rc = Native.aion_capsule_save(ref cap, p);
        if (rc != 0)
        {
            throw new InvalidOperationException("aion_capsule_save " + rc);
        }

        var rep = new Native.AionRunResult();
        rc = Native.aion_replay_capsule(p, ref rep);
        if (rc != 0)
        {
            throw new InvalidOperationException("aion_replay_capsule " + rc);
        }

        Native.aion_replay_symmetry_ok(p, out var sym);
        Native.aion_capsule_deterministic_hash_hex(p, out var hx);
        var hex = hx != IntPtr.Zero ? Marshal.PtrToStringAnsi(hx) ?? "" : "";
        if (hx != IntPtr.Zero)
        {
            Native.aion_free_string(hx);
        }

        Native.aion_free_run_result(ref rep);

        Console.WriteLine("AION | csharp pilot — symmetry_ok " + (sym != 0));
        Console.WriteLine("AION | deterministic_hash_hex " + hex[..Math.Min(32, hex.Length)] + "...");

        Marshal.FreeCoTaskMem(cap.model);
        Marshal.FreeCoTaskMem(cap.prompt);
        Marshal.FreeCoTaskMem(cap.determinism_profile_json);
        Marshal.FreeCoTaskMem(cap.token_trace_json);
        Marshal.FreeCoTaskMem(cap.events_json);
        Marshal.FreeCoTaskMem(cap.graph_json);
        Marshal.FreeCoTaskMem(cap.why_report_json);
        Marshal.FreeCoTaskMem(cap.drift_report_json);
        Marshal.FreeCoTaskMem(cap.evidence_path);
        Marshal.FreeCoTaskMem(p);
    }
}
