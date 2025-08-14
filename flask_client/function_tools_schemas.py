state_request_tool_schema = {
    "type": "function",
    "function": {
        "name": "state_request",
        "description": "Requests the current state of the robot",
        "strict": True,
        "parameters": {
            "additionalProperties": False,
            "type": "object",
            "parameters": {"type": "object", "properties": {}, "required": []},
        },
    },
}

photo_request_tool_schema = {
    "type": "function",
    "function": {
        "name": "photo_request",
        "description": "Requests a Photo from the 'electric eye' on the robot",
        "strict": True,
        "parameters": {
            "additionalProperties": False,
            "type": "object",
            "parameters": {"type": "object", "properties": {}, "required": []},
        },
    },
}

store_route_request_tool_schema = {
    "type": "function",
    "function": {
        "name": "store_routes_request",
        "description": "Stores one or more routes with start position name, route steps, and destination name. Try to satisfy the restriction : list[n].destination_position_name == list[n+1].start_position_name",
        "strict": True,
        "parameters": {
            "additionalProperties": False,
            "type": "object",
            "properties": {
                "routes": {
                    "type": "array",
                    "description": "List of routes to store.",
                    "items": {
                        "type": "object",
                        "properties": {
                            "StoreRoute": {
                                "type": "object",
                                "properties": {
                                    "start_position_name": {
                                        "type": "string",
                                        "description": "Name of the starting position.",
                                    },
                                    "route": {
                                        "type": "array",
                                        "description": "List of movement steps.",
                                        "items": {
                                            "type": "object",
                                            "properties": {
                                                "direction_type": {
                                                    "type": "string",
                                                    "description": "Directions that are either NoMovement | Forward | Right | Backward | Left | RotateLeft | RotateRight ",
                                                },
                                                "value": {
                                                    "type": "number",
                                                    "description": "Value associated with the direction (e.g., distance or angle).",
                                                },
                                            },
                                            "required": ["direction_type", "value"],
                                        },
                                    },
                                    "destination_position_name": {
                                        "type": "string",
                                        "description": "Name of the destination position.",
                                    },
                                },
                                "required": [
                                    "start_position_name",
                                    "route",
                                    "destination_position_name",
                                ],
                            }
                        },
                        "required": ["StoreRoute"],
                    },
                }
            },
            "required": ["routes"],
        },
    },
}

define_home_request_tool_schema = {
    "type": "function",
    "function": {
        "name": "define_home",
        "description": "Defines the home position with coordinates and orientation.",
        "strict": True,
        "parameters": {
            "additionalProperties": False,
            "type": "object",
            "properties": {
                "DefineHome": {
                    "type": "object",
                    "properties": {
                        "home_x": {
                            "type": "number",
                            "description": "X coordinate of the home position.",
                        },
                        "home_y": {
                            "type": "number",
                            "description": "Y coordinate of the home position.",
                        },
                        "home_theta": {
                            "type": "number",
                            "description": "Orientation angle at the home position, in degrees.",
                        },
                    },
                    "required": ["home_x", "home_y", "home_theta"],
                }
            },
            "required": ["DefineHome"],
        },
    },
}

tools = [
    state_request_tool_schema,
    photo_request_tool_schema,
    store_route_request_tool_schema,
    define_home_request_tool_schema,
]


def call_function(name, args):
    if name == "state_request":
        return
