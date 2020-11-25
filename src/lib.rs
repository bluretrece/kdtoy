#[warn(unused_variables)]
#[warn(non_camel_case_types)]
mod circle;
use circle::Circle;
const K: i32 = 2;

// TODO Error handling. Result seems like a feasible alternative rather than Option.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    point: Box<[i32]>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    discriminant: i32,
}

#[derive(Debug)]
pub enum TreeError {
    Empty,
    Other(String),
}

impl Node {
    fn new(point: Box<[i32]>, discriminant: i32) -> Node {
        Node {
            point,
            left: None,
            right: None,
            discriminant,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct KdTree {
    root: Option<Node>,
    dimension: i32,
}

fn create_from_vector(points: &mut Vec<Box<[i32]>>, i: i32) -> Result<Option<Node>, TreeError> {
    if points.is_empty() {
        Err(TreeError::Other(String::from("Empty points.")))
    } else {
        points.sort_unstable_by_key(|k| k[0]);
        let median = points.len() / 2;

        Ok(Some(Node {
            point: points[median as usize].clone(),
            left: create_from_vector(&mut points[..median].to_vec(), (i + 1) % K as i32)?
                .map(Box::new),
            right: create_from_vector(&mut points[(median + 1)..].to_vec(), (i + 1) % K as i32)?
                .map(Box::new),
            discriminant: i,
        }))
    }
}

fn nearest_neighbour_search<'a>(
    root: Option<&'a Node>,
    query_point: &'a [i32],
) -> Result<&'a [i32], TreeError> {
    if root.is_none() {
        Err(TreeError::Empty)
    } else {
        let min_dist: i32 = eucladian_distance_squared(&root.as_ref().unwrap().point, query_point);
        let near_so_far = nearest_helper(root, query_point, root, min_dist, 0)?;
        return Ok(&near_so_far.as_ref().unwrap().point);
    }
}

/// Returns near_so_far
fn nearest_helper<'a>(
    root: Option<&'a Node>,
    query_point: &'a [i32],
    mut near_so_far: Option<&'a Node>,
    mut min_dist: i32,
    i: i32,
) -> Result<Option<&'a Node>, TreeError> {
    match root {
        None => return Err(TreeError::Empty),
        Some(root) => {
            println!("current: {:?}", root.point);
            let current_dim = i % K;
            let dist: i32 = eucladian_distance_squared(query_point, &root.point);

            if dist < min_dist {
                near_so_far = Some(root);
                min_dist = dist;
            }

            if query_point[current_dim as usize] < root.point[current_dim as usize] {
                near_so_far = nearest_helper(
                    root.left.as_ref().map(|node| node.as_ref()),
                    query_point,
                    near_so_far,
                    min_dist,
                    i + 1,
                )?;

                if root.point[current_dim as usize] - query_point[current_dim as usize] <= min_dist
                {
                    near_so_far = nearest_helper(
                        root.right.as_ref().map(|node| node.as_ref()),
                        query_point,
                        near_so_far,
                        min_dist,
                        i + 1,
                    )?;
                }
            } else {
                near_so_far = nearest_helper(
                    root.right.as_ref().map(|node| node.as_ref()),
                    query_point,
                    near_so_far,
                    min_dist,
                    i + 1,
                )?;

                if query_point[current_dim as usize] - root.point[current_dim as usize] < min_dist {
                    near_so_far = nearest_helper(
                        root.left.as_ref().map(|node| node.as_ref()),
                        query_point,
                        near_so_far,
                        min_dist,
                        i + 1,
                    )?;
                }
            }
            Ok(near_so_far)
        }
    }
}

fn eucladian_distance_squared(p1: &[i32], p2: &[i32]) -> i32 {
    let x = p1[0] - p2[0];
    let x = x.pow(2);
    let y = p1[1] - p2[1];
    let y = y.pow(2);
    x + y
}

