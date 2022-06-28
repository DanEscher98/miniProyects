import subprocess
import requests

token = "2138473699:AAEAZ0aSgq_hEIYqLQcu25txBzbaWyUEPtg"
api_address = f"https://api.telegram.org/bot{token}"


def system_call(commands):
    pipe = subprocess.Popen(commands,
                            stdout=subprocess.PIPE)
    return pipe.stdout.read()


if __name__ == '__main__':
    # id = system_call(["curl {address}/getUpdates 2> /dev/null",
    #                 "jq -r '.result[].message.chat.id'"])
    # print(system_call(["pwd --version | wc -l"]))
    print(api_address + "/getUpdates")
    resp = requests.get(api_address + "/getUpdates")
    print(resp.content)
