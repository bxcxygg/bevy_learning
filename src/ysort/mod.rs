use bevy::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct YSort(pub i32);

fn sort(mut query: Query<(&mut Transform, &YSort)>) {
    let mut keys: Vec<i32> = Vec::new();
    let mut transformers: HashMap<i32, Vec<Mut<Transform>>> = HashMap::new();
    for (transform, ysort) in query.iter_mut() {
        if !transformers.contains_key(&ysort.0) {
            transformers.insert(ysort.0, Vec::new());
            keys.push(ysort.0);
        }
        let collector = transformers.get_mut(&ysort.0);
        if let Some(collector) = collector {
            collector.push(transform);
        }
    }

    keys.sort_unstable();
    let mut index: f32 = 0.;
    for k in keys.iter() {
        let v = transformers.get_mut(k).unwrap();
        v.sort_unstable_by(|a, b| match a.translation.y.partial_cmp(&b.translation.y) {
            None => Ordering::Equal,
            Some(other) => other,
        });

        for transform in v.iter_mut() {
            transform.translation.z = *k as f32 + index;
            index += 1.;
        }
    }
}

pub struct YSortPlugin;

impl Plugin for YSortPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, sort)
            .register_type::<YSort>();
    }

    fn name(&self) -> &str {
        "YSort"
    }
}