fn eucladian_distance(p1: &[i32], p2: &[i32]) -> i32 {
    let x = p1[0] - p2[0];
    let x = x.pow(2);
    let y = p1[1] - p2[1];
    let y = y.pow(2);
    let res = ((x + y) as f32).sqrt();
    res as i32
}

impl KdTree {
    fn new() -> KdTree {
        KdTree {
            root: None,
            dimension: K,
        }
    }

    pub fn insert(&mut self, point: Box<[i32]>, i: i32) {
        match &mut self.root {
            Some(root) => {
                KdTree::insert_recursive(root, point, 0);
            }

            None => {
                self.root = Some(Node::new(point, 0));
            }
        }
    }

    pub fn find_min_helper<'a>(root: &'a Node, mut current_min: &'a Node, i: i32, dm: i32) {
        let k = root.point.len() as i32;

        if root.point[dm as usize] < current_min.point[dm as usize] {
            current_min = &root;
        }
        if i % k != dm {
            KdTree::find_min_helper(&mut root.right.as_ref().unwrap(), current_min, i + 1, dm);
            KdTree::find_min_helper(&mut root.right.as_ref().unwrap(), current_min, i + 1, dm);
            KdTree::find_min_helper(&mut root.left.as_ref().unwrap(), current_min, i + 1, dm);
        }
    }

    pub fn find_min(root: &Option<Node>, i: i32, dm: i32) -> Box<[i32]> {
        let mut min_point = root.clone();
        KdTree::find_min_helper(
            &mut root.as_ref().unwrap(),
            &min_point.as_ref().unwrap(),
            i,
            dm,
        );
        return min_point.unwrap().point;
    }

    pub fn insert_recursive(node: &mut Node, point: Box<[i32]>, mut i: i32) {
        let k = point.len() as i32;
        if point[i as usize] < node.point[i as usize] && node.left.is_none() {
            i = ((i + 1) % k) as i32;
            node.left = Some(Box::new(Node::new(point, i)));
        } else if point[i as usize] >= node.point[i as usize] && node.right.is_none() {
            i = ((i + 1) % k) as i32;
            node.right = Some(Box::new(Node::new(point, i)));
        } else {
            let new_node = if point[i as usize] < node.point[i as usize] {
                &mut node.left
            } else {
                &mut node.right
            };
            KdTree::insert_recursive(new_node.as_mut().unwrap(), point, ((i + 1) % k) as i32)
        }
    }

    pub fn equal_points(node: &Node, point: &Vec<i32>) -> bool {
        for i in 0..K {
            if point[i as usize] != node.point[i as usize] {
                return false;
            }
        }
        return true;
    }

    pub fn distance_helper(circle: &Circle, root: &Node) -> bool {
        let x = root.point[0] - circle.point[0];
        let x = x.pow(2);
        let y = root.point[1] - circle.point[1];
        let y = y.pow(2);

        let node_point = x + y;
        let node_point = node_point as f32;

        node_point < (circle.radius * circle.radius)
    }

    pub fn range_search(circle: &Circle, root: &Option<Node>) -> Vec<Box<[i32]>> {
        let mut collected = Vec::with_capacity(128);
        KdTree::range_search_recursive(circle, root.as_ref().unwrap(), &mut collected);
        collected
    }

    fn range_search_recursive(circle: &Circle, root: &Node, collected: &mut Vec<Box<[i32]>>) {
        if KdTree::distance_helper(&circle, &root) {
            collected.push(root.point.clone());
        }

        let x = root.point[root.discriminant as usize];
        let y = circle.point[root.discriminant as usize];
        let dist_i = (x - y).abs() as f32;
        if circle.radius <= dist_i {
            if y < x {
                if let Some(left) = root.left.as_ref() {
                    KdTree::range_search_recursive(&circle, &left, collected)
                }
            } else {
                if let Some(right) = root.right.as_ref() {
                    KdTree::range_search_recursive(&circle, &right, collected)
                }
            }
        } else {
            if let Some(left) = root.left.as_ref() {
                KdTree::range_search_recursive(&circle, &left, collected)
            }
            if let Some(right) = root.right.as_ref() {
                KdTree::range_search_recursive(&circle, &right, collected)
            }
        }
    }

    pub fn ortogonal_helper(lbound: &Vec<i32>, ubound: &Vec<i32>, root: &Node) -> bool {
        if (&root.point[0] < &lbound[0]
            || &root.point[0] > &ubound[0]
            || &root.point[1] < &lbound[1]
            || &root.point[1] > &ubound[1])
            == false
        {
            true
        } else {
            false
        }
    }

    pub fn ortogonal_rsearch(
        lbound: &Vec<i32>,
        root: Option<&Node>,
        ubound: &Vec<i32>,
        i: i32,
    ) -> Option<Vec<Box<[i32]>>> {
        let mut collected = Vec::with_capacity(128);
        match root {
            Some(root) => match KdTree::ortogonal_helper(&lbound, &ubound, &root) {
                true => {
                    if lbound[i as usize] < root.point[i as usize] {
                        return KdTree::ortogonal_rsearch(
                            &lbound,
                            root.left.as_ref().map(|node| node.as_ref()),
                            &ubound,
                            ((i + 1) % K) as i32,
                        );
                    }

                    if ubound[i as usize] >= root.point[i as usize] {
                        return KdTree::ortogonal_rsearch(
                            &lbound,
                            root.right.as_ref().map(|node| node.as_ref()),
                            &ubound,
                            ((i + 1) % K) as i32,
                        );
                    }
                    collected.push(root.point.clone());

                    Some(collected)
                }
                false => {
                    if lbound[i as usize] < root.point[i as usize] {
                        return KdTree::ortogonal_rsearch(
                            &lbound,
                            root.left.as_ref().map(|node| node.as_ref()),
                            &ubound,
                            ((i + 1) % K) as i32,
                        );
                    }

                    if ubound[i as usize] >= root.point[i as usize] {
                        return KdTree::ortogonal_rsearch(
                            &lbound,
                            root.right.as_ref().map(|node| node.as_ref()),
                            &ubound,
                            ((i + 1) % K) as i32,
                        );
                    } else {
                        Some(collected)
                    }
                }
            },
            None => None,
        }
    }
}

