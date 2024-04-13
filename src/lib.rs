use bevy::{
    asset::load_internal_asset,
    core_pipeline::{
        core_3d::graph::{Core3d, Node3d},
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_graph::{RenderGraphApp, ViewNodeRunner},
        render_resource::{
            binding_types::texture_2d, BindGroupLayout, BindGroupLayoutEntries,
            CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, MultisampleState,
            PipelineCache, PrimitiveState, RenderPipelineDescriptor, ShaderStages, TextureFormat,
            TextureSampleType,
        },
        renderer::RenderDevice,
        texture::BevyDefault,
        RenderApp,
    },
};
use node::NormalNode;

use crate::node::NormalNodeLabel;

mod node;

pub const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(410592619790336);

pub struct ShowNormalPlugin;
impl Plugin for ShowNormalPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER_HANDLE, "normal.wgsl", Shader::from_wgsl);

        app.add_plugins(ExtractComponentPlugin::<NormalCamera>::default());

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<NormalNode>>(Core3d, NormalNodeLabel)
            .add_render_graph_edges(
                Core3d,
                (Node3d::EndMainPass, NormalNodeLabel, Node3d::Tonemapping),
            );
    }
    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.init_resource::<NormalPipeline>();
    }
}

#[derive(Component, Clone, Copy, ExtractComponent)]
pub struct NormalCamera;

#[derive(Resource)]
struct NormalPipeline {
    layout: BindGroupLayout,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for NormalPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "normal_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    // normal prepass
                    texture_2d(TextureSampleType::Float { filterable: false }),
                ),
            ),
        );

        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("normal_pipeline".into()),
                    layout: vec![layout.clone()],
                    // This will setup a fullscreen triangle for the vertex state
                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader: SHADER_HANDLE,
                        shader_defs: vec![],
                        entry_point: "fragment".into(),
                        targets: vec![Some(ColorTargetState {
                            format: TextureFormat::bevy_default(),
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    push_constant_ranges: vec![],
                });

        Self {
            layout,
            pipeline_id,
        }
    }
}
