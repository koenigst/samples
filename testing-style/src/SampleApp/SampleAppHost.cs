namespace SampleApp;

public sealed class SampleAppHost
{
    public static IHostBuilder CreateHostBuilder(string[] args)
    {
        return new HostBuilder()
            .ConfigureDefaults(args)
            .ConfigureWebHostDefaults(Configure);
    }

    private static void Configure(IWebHostBuilder builder)
    {
        builder
            .ConfigureServices(Configure)
            .Configure(Configure);
    }

    private static void Configure(IServiceCollection services)
    {
        services
            .AddApp()
            .AddAppInfrastructure();
    }

    private static void Configure(IApplicationBuilder builder)
    {
        builder
            .UseRouting()
            .UseEndpoints(e => e.MapControllers());
    }
}