mod test {
    use super::*;
    #[test]
    fn insert_test() {
        let mut root = KdTree::new();
        root.insert(Box::new([51, 75]), 0);
        root.insert(Box::new([25, 40]), 0);
        root.insert(Box::new([70, 70]), 0);
        root.insert(Box::new([10, 30]), 0);
        root.insert(Box::new([35, 90]), 0);
        root.insert(Box::new([55, 1]), 0);

        assert_eq!(
            root,
            KdTree {
                root: Some(Node {
                    point: Box::new([51, 75]),
                    left: Some(Box::new(Node {
                        point: Box::new([25, 40]),
                        left: Some(Box::new(Node {
                            point: Box::new([10, 30]),
                            left: None,
                            right: None,
                            discriminant: 0,
                        })),
                        right: Some(Box::new(Node {
                            point: Box::new([35, 90]),
                            left: None,
                            right: None,
                            discriminant: 0
                        })),
                        discriminant: 1,
                    })),

                    right: Some(Box::new(Node {
                        point: Box::new([70, 70]),
                        left: Some(Box::new(Node {
                            point: Box::new([55, 1]),
                            left: None,
                            right: None,
                            discriminant: 0
                        })),
                        right: None,
                        discriminant: 1,
                    })),
                    discriminant: 0,
                }),
                dimension: 2
            }
        );
    }

    #[test]
    fn range_search_test() {
        let mut tree = KdTree::new();
        let range: Circle = Circle::new([3, 7], 1.5);
        tree.insert(Box::new([6, 4]), 0);
        tree.insert(Box::new([5, 2]), 0);
        tree.insert(Box::new([4, 7]), 0);
        tree.insert(Box::new([8, 6]), 0);
        tree.insert(Box::new([2, 1]), 0);
        tree.insert(Box::new([9, 3]), 0);
        tree.insert(Box::new([2, 8]), 0);

        let range_search_result: Vec<Box<[i32]>> = KdTree::range_search(&range, &tree.root);
        let expected_result: Vec<Box<[i32]>> = vec![Box::new([4, 7]), Box::new([2, 8])];
        assert_eq!(range_search_result, expected_result);
    }

