import httpx
from openai import OpenAI
import os
from dotenv import load_dotenv
import json

load_dotenv()


httpx_client = httpx.Client(http2=True, verify=False)
client = OpenAI(
    api_key=os.environ.get("ASK_AI_API_KEY"),
    base_url="https://ask.ai.stratec.com/api/v1/stratec/openai",
    http_client=httpx_client,
)

# models available (all have vision capability)
# gpt-4.1 -> best non reasoning model
# gpt-4.1-mini -> good compromise between latency and speed
# gpt-4.1-nano -> weakest but best latency , 1 mio context lengh
# o3 -> powerful reasoning model slow but great at complex tasks like planning
# o4-mini -> reasoning model , little faster than o3
# gpt-4o-mini -> cheaper than 4.1 mini , stronger than 4.1 nano


def get_weather(city: str):
    return "the weather is rainy in " + city


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

tools = [
    state_request_tool_schema,
    photo_request_tool_schema,
    store_route_request_tool_schema,
]


def query_ghiptty(prompt: str) -> str | None:
    response = client.chat.completions.create(
        model="gpt-4o-mini",
        messages=[
            {"role": "developer", "content": "Esti un nibun"},
            {"role": "user", "content": prompt},
        ],
    )
    return response.choices[0].message.content


def call_function(name, args):
    if name == "get_weather":
        return get_weather(**args)


def jesus_take_the_wheel(user_input: str):
    messages = [
        {"role": "system", "content": "You can use the get_weather tool if needed."},
        {"role": "user", "content": user_input},
    ]

    response = client.chat.completions.create(
        model="gpt-4o-mini",
        messages=messages,
        tool_choice={
            "type": "allowed_tools",
            "parallel_tool_calls": "false",
            "mode": "auto",
            "tools": tools,
        },
    )

    gpt_message = response.choices[0].message

    print("raspuns gipttty:", gpt_message)

    if gpt_message.tool_calls:
        # Model requested a tool call
        for tool_call in gpt_message.tool_calls:
            name = tool_call.function.name
            args = json.loads(tool_call.function.arguments)

            print("vremea:", args)
            result = call_function(name, args)

            messages.append(
                {"role": "tool", "tool_call_id": tool_call.id, "content": str(result)}
            )

            messages.append(
                {
                    "role": "system",
                    "content": "Generate final response based on the results from the tool call",
                },
            )

            print("vremea:", result)

            # Append result as tool output and ask model for final answer
            follow_up = client.chat.completions.create(
                model="gpt-4o-mini",
                messages=messages,
            )
            return follow_up.choices[0].message.content

    else:
        # No tool used — model answered directly
        return gpt_message.content
