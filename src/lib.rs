mod pose;

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Mutex;

use glam::{IVec2, Mat4, Vec2, Vec4};

pub use pose::Pose;
use shipyard::sparse_set::SparseSet;
use shipyard::{
    AllStoragesViewMut, Component, EntitiesView, EntitiesViewMut, EntityId, Get, IntoIter, IntoWorkload, Unique, UniqueView, View, ViewMut, Workload, World
};

static mut POSE_DATA: pose::PoseData = [0.0; 17 * 2];
pub(crate) static STATE: Mutex<Option<State>> = Mutex::new(None);

pub trait Game: Send + 'static {
    fn init(&mut self, world: &mut World);
    fn update(&mut self, world: &mut World, delta: f32);
}

pub(crate) struct State {
    pub(crate) world: World,
    pub(crate) poses: HashMap<u32, EntityId>,
    pub(crate) game: Box<dyn Game>,
}

#[derive(Unique)]
pub struct Delta(pub f32);

pub struct Material(u32);

impl Material {
    pub fn texture(texture_name: &str, tint_r: f32, tint_g: f32, tint_b: f32) -> Self {
        unsafe {
            Material(simulo_create_material(
                texture_name.as_ptr(),
                texture_name.len(),
                tint_r,
                tint_g,
                tint_b,
            ))
        }
    }

    pub fn solid_color(r: f32, g: f32, b: f32) -> Self {
        unsafe { Material(simulo_create_material(std::ptr::null(), 0, r, g, b)) }
    }
}

impl std::ops::Drop for Material {
    fn drop(&mut self) {
        unsafe { simulo_drop_material(self.0) }
    }
}

#[derive(Component)]
pub struct Rendered(u32);

impl Rendered {
    pub fn new(material: &Material) -> Self {
        Rendered(unsafe { simulo_create_rendered_object(material.0) })
    }
}

