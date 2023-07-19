using SampleApp;

var builder = SampleAppHost.CreateHostBuilder(args);

var host = builder.Build();
try
{
    await host.RunAsync().ConfigureAwait(false);
}
finally
{
    switch (host)
    {
        case IAsyncDisposable asyncDisposable:
            await asyncDisposable.DisposeAsync().ConfigureAwait(false);
            break;
        default:
            host.Dispose();
            break;
    }
}
