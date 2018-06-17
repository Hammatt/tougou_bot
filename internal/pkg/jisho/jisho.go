package jisho

import (
	"encoding/json"
	"net/url"
	"strings"

	"github.com/bwmarrin/discordgo"
	"github.com/hammatt/tougou_bot/internal/pkg/apicaller"
	"github.com/hammatt/tougou_bot/internal/pkg/commandsplitter"
	"github.com/hammatt/tougou_bot/internal/pkg/util"
)

type apiResult struct {
	Meta struct {
		Status int `json:"status"`
	} `json:"meta"`
	Data []struct {
		IsCommon bool          `json:"is_common"`
		Tags     []interface{} `json:"tags"`
		Japanese []struct {
			Word    string `json:"word"`
			Reading string `json:"reading"`
		} `json:"japanese"`
		Senses []struct {
			EnglishDefinitions []string      `json:"english_definitions"`
			PartsOfSpeech      []string      `json:"parts_of_speech"`
			Links              []interface{} `json:"links"`
			Tags               []interface{} `json:"tags"`
			Restrictions       []interface{} `json:"restrictions"`
			SeeAlso            []interface{} `json:"see_also"`
			Antonyms           []interface{} `json:"antonyms"`
			Source             []interface{} `json:"source"`
			Info               []interface{} `json:"info"`
		} `json:"senses"`
		Attribution struct {
			Jmdict   bool `json:"jmdict"`
			Jmnedict bool `json:"jmnedict"`
			Dbpedia  bool `json:"dbpedia"`
		} `json:"attribution"`
	} `json:"data"`
}

func jishoAPISearch(s string) (string, error) {
	searchURL := "https://jisho.org/api/v1/search/words?keyword=\"" + url.QueryEscape(s) + "\""

	result, err := apicaller.CallAPI(searchURL)
	if err != nil {
		return "", err
	}

	return result, nil
}

func parseAPIJSONResultToStruct(jsonApiResult string) apiResult {
	var result apiResult
	json.Unmarshal([]byte(jsonApiResult), &result)
	return result
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
		//take the first result from the api.
		jishoResult := parseAPIJSONResultToStruct(jishoAPIResult).Data[0]

		embed := util.NewEmbed().
			AddField("Reading(s)", jishoResult.Japanese[0].Reading).
			AddField("Definition(s):", jishoResult.Senses[0].EnglishDefinitions[0]).
			SetColor(0xff0000).MessageEmbed
		s.ChannelMessageSendEmbed(m.ChannelID, embed)
	}
}
