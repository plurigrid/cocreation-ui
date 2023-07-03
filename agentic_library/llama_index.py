```python
import requests
import json

llama_index_api_key = "YOUR_LLAMA_INDEX_API_KEY"

def get_data_from_llama_index():
    url = "https://llama-index/api/data"
    headers = {
        "Authorization": f"Bearer {llama_index_api_key}"
    }

    response = requests.get(url, headers=headers)

    if response.status_code != 200:
        handle_error(response)

    data = json.loads(response.text)
    return data

def handle_error(response):
    print(f"Error: {response.status_code}")
    print(f"Message: {response.text}")
    exit(1)
```