use bevy::{
    core_pipeline::prepass::ViewPrepassTextures,
    prelude::*,
    render::{
        render_graph::{NodeRunError, RenderGraphContext, RenderLabel, ViewNode},
        render_resource::{
            BindGroupEntries, Operations, PipelineCache, RenderPassColorAttachment,
            RenderPassDescriptor,
        },
        renderer::RenderContext,
        view::ViewTarget,
    },
};

use crate::NormalPipeline;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct NormalNodeLabel;

#[derive(Default)]
pub struct NormalNode;

impl ViewNode for NormalNode {
    type ViewQuery = (&'static ViewTarget, &'static ViewPrepassTextures);

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, prepass_textures): bevy::ecs::query::QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let edge_detection_pipeline = world.resource::<NormalPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let Some(pipeline) =
            pipeline_cache.get_render_pipeline(edge_detection_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let Some(normal_texture) = prepass_textures.normal_view() else {
            return Ok(());
        };

        let bind_group = render_context.render_device().create_bind_group(
            "normal_bind_group",
            &edge_detection_pipeline.layout,
            &BindGroupEntries::sequential((normal_texture,)),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("normal_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}
