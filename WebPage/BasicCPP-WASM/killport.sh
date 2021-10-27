#!/usr/bin/bash

ps aux \
	| grep -E "dany98.*python.*http" \
	| head -1 \
	| awk '{ print $2 }' \
	| xargs -I % kill -9 --verbose --signal QUIT %
