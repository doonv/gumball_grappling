use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
};
use bevy_toon_shader::{ToonShaderMainCamera, ToonShaderSun};

pub struct CustomMaterialsPlugin;
impl Plugin for CustomMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<OutlineToonMaterial>::default())
            .add_plugins(UiMaterialPlugin::<RoundedRectangleMaterial>::default())
            .add_systems(Update, update_outline_toon_shader);
    }
}

pub fn update_outline_toon_shader(
    main_cam: Query<&GlobalTransform, With<ToonShaderMainCamera>>,
    sun: Query<(&GlobalTransform, &DirectionalLight), With<ToonShaderSun>>,
    ambient_light: Option<Res<AmbientLight>>,
    mut toon_materials: ResMut<Assets<OutlineToonMaterial>>,
) {
    for (_, toon_mat) in toon_materials.iter_mut() {
        if let Ok(cam_transform) = main_cam.get_single() {
            toon_mat.camera_pos = cam_transform.translation();
        }
        if let Ok((sun_t, dir_light)) = sun.get_single() {
            toon_mat.sun_dir = sun_t.back();
            toon_mat.sun_color = dir_light.color;
        }
        if let Some(light) = &ambient_light {
            toon_mat.ambient_color = light.color;
        }
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
#[uniform(0, ToonShaderOutlineMaterialUniform)]
pub struct OutlineToonMaterial {
    pub color: Color,
    pub sun_dir: Vec3,
    pub sun_color: Color,
    pub camera_pos: Vec3,
    pub ambient_color: Color,
    pub outline_color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Option<Handle<Image>>,
}

impl Material for OutlineToonMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/outline_toon.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

impl AsBindGroupShaderType<ToonShaderOutlineMaterialUniform> for OutlineToonMaterial {
    fn as_bind_group_shader_type(
        &self,
        _images: &bevy::render::render_asset::RenderAssets<Image>,
    ) -> ToonShaderOutlineMaterialUniform {
        ToonShaderOutlineMaterialUniform {
            color: self.color.into(),
            sun_dir: self.sun_dir,
            sun_color: self.sun_color.into(),
            camera_pos: self.camera_pos,
            ambient_color: self.ambient_color.into(),
            outline_color: self.outline_color.into(),
        }
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct ToonShaderOutlineMaterialUniform {
    pub color: Vec4,
    pub sun_dir: Vec3,
    pub sun_color: Vec4,
    pub camera_pos: Vec3,
    pub ambient_color: Vec4,
    pub outline_color: Vec4,
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct RoundedRectangleMaterial {
    #[uniform(0)]
    pub color: Vec4,
    #[uniform(1)]
    pub roundedness: Vec2,
}

impl UiMaterial for RoundedRectangleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/rounded_rectangle.wgsl".into()
    }
}
