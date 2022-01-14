#![no_main]

use wasm4 as w4;

struct SansRuntime {
    framebuffer: w4::draw::Framebuffer,
}

// Displays sans.
// You can take a look at the result of this program at `TODO`.
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
            const SMILE: _ = include_sprite!("src/sans.png");
            const TALK: _ = include_sprite!("src/talk.png");
        };

        self.framebuffer.replace_palette(PALETTE);
        self.framebuffer.blit(&SMILE, [68, 32], <_>::default());
        self.framebuffer.blit(&TALK, [0, 92], <_>::default());
    }
}

w4::main! { SansRuntime }
