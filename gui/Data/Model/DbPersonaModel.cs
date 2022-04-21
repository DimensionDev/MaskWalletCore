using System;
using System.Text.Json;
using Dimension.MaskWalletCore;
using MongoDB.Bson;
using Realms;

namespace Dimension.MaskCore.Data.Model;

internal class DbPersonaModel : RealmObject
{
    [PrimaryKey] public ObjectId Id { get; set; } = ObjectId.GenerateNewId();
    [Required] public string Name { get; set; } = string.Empty;
    [Required] public string Identifier { get; set; } = string.Empty;
    [Required] public string Mnemonic { get; set; } = string.Empty;
    [Required] public string Path { get; set; } = string.Empty;
    public bool WithPassword { get; set; }
    [Required] public string Password { get; set; } = string.Empty;
    [Required] public string PrivateKey { get; set; } = string.Empty;
    [Required] public string PublicKey { get; set; } = string.Empty;
    public string? LocalKey { get; set; }
    public DateTimeOffset CreatedAt { get; set; } = DateTimeOffset.UtcNow;
    public DateTimeOffset UpdatedAt { get; set; } = DateTimeOffset.UtcNow;

    public static DbPersonaModel FromPersona(PersonaKey persona, string mnemonic, string path, string password, bool withPassword, string name)
    {
        return new DbPersonaModel
        {
            Name = name,
            Identifier = persona.Identifier,
            Mnemonic = mnemonic,
            Path = path,
            WithPassword = withPassword,
            Password = password,
            PrivateKey = JsonSerializer.Serialize(persona.PrivateKey),
            PublicKey = JsonSerializer.Serialize(persona.PrivateKey with { d = null }),
            LocalKey = persona.LocalKey == null ? null : JsonSerializer.Serialize(persona.LocalKey),
        };
    }
}