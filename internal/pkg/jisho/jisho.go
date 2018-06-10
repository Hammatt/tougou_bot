package jisho

import (
	"errors"
	"strings"

	"github.com/bwmarrin/discordgo"
)

func jishoAPISearch(s string) (string, error) {
	//TODO everything

	return "", errors.New("not yet implemented")
}

//CommandHandler : to be added as a handler by the discordgo library to handle the !jisho command
func CommandHandler(s *discordgo.Session, m *discordgo.MessageCreate) {
	//ignore messages from self
	if m.Author.ID == s.State.User.ID {
		return
	}

	if strings.HasPrefix(m.Content, "!jisho") {
		s.ChannelMessageSend(m.ChannelID, "got jisho command")
	}
}
