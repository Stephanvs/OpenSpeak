using System.CommandLine;

namespace Yap.Commands.Speak;

public sealed class SpeakCommand : Command
{
    private Argument<string> TextArgument { get; } = new("text")
    {
        Arity = ArgumentArity.ExactlyOne,
        Description = "Text to speak"
    };

    public SpeakCommand()
        : base("speak", "Speak text using the default voice")
    {
        Aliases.Add("say");
        Add(TextArgument);
        SetAction(Execute);
    }

    private int Execute(ParseResult parseResult)
    {
        var text = parseResult.GetValue(TextArgument);

        if (string.IsNullOrWhiteSpace(text))
        {
            Console.Error.WriteLine("No text supplied for speak.");
            return 1;
        }

        Console.WriteLine($"[speak] {text}");
        return 0;
    }
}
