package modules

import (
	"strings"

	"github.com/bwmarrin/discordgo"
)

//JishoCommand : to be added as a handler by the discordgo library to handle the !jisho command
func JishoCommand(s *discordgo.Session, m *discordgo.MessageCreate) {
	//ignore messages from self
	if m.Author.ID == s.State.User.ID {
		return
	}

	if strings.HasPrefix(m.Content, "!jisho") {
		s.ChannelMessageSend(m.ChannelID, "got jisho command")
	}
}