impl std::ops::Drop for Rendered {
    fn drop(&mut self) {
        unsafe {
            simulo_drop_rendered_object(self.0);
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct Position2d(pub Vec2);

#[derive(Component, Clone, Default)]
pub struct Rotation2d(pub f32);

#[derive(Component, Clone, Default)]
pub struct Scale2d(pub Vec2);

#[derive(Component, Clone, Default)]
pub struct Velocity2d(pub Vec2);

#[derive(Component, Clone, Default)]
#[track(Insertion, Modification)]
pub struct Transform(pub Mat4);

impl Transform {
    pub fn from_2d_pos(position: Vec2) -> Self {
        Self(Mat4::from_translation(position.extend(0.0)))
    }

    pub fn from_2d_pos_scale(position: Vec2, scale: Vec2) -> Self {
        Self(Mat4::from_translation(position.extend(0.0)) * Mat4::from_scale(scale.extend(1.0)))
    }

    pub fn from_2d_pos_rotation(position: Vec2, rotation: f32) -> Self {
        Self(Mat4::from_translation(position.extend(0.0)) * Mat4::from_rotation_z(rotation))
    }

    pub fn from_2d_pos_rotation_scale(position: Vec2, rotation: f32, scale: Vec2) -> Self {
        Self(Mat4::from_translation(position.extend(0.0)) * Mat4::from_rotation_z(rotation) * Mat4::from_scale(scale.extend(1.0)))
    }

    pub fn from_2d_pos_rotation_scale_skew(position: Vec2, rotation: f32, scale: Vec2, skew: Vec2) -> Self {
        Self(Mat4::from_translation(position.extend(0.0)) * Mat4::from_rotation_z(rotation) * Mat4::from_scale(scale.extend(1.0)) * Mat4::from_cols(
            Vec4::new(1.0, skew.y, 0.0, 0.0),
            Vec4::new(skew.x, 1.0, 0.0, 0.0),
            Vec4::Z,
            Vec4::W,
        ))
    }
}

#[derive(Component, Clone)]
#[track(All)]
pub struct GlobalTransform {
    pub parent: Mat4,
    pub global: Mat4,
}

impl Default for GlobalTransform {
    fn default() -> Self {
        Self {
            parent: Mat4::IDENTITY,
            global: Mat4::IDENTITY,
        }
    }
}

#[derive(Component)]
pub struct Hierarchy {
    pub children: Vec<EntityId>,
    pub level: usize,
}

impl Hierarchy {
    pub fn root() -> Self {
        Self {
            children: Vec::new(),
            level: 0,
        }
    }

    pub fn root_with_children<const N: usize>(children: [EntityId; N]) -> Self {
        Self {
            children: children.to_vec(),
            level: 0,
        }
    }

    pub fn child_of<const N: usize>(parent: &Hierarchy, children: [EntityId; N]) -> Self {
        Self {
            children: children.to_vec(),
            level: parent.level + 1,
        }
    }
}

#[derive(Component)]
pub struct Delete;

fn velocity_tick(
    delta: UniqueView<Delta>,
    mut positions: ViewMut<Position2d>,
    velocities: View<Velocity2d>,
) {
    for (position, velocity) in (&mut positions, &velocities).iter() {
        position.0 += velocity.0 * delta.0;
    }
}

fn recalculate_transforms(
    transforms: View<Transform>,
    entites: EntitiesViewMut,
    mut transform_states: ViewMut<GlobalTransform>,
    hierarchies: View<Hierarchy>,
) {
    let mut updated_transforms = (
        transforms.inserted_or_modified(),
        &transform_states,
        &hierarchies,
    )
        .iter()
        .with_id()
        .map(|(entity, (_, transform_state, hierarchy))| {
            (entity, transform_state.parent.clone(), hierarchy.level)
        })
        .collect::<Vec<_>>();

    updated_transforms.sort_by_key(|&(_, _, level)| level);

    let mut updated_entities = HashSet::new();
    let mut bfs = VecDeque::new();
    bfs.extend(
        updated_transforms
            .into_iter()
            .map(|(e, parent_transform, _)| (e, parent_transform)),
    );

    while let Some((entity, parent_transform)) = bfs.pop_front() {
        if !updated_entities.insert(entity) {
            continue;
        }

        let global_transform = parent_transform * transforms.get(entity).unwrap().0;

        entites.add_component(
            entity,
            &mut transform_states,
            GlobalTransform {
                parent: parent_transform,
                global: global_transform,
            },
        );

        if let Ok(hierarchy) = hierarchies.get(entity) {
            for &child in &hierarchy.children {
                bfs.push_back((child, global_transform));
            }
        }
    }
}

fn update_global_transforms(transform_states: View<GlobalTransform>, renders: View<Rendered>) {
    for (transform, rendered) in (transform_states.inserted_or_modified(), &renders).iter() {
        unsafe {
            simulo_set_rendered_object_transform(
                rendered.0,
                transform.global.to_cols_array().as_ptr(),
            );
        }
    }
}

fn propagate_delete_to_children(
    entities: EntitiesView,
    hierarchies: View<Hierarchy>,
    mut deleted: ViewMut<Delete>,
) {
    let mut bfs = VecDeque::new();
    let mut seen_children = HashSet::new();

    for (hierarchy, _) in (&hierarchies, &deleted).iter() {
        for &child in &hierarchy.children {
            if seen_children.insert(child) {
                bfs.push_back(child);
            }
        }
    }

    while let Some(entity) = bfs.pop_front() {
        if let Ok(heirarchy) = hierarchies.get(entity) {
            for &child in &heirarchy.children {
                if seen_children.insert(child) {
                    bfs.push_back(child);
                }
            }
        }
        entities.add_component(entity, &mut deleted, Delete);
    }
}

fn do_delete(mut all: AllStoragesViewMut) {
    all.delete_any::<SparseSet<Delete>>();
}

fn post_update_workload() -> Workload {
    (
        velocity_tick,
        recalculate_transforms,
        update_global_transforms,
        propagate_delete_to_children,
        do_delete,
    )
        .into_workload()
}

pub fn window_size() -> IVec2 {
    unsafe { IVec2::new(simulo_window_width(), simulo_window_height()) }
}

#[allow(static_mut_refs)]
pub fn init(game: Box<dyn Game>) {
    unsafe {
        simulo_set_buffers(POSE_DATA.as_mut_ptr());
        let mut lock = STATE.lock().unwrap();
        let state = lock.insert(State {
            poses: HashMap::new(),
            world: World::new(),
            game,
        });

        state.world.add_workload(post_update_workload);

        state.game.as_mut().init(&mut state.world);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn simulo__update(delta: f32) {
    let mut lock = STATE.lock().unwrap();
    let state = lock.as_mut().unwrap();

    state.world.add_unique(Delta(delta));
    state.game.as_mut().update(&mut state.world, delta);

    state.world.run_workload(post_update_workload).unwrap();

    state.world.clear_all_inserted_and_modified();
    state.world.clear_all_removed_and_deleted();
}

#[unsafe(no_mangle)]
unsafe extern "C" fn simulo__pose(id: u32, alive: bool) {
    let mut lock = STATE.lock().unwrap();
    let state = lock.as_mut().unwrap();

    if alive {
        unsafe {
            if let Some(entity) = state.poses.get(&id) {
                let mut poses = state.world.borrow::<ViewMut<Pose>>().unwrap();
                (&mut poses).get(*entity).unwrap().0 = POSE_DATA;
            } else {
                let entity = state.world.add_entity((Pose(POSE_DATA), Hierarchy::root()));
                state.poses.insert(id, entity);
            }
        }
    } else {
        let entity = state.poses.remove(&id).unwrap();
        state.world.add_component(entity, Delete);
    }
}

unsafe extern "C" {
    fn simulo_set_buffers(pose: *mut f32);

    fn simulo_create_material(name: *const u8, name_len: usize, r: f32, g: f32, b: f32) -> u32;
    fn simulo_update_material(material: u32, r: f32, g: f32, b: f32);
    fn simulo_drop_material(material: u32);

    fn simulo_create_rendered_object(material: u32) -> u32;
    fn simulo_set_rendered_object_material(id: u32, material: u32);
    fn simulo_set_rendered_object_transform(id: u32, matrix: *const f32);
    fn simulo_drop_rendered_object(id: u32);

    fn simulo_window_width() -> i32;
    fn simulo_window_height() -> i32;
}
