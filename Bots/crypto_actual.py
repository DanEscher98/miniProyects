import time
import schedule
import requests


def bot_sendtext(message):
    bot_token = '2138473699:AAEAZ0aSgq_hEIYqLQcu25txBzbaWyUEPtg'
    chat_ID = '1251238054'
    send_text = 'https://api.telegram.org/bot' + \
        bot_token + '/sendMessage?chat_id=' + \
        chat_ID + '&parse_mode=Markdown&text=' + message
    print(send_text)
    response = requests.get(send_text)
    return response.json()


def report():
    my_balance = 1234
    message = f"Current balance is: *{my_balance}*"
    bot_sendtext(message)


if __name__ == '__main__':
    schedule.every().second.do(report)
    while True:
        schedule.run_pending()
        time.sleep(1)
