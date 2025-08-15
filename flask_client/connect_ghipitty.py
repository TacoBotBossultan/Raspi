import httpx
from openai import OpenAI
import os
from dotenv import load_dotenv
import json
from function_tools_schemas import tools

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


def query_ghiptty(prompt: str) -> str | None:
    response = client.chat.completions.create(
        model="gpt-4o-mini",
        messages=[
            {"role": "developer", "content": "Esti un nibun"},
            {"role": "user", "content": prompt},
        ],
    )
    return response.choices[0].message.content


def jesus_take_the_wheel(user_input: str) -> str | None:
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
