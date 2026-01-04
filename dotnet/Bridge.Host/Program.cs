using System;
using System.Runtime.InteropServices;

internal static class Native
{
    [DllImport("libbridge_core.so", EntryPoint = "bridge_create")]
    internal static extern IntPtr Create();

    [DllImport("libbridge_core.so", EntryPoint = "bridge_destroy")]
    internal static extern void Destroy(IntPtr ctx);

    [DllImport("libbridge_core.so", EntryPoint = "bridge_add")]
    internal static extern int Add(IntPtr ctx, int x);
}

class Program
{
    static void Main()
    {
        Console.WriteLine("Managed entry reached");

        IntPtr ctx = Native.Create();
        Console.WriteLine("Context created");

        Console.WriteLine(Native.Add(ctx, 2));
        Console.WriteLine(Native.Add(ctx, 3));

        Native.Destroy(ctx);
        Console.WriteLine("Done");
    }
}
