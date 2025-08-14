import httpx 
from openai import OpenAI
import os
from dotenv import load_dotenv

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


def query_ghiptty(prompt: str):
    return  client.chat.completions.create(
  model="gpt-4o-mini",
  messages=[
    {"role": "developer", "content": "Esti un nibun"},
    {"role": "user", "content": prompt }
  ]
)
