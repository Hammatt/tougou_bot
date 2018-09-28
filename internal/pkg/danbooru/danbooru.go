package danbooru

import (
	"encoding/json"
	"net/url"
	"strconv"
	"strings"

	"github.com/bwmarrin/discordgo"
	"github.com/hammatt/tougou_bot/internal/pkg/apicaller"
	"github.com/hammatt/tougou_bot/internal/pkg/commandsplitter"
	"github.com/hammatt/tougou_bot/internal/pkg/util"
)

type apiResult []struct {
	ID                  int         `json:"id"`
	CreatedAt           string      `json:"created_at"`
	UploaderID          int         `json:"uploader_id"`
	Score               int         `json:"score"`
	Source              string      `json:"source"`
	Md5                 string      `json:"md5"`
	LastCommentBumpedAt interface{} `json:"last_comment_bumped_at"`
	Rating              string      `json:"rating"`
	ImageWidth          int         `json:"image_width"`
	ImageHeight         int         `json:"image_height"`
	TagString           string      `json:"tag_string"`
	IsNoteLocked        bool        `json:"is_note_locked"`
	FavCount            int         `json:"fav_count"`
	FileExt             string      `json:"file_ext"`
	LastNotedAt         interface{} `json:"last_noted_at"`
	IsRatingLocked      bool        `json:"is_rating_locked"`
	ParentID            interface{} `json:"parent_id"`
	HasChildren         bool        `json:"has_children"`
	ApproverID          int         `json:"approver_id"`
	TagCountGeneral     int         `json:"tag_count_general"`
	TagCountArtist      int         `json:"tag_count_artist"`
	TagCountCharacter   int         `json:"tag_count_character"`
	TagCountCopyright   int         `json:"tag_count_copyright"`
	FileSize            int         `json:"file_size"`
	IsStatusLocked      bool        `json:"is_status_locked"`
	PoolString          string      `json:"pool_string"`
	UpScore             int         `json:"up_score"`
	DownScore           int         `json:"down_score"`
	IsPending           bool        `json:"is_pending"`
	IsFlagged           bool        `json:"is_flagged"`
	IsDeleted           bool        `json:"is_deleted"`
	TagCount            int         `json:"tag_count"`
	UpdatedAt           string      `json:"updated_at"`
	IsBanned            bool        `json:"is_banned"`
	PixivID             interface{} `json:"pixiv_id"`
	LastCommentedAt     interface{} `json:"last_commented_at"`
	HasActiveChildren   bool        `json:"has_active_children"`
	BitFlags            int         `json:"bit_flags"`
	TagCountMeta        int         `json:"tag_count_meta"`
	KeeperData          interface{} `json:"keeper_data"`
	UploaderName        string      `json:"uploader_name"`
	HasLarge            bool        `json:"has_large"`
	HasVisibleChildren  bool        `json:"has_visible_children"`
	ChildrenIds         interface{} `json:"children_ids"`
	IsFavorited         bool        `json:"is_favorited"`
	TagStringGeneral    string      `json:"tag_string_general"`
	TagStringCharacter  string      `json:"tag_string_character"`
	TagStringCopyright  string      `json:"tag_string_copyright"`
	TagStringArtist     string      `json:"tag_string_artist"`
	TagStringMeta       string      `json:"tag_string_meta"`
	FileURL             string      `json:"file_url"`
	LargeFileURL        string      `json:"large_file_url"`
	PreviewFileURL      string      `json:"preview_file_url"`
}

func parseAPIJSONResultToStruct(jsonAPIResult string) apiResult {
	var result apiResult
	json.Unmarshal([]byte(jsonAPIResult), &result)
	return result
}

func danbooruAPISearch(s string) (string, error) {
	//the api does most of the heavy lifting.
	searchURL := "https://danbooru.donmai.us/posts.json?search&random=true&limit=1&tags=" + url.QueryEscape(s)

	result, err := apicaller.CallAPI(searchURL)
	if err != nil {
		return "", err
	}

	return result, nil
}

//CommandHandler : to be added as a handler by the discordgo library to handle the !pic commands
func CommandHandler(s *discordgo.Session, m *discordgo.MessageCreate) {
	//ignore messages from self
	if m.Author.ID == s.State.User.ID {
		return
	}

	if strings.HasPrefix(m.Content, "!pic") {
		//build the query
		_, args := commandsplitter.SplitCommand(m.Content)

		danbooruAPIResult, err := danbooruAPISearch(args)
		if err != nil {
			s.ChannelMessageSend(m.ChannelID, "TODO: danbooru error message: "+err.Error()) //TODO
			return
		}
		danbooruResults := parseAPIJSONResultToStruct(danbooruAPIResult)
		if len(danbooruResults) < 1 {
			s.ChannelMessageSend(m.ChannelID, "TODO: danbooru error message...")
			return
		}
		danbooruResult := danbooruResults[0]

		embed := util.NewEmbed().
			SetTitle("<https://danbooru.donmai.us/posts/" + strconv.Itoa(danbooruResult.ID) + ">").
			SetImage(danbooruResult.FileURL).
			SetColor(0xADD8E6).MessageEmbed
		s.ChannelMessageSendEmbed(m.ChannelID, embed)
	}
}
