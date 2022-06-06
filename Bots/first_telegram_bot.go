package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"strings"
)

// A struct that mimics the webhook response
type webhookReqBody struct {
	Message struct {
		Text string `json:"text"`
		Chat struct {
			ID int64 `json:"id"`
		} `json:"chat"`
	} `json:"message"`
}

// This handler is called everytime telegram sends a event
func Handler(res http.ResponseWriter, req *http.Request) {
	// Decode Json response
	body := &webhookReqBody{}
	if err := json.NewDecoder(req.Body).Decode(body); err != nil {
		fmt.Println("could not decode request body", err)
		return
	}

	// Check if the message contains the word "marco"
	if !strings.Contains(strings.ToLower(body.Message.Text), "marco") {
		return
	}

	// If the text contains marco, call the `sayPolo` function
	if err := sayPolo(body.Message.Chat.ID); err != nil {
		fmt.Println("error in sending replu: ", err)
		return
	}

	// log a confirmation message if sucessfully sent
	fmt. Println("reply sent")
}

type sendMessageReqBody struct {
	ChatID int64 `json:"chat_id"`
	Text string `json:"text"`
}

// sayPolo takes a chatID and sends "polo" to then
func sayPolo(chatID int64) error {
	// Create the request body struct
	reqBody := &sendMessageReqBody {
		ChatID: chatID,
		Text: "Polo!"
	}
	// Create the JSON body from the struct
	if reqBytes, err := json.Marshal(reqBody); err != nil {
		return err
	}
	// Send a post request with your token
	if res, err := http.Post(
		"https://api.telegram.org/bot<token>/sendMessage",
		"application/json",
		bytes.NewBuffer(reqBytes)); err != nil {
		return err
	} else if res.StatusCode != http.StatusOK {
		return errors.New("unexpected status" + res.Status)
	}
	return nil
}

func main() {
	http.ListenAndServe(":3000", http.HandlerFunc(Handler))
	// To deply service run: $ ngrok http 3000
	// $ curl -F "url=https://xxxx.ngrok.io/"
	// http://api.telegram.org/bot<token>/setWebhook
}
