use raspi::navigation_computing::nav_computer_states::Direction::{
    Backward, Forward, Left, Right, RotateLeft, RotateRight,
};

mod store_route_helper;
use store_route_helper::StoreRouteHelper;

#[test]
fn rotate_left_route() {
    let mut helper = StoreRouteHelper::new();
    let home_location = (100, 100, 0);
    let route = ([(RotateLeft, 90)], "Home", "Target");
    let get_route = ("Home", "Target");
    let expected_absolute_route = [(Some("Home"), 100, 100, 0), (Some("Target"), 100, 100, 270)];

    helper.define_home(home_location);
    assert!(helper.store_route(route).is_ok());
    let get_route_res = helper.get_route(get_route);

    assert!(get_route_res.is_ok());
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn rotate_right_route() {
    let mut helper = StoreRouteHelper::new();
    let home_location = (100, 100, 0);
    let route = ([(RotateRight, 90)], "Home", "Target");
    let get_route = ("Home", "Target");
    let expected_absolute_route = [(Some("Home"), 100, 100, 0), (Some("Target"), 100, 100, 90)];

    helper.define_home(home_location);
    assert!(helper.store_route(route).is_ok());
    let get_route_res = helper.get_route(get_route);

    assert!(get_route_res.is_ok());
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn strafe_left_route() {
    let mut helper = StoreRouteHelper::new();
    let home_location = (100, 100, 0);
    let route = ([(Left, 50)], "Home", "Target");
    let get_route = ("Home", "Target");
    let expected_absolute_route = [(Some("Home"), 100, 100, 0), (Some("Target"), 100, 150, 0)];

    helper.define_home(home_location);
    assert!(helper.store_route(route).is_ok());
    let get_route_res = helper.get_route(get_route);

    assert!(get_route_res.is_ok());
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn strafe_right_route() {
    let mut helper = StoreRouteHelper::new();
    let home_location = (100, 100, 0);
    let route = ([(Right, 50)], "Home", "Target");
    let get_route = ("Home", "Target");
    let expected_absolute_route = [(Some("Home"), 100, 100, 0), (Some("Target"), 100, 50, 0)];

    helper.define_home(home_location);
    assert!(helper.store_route(route).is_ok());
    let get_route_res = helper.get_route(get_route);

    assert!(get_route_res.is_ok());
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn backwards_route() {
    let mut helper = StoreRouteHelper::new();
    let home_location = (100, 100, 0);
    let route = ([(Backward, 50)], "Home", "Target");
    let get_route = ("Home", "Target");
    let expected_absolute_route = [(Some("Home"), 100, 100, 0), (Some("Target"), 50, 100, 0)];

    helper.define_home(home_location);
    assert!(helper.store_route(route).is_ok());
    let get_route_res = helper.get_route(get_route);

    assert!(get_route_res.is_ok());
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn forwards_route() {
    let mut helper = StoreRouteHelper::new();
    let home_location = (100, 100, 0);
    let route = ([(Forward, 50)], "Home", "Target");
    let get_route = ("Home", "Target");
    let expected_absolute_route = [(Some("Home"), 100, 100, 0), (Some("Target"), 150, 100, 0)];

    helper.define_home(home_location);
    assert!(helper.store_route(route).is_ok());
    let get_route_res = helper.get_route(get_route);

    assert!(get_route_res.is_ok());
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn rotate_left_route_reversed() {
    let mut helper = StoreRouteHelper::new();
    let target_location = (Some("Target"), 100, 100, 270);
    let route = ([(RotateLeft, 90)], "Home", "Target");
    let get_route = ("Home", "Target");
    let expected_absolute_route = [(Some("Home"), 100, 100, 0), (Some("Target"), 100, 100, 270)];

    helper.store_position(target_location);
    assert!(helper.store_route(route).is_ok());
    let get_route_res = helper.get_route(get_route);

    assert!(get_route_res.is_ok());
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn rotate_right_route_reversed() {
    let mut helper = StoreRouteHelper::new();
    let target_location = (Some("Target"), 100, 100, 90);
    let route = ([(RotateRight, 90)], "Home", "Target");
    let get_route = ("Home", "Target");
    let expected_absolute_route = [(Some("Home"), 100, 100, 0), (Some("Target"), 100, 100, 90)];

    helper.store_position(target_location);
    assert!(helper.store_route(route).is_ok());
    let get_route_res = helper.get_route(get_route);

    assert!(get_route_res.is_ok());
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn strafe_left_route_reversed() {
    let mut helper = StoreRouteHelper::new();
    let target_location = (Some("Target"), 100, 150, 0);
    let route = ([(Left, 50)], "Home", "Target");
    let get_route = ("Home", "Target");
    let expected_absolute_route = [(Some("Home"), 100, 100, 0), (Some("Target"), 100, 150, 0)];

    helper.store_position(target_location);
    assert!(helper.store_route(route).is_ok());
    let get_route_res = helper.get_route(get_route);

    assert!(get_route_res.is_ok());
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn strafe_right_route_reversed() {
    let mut helper = StoreRouteHelper::new();
    let target_location = (Some("Target"), 100, 50, 0);
    let route = ([(Right, 50)], "Home", "Target");
    let get_route = ("Home", "Target");
    let expected_absolute_route = [(Some("Home"), 100, 100, 0), (Some("Target"), 100, 50, 0)];

    helper.store_position(target_location);
    assert!(helper.store_route(route).is_ok());
    let get_route_res = helper.get_route(get_route);

    assert!(get_route_res.is_ok());
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn backwards_route_reversed() {
    let mut helper = StoreRouteHelper::new();
    let target_location = (Some("Target"), 50, 100, 0);
    let route = ([(Backward, 50)], "Home", "Target");
    let get_route = ("Home", "Target");
    let expected_absolute_route = [(Some("Home"), 100, 100, 0), (Some("Target"), 50, 100, 0)];

    helper.store_position(target_location);
    assert!(helper.store_route(route).is_ok());
    let get_route_res = helper.get_route(get_route);

    assert!(get_route_res.is_ok());
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn forwards_route_reversed() {
    let mut helper = StoreRouteHelper::new();
    let target_location = (Some("Target"), 150, 100, 0);
    let route = ([(Forward, 50)], "Home", "Target");
    let get_route = ("Home", "Target");
    let expected_absolute_route = [(Some("Home"), 100, 100, 0), (Some("Target"), 150, 100, 0)];

    helper.store_position(target_location);
    assert!(helper.store_route(route).is_ok());
    let get_route_res = helper.get_route(get_route);

    assert!(get_route_res.is_ok());
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}
#[test]
fn store_route_home_acolo() {
    let mut helper = StoreRouteHelper::new();
    let home_location = (200, 200, 90);
    let route = (
        [(Forward, 100), (Right, 100), (RotateLeft, 90)],
        "Home",
        "acolo",
    );
    let get_route = ("Home", "acolo");
    let expected_absolute_route = [
        (Some("Home"), 200, 200, 90),
        (None, 200, 100, 90),
        (None, 100, 100, 90),
        (Some("acolo"), 100, 100, 0),
    ];

    helper.define_home(home_location);
    let store_route_res = helper.store_route(route);
    assert!(store_route_res.is_ok());
    let get_route_res = helper.get_route(get_route);

    assert!(get_route_res.is_ok());
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn store_route_home_acolo_invers() {
    let mut helper = StoreRouteHelper::new();
    let acolo_location = (Some("acolo"), 100, 100, 0);
    let route = (
        [(Forward, 100), (Right, 100), (RotateLeft, 90)],
        "Home",
        "acolo",
    );
    let get_route = ("Home", "acolo");
    let expected_absolute_route = [
        (Some("Home"), 200, 200, 90),
        (None, 200, 100, 90),
        (None, 100, 100, 90),
        (Some("acolo"), 100, 100, 0),
    ];

    println!("definitning home");
    helper.store_position(acolo_location);

    println!("Storin route");
    let store_route_res = helper.store_route(route);
    assert!(store_route_res.is_ok());

    println!("Getting the route");
    let get_route_res = helper.get_route(get_route);
    assert!(get_route_res.is_ok());

    println!("getting back the route and seeing if it's equal");
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn test_exotic() {
    let mut helper = StoreRouteHelper::new();
    let home_location = (100, 100, 0);
    let route = ([(RotateLeft, 90)], "Home", "acolo");
    let get_route = ("Home", "acolo");
    let expected_absolute_route = [(Some("Home"), 100, 100, 0), (Some("acolo"), 100, 100, 270)];

    println!("definitning home");
    helper.define_home(home_location);

    println!("Storin route");
    let store_route_res = helper.store_route(route);
    assert!(store_route_res.is_ok());

    println!("Getting the route");
    let get_route_res = helper.get_route(get_route);
    assert!(get_route_res.is_ok());

    println!("getting back the route and seeing if it's equal");
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn test_chiar_mai_exotic() {
    let mut helper = StoreRouteHelper::new();
    let home_location = (100, 100, 270);
    let route = ([(RotateRight, 90)], "Home", "acolo");
    let get_route = ("Home", "acolo");
    let expected_absolute_route = [(Some("Home"), 100, 100, 270), (Some("acolo"), 100, 100, 0)];

    println!("definitning home");
    helper.define_home(home_location);

    println!("Storin route");
    let store_route_res = helper.store_route(route);
    assert!(store_route_res.is_ok());

    println!("Getting the route");
    let get_route_res = helper.get_route(get_route);
    assert!(get_route_res.is_ok());

    println!("getting back the route and seeing if it's equal");
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn test_exotic_reversed() {
    let mut helper = StoreRouteHelper::new();
    let acolo_location = (Some("acolo"), 100, 100, 270);
    let route = ([(RotateLeft, 90)], "Home", "acolo");
    let get_route = ("Home", "acolo");
    let expected_absolute_route = [(Some("Home"), 100, 100, 0), (Some("acolo"), 100, 100, 270)];

    println!("definitning home");
    helper.store_position(acolo_location);

    println!("Storin route");
    let store_route_res = helper.store_route(route);
    assert!(store_route_res.is_ok());

    println!("Getting the route");
    let get_route_res = helper.get_route(get_route);
    assert!(get_route_res.is_ok());

    println!("getting back the route and seeing if it's equal");
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn test_chiar_mai_exotic_resversed() {
    let mut helper = StoreRouteHelper::new();
    let acolo_location = (Some("acolo"), 100, 100, 0);
    let route = ([(RotateRight, 90)], "Home", "acolo");
    let get_route = ("Home", "acolo");
    let expected_absolute_route = [(Some("Home"), 100, 100, 270), (Some("acolo"), 100, 100, 0)];

    println!("definitning home");
    helper.store_position(acolo_location);

    println!("Storin route");
    let store_route_res = helper.store_route(route);
    assert!(store_route_res.is_ok());

    println!("Getting the route");
    let get_route_res = helper.get_route(get_route);
    assert!(get_route_res.is_ok());

    println!("getting back the route and seeing if it's equal");
    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn long_route() {
    let mut helper = StoreRouteHelper::new();
    let home_location = (100, 100, 0);
    let route = (
        [
            (Forward, 1000),
            (Backward, 1100),
            (RotateLeft, 90),
            (RotateRight, 180),
            (Left, 200),
            (RotateLeft, 180),
            (Forward, 2000),
        ],
        "Home",
        "acolo",
    );

    let expected_absolute_route = [
        (Some("Home"), 100, 100, 0),
        (None, 1100, 100, 0),
        (None, 0, 100, 0),
        (None, 0, 100, 270),
        (None, 0, 100, 90),
        (None, 200, 100, 90),
        (None, 200, 100, 270),
        (Some("acolo"), 200, 2100, 270),
    ];

    helper.define_home(home_location);

    let store_route_res = helper.store_route(route);
    assert!(store_route_res.is_ok());

    let get_route_res = helper.get_route(("Home", "acolo"));
    assert!(get_route_res.is_ok());

    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}

#[test]
fn long_route_reversed() {
    let mut helper = StoreRouteHelper::new();
    let acolo_location = (Some("acolo"), 200, 2100, 270);
    let route = (
        [
            (Forward, 1000),
            (Backward, 1100),
            (RotateLeft, 90),
            (RotateRight, 180),
            (Left, 200),
            (RotateLeft, 180),
            (Forward, 2000),
        ],
        "Home",
        "acolo",
    );

    let expected_absolute_route = [
        (Some("Home"), 100, 100, 0),
        (None, 1100, 100, 0),
        (None, 0, 100, 0),
        (None, 0, 100, 270),
        (None, 0, 100, 90),
        (None, 200, 100, 90),
        (None, 200, 100, 270),
        (Some("acolo"), 200, 2100, 270),
    ];

    helper.store_position(acolo_location);

    let store_route_res = helper.store_route(route);
    assert!(store_route_res.is_ok());

    let get_route_res = helper.get_route(("Home", "acolo"));
    assert!(get_route_res.is_ok());

    let route_vector = get_route_res.unwrap();
    let absolute_route = StoreRouteHelper::arr_to_position_vector(expected_absolute_route);
    assert_eq!(absolute_route, route_vector);
}
