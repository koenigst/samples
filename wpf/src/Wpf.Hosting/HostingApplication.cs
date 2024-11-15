using System.Windows;

namespace Wpf.Hosting;

public class HostingApplication : Application, IDisposable
{
    protected HostRunner Host { get; } = new();

    public void Dispose()
    {
        Dispose(true);
        GC.SuppressFinalize(this);
    }

    protected virtual void Configure(IHostBuilder builder, string[] args)
    {
        builder
            .ConfigureDefaults(args)
            .UseWpfLifetime(this);
    }

    protected sealed override void OnStartup(StartupEventArgs e)
    {
        base.OnStartup(e);
        Configure(Host.Builder, e.Args);
        Host.Start();
    }

    protected virtual void Dispose(bool disposing)
    {
        if (disposing)
        {
            Host.Dispose();
        }
    }
}
