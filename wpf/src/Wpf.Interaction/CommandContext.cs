namespace Wpf.Interaction;

public sealed class CommandContext : ICommandContext
{
    private volatile object? _currentCommand;

    public event EventHandler? CanExecuteChanged;

    public bool CanExecute(object command)
    {
        return _currentCommand is null;
    }

    public IDisposable Begin(object command)
    {
        if (Interlocked.CompareExchange(ref _currentCommand, command, null) is not null)
        {
            throw new InvalidOperationException("Command is already running.");
        }

        CanExecuteChanged?.Invoke(this, EventArgs.Empty);

        return new CommandExecution(this, command);
    }

    private void End(object command)
    {
        Interlocked.CompareExchange(ref _currentCommand, null, command);
        CanExecuteChanged?.Invoke(this, EventArgs.Empty);
    }

    private sealed class CommandExecution(
        CommandContext context,
        object command)
        : IDisposable
    {
        public void Dispose()
        {
            context.End(command);
        }
    }
}
