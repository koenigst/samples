using System.IO;
using Microsoft.Extensions.Hosting;
using Wpf.Interaction;
using Wpf.Navigation;

namespace Wpf.Sample;

public sealed class HomePageViewModel
{
    private readonly IHostApplicationLifetime _applicationLifetime;

    public HomePageViewModel(
        IHostApplicationLifetime applicationLifetime,
        ICommandContext commandContext)
    {
        _applicationLifetime = applicationLifetime;

        Stop = commandContext.Create(ExecuteStop);
        ThrowException = commandContext.Create(ExecuteThrowException);
        Slow = commandContext.Create(ExecuteSlow);
        NavigateOtherPage = commandContext.CreateNavigate(() => new("/OtherPage.xaml", UriKind.Relative));
    }

    public IAsyncCommand Stop { get; }
    public IAsyncCommand ThrowException { get; }
    public IAsyncCommand Slow { get; }
    public IAsyncCommand NavigateOtherPage { get; }

    private void ExecuteStop()
    {
        _applicationLifetime.StopApplication();
    }

    private static void ExecuteThrowException()
    {
        throw new IOException("boom!");
    }

    private static async ValueTask ExecuteSlow()
    {
        await Task.Delay(TimeSpan.FromSeconds(1)).ConfigureAwait(false);
    }
}
