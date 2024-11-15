using System.Windows.Input;

namespace Wpf.Interaction;

internal sealed class CommandAdapter(
    IAsyncCommand command,
    ICommandContext? context = null)
    : ICommand, IAsyncCommand
{
    public event EventHandler? CanExecuteChanged
    {
        add
        {
            command.CanExecuteChanged += value;

            if (context is not null)
            {
                context.CanExecuteChanged += value;
            }
        }
        remove
        {
            if (context is not null)
            {
                context.CanExecuteChanged -= value;
            }

            command.CanExecuteChanged -= value;
        }
    }

    public bool CanExecute(object? parameter)
    {
        return (context?.CanExecute(this) ?? true)
            && command.CanExecute(parameter);
    }

    public async ValueTask ExecuteAsync(object? parameter)
    {
        using var execution = context?.Begin(this);
        await command.ExecuteAsync(parameter).ConfigureAwait(true);
    }

    // ReSharper disable once AsyncVoidMethod - By design to adapt to non-async interface
    async void ICommand.Execute(object? parameter)
    {
        await ExecuteAsync(parameter).ConfigureAwait(false);
    }
}
