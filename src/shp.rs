use anyhow::anyhow;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::{Handle, Texture},
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use bytes::{Buf, BufMut, Bytes};
use image::{DynamicImage, ImageBuffer, Rgba};

use crate::Pal;

#[derive(Debug, TypeUuid, Default)]
#[uuid = "5d74a5d7-cd53-431d-91ae-c4750f677897"]
pub struct Shp {
    pub image_count: usize,
    bytes: Vec<u8>,
}

impl Shp {
    pub fn new(bytes: &[u8]) -> anyhow::Result<Self> {
        let image_count = Shp::is_shp(bytes)?;
        Ok(Shp {
            image_count,
            bytes: bytes.to_vec(),
        })
    }

    pub fn is_shp(bytes: &[u8]) -> anyhow::Result<usize> {
        let mut bytes = Bytes::copy_from_slice(bytes);
        let max_file_offest = bytes.len();

        if bytes.get_u16_le() != 0 {
            return Err(anyhow!(
                "The shp file is damaged or the file format is wrong"
            ));
        }

        bytes.advance(4);
        let image_count = bytes.get_u16_le();
        //判断每一帧的帧头是否存在
        if bytes.len() < 24 * (image_count as usize) {
            return Err(anyhow!(
                "The shp file is damaged or the file format is wrong"
            ));
        }

        for _ in 0..image_count {
            bytes.advance(20);
            let file_offest = bytes.get_u32_le();
            if (file_offest as usize) > max_file_offest {
                return Err(anyhow!(
                    "The shp file is damaged or the file format is wrong"
                ));
            }
        }

        Ok(image_count as usize)
    }

    pub fn get_image(&self, pal: &Pal, index: usize) -> Option<DynamicImage> {
        let shp_frame_header = ShpFrameHeader::new(&self.bytes, index);
        let size = (shp_frame_header.w * shp_frame_header.h) as usize;
        let mut bytes = Bytes::copy_from_slice(&self.bytes);

        bytes.advance(shp_frame_header.offest as usize);

        let mut frame_buf = Vec::new();
        //image
        if shp_frame_header.format == 3 {
            for _ in 0..shp_frame_header.h {
                let len = bytes.get_u16_le() - 2;
                let mut is_zero = false;
                for _ in 0..len {
                    let cmd = bytes.get_u8();
                    if cmd == 0 {
                        is_zero = true;
                        continue;
                    }

                    if is_zero {
                        is_zero = false;
                        for _ in 0..cmd {
                            frame_buf.put_u8(0);
                        }
                    } else {
                        frame_buf.put_u8(cmd);
                    }
                }
            }
        } else if shp_frame_header.format == 2 {
            let len = bytes.get_u16_le() - 2;

            for _ in 0..shp_frame_header.h {
                //
                frame_buf.put(bytes.split_to(len as usize));
            }
        } else {
            for _ in 0..shp_frame_header.h {
                frame_buf.put(bytes.split_to(shp_frame_header.w as usize));
            }
        }

        //不确定是否需要
        if frame_buf.len() < size {
            for _ in frame_buf.len()..size {
                frame_buf.put_u8(0);
            }
        }

        let mut data = Vec::new();

        frame_buf.iter().for_each(|ele| {
            if *ele == 0 {
                data.push(0);
                data.push(0);
                data.push(0);
                data.push(0);
            } else {
                data.push(pal.colors[*ele as usize][0]);
                data.push(pal.colors[*ele as usize][1]);
                data.push(pal.colors[*ele as usize][2]);
                data.push(pal.colors[*ele as usize][3]);
            }
        });

        if data.len() == 0 {
            return None;
        }
        let img_buffer: ImageBuffer<Rgba<u8>, _> =
            ImageBuffer::from_raw(shp_frame_header.w as u32, shp_frame_header.h as u32, data)
                .unwrap();

        Some(DynamicImage::ImageRgba8(img_buffer))
    }
}

#[derive(Debug)]
pub struct ShpFrameHeader {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
    format: u8,
    offest: u32,
}

impl ShpFrameHeader {
    fn new(data: &[u8], index: usize) -> ShpFrameHeader {
        let mut bytes = Bytes::copy_from_slice(data);
        bytes.advance(8);
        bytes.advance(24 * index);
        let x = bytes.get_u16_le();
        let y = bytes.get_u16_le();
        let w = bytes.get_u16_le();
        let h = bytes.get_u16_le();
        let format = bytes.get_u8();
        let _r = bytes.get_u8();
        let _g = bytes.get_u8();
        let _b = bytes.get_u8();
        bytes.advance(8);

        let offest = bytes.get_u32_le();

        ShpFrameHeader {
            x,
            y,
            w,
            h,
            format,
            offest,
        }
    }
}

#[derive(Debug, Default)]
pub struct ShpLoader;

impl AssetLoader for ShpLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            match Shp::new(bytes) {
                Ok(shp) => {
                    load_context.set_default_asset(LoadedAsset::new(shp));
                    Ok(())
                }
                Err(e) => Err(e),
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["shp"]
    }
}
