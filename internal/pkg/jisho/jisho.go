package jisho

import (
	"net/url"
	"strings"

	"github.com/bwmarrin/discordgo"
	"github.com/hammatt/tougou_bot/internal/pkg/apicaller"
	"github.com/hammatt/tougou_bot/internal/pkg/commandsplitter"
)

func jishoAPISearch(s string) (string, error) {
	searchURL := "https://jisho.org/api/v1/search/words?keyword=\"" + url.QueryEscape(s) + "\""

	result, err := apicaller.CallAPI(searchURL)
	if err != nil {
		return "", err
	}

	return result, nil
}

//CommandHandler : to be added as a handler by the discordgo library to handle the !jisho command
func CommandHandler(s *discordgo.Session, m *discordgo.MessageCreate) {
	//ignore messages from self
	if m.Author.ID == s.State.User.ID {
		return
	}

	if strings.HasPrefix(m.Content, "!jisho") {
		s.ChannelMessageSend(m.ChannelID, "got jisho command")

		splitCommand := commandsplitter.SplitCommand(m.Content)
		jishoAPIResult, err := jishoAPISearch(splitCommand[1])
		if err != nil {
			s.ChannelMessageSend(m.ChannelID, "jisho search error: "+err.Error())
			return
		}
		s.ChannelMessageSend(m.ChannelID, jishoAPIResult)
	}
}
