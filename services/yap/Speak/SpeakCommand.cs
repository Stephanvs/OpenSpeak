namespace Yap.Speak;

public sealed class SpeakCommand : Command
{
    private Argument<string> TextArgument { get; } = new("text")
    {
        Arity = ArgumentArity.ExactlyOne,
        Description = "Text to speak"
    };

    public SpeakCommand()
        : base("speak", "Convert text to speech and play immediately")
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
            Console.Error.WriteLine("No text supplied for synthesize.");
            return 1;
        }

        Console.WriteLine($"[synthesize] generated audio for: {text}");
        return 0;
    }
}
