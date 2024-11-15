namespace Wpf.Interaction;

public static class Command
{
    private static readonly Func<object?, bool> Always = _ => true;

    public static IAsyncCommand Wrap(this IAsyncCommand command, ICommandContext? context = null)
    {
        return new CommandAdapter(command, context);
    }

    public static IAsyncCommand Create(
        Func<object?, ValueTask> executeAsync,
        Func<object?, bool>? canExecute = null,
        ICommandContext? context = null)
    {
        return new DelegateAsyncCommand(executeAsync, canExecute ?? Always)
            .Wrap(context);
    }

    public static IAsyncCommand Create(
        Func<ValueTask> executeAsync,
        Func<bool>? canExecute = null,
        ICommandContext? context = null)
    {
        return Create(_ => executeAsync(), TryWrap(canExecute), context);
    }

    public static IAsyncCommand Create(
        Action<object?> execute,
        Func<object?, bool>? canExecute = null,
        ICommandContext? context = null)
    {
        return Create(Wrap(execute), canExecute, context);
    }

    public static IAsyncCommand Create(
        Action execute,
        Func<bool>? canExecute = null,
        ICommandContext? context = null)
    {
        return Create(Wrap(execute), TryWrap(canExecute), context);
    }

    public static IAsyncCommand Create(
        this ICommandContext context,
        Func<object?, ValueTask> executeAsync,
        Func<object?, bool>? canExecute = null)
    {
        return Create(executeAsync, canExecute, context);
    }

    public static IAsyncCommand Create(
        this ICommandContext context,
        Func<ValueTask> executeAsync,
        Func<bool>? canExecute = null)
    {
        return Create(executeAsync, canExecute, context);
    }

    public static IAsyncCommand Create(
        this ICommandContext context,
        Action<object?> execute,
        Func<object?, bool>? canExecute = null)
    {
        return Create(execute, canExecute, context);
    }

    public static IAsyncCommand Create(
        this ICommandContext context,
        Action execute,
        Func<bool>? canExecute = null)
    {
        return Create(execute, canExecute, context);
    }

    private static Func<object?, bool>? TryWrap(Func<bool>? canExecute)
    {
        return canExecute is not null
            ? Wrap(canExecute)
            : null;
    }

    private static Func<object?, bool> Wrap(Func<bool> canExecute)
    {
        return _ => canExecute();
    }

    private static Func<object?, ValueTask> Wrap(Action<object?> execute)
    {
        return o =>
        {
            execute(o);
            return ValueTask.CompletedTask;
        };
    }

    private static Func<object?, ValueTask> Wrap(Action execute)
    {
        return _ =>
        {
            execute();
            return ValueTask.CompletedTask;
        };
    }
}
