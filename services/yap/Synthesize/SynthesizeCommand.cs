namespace Yap.Synthesize;

public sealed class SynthesizeCommand : Command
{
    private Argument<string> TextArgument { get; } = new("text")
    {
        Arity = ArgumentArity.ExactlyOne,
        Description = "Text to synthesize"
    };

    public SynthesizeCommand()
        : base("synthesize", "Generate audio file from text")
    {
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
