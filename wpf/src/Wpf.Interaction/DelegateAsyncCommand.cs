namespace Wpf.Interaction;

public sealed class DelegateAsyncCommand(
    Func<object?, ValueTask> executeAsync,
    Func<object?, bool>? canExecute)
    : IAsyncCommand
{
    public event EventHandler? CanExecuteChanged;

    public bool CanExecute(object? parameter)
    {
        return canExecute?.Invoke(parameter)
            ?? true;
    }

    public ValueTask ExecuteAsync(object? parameter)
    {
        return executeAsync(parameter);
    }

    public void TriggerCanExecuteChanged()
    {
        CanExecuteChanged?.Invoke(this, EventArgs.Empty);
    }
}
