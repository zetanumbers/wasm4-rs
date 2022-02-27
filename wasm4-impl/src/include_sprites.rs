mod parse;
mod to_tokens;

use anyhow::{anyhow, Context};
use bitvec::{order::Msb0, prelude::BitVec};
use heapless::Vec as SVec;
use image::io::Reader as ImageReader;
use once_cell::unsync::OnceCell;
use std::{collections::HashMap, mem, path::PathBuf};
use wasm4_common::draw::{BitsPerPixel, DrawIndex, DrawIndices};

pub use parse::Input;
pub use to_tokens::Output;

type Sprite = wasm4_common::draw::Sprite<Vec<u8>>;

pub fn implementation(input: parse::Input) -> syn::Result<Output> {
    let package_name = input.package.package_name;

    let pkg_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let palette = OnceCell::new();
    let mut image_paths = HashMap::new();

    for item in input.input.consts {
        match &item.expr {
            parse::Expr::IncludeSprite(include_sprite) => {
                if let Some(_) = image_paths.insert(
                    include_sprite.path.value(),
                    item.clone().map_expr(|_| include_sprite.clone()),
                ) {
                    return Err(syn::Error::new_spanned(
                        &include_sprite,
                        "detected image duplication",
                    ));
                }
            }
            parse::Expr::CommonPalette(common_palette) => {
                let plt = SVec::<image::Bgr<u8>, 4>::from_iter(
                    common_palette
                        .colors
                        .iter()
                        .map(|rgb| image::Bgr(rgb.value.to_le_bytes()[..3].try_into().unwrap())),
                );

                palette
                    .try_insert((plt, item.clone().map_expr(|_| common_palette.clone())))
                    .map_err(|_| {
                        syn::Error::new(
                            common_palette.name_token.span,
                            "trying to assign a palette twice",
                        )
                    })?;
            }
        }
    }

    let (mut palette, palette_item) = palette.into_inner().ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "no `common_palette!` expresion found",
        )
    })?;

    let sprites = image_paths
        .into_iter()
        .map(|(path, item)| {
            let span = item.expr.path.span();

            (|| -> anyhow::Result<_> {
                let fullpath = pkg_root.join(&path);
                let img = ImageReader::open(&fullpath)
                    .with_context(|| format!("could not open {:?} file", &path))?
                    .decode()
                    .with_context(|| format!("could not decode {:?} file", &path))?
                    .into_bgra8();

                let mut bpp = BitsPerPixel::One;
                let mut indices = SVec::<Option<usize>, 4>::new();
                let mut bits = BitVec::<u8, Msb0>::new();

                for &p in img.pixels() {
                    let color = image::Bgr(p.0[..3].try_into().unwrap());
                    let idx = match p.0[3] {
                        0 => None,
                        255 => Some(
                            match palette
                                .iter()
                                .enumerate()
                                .find_map(|(i, c)| (*c == color).then(|| i))
                            {
                                Some(i) => i,
                                None => {
                                    let i = palette.len();
                                    palette.push(color).map_err(|c| {
                                        anyhow!(
                                            "all images combined have more than 4 colors \
                                         (transparent is not counted) \
                                         ({:?} and {:?} using indices {:?})",
                                            palette,
                                            indices,
                                            c
                                        )
                                    })?;
                                    i
                                }
                            },
                        ),
                        a => anyhow::bail!("got ambiguous alpha value: {}", a),
                    };

                    let i = match indices
                        .iter()
                        .enumerate()
                        .find_map(|(n, i)| (*i == idx).then(|| n))
                    {
                        Some(i) => i,
                        None => {
                            let i = indices.len();
                            indices.push(idx).map_err(|_| {
                                anyhow!(
                                    "image have more than 4 colors \
                                             (TRANSPARENT IS COUNTED) \
                                             ({:?} and {:?})",
                                    palette,
                                    color
                                )
                            })?;

                            if i == 2 {
                                // resize into 2bpp
                                bpp = BitsPerPixel::Two;
                                let old_bits = mem::take(&mut bits);
                                bits.reserve(old_bits.len() * 2);
                                bits.extend(old_bits.into_iter().flat_map(|b| [false, b]));
                            }

                            i
                        }
                    };

                    match bpp {
                        BitsPerPixel::One => bits.push(i != 0),
                        BitsPerPixel::Two => bits.extend([i & 0b10 != 0, i & 0b01 != 0]),
                    }
                }

                let indices = DrawIndices::from_array([0, 1, 2, 3].map(|i| {
                    DrawIndex::new(
                        indices
                            .get(i)
                            .copied()
                            .flatten()
                            .map_or(0, |idx| u16::try_from(idx).unwrap() + 1),
                    )
                    .unwrap()
                }));
                bits.set_uninitialized(false);

                Ok((
                    Sprite::from_bytes(bits.into_vec(), [img.width(), img.height()], bpp, indices)
                        .expect("something's wrong with the image dimentions"),
                    item,
                ))
            })()
            .map_err(|e| syn::Error::new(span, format!("{:#}", e)))
        })
        .collect::<syn::Result<_>>()?;

    palette.resize(4, image::Bgr([0; 3])).unwrap();
    let palette = (
        palette.into_array().unwrap().map(|bgr| {
            wasm4_common::draw::Color(u32::from_le_bytes(
                [0, 1, 2, 3].map(|i| bgr.0.get(i).copied().unwrap_or(0)),
            ))
        }),
        palette_item,
    );

    Ok(Output {
        package_name,
        sprites,
        palette,
    })
}
