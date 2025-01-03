use bevy::{
    asset::AssetPath,
    image::{
        ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler,
        ImageSamplerDescriptor,
    },
    prelude::*,
};

#[allow(dead_code)]
pub trait AssetServerExtensions {
    /// My custom method to load textures exactly as I want them
    fn load_image_custom<'a>(&self, path: impl Into<AssetPath<'a>>) -> Handle<Image>;
}

impl<'a> AssetServerExtensions for Res<'a, AssetServer> {
    fn load_image_custom<'path>(&self, path: impl Into<AssetPath<'path>>) -> Handle<Image> {
        self.load_with_settings::<_, ImageLoaderSettings>(path, |settings| {
            settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                label: Some("QoS Better Default".to_string()),
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                mag_filter: ImageFilterMode::Linear,
                min_filter: ImageFilterMode::Linear,
                mipmap_filter: ImageFilterMode::Linear,
                ..Default::default()
            });
        })
    }
}
