container_name = my-bot-running
image_name = gpt3-discord
container_secrets_directory = /run/secrets

local_secrets_directory = ${GPT3_DISCORD_SECRETS_DIRECTORY}
openai_key_filename = openaitoken
discord_token_filename = discordtoken

openai_key_file = $(container_secrets_directory)/$(openai_key_filename)
discord_token_file = $(container_secrets_directory)/$(discord_token_filename)


.PHONY = build-image run run-background run-interactive

build-image:
	sudo docker build -t $(image_name) .

run: run-background

run-background:
	sudo docker run -d --rm \
		-e OPENAI_KEY_FILE=$(openai_key_file) \
		-e DISCORD_TOKEN_FILE=$(discord_token_file) \
		-v $(local_secrets_directory):$(container_secrets_directory)\
		--name $(container_name) \
		$(image_name)

run-interactive:
	sudo docker run -it --rm \
		-e OPENAI_KEY_FILE=$(openai_key_file) \
		-e DISCORD_TOKEN_FILE=$(discord_token_file) \
		-v $(local_secrets_directory):$(container_secrets_directory)\
		--name $(container_name) \
		$(image_name)