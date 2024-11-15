using Microsoft.Extensions.Hosting;

namespace Wpf.Sample;

public sealed partial class App
{
    protected override void Configure(IHostBuilder builder, string[] args)
    {
        base.Configure(builder, args);

        builder.ConfigureServices(s => s.AddApplication());
    }
}
