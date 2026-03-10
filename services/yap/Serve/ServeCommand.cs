namespace Yap.Serve;

public sealed class ServeCommand : Command
{
    public ServeCommand()
        : base("serve", "Start a local server (placeholder)")
    {
        SetAction(Execute);
    }

    private int Execute(ParseResult parseResult)
    {
        Console.WriteLine("serve command is currently a placeholder and not implemented yet.");
        return 0;
    }
}
