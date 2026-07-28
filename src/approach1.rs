use rand::RngExt;use rand::seq::IndexedRandom;
use crate::util::Point;

pub fn solve(max_path_length: usize, plants: &Vec<Point>)
  -> (usize, Vec<Point>, Vec<Vec<Point>>) {
  println!("Allowed max length: {}", max_path_length);
  println!("Number of plants: {}", plants.len());

  let max_considered_k = plants.len();
  let (k, centers, routes) = find_ideal_k(plants, max_considered_k, max_path_length);

  println!("Number of robots: {k}");
  for (i, center) in centers.iter().enumerate() {
    println!("Starting position: X:{}, Y:{}", center.x, center.y);

    // Removing the first and last element, since it is the center itself
    let mut route = routes[i].clone();
    route.remove(0);
    route.remove(route.len() - 1);
    for plant in route {
      println!("Visit plant: X:{}, Y:{}", plant.x, plant.y);
    }
  }
  (k, centers, routes)
}

fn run_greedy(data_points: &Vec<Point>, centers: &Vec<Point>,
              assignment: &Vec<usize>, max_path_length: usize)
  -> Option<Vec<Vec<Point>>> {
  // Assigning each center a list of indices that belong to it
  let mut assigned_point_indices: Vec<Vec<usize>> = vec![Vec::new(); centers.len()];
  for (i, center_index) in assignment.iter().enumerate() {
    assigned_point_indices[*center_index].push(i);
  };

  // This will store a route for each center
  let mut routes: Vec<Vec<Point>> = Vec::with_capacity(centers.len());
  // Now we will fill this list be calculating the ideal path; This is the actual
  // greedy algorithm
  // Iterating over every center to find the best route for each one
  for (i, center) in centers.iter().enumerate() {
    let mut current = *center;
    let mut route =vec![current];
    let mut unvisited = assigned_point_indices[i].clone();

    // Running until no points are left to visit
    while !unvisited.is_empty() {
      let mut min_dis = f32::MAX;
      let mut min_index = 0;
      // Finding the min distance between the current point and all other points
      // in the cluster
      for (x, assigned_index) in unvisited.iter().enumerate() {
        let cur_dis = current.calc_dis_squared(&data_points[*assigned_index]);
        if cur_dis < min_dis {
          min_dis = cur_dis;
          min_index = x;
        }
      }

      let next_point = data_points[unvisited[min_index]].clone();
      current = next_point.clone();
      // Adding it to the route and moving to the next
      route.push(current);
      // Using swap_remove which is O(1) (unlike remove())
      unvisited.swap_remove(min_index);
    }

    // Adding the center itself to the route, since we want to get back to
    // the starting point
    route.push(*center);

    // Running an optimization algorithm over the route
    two_opt(&mut route);

    // Finally calculating the total distance
    let mut cumulative_distance = 0_f32;
    for neighbor_pair in route.windows(2) {
      cumulative_distance += neighbor_pair[0].calc_dis(&neighbor_pair[1]);
    }

    // Checking if the route is valid, if not we can exit early
    if cumulative_distance > max_path_length as f32 {
      return None;
    }
    // Calculated route will be added to the routes
    routes.push(route);
  }

  // After iterating we return the results
  Some(routes)
}

fn two_opt(route: &mut Vec<Point>) {
  // No optimization needed/possible (basically 3 points since start = end)
  if route.len() < 4 {
    return;
  }

  let mut improved = true;

  while improved {
    improved = false;

    // Running for every Point in the route,
    // besides the last one to make sure that i+1 is valid
    for i in 0..route.len() - 2 {
      // Running for all points ahead, starting with the one after the next
      for j in i + 2..route.len() - 1 {
        // Calculating the current distance between the current two pairs
        let current_dist = route[i].calc_dis_squared(&route[i + 1]) +
          route[j].calc_dis_squared(&route[j + 1]);
        // Calculating the distance if we switched the connection between the pairs
        let new_dist = route[i].calc_dis_squared(&route[j]) +
          route[i + 1].calc_dis_squared(&route[j + 1]);

        // If this is an optimization we apply the new route
        if new_dist < current_dist {
          route[i + 1..=j].reverse();
          improved = true;
          // Running again to check if we can apply more optimizations
          break;
        }
      }

      // Running again to check if we can apply more optimizations
      if improved { break; }
    }
  }
}

