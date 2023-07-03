```python
from fastapi import FastAPI, HTTPException
import requests
import os

app = FastAPI()

poe_api_key = os.getenv("POE_API_KEY")

@app.get("/get_data_from_poe_api")
def get_data_from_poe_api():
    try:
        response = requests.get(
            "https://poeapi.com/data",
            headers={"Authorization": f"Bearer {poe_api_key}"}
        )
        response.raise_for_status()
        return response.json()
    except requests.exceptions.HTTPError as err:
        raise HTTPException(status_code=400, detail="POE API request failed") from err
    except Exception as err:
        raise HTTPException(status_code=500, detail="An unexpected error occurred") from err
```
