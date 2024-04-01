use super::VideoClip;
use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext},
    render::texture::Image,
};

pub struct AviLoader;

impl AssetLoader for AviLoader {
    type Asset = VideoClip;

    type Settings = ();

    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            load_avi(&bytes, load_context).await
        })
    }

    fn extensions(&self) -> &[&str] {
        &["avi"]
    }
}

async fn load_avi<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
) -> Result<VideoClip, anyhow::Error> {
    let (avi, frames) = avi::read_avi(bytes)?;
    let avi_header = avi.avi_header().unwrap();

    let mut out_frames = vec![];

    for (index, frame) in frames.iter().enumerate() {
        let dyn_image = image::load_from_memory(&frame.0)?;
        let image = Image::from_dynamic(dyn_image, true);
        let frame_label = format!("Frame{}", index);
        let frame = load_context.add_labeled_asset(frame_label, image);
        out_frames.push(frame);
    }

    Ok(VideoClip {
        frames: out_frames,
        frame_rate: avi_header.fps(),
    })
}
