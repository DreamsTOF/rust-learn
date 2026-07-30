using System.IO;
using System.Text;

var dir = @"C:\code\testruetlearn\leptos-learn\07_ssr";
var files = Directory.GetFiles(dir, "Cargo.toml", SearchOption.AllDirectories);

foreach (var file in files)
{
    var content = File.ReadAllText(file);
    var lines = File.ReadAllLines(file);
    if (lines.Length <= 1)
    {
        Console.WriteLine($"Fixing: {file}");
        var fixed1 = content.Replace("[package]", "[package]\r\n");
        var fixed2 = fixed1.Replace("]name ", "]\r\nname ");
        var fixed3 = fixed2.Replace("\"version ", "\"\r\nversion ");
        var fixed4 = fixed3.Replace("\"edition ", "\"\r\nedition ");
        var fixed5 = fixed4.Replace("\"[dependencies]", "\"\r\n[dependencies]");
        var fixed6 = fixed5.Replace("](leptos)", "]\r\n$1");
        // Last case: ]leptos.workspace
        var fixed7 = fixed6.Replace("]leptos", "]\r\nleptos");
        File.WriteAllText(file, fixed7, new UTF8Encoding(false));
    }
}

Console.WriteLine("Done fixing all Cargo.toml files");
