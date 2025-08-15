# Raspi

Este raspi.

Pentru run la `flask`

`.env` tre sa arate cv de genu
`flask_client/.env`

```
ASK_AI_API_KEY=LMAO
```

faci rost de el de pe [StratecGPT](https://ask.ai.stratec.com) -> Dreapta sus -> Settings -> Account -> API Keys

```bash
cd flask_client
pip install -r requirements.txt
#pui aici keyu
nvim flask_client/.env
#sa-ti fie viata mai usoara
conf pull
#si acum poti sa folosesti pe scurt:
fr
#care are alias la
clear && flask --app . --debug run
```

SUCCISSS <3333
