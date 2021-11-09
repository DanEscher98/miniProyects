# pip install python-telegram-bot
from telegram.ext.updater import Updater    # API key
from telegram.update import Update          # invoke when recive msg
from telegram.ext.callbackcontext import CallbackContext
from telegram.ext.commandhandler import CommandHandler
from telegram.ext.messagehandler import MessageHandler
from telegram.ext.filters import Filters

updater = Updater(
    "2138473699:AAEAZ0aSgq_hEIYqLQcu25txBzbaWyUEPtg",
    use_context=True)


def start(update: Update, context: CallbackContext):
    update.message.reply_text("Hello world!")


def helps(update: Update, context: CallbackContext):
    update.message.reply_text("This is a helper.")


def greets(update: Update, context: CallbackContext):
    update.message.reply_text("What's your name?: ")
    chat_id = update.effective_user.id
    update.message.reply_text(f"Hello, {chat_id}!")


def unknown_text(update: Update, context: CallbackContext):
    update.message.reply_text(
        "Sorry I can't recognize you , you said '%s'" % update.message.text)


def unknown(update: Update, context: CallbackContext):
    update.message.reply_text(
        "Sorry '%s' is not a valid command" % update.message.text)


dp = updater.dispatcher
dp.add_handler(CommandHandler('start', start))
dp.add_handler(CommandHandler('help', helps))
dp.add_handler(CommandHandler('greets', greets))
dp.add_handler(MessageHandler(Filters.text, unknown))
dp.add_handler(MessageHandler(Filters.command, unknown))
dp.add_handler(MessageHandler(Filters.text, unknown_text))

updater.start_polling()
