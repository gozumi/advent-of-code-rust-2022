#[derive(Copy, Clone, Debug)]
struct ForwardCounting {
    visible_count: u32,
    tallest_height: u32,
}


#[derive(Clone, Debug)]
struct BackwardCounting {
    visible_heights: Vec<u32>,
}

#[derive(Debug)]
pub struct VisibilityCountStruct {
    from_left_collection: Vec<ForwardCounting>,
    from_right_collection: Vec<BackwardCounting>,
    from_top: Vec<ForwardCounting>,
    from_bottom: Vec<BackwardCounting>,
    total_visible_from_left: u32,
    total_visible_from_right: u32,
    total_visible_from_top: u32,
    total_visible_from_bottom: u32,
    visibility_grand_total: u32
}

#[derive(Debug)]
pub enum VisibilityCount {
    Some(VisibilityCountStruct),
    None,
}

pub fn get_grid_row(grid_row_string: &str) -> Vec<u32> {
    let chars = grid_row_string.chars();
    let mut grid_row: Vec<u32> = vec![];

    for c in chars {
        let height = c.to_digit(10).unwrap();
        grid_row.push(height);
    }

    return grid_row;
}

pub fn get_visibility_count(
    grid_row: &Vec<u32>,
    visibility_count: VisibilityCount,
) -> VisibilityCount {
    match visibility_count {
        VisibilityCount::Some(mut v_count) => {
            v_count = get_visibility_count_details(grid_row, v_count);
            return VisibilityCount::Some(v_count);
        }
        VisibilityCount::None => {
            let v_count = VisibilityCountStruct {
                from_left_collection: vec![],
                from_right_collection: vec![],
                from_top: vec![],
                from_bottom: vec![],
                total_visible_from_left: 0,
                total_visible_from_right: 0,
                total_visible_from_top: 0,
                total_visible_from_bottom: 0,
                visibility_grand_total: 0,
            };
            return VisibilityCount::Some(v_count);
        }
    }
}

fn get_visibility_count_details(
    grid_row: &Vec<u32>,
    mut visibility_count: VisibilityCountStruct,
) -> VisibilityCountStruct {
    let mut from_left = ForwardCounting {
        visible_count: 0,
        tallest_height: 0,
    };

    let mut from_right = BackwardCounting {
        visible_heights: vec![],
    };

    let mut from_top: ForwardCounting = ForwardCounting {
        visible_count: 0,
        tallest_height: 0,
    };

    let mut from_bottom = BackwardCounting {
        visible_heights: vec![],
    };

    for (i, tree_height) in grid_row.iter().enumerate() {
        from_left = get_forward_visibility_count_details(from_left, *tree_height);
        from_right = get_backward_visibility_count_details(from_right, *tree_height);
        from_top = match visibility_count.from_top.get(i) {
            Some(from_top) => {
                let r = *from_top;
                r.clone()
            },
            None => {
                from_top = ForwardCounting {
                    visible_count: 0,
                    tallest_height: 0,
                };
                visibility_count.from_top.push(from_top);
                let from_top = visibility_count.from_top.get(i).unwrap();
                let r = *from_top;
                r.clone()
            },
        };
        from_top = get_forward_visibility_count_details(from_top, *tree_height);
        visibility_count.from_top[i] = from_top.clone();

        from_bottom = match visibility_count.from_bottom.get(i) {
            Some(from_bottom) => {
                let mut r:BackwardCounting = BackwardCounting {
                    visible_heights: vec![],
                };
                from_bottom.clone_into(&mut r);
                r
            },
            None => {
                from_bottom = BackwardCounting {
                    visible_heights: vec![],
                };

                visibility_count.from_bottom.push(from_bottom);
                let from_bottom = visibility_count.from_bottom.get(i).unwrap();
                let mut r:BackwardCounting = BackwardCounting {
                    visible_heights: vec![],
                };
                from_bottom.clone_into(&mut r);
                r
            },
        };
        from_bottom = get_backward_visibility_count_details(from_bottom, *tree_height);
        visibility_count.from_bottom[i] = from_bottom.clone();
    }

    visibility_count.total_visible_from_left += from_left.visible_count;
    visibility_count.total_visible_from_right += from_right.visible_heights.len() as u32;
    visibility_count.total_visible_from_top += from_top.visible_count;
    visibility_count.total_visible_from_bottom += from_bottom.visible_heights.len() as u32;
    visibility_count.from_left_collection.push(from_left);
    visibility_count.from_right_collection.push(from_right);

    visibility_count.visibility_grand_total = visibility_count.total_visible_from_left + visibility_count.total_visible_from_right + visibility_count.total_visible_from_top + visibility_count.total_visible_from_bottom;

    return visibility_count;
}

fn get_forward_visibility_count_details(
    mut foward_counting: ForwardCounting,
    tree_height: u32,
) -> ForwardCounting {
    if foward_counting.visible_count == 0 || tree_height > foward_counting.tallest_height {
        foward_counting.tallest_height = tree_height;
        foward_counting.visible_count += 1;
    }

    return foward_counting;
}

fn get_backward_visibility_count_details(
    mut backward_counting: BackwardCounting,
    tree_height: u32,
) -> BackwardCounting {
    loop {
        let last_heigth = match backward_counting.visible_heights.pop() {
            Some(height) => height,
            None => {
                backward_counting.visible_heights.push(tree_height);
                break;
            }
        };

        if last_heigth <= tree_height {
            continue;
        } else {
            backward_counting.visible_heights.push(last_heigth);
            backward_counting.visible_heights.push(tree_height);
            break;
        }
    }

    return backward_counting;
}