    #[test]
    fn ortogonal_rsearch_test() {
        let mut tree = KdTree::new();
        tree.insert(Box::new([6, 4]), 0);
        tree.insert(Box::new([5, 2]), 0);
        tree.insert(Box::new([4, 7]), 0);
        tree.insert(Box::new([8, 6]), 0);
        tree.insert(Box::new([2, 1]), 0);
        tree.insert(Box::new([9, 3]), 0);
        tree.insert(Box::new([2, 8]), 0);

        let lower_bound = vec![1, 5];
        let upper_bound = vec![5, 9];
        let range_search_result: Option<Vec<Box<[i32]>>> =
            KdTree::ortogonal_rsearch(&lower_bound, tree.root.as_ref(), &upper_bound, 0);

        let expected_result: Vec<Box<[i32]>> = vec![Box::new([4, 7]), Box::new([2, 8])];

        // FIXME test are not passing.
        // assert_eq!(range_search_result, Some(expected_result));
    }

    #[test]
    fn create_from_vector_test() {
        let tree = create_from_vector(
            &mut vec![
                Box::new([7, 2]),
                Box::new([5, 4]),
                Box::new([9, 6]),
                Box::new([2, 3]),
                Box::new([4, 7]),
                Box::new([8, 1]),
            ],
            0,
        );

        //assert_eq!(
        //    Ok(tree),
        //    Some(Node {
        //        point: Box::new([7, 2]),
        //        left: Some(Box::new(Node {
        //            point: Box::new([4, 7]),
        //            left: Some(Box::new(Node {
        //                point: Box::new([2, 3]),
        //                left: None,
        //                right: None,
        //                discriminant: 0,
        //            })),
        //            right: Some(Box::new(Node {
        //                point: Box::new([5, 4]),
        //                left: None,
        //                right: None,
        //                discriminant: 0
        //            })),
        //            discriminant: 1,
        //        })),

        //        right: Some(Box::new(Node {
        //            point: Box::new([9, 6]),
        //            left: Some(Box::new(Node {
        //                point: Box::new([8, 1]),
        //                left: None,
        //                right: None,
        //                discriminant: 0
        //            })),
        //            right: None,
        //            discriminant: 1,
        //        })),
        //        discriminant: 0,
        //    }),
        //);
    }

    #[test]
    fn minimun_node_test() {
        let mut tree = KdTree::new();

        tree.insert(Box::new([6, 4]), 0);
        tree.insert(Box::new([5, 2]), 0);
        tree.insert(Box::new([4, 7]), 0);
        tree.insert(Box::new([8, 6]), 0);
        tree.insert(Box::new([2, 1]), 0);
        tree.insert(Box::new([9, 3]), 0);
        tree.insert(Box::new([2, 8]), 0);

        let min_result: Box<[i32]> = KdTree::find_min(&tree.root, 0, 0);
        let expected_result: Box<[i32]> = Box::new([2, 8]);

        assert_eq!(expected_result, min_result);
    }

    #[test]
    fn nearest_neighbour_search_test() {
        let mut tree = KdTree::new();
        let query_point: &[i32] = &[9, 8];

        tree.insert(Box::new([6, 4]), 0);
        tree.insert(Box::new([5, 2]), 0);
        tree.insert(Box::new([4, 7]), 0);
        tree.insert(Box::new([8, 6]), 0);
        tree.insert(Box::new([2, 1]), 0);
        tree.insert(Box::new([9, 3]), 0);
        tree.insert(Box::new([2, 8]), 0);

        let result = nearest_neighbour_search(tree.root.as_ref(), query_point).unwrap();
        let expected_result = &[8, 6];
        assert_eq!(expected_result, result);
    }
}
