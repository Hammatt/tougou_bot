package main

import (
	"flag"
	"fmt"
	"os"
	"os/signal"
	"syscall"

	"github.com/bwmarrin/discordgo"
	"github.com/hammatt/tougou_bot/internal/pkg/modules"
)

var (
	Token string
)

func init() {
	flag.StringVar(&Token, "t", "", "BotToken")
	flag.Parse()
}

func message_create(s *discordgo.Session, m *discordgo.MessageCreate) {
	//ignore messages from self
	if m.Author.ID == s.State.User.ID {
		return
	}

	//ping command
	if m.Content == "!ping" {
		s.ChannelMessageSend(m.ChannelID, "Pong!")
	}
}

func main() {
	dg, err := discordgo.New("Bot " + Token)
	if err != nil {
		fmt.Println("error creating Discord session,", err)
		return
	}

	dg.AddHandler(message_create)

	dg.AddHandler(modules.Jisho_command)

	err = dg.Open()
	if err != nil {
		fmt.Println("error opening connection,", err)
		return
	}

	//wait until recieve exit
	fmt.Println("tougou is now running. Press CTRL-C to exit.")
	sc := make(chan os.Signal, 1)
	signal.Notify(sc, syscall.SIGINT, syscall.SIGTERM, os.Interrupt, os.Kill)
	<-sc

	dg.Close()
}
