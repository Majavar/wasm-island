pub fn submap(
    current_width: usize,
    current_height: usize,
    target_width: usize,
    target_height: usize,
    data: Vec<f64>,
) -> Vec<f64> {
    if current_width == target_width && current_height == target_height {
        data
    } else {
        data.into_iter()
            .take(current_width * target_height)
            .enumerate()
            .filter_map(|(i, v)| {
                if i % current_width < target_width {
                    Some(v)
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn normalize(mut heightmap: Vec<f64>) -> Vec<f64> {
    let mut it = heightmap.iter();

    if let Some((min, max)) = it.next().map(|&first| {
        it.fold((first, first), |(min, max), &val| {
            (min.min(val), max.max(val))
        })
    }) {
        heightmap.iter_mut().for_each(|val| {
            *val = (*val - min) / (max - min);
        });
    };

    heightmap
}
