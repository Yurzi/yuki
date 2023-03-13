/*
 * Some utils for this project
 */

// Attention, this function is not really safe for all type, like f32 or f64.
pub fn is_in_scope<T: std::cmp::PartialOrd + Copy>(
    var: T,
    left: T,
    right: T,
    left_eq: bool,
    right_eq: bool,
) -> bool {
    let mut res: bool = true;
    let left_res = if left_eq { var >= left } else { var > left };
    let right_res = if right_eq { var <= right } else { var < right };
    // return result
    res = res & left_res & right_res;
    res
}
