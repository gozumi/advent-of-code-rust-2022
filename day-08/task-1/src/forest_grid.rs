#[derive(Copy, Clone, Debug)]
struct ForwardCounting {
    visible_count: u32,
    tallest_height: u32,
}


#[derive(Debug)]
struct BockwardCounting {
    visible_heights: Vec<u32>,
}

#[derive(Debug)]
pub struct VisibilityCountStruct {
    from_left_collection: Vec<ForwardCounting>,
    from_right_collection: Vec<BockwardCounting>,
    from_top: Vec<ForwardCounting>,
    // from_bottom: Vec<BockwardCounting>,
    total_visible_from_left: u32,
    total_visible_from_right: u32,
    total_visible_from_top: u32,
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
                // from_bottom: vec![],
                total_visible_from_left: 0,
                total_visible_from_right: 0,
                total_visible_from_top: 0,
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

    let mut from_right = BockwardCounting {
        visible_heights: vec![],
    };

    let mut from_top: ForwardCounting = ForwardCounting {
        visible_count: 0,
        tallest_height: 0,
    };

    for (i, tree_height) in grid_row.iter().enumerate() {
        from_left = get_forward_visibility_count_details(from_left, *tree_height);
        from_right = get_right_visibility_count_details(from_right, *tree_height);
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
    }

    visibility_count.total_visible_from_left += from_left.visible_count;
    visibility_count.total_visible_from_right += from_right.visible_heights.len() as u32;
    visibility_count.total_visible_from_top += from_top.visible_count;
    visibility_count.from_left_collection.push(from_left);
    visibility_count.from_right_collection.push(from_right);
    visibility_count.from_left_collection.push(from_top);

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

fn get_right_visibility_count_details(
    mut from_right: BockwardCounting,
    tree_height: u32,
) -> BockwardCounting {
    loop {
        let last_heigth = match from_right.visible_heights.pop() {
            Some(height) => height,
            None => {
                from_right.visible_heights.push(tree_height);
                break;
            }
        };

        if last_heigth <= tree_height {
            continue;
        } else {
            from_right.visible_heights.push(last_heigth);
            from_right.visible_heights.push(tree_height);
            break;
        }
    }

    return from_right;
}
