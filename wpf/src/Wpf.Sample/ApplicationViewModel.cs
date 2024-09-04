using System.IO;
using System.Windows.Input;

namespace Wpf.Sample;

public sealed class ApplicationViewModel
{
    private readonly IHostApplicationLifetime _applicationLifetime;

    public ApplicationViewModel(IHostApplicationLifetime applicationLifetime)
    {
        _applicationLifetime = applicationLifetime;

        Stop = new DelegateCommand(ExecuteStop);
        ThrowException = new DelegateCommand(ExecuteThrowException);
    }

    public ICommand Stop { get; }
    public ICommand ThrowException { get; }

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
