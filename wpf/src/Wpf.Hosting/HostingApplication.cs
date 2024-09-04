using System.Windows;

namespace Wpf.Hosting;

public class HostingApplication : Application
{
    private readonly HostRunner _host = new();

    protected virtual TimeSpan StopTimeout => TimeSpan.FromSeconds(30);

    protected virtual void Configure(IHostBuilder builder, string[] args)
    {
        builder
            .ConfigureDefaults(args)
            .UseWpfLifetime(this);
    }

    protected sealed override void OnStartup(StartupEventArgs e)
    {
        base.OnStartup(e);
        Configure(_host.Builder, e.Args);
        _host.Run();
    }

    protected sealed override void OnExit(ExitEventArgs e)
    {
        try
        {
            try
            {
                base.OnExit(e);
            }
            finally
            {
                _host.StopAndWait(StopTimeout);
            }
        }
        finally
        {
            _host.Dispose();
        }
    }
}