fn find_ideal_k(data_points: &Vec<Point>, mut max_k: usize, max_path_length: usize)
  -> (usize, Vec<Point>, Vec<Vec<Point>>) {
  let mut best_k = max_k;
  let mut best_centers = Vec::new();
  let mut best_routes = Vec::new();
  let mut min_k = 1;

  // Binary searching for k
  while min_k <= max_k {
    let mid_k = (min_k + max_k) / 2;
    println!("... trying k={mid_k}");

    // Running k-Means
    let mut best_wcss = f32::MAX;
    let mut centers = Vec::new();
    let mut assignment = Vec::new();
    // Running multiple times since k-means++ initialization is in part random
    for _ in 0..3 {
      let (cen, wcss, ass) = run_k_means(data_points, mid_k);
      // Only keeping the best run
      if wcss < best_wcss {
        best_wcss = wcss;
        centers = cen;
        assignment = ass;
      }
    }

    // Creating the route
    let result = run_greedy(data_points, &centers, &assignment, max_path_length);
    match result {
      // If we get a valid result we will try smaller k
      Some(r) => {
        best_k = mid_k;
        best_centers = centers;
        best_routes = r;

        if mid_k == 0 {
          break;
        }
        max_k = mid_k - 1;
      }
      // If not we will try bigger k
      None => {
        min_k = mid_k + 1;
      }
    }
  }

  // Returning the lowest valid result
  (best_k, best_centers, best_routes)
}

fn run_k_means(data_points: &Vec<Point>, centers_count: usize)
  -> (Vec<Point>, f32, Vec<usize>) {
  // Initializing centers using k-means++ approach
  let mut rng = rand::rng();
  let mut centers = Vec::with_capacity(centers_count);
  // First center will be a random data point
  centers.push(data_points.choose(&mut rng).unwrap().clone());
  // Repeating until centers vec contains the required number of centers
  while centers.len() < centers_count {
    let mut distances: Vec<f32> = Vec::new();
    // Computing the minimum squared distance to each center for every data point
    for data_point in data_points.iter() {
      let mut min_distance = f32::MAX;
      for center in centers.iter() {
        let distance = data_point.calc_dis_squared(center);
        if distance < min_distance {
          min_distance = distance;
        }
      }
      distances.push(min_distance);
    }
    let sum = distances.iter().sum();
    let mut ran = if sum == 0_f32 {0_f32} else {rng.random_range(0.0..sum)};
    // Picking a new center based on the distance to the nearest center
    // using weighted random sampling
    for (i, &distance) in distances.iter().enumerate() {
      ran -= distance;
      if ran <= 0.0 {
        centers.push(data_points[i].clone());
        break;
      }
    }
  }

  let data_points_count = data_points.len();
  let mut total_center_change;

  // Main calculation loop
  loop {
    // Resetting total change after each iteration
    total_center_change = 0_f32;

    // Assigning data points to centers
    let mut assignment = vec![0; data_points_count];
    // Iterating over all data points
    for i in 0..data_points_count  {
      let mut min_distance: f32 = f32::MAX;
      // Checking the distance to each center
      for x in 0..centers_count  {
        let cur_dis = data_points[i].calc_dis(&centers[x]);
        // Data point will be assigned to the closest center
        if cur_dis < min_distance {
          min_distance = cur_dis;
          assignment[i] = x;
        }
      }
    }

    // Calculating new centers
    let mut sum_x: Vec<f32> = vec![0_f32; centers_count];
    let mut sum_y: Vec<f32> = vec![0_f32; centers_count];
    let mut count: Vec<i32> = vec![0; centers_count];
    // Calculating the sum of all x and y coordinates of all points of each center
    for i in 0..data_points_count {
      let ass = assignment[i];
      sum_x[ass] += data_points[i].x;
      sum_y[ass] += data_points[i].y;
      count[ass] += 1;
    }
    // Iterating over each center to calculate the new ones
    for i in 0..centers_count {
      // If no data is being assigned to a center, we will reinitialize it
      if count[i] == 0 {
        centers[i] = data_points.choose(&mut rng).unwrap().clone();
        continue;
      }
      // Otherwise the new position will be the average position of
      // all data points assigned to this center
      let new_point = Point::new(sum_x[i] / count[i] as f32, sum_y[i]/ count[i] as f32);
      let old_point = &centers[i];
      total_center_change += (new_point.x - old_point.x).abs();
      total_center_change += (new_point.y - old_point.y).abs();
      centers[i] = new_point;
    }

    // Checking if the total change is small enough to call it a day
    if total_center_change < 1e-2 {
      let mut wcss = 0_f32;
      // Calculating within cluster sum of squares (wcss)
      for i in 0..data_points_count  {
        let center = assignment[i];
        wcss += data_points[i].calc_dis_squared(&centers[center]);
      }

      return (centers, wcss, assignment)
    }
  }
}