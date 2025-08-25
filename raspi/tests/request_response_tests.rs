use raspi::{
    mission_controller::missions::ActionType,
    navigation_computing::nav_computer_states::Direction::{Forward, Right, RotateLeft},
    request_response::{
        requests::{DefineHomeRequest, MissionRequest, PhotoRequest, Requests, StateReqest, StoreRouteRequest},
        responses::{
            GeneralResponse, PhotoResponse, Responses,
            RobotStates::{Busy, Free},
            StateResponse,
        },
    },
};

// in mare parte vreau saami verific assumptionu de cum ar trebui sa arate jsonaele alea

/// requesturile
#[test]
fn deser_photo_request() {
    let req = r#"
    {
        "Photo": null
    } 
"#;

    // let req_res: Result<Requests, Error> = serde_json::from_str(req);

    let req: Requests = match serde_json::from_str(req) {
        Ok(req) => req,
        Err(e) => {
            panic!("Vezi ca nu s-a deser frumus {e:#?}");
        }
    };

    assert_eq!(req, Requests::PhotoRequest(PhotoRequest::new()));
}

#[test]
fn deser_robot_state_request() {
    let req = r#"
    {
        "State": null
    } 
"#;

    // let req_res: Result<Requests, Error> = serde_json::from_str(req);

    let req: Requests = match serde_json::from_str(req) {
        Ok(req) => req,
        Err(e) => {
            panic!("Vezi ca nu s-a deser frumus {e:#?}");
        }
    };

    assert_eq!(req, Requests::StateRequest(StateReqest::new()));
}

#[test]
fn deser_define_home_request() {
    let req = r#"
{
  "DefineHome": {
    "home_x": 200,
    "home_y": 200,
    "home_theta": 90
  }
}
    "#;

    // let req_res: Result<Requests, Error> = serde_json::from_str(req);

    let req: Requests = match serde_json::from_str(req) {
        Ok(req) => req,
        Err(e) => {
        }
        panic!("Vezi ca nu s-a deser frumus {e:#?}");
    };

    assert_eq!(req, Requests::DefineHomeRequest(DefineHomeRequest::new(200, 200, 90)));
}

#[test]
fn deser_store_route_request() {
    let req = r#"
{
  "StoreRoute": {
    "start_position_name": "Home",
    "route": [
      {
        "direction_type": "Forward",
        "value": 100
      },
      {
        "direction_type": "Right",
        "value": 100
      },
      {
        "direction_type": "RotateLeft",
        "value": 90
      }
    ],
    "destination_position_name": "acolo"
  }
}
    "#;

    let req: Requests = match serde_json::from_str(req) {
        Ok(req) => req,
        Err(e) => {
            panic!("Vezi ca nu s-a deser frumus {e:#?}");
        }
    };

    let route = [(Forward, 100), (Right, 100), (RotateLeft, 90)];

    assert_eq!(
        req,
        Requests::StoreRouteRequest(StoreRouteRequest::new(
            "Home".to_string(),
            StoreRouteHelper::route_arr_to_vecdeque(route),
            "acolo".to_string()
        ))
    );
}

#[test]
fn deser_mission_request() {
    let req = r#"
{
  "MissionRequest": {
    "action": "TakePhoto",
    "route": {
      "start_name": "Home",
      "destination_name": "acolo"
    }
  }
}
    "#;

    let req: Requests = match serde_json::from_str(req) {
        Ok(req) => req,
        Err(e) => {
            panic!("Vezi ca nu s-a deser frumus {e:#?}");
        }
    };

    assert_eq!(
        req,
        Requests::MissionRequest(MissionReqest::new(
            ActionType::TakePhoto,
            "Home".to_string(),
            "acolo".to_string()
        ))
    );
}

//responsurile

#[test]
fn serialise_robot_state_response_free() {
    let response = Responses::StateResponse(StateResponse::new(Free));
    let expected_json = r#"
        {
        "StateResponse": {
            "state": "Free"
            }
        }
    "#;

    let no_whitespace_expected_json: String = expected_json.split_whitespace().collect();
    let resposne = serde_json::to_string(&response).unwrap();
    assert_eq!(resposne, no_whitespace_expected_json);
}

#[test]
fn serialise_robot_state_response_busy() {
    let response = Responses::StateResponse(StateResponse::new(Busy));
    let expected_json = r#"
        {
        "StateResponse": {
            "state": "Busy"
            }
        }
    "#;

    let no_whitespace_expected_json: String = expected_json.split_whitespace().collect();
    let resposne = serde_json::to_string(&response).unwrap();
    assert_eq!(resposne, no_whitespace_expected_json);
}

#[test]
fn serialise_photo_response() {
    let response = Responses::PhotoResponse(PhotoResponse::new(vec![1, 2, 3, 4, 5]));
    let expected_json = r#"
        {
        "PhotoResponse": {
            "photo_data": [
                    1,
                    2,
                    3,
                    4,
                    5
                ]
            }
        }
    "#;

    let no_whitespace_expected_json: String = expected_json.split_whitespace().collect();
    let resposne = serde_json::to_string(&response).unwrap();
    assert_eq!(resposne, no_whitespace_expected_json);
}

#[test]
fn stress_test_serialise_photo_response() {
    let photo_data_arr = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51,
    ];

    let response = Responses::PhotoResponse(PhotoResponse::new(photo_data_arr));
    let expected_json = r#"
        {
        "PhotoResponse": {
            "photo_data": [
                    1,
                    2,
                    3,
                    4,
                    5,
                    6,
                    7,
                    8,
                    9,
                    10,
                    11,
                    12,
                    13,
                    14,
                    15,
                    16,
                    17,
                    18,
                    19,
                    20,
                    21,
                    22,
                    23,
                    24,
                    25,
                    26,
                    27,
                    28,
                    29,
                    30,
                    31,
                    32,
                    33,
                    34,
                    35,
                    36,
                    37,
                    38,
                    39,
                    40,
                    41,
                    42,
                    43,
                    44,
                    45,
                    46,
                    47,
                    48,
                    49,
                    50,
                    51
                ]
            }
        }
    "#;

    let no_whitespace_expected_json: String = expected_json.split_whitespace().collect();
    let resposne = serde_json::to_string(&response).unwrap();
    assert_eq!(resposne, no_whitespace_expected_json);
}

#[test]
fn serialise_ack_response() {
    let response = Responses::GeneralResponse(GeneralResponse::new(200, "OK".to_string()));
    let expected_json = r#"
        {
        "GeneralResponse": {
            "status": 200,
            "message": "OK"
            }
        }
    "#;

    let trimmed_exp_json: String = expected_json.split_whitespace().collect();
    let resposne = serde_json::to_string(&response).unwrap();
    assert_eq!(resposne, trimmed_exp_json);
}
