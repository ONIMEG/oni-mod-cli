pub const FILE_NAME: &str = "Mod.cs";

pub const CONTENT: &str = r#"using HarmonyLib;

namespace {assembly_title} {
    public sealed class Mod : KMod.UserMod2 {
        public override void OnLoad(Harmony harmony) {
            base.OnLoad(harmony);
        }
    }
}"#;