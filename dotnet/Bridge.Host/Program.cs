using System;
using System.Runtime.InteropServices;

internal static class Native
{
    const string Lib = "libbridge_core.so";

    [DllImport(Lib, EntryPoint = "bridge_abi_version")]
    internal static extern uint AbiVersion();

    [DllImport(Lib, EntryPoint = "bridge_create")]
    internal static extern IntPtr Create();

    [DllImport(Lib, EntryPoint = "bridge_destroy")]
    internal static extern void Destroy(IntPtr ctx);

    [DllImport(Lib, EntryPoint = "bridge_command")]
    internal static extern int Command(
        IntPtr ctx,
        uint cmd,
        long arg,
        out long result
    );
}

class Program
{
    const uint CMD_STEP = 1;
    const uint CMD_RESET = 2;
    const uint CMD_GETLAST = 3;
    const uint CMD_ADD = 4;

    static void Main()
    {
        Console.WriteLine("Managed entry reached");

        uint abi = Native.AbiVersion();
        Console.WriteLine($"ABI={abi}");
        if (abi != 1)
            throw new Exception($"ABI mismatch: {abi}");

        IntPtr ctx = Native.Create();
        if (ctx == IntPtr.Zero)
            throw new Exception("Create failed");

        Console.WriteLine("Context created");

        Native.Command(ctx, CMD_ADD, 2, out var r1);
        Console.WriteLine($"add1={r1}");

        Native.Command(ctx, CMD_ADD, 3, out var r2);
        Console.WriteLine($"add2={r2}");

        Native.Command(ctx, CMD_STEP, 0, out var tick);
        Console.WriteLine($"tick={tick}");

        Native.Command(ctx, CMD_GETLAST, 0, out var last);
        Console.WriteLine($"last={last}");

        Native.Destroy(ctx);
        Console.WriteLine("Done");
    }
}
