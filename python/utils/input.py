from pathlib import Path
import os
import urllib.request


def _load_session_id() -> str:
    res = os.getenv("AOC_SESSION_ID")
    if not res:
        raise Exception("Set the AOC_SESSION_ID environment variable if you want to download inputs.")
    return res

FIRST_DAY = 1
LAST_DAY = 12
INPUT_DIR = Path("/tmp/aoc26")

HEADERS = {}
COOKIES = {"session": _load_session_id()}

def get_input(day: int) -> str:
    if not (FIRST_DAY <= day <= LAST_DAY):
        raise Exception(f"day must be between {FIRST_DAY} and {LAST_DAY}")

    file_path = INPUT_DIR / f"{day:02d}.txt"

    if not file_path.exists():
        fetched = _fetch_remote(day)
        print("Fetched lines: ", len(fetched))
        os.makedirs(INPUT_DIR, exist_ok=True)
        with open(file_path, "w") as f:
            f.write(fetched)

    return open(file_path).read()


def _fetch_remote(day: int) -> str:
    url = f"https://adventofcode.com/2025/day/{day}/input"

    # Build the request with headers and cookies
    req = urllib.request.Request(
        url,
        headers={
            **HEADERS,
            "Cookie": "; ".join(f"{k}={v}" for k, v in COOKIES.items())
        }
    )

    print(f"Fetching request: {req.full_url}")

    # Fetch the content
    with urllib.request.urlopen(req) as response:
        if response.status != 200:
            raise RuntimeError(
                f"Request failed\n\tstatus code: {response.status}\n\tmessage: {response.read().decode()}"
            )
        return response.read().decode().strip()
