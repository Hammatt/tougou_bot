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

func parseAPIJSONResultToStruct(jsonAPIResult string) apiResult {
	var result apiResult
	json.Unmarshal([]byte(jsonAPIResult), &result)
	return result
}

//CommandHandler : to be added as a handler by the discordgo library to handle the !jisho command
func CommandHandler(s *discordgo.Session, m *discordgo.MessageCreate) {
	//ignore messages from self
	if m.Author.ID == s.State.User.ID {
		return
	}

	if strings.HasPrefix(m.Content, "!jisho") {

		splitCommand := commandsplitter.SplitCommand(m.Content)
		jishoAPIResult, err := jishoAPISearch(splitCommand[1])
		if err != nil {
			s.ChannelMessageSend(m.ChannelID, "辞書を調べて中、エラーが発生しました: "+err.Error())
			return
		}
		//take the first result from the api.
		jishoResults := parseAPIJSONResultToStruct(jishoAPIResult)
		if len(jishoResults.Data) < 1 {
			s.ChannelMessageSend(m.ChannelID, "その言葉は辞書にない")
			return
		}

		//pick the top result
		jishoResult := jishoResults.Data[0]
		readings, sep := "", ""
		for i := 0; i < len(jishoResult.Japanese); i++ {
			if jishoResult.Japanese[i].Word != "" {
				readings += sep + jishoResult.Japanese[i].Word
				readings += "(" + jishoResult.Japanese[i].Reading + ")"
			} else {
				readings += sep + jishoResult.Japanese[i].Reading
			}
			sep = ", "
		}
		definitions := ""
		sep = ""
		for i := 0; i < len(jishoResult.Senses[0].EnglishDefinitions); i++ {
			definitions += sep + jishoResult.Senses[0].EnglishDefinitions[i]
			sep = ", "
		}
		embed := util.NewEmbed().
			AddField("Reading(s)", readings).
			AddField("Definition(s):", definitions).
			SetColor(0x56d926).MessageEmbed
		s.ChannelMessageSendEmbed(m.ChannelID, embed)
	}
}
