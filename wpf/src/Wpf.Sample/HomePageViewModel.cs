using System.IO;
using System.Windows.Input;
using Wpf.Navigation;

namespace Wpf.Sample;

public sealed class HomePageViewModel
{
    private readonly IHostApplicationLifetime _applicationLifetime;

    public HomePageViewModel(IHostApplicationLifetime applicationLifetime)
    {
        _applicationLifetime = applicationLifetime;

        Stop = new DelegateCommand(ExecuteStop);
        ThrowException = new DelegateCommand(ExecuteThrowException);
        NavigateOtherPage = NavigationCommand.Create(() => new("/OtherPage.xaml", UriKind.Relative));
    }

    public ICommand Stop { get; }
    public ICommand ThrowException { get; }
    public NavigationCommand NavigateOtherPage { get; }

    private void ExecuteStop()
    {
        _applicationLifetime.StopApplication();
    }

    private static void ExecuteThrowException()
    {
        throw new IOException("boom!");
    }

    private sealed class DelegateCommand(Action action) : ICommand
    {
        public bool CanExecute(object? parameter)
        {
            return true;
        }

        public void Execute(object? parameter)
        {
            action();
        }

        public event EventHandler? CanExecuteChanged
        {
            add { }
            remove { }
        }
    }
}
