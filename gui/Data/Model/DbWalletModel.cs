using System;
using Dimension.MaskCore.Model;
using MongoDB.Bson;
using Realms;

namespace Dimension.MaskCore.Data.Model;

internal class DbWalletModel : RealmObject
{
    [PrimaryKey] public ObjectId Id { get; set; } = ObjectId.GenerateNewId();
    [Required] public string Name { get; set; } = string.Empty;
    [Required] public string Address { get; set; } = string.Empty;
    public string DerivationPath { get; set; } = string.Empty;
    public byte[] Data { get; set; } = Array.Empty<byte>();
    private string PlatformTypeRaw { get; set; } = string.Empty;

    public PlatformType PlatformType
    {
        get => Enum.TryParse(PlatformTypeRaw, out PlatformType result) ? result : PlatformType.Ethereum;
        set => PlatformTypeRaw = value.ToString();
    }

    public DateTimeOffset CreatedAt { get; set; } = DateTimeOffset.UtcNow;
    public DateTimeOffset UpdatedAt { get; set; } = DateTimeOffset.UtcNow;
}