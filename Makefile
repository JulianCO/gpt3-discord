.PHONY = build-image run

build-image:
	docker build -t gpt3-discord .

run:
	docker run -it -e OPENAI_KEY=`cat openaitoken` -e DISCORD_TOKEN=`cat discordtoken` --rm --name my-bot-running gpt3-discord
