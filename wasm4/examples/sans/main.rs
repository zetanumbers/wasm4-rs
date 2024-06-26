use wasm4 as w4;

struct SansRuntime {
    framebuffer: w4::draw::Framebuffer,
}

// Displays sans.
// Preview is at: https://raw.githubusercontent.com/ZetaNumbers/wasm4-rs/main/examples/sans/preview.png
impl w4::rt::Runtime for SansRuntime {
    fn start(res: w4::rt::Resources) -> Self {
        SansRuntime {
            framebuffer: res.framebuffer,
        }
    }

    fn update(&mut self) {
        w4::include_sprites! {
            // 0xa64902 is a background color (indexed 0)
            // 0x000000 is a foreground color (indexed 1)
            // every other color is added if needed
            const PALETTE: _ = common_palette!(0xa64902, 0x000000);
            // every image may contain at most 4 colors each (transparent included)
            // all images may contain at most 4 color total (transparent excluded)
            // paths are relative to your Cargo.toml manifest
            const SMILE: _ = include_sprite!("examples/sans/sans.png");
            const TALK: _ = include_sprite!("examples/sans/talk.png");
        };

        self.framebuffer.replace_palette(PALETTE);
        self.framebuffer.blit(&SMILE, [68, 32], <_>::default());
        self.framebuffer.blit(&TALK, [0, 92], <_>::default());
    }
}

w4::main! { SansRuntime }
