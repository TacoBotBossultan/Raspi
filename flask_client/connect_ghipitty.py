import httpx 
from openai import OpenAI
import os
from dotenv import load_dotenv
from openai.types.chat import ChatCompletionMessageParam, ChatCompletionToolUnionParam
import json

load_dotenv()


httpx_client = httpx.Client(http2=True, verify=False)
client = OpenAI(
    api_key=os.environ.get("ASK_AI_API_KEY"),
    base_url="https://ask.ai.stratec.com/api/v1/stratec/openai",
    http_client=httpx_client
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

# Tool-Call Example
tool_schema = {
    "type": "function",
    "function": {
        "name": "get_weather",
        "description": "Provides the current weather in a city",
        "parameters": {
            "type": "object",
            "properties": {
                "city": {
                    "type": "string",
                    "description": "name of the city"
                }
            },
            "required": ["city"]
        }
    }
}


def query_ghiptty(prompt: str) -> str | None:
    response = client.chat.completions.create(
  model="gpt-4o-mini",
  messages=[
    {"role": "developer", "content": "Esti un nibun"},
    {"role": "user", "content": prompt }
  ]
)
    return response.choices[0].message.content

def zi_vremea(user_input: str):
    # First request — let model decide if it wants to call tool
    response = client.chat.completions.create(
        model="gpt-4o-mini",
        messages=[
            {"role": "system", "content": "You can use the get_weather tool if needed."},
            {"role": "user", "content": user_input}
        ],
        tools=[tool_schema],
        tool_choice="auto"
    )

    message = response.choices[0].message

    if message.tool_calls:
        # Model requested a tool call
        tool_call = message.tool_calls[0]
        args = json.loads(tool_call.function.arguments)

        # Execute your Python function
        result = get_weather(**args)

        # Append result as tool output and ask model for final answer
        follow_up = client.chat.completions.create(
            model="gpt-4o-mini",
            messages=[
                {"role": "system", "content": "You can use the get_weather tool if needed."},
                {"role": "user", "content": user_input},
                {"role": "assistant", "tool_calls": message.tool_calls},
                {"role": "tool", "tool_call_id": tool_call.id, "content": json.dumps(result)}
            ],
            tools=[tool_schema]
        )
        return follow_up.choices[0].message.content

    else:
        # No tool used — model answered directly
        return message.content

