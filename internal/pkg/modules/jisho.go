package modules

import (
	"strings"

	"github.com/bwmarrin/discordgo"
)

func Jisho_command(s *discordgo.Session, m *discordgo.MessageCreate) {
	//ignore messages from self
	if m.Author.ID == s.State.User.ID {
		return
	}

	if strings.HasPrefix(m.Content, "!jisho") {
		s.ChannelMessageSend(m.ChannelID, "got jisho command")
	}
}
