use pokemonsms_core::resource::ResourceLocation;
use text::TextComponent;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(with = "::text::embed_json")]
    name: TextComponent,
    pc_sprite: ResourceLocation,
}
