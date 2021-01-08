import requests
import uuid

post_body = {
    "meta": {
        "session_id": "d69659e8-db94-4cb0-8701-be377842564d",
        "client_id": "d69659e8-db94-4cb0-8701-be377842564d",
        "page_load_id": str(uuid.uuid4()),
        "recorder_version": 1,
        "user_agent": "meeshkan_tests",
        "language": "en-GB",
    },
    "events": [
        {
            "target": {"nodeName": "#document"},
            "type": "readystatechange",
            "isTrusted": True,
            "timeStamp": 49.404999997932464,
            "bubbles": False,
            "now": 1606774798304,
            "documentUrl": "https://example.com"
        }
    ],
}

# Call /store endpoint:
response = requests.post("https://cujibff5f2.execute-api.eu-west-1.amazonaws.com/citest/store", json=post_body)
print(f"Response text: {response.text}")

